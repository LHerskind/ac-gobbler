mod types;
mod bindings;
mod cli;

use std::path::PathBuf;
use std::sync::Arc;
use crate::types::block::Block;
use ethers::{
    providers::{Http, Provider},
};
use microkv::MicroKV;
use clap::Parser;
use crate::cli::collector::sync_blocks;
use crate::cli::export::export_transactions_csv;

pub const DEPLOYMENT_BLOCK: u64 = 14923081;

pub fn decode_block(db: &MicroKV, rollup_id: u64) {
    let id = format!("{:?}", rollup_id);
    let block = db.get::<Block>(id).unwrap().unwrap();
    println!("{:#?}", block);
}

#[derive(Parser, Debug)]
#[clap(name = "gobbler", version)]
#[command(author = "LHerskind <lasse@aztecprotocol.com>")]
#[command(about = "Aztec Connect Data Gobbler", long_about = "A tool for collecting rollup blocks from the Aztec Connect rollup, and exporting them to a csv file")]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[clap(name = "sync", about = "Synchronise the local database with the rollup")]
    Sync {
        #[clap(
        long,
        default_value_t = String::from("http://localhost:8545"),
        help = "The RPC url to an ethereum node"
        )]
        rpc_url: String,
        #[clap(long, default_value_t = String::from("./data/"), help = "The path to the dir of the database")]
        data_path: String,
    },
    #[clap(name = "decode", about = "Print the contents of a block in a semi readable manner")]
    Decode {
        #[clap(
        help = "The rollup id of the block to decode"
        )]
        rollup_id: u64,
        #[clap(long, default_value_t = String::from("./data/"), help = "The path to the dir of the database")]
        data_path: String,
    },
    #[clap(name = "export", about = "Exports all transactions to a csv file")]
    Export {
        #[clap(long, default_value_t = String::from("./export/txs.csv"), help = "The file to write csv to")]
        export_path: String,
        #[clap(long, short, help = "Export only deposits and withdrawals")]
        l1_only: bool,
        #[clap(long, default_value_t = String::from("./data/"), help = "The path to the dir of the database")]
        data_path: String,
    },

}

fn get_db(path: String) -> MicroKV {
    MicroKV::open_with_base_path("gobbler", PathBuf::from(path)).expect("Failed to open database")
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync { rpc_url, data_path } => {
            let client = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());
            let db = get_db(data_path).set_auto_commit(true);

            let loaded_block = |db: &MicroKV| {
                let keys = db.keys().unwrap();
                if keys.len() == 0 {
                    return (0, DEPLOYMENT_BLOCK);
                }
                let max_key = keys.iter().map(|key| match key.parse::<u64>() {
                    Ok(key) => key,
                    Err(_) => 0,
                }).max().unwrap();
                let id = format!("{:?}", max_key);
                let block = db.get::<Block>(id).unwrap().unwrap();
                (max_key, block.metadata.block_number.as_u64())
            };

            let (l2_starting_block, l1_starting_block) = loaded_block(&db);
            println!("Synchronizing from Aztec Connect block {}, Ethereum L1 block: {}", l2_starting_block, l1_starting_block);
            sync_blocks(&client, &db, l1_starting_block).await;
            println!("Sync completed");
        },
        Commands::Decode { rollup_id, data_path } => {
            decode_block(&get_db(data_path), rollup_id);
        },
        Commands::Export { export_path, l1_only, data_path } => {
            export_transactions_csv(&get_db(data_path), export_path, l1_only);
        }
    }
}
