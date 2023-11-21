use std::fmt::Write;
use std::sync::Arc;
use crate::bindings::rollup_processor::{ProcessRollupCall, RollupProcessedFilter, RollupProcessor};
use crate::types::block::Block;
use ethers::{
    abi::{AbiDecode, RawLog},
    prelude::EthEvent,
    providers::{Http, Middleware, Provider, StreamExt},
    types::{Address, H256},
};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use microkv::MicroKV;

async fn decode_block(client: &Arc<Provider<Http>>, db: &MicroKV, tx_hash: H256, event: RollupProcessedFilter) {
    let tx = match client.get_transaction(tx_hash).await.unwrap() {
        Some(tx) => tx,
        None => {
            println!("No decodable transaction found at: {:?}", tx_hash);
            todo!();
        }
    };

    let proof_calldata = ProcessRollupCall::decode(&tx.input).unwrap().proof_data;
    let l1_block = client.get_block(tx.block_number.unwrap()).await.unwrap().unwrap();
    let block = Block::from((tx_hash, tx.block_number.unwrap(), l1_block.timestamp, proof_calldata, event));

    let id = format!("{:?}", block.inner.header.rollup_id);

    match db.put(id, &block) {
        Ok(_) => (),
        Err(err) => {
            println!("Database err when writing {:?}: {:?}", tx_hash, err);
        }
    }
}

pub async fn sync_blocks(client: &Arc<Provider<Http>>, db: &MicroKV, starting_block: u64) {
    let rollup: Address = client.resolve_name("rollup.aztec.eth").await.unwrap();
    let rollup = RollupProcessor::new(rollup, Arc::clone(&client));

    let block_number = client.get_block_number().await.unwrap().as_u64();
    let pb = ProgressBar::new(block_number - starting_block);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} (eta: {eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let filter = rollup
        .rollup_processed_filter()
        .from_block(starting_block)
        .filter;

    let stream = client.get_logs_paginated(&filter, 5000);

    stream
        .for_each_concurrent(10, |res| async {
            let log = res.unwrap();
            let event = RollupProcessedFilter::decode_log(&RawLog {
                topics: log.topics,
                data: log.data.to_vec(),
            })
                .unwrap();
            decode_block(&client, &db, log.transaction_hash.unwrap(), event).await;

            let current_block = log.block_number.unwrap().as_u64();
            if pb.position() <= current_block - starting_block {
                pb.set_position(current_block - starting_block);
            }
        })
        .await;
}