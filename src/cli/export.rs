use std::fs;
use std::path::Path;
use ethers::types::{Address, H256, U256};
use indicatif::{ProgressBar, ProgressStyle};
use microkv::MicroKV;
use crate::types::block::Block;
use csv::Writer;
use crate::types::transaction::ProofId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportInnerProofData {
    pub rollup_id: U256,
    pub proof_id: ProofId,
    pub note_commitment_1: H256,
    pub note_commitment_2: H256,
    pub nullifier_1: H256,
    pub nullifier_2: H256,
    pub public_value: U256,
    pub public_owner: Address,
    pub public_asset_id: U256,
}

pub fn export_transactions_csv(db: &MicroKV, path: String, l1_only: bool) {
    let path = Path::new(&path);
    match path.parent() {
        None => (),
        Some(parent) => match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(_) => todo!(),
        },
    }

    let mut keys = db.keys().unwrap();
    // Not robust, but works for now
    keys.sort_by_key(|k| k.parse::<u64>().unwrap());
    let mut wtr = Writer::from_path(path).unwrap();
    let mut tx_count = 0;

    let pb = ProgressBar::new(keys.len() as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] blocks exported {pos}/{len}")
        .unwrap()
        .progress_chars("#>-"));

    for key in keys {
        let block = db.get::<Block>(key).unwrap().unwrap();
        for tx in block.inner.inner_proofs {
            if l1_only && tx.proof_id != ProofId::Deposit && tx.proof_id != ProofId::Withdraw {
                continue;
            }

            let tx = ExportInnerProofData {
                rollup_id: block.inner.header.rollup_id,
                proof_id: tx.proof_id,
                note_commitment_1: tx.note_commitment_1,
                note_commitment_2: tx.note_commitment_2,
                nullifier_1: tx.nullifier_1,
                nullifier_2: tx.nullifier_2,
                public_value: tx.public_value,
                public_owner: tx.public_owner,
                public_asset_id: tx.public_asset_id,
            };

            wtr.serialize(tx).unwrap();
            tx_count += 1;
        }
        pb.inc(1);
    }
    pb.finish();

    println!("Exported {} transactions in {:.2} seconds", tx_count, pb.elapsed().as_secs_f64());
}
