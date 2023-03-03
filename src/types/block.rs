use crate::bindings::rollup_processor::RollupProcessedFilter;
use crate::types::{
    defi_interaction::DefiInteraction,
    rollup_proof::Proof,
    transaction::{InnerProofData, ProofId},
};
use ethers::types::{Address, Bytes, TxHash, H256, U256, U64};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FeeAsset {
    pub asset_id: U256,
    pub amount: U256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Header {
    pub rollup_id: U256,
    pub rollup_size: U256,
    pub data_start_index: U256,
    pub old_data_root: H256,
    pub new_data_root: H256,
    pub old_null_root: H256,
    pub new_null_root: H256,
    pub old_data_roots_root: H256,
    pub new_data_roots_root: H256,
    pub old_defi_root: H256,
    pub new_defi_root: H256,
    pub defi_interactions: Vec<DefiInteraction>,
    pub fees: Vec<FeeAsset>,
    pub prev_defi_interaction_hashes: Vec<H256>,
    pub prev_defi_interaction_hash: H256,
    pub rollup_beneficiary: Address,
    pub num_rollup_txs: U256,
}

impl From<Bytes> for Header {
    fn from(src: Bytes) -> Self {
        let sl = &src;
        let mut header = Header::default();

        header.rollup_id = U256::from_big_endian(&sl[0..32]);
        header.rollup_size = U256::from_big_endian(&sl[32..64]);
        header.data_start_index = U256::from_big_endian(&sl[64..96]);
        header.old_data_root = H256::from_slice(&sl[96..128]);
        header.new_data_root = H256::from_slice(&sl[128..160]);
        header.old_null_root = H256::from_slice(&sl[160..192]);
        header.new_null_root = H256::from_slice(&sl[192..224]);
        header.old_data_roots_root = H256::from_slice(&sl[224..256]);
        header.new_data_roots_root = H256::from_slice(&sl[256..288]);
        header.old_defi_root = H256::from_slice(&sl[288..320]);
        header.new_defi_root = H256::from_slice(&sl[320..352]);

        for i in 0..32 {
            if i < 16 {
                // Note: While we could practically screw up the ordering here, it don't matter much
                // because, it does not go into the rollup state.
                let fee = FeeAsset {
                    asset_id: U256::from_big_endian(&sl[2400 + 32 * i..2400 + 32 * i + 32]),
                    amount: U256::from_big_endian(&sl[2912 + 32 * i..2912 + 32 * i + 32]),
                };
                if !fee.amount.is_zero() {
                    header.fees.push(fee);
                }
            }
            let defi_interaction = DefiInteraction::from((
                Bytes::from(sl[352 + 32 * i..352 + 32 * i + 32].to_vec()),
                U256::from_big_endian(&sl[1376 + 32 * i..1376 + 32 * i + 32]),
            ));
            if !defi_interaction.bridge_address_id.is_zero() {
                header.defi_interactions.push(defi_interaction);
            }

            // Note: The notes that we will be adding to the defi tree, up to 32 this will be the one from the last
            // rollup we executed. Hashes in n..32 will be the the "empty defi interaction result hash".
            let prev_defi_interaction_hash_i =
                H256::from_slice(&sl[3424 + 32 * i..3424 + 32 * i + 32]);
            if !prev_defi_interaction_hash_i.is_zero() {
                header
                    .prev_defi_interaction_hashes
                    .push(prev_defi_interaction_hash_i);
            }
        }

        header.prev_defi_interaction_hash = H256::from_slice(&sl[4448..4448 + 32]);
        header.rollup_beneficiary = Address::from_slice(&sl[4480 + 12..4480 + 32]);
        header.num_rollup_txs = U256::from_big_endian(&sl[4512..4512 + 32]);

        return header;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    pub tx_hash: TxHash,
    pub block_number: U64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InnerBlock {
    pub header: Header,
    pub num_real_txs: U256,
    pub encoded_inner_tx_data_length: U256,
    pub inner_proofs: Vec<InnerProofData>,
    pub proof: Proof,
}

impl From<Bytes> for InnerBlock {
    fn from(calldata: Bytes) -> Self {
        let sl = &calldata;
        let mut inner_block = InnerBlock::default();

        inner_block.header = Header::from(calldata.clone());
        inner_block.num_real_txs = U256::from_big_endian(&sl[4544..4548]);
        inner_block.encoded_inner_tx_data_length = U256::from_big_endian(&sl[4548..4552]);

        let mut start = 4552;
        let end = start + inner_block.encoded_inner_tx_data_length.as_usize();

        while start < end {
            let proof_id = ProofId::try_from(sl[start]).unwrap();
            let proof_size = proof_id.data_size();
            let data = Bytes::from(sl[start..start + proof_size].to_vec());
            let inner_proof = InnerProofData::try_from(data).unwrap();

            match inner_proof.proof_id {
                ProofId::Padding => (),
                _ => inner_block.inner_proofs.push(inner_proof),
            }

            start += proof_size;
        }

        inner_block.proof = Proof::from(Bytes::from(sl[start..calldata.len()].to_vec()));

        return inner_block;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Block {
    pub metadata: Metadata,
    pub inner: InnerBlock,
    pub next_expected_defi_hashes: Vec<H256>,
    pub sequencer: Address,
}

impl From<(TxHash, U64, Bytes, RollupProcessedFilter)> for Block {
    fn from(
        (tx_hash, block_number, proof_calldata, rollup_processed_event): (
            TxHash,
            U64,
            Bytes,
            RollupProcessedFilter,
        ),
    ) -> Self {
        let mut block = Block::default();
        block.inner = InnerBlock::from(proof_calldata);
        block.metadata = Metadata {
            tx_hash,
            block_number,
        };

        block.next_expected_defi_hashes = rollup_processed_event
            .next_expected_defi_hashes
            .iter()
            .map(|x| H256::from_slice(&x[0..32]))
            .collect();

        block.sequencer = rollup_processed_event.sender;

        return block;
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        return serde_json::to_string(self).unwrap();
    }
}
