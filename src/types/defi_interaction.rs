use ethers::types::{Bytes, U256};
use serde::{Deserialize, Serialize};
use std::ops::Shr;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum AssetType {
    #[default]
    Unused,
    Real,
    Virtual,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Asset {
    pub asset_type: AssetType,
    pub asset_id: U256,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DefiInteraction {
    pub bridge_address_id: U256,
    pub input_asset_a: Asset,
    pub input_asset_b: Asset,
    pub output_asset_a: Asset,
    pub output_asset_b: Asset,
    pub aux_data: U256,
    pub total_input_value: U256,
}

impl From<(Bytes, U256)> for DefiInteraction {
    fn from((bridge_calldata, total_input_value): (Bytes, U256)) -> Self {
        let mask_32 = U256::from_str("0xffffffff").unwrap();
        let mask_30 = U256::from_str("0x3fffffff").unwrap();
        let mut defi_interaction = DefiInteraction::default();
        defi_interaction.total_input_value = total_input_value;
        defi_interaction.aux_data = U256::from_big_endian(&bridge_calldata[1..9]);

        let src = U256::from_big_endian(&bridge_calldata[0..32]);
        defi_interaction.bridge_address_id = src & mask_32;

        let input_asset_id_a = src.shr(32) & mask_30;
        let input_asset_id_b = src.shr(62) & mask_30;
        let output_asset_id_a = src.shr(92) & mask_30;
        let output_asset_id_b = src.shr(122) & mask_30;
        let bit_config = src.shr(152) & mask_32;

        let second_input_in_use = bit_config.bit(0);
        let second_output_in_use = bit_config.bit(1);

        fn asset_in_use(asset_id: U256) -> Asset {
            Asset {
                asset_id,
                asset_type: match asset_id.bit(30) {
                    true => AssetType::Virtual,
                    false => AssetType::Real,
                },
            }
        }

        fn asset_possible_in_use(in_use: bool, asset_id: U256) -> Asset {
            return match in_use {
                true => asset_in_use(asset_id),
                false => Asset {
                    asset_type: AssetType::Unused,
                    asset_id: U256::from(0),
                },
            };
        }

        defi_interaction.input_asset_a = asset_in_use(input_asset_id_a);
        defi_interaction.output_asset_a = asset_in_use(output_asset_id_a);
        defi_interaction.input_asset_b =
            asset_possible_in_use(second_input_in_use, input_asset_id_b);
        defi_interaction.output_asset_b =
            asset_possible_in_use(second_output_in_use, output_asset_id_b);
        return defi_interaction;
    }
}
