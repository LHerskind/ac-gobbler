use ethers::types::{Address, Bytes, H256, U256};
use ethers::utils::rlp::DecoderError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub enum ProofId {
    #[default]
    Padding,
    Deposit,
    Withdraw,
    Send,
    Account,
    DefiDeposit,
    DefiClaim,
}

impl TryFrom<u8> for ProofId {
    type Error = DecoderError;
    fn try_from(src: u8) -> Result<Self, Self::Error> {
        return match src {
            0 => Ok(ProofId::Padding),
            1 => Ok(ProofId::Deposit),
            2 => Ok(ProofId::Withdraw),
            3 => Ok(ProofId::Send),
            4 => Ok(ProofId::Account),
            5 => Ok(ProofId::DefiDeposit),
            6 => Ok(ProofId::DefiClaim),
            _ => Err(DecoderError::Custom("Invalid type")),
        };
    }
}

impl ProofId {
    pub fn data_size(&self) -> usize {
        return match self {
            ProofId::Padding => 1,
            ProofId::Deposit => 185,
            ProofId::Withdraw => 185,
            ProofId::Send => 129,
            ProofId::Account => 129,
            ProofId::DefiDeposit => 129,
            ProofId::DefiClaim => 129,
        };
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InnerProofData {
    pub proof_id: ProofId,
    pub note_commitment_1: H256,
    pub note_commitment_2: H256,
    pub nullifier_1: H256,
    pub nullifier_2: H256,
    pub public_value: U256,
    pub public_owner: Address,
    pub public_asset_id: U256,
}

impl TryFrom<Bytes> for InnerProofData {
    type Error = DecoderError;
    fn try_from(src: Bytes) -> Result<InnerProofData, Self::Error> {
        let mut proof = InnerProofData::default();
        proof.proof_id = ProofId::try_from(src[0]).unwrap();

        match proof.proof_id {
            ProofId::Padding => return Ok(proof),
            _ => (),
        }

        proof.note_commitment_1 = H256::from_slice(&src[1..33]);
        proof.note_commitment_2 = H256::from_slice(&src[33..65]);
        proof.nullifier_1 = H256::from_slice(&src[65..97]);
        proof.nullifier_2 = H256::from_slice(&src[97..129]);

        match proof.proof_id {
            ProofId::Deposit | ProofId::Withdraw => {
                if src.len() != 185 {
                    return Err(DecoderError::Custom("Invalid proof size"));
                }
                proof.public_value = U256::from_big_endian(&src[129..161]);
                proof.public_owner = Address::from_slice(&src[161..181]);
                proof.public_asset_id = U256::from_big_endian(&src[181..185]);
            }
            _ => (),
        };

        return Ok(proof);
    }
}
