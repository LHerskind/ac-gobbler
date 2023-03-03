use ethers::types::{Bytes, H256, U256};
use serde::{Deserialize, Serialize};
use std::ops::Shl;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct G1Point {
    pub x: U256,
    pub y: U256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Proof {
    pub public_input_hash: H256,
    pub recursive_p1: G1Point,
    pub recursive_p2: G1Point,
    pub w1: G1Point,
    pub w2: G1Point,
    pub w3: G1Point,
    pub t1: G1Point,
    pub t2: G1Point,
    pub t3: G1Point,
    pub w1_eval: U256,
    pub w2_eval: U256,
    pub w3_eval: U256,
    pub sigma_1_eval: U256,
    pub sigma_2_eval: U256,
    pub z_omega_eval: U256,
    pub pi_z: G1Point,
    pub pi_z_omega: G1Point,
}

impl From<Bytes> for Proof {
    fn from(src: Bytes) -> Self {
        let mut proof = Proof::default();

        proof.public_input_hash = H256::from_slice(&src[0..32]);

        // The G1Points that we need for the recursive proof need to go into the snark circuit.
        // This is another field, so they are split into smaller chunks for the circuit to be happy
        // logically, we can however put it back together for this.
        // Each of the coordinates are stored in 4 chunks of 68 bits.
        let mut recursive_p1_x = U256::from(0);
        let mut recursive_p1_y = U256::from(0);
        let mut recursive_p2_x = U256::from(0);
        let mut recursive_p2_y = U256::from(0);

        for i in 0..4 {
            let offset = 68 * i;
            recursive_p1_x += U256::from_big_endian(&src[32 + i * 32..64 + i * 32]).shl(offset);
            recursive_p1_y += U256::from_big_endian(&src[160 + i * 32..192 + i * 32]).shl(offset);
            recursive_p2_x += U256::from_big_endian(&src[288 + i * 32..320 + i * 32]).shl(offset);
            recursive_p2_y += U256::from_big_endian(&src[416 + i * 32..448 + i * 32]).shl(offset);
        }

        proof.recursive_p1 = G1Point {
            x: recursive_p1_x,
            y: recursive_p1_y,
        };

        proof.recursive_p2 = G1Point {
            x: recursive_p2_x,
            y: recursive_p2_y,
        };

        let q = U256::from("0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47");
        if !(recursive_p1_x.lt(&q)
            && recursive_p1_y.lt(&q)
            && recursive_p2_x.lt(&q)
            && recursive_p2_y.lt(&q))
        {
            panic!("Pisso lorto, frem med modolo");
        }

        proof.w1 = G1Point {
            x: U256::from_big_endian(&src[544..544 + 32]),
            y: U256::from_big_endian(&src[544 + 32 * 1..544 + 32 * 2]),
        };

        proof.w2 = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 2..544 + 32 * 3]),
            y: U256::from_big_endian(&src[544 + 32 * 3..544 + 32 * 4]),
        };

        proof.w3 = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 4..544 + 32 * 5]),
            y: U256::from_big_endian(&src[544 + 32 * 5..544 + 32 * 6]),
        };

        proof.t1 = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 6..544 + 32 * 7]),
            y: U256::from_big_endian(&src[544 + 32 * 7..544 + 32 * 8]),
        };

        proof.t2 = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 8..544 + 32 * 9]),
            y: U256::from_big_endian(&src[544 + 32 * 9..544 + 32 * 10]),
        };

        proof.t3 = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 10..544 + 32 * 11]),
            y: U256::from_big_endian(&src[544 + 32 * 11..544 + 32 * 12]),
        };

        proof.w1_eval = U256::from_big_endian(&src[544 + 32 * 12..544 + 32 * 13]);
        proof.w2_eval = U256::from_big_endian(&src[544 + 32 * 13..544 + 32 * 14]);
        proof.w3_eval = U256::from_big_endian(&src[544 + 32 * 14..544 + 32 * 15]);
        proof.sigma_1_eval = U256::from_big_endian(&src[544 + 32 * 15..544 + 32 * 16]);
        proof.sigma_2_eval = U256::from_big_endian(&src[544 + 32 * 16..544 + 32 * 17]);
        proof.z_omega_eval = U256::from_big_endian(&src[544 + 32 * 17..544 + 32 * 18]);

        proof.pi_z = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 18..544 + 32 * 19]),
            y: U256::from_big_endian(&src[544 + 32 * 19..544 + 32 * 20]),
        };

        proof.pi_z_omega = G1Point {
            x: U256::from_big_endian(&src[544 + 32 * 20..544 + 32 * 21]),
            y: U256::from_big_endian(&src[544 + 32 * 21..544 + 32 * 22]),
        };

        return proof;
    }
}
