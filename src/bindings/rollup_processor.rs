pub use rollup_processor::*;

#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod rollup_processor {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]

    #[doc = "RollupProcessor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;

    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;

    #[doc = r" The parsed human readable ABI of the contract."]
    pub static ROLLUPPROCESSOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: abi :: parse_abi_str ("[\n        function processRollup(bytes calldata proofData, bytes calldata signatures) external\n        event RollupProcessed(uint256 indexed rollupId, bytes32[] nextExpectedDefiHashes, address sender)\n    ]") . expect ("invalid abi")
        });
    pub struct RollupProcessor<M>(ethers::contract::Contract<M>);
    impl<M> Clone for RollupProcessor<M> {
        fn clone(&self) -> Self {
            RollupProcessor(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for RollupProcessor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for RollupProcessor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(RollupProcessor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> RollupProcessor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ROLLUPPROCESSOR_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `processRollup` (0xf81cccbe) function"]
        pub fn process_rollup(
            &self,
            proof_data: ethers::core::types::Bytes,
            signatures: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 28, 204, 190], (proof_data, signatures))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `RollupProcessed` event"]
        pub fn rollup_processed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RollupProcessedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, RollupProcessedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for RollupProcessor<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RollupProcessed",
        abi = "RollupProcessed(uint256,bytes32[],address)"
    )]
    pub struct RollupProcessedFilter {
        #[ethevent(indexed)]
        pub rollup_id: ethers::core::types::U256,
        pub next_expected_defi_hashes: Vec<[u8; 32]>,
        pub sender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `processRollup` function with signature `processRollup(bytes,bytes)` and selector `[248, 28, 204, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "processRollup", abi = "processRollup(bytes,bytes)")]
    pub struct ProcessRollupCall {
        pub proof_data: ethers::core::types::Bytes,
        pub signatures: ethers::core::types::Bytes,
    }
}
