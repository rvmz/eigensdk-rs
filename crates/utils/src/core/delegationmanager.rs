///Module containing a contract's types and functions.
/**

```solidity
library IDelegationManager {
    struct OperatorDetails { address __deprecated_earningsReceiver; address delegationApprover; uint32 stakerOptOutWindowBlocks; }
    struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] shares; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IDelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct OperatorDetails { address __deprecated_earningsReceiver; address delegationApprover; uint32 stakerOptOutWindowBlocks; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDetails {
        pub __deprecated_earningsReceiver: alloy::sol_types::private::Address,
        pub delegationApprover: alloy::sol_types::private::Address,
        pub stakerOptOutWindowBlocks: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            u32,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorDetails> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDetails) -> Self {
                (
                    value.__deprecated_earningsReceiver,
                    value.delegationApprover,
                    value.stakerOptOutWindowBlocks,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDetails {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    __deprecated_earningsReceiver: tuple.0,
                    delegationApprover: tuple.1,
                    stakerOptOutWindowBlocks: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDetails {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorDetails {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.__deprecated_earningsReceiver,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegationApprover,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.stakerOptOutWindowBlocks,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorDetails {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorDetails {
            const NAME: &'static str = "OperatorDetails";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDetails(address __deprecated_earningsReceiver,address delegationApprover,uint32 stakerOptOutWindowBlocks)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.__deprecated_earningsReceiver,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegationApprover,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stakerOptOutWindowBlocks,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDetails {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.__deprecated_earningsReceiver,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegationApprover,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stakerOptOutWindowBlocks,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.__deprecated_earningsReceiver,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegationApprover,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stakerOptOutWindowBlocks,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct QueuedWithdrawalParams { address[] strategies; uint256[] shares; address withdrawer; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QueuedWithdrawalParams {
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub shares:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        pub withdrawer: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<QueuedWithdrawalParams> for UnderlyingRustTuple<'_> {
            fn from(value: QueuedWithdrawalParams) -> Self {
                (value.strategies, value.shares, value.withdrawer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QueuedWithdrawalParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategies: tuple.0,
                    shares: tuple.1,
                    withdrawer: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QueuedWithdrawalParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QueuedWithdrawalParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for QueuedWithdrawalParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for QueuedWithdrawalParams {
            const NAME: &'static str = "QueuedWithdrawalParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QueuedWithdrawalParams(address[] strategies,uint256[] shares,address withdrawer)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.shares)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QueuedWithdrawalParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.shares,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.shares,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct Withdrawal { address staker; address delegatedTo; address withdrawer; uint256 nonce; uint32 startBlock; address[] strategies; uint256[] shares; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Withdrawal {
        pub staker: alloy::sol_types::private::Address,
        pub delegatedTo: alloy::sol_types::private::Address,
        pub withdrawer: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub startBlock: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub shares:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Withdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: Withdrawal) -> Self {
                (
                    value.staker,
                    value.delegatedTo,
                    value.withdrawer,
                    value.nonce,
                    value.startBlock,
                    value.strategies,
                    value.shares,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Withdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    staker: tuple.0,
                    delegatedTo: tuple.1,
                    withdrawer: tuple.2,
                    nonce: tuple.3,
                    startBlock: tuple.4,
                    strategies: tuple.5,
                    shares: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Withdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Withdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedTo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawer,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.shares),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Withdrawal {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Withdrawal {
            const NAME: &'static str = "Withdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Withdrawal(address staker,address delegatedTo,address withdrawer,uint256 nonce,uint32 startBlock,address[] strategies,uint256[] shares)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.staker,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedTo,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawer,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.nonce)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.shares)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Withdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.staker,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedTo,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawer,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.nonce)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.shares,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.staker,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedTo,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawer,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonce,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.shares,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IDelegationManager`](self) contract instance.

    See the [wrapper's documentation](`IDelegationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelegationManagerInstance<T, P, N> {
        IDelegationManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelegationManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IDelegationManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelegationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelegationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelegationManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IDelegationManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IDelegationManager`](self) contract instance.

        See the [wrapper's documentation](`IDelegationManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IDelegationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelegationManagerInstance<T, P, N> {
            IDelegationManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IDelegationManagerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > IDelegationManagerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithExpiry { bytes signature; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct SignatureWithExpiry { bytes signature; uint256 expiry; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithExpiry) -> Self {
                (value.signature, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    expiry: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.expiry,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithExpiry {
            const NAME: &'static str = "SignatureWithExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithExpiry(bytes signature,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtils`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISignatureUtilsInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IDelegationManager {
    struct OperatorDetails {
        address __deprecated_earningsReceiver;
        address delegationApprover;
        uint32 stakerOptOutWindowBlocks;
    }
    struct QueuedWithdrawalParams {
        address[] strategies;
        uint256[] shares;
        address withdrawer;
    }
    struct Withdrawal {
        address staker;
        address delegatedTo;
        address withdrawer;
        uint256 nonce;
        uint32 startBlock;
        address[] strategies;
        uint256[] shares;
    }
}

library ISignatureUtils {
    struct SignatureWithExpiry {
        bytes signature;
        uint256 expiry;
    }
}

interface DelegationManager {
    event Initialized(uint8 version);
    event MinWithdrawalDelayBlocksSet(uint256 previousValue, uint256 newValue);
    event OperatorDetailsModified(address indexed operator, IDelegationManager.OperatorDetails newOperatorDetails);
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    event OperatorRegistered(address indexed operator, IDelegationManager.OperatorDetails operatorDetails);
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event StakerDelegated(address indexed staker, address indexed operator);
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    event StakerUndelegated(address indexed staker, address indexed operator);
    event StrategyWithdrawalDelayBlocksSet(address strategy, uint256 previousValue, uint256 newValue);
    event Unpaused(address indexed account, uint256 newPausedStatus);
    event WithdrawalCompleted(bytes32 withdrawalRoot);
    event WithdrawalQueued(bytes32 withdrawalRoot, IDelegationManager.Withdrawal withdrawal);

    constructor(address _strategyManager, address _slasher, address _eigenPodManager);

    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    function DOMAIN_TYPEHASH() external view returns (bytes32);
    function MAX_STAKER_OPT_OUT_WINDOW_BLOCKS() external view returns (uint256);
    function MAX_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
    function STAKER_DELEGATION_TYPEHASH() external view returns (bytes32);
    function beaconChainETHStrategy() external view returns (address);
    function calculateCurrentStakerDelegationDigestHash(address staker, address operator, uint256 expiry) external view returns (bytes32);
    function calculateDelegationApprovalDigestHash(address staker, address operator, address _delegationApprover, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    function calculateStakerDelegationDigestHash(address staker, uint256 _stakerNonce, address operator, uint256 expiry) external view returns (bytes32);
    function calculateWithdrawalRoot(IDelegationManager.Withdrawal memory withdrawal) external pure returns (bytes32);
    function completeQueuedWithdrawal(IDelegationManager.Withdrawal memory withdrawal, address[] memory tokens, uint256 middlewareTimesIndex, bool receiveAsTokens) external;
    function completeQueuedWithdrawals(IDelegationManager.Withdrawal[] memory withdrawals, address[][] memory tokens, uint256[] memory middlewareTimesIndexes, bool[] memory receiveAsTokens) external;
    function cumulativeWithdrawalsQueued(address) external view returns (uint256);
    function decreaseDelegatedShares(address staker, address strategy, uint256 shares) external;
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegateToBySignature(address staker, address operator, ISignatureUtils.SignatureWithExpiry memory stakerSignatureAndExpiry, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    function delegatedTo(address) external view returns (address);
    function delegationApprover(address operator) external view returns (address);
    function delegationApproverSaltIsSpent(address, bytes32) external view returns (bool);
    function domainSeparator() external view returns (bytes32);
    function eigenPodManager() external view returns (address);
    function getDelegatableShares(address staker) external view returns (address[] memory, uint256[] memory);
    function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    function getWithdrawalDelay(address[] memory strategies) external view returns (uint256);
    function increaseDelegatedShares(address staker, address strategy, uint256 shares) external;
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, uint256 _minWithdrawalDelayBlocks, address[] memory _strategies, uint256[] memory _withdrawalDelayBlocks) external;
    function isDelegated(address staker) external view returns (bool);
    function isOperator(address operator) external view returns (bool);
    function minWithdrawalDelayBlocks() external view returns (uint256);
    function modifyOperatorDetails(IDelegationManager.OperatorDetails memory newOperatorDetails) external;
    function operatorDetails(address operator) external view returns (IDelegationManager.OperatorDetails memory);
    function operatorShares(address, address) external view returns (uint256);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function pendingWithdrawals(bytes32) external view returns (bool);
    function queueWithdrawals(IDelegationManager.QueuedWithdrawalParams[] memory queuedWithdrawalParams) external returns (bytes32[] memory);
    function registerAsOperator(IDelegationManager.OperatorDetails memory registeringOperatorDetails, string memory metadataURI) external;
    function renounceOwnership() external;
    function setMinWithdrawalDelayBlocks(uint256 newMinWithdrawalDelayBlocks) external;
    function setPauserRegistry(address newPauserRegistry) external;
    function setStrategyWithdrawalDelayBlocks(address[] memory strategies, uint256[] memory withdrawalDelayBlocks) external;
    function slasher() external view returns (address);
    function stakerNonce(address) external view returns (uint256);
    function stakerOptOutWindowBlocks(address operator) external view returns (uint256);
    function strategyManager() external view returns (address);
    function strategyWithdrawalDelayBlocks(address) external view returns (uint256);
    function transferOwnership(address newOwner) external;
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    function unpause(uint256 newPausedStatus) external;
    function updateOperatorMetadataURI(string memory metadataURI) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_strategyManager",
        "type": "address",
        "internalType": "contract IStrategyManager"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "contract ISlasher"
      },
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "DELEGATION_APPROVAL_TYPEHASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "DOMAIN_TYPEHASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "MAX_WITHDRAWAL_DELAY_BLOCKS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "STAKER_DELEGATION_TYPEHASH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "beaconChainETHStrategy",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateCurrentStakerDelegationDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateDelegationApprovalDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_delegationApprover",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateStakerDelegationDigestHash",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_stakerNonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateWithdrawalRoot",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManager.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawal",
    "inputs": [
      {
        "name": "withdrawal",
        "type": "tuple",
        "internalType": "struct IDelegationManager.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "middlewareTimesIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "completeQueuedWithdrawals",
    "inputs": [
      {
        "name": "withdrawals",
        "type": "tuple[]",
        "internalType": "struct IDelegationManager.Withdrawal[]",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "tokens",
        "type": "address[][]",
        "internalType": "contract IERC20[][]"
      },
      {
        "name": "middlewareTimesIndexes",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "receiveAsTokens",
        "type": "bool[]",
        "internalType": "bool[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "cumulativeWithdrawalsQueued",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "decreaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateTo",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approverSignatureAndExpiry",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateToBySignature",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "stakerSignatureAndExpiry",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "approverSignatureAndExpiry",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "approverSalt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegatedTo",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApprover",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationApproverSaltIsSpent",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "domainSeparator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDelegatableShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorShares",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWithdrawalDelay",
    "inputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "increaseDelegatedShares",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_minWithdrawalDelayBlocks",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "_withdrawalDelayBlocks",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isDelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minWithdrawalDelayBlocks",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "modifyOperatorDetails",
    "inputs": [
      {
        "name": "newOperatorDetails",
        "type": "tuple",
        "internalType": "struct IDelegationManager.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "operatorDetails",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IDelegationManager.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorShares",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pendingWithdrawals",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "queueWithdrawals",
    "inputs": [
      {
        "name": "queuedWithdrawalParams",
        "type": "tuple[]",
        "internalType": "struct IDelegationManager.QueuedWithdrawalParams[]",
        "components": [
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerAsOperator",
    "inputs": [
      {
        "name": "registeringOperatorDetails",
        "type": "tuple",
        "internalType": "struct IDelegationManager.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinWithdrawalDelayBlocks",
    "inputs": [
      {
        "name": "newMinWithdrawalDelayBlocks",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setPauserRegistry",
    "inputs": [
      {
        "name": "newPauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setStrategyWithdrawalDelayBlocks",
    "inputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "withdrawalDelayBlocks",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slasher",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlasher"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakerNonce",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakerOptOutWindowBlocks",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategyManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyWithdrawalDelayBlocks",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "undelegate",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "withdrawalRoots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorMetadataURI",
    "inputs": [
      {
        "name": "metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MinWithdrawalDelayBlocksSet",
    "inputs": [
      {
        "name": "previousValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDetailsModified",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOperatorDetails",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManager.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorMetadataURIUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "metadataURI",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorDetails",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManager.OperatorDetails",
        "components": [
          {
            "name": "__deprecated_earningsReceiver",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegationApprover",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "stakerOptOutWindowBlocks",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesDecreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSharesIncreased",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "staker",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "shares",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PauserRegistrySet",
    "inputs": [
      {
        "name": "pauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "newPauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerDelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerForceUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StakerUndelegated",
    "inputs": [
      {
        "name": "staker",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyWithdrawalDelayBlocksSet",
    "inputs": [
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "previousValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalCompleted",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalQueued",
    "inputs": [
      {
        "name": "withdrawalRoot",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "withdrawal",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDelegationManager.Withdrawal",
        "components": [
          {
            "name": "staker",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedTo",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "withdrawer",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "nonce",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "shares",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod DelegationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101006040523480156200001257600080fd5b5060405162005c4638038062005c46833981016040819052620000359162000140565b6001600160a01b0380841660805280821660c052821660a0526200005862000065565b50504660e0525062000194565b600054610100900460ff1615620000d25760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000125576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200013d57600080fd5b50565b6000806000606084860312156200015657600080fd5b8351620001638162000127565b6020850151909350620001768162000127565b6040850151909250620001898162000127565b809150509250925092565b60805160a05160c05160e051615a1d6200022960003960006126a00152600081816105b10152818161102e015281816113aa01528181611c23015281816129f901528181613eac0152614398015260006107620152600081816104f901528181610ffc0152818161137801528181611cb701528181612ac601528181612c4901528181613fd2015261443e0152615a1d6000f3fe608060405234801561001057600080fd5b50600436106103425760003560e01c8063635bbd10116101b8578063b7f06ebe11610104578063cf80873e116100a2578063f16172b01161007c578063f16172b014610908578063f2fde38b1461091b578063f698da251461092e578063fabc1cbc1461093657600080fd5b8063cf80873e146108c1578063da8be864146108e2578063eea9064b146108f557600080fd5b8063c488375a116100de578063c488375a146107de578063c5e480db146107fe578063c94b5111146108a4578063ca661c04146108b757600080fd5b8063b7f06ebe14610784578063bb45fef2146107a7578063c448feb8146107d557600080fd5b8063886f1195116101715780639104c3191161014b5780639104c3191461070f57806399be81c81461072a578063a17884841461073d578063b13442711461075d57600080fd5b8063886f1195146106cb5780638da5cb5b146106de57806390041347146106ef57600080fd5b8063635bbd101461063657806365da1264146106495780636d70f7ae14610672578063715018a614610685578063778e55f31461068d5780637f548071146106b857600080fd5b806328a573ae116102925780634665bcda11610230578063597b36da1161020a578063597b36da146105e55780635ac86ab7146105f85780635c975abb1461061b57806360d7faed1461062357600080fd5b80634665bcda146105ac5780634fc40b61146105d3578063595c6a67146105dd57600080fd5b806339b70e381161026c57806339b70e38146104f45780633cdeb5e0146105335780633e28391d14610562578063433773821461058557600080fd5b806328a573ae146104ae57806329c77d4f146104c157806333404396146104e157600080fd5b8063132d4967116102ff57806316928365116102d957806316928365146104285780631bbce0911461046157806320606b701461047457806322bf40e41461049b57600080fd5b8063132d4967146103ef578063136439dd146104025780631522bf021461041557600080fd5b80630449ca391461034757806304a4f9791461036d5780630b9f487a146103945780630dd8dd02146103a75780630f589e59146103c757806310d67a2f146103dc575b600080fd5b61035a61035536600461484e565b610949565b6040519081526020015b60405180910390f35b61035a7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b61035a6103a23660046148b4565b6109ce565b6103ba6103b536600461484e565b610a90565b604051610364919061490f565b6103da6103d53660046149ac565b610df9565b005b6103da6103ea3660046149ff565b610f3e565b6103da6103fd366004614a23565b610ff1565b6103da610410366004614a64565b6110a8565b6103da610423366004614a7d565b6111e7565b61035a6104363660046149ff565b6001600160a01b0316600090815260996020526040902060010154600160a01b900463ffffffff1690565b61035a61046f366004614a23565b6111fb565b61035a7f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a86681565b6103da6104a9366004614ae8565b611229565b6103da6104bc366004614a23565b61136d565b61035a6104cf3660046149ff565b609b6020526000908152604090205481565b6103da6104ef366004614b8f565b61141d565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610364565b61051b6105413660046149ff565b6001600160a01b039081166000908152609960205260409020600101541690565b6105756105703660046149ff565b61155a565b6040519015158152602001610364565b61035a7f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b81565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b61035a6213c68081565b6103da61157a565b61035a6105f3366004614e8c565b611641565b610575610606366004614ec8565b606654600160ff9092169190911b9081161490565b60665461035a565b6103da610631366004614ef9565b611671565b6103da610644366004614a64565b61170c565b61051b6106573660046149ff565b609a602052600090815260409020546001600160a01b031681565b6105756106803660046149ff565b61171d565b6103da611757565b61035a61069b366004614f88565b609860209081526000928352604080842090915290825290205481565b6103da6106c6366004615069565b61176b565b60655461051b906001600160a01b031681565b6033546001600160a01b031661051b565b6107026106fd3660046150f9565b611997565b6040516103649190615183565b61051b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b6103da610738366004615196565b611a71565b61035a61074b3660046149ff565b609f6020526000908152604090205481565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b610575610792366004614a64565b609e6020526000908152604090205460ff1681565b6105756107b53660046151cb565b609c60209081526000928352604080842090915290825290205460ff1681565b61035a609d5481565b61035a6107ec3660046149ff565b60a16020526000908152604090205481565b61086e61080c3660046149ff565b6040805160608082018352600080835260208084018290529284018190526001600160a01b03948516815260998352839020835191820184528054851682526001015493841691810191909152600160a01b90920463ffffffff169082015290565b6040805182516001600160a01b039081168252602080850151909116908201529181015163ffffffff1690820152606001610364565b61035a6108b23660046151f7565b611b43565b61035a62034bc081565b6108d46108cf3660046149ff565b611bfc565b604051610364929190615278565b6103ba6108f03660046149ff565b611fb4565b6103da61090336600461529d565b612478565b6103da6109163660046152f5565b612595565b6103da6109293660046149ff565b612626565b61035a61269c565b6103da610944366004614a64565b6126da565b609d54600090815b838110156109c657600060a1600087878581811061097157610971615311565b905060200201602081019061098691906149ff565b6001600160a01b03166001600160a01b03168152602001908152602001600020549050828111156109b5578092505b506109bf8161533d565b9050610951565b509392505050565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad6020808301919091526001600160a01b038681168385015288811660608401528716608083015260a0820185905260c08083018590528351808403909101815260e0909201909252805191012060009081610a4c61269c565b60405161190160f01b602082015260228101919091526042810183905260620160408051808303601f19018152919052805160209091012098975050505050505050565b60665460609060019060029081161415610ac55760405162461bcd60e51b8152600401610abc90615358565b60405180910390fd5b6000836001600160401b03811115610adf57610adf614c31565b604051908082528060200260200182016040528015610b08578160200160208202803683370190505b50336000908152609a60205260408120549192506001600160a01b03909116905b85811015610dee57868682818110610b4357610b43615311565b9050602002810190610b55919061538f565b610b639060208101906153af565b9050878783818110610b7757610b77615311565b9050602002810190610b89919061538f565b610b9390806153af565b905014610c085760405162461bcd60e51b815260206004820152603860248201527f44656c65676174696f6e4d616e616765722e717565756557697468647261776160448201527f6c3a20696e707574206c656e677468206d69736d6174636800000000000000006064820152608401610abc565b33878783818110610c1b57610c1b615311565b9050602002810190610c2d919061538f565b610c3e9060608101906040016149ff565b6001600160a01b031614610cba5760405162461bcd60e51b815260206004820152603c60248201527f44656c65676174696f6e4d616e616765722e717565756557697468647261776160448201527f6c3a2077697468647261776572206d757374206265207374616b6572000000006064820152608401610abc565b610dbf3383898985818110610cd157610cd1615311565b9050602002810190610ce3919061538f565b610cf49060608101906040016149ff565b8a8a86818110610d0657610d06615311565b9050602002810190610d18919061538f565b610d2290806153af565b808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152508e92508d9150889050818110610d6857610d68615311565b9050602002810190610d7a919061538f565b610d889060208101906153af565b8080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061283692505050565b838281518110610dd157610dd1615311565b602090810291909101015280610de68161533d565b915050610b29565b509095945050505050565b610e023361155a565b15610e885760405162461bcd60e51b815260206004820152604a60248201527f44656c65676174696f6e4d616e616765722e726567697374657241734f70657260448201527f61746f723a2063616c6c657220697320616c7265616479206163746976656c796064820152690819195b1959d85d195960b21b608482015260a401610abc565b610e923384612df6565b604080518082019091526060815260006020820152610eb43380836000612fe9565b336001600160a01b03167f8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e285604051610eed91906153f8565b60405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051610f3092919061544a565b60405180910390a250505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f91573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fb59190615479565b6001600160a01b0316336001600160a01b031614610fe55760405162461bcd60e51b8152600401610abc90615496565b610fee8161327f565b50565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806110505750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b61106c5760405162461bcd60e51b8152600401610abc906154e0565b6110758361155a565b156110a3576001600160a01b038084166000908152609a6020526040902054166110a181858585613376565b505b505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156110f0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611114919061553d565b6111305760405162461bcd60e51b8152600401610abc9061555a565b606654818116146111a95760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610abc565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6111ef6133f1565b6110a18484848461344b565b6001600160a01b0383166000908152609b602052604081205461122085828686611b43565b95945050505050565b600054610100900460ff16158080156112495750600054600160ff909116105b806112635750303b158015611263575060005460ff166001145b6112c65760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610abc565b6000805460ff1916600117905580156112e9576000805461ff0019166101001790555b6112f38888613671565b6112fb61375b565b609755611307896137f2565b61131086613844565b61131c8585858561344b565b8015611362576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806113cc5750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b6113e85760405162461bcd60e51b8152600401610abc906154e0565b6113f18361155a565b156110a3576001600160a01b038084166000908152609a6020526040902054166110a18185858561393e565b606654600290600490811614156114465760405162461bcd60e51b8152600401610abc90615358565b600260c95414156114995760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610abc565b600260c95560005b88811015611549576115398a8a838181106114be576114be615311565b90506020028101906114d091906155a2565b8989848181106114e2576114e2615311565b90506020028101906114f491906153af565b89898681811061150657611506615311565b9050602002013588888781811061151f5761151f615311565b905060200201602081019061153491906155b8565b6139b9565b6115428161533d565b90506114a1565b5050600160c9555050505050505050565b6001600160a01b039081166000908152609a602052604090205416151590565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156115c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115e6919061553d565b6116025760405162461bcd60e51b8152600401610abc9061555a565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6000816040516020016116549190615649565b604051602081830303815290604052805190602001209050919050565b6066546002906004908116141561169a5760405162461bcd60e51b8152600401610abc90615358565b600260c95414156116ed5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610abc565b600260c9556116ff86868686866139b9565b5050600160c95550505050565b6117146133f1565b610fee81613844565b60006001600160a01b0382161580159061175157506001600160a01b038083166000818152609a6020526040902054909116145b92915050565b61175f6133f1565b61176960006137f2565b565b42836020015110156117ef5760405162461bcd60e51b815260206004820152604160248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a207374616b6572207369676e6174757265206578706972656064820152601960fa1b608482015260a401610abc565b6117f88561155a565b156118815760405162461bcd60e51b815260206004820152604d60248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a207374616b657220697320616c726561647920616374697660648201526c195b1e4819195b1959d85d1959609a1b608482015260a401610abc565b61188a8461171d565b6119165760405162461bcd60e51b815260206004820152605160248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a206f70657261746f72206973206e6f7420726567697374656064820152703932b21034b71022b4b3b2b72630bcb2b960791b608482015260a401610abc565b6000609b6000876001600160a01b03166001600160a01b0316815260200190815260200160002054905060006119528783888860200151611b43565b6001600160a01b0388166000908152609b60205260409020600184019055855190915061198290889083906141a3565b61198e87878686612fe9565b50505050505050565b6060600082516001600160401b038111156119b4576119b4614c31565b6040519080825280602002602001820160405280156119dd578160200160208202803683370190505b50905060005b83518110156109c6576001600160a01b03851660009081526098602052604081208551909190869084908110611a1b57611a1b615311565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002054828281518110611a5657611a56615311565b6020908102919091010152611a6a8161533d565b90506119e3565b611a7a3361171d565b611afc5760405162461bcd60e51b815260206004820152604760248201527f44656c65676174696f6e4d616e616765722e7570646174654f70657261746f7260448201527f4d657461646174615552493a2063616c6c6572206d75737420626520616e206f6064820152663832b930ba37b960c91b608482015260a401610abc565b336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051611b3792919061544a565b60405180910390a25050565b604080517f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b6020808301919091526001600160a01b0387811683850152851660608301526080820186905260a08083018590528351808403909101815260c0909201909252805191012060009081611bb961269c565b60405161190160f01b602082015260228101919091526042810183905260620160408051808303601f190181529190528051602090910120979650505050505050565b6040516360f4062b60e01b81526001600160a01b03828116600483015260609182916000917f0000000000000000000000000000000000000000000000000000000000000000909116906360f4062b90602401602060405180830381865afa158015611c6c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c90919061565c565b6040516394f649dd60e01b81526001600160a01b03868116600483015291925060009182917f0000000000000000000000000000000000000000000000000000000000000000909116906394f649dd90602401600060405180830381865afa158015611d00573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d2891908101906156d0565b9150915060008313611d3f57909590945092505050565b606080835160001415611df9576040805160018082528183019092529060208083019080368337505060408051600180825281830190925292945090506020808301908036833701905050905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac082600081518110611db457611db4615311565b60200260200101906001600160a01b031690816001600160a01b0316815250508481600081518110611de857611de8615311565b602002602001018181525050611fa7565b8351611e0690600161578a565b6001600160401b03811115611e1d57611e1d614c31565b604051908082528060200260200182016040528015611e46578160200160208202803683370190505b50915081516001600160401b03811115611e6257611e62614c31565b604051908082528060200260200182016040528015611e8b578160200160208202803683370190505b50905060005b8451811015611f2557848181518110611eac57611eac615311565b6020026020010151838281518110611ec657611ec6615311565b60200260200101906001600160a01b031690816001600160a01b031681525050838181518110611ef857611ef8615311565b6020026020010151828281518110611f1257611f12615311565b6020908102919091010152600101611e91565b5073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08260018451611f4a91906157a2565b81518110611f5a57611f5a615311565b60200260200101906001600160a01b031690816001600160a01b031681525050848160018451611f8a91906157a2565b81518110611f9a57611f9a615311565b6020026020010181815250505b9097909650945050505050565b60665460609060019060029081161415611fe05760405162461bcd60e51b8152600401610abc90615358565b611fe98361155a565b6120695760405162461bcd60e51b8152602060048201526044602482018190527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a207374908201527f616b6572206d7573742062652064656c65676174656420746f20756e64656c656064820152636761746560e01b608482015260a401610abc565b6120728361171d565b156120e55760405162461bcd60e51b815260206004820152603d60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a206f7060448201527f657261746f72732063616e6e6f7420626520756e64656c6567617465640000006064820152608401610abc565b6001600160a01b0383166121615760405162461bcd60e51b815260206004820152603c60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a20636160448201527f6e6e6f7420756e64656c6567617465207a65726f2061646472657373000000006064820152608401610abc565b6001600160a01b038084166000818152609a6020526040902054909116903314806121945750336001600160a01b038216145b806121bb57506001600160a01b038181166000908152609960205260409020600101541633145b61222d5760405162461bcd60e51b815260206004820152603d60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a20636160448201527f6c6c65722063616e6e6f7420756e64656c6567617465207374616b65720000006064820152608401610abc565b60008061223986611bfc565b9092509050336001600160a01b0387161461228f57826001600160a01b0316866001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a35b826001600160a01b0316866001600160a01b03167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467660405160405180910390a36001600160a01b0386166000908152609a6020526040902080546001600160a01b0319169055815161231157604080516000815260208101909152945061246f565b81516001600160401b0381111561232a5761232a614c31565b604051908082528060200260200182016040528015612353578160200160208202803683370190505b50945060005b825181101561246d576040805160018082528183019092526000916020808301908036833750506040805160018082528183019092529293506000929150602080830190803683370190505090508483815181106123b9576123b9615311565b6020026020010151826000815181106123d4576123d4615311565b60200260200101906001600160a01b031690816001600160a01b03168152505083838151811061240657612406615311565b60200260200101518160008151811061242157612421615311565b60200260200101818152505061243a89878b8585612836565b88848151811061244c5761244c615311565b602002602001018181525050505080806124659061533d565b915050612359565b505b50505050919050565b6124813361155a565b156124ff5760405162461bcd60e51b815260206004820152604260248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f3a20737460448201527f616b657220697320616c7265616479206163746976656c792064656c65676174606482015261195960f21b608482015260a401610abc565b6125088361171d565b6125895760405162461bcd60e51b815260206004820152604660248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f3a206f7060448201527f657261746f72206973206e6f74207265676973746572656420696e2045696765606482015265372630bcb2b960d11b608482015260a401610abc565b6110a333848484612fe9565b61259e3361171d565b61261c5760405162461bcd60e51b815260206004820152604360248201527f44656c65676174696f6e4d616e616765722e6d6f646966794f70657261746f7260448201527f44657461696c733a2063616c6c6572206d75737420626520616e206f706572616064820152623a37b960e91b608482015260a401610abc565b610fee3382612df6565b61262e6133f1565b6001600160a01b0381166126935760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610abc565b610fee816137f2565b60007f00000000000000000000000000000000000000000000000000000000000000004614156126cd575060975490565b6126d561375b565b905090565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561272d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127519190615479565b6001600160a01b0316336001600160a01b0316146127815760405162461bcd60e51b8152600401610abc90615496565b6066541981196066541916146127ff5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610abc565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016111dc565b60006001600160a01b0386166128cd5760405162461bcd60e51b815260206004820152605060248201527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448201527f6e6451756575655769746864726177616c3a207374616b65722063616e6e6f7460648201526f206265207a65726f206164647265737360801b608482015260a401610abc565b82516129575760405162461bcd60e51b815260206004820152604d60248201527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448201527f6e6451756575655769746864726177616c3a207374726174656769657320636160648201526c6e6e6f7420626520656d70747960981b608482015260a401610abc565b60005b8351811015612d04576001600160a01b038616156129b0576129b0868886848151811061298957612989615311565b60200260200101518685815181106129a3576129a3615311565b6020026020010151613376565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b03168482815181106129e0576129e0615311565b60200260200101516001600160a01b03161415612aa9577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663beffbb8988858481518110612a3957612a39615311565b60200260200101516040518363ffffffff1660e01b8152600401612a729291906001600160a01b03929092168252602082015260400190565b600060405180830381600087803b158015612a8c57600080fd5b505af1158015612aa0573d6000803e3d6000fd5b50505050612cfc565b846001600160a01b0316876001600160a01b03161480612b7b57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639b4da03d858381518110612b0557612b05615311565b60200260200101516040518263ffffffff1660e01b8152600401612b3891906001600160a01b0391909116815260200190565b602060405180830381865afa158015612b55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b79919061553d565b155b612c475760405162461bcd60e51b8152602060048201526084602482018190527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448301527f6e6451756575655769746864726177616c3a2077697468647261776572206d7560648301527f73742062652073616d652061646472657373206173207374616b657220696620908201527f746869726450617274795472616e7366657273466f7262696464656e2061726560a482015263081cd95d60e21b60c482015260e401610abc565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638c80d4e588868481518110612c8957612c89615311565b6020026020010151868581518110612ca357612ca3615311565b60200260200101516040518463ffffffff1660e01b8152600401612cc9939291906157b9565b600060405180830381600087803b158015612ce357600080fd5b505af1158015612cf7573d6000803e3d6000fd5b505050505b60010161295a565b506001600160a01b0386166000908152609f60205260408120805491829190612d2c8361533d565b919050555060006040518060e00160405280896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018381526020014363ffffffff1681526020018681526020018581525090506000612d9482611641565b6000818152609e602052604090819020805460ff19166001179055519091507f9009ab153e8014fbfb02f2217f5cde7aa7f9ad734ae85ca3ee3f4ca2fdd499f990612de290839085906157dd565b60405180910390a198975050505050505050565b6213c680612e0a60608301604084016157f6565b63ffffffff161115612ebf5760405162461bcd60e51b815260206004820152606c60248201527f44656c65676174696f6e4d616e616765722e5f7365744f70657261746f72446560448201527f7461696c733a207374616b65724f70744f757457696e646f77426c6f636b732060648201527f63616e6e6f74206265203e204d41585f5354414b45525f4f50545f4f55545f5760848201526b494e444f575f424c4f434b5360a01b60a482015260c401610abc565b6001600160a01b0382166000908152609960205260409081902060010154600160a01b900463ffffffff1690612efb90606084019084016157f6565b63ffffffff161015612f915760405162461bcd60e51b815260206004820152605360248201527f44656c65676174696f6e4d616e616765722e5f7365744f70657261746f72446560448201527f7461696c733a207374616b65724f70744f757457696e646f77426c6f636b732060648201527218d85b9b9bdd08189948191958dc99585cd959606a1b608482015260a401610abc565b6001600160a01b03821660009081526099602052604090208190612fb58282615833565b505060405133907ffebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac90611b379084906153f8565b606654600090600190811614156130125760405162461bcd60e51b8152600401610abc90615358565b6001600160a01b038085166000908152609960205260409020600101541680158015906130485750336001600160a01b03821614155b801561305d5750336001600160a01b03861614155b156131ca5742846020015110156130dc5760405162461bcd60e51b815260206004820152603760248201527f44656c65676174696f6e4d616e616765722e5f64656c65676174653a2061707060448201527f726f766572207369676e617475726520657870697265640000000000000000006064820152608401610abc565b6001600160a01b0381166000908152609c6020908152604080832086845290915290205460ff16156131765760405162461bcd60e51b815260206004820152603760248201527f44656c65676174696f6e4d616e616765722e5f64656c65676174653a2061707060448201527f726f76657253616c7420616c7265616479207370656e740000000000000000006064820152608401610abc565b6001600160a01b0381166000908152609c6020908152604080832086845282528220805460ff191660011790558501516131b79088908890859088906109ce565b90506131c8828287600001516141a3565b505b6001600160a01b038681166000818152609a602052604080822080546001600160a01b031916948a169485179055517fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049190a360008061322988611bfc565b9150915060005b825181101561136257613277888a85848151811061325057613250615311565b602002602001015185858151811061326a5761326a615311565b602002602001015161393e565b600101613230565b6001600160a01b03811661330d5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610abc565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038085166000908152609860209081526040808320938616835292905290812080548392906133ad9084906157a2565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051610f30939291906157b9565b6033546001600160a01b031633146117695760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610abc565b8281146134d35760405162461bcd60e51b815260206004820152604a60248201527f44656c65676174696f6e4d616e616765722e5f7365745374726174656779576960448201527f746864726177616c44656c6179426c6f636b733a20696e707574206c656e67746064820152690d040dad2e6dac2e8c6d60b31b608482015260a401610abc565b8260005b818110156136695760008686838181106134f3576134f3615311565b905060200201602081019061350891906149ff565b6001600160a01b038116600090815260a1602052604081205491925086868581811061353657613536615311565b90506020020135905062034bc08111156135fa5760405162461bcd60e51b815260206004820152607360248201527f44656c65676174696f6e4d616e616765722e5f7365745374726174656779576960448201527f746864726177616c44656c6179426c6f636b733a205f7769746864726177616c60648201527f44656c6179426c6f636b732063616e6e6f74206265203e204d41585f5749544860848201527244524157414c5f44454c41595f424c4f434b5360681b60a482015260c401610abc565b6001600160a01b038316600081815260a160209081526040918290208490558151928352820184905281018290527f0e7efa738e8b0ce6376a0c1af471655540d2e9a81647d7b09ed823018426576d9060600160405180910390a1505050806136629061533d565b90506134d7565b505050505050565b6065546001600160a01b031615801561369257506001600160a01b03821615155b6137145760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610abc565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26137578261327f565b5050565b604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b62034bc08111156138fd5760405162461bcd60e51b815260206004820152607160248201527f44656c65676174696f6e4d616e616765722e5f7365744d696e5769746864726160448201527f77616c44656c6179426c6f636b733a205f6d696e5769746864726177616c446560648201527f6c6179426c6f636b732063616e6e6f74206265203e204d41585f5749544844526084820152704157414c5f44454c41595f424c4f434b5360781b60a482015260c401610abc565b609d5460408051918252602082018390527fafa003cd76f87ff9d62b35beea889920f33c0c42b8d45b74954d61d50f4b6b69910160405180910390a1609d55565b6001600160a01b0380851660009081526098602090815260408083209386168352929052908120805483929061397590849061578a565b92505081905550836001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c848484604051610f30939291906157b9565b60006139c76105f387615896565b6000818152609e602052604090205490915060ff16613a485760405162461bcd60e51b815260206004820152604360248201526000805160206159c883398151915260448201527f645769746864726177616c3a20616374696f6e206973206e6f7420696e20717560648201526265756560e81b608482015260a401610abc565b609d544390613a5d60a0890160808a016157f6565b63ffffffff16613a6d919061578a565b1115613af55760405162461bcd60e51b815260206004820152605f60248201526000805160206159c883398151915260448201527f645769746864726177616c3a206d696e5769746864726177616c44656c61794260648201527f6c6f636b7320706572696f6420686173206e6f74207965742070617373656400608482015260a401610abc565b613b0560608701604088016149ff565b6001600160a01b0316336001600160a01b031614613b925760405162461bcd60e51b815260206004820152605060248201526000805160206159c883398151915260448201527f645769746864726177616c3a206f6e6c7920776974686472617765722063616e60648201526f1031b7b6b83632ba329030b1ba34b7b760811b608482015260a401610abc565b8115613c1457613ba560a08701876153af565b85149050613c145760405162461bcd60e51b815260206004820152604260248201526000805160206159c883398151915260448201527f645769746864726177616c3a20696e707574206c656e677468206d69736d61746064820152610c6d60f31b608482015260a401610abc565b6000818152609e60205260409020805460ff191690558115613d795760005b613c4060a08801886153af565b9050811015613d73574360a16000613c5b60a08b018b6153af565b85818110613c6b57613c6b615311565b9050602002016020810190613c8091906149ff565b6001600160a01b03168152602081019190915260400160002054613caa60a08a0160808b016157f6565b63ffffffff16613cba919061578a565b1115613cd85760405162461bcd60e51b8152600401610abc906158a2565b613d6b613ce860208901896149ff565b33613cf660a08b018b6153af565b85818110613d0657613d06615311565b9050602002016020810190613d1b91906149ff565b613d2860c08c018c6153af565b86818110613d3857613d38615311565b905060200201358a8a87818110613d5157613d51615311565b9050602002016020810190613d6691906149ff565b61435d565b600101613c33565b50614168565b336000908152609a60205260408120546001600160a01b0316905b613da160a08901896153af565b9050811015614165574360a16000613dbc60a08c018c6153af565b85818110613dcc57613dcc615311565b9050602002016020810190613de191906149ff565b6001600160a01b03168152602081019190915260400160002054613e0b60a08b0160808c016157f6565b63ffffffff16613e1b919061578a565b1115613e395760405162461bcd60e51b8152600401610abc906158a2565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0613e5b60a08a018a6153af565b83818110613e6b57613e6b615311565b9050602002016020810190613e8091906149ff565b6001600160a01b03161415613fd0576000613e9e60208a018a6149ff565b905060006001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016630e81073c83613edf60c08e018e6153af565b87818110613eef57613eef615311565b6040516001600160e01b031960e087901b1681526001600160a01b03909416600485015260200291909101356024830152506044016020604051808303816000875af1158015613f43573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613f67919061565c565b6001600160a01b038084166000908152609a6020526040902054919250168015613fc857613fc88184613f9d60a08f018f6153af565b88818110613fad57613fad615311565b9050602002016020810190613fc291906149ff565b8561393e565b50505061415d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c4623ea13389898581811061401257614012615311565b905060200201602081019061402791906149ff565b61403460a08d018d6153af565b8681811061404457614044615311565b905060200201602081019061405991906149ff565b61406660c08e018e6153af565b8781811061407657614076615311565b60405160e088901b6001600160e01b03191681526001600160a01b03968716600482015294861660248601529290941660448401526020909102013560648201526084019050600060405180830381600087803b1580156140d657600080fd5b505af11580156140ea573d6000803e3d6000fd5b505050506001600160a01b0382161561415d5761415d823361410f60a08c018c6153af565b8581811061411f5761411f615311565b905060200201602081019061413491906149ff565b61414160c08d018d6153af565b8681811061415157614151615311565b9050602002013561393e565b600101613d94565b50505b6040518181527fc97098c2f658800b4df29001527f7324bcdffcf6e8751a699ab920a1eced5b1d9060200160405180910390a1505050505050565b6001600160a01b0383163b156142bd57604051630b135d3f60e11b808252906001600160a01b03851690631626ba7e906141e3908690869060040161592a565b602060405180830381865afa158015614200573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906142249190615987565b6001600160e01b031916146110a35760405162461bcd60e51b815260206004820152605360248201527f454950313237315369676e61747572655574696c732e636865636b5369676e6160448201527f747572655f454950313237313a2045524331323731207369676e6174757265206064820152721d995c9a599a58d85d1a5bdb8819985a5b1959606a1b608482015260a401610abc565b826001600160a01b03166142d1838361449d565b6001600160a01b0316146110a35760405162461bcd60e51b815260206004820152604760248201527f454950313237315369676e61747572655574696c732e636865636b5369676e6160448201527f747572655f454950313237313a207369676e6174757265206e6f742066726f6d6064820152661039b4b3b732b960c91b608482015260a401610abc565b6001600160a01b03831673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014156144085760405162387b1360e81b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063387b1300906143d1908890889087906004016157b9565b600060405180830381600087803b1580156143eb57600080fd5b505af11580156143ff573d6000803e3d6000fd5b50505050614496565b60405163c608c7f360e01b81526001600160a01b03858116600483015284811660248301526044820184905282811660648301527f0000000000000000000000000000000000000000000000000000000000000000169063c608c7f390608401600060405180830381600087803b15801561448257600080fd5b505af1158015611362573d6000803e3d6000fd5b5050505050565b60008060006144ac85856144b9565b915091506109c681614529565b6000808251604114156144f05760208301516040840151606085015160001a6144e4878285856146e4565b94509450505050614522565b82516040141561451a576020830151604084015161450f8683836147d1565b935093505050614522565b506000905060025b9250929050565b600081600481111561453d5761453d6159b1565b14156145465750565b600181600481111561455a5761455a6159b1565b14156145a85760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610abc565b60028160048111156145bc576145bc6159b1565b141561460a5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610abc565b600381600481111561461e5761461e6159b1565b14156146775760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610abc565b600481600481111561468b5761468b6159b1565b1415610fee5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610abc565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561471b57506000905060036147c8565b8460ff16601b1415801561473357508460ff16601c14155b1561474457506000905060046147c8565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614798573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166147c1576000600192509250506147c8565b9150600090505b94509492505050565b6000806001600160ff1b038316816147ee60ff86901c601b61578a565b90506147fc878288856146e4565b935093505050935093915050565b60008083601f84011261481c57600080fd5b5081356001600160401b0381111561483357600080fd5b6020830191508360208260051b850101111561452257600080fd5b6000806020838503121561486157600080fd5b82356001600160401b0381111561487757600080fd5b6148838582860161480a565b90969095509350505050565b6001600160a01b0381168114610fee57600080fd5b80356148af8161488f565b919050565b600080600080600060a086880312156148cc57600080fd5b85356148d78161488f565b945060208601356148e78161488f565b935060408601356148f78161488f565b94979396509394606081013594506080013592915050565b6020808252825182820181905260009190848201906040850190845b818110156149475783518352928401929184019160010161492b565b50909695505050505050565b60006060828403121561496557600080fd5b50919050565b60008083601f84011261497d57600080fd5b5081356001600160401b0381111561499457600080fd5b60208301915083602082850101111561452257600080fd5b6000806000608084860312156149c157600080fd5b6149cb8585614953565b925060608401356001600160401b038111156149e657600080fd5b6149f28682870161496b565b9497909650939450505050565b600060208284031215614a1157600080fd5b8135614a1c8161488f565b9392505050565b600080600060608486031215614a3857600080fd5b8335614a438161488f565b92506020840135614a538161488f565b929592945050506040919091013590565b600060208284031215614a7657600080fd5b5035919050565b60008060008060408587031215614a9357600080fd5b84356001600160401b0380821115614aaa57600080fd5b614ab68883890161480a565b90965094506020870135915080821115614acf57600080fd5b50614adc8782880161480a565b95989497509550505050565b60008060008060008060008060c0898b031215614b0457600080fd5b8835614b0f8161488f565b97506020890135614b1f8161488f565b9650604089013595506060890135945060808901356001600160401b0380821115614b4957600080fd5b614b558c838d0161480a565b909650945060a08b0135915080821115614b6e57600080fd5b50614b7b8b828c0161480a565b999c989b5096995094979396929594505050565b6000806000806000806000806080898b031215614bab57600080fd5b88356001600160401b0380821115614bc257600080fd5b614bce8c838d0161480a565b909a50985060208b0135915080821115614be757600080fd5b614bf38c838d0161480a565b909850965060408b0135915080821115614c0c57600080fd5b614c188c838d0161480a565b909650945060608b0135915080821115614b6e57600080fd5b634e487b7160e01b600052604160045260246000fd5b60405160e081016001600160401b0381118282101715614c6957614c69614c31565b60405290565b604080519081016001600160401b0381118282101715614c6957614c69614c31565b604051601f8201601f191681016001600160401b0381118282101715614cb957614cb9614c31565b604052919050565b63ffffffff81168114610fee57600080fd5b80356148af81614cc1565b60006001600160401b03821115614cf757614cf7614c31565b5060051b60200190565b600082601f830112614d1257600080fd5b81356020614d27614d2283614cde565b614c91565b82815260059290921b84018101918181019086841115614d4657600080fd5b8286015b84811015614d6a578035614d5d8161488f565b8352918301918301614d4a565b509695505050505050565b600082601f830112614d8657600080fd5b81356020614d96614d2283614cde565b82815260059290921b84018101918181019086841115614db557600080fd5b8286015b84811015614d6a5780358352918301918301614db9565b600060e08284031215614de257600080fd5b614dea614c47565b9050614df5826148a4565b8152614e03602083016148a4565b6020820152614e14604083016148a4565b604082015260608201356060820152614e2f60808301614cd3565b608082015260a08201356001600160401b0380821115614e4e57600080fd5b614e5a85838601614d01565b60a084015260c0840135915080821115614e7357600080fd5b50614e8084828501614d75565b60c08301525092915050565b600060208284031215614e9e57600080fd5b81356001600160401b03811115614eb457600080fd5b614ec084828501614dd0565b949350505050565b600060208284031215614eda57600080fd5b813560ff81168114614a1c57600080fd5b8015158114610fee57600080fd5b600080600080600060808688031215614f1157600080fd5b85356001600160401b0380821115614f2857600080fd5b9087019060e0828a031215614f3c57600080fd5b90955060208701359080821115614f5257600080fd5b50614f5f8882890161480a565b909550935050604086013591506060860135614f7a81614eeb565b809150509295509295909350565b60008060408385031215614f9b57600080fd5b8235614fa68161488f565b91506020830135614fb68161488f565b809150509250929050565b600060408284031215614fd357600080fd5b614fdb614c6f565b905081356001600160401b0380821115614ff457600080fd5b818401915084601f83011261500857600080fd5b813560208282111561501c5761501c614c31565b61502e601f8301601f19168201614c91565b9250818352868183860101111561504457600080fd5b8181850182850137600081838501015282855280860135818601525050505092915050565b600080600080600060a0868803121561508157600080fd5b853561508c8161488f565b9450602086013561509c8161488f565b935060408601356001600160401b03808211156150b857600080fd5b6150c489838a01614fc1565b945060608801359150808211156150da57600080fd5b506150e788828901614fc1565b95989497509295608001359392505050565b6000806040838503121561510c57600080fd5b82356151178161488f565b915060208301356001600160401b0381111561513257600080fd5b61513e85828601614d01565b9150509250929050565b600081518084526020808501945080840160005b838110156151785781518752958201959082019060010161515c565b509495945050505050565b602081526000614a1c6020830184615148565b600080602083850312156151a957600080fd5b82356001600160401b038111156151bf57600080fd5b6148838582860161496b565b600080604083850312156151de57600080fd5b82356151e98161488f565b946020939093013593505050565b6000806000806080858703121561520d57600080fd5b84356152188161488f565b935060208501359250604085013561522f8161488f565b9396929550929360600135925050565b600081518084526020808501945080840160005b838110156151785781516001600160a01b031687529582019590820190600101615253565b60408152600061528b604083018561523f565b82810360208401526112208185615148565b6000806000606084860312156152b257600080fd5b83356152bd8161488f565b925060208401356001600160401b038111156152d857600080fd5b6152e486828701614fc1565b925050604084013590509250925092565b60006060828403121561530757600080fd5b614a1c8383614953565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561535157615351615327565b5060010190565b60208082526019908201527f5061757361626c653a20696e6465782069732070617573656400000000000000604082015260600190565b60008235605e198336030181126153a557600080fd5b9190910192915050565b6000808335601e198436030181126153c657600080fd5b8301803591506001600160401b038211156153e057600080fd5b6020019150600581901b360382131561452257600080fd5b6060810182356154078161488f565b6001600160a01b0390811683526020840135906154238261488f565b166020830152604083013561543781614cc1565b63ffffffff811660408401525092915050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b60006020828403121561548b57600080fd5b8151614a1c8161488f565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b60208082526037908201527f44656c65676174696f6e4d616e616765723a206f6e6c7953747261746567794d60408201527f616e616765724f72456967656e506f644d616e61676572000000000000000000606082015260800190565b60006020828403121561554f57600080fd5b8151614a1c81614eeb565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b6000823560de198336030181126153a557600080fd5b6000602082840312156155ca57600080fd5b8135614a1c81614eeb565b600060018060a01b03808351168452806020840151166020850152806040840151166040850152506060820151606084015263ffffffff608083015116608084015260a082015160e060a085015261563060e085018261523f565b905060c083015184820360c08601526112208282615148565b602081526000614a1c60208301846155d5565b60006020828403121561566e57600080fd5b5051919050565b600082601f83011261568657600080fd5b81516020615696614d2283614cde565b82815260059290921b840181019181810190868411156156b557600080fd5b8286015b84811015614d6a57805183529183019183016156b9565b600080604083850312156156e357600080fd5b82516001600160401b03808211156156fa57600080fd5b818501915085601f83011261570e57600080fd5b8151602061571e614d2283614cde565b82815260059290921b8401810191818101908984111561573d57600080fd5b948201945b838610156157645785516157558161488f565b82529482019490820190615742565b9188015191965090935050508082111561577d57600080fd5b5061513e85828601615675565b6000821982111561579d5761579d615327565b500190565b6000828210156157b4576157b4615327565b500390565b6001600160a01b039384168152919092166020820152604081019190915260600190565b828152604060208201526000614ec060408301846155d5565b60006020828403121561580857600080fd5b8135614a1c81614cc1565b80546001600160a01b0319166001600160a01b0392909216919091179055565b813561583e8161488f565b6158488183615813565b5060018101602083013561585b8161488f565b6158658183615813565b50604083013561587481614cc1565b815463ffffffff60a01b191660a09190911b63ffffffff60a01b161790555050565b60006117513683614dd0565b6020808252606e908201526000805160206159c883398151915260408201527f645769746864726177616c3a207769746864726177616c44656c6179426c6f6360608201527f6b7320706572696f6420686173206e6f74207965742070617373656420666f7260808201526d207468697320737472617465677960901b60a082015260c00190565b82815260006020604081840152835180604085015260005b8181101561595e57858101830151858201606001528201615942565b81811115615970576000606083870101525b50601f01601f191692909201606001949350505050565b60006020828403121561599957600080fd5b81516001600160e01b031981168114614a1c57600080fd5b634e487b7160e01b600052602160045260246000fdfe44656c65676174696f6e4d616e616765722e5f636f6d706c6574655175657565a264697066735822122026b1fed484881843a1d9811e493fabdf693f068a2dfa3af9289c2a7fba74873e64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0\\F8\x03\x80b\0\\F\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x01@V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x80\x82\x16`\xC0R\x82\x16`\xA0Rb\0\0Xb\0\0eV[PPF`\xE0RPb\0\x01\x94V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01%W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01=W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01VW`\0\x80\xFD[\x83Qb\0\x01c\x81b\0\x01'V[` \x85\x01Q\x90\x93Pb\0\x01v\x81b\0\x01'V[`@\x85\x01Q\x90\x92Pb\0\x01\x89\x81b\0\x01'V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0QaZ\x1Db\0\x02)`\09`\0a&\xA0\x01R`\0\x81\x81a\x05\xB1\x01R\x81\x81a\x10.\x01R\x81\x81a\x13\xAA\x01R\x81\x81a\x1C#\x01R\x81\x81a)\xF9\x01R\x81\x81a>\xAC\x01RaC\x98\x01R`\0a\x07b\x01R`\0\x81\x81a\x04\xF9\x01R\x81\x81a\x0F\xFC\x01R\x81\x81a\x13x\x01R\x81\x81a\x1C\xB7\x01R\x81\x81a*\xC6\x01R\x81\x81a,I\x01R\x81\x81a?\xD2\x01RaD>\x01RaZ\x1D`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03BW`\x005`\xE0\x1C\x80cc[\xBD\x10\x11a\x01\xB8W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\x08W\x80c\xF2\xFD\xE3\x8B\x14a\t\x1BW\x80c\xF6\x98\xDA%\x14a\t.W\x80c\xFA\xBC\x1C\xBC\x14a\t6W`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\x08\xC1W\x80c\xDA\x8B\xE8d\x14a\x08\xE2W\x80c\xEE\xA9\x06K\x14a\x08\xF5W`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x07\xDEW\x80c\xC5\xE4\x80\xDB\x14a\x07\xFEW\x80c\xC9KQ\x11\x14a\x08\xA4W\x80c\xCAf\x1C\x04\x14a\x08\xB7W`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\x84W\x80c\xBBE\xFE\xF2\x14a\x07\xA7W\x80c\xC4H\xFE\xB8\x14a\x07\xD5W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07\x0FW\x80c\x99\xBE\x81\xC8\x14a\x07*W\x80c\xA1x\x84\x84\x14a\x07=W\x80c\xB14Bq\x14a\x07]W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x06\xCBW\x80c\x8D\xA5\xCB[\x14a\x06\xDEW\x80c\x90\x04\x13G\x14a\x06\xEFW`\0\x80\xFD[\x80cc[\xBD\x10\x14a\x066W\x80ce\xDA\x12d\x14a\x06IW\x80cmp\xF7\xAE\x14a\x06rW\x80cqP\x18\xA6\x14a\x06\x85W\x80cw\x8EU\xF3\x14a\x06\x8DW\x80c\x7FT\x80q\x14a\x06\xB8W`\0\x80\xFD[\x80c(\xA5s\xAE\x11a\x02\x92W\x80cFe\xBC\xDA\x11a\x020W\x80cY{6\xDA\x11a\x02\nW\x80cY{6\xDA\x14a\x05\xE5W\x80cZ\xC8j\xB7\x14a\x05\xF8W\x80c\\\x97Z\xBB\x14a\x06\x1BW\x80c`\xD7\xFA\xED\x14a\x06#W`\0\x80\xFD[\x80cFe\xBC\xDA\x14a\x05\xACW\x80cO\xC4\x0Ba\x14a\x05\xD3W\x80cY\\jg\x14a\x05\xDDW`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02lW\x80c9\xB7\x0E8\x14a\x04\xF4W\x80c<\xDE\xB5\xE0\x14a\x053W\x80c>(9\x1D\x14a\x05bW\x80cC7s\x82\x14a\x05\x85W`\0\x80\xFD[\x80c(\xA5s\xAE\x14a\x04\xAEW\x80c)\xC7}O\x14a\x04\xC1W\x80c3@C\x96\x14a\x04\xE1W`\0\x80\xFD[\x80c\x13-Ig\x11a\x02\xFFW\x80c\x16\x92\x83e\x11a\x02\xD9W\x80c\x16\x92\x83e\x14a\x04(W\x80c\x1B\xBC\xE0\x91\x14a\x04aW\x80c `kp\x14a\x04tW\x80c\"\xBF@\xE4\x14a\x04\x9BW`\0\x80\xFD[\x80c\x13-Ig\x14a\x03\xEFW\x80c\x13d9\xDD\x14a\x04\x02W\x80c\x15\"\xBF\x02\x14a\x04\x15W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03GW\x80c\x04\xA4\xF9y\x14a\x03mW\x80c\x0B\x9FHz\x14a\x03\x94W\x80c\r\xD8\xDD\x02\x14a\x03\xA7W\x80c\x0FX\x9EY\x14a\x03\xC7W\x80c\x10\xD6z/\x14a\x03\xDCW[`\0\x80\xFD[a\x03Za\x03U6`\x04aHNV[a\tIV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Z\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[a\x03Za\x03\xA26`\x04aH\xB4V[a\t\xCEV[a\x03\xBAa\x03\xB56`\x04aHNV[a\n\x90V[`@Qa\x03d\x91\x90aI\x0FV[a\x03\xDAa\x03\xD56`\x04aI\xACV[a\r\xF9V[\0[a\x03\xDAa\x03\xEA6`\x04aI\xFFV[a\x0F>V[a\x03\xDAa\x03\xFD6`\x04aJ#V[a\x0F\xF1V[a\x03\xDAa\x04\x106`\x04aJdV[a\x10\xA8V[a\x03\xDAa\x04#6`\x04aJ}V[a\x11\xE7V[a\x03Za\x0466`\x04aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03Za\x04o6`\x04aJ#V[a\x11\xFBV[a\x03Z\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xDAa\x04\xA96`\x04aJ\xE8V[a\x12)V[a\x03\xDAa\x04\xBC6`\x04aJ#V[a\x13mV[a\x03Za\x04\xCF6`\x04aI\xFFV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xDAa\x04\xEF6`\x04aK\x8FV[a\x14\x1DV[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03dV[a\x05\x1Ba\x05A6`\x04aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05ua\x05p6`\x04aI\xFFV[a\x15ZV[`@Q\x90\x15\x15\x81R` \x01a\x03dV[a\x03Z\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Zb\x13\xC6\x80\x81V[a\x03\xDAa\x15zV[a\x03Za\x05\xF36`\x04aN\x8CV[a\x16AV[a\x05ua\x06\x066`\x04aN\xC8V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03ZV[a\x03\xDAa\x0616`\x04aN\xF9V[a\x16qV[a\x03\xDAa\x06D6`\x04aJdV[a\x17\x0CV[a\x05\x1Ba\x06W6`\x04aI\xFFV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05ua\x06\x806`\x04aI\xFFV[a\x17\x1DV[a\x03\xDAa\x17WV[a\x03Za\x06\x9B6`\x04aO\x88V[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xDAa\x06\xC66`\x04aPiV[a\x17kV[`eTa\x05\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05\x1BV[a\x07\x02a\x06\xFD6`\x04aP\xF9V[a\x19\x97V[`@Qa\x03d\x91\x90aQ\x83V[a\x05\x1Bs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xDAa\x0786`\x04aQ\x96V[a\x1AqV[a\x03Za\x07K6`\x04aI\xFFV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05ua\x07\x926`\x04aJdV[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05ua\x07\xB56`\x04aQ\xCBV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03Z`\x9DT\x81V[a\x03Za\x07\xEC6`\x04aI\xFFV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08na\x08\x0C6`\x04aI\xFFV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03dV[a\x03Za\x08\xB26`\x04aQ\xF7V[a\x1BCV[a\x03Zb\x03K\xC0\x81V[a\x08\xD4a\x08\xCF6`\x04aI\xFFV[a\x1B\xFCV[`@Qa\x03d\x92\x91\x90aRxV[a\x03\xBAa\x08\xF06`\x04aI\xFFV[a\x1F\xB4V[a\x03\xDAa\t\x036`\x04aR\x9DV[a$xV[a\x03\xDAa\t\x166`\x04aR\xF5V[a%\x95V[a\x03\xDAa\t)6`\x04aI\xFFV[a&&V[a\x03Za&\x9CV[a\x03\xDAa\tD6`\x04aJdV[a&\xDAV[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\t\xC6W`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\tqWa\tqaS\x11V[\x90P` \x02\x01` \x81\x01\x90a\t\x86\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\t\xB5W\x80\x92P[Pa\t\xBF\x81aS=V[\x90Pa\tQV[P\x93\x92PPPV[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\nLa&\x9CV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\n\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xDFWa\n\xDFaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\r\xEEW\x86\x86\x82\x81\x81\x10a\x0BCWa\x0BCaS\x11V[\x90P` \x02\x81\x01\x90a\x0BU\x91\x90aS\x8FV[a\x0Bc\x90` \x81\x01\x90aS\xAFV[\x90P\x87\x87\x83\x81\x81\x10a\x0BwWa\x0BwaS\x11V[\x90P` \x02\x81\x01\x90a\x0B\x89\x91\x90aS\x8FV[a\x0B\x93\x90\x80aS\xAFV[\x90P\x14a\x0C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[3\x87\x87\x83\x81\x81\x10a\x0C\x1BWa\x0C\x1BaS\x11V[\x90P` \x02\x81\x01\x90a\x0C-\x91\x90aS\x8FV[a\x0C>\x90``\x81\x01\x90`@\x01aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: withdrawer must be staker\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\r\xBF3\x83\x89\x89\x85\x81\x81\x10a\x0C\xD1Wa\x0C\xD1aS\x11V[\x90P` \x02\x81\x01\x90a\x0C\xE3\x91\x90aS\x8FV[a\x0C\xF4\x90``\x81\x01\x90`@\x01aI\xFFV[\x8A\x8A\x86\x81\x81\x10a\r\x06Wa\r\x06aS\x11V[\x90P` \x02\x81\x01\x90a\r\x18\x91\x90aS\x8FV[a\r\"\x90\x80aS\xAFV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\rhWa\rhaS\x11V[\x90P` \x02\x81\x01\x90a\rz\x91\x90aS\x8FV[a\r\x88\x90` \x81\x01\x90aS\xAFV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(6\x92PPPV[\x83\x82\x81Q\x81\x10a\r\xD1Wa\r\xD1aS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\r\xE6\x81aS=V[\x91PPa\x0B)V[P\x90\x95\x94PPPPPV[a\x0E\x023a\x15ZV[\x15a\x0E\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: caller is already actively`d\x82\x01Ri\x08\x19\x19[\x19Y\xD8]\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x0E\x923\x84a-\xF6V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0E\xB43\x80\x83`\0a/\xE9V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0E\xED\x91\x90aS\xF8V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F0\x92\x91\x90aTJV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB5\x91\x90aTyV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\x96V[a\x0F\xEE\x81a2\x7FV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10PWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\xE0V[a\x10u\x83a\x15ZV[\x15a\x10\xA3W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x10\xA1\x81\x85\x85\x85a3vV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x14\x91\x90aU=V[a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aUZV[`fT\x81\x81\x16\x14a\x11\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x11\xEFa3\xF1V[a\x10\xA1\x84\x84\x84\x84a4KV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12 \x85\x82\x86\x86a\x1BCV[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12IWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12cWP0;\x15\x80\x15a\x12cWP`\0T`\xFF\x16`\x01\x14[a\x12\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xE9W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xF3\x88\x88a6qV[a\x12\xFBa7[V[`\x97Ua\x13\x07\x89a7\xF2V[a\x13\x10\x86a8DV[a\x13\x1C\x85\x85\x85\x85a4KV[\x80\x15a\x13bW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13\xCCWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\xE0V[a\x13\xF1\x83a\x15ZV[\x15a\x10\xA3W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x10\xA1\x81\x85\x85\x85a9>V[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x02`\xC9T\x14\x15a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xBCV[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15IWa\x159\x8A\x8A\x83\x81\x81\x10a\x14\xBEWa\x14\xBEaS\x11V[\x90P` \x02\x81\x01\x90a\x14\xD0\x91\x90aU\xA2V[\x89\x89\x84\x81\x81\x10a\x14\xE2Wa\x14\xE2aS\x11V[\x90P` \x02\x81\x01\x90a\x14\xF4\x91\x90aS\xAFV[\x89\x89\x86\x81\x81\x10a\x15\x06Wa\x15\x06aS\x11V[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x1FWa\x15\x1FaS\x11V[\x90P` \x02\x01` \x81\x01\x90a\x154\x91\x90aU\xB8V[a9\xB9V[a\x15B\x81aS=V[\x90Pa\x14\xA1V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE6\x91\x90aU=V[a\x16\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aUZV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16T\x91\x90aVIV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x16\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x02`\xC9T\x14\x15a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xBCV[`\x02`\xC9Ua\x16\xFF\x86\x86\x86\x86\x86a9\xB9V[PP`\x01`\xC9UPPPPV[a\x17\x14a3\xF1V[a\x0F\xEE\x81a8DV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x17QWP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[a\x17_a3\xF1V[a\x17i`\0a7\xF2V[V[B\x83` \x01Q\x10\x15a\x17\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x17\xF8\x85a\x15ZV[\x15a\x18\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker is already activ`d\x82\x01Rl\x19[\x1EH\x19\x19[\x19Y\xD8]\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x18\x8A\x84a\x17\x1DV[a\x19\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: operator is not registe`d\x82\x01Rp92\xB2\x104\xB7\x10\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`y\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x19R\x87\x83\x88\x88` \x01Qa\x1BCV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x19\x82\x90\x88\x90\x83\x90aA\xA3V[a\x19\x8E\x87\x87\x86\x86a/\xE9V[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xB4Wa\x19\xB4aL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\t\xC6W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1A\x1BWa\x1A\x1BaS\x11V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1AVWa\x1AVaS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1Aj\x81aS=V[\x90Pa\x19\xE3V[a\x1Az3a\x17\x1DV[a\x1A\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1B7\x92\x91\x90aTJV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1B\xB9a&\x9CV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x90\x91\x90aV\\V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D(\x91\x90\x81\x01\x90aV\xD0V[\x91P\x91P`\0\x83\x13a\x1D?W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1D\xF9W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1D\xB4Wa\x1D\xB4aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1D\xE8Wa\x1D\xE8aS\x11V[` \x02` \x01\x01\x81\x81RPPa\x1F\xA7V[\x83Qa\x1E\x06\x90`\x01aW\x8AV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x1DWa\x1E\x1DaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EbWa\x1EbaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x8BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a\x1F%W\x84\x81\x81Q\x81\x10a\x1E\xACWa\x1E\xACaS\x11V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1E\xC6Wa\x1E\xC6aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a\x1E\xF8Wa\x1E\xF8aS\x11V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\x12Wa\x1F\x12aS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1E\x91V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa\x1FJ\x91\x90aW\xA2V[\x81Q\x81\x10a\x1FZWa\x1FZaS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa\x1F\x8A\x91\x90aW\xA2V[\x81Q\x81\x10a\x1F\x9AWa\x1F\x9AaS\x11V[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[a\x1F\xE9\x83a\x15ZV[a iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a r\x83a\x17\x1DV[\x15a \xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16a!aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a!\x94WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a!\xBBWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a\"-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80a\"9\x86a\x1B\xFCV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a\"\x8FW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa#\x11W`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa$oV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#*Wa#*aL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a$mW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a#\xB9Wa#\xB9aS\x11V[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a#\xD4Wa#\xD4aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a$\x06Wa$\x06aS\x11V[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a$!Wa$!aS\x11V[` \x02` \x01\x01\x81\x81RPPa$:\x89\x87\x8B\x85\x85a(6V[\x88\x84\x81Q\x81\x10a$LWa$LaS\x11V[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a$e\x90aS=V[\x91PPa#YV[P[PPPP\x91\x90PV[a$\x813a\x15ZV[\x15a$\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FDelegationManager.delegateTo: st`D\x82\x01R\x7Faker is already actively delegat`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a%\x08\x83a\x17\x1DV[a%\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FDelegationManager.delegateTo: op`D\x82\x01R\x7Ferator is not registered in Eige`d\x82\x01Re7&0\xBC\xB2\xB9`\xD1\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x10\xA33\x84\x84\x84a/\xE9V[a%\x9E3a\x17\x1DV[a&\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x0F\xEE3\x82a-\xF6V[a&.a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a\x0F\xEE\x81a7\xF2V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a&\xCDWP`\x97T\x90V[a&\xD5a7[V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'Q\x91\x90aTyV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\x96V[`fT\x19\x81\x19`fT\x19\x16\x14a'\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x11\xDCV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a(\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82Qa)WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0[\x83Q\x81\x10\x15a-\x04W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a)\xB0Wa)\xB0\x86\x88\x86\x84\x81Q\x81\x10a)\x89Wa)\x89aS\x11V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a)\xA3Wa)\xA3aS\x11V[` \x02` \x01\x01Qa3vV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a)\xE0Wa)\xE0aS\x11V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a*\xA9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*9Wa*9aS\x11V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*r\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xA0W=`\0\x80>=`\0\xFD[PPPPa,\xFCV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a+{WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\x05Wa+\x05aS\x11V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+8\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+y\x91\x90aU=V[\x15[a,GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\n\xBCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a,\x89Wa,\x89aS\x11V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a,\xA3Wa,\xA3aS\x11V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xC9\x93\x92\x91\x90aW\xB9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\xF7W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a)ZV[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a-,\x83aS=V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a-\x94\x82a\x16AV[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a-\xE2\x90\x83\x90\x85\x90aW\xDDV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[b\x13\xC6\x80a.\n``\x83\x01`@\x84\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16\x11\x15a.\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a.\xFB\x90``\x84\x01\x90\x84\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16\x10\x15a/\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a/\xB5\x82\x82aX3V[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1B7\x90\x84\x90aS\xF8V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a0\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a0HWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a0]WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a1\xCAWB\x84` \x01Q\x10\x15a0\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a1vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa1\xB7\x90\x88\x90\x88\x90\x85\x90\x88\x90a\t\xCEV[\x90Pa1\xC8\x82\x82\x87`\0\x01QaA\xA3V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a2)\x88a\x1B\xFCV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13bWa2w\x88\x8A\x85\x84\x81Q\x81\x10a2PWa2PaS\x11V[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a2jWa2jaS\x11V[` \x02` \x01\x01Qa9>V[`\x01\x01a20V[`\x01`\x01`\xA0\x1B\x03\x81\x16a3\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a3\xAD\x90\x84\x90aW\xA2V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F0\x93\x92\x91\x90aW\xB9V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\xBCV[\x82\x81\x14a4\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\0[\x81\x81\x10\x15a6iW`\0\x86\x86\x83\x81\x81\x10a4\xF3Wa4\xF3aS\x11V[\x90P` \x02\x01` \x81\x01\x90a5\x08\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a56Wa56aS\x11V[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a5\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a6b\x90aS=V[\x90Pa4\xD7V[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a6\x92WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a7\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a7W\x82a2\x7FV[PPV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a8\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`q`$\x82\x01R\x7FDelegationManager._setMinWithdra`D\x82\x01R\x7FwalDelayBlocks: _minWithdrawalDe`d\x82\x01R\x7FlayBlocks cannot be > MAX_WITHDR`\x84\x82\x01RpAWAL_DELAY_BLOCKS`x\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a9u\x90\x84\x90aW\x8AV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F0\x93\x92\x91\x90aW\xB9V[`\0a9\xC7a\x05\xF3\x87aX\x96V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a:HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x9DTC\x90a:]`\xA0\x89\x01`\x80\x8A\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a:m\x91\x90aW\x8AV[\x11\x15a:\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[a;\x05``\x87\x01`@\x88\x01aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a;\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x81\x15a<\x14Wa;\xA5`\xA0\x87\x01\x87aS\xAFV[\x85\x14\x90Pa<\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a=yW`\0[a<@`\xA0\x88\x01\x88aS\xAFV[\x90P\x81\x10\x15a=sWC`\xA1`\0a<[`\xA0\x8B\x01\x8BaS\xAFV[\x85\x81\x81\x10a<kWa<kaS\x11V[\x90P` \x02\x01` \x81\x01\x90a<\x80\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta<\xAA`\xA0\x8A\x01`\x80\x8B\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a<\xBA\x91\x90aW\x8AV[\x11\x15a<\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aX\xA2V[a=ka<\xE8` \x89\x01\x89aI\xFFV[3a<\xF6`\xA0\x8B\x01\x8BaS\xAFV[\x85\x81\x81\x10a=\x06Wa=\x06aS\x11V[\x90P` \x02\x01` \x81\x01\x90a=\x1B\x91\x90aI\xFFV[a=(`\xC0\x8C\x01\x8CaS\xAFV[\x86\x81\x81\x10a=8Wa=8aS\x11V[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a=QWa=QaS\x11V[\x90P` \x02\x01` \x81\x01\x90a=f\x91\x90aI\xFFV[aC]V[`\x01\x01a<3V[PaAhV[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a=\xA1`\xA0\x89\x01\x89aS\xAFV[\x90P\x81\x10\x15aAeWC`\xA1`\0a=\xBC`\xA0\x8C\x01\x8CaS\xAFV[\x85\x81\x81\x10a=\xCCWa=\xCCaS\x11V[\x90P` \x02\x01` \x81\x01\x90a=\xE1\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta>\x0B`\xA0\x8B\x01`\x80\x8C\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a>\x1B\x91\x90aW\x8AV[\x11\x15a>9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aX\xA2V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a>[`\xA0\x8A\x01\x8AaS\xAFV[\x83\x81\x81\x10a>kWa>kaS\x11V[\x90P` \x02\x01` \x81\x01\x90a>\x80\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a?\xD0W`\0a>\x9E` \x8A\x01\x8AaI\xFFV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a>\xDF`\xC0\x8E\x01\x8EaS\xAFV[\x87\x81\x81\x10a>\xEFWa>\xEFaS\x11V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a?CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?g\x91\x90aV\\V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a?\xC8Wa?\xC8\x81\x84a?\x9D`\xA0\x8F\x01\x8FaS\xAFV[\x88\x81\x81\x10a?\xADWa?\xADaS\x11V[\x90P` \x02\x01` \x81\x01\x90a?\xC2\x91\x90aI\xFFV[\x85a9>V[PPPaA]V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10a@\x12Wa@\x12aS\x11V[\x90P` \x02\x01` \x81\x01\x90a@'\x91\x90aI\xFFV[a@4`\xA0\x8D\x01\x8DaS\xAFV[\x86\x81\x81\x10a@DWa@DaS\x11V[\x90P` \x02\x01` \x81\x01\x90a@Y\x91\x90aI\xFFV[a@f`\xC0\x8E\x01\x8EaS\xAFV[\x87\x81\x81\x10a@vWa@vaS\x11V[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@\xEAW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aA]WaA]\x823aA\x0F`\xA0\x8C\x01\x8CaS\xAFV[\x85\x81\x81\x10aA\x1FWaA\x1FaS\x11V[\x90P` \x02\x01` \x81\x01\x90aA4\x91\x90aI\xFFV[aAA`\xC0\x8D\x01\x8DaS\xAFV[\x86\x81\x81\x10aAQWaAQaS\x11V[\x90P` \x02\x015a9>V[`\x01\x01a=\x94V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aB\xBDW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aA\xE3\x90\x86\x90\x86\x90`\x04\x01aY*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB$\x91\x90aY\x87V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\x01`\x01`\xA0\x1B\x03\x16aB\xD1\x83\x83aD\x9DV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aD\x08W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aC\xD1\x90\x88\x90\x88\x90\x87\x90`\x04\x01aW\xB9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xEBW`\0\x80\xFD[PZ\xF1\x15\x80\x15aC\xFFW=`\0\x80>=`\0\xFD[PPPPaD\x96V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13bW=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aD\xAC\x85\x85aD\xB9V[\x91P\x91Pa\t\xC6\x81aE)V[`\0\x80\x82Q`A\x14\x15aD\xF0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaD\xE4\x87\x82\x85\x85aF\xE4V[\x94P\x94PPPPaE\"V[\x82Q`@\x14\x15aE\x1AW` \x83\x01Q`@\x84\x01QaE\x0F\x86\x83\x83aG\xD1V[\x93P\x93PPPaE\"V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aE=WaE=aY\xB1V[\x14\x15aEFWPV[`\x01\x81`\x04\x81\x11\x15aEZWaEZaY\xB1V[\x14\x15aE\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[`\x02\x81`\x04\x81\x11\x15aE\xBCWaE\xBCaY\xB1V[\x14\x15aF\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xBCV[`\x03\x81`\x04\x81\x11\x15aF\x1EWaF\x1EaY\xB1V[\x14\x15aFwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\x04\x81`\x04\x81\x11\x15aF\x8BWaF\x8BaY\xB1V[\x14\x15a\x0F\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aG\x1BWP`\0\x90P`\x03aG\xC8V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aG3WP\x84`\xFF\x16`\x1C\x14\x15[\x15aGDWP`\0\x90P`\x04aG\xC8V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aG\x98W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aG\xC1W`\0`\x01\x92P\x92PPaG\xC8V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aG\xEE`\xFF\x86\x90\x1C`\x1BaW\x8AV[\x90PaG\xFC\x87\x82\x88\x85aF\xE4V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aH\x1CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aE\"W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aHaW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aHwW`\0\x80\xFD[aH\x83\x85\x82\x86\x01aH\nV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xEEW`\0\x80\xFD[\x805aH\xAF\x81aH\x8FV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aH\xCCW`\0\x80\xFD[\x855aH\xD7\x81aH\x8FV[\x94P` \x86\x015aH\xE7\x81aH\x8FV[\x93P`@\x86\x015aH\xF7\x81aH\x8FV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aIGW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aI+V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aIeW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aI}W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x94W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aE\"W`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aI\xC1W`\0\x80\xFD[aI\xCB\x85\x85aISV[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xE6W`\0\x80\xFD[aI\xF2\x86\x82\x87\x01aIkV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aJ\x11W`\0\x80\xFD[\x815aJ\x1C\x81aH\x8FV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aJ8W`\0\x80\xFD[\x835aJC\x81aH\x8FV[\x92P` \x84\x015aJS\x81aH\x8FV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aJvW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aJ\x93W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\xAAW`\0\x80\xFD[aJ\xB6\x88\x83\x89\x01aH\nV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aJ\xCFW`\0\x80\xFD[PaJ\xDC\x87\x82\x88\x01aH\nV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aK\x04W`\0\x80\xFD[\x885aK\x0F\x81aH\x8FV[\x97P` \x89\x015aK\x1F\x81aH\x8FV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aKIW`\0\x80\xFD[aKU\x8C\x83\x8D\x01aH\nV[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aKnW`\0\x80\xFD[PaK{\x8B\x82\x8C\x01aH\nV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aK\xABW`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aK\xC2W`\0\x80\xFD[aK\xCE\x8C\x83\x8D\x01aH\nV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aK\xE7W`\0\x80\xFD[aK\xF3\x8C\x83\x8D\x01aH\nV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aL\x0CW`\0\x80\xFD[aL\x18\x8C\x83\x8D\x01aH\nV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aKnW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aLiWaLiaL1V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aLiWaLiaL1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aL\xB9WaL\xB9aL1V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xEEW`\0\x80\xFD[\x805aH\xAF\x81aL\xC1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xF7WaL\xF7aL1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aM\x12W`\0\x80\xFD[\x815` aM'aM\"\x83aL\xDEV[aL\x91V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aMFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x805aM]\x81aH\x8FV[\x83R\x91\x83\x01\x91\x83\x01aMJV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aM\x86W`\0\x80\xFD[\x815` aM\x96aM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aM\xB5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x805\x83R\x91\x83\x01\x91\x83\x01aM\xB9V[`\0`\xE0\x82\x84\x03\x12\x15aM\xE2W`\0\x80\xFD[aM\xEAaLGV[\x90PaM\xF5\x82aH\xA4V[\x81RaN\x03` \x83\x01aH\xA4V[` \x82\x01RaN\x14`@\x83\x01aH\xA4V[`@\x82\x01R``\x82\x015``\x82\x01RaN/`\x80\x83\x01aL\xD3V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNNW`\0\x80\xFD[aNZ\x85\x83\x86\x01aM\x01V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aNsW`\0\x80\xFD[PaN\x80\x84\x82\x85\x01aMuV[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aN\x9EW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xB4W`\0\x80\xFD[aN\xC0\x84\x82\x85\x01aM\xD0V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aN\xDAW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aJ\x1CW`\0\x80\xFD[\x80\x15\x15\x81\x14a\x0F\xEEW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO\x11W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO(W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aO<W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aORW`\0\x80\xFD[PaO_\x88\x82\x89\x01aH\nV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aOz\x81aN\xEBV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aO\x9BW`\0\x80\xFD[\x825aO\xA6\x81aH\x8FV[\x91P` \x83\x015aO\xB6\x81aH\x8FV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aO\xD3W`\0\x80\xFD[aO\xDBaLoV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\xF4W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aP\x08W`\0\x80\xFD[\x815` \x82\x82\x11\x15aP\x1CWaP\x1CaL1V[aP.`\x1F\x83\x01`\x1F\x19\x16\x82\x01aL\x91V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aPDW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aP\x81W`\0\x80\xFD[\x855aP\x8C\x81aH\x8FV[\x94P` \x86\x015aP\x9C\x81aH\x8FV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xB8W`\0\x80\xFD[aP\xC4\x89\x83\x8A\x01aO\xC1V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aP\xDAW`\0\x80\xFD[PaP\xE7\x88\x82\x89\x01aO\xC1V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x0CW`\0\x80\xFD[\x825aQ\x17\x81aH\x8FV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ2W`\0\x80\xFD[aQ>\x85\x82\x86\x01aM\x01V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aQxW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aQ\\V[P\x94\x95\x94PPPPPV[` \x81R`\0aJ\x1C` \x83\x01\x84aQHV[`\0\x80` \x83\x85\x03\x12\x15aQ\xA9W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xBFW`\0\x80\xFD[aH\x83\x85\x82\x86\x01aIkV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xDEW`\0\x80\xFD[\x825aQ\xE9\x81aH\x8FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aR\rW`\0\x80\xFD[\x845aR\x18\x81aH\x8FV[\x93P` \x85\x015\x92P`@\x85\x015aR/\x81aH\x8FV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aQxW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aRSV[`@\x81R`\0aR\x8B`@\x83\x01\x85aR?V[\x82\x81\x03` \x84\x01Ra\x12 \x81\x85aQHV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\xB2W`\0\x80\xFD[\x835aR\xBD\x81aH\x8FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xD8W`\0\x80\xFD[aR\xE4\x86\x82\x87\x01aO\xC1V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aS\x07W`\0\x80\xFD[aJ\x1C\x83\x83aISV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aSQWaSQaS'V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aS\xA5W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aS\xC6W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aS\xE0W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\"W`\0\x80\xFD[``\x81\x01\x825aT\x07\x81aH\x8FV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aT#\x82aH\x8FV[\x16` \x83\x01R`@\x83\x015aT7\x81aL\xC1V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aT\x8BW`\0\x80\xFD[\x81QaJ\x1C\x81aH\x8FV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aUOW`\0\x80\xFD[\x81QaJ\x1C\x81aN\xEBV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aS\xA5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aU\xCAW`\0\x80\xFD[\x815aJ\x1C\x81aN\xEBV[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaV0`\xE0\x85\x01\x82aR?V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12 \x82\x82aQHV[` \x81R`\0aJ\x1C` \x83\x01\x84aU\xD5V[`\0` \x82\x84\x03\x12\x15aVnW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aV\x86W`\0\x80\xFD[\x81Q` aV\x96aM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aV\xB5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x80Q\x83R\x91\x83\x01\x91\x83\x01aV\xB9V[`\0\x80`@\x83\x85\x03\x12\x15aV\xE3W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV\xFAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aW\x0EW`\0\x80\xFD[\x81Q` aW\x1EaM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aW=W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aWdW\x85QaWU\x81aH\x8FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aWBV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15aW}W`\0\x80\xFD[PaQ>\x85\x82\x86\x01aVuV[`\0\x82\x19\x82\x11\x15aW\x9DWaW\x9DaS'V[P\x01\x90V[`\0\x82\x82\x10\x15aW\xB4WaW\xB4aS'V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x82\x81R`@` \x82\x01R`\0aN\xC0`@\x83\x01\x84aU\xD5V[`\0` \x82\x84\x03\x12\x15aX\x08W`\0\x80\xFD[\x815aJ\x1C\x81aL\xC1V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815aX>\x81aH\x8FV[aXH\x81\x83aX\x13V[P`\x01\x81\x01` \x83\x015aX[\x81aH\x8FV[aXe\x81\x83aX\x13V[P`@\x83\x015aXt\x81aL\xC1V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a\x17Q6\x83aM\xD0V[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15aY^W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01aYBV[\x81\x81\x11\x15aYpW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aY\x99W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aJ\x1CW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 &\xB1\xFE\xD4\x84\x88\x18C\xA1\xD9\x81\x1EI?\xAB\xDFi?\x06\x8A-\xFA:\xF9(\x9C*\x7F\xBAt\x87>dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106103425760003560e01c8063635bbd10116101b8578063b7f06ebe11610104578063cf80873e116100a2578063f16172b01161007c578063f16172b014610908578063f2fde38b1461091b578063f698da251461092e578063fabc1cbc1461093657600080fd5b8063cf80873e146108c1578063da8be864146108e2578063eea9064b146108f557600080fd5b8063c488375a116100de578063c488375a146107de578063c5e480db146107fe578063c94b5111146108a4578063ca661c04146108b757600080fd5b8063b7f06ebe14610784578063bb45fef2146107a7578063c448feb8146107d557600080fd5b8063886f1195116101715780639104c3191161014b5780639104c3191461070f57806399be81c81461072a578063a17884841461073d578063b13442711461075d57600080fd5b8063886f1195146106cb5780638da5cb5b146106de57806390041347146106ef57600080fd5b8063635bbd101461063657806365da1264146106495780636d70f7ae14610672578063715018a614610685578063778e55f31461068d5780637f548071146106b857600080fd5b806328a573ae116102925780634665bcda11610230578063597b36da1161020a578063597b36da146105e55780635ac86ab7146105f85780635c975abb1461061b57806360d7faed1461062357600080fd5b80634665bcda146105ac5780634fc40b61146105d3578063595c6a67146105dd57600080fd5b806339b70e381161026c57806339b70e38146104f45780633cdeb5e0146105335780633e28391d14610562578063433773821461058557600080fd5b806328a573ae146104ae57806329c77d4f146104c157806333404396146104e157600080fd5b8063132d4967116102ff57806316928365116102d957806316928365146104285780631bbce0911461046157806320606b701461047457806322bf40e41461049b57600080fd5b8063132d4967146103ef578063136439dd146104025780631522bf021461041557600080fd5b80630449ca391461034757806304a4f9791461036d5780630b9f487a146103945780630dd8dd02146103a75780630f589e59146103c757806310d67a2f146103dc575b600080fd5b61035a61035536600461484e565b610949565b6040519081526020015b60405180910390f35b61035a7f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad81565b61035a6103a23660046148b4565b6109ce565b6103ba6103b536600461484e565b610a90565b604051610364919061490f565b6103da6103d53660046149ac565b610df9565b005b6103da6103ea3660046149ff565b610f3e565b6103da6103fd366004614a23565b610ff1565b6103da610410366004614a64565b6110a8565b6103da610423366004614a7d565b6111e7565b61035a6104363660046149ff565b6001600160a01b0316600090815260996020526040902060010154600160a01b900463ffffffff1690565b61035a61046f366004614a23565b6111fb565b61035a7f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a86681565b6103da6104a9366004614ae8565b611229565b6103da6104bc366004614a23565b61136d565b61035a6104cf3660046149ff565b609b6020526000908152604090205481565b6103da6104ef366004614b8f565b61141d565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610364565b61051b6105413660046149ff565b6001600160a01b039081166000908152609960205260409020600101541690565b6105756105703660046149ff565b61155a565b6040519015158152602001610364565b61035a7f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b81565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b61035a6213c68081565b6103da61157a565b61035a6105f3366004614e8c565b611641565b610575610606366004614ec8565b606654600160ff9092169190911b9081161490565b60665461035a565b6103da610631366004614ef9565b611671565b6103da610644366004614a64565b61170c565b61051b6106573660046149ff565b609a602052600090815260409020546001600160a01b031681565b6105756106803660046149ff565b61171d565b6103da611757565b61035a61069b366004614f88565b609860209081526000928352604080842090915290825290205481565b6103da6106c6366004615069565b61176b565b60655461051b906001600160a01b031681565b6033546001600160a01b031661051b565b6107026106fd3660046150f9565b611997565b6040516103649190615183565b61051b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b6103da610738366004615196565b611a71565b61035a61074b3660046149ff565b609f6020526000908152604090205481565b61051b7f000000000000000000000000000000000000000000000000000000000000000081565b610575610792366004614a64565b609e6020526000908152604090205460ff1681565b6105756107b53660046151cb565b609c60209081526000928352604080842090915290825290205460ff1681565b61035a609d5481565b61035a6107ec3660046149ff565b60a16020526000908152604090205481565b61086e61080c3660046149ff565b6040805160608082018352600080835260208084018290529284018190526001600160a01b03948516815260998352839020835191820184528054851682526001015493841691810191909152600160a01b90920463ffffffff169082015290565b6040805182516001600160a01b039081168252602080850151909116908201529181015163ffffffff1690820152606001610364565b61035a6108b23660046151f7565b611b43565b61035a62034bc081565b6108d46108cf3660046149ff565b611bfc565b604051610364929190615278565b6103ba6108f03660046149ff565b611fb4565b6103da61090336600461529d565b612478565b6103da6109163660046152f5565b612595565b6103da6109293660046149ff565b612626565b61035a61269c565b6103da610944366004614a64565b6126da565b609d54600090815b838110156109c657600060a1600087878581811061097157610971615311565b905060200201602081019061098691906149ff565b6001600160a01b03166001600160a01b03168152602001908152602001600020549050828111156109b5578092505b506109bf8161533d565b9050610951565b509392505050565b604080517f14bde674c9f64b2ad00eaaee4a8bed1fabef35c7507e3c5b9cfc9436909a2dad6020808301919091526001600160a01b038681168385015288811660608401528716608083015260a0820185905260c08083018590528351808403909101815260e0909201909252805191012060009081610a4c61269c565b60405161190160f01b602082015260228101919091526042810183905260620160408051808303601f19018152919052805160209091012098975050505050505050565b60665460609060019060029081161415610ac55760405162461bcd60e51b8152600401610abc90615358565b60405180910390fd5b6000836001600160401b03811115610adf57610adf614c31565b604051908082528060200260200182016040528015610b08578160200160208202803683370190505b50336000908152609a60205260408120549192506001600160a01b03909116905b85811015610dee57868682818110610b4357610b43615311565b9050602002810190610b55919061538f565b610b639060208101906153af565b9050878783818110610b7757610b77615311565b9050602002810190610b89919061538f565b610b9390806153af565b905014610c085760405162461bcd60e51b815260206004820152603860248201527f44656c65676174696f6e4d616e616765722e717565756557697468647261776160448201527f6c3a20696e707574206c656e677468206d69736d6174636800000000000000006064820152608401610abc565b33878783818110610c1b57610c1b615311565b9050602002810190610c2d919061538f565b610c3e9060608101906040016149ff565b6001600160a01b031614610cba5760405162461bcd60e51b815260206004820152603c60248201527f44656c65676174696f6e4d616e616765722e717565756557697468647261776160448201527f6c3a2077697468647261776572206d757374206265207374616b6572000000006064820152608401610abc565b610dbf3383898985818110610cd157610cd1615311565b9050602002810190610ce3919061538f565b610cf49060608101906040016149ff565b8a8a86818110610d0657610d06615311565b9050602002810190610d18919061538f565b610d2290806153af565b808060200260200160405190810160405280939291908181526020018383602002808284376000920191909152508e92508d9150889050818110610d6857610d68615311565b9050602002810190610d7a919061538f565b610d889060208101906153af565b8080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061283692505050565b838281518110610dd157610dd1615311565b602090810291909101015280610de68161533d565b915050610b29565b509095945050505050565b610e023361155a565b15610e885760405162461bcd60e51b815260206004820152604a60248201527f44656c65676174696f6e4d616e616765722e726567697374657241734f70657260448201527f61746f723a2063616c6c657220697320616c7265616479206163746976656c796064820152690819195b1959d85d195960b21b608482015260a401610abc565b610e923384612df6565b604080518082019091526060815260006020820152610eb43380836000612fe9565b336001600160a01b03167f8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e285604051610eed91906153f8565b60405180910390a2336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908484604051610f3092919061544a565b60405180910390a250505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f91573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610fb59190615479565b6001600160a01b0316336001600160a01b031614610fe55760405162461bcd60e51b8152600401610abc90615496565b610fee8161327f565b50565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806110505750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b61106c5760405162461bcd60e51b8152600401610abc906154e0565b6110758361155a565b156110a3576001600160a01b038084166000908152609a6020526040902054166110a181858585613376565b505b505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156110f0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611114919061553d565b6111305760405162461bcd60e51b8152600401610abc9061555a565b606654818116146111a95760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610abc565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6111ef6133f1565b6110a18484848461344b565b6001600160a01b0383166000908152609b602052604081205461122085828686611b43565b95945050505050565b600054610100900460ff16158080156112495750600054600160ff909116105b806112635750303b158015611263575060005460ff166001145b6112c65760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610abc565b6000805460ff1916600117905580156112e9576000805461ff0019166101001790555b6112f38888613671565b6112fb61375b565b609755611307896137f2565b61131086613844565b61131c8585858561344b565b8015611362576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806113cc5750336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016145b6113e85760405162461bcd60e51b8152600401610abc906154e0565b6113f18361155a565b156110a3576001600160a01b038084166000908152609a6020526040902054166110a18185858561393e565b606654600290600490811614156114465760405162461bcd60e51b8152600401610abc90615358565b600260c95414156114995760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610abc565b600260c95560005b88811015611549576115398a8a838181106114be576114be615311565b90506020028101906114d091906155a2565b8989848181106114e2576114e2615311565b90506020028101906114f491906153af565b89898681811061150657611506615311565b9050602002013588888781811061151f5761151f615311565b905060200201602081019061153491906155b8565b6139b9565b6115428161533d565b90506114a1565b5050600160c9555050505050505050565b6001600160a01b039081166000908152609a602052604090205416151590565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156115c2573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115e6919061553d565b6116025760405162461bcd60e51b8152600401610abc9061555a565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6000816040516020016116549190615649565b604051602081830303815290604052805190602001209050919050565b6066546002906004908116141561169a5760405162461bcd60e51b8152600401610abc90615358565b600260c95414156116ed5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610abc565b600260c9556116ff86868686866139b9565b5050600160c95550505050565b6117146133f1565b610fee81613844565b60006001600160a01b0382161580159061175157506001600160a01b038083166000818152609a6020526040902054909116145b92915050565b61175f6133f1565b61176960006137f2565b565b42836020015110156117ef5760405162461bcd60e51b815260206004820152604160248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a207374616b6572207369676e6174757265206578706972656064820152601960fa1b608482015260a401610abc565b6117f88561155a565b156118815760405162461bcd60e51b815260206004820152604d60248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a207374616b657220697320616c726561647920616374697660648201526c195b1e4819195b1959d85d1959609a1b608482015260a401610abc565b61188a8461171d565b6119165760405162461bcd60e51b815260206004820152605160248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f4279536960448201527f676e61747572653a206f70657261746f72206973206e6f7420726567697374656064820152703932b21034b71022b4b3b2b72630bcb2b960791b608482015260a401610abc565b6000609b6000876001600160a01b03166001600160a01b0316815260200190815260200160002054905060006119528783888860200151611b43565b6001600160a01b0388166000908152609b60205260409020600184019055855190915061198290889083906141a3565b61198e87878686612fe9565b50505050505050565b6060600082516001600160401b038111156119b4576119b4614c31565b6040519080825280602002602001820160405280156119dd578160200160208202803683370190505b50905060005b83518110156109c6576001600160a01b03851660009081526098602052604081208551909190869084908110611a1b57611a1b615311565b60200260200101516001600160a01b03166001600160a01b0316815260200190815260200160002054828281518110611a5657611a56615311565b6020908102919091010152611a6a8161533d565b90506119e3565b611a7a3361171d565b611afc5760405162461bcd60e51b815260206004820152604760248201527f44656c65676174696f6e4d616e616765722e7570646174654f70657261746f7260448201527f4d657461646174615552493a2063616c6c6572206d75737420626520616e206f6064820152663832b930ba37b960c91b608482015260a401610abc565b336001600160a01b03167f02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b67080908383604051611b3792919061544a565b60405180910390a25050565b604080517f39111bc4a4d688e1f685123d7497d4615370152a8ee4a0593e647bd06ad8bb0b6020808301919091526001600160a01b0387811683850152851660608301526080820186905260a08083018590528351808403909101815260c0909201909252805191012060009081611bb961269c565b60405161190160f01b602082015260228101919091526042810183905260620160408051808303601f190181529190528051602090910120979650505050505050565b6040516360f4062b60e01b81526001600160a01b03828116600483015260609182916000917f0000000000000000000000000000000000000000000000000000000000000000909116906360f4062b90602401602060405180830381865afa158015611c6c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c90919061565c565b6040516394f649dd60e01b81526001600160a01b03868116600483015291925060009182917f0000000000000000000000000000000000000000000000000000000000000000909116906394f649dd90602401600060405180830381865afa158015611d00573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611d2891908101906156d0565b9150915060008313611d3f57909590945092505050565b606080835160001415611df9576040805160018082528183019092529060208083019080368337505060408051600180825281830190925292945090506020808301908036833701905050905073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac082600081518110611db457611db4615311565b60200260200101906001600160a01b031690816001600160a01b0316815250508481600081518110611de857611de8615311565b602002602001018181525050611fa7565b8351611e0690600161578a565b6001600160401b03811115611e1d57611e1d614c31565b604051908082528060200260200182016040528015611e46578160200160208202803683370190505b50915081516001600160401b03811115611e6257611e62614c31565b604051908082528060200260200182016040528015611e8b578160200160208202803683370190505b50905060005b8451811015611f2557848181518110611eac57611eac615311565b6020026020010151838281518110611ec657611ec6615311565b60200260200101906001600160a01b031690816001600160a01b031681525050838181518110611ef857611ef8615311565b6020026020010151828281518110611f1257611f12615311565b6020908102919091010152600101611e91565b5073beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08260018451611f4a91906157a2565b81518110611f5a57611f5a615311565b60200260200101906001600160a01b031690816001600160a01b031681525050848160018451611f8a91906157a2565b81518110611f9a57611f9a615311565b6020026020010181815250505b9097909650945050505050565b60665460609060019060029081161415611fe05760405162461bcd60e51b8152600401610abc90615358565b611fe98361155a565b6120695760405162461bcd60e51b8152602060048201526044602482018190527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a207374908201527f616b6572206d7573742062652064656c65676174656420746f20756e64656c656064820152636761746560e01b608482015260a401610abc565b6120728361171d565b156120e55760405162461bcd60e51b815260206004820152603d60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a206f7060448201527f657261746f72732063616e6e6f7420626520756e64656c6567617465640000006064820152608401610abc565b6001600160a01b0383166121615760405162461bcd60e51b815260206004820152603c60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a20636160448201527f6e6e6f7420756e64656c6567617465207a65726f2061646472657373000000006064820152608401610abc565b6001600160a01b038084166000818152609a6020526040902054909116903314806121945750336001600160a01b038216145b806121bb57506001600160a01b038181166000908152609960205260409020600101541633145b61222d5760405162461bcd60e51b815260206004820152603d60248201527f44656c65676174696f6e4d616e616765722e756e64656c65676174653a20636160448201527f6c6c65722063616e6e6f7420756e64656c6567617465207374616b65720000006064820152608401610abc565b60008061223986611bfc565b9092509050336001600160a01b0387161461228f57826001600160a01b0316866001600160a01b03167ff0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a60405160405180910390a35b826001600160a01b0316866001600160a01b03167ffee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af4467660405160405180910390a36001600160a01b0386166000908152609a6020526040902080546001600160a01b0319169055815161231157604080516000815260208101909152945061246f565b81516001600160401b0381111561232a5761232a614c31565b604051908082528060200260200182016040528015612353578160200160208202803683370190505b50945060005b825181101561246d576040805160018082528183019092526000916020808301908036833750506040805160018082528183019092529293506000929150602080830190803683370190505090508483815181106123b9576123b9615311565b6020026020010151826000815181106123d4576123d4615311565b60200260200101906001600160a01b031690816001600160a01b03168152505083838151811061240657612406615311565b60200260200101518160008151811061242157612421615311565b60200260200101818152505061243a89878b8585612836565b88848151811061244c5761244c615311565b602002602001018181525050505080806124659061533d565b915050612359565b505b50505050919050565b6124813361155a565b156124ff5760405162461bcd60e51b815260206004820152604260248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f3a20737460448201527f616b657220697320616c7265616479206163746976656c792064656c65676174606482015261195960f21b608482015260a401610abc565b6125088361171d565b6125895760405162461bcd60e51b815260206004820152604660248201527f44656c65676174696f6e4d616e616765722e64656c6567617465546f3a206f7060448201527f657261746f72206973206e6f74207265676973746572656420696e2045696765606482015265372630bcb2b960d11b608482015260a401610abc565b6110a333848484612fe9565b61259e3361171d565b61261c5760405162461bcd60e51b815260206004820152604360248201527f44656c65676174696f6e4d616e616765722e6d6f646966794f70657261746f7260448201527f44657461696c733a2063616c6c6572206d75737420626520616e206f706572616064820152623a37b960e91b608482015260a401610abc565b610fee3382612df6565b61262e6133f1565b6001600160a01b0381166126935760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610abc565b610fee816137f2565b60007f00000000000000000000000000000000000000000000000000000000000000004614156126cd575060975490565b6126d561375b565b905090565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561272d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906127519190615479565b6001600160a01b0316336001600160a01b0316146127815760405162461bcd60e51b8152600401610abc90615496565b6066541981196066541916146127ff5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610abc565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016111dc565b60006001600160a01b0386166128cd5760405162461bcd60e51b815260206004820152605060248201527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448201527f6e6451756575655769746864726177616c3a207374616b65722063616e6e6f7460648201526f206265207a65726f206164647265737360801b608482015260a401610abc565b82516129575760405162461bcd60e51b815260206004820152604d60248201527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448201527f6e6451756575655769746864726177616c3a207374726174656769657320636160648201526c6e6e6f7420626520656d70747960981b608482015260a401610abc565b60005b8351811015612d04576001600160a01b038616156129b0576129b0868886848151811061298957612989615311565b60200260200101518685815181106129a3576129a3615311565b6020026020010151613376565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac06001600160a01b03168482815181106129e0576129e0615311565b60200260200101516001600160a01b03161415612aa9577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663beffbb8988858481518110612a3957612a39615311565b60200260200101516040518363ffffffff1660e01b8152600401612a729291906001600160a01b03929092168252602082015260400190565b600060405180830381600087803b158015612a8c57600080fd5b505af1158015612aa0573d6000803e3d6000fd5b50505050612cfc565b846001600160a01b0316876001600160a01b03161480612b7b57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639b4da03d858381518110612b0557612b05615311565b60200260200101516040518263ffffffff1660e01b8152600401612b3891906001600160a01b0391909116815260200190565b602060405180830381865afa158015612b55573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b79919061553d565b155b612c475760405162461bcd60e51b8152602060048201526084602482018190527f44656c65676174696f6e4d616e616765722e5f72656d6f76655368617265734160448301527f6e6451756575655769746864726177616c3a2077697468647261776572206d7560648301527f73742062652073616d652061646472657373206173207374616b657220696620908201527f746869726450617274795472616e7366657273466f7262696464656e2061726560a482015263081cd95d60e21b60c482015260e401610abc565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638c80d4e588868481518110612c8957612c89615311565b6020026020010151868581518110612ca357612ca3615311565b60200260200101516040518463ffffffff1660e01b8152600401612cc9939291906157b9565b600060405180830381600087803b158015612ce357600080fd5b505af1158015612cf7573d6000803e3d6000fd5b505050505b60010161295a565b506001600160a01b0386166000908152609f60205260408120805491829190612d2c8361533d565b919050555060006040518060e00160405280896001600160a01b03168152602001886001600160a01b03168152602001876001600160a01b031681526020018381526020014363ffffffff1681526020018681526020018581525090506000612d9482611641565b6000818152609e602052604090819020805460ff19166001179055519091507f9009ab153e8014fbfb02f2217f5cde7aa7f9ad734ae85ca3ee3f4ca2fdd499f990612de290839085906157dd565b60405180910390a198975050505050505050565b6213c680612e0a60608301604084016157f6565b63ffffffff161115612ebf5760405162461bcd60e51b815260206004820152606c60248201527f44656c65676174696f6e4d616e616765722e5f7365744f70657261746f72446560448201527f7461696c733a207374616b65724f70744f757457696e646f77426c6f636b732060648201527f63616e6e6f74206265203e204d41585f5354414b45525f4f50545f4f55545f5760848201526b494e444f575f424c4f434b5360a01b60a482015260c401610abc565b6001600160a01b0382166000908152609960205260409081902060010154600160a01b900463ffffffff1690612efb90606084019084016157f6565b63ffffffff161015612f915760405162461bcd60e51b815260206004820152605360248201527f44656c65676174696f6e4d616e616765722e5f7365744f70657261746f72446560448201527f7461696c733a207374616b65724f70744f757457696e646f77426c6f636b732060648201527218d85b9b9bdd08189948191958dc99585cd959606a1b608482015260a401610abc565b6001600160a01b03821660009081526099602052604090208190612fb58282615833565b505060405133907ffebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac90611b379084906153f8565b606654600090600190811614156130125760405162461bcd60e51b8152600401610abc90615358565b6001600160a01b038085166000908152609960205260409020600101541680158015906130485750336001600160a01b03821614155b801561305d5750336001600160a01b03861614155b156131ca5742846020015110156130dc5760405162461bcd60e51b815260206004820152603760248201527f44656c65676174696f6e4d616e616765722e5f64656c65676174653a2061707060448201527f726f766572207369676e617475726520657870697265640000000000000000006064820152608401610abc565b6001600160a01b0381166000908152609c6020908152604080832086845290915290205460ff16156131765760405162461bcd60e51b815260206004820152603760248201527f44656c65676174696f6e4d616e616765722e5f64656c65676174653a2061707060448201527f726f76657253616c7420616c7265616479207370656e740000000000000000006064820152608401610abc565b6001600160a01b0381166000908152609c6020908152604080832086845282528220805460ff191660011790558501516131b79088908890859088906109ce565b90506131c8828287600001516141a3565b505b6001600160a01b038681166000818152609a602052604080822080546001600160a01b031916948a169485179055517fc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d87433049190a360008061322988611bfc565b9150915060005b825181101561136257613277888a85848151811061325057613250615311565b602002602001015185858151811061326a5761326a615311565b602002602001015161393e565b600101613230565b6001600160a01b03811661330d5760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610abc565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038085166000908152609860209081526040808320938616835292905290812080548392906133ad9084906157a2565b92505081905550836001600160a01b03167f6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd848484604051610f30939291906157b9565b6033546001600160a01b031633146117695760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610abc565b8281146134d35760405162461bcd60e51b815260206004820152604a60248201527f44656c65676174696f6e4d616e616765722e5f7365745374726174656779576960448201527f746864726177616c44656c6179426c6f636b733a20696e707574206c656e67746064820152690d040dad2e6dac2e8c6d60b31b608482015260a401610abc565b8260005b818110156136695760008686838181106134f3576134f3615311565b905060200201602081019061350891906149ff565b6001600160a01b038116600090815260a1602052604081205491925086868581811061353657613536615311565b90506020020135905062034bc08111156135fa5760405162461bcd60e51b815260206004820152607360248201527f44656c65676174696f6e4d616e616765722e5f7365745374726174656779576960448201527f746864726177616c44656c6179426c6f636b733a205f7769746864726177616c60648201527f44656c6179426c6f636b732063616e6e6f74206265203e204d41585f5749544860848201527244524157414c5f44454c41595f424c4f434b5360681b60a482015260c401610abc565b6001600160a01b038316600081815260a160209081526040918290208490558151928352820184905281018290527f0e7efa738e8b0ce6376a0c1af471655540d2e9a81647d7b09ed823018426576d9060600160405180910390a1505050806136629061533d565b90506134d7565b505050505050565b6065546001600160a01b031615801561369257506001600160a01b03821615155b6137145760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610abc565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26137578261327f565b5050565b604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b62034bc08111156138fd5760405162461bcd60e51b815260206004820152607160248201527f44656c65676174696f6e4d616e616765722e5f7365744d696e5769746864726160448201527f77616c44656c6179426c6f636b733a205f6d696e5769746864726177616c446560648201527f6c6179426c6f636b732063616e6e6f74206265203e204d41585f5749544844526084820152704157414c5f44454c41595f424c4f434b5360781b60a482015260c401610abc565b609d5460408051918252602082018390527fafa003cd76f87ff9d62b35beea889920f33c0c42b8d45b74954d61d50f4b6b69910160405180910390a1609d55565b6001600160a01b0380851660009081526098602090815260408083209386168352929052908120805483929061397590849061578a565b92505081905550836001600160a01b03167f1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c848484604051610f30939291906157b9565b60006139c76105f387615896565b6000818152609e602052604090205490915060ff16613a485760405162461bcd60e51b815260206004820152604360248201526000805160206159c883398151915260448201527f645769746864726177616c3a20616374696f6e206973206e6f7420696e20717560648201526265756560e81b608482015260a401610abc565b609d544390613a5d60a0890160808a016157f6565b63ffffffff16613a6d919061578a565b1115613af55760405162461bcd60e51b815260206004820152605f60248201526000805160206159c883398151915260448201527f645769746864726177616c3a206d696e5769746864726177616c44656c61794260648201527f6c6f636b7320706572696f6420686173206e6f74207965742070617373656400608482015260a401610abc565b613b0560608701604088016149ff565b6001600160a01b0316336001600160a01b031614613b925760405162461bcd60e51b815260206004820152605060248201526000805160206159c883398151915260448201527f645769746864726177616c3a206f6e6c7920776974686472617765722063616e60648201526f1031b7b6b83632ba329030b1ba34b7b760811b608482015260a401610abc565b8115613c1457613ba560a08701876153af565b85149050613c145760405162461bcd60e51b815260206004820152604260248201526000805160206159c883398151915260448201527f645769746864726177616c3a20696e707574206c656e677468206d69736d61746064820152610c6d60f31b608482015260a401610abc565b6000818152609e60205260409020805460ff191690558115613d795760005b613c4060a08801886153af565b9050811015613d73574360a16000613c5b60a08b018b6153af565b85818110613c6b57613c6b615311565b9050602002016020810190613c8091906149ff565b6001600160a01b03168152602081019190915260400160002054613caa60a08a0160808b016157f6565b63ffffffff16613cba919061578a565b1115613cd85760405162461bcd60e51b8152600401610abc906158a2565b613d6b613ce860208901896149ff565b33613cf660a08b018b6153af565b85818110613d0657613d06615311565b9050602002016020810190613d1b91906149ff565b613d2860c08c018c6153af565b86818110613d3857613d38615311565b905060200201358a8a87818110613d5157613d51615311565b9050602002016020810190613d6691906149ff565b61435d565b600101613c33565b50614168565b336000908152609a60205260408120546001600160a01b0316905b613da160a08901896153af565b9050811015614165574360a16000613dbc60a08c018c6153af565b85818110613dcc57613dcc615311565b9050602002016020810190613de191906149ff565b6001600160a01b03168152602081019190915260400160002054613e0b60a08b0160808c016157f6565b63ffffffff16613e1b919061578a565b1115613e395760405162461bcd60e51b8152600401610abc906158a2565b73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0613e5b60a08a018a6153af565b83818110613e6b57613e6b615311565b9050602002016020810190613e8091906149ff565b6001600160a01b03161415613fd0576000613e9e60208a018a6149ff565b905060006001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016630e81073c83613edf60c08e018e6153af565b87818110613eef57613eef615311565b6040516001600160e01b031960e087901b1681526001600160a01b03909416600485015260200291909101356024830152506044016020604051808303816000875af1158015613f43573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613f67919061565c565b6001600160a01b038084166000908152609a6020526040902054919250168015613fc857613fc88184613f9d60a08f018f6153af565b88818110613fad57613fad615311565b9050602002016020810190613fc291906149ff565b8561393e565b50505061415d565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663c4623ea13389898581811061401257614012615311565b905060200201602081019061402791906149ff565b61403460a08d018d6153af565b8681811061404457614044615311565b905060200201602081019061405991906149ff565b61406660c08e018e6153af565b8781811061407657614076615311565b60405160e088901b6001600160e01b03191681526001600160a01b03968716600482015294861660248601529290941660448401526020909102013560648201526084019050600060405180830381600087803b1580156140d657600080fd5b505af11580156140ea573d6000803e3d6000fd5b505050506001600160a01b0382161561415d5761415d823361410f60a08c018c6153af565b8581811061411f5761411f615311565b905060200201602081019061413491906149ff565b61414160c08d018d6153af565b8681811061415157614151615311565b9050602002013561393e565b600101613d94565b50505b6040518181527fc97098c2f658800b4df29001527f7324bcdffcf6e8751a699ab920a1eced5b1d9060200160405180910390a1505050505050565b6001600160a01b0383163b156142bd57604051630b135d3f60e11b808252906001600160a01b03851690631626ba7e906141e3908690869060040161592a565b602060405180830381865afa158015614200573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906142249190615987565b6001600160e01b031916146110a35760405162461bcd60e51b815260206004820152605360248201527f454950313237315369676e61747572655574696c732e636865636b5369676e6160448201527f747572655f454950313237313a2045524331323731207369676e6174757265206064820152721d995c9a599a58d85d1a5bdb8819985a5b1959606a1b608482015260a401610abc565b826001600160a01b03166142d1838361449d565b6001600160a01b0316146110a35760405162461bcd60e51b815260206004820152604760248201527f454950313237315369676e61747572655574696c732e636865636b5369676e6160448201527f747572655f454950313237313a207369676e6174757265206e6f742066726f6d6064820152661039b4b3b732b960c91b608482015260a401610abc565b6001600160a01b03831673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014156144085760405162387b1360e81b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063387b1300906143d1908890889087906004016157b9565b600060405180830381600087803b1580156143eb57600080fd5b505af11580156143ff573d6000803e3d6000fd5b50505050614496565b60405163c608c7f360e01b81526001600160a01b03858116600483015284811660248301526044820184905282811660648301527f0000000000000000000000000000000000000000000000000000000000000000169063c608c7f390608401600060405180830381600087803b15801561448257600080fd5b505af1158015611362573d6000803e3d6000fd5b5050505050565b60008060006144ac85856144b9565b915091506109c681614529565b6000808251604114156144f05760208301516040840151606085015160001a6144e4878285856146e4565b94509450505050614522565b82516040141561451a576020830151604084015161450f8683836147d1565b935093505050614522565b506000905060025b9250929050565b600081600481111561453d5761453d6159b1565b14156145465750565b600181600481111561455a5761455a6159b1565b14156145a85760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610abc565b60028160048111156145bc576145bc6159b1565b141561460a5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610abc565b600381600481111561461e5761461e6159b1565b14156146775760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610abc565b600481600481111561468b5761468b6159b1565b1415610fee5760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202776272076616c604482015261756560f01b6064820152608401610abc565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561471b57506000905060036147c8565b8460ff16601b1415801561473357508460ff16601c14155b1561474457506000905060046147c8565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015614798573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166147c1576000600192509250506147c8565b9150600090505b94509492505050565b6000806001600160ff1b038316816147ee60ff86901c601b61578a565b90506147fc878288856146e4565b935093505050935093915050565b60008083601f84011261481c57600080fd5b5081356001600160401b0381111561483357600080fd5b6020830191508360208260051b850101111561452257600080fd5b6000806020838503121561486157600080fd5b82356001600160401b0381111561487757600080fd5b6148838582860161480a565b90969095509350505050565b6001600160a01b0381168114610fee57600080fd5b80356148af8161488f565b919050565b600080600080600060a086880312156148cc57600080fd5b85356148d78161488f565b945060208601356148e78161488f565b935060408601356148f78161488f565b94979396509394606081013594506080013592915050565b6020808252825182820181905260009190848201906040850190845b818110156149475783518352928401929184019160010161492b565b50909695505050505050565b60006060828403121561496557600080fd5b50919050565b60008083601f84011261497d57600080fd5b5081356001600160401b0381111561499457600080fd5b60208301915083602082850101111561452257600080fd5b6000806000608084860312156149c157600080fd5b6149cb8585614953565b925060608401356001600160401b038111156149e657600080fd5b6149f28682870161496b565b9497909650939450505050565b600060208284031215614a1157600080fd5b8135614a1c8161488f565b9392505050565b600080600060608486031215614a3857600080fd5b8335614a438161488f565b92506020840135614a538161488f565b929592945050506040919091013590565b600060208284031215614a7657600080fd5b5035919050565b60008060008060408587031215614a9357600080fd5b84356001600160401b0380821115614aaa57600080fd5b614ab68883890161480a565b90965094506020870135915080821115614acf57600080fd5b50614adc8782880161480a565b95989497509550505050565b60008060008060008060008060c0898b031215614b0457600080fd5b8835614b0f8161488f565b97506020890135614b1f8161488f565b9650604089013595506060890135945060808901356001600160401b0380821115614b4957600080fd5b614b558c838d0161480a565b909650945060a08b0135915080821115614b6e57600080fd5b50614b7b8b828c0161480a565b999c989b5096995094979396929594505050565b6000806000806000806000806080898b031215614bab57600080fd5b88356001600160401b0380821115614bc257600080fd5b614bce8c838d0161480a565b909a50985060208b0135915080821115614be757600080fd5b614bf38c838d0161480a565b909850965060408b0135915080821115614c0c57600080fd5b614c188c838d0161480a565b909650945060608b0135915080821115614b6e57600080fd5b634e487b7160e01b600052604160045260246000fd5b60405160e081016001600160401b0381118282101715614c6957614c69614c31565b60405290565b604080519081016001600160401b0381118282101715614c6957614c69614c31565b604051601f8201601f191681016001600160401b0381118282101715614cb957614cb9614c31565b604052919050565b63ffffffff81168114610fee57600080fd5b80356148af81614cc1565b60006001600160401b03821115614cf757614cf7614c31565b5060051b60200190565b600082601f830112614d1257600080fd5b81356020614d27614d2283614cde565b614c91565b82815260059290921b84018101918181019086841115614d4657600080fd5b8286015b84811015614d6a578035614d5d8161488f565b8352918301918301614d4a565b509695505050505050565b600082601f830112614d8657600080fd5b81356020614d96614d2283614cde565b82815260059290921b84018101918181019086841115614db557600080fd5b8286015b84811015614d6a5780358352918301918301614db9565b600060e08284031215614de257600080fd5b614dea614c47565b9050614df5826148a4565b8152614e03602083016148a4565b6020820152614e14604083016148a4565b604082015260608201356060820152614e2f60808301614cd3565b608082015260a08201356001600160401b0380821115614e4e57600080fd5b614e5a85838601614d01565b60a084015260c0840135915080821115614e7357600080fd5b50614e8084828501614d75565b60c08301525092915050565b600060208284031215614e9e57600080fd5b81356001600160401b03811115614eb457600080fd5b614ec084828501614dd0565b949350505050565b600060208284031215614eda57600080fd5b813560ff81168114614a1c57600080fd5b8015158114610fee57600080fd5b600080600080600060808688031215614f1157600080fd5b85356001600160401b0380821115614f2857600080fd5b9087019060e0828a031215614f3c57600080fd5b90955060208701359080821115614f5257600080fd5b50614f5f8882890161480a565b909550935050604086013591506060860135614f7a81614eeb565b809150509295509295909350565b60008060408385031215614f9b57600080fd5b8235614fa68161488f565b91506020830135614fb68161488f565b809150509250929050565b600060408284031215614fd357600080fd5b614fdb614c6f565b905081356001600160401b0380821115614ff457600080fd5b818401915084601f83011261500857600080fd5b813560208282111561501c5761501c614c31565b61502e601f8301601f19168201614c91565b9250818352868183860101111561504457600080fd5b8181850182850137600081838501015282855280860135818601525050505092915050565b600080600080600060a0868803121561508157600080fd5b853561508c8161488f565b9450602086013561509c8161488f565b935060408601356001600160401b03808211156150b857600080fd5b6150c489838a01614fc1565b945060608801359150808211156150da57600080fd5b506150e788828901614fc1565b95989497509295608001359392505050565b6000806040838503121561510c57600080fd5b82356151178161488f565b915060208301356001600160401b0381111561513257600080fd5b61513e85828601614d01565b9150509250929050565b600081518084526020808501945080840160005b838110156151785781518752958201959082019060010161515c565b509495945050505050565b602081526000614a1c6020830184615148565b600080602083850312156151a957600080fd5b82356001600160401b038111156151bf57600080fd5b6148838582860161496b565b600080604083850312156151de57600080fd5b82356151e98161488f565b946020939093013593505050565b6000806000806080858703121561520d57600080fd5b84356152188161488f565b935060208501359250604085013561522f8161488f565b9396929550929360600135925050565b600081518084526020808501945080840160005b838110156151785781516001600160a01b031687529582019590820190600101615253565b60408152600061528b604083018561523f565b82810360208401526112208185615148565b6000806000606084860312156152b257600080fd5b83356152bd8161488f565b925060208401356001600160401b038111156152d857600080fd5b6152e486828701614fc1565b925050604084013590509250925092565b60006060828403121561530757600080fd5b614a1c8383614953565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060001982141561535157615351615327565b5060010190565b60208082526019908201527f5061757361626c653a20696e6465782069732070617573656400000000000000604082015260600190565b60008235605e198336030181126153a557600080fd5b9190910192915050565b6000808335601e198436030181126153c657600080fd5b8301803591506001600160401b038211156153e057600080fd5b6020019150600581901b360382131561452257600080fd5b6060810182356154078161488f565b6001600160a01b0390811683526020840135906154238261488f565b166020830152604083013561543781614cc1565b63ffffffff811660408401525092915050565b60208152816020820152818360408301376000818301604090810191909152601f909201601f19160101919050565b60006020828403121561548b57600080fd5b8151614a1c8161488f565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b60208082526037908201527f44656c65676174696f6e4d616e616765723a206f6e6c7953747261746567794d60408201527f616e616765724f72456967656e506f644d616e61676572000000000000000000606082015260800190565b60006020828403121561554f57600080fd5b8151614a1c81614eeb565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b6000823560de198336030181126153a557600080fd5b6000602082840312156155ca57600080fd5b8135614a1c81614eeb565b600060018060a01b03808351168452806020840151166020850152806040840151166040850152506060820151606084015263ffffffff608083015116608084015260a082015160e060a085015261563060e085018261523f565b905060c083015184820360c08601526112208282615148565b602081526000614a1c60208301846155d5565b60006020828403121561566e57600080fd5b5051919050565b600082601f83011261568657600080fd5b81516020615696614d2283614cde565b82815260059290921b840181019181810190868411156156b557600080fd5b8286015b84811015614d6a57805183529183019183016156b9565b600080604083850312156156e357600080fd5b82516001600160401b03808211156156fa57600080fd5b818501915085601f83011261570e57600080fd5b8151602061571e614d2283614cde565b82815260059290921b8401810191818101908984111561573d57600080fd5b948201945b838610156157645785516157558161488f565b82529482019490820190615742565b9188015191965090935050508082111561577d57600080fd5b5061513e85828601615675565b6000821982111561579d5761579d615327565b500190565b6000828210156157b4576157b4615327565b500390565b6001600160a01b039384168152919092166020820152604081019190915260600190565b828152604060208201526000614ec060408301846155d5565b60006020828403121561580857600080fd5b8135614a1c81614cc1565b80546001600160a01b0319166001600160a01b0392909216919091179055565b813561583e8161488f565b6158488183615813565b5060018101602083013561585b8161488f565b6158658183615813565b50604083013561587481614cc1565b815463ffffffff60a01b191660a09190911b63ffffffff60a01b161790555050565b60006117513683614dd0565b6020808252606e908201526000805160206159c883398151915260408201527f645769746864726177616c3a207769746864726177616c44656c6179426c6f6360608201527f6b7320706572696f6420686173206e6f74207965742070617373656420666f7260808201526d207468697320737472617465677960901b60a082015260c00190565b82815260006020604081840152835180604085015260005b8181101561595e57858101830151858201606001528201615942565b81811115615970576000606083870101525b50601f01601f191692909201606001949350505050565b60006020828403121561599957600080fd5b81516001600160e01b031981168114614a1c57600080fd5b634e487b7160e01b600052602160045260246000fdfe44656c65676174696f6e4d616e616765722e5f636f6d706c6574655175657565a264697066735822122026b1fed484881843a1d9811e493fabdf693f068a2dfa3af9289c2a7fba74873e64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03BW`\x005`\xE0\x1C\x80cc[\xBD\x10\x11a\x01\xB8W\x80c\xB7\xF0n\xBE\x11a\x01\x04W\x80c\xCF\x80\x87>\x11a\0\xA2W\x80c\xF1ar\xB0\x11a\0|W\x80c\xF1ar\xB0\x14a\t\x08W\x80c\xF2\xFD\xE3\x8B\x14a\t\x1BW\x80c\xF6\x98\xDA%\x14a\t.W\x80c\xFA\xBC\x1C\xBC\x14a\t6W`\0\x80\xFD[\x80c\xCF\x80\x87>\x14a\x08\xC1W\x80c\xDA\x8B\xE8d\x14a\x08\xE2W\x80c\xEE\xA9\x06K\x14a\x08\xF5W`\0\x80\xFD[\x80c\xC4\x887Z\x11a\0\xDEW\x80c\xC4\x887Z\x14a\x07\xDEW\x80c\xC5\xE4\x80\xDB\x14a\x07\xFEW\x80c\xC9KQ\x11\x14a\x08\xA4W\x80c\xCAf\x1C\x04\x14a\x08\xB7W`\0\x80\xFD[\x80c\xB7\xF0n\xBE\x14a\x07\x84W\x80c\xBBE\xFE\xF2\x14a\x07\xA7W\x80c\xC4H\xFE\xB8\x14a\x07\xD5W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\x01qW\x80c\x91\x04\xC3\x19\x11a\x01KW\x80c\x91\x04\xC3\x19\x14a\x07\x0FW\x80c\x99\xBE\x81\xC8\x14a\x07*W\x80c\xA1x\x84\x84\x14a\x07=W\x80c\xB14Bq\x14a\x07]W`\0\x80\xFD[\x80c\x88o\x11\x95\x14a\x06\xCBW\x80c\x8D\xA5\xCB[\x14a\x06\xDEW\x80c\x90\x04\x13G\x14a\x06\xEFW`\0\x80\xFD[\x80cc[\xBD\x10\x14a\x066W\x80ce\xDA\x12d\x14a\x06IW\x80cmp\xF7\xAE\x14a\x06rW\x80cqP\x18\xA6\x14a\x06\x85W\x80cw\x8EU\xF3\x14a\x06\x8DW\x80c\x7FT\x80q\x14a\x06\xB8W`\0\x80\xFD[\x80c(\xA5s\xAE\x11a\x02\x92W\x80cFe\xBC\xDA\x11a\x020W\x80cY{6\xDA\x11a\x02\nW\x80cY{6\xDA\x14a\x05\xE5W\x80cZ\xC8j\xB7\x14a\x05\xF8W\x80c\\\x97Z\xBB\x14a\x06\x1BW\x80c`\xD7\xFA\xED\x14a\x06#W`\0\x80\xFD[\x80cFe\xBC\xDA\x14a\x05\xACW\x80cO\xC4\x0Ba\x14a\x05\xD3W\x80cY\\jg\x14a\x05\xDDW`\0\x80\xFD[\x80c9\xB7\x0E8\x11a\x02lW\x80c9\xB7\x0E8\x14a\x04\xF4W\x80c<\xDE\xB5\xE0\x14a\x053W\x80c>(9\x1D\x14a\x05bW\x80cC7s\x82\x14a\x05\x85W`\0\x80\xFD[\x80c(\xA5s\xAE\x14a\x04\xAEW\x80c)\xC7}O\x14a\x04\xC1W\x80c3@C\x96\x14a\x04\xE1W`\0\x80\xFD[\x80c\x13-Ig\x11a\x02\xFFW\x80c\x16\x92\x83e\x11a\x02\xD9W\x80c\x16\x92\x83e\x14a\x04(W\x80c\x1B\xBC\xE0\x91\x14a\x04aW\x80c `kp\x14a\x04tW\x80c\"\xBF@\xE4\x14a\x04\x9BW`\0\x80\xFD[\x80c\x13-Ig\x14a\x03\xEFW\x80c\x13d9\xDD\x14a\x04\x02W\x80c\x15\"\xBF\x02\x14a\x04\x15W`\0\x80\xFD[\x80c\x04I\xCA9\x14a\x03GW\x80c\x04\xA4\xF9y\x14a\x03mW\x80c\x0B\x9FHz\x14a\x03\x94W\x80c\r\xD8\xDD\x02\x14a\x03\xA7W\x80c\x0FX\x9EY\x14a\x03\xC7W\x80c\x10\xD6z/\x14a\x03\xDCW[`\0\x80\xFD[a\x03Za\x03U6`\x04aHNV[a\tIV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03Z\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD\x81V[a\x03Za\x03\xA26`\x04aH\xB4V[a\t\xCEV[a\x03\xBAa\x03\xB56`\x04aHNV[a\n\x90V[`@Qa\x03d\x91\x90aI\x0FV[a\x03\xDAa\x03\xD56`\x04aI\xACV[a\r\xF9V[\0[a\x03\xDAa\x03\xEA6`\x04aI\xFFV[a\x0F>V[a\x03\xDAa\x03\xFD6`\x04aJ#V[a\x0F\xF1V[a\x03\xDAa\x04\x106`\x04aJdV[a\x10\xA8V[a\x03\xDAa\x04#6`\x04aJ}V[a\x11\xE7V[a\x03Za\x0466`\x04aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90V[a\x03Za\x04o6`\x04aJ#V[a\x11\xFBV[a\x03Z\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81V[a\x03\xDAa\x04\xA96`\x04aJ\xE8V[a\x12)V[a\x03\xDAa\x04\xBC6`\x04aJ#V[a\x13mV[a\x03Za\x04\xCF6`\x04aI\xFFV[`\x9B` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xDAa\x04\xEF6`\x04aK\x8FV[a\x14\x1DV[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03dV[a\x05\x1Ba\x05A6`\x04aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x90V[a\x05ua\x05p6`\x04aI\xFFV[a\x15ZV[`@Q\x90\x15\x15\x81R` \x01a\x03dV[a\x03Z\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B\x81V[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03Zb\x13\xC6\x80\x81V[a\x03\xDAa\x15zV[a\x03Za\x05\xF36`\x04aN\x8CV[a\x16AV[a\x05ua\x06\x066`\x04aN\xC8V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03ZV[a\x03\xDAa\x0616`\x04aN\xF9V[a\x16qV[a\x03\xDAa\x06D6`\x04aJdV[a\x17\x0CV[a\x05\x1Ba\x06W6`\x04aI\xFFV[`\x9A` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x05ua\x06\x806`\x04aI\xFFV[a\x17\x1DV[a\x03\xDAa\x17WV[a\x03Za\x06\x9B6`\x04aO\x88V[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x03\xDAa\x06\xC66`\x04aPiV[a\x17kV[`eTa\x05\x1B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05\x1BV[a\x07\x02a\x06\xFD6`\x04aP\xF9V[a\x19\x97V[`@Qa\x03d\x91\x90aQ\x83V[a\x05\x1Bs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x03\xDAa\x0786`\x04aQ\x96V[a\x1AqV[a\x03Za\x07K6`\x04aI\xFFV[`\x9F` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x1B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05ua\x07\x926`\x04aJdV[`\x9E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05ua\x07\xB56`\x04aQ\xCBV[`\x9C` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x03Z`\x9DT\x81V[a\x03Za\x07\xEC6`\x04aI\xFFV[`\xA1` R`\0\x90\x81R`@\x90 T\x81V[a\x08na\x08\x0C6`\x04aI\xFFV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x92\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R`\x99\x83R\x83\x90 \x83Q\x91\x82\x01\x84R\x80T\x85\x16\x82R`\x01\x01T\x93\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\xA0\x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x85\x01Q\x90\x91\x16\x90\x82\x01R\x91\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x01a\x03dV[a\x03Za\x08\xB26`\x04aQ\xF7V[a\x1BCV[a\x03Zb\x03K\xC0\x81V[a\x08\xD4a\x08\xCF6`\x04aI\xFFV[a\x1B\xFCV[`@Qa\x03d\x92\x91\x90aRxV[a\x03\xBAa\x08\xF06`\x04aI\xFFV[a\x1F\xB4V[a\x03\xDAa\t\x036`\x04aR\x9DV[a$xV[a\x03\xDAa\t\x166`\x04aR\xF5V[a%\x95V[a\x03\xDAa\t)6`\x04aI\xFFV[a&&V[a\x03Za&\x9CV[a\x03\xDAa\tD6`\x04aJdV[a&\xDAV[`\x9DT`\0\x90\x81[\x83\x81\x10\x15a\t\xC6W`\0`\xA1`\0\x87\x87\x85\x81\x81\x10a\tqWa\tqaS\x11V[\x90P` \x02\x01` \x81\x01\x90a\t\x86\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x82\x81\x11\x15a\t\xB5W\x80\x92P[Pa\t\xBF\x81aS=V[\x90Pa\tQV[P\x93\x92PPPV[`@\x80Q\x7F\x14\xBD\xE6t\xC9\xF6K*\xD0\x0E\xAA\xEEJ\x8B\xED\x1F\xAB\xEF5\xC7P~<[\x9C\xFC\x946\x90\x9A-\xAD` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x83\x85\x01R\x88\x81\x16``\x84\x01R\x87\x16`\x80\x83\x01R`\xA0\x82\x01\x85\x90R`\xC0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\nLa&\x9CV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x98\x97PPPPPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\n\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`@Q\x80\x91\x03\x90\xFD[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xDFWa\n\xDFaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P3`\0\x90\x81R`\x9A` R`@\x81 T\x91\x92P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90[\x85\x81\x10\x15a\r\xEEW\x86\x86\x82\x81\x81\x10a\x0BCWa\x0BCaS\x11V[\x90P` \x02\x81\x01\x90a\x0BU\x91\x90aS\x8FV[a\x0Bc\x90` \x81\x01\x90aS\xAFV[\x90P\x87\x87\x83\x81\x81\x10a\x0BwWa\x0BwaS\x11V[\x90P` \x02\x81\x01\x90a\x0B\x89\x91\x90aS\x8FV[a\x0B\x93\x90\x80aS\xAFV[\x90P\x14a\x0C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: input length mismatch\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[3\x87\x87\x83\x81\x81\x10a\x0C\x1BWa\x0C\x1BaS\x11V[\x90P` \x02\x81\x01\x90a\x0C-\x91\x90aS\x8FV[a\x0C>\x90``\x81\x01\x90`@\x01aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.queueWithdrawa`D\x82\x01R\x7Fl: withdrawer must be staker\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[a\r\xBF3\x83\x89\x89\x85\x81\x81\x10a\x0C\xD1Wa\x0C\xD1aS\x11V[\x90P` \x02\x81\x01\x90a\x0C\xE3\x91\x90aS\x8FV[a\x0C\xF4\x90``\x81\x01\x90`@\x01aI\xFFV[\x8A\x8A\x86\x81\x81\x10a\r\x06Wa\r\x06aS\x11V[\x90P` \x02\x81\x01\x90a\r\x18\x91\x90aS\x8FV[a\r\"\x90\x80aS\xAFV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8E\x92P\x8D\x91P\x88\x90P\x81\x81\x10a\rhWa\rhaS\x11V[\x90P` \x02\x81\x01\x90a\rz\x91\x90aS\x8FV[a\r\x88\x90` \x81\x01\x90aS\xAFV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa(6\x92PPPV[\x83\x82\x81Q\x81\x10a\r\xD1Wa\r\xD1aS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a\r\xE6\x81aS=V[\x91PPa\x0B)V[P\x90\x95\x94PPPPPV[a\x0E\x023a\x15ZV[\x15a\x0E\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager.registerAsOper`D\x82\x01R\x7Fator: caller is already actively`d\x82\x01Ri\x08\x19\x19[\x19Y\xD8]\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x0E\x923\x84a-\xF6V[`@\x80Q\x80\x82\x01\x90\x91R``\x81R`\0` \x82\x01Ra\x0E\xB43\x80\x83`\0a/\xE9V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8E\x84\x85X:#\x10\xD4\x1F|\x82\xB9B}\x0B\xD4\x9B\xADt\xBB\x9C\xFF\x9D4\x02\xA2\x9D\x8F\x9B(\xA0\xE2\x85`@Qa\x0E\xED\x91\x90aS\xF8V[`@Q\x80\x91\x03\x90\xA23`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x84\x84`@Qa\x0F0\x92\x91\x90aTJV[`@Q\x80\x91\x03\x90\xA2PPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xB5\x91\x90aTyV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\x96V[a\x0F\xEE\x81a2\x7FV[PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x10PWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x10lW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\xE0V[a\x10u\x83a\x15ZV[\x15a\x10\xA3W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x10\xA1\x81\x85\x85\x85a3vV[P[PPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x14\x91\x90aU=V[a\x110W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aUZV[`fT\x81\x81\x16\x14a\x11\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[a\x11\xEFa3\xF1V[a\x10\xA1\x84\x84\x84\x84a4KV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x9B` R`@\x81 Ta\x12 \x85\x82\x86\x86a\x1BCV[\x95\x94PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12IWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12cWP0;\x15\x80\x15a\x12cWP`\0T`\xFF\x16`\x01\x14[a\x12\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xE9W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xF3\x88\x88a6qV[a\x12\xFBa7[V[`\x97Ua\x13\x07\x89a7\xF2V[a\x13\x10\x86a8DV[a\x13\x1C\x85\x85\x85\x85a4KV[\x80\x15a\x13bW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x13\xCCWP3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14[a\x13\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\xE0V[a\x13\xF1\x83a\x15ZV[\x15a\x10\xA3W`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16a\x10\xA1\x81\x85\x85\x85a9>V[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x14FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x02`\xC9T\x14\x15a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xBCV[`\x02`\xC9U`\0[\x88\x81\x10\x15a\x15IWa\x159\x8A\x8A\x83\x81\x81\x10a\x14\xBEWa\x14\xBEaS\x11V[\x90P` \x02\x81\x01\x90a\x14\xD0\x91\x90aU\xA2V[\x89\x89\x84\x81\x81\x10a\x14\xE2Wa\x14\xE2aS\x11V[\x90P` \x02\x81\x01\x90a\x14\xF4\x91\x90aS\xAFV[\x89\x89\x86\x81\x81\x10a\x15\x06Wa\x15\x06aS\x11V[\x90P` \x02\x015\x88\x88\x87\x81\x81\x10a\x15\x1FWa\x15\x1FaS\x11V[\x90P` \x02\x01` \x81\x01\x90a\x154\x91\x90aU\xB8V[a9\xB9V[a\x15B\x81aS=V[\x90Pa\x14\xA1V[PP`\x01`\xC9UPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x9A` R`@\x90 T\x16\x15\x15\x90V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xE6\x91\x90aU=V[a\x16\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aUZV[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0\x81`@Q` \x01a\x16T\x91\x90aVIV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x16\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x02`\xC9T\x14\x15a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\n\xBCV[`\x02`\xC9Ua\x16\xFF\x86\x86\x86\x86\x86a9\xB9V[PP`\x01`\xC9UPPPPV[a\x17\x14a3\xF1V[a\x0F\xEE\x81a8DV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x17QWP`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x14[\x92\x91PPV[a\x17_a3\xF1V[a\x17i`\0a7\xF2V[V[B\x83` \x01Q\x10\x15a\x17\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker signature expire`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x17\xF8\x85a\x15ZV[\x15a\x18\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: staker is already activ`d\x82\x01Rl\x19[\x1EH\x19\x19[\x19Y\xD8]\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x18\x8A\x84a\x17\x1DV[a\x19\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FDelegationManager.delegateToBySi`D\x82\x01R\x7Fgnature: operator is not registe`d\x82\x01Rp92\xB2\x104\xB7\x10\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`y\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0`\x9B`\0\x87`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P`\0a\x19R\x87\x83\x88\x88` \x01Qa\x1BCV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x9B` R`@\x90 `\x01\x84\x01\x90U\x85Q\x90\x91Pa\x19\x82\x90\x88\x90\x83\x90aA\xA3V[a\x19\x8E\x87\x87\x86\x86a/\xE9V[PPPPPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xB4Wa\x19\xB4aL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\t\xC6W`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x98` R`@\x81 \x85Q\x90\x91\x90\x86\x90\x84\x90\x81\x10a\x1A\x1BWa\x1A\x1BaS\x11V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x82\x82\x81Q\x81\x10a\x1AVWa\x1AVaS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x1Aj\x81aS=V[\x90Pa\x19\xE3V[a\x1Az3a\x17\x1DV[a\x1A\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FDelegationManager.updateOperator`D\x82\x01R\x7FMetadataURI: caller must be an o`d\x82\x01Rf82\xB90\xBA7\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x02\xA9\x19\xED\x0E*\xCA\xD1\xDD\x90\xF1~\xF2\xFAJ\xE5F.\xE13\x91p\x03J\x851\xCC\xA4\xB6p\x80\x90\x83\x83`@Qa\x1B7\x92\x91\x90aTJV[`@Q\x80\x91\x03\x90\xA2PPV[`@\x80Q\x7F9\x11\x1B\xC4\xA4\xD6\x88\xE1\xF6\x85\x12=t\x97\xD4aSp\x15*\x8E\xE4\xA0Y>d{\xD0j\xD8\xBB\x0B` \x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x83\x85\x01R\x85\x16``\x83\x01R`\x80\x82\x01\x86\x90R`\xA0\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 `\0\x90\x81a\x1B\xB9a&\x9CV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x97\x96PPPPPPPV[`@Qc`\xF4\x06+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R``\x91\x82\x91`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c`\xF4\x06+\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1ClW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x90\x91\x90aV\\V[`@Qc\x94\xF6I\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x94\xF6I\xDD\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1D(\x91\x90\x81\x01\x90aV\xD0V[\x91P\x91P`\0\x83\x13a\x1D?W\x90\x95\x90\x94P\x92PPPV[``\x80\x83Q`\0\x14\x15a\x1D\xF9W`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x90` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x94P\x90P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\0\x81Q\x81\x10a\x1D\xB4Wa\x1D\xB4aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\0\x81Q\x81\x10a\x1D\xE8Wa\x1D\xE8aS\x11V[` \x02` \x01\x01\x81\x81RPPa\x1F\xA7V[\x83Qa\x1E\x06\x90`\x01aW\x8AV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x1DWa\x1E\x1DaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EbWa\x1EbaL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\x8BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84Q\x81\x10\x15a\x1F%W\x84\x81\x81Q\x81\x10a\x1E\xACWa\x1E\xACaS\x11V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\x1E\xC6Wa\x1E\xC6aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x81\x81Q\x81\x10a\x1E\xF8Wa\x1E\xF8aS\x11V[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x1F\x12Wa\x1F\x12aS\x11V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1E\x91V[Ps\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x82`\x01\x84Qa\x1FJ\x91\x90aW\xA2V[\x81Q\x81\x10a\x1FZWa\x1FZaS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x84\x81`\x01\x84Qa\x1F\x8A\x91\x90aW\xA2V[\x81Q\x81\x10a\x1F\x9AWa\x1F\x9AaS\x11V[` \x02` \x01\x01\x81\x81RPP[\x90\x97\x90\x96P\x94PPPPPV[`fT``\x90`\x01\x90`\x02\x90\x81\x16\x14\x15a\x1F\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[a\x1F\xE9\x83a\x15ZV[a iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FDelegationManager.undelegate: st\x90\x82\x01R\x7Faker must be delegated to undele`d\x82\x01Rcgate`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a r\x83a\x17\x1DV[\x15a \xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: op`D\x82\x01R\x7Ferators cannot be undelegated\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16a!aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fnnot undelegate zero address\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x81\x81R`\x9A` R`@\x90 T\x90\x91\x16\x903\x14\x80a!\x94WP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14[\x80a!\xBBWP`\x01`\x01`\xA0\x1B\x03\x81\x81\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x163\x14[a\"-W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelegationManager.undelegate: ca`D\x82\x01R\x7Fller cannot undelegate staker\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80a\"9\x86a\x1B\xFCV[\x90\x92P\x90P3`\x01`\x01`\xA0\x1B\x03\x87\x16\x14a\"\x8FW\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\xED\xDF\x07\xE6\xEA\x14\xF3\x88\xB4~\x1E\x94\xA0\xF4d\xEC\xBD\x9E\xEDAq\x13\x0E\x0F\xC0\xE9\x9F\xB4\x03\n\x8A`@Q`@Q\x80\x91\x03\x90\xA3[\x82`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xE3\tf\xA2V\xB7\x1E\x14\xBC\x0E\xBF\xC9C\x15\xE2\x8E\xF4\xA9zq1\xA9\xE2\xB7\xA3\x10\xA7:\xF4Fv`@Q`@Q\x80\x91\x03\x90\xA3`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9A` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x81Qa#\x11W`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x94Pa$oV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#*Wa#*aL1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P`\0[\x82Q\x81\x10\x15a$mW`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R\x92\x93P`\0\x92\x91P` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x83\x81Q\x81\x10a#\xB9Wa#\xB9aS\x11V[` \x02` \x01\x01Q\x82`\0\x81Q\x81\x10a#\xD4Wa#\xD4aS\x11V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x83\x83\x81Q\x81\x10a$\x06Wa$\x06aS\x11V[` \x02` \x01\x01Q\x81`\0\x81Q\x81\x10a$!Wa$!aS\x11V[` \x02` \x01\x01\x81\x81RPPa$:\x89\x87\x8B\x85\x85a(6V[\x88\x84\x81Q\x81\x10a$LWa$LaS\x11V[` \x02` \x01\x01\x81\x81RPPPP\x80\x80a$e\x90aS=V[\x91PPa#YV[P[PPPP\x91\x90PV[a$\x813a\x15ZV[\x15a$\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FDelegationManager.delegateTo: st`D\x82\x01R\x7Faker is already actively delegat`d\x82\x01Ra\x19Y`\xF2\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a%\x08\x83a\x17\x1DV[a%\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FDelegationManager.delegateTo: op`D\x82\x01R\x7Ferator is not registered in Eige`d\x82\x01Re7&0\xBC\xB2\xB9`\xD1\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x10\xA33\x84\x84\x84a/\xE9V[a%\x9E3a\x17\x1DV[a&\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FDelegationManager.modifyOperator`D\x82\x01R\x7FDetails: caller must be an opera`d\x82\x01Rb:7\xB9`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[a\x0F\xEE3\x82a-\xF6V[a&.a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a&\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[a\x0F\xEE\x81a7\xF2V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a&\xCDWP`\x97T\x90V[a&\xD5a7[V[\x90P\x90V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'-W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'Q\x91\x90aTyV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a'\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aT\x96V[`fT\x19\x81\x19`fT\x19\x16\x14a'\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x11\xDCV[`\0`\x01`\x01`\xA0\x1B\x03\x86\x16a(\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: staker cannot`d\x82\x01Ro be zero address`\x80\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82Qa)WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FDelegationManager._removeSharesA`D\x82\x01R\x7FndQueueWithdrawal: strategies ca`d\x82\x01Rlnnot be empty`\x98\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0[\x83Q\x81\x10\x15a-\x04W`\x01`\x01`\xA0\x1B\x03\x86\x16\x15a)\xB0Wa)\xB0\x86\x88\x86\x84\x81Q\x81\x10a)\x89Wa)\x89aS\x11V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a)\xA3Wa)\xA3aS\x11V[` \x02` \x01\x01Qa3vV[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a)\xE0Wa)\xE0aS\x11V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a*\xA9W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xBE\xFF\xBB\x89\x88\x85\x84\x81Q\x81\x10a*9Wa*9aS\x11V[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*r\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a*\xA0W=`\0\x80>=`\0\xFD[PPPPa,\xFCV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a+{WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x9BM\xA0=\x85\x83\x81Q\x81\x10a+\x05Wa+\x05aS\x11V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a+8\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+y\x91\x90aU=V[\x15[a,GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x84`$\x82\x01\x81\x90R\x7FDelegationManager._removeSharesA`D\x83\x01R\x7FndQueueWithdrawal: withdrawer mu`d\x83\x01R\x7Fst be same address as staker if \x90\x82\x01R\x7FthirdPartyTransfersForbidden are`\xA4\x82\x01Rc\x08\x1C\xD9]`\xE2\x1B`\xC4\x82\x01R`\xE4\x01a\n\xBCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x80\xD4\xE5\x88\x86\x84\x81Q\x81\x10a,\x89Wa,\x89aS\x11V[` \x02` \x01\x01Q\x86\x85\x81Q\x81\x10a,\xA3Wa,\xA3aS\x11V[` \x02` \x01\x01Q`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xC9\x93\x92\x91\x90aW\xB9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,\xE3W`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\xF7W=`\0\x80>=`\0\xFD[PPPP[`\x01\x01a)ZV[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x9F` R`@\x81 \x80T\x91\x82\x91\x90a-,\x83aS=V[\x91\x90PUP`\0`@Q\x80`\xE0\x01`@R\x80\x89`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83\x81R` \x01Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86\x81R` \x01\x85\x81RP\x90P`\0a-\x94\x82a\x16AV[`\0\x81\x81R`\x9E` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x90\x91P\x7F\x90\t\xAB\x15>\x80\x14\xFB\xFB\x02\xF2!\x7F\\\xDEz\xA7\xF9\xADsJ\xE8\\\xA3\xEE?L\xA2\xFD\xD4\x99\xF9\x90a-\xE2\x90\x83\x90\x85\x90aW\xDDV[`@Q\x80\x91\x03\x90\xA1\x98\x97PPPPPPPPV[b\x13\xC6\x80a.\n``\x83\x01`@\x84\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16\x11\x15a.\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`l`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01R\x7Fcannot be > MAX_STAKER_OPT_OUT_W`\x84\x82\x01RkINDOW_BLOCKS`\xA0\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90\x81\x90 `\x01\x01T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90a.\xFB\x90``\x84\x01\x90\x84\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16\x10\x15a/\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FDelegationManager._setOperatorDe`D\x82\x01R\x7Ftails: stakerOptOutWindowBlocks `d\x82\x01Rr\x18\xD8[\x9B\x9B\xDD\x08\x18\x99H\x19\x19X\xDC\x99X\\\xD9Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x99` R`@\x90 \x81\x90a/\xB5\x82\x82aX3V[PP`@Q3\x90\x7F\xFE\xBE\\\xD2K,\xBC{\x06[\x9D\x0F\xDE\xB9\x04F\x1EJ\xFC\xFFW\xDDW\xAC\xDA\x1Ex2\x03\x1B\xA7\xAC\x90a\x1B7\x90\x84\x90aS\xF8V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a0\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aSXV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` R`@\x90 `\x01\x01T\x16\x80\x15\x80\x15\x90a0HWP3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x15[\x80\x15a0]WP3`\x01`\x01`\xA0\x1B\x03\x86\x16\x14\x15[\x15a1\xCAWB\x84` \x01Q\x10\x15a0\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7Frover signature expired\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 T`\xFF\x16\x15a1vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FDelegationManager._delegate: app`D\x82\x01R\x7FroverSalt already spent\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9C` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\xFF\x19\x16`\x01\x17\x90U\x85\x01Qa1\xB7\x90\x88\x90\x88\x90\x85\x90\x88\x90a\t\xCEV[\x90Pa1\xC8\x82\x82\x87`\0\x01QaA\xA3V[P[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x81\x81R`\x9A` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x94\x8A\x16\x94\x85\x17\x90UQ\x7F\xC3\xEE\x9F._\xDA\x98\xE8\x06j\x1Ft[-\xF9(_Ao\xE9\x8C\xF2U\x9C\xD2\x14\x84\xB3\xD8t3\x04\x91\x90\xA3`\0\x80a2)\x88a\x1B\xFCV[\x91P\x91P`\0[\x82Q\x81\x10\x15a\x13bWa2w\x88\x8A\x85\x84\x81Q\x81\x10a2PWa2PaS\x11V[` \x02` \x01\x01Q\x85\x85\x81Q\x81\x10a2jWa2jaS\x11V[` \x02` \x01\x01Qa9>V[`\x01\x01a20V[`\x01`\x01`\xA0\x1B\x03\x81\x16a3\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a3\xAD\x90\x84\x90aW\xA2V[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7Fi\t`\x007\xB7]{G3\xAE\xDD\x81TB\xB5\xEC\x01\x8A\x82wQ\xC82\xAA\xFFd\xEB\xA5\xD6\xD2\xDD\x84\x84\x84`@Qa\x0F0\x93\x92\x91\x90aW\xB9V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\n\xBCV[\x82\x81\x14a4\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: input lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\0[\x81\x81\x10\x15a6iW`\0\x86\x86\x83\x81\x81\x10a4\xF3Wa4\xF3aS\x11V[\x90P` \x02\x01` \x81\x01\x90a5\x08\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xA1` R`@\x81 T\x91\x92P\x86\x86\x85\x81\x81\x10a56Wa56aS\x11V[\x90P` \x02\x015\x90Pb\x03K\xC0\x81\x11\x15a5\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`s`$\x82\x01R\x7FDelegationManager._setStrategyWi`D\x82\x01R\x7FthdrawalDelayBlocks: _withdrawal`d\x82\x01R\x7FDelayBlocks cannot be > MAX_WITH`\x84\x82\x01RrDRAWAL_DELAY_BLOCKS`h\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\xA1` \x90\x81R`@\x91\x82\x90 \x84\x90U\x81Q\x92\x83R\x82\x01\x84\x90R\x81\x01\x82\x90R\x7F\x0E~\xFAs\x8E\x8B\x0C\xE67j\x0C\x1A\xF4qeU@\xD2\xE9\xA8\x16G\xD7\xB0\x9E\xD8#\x01\x84&Wm\x90``\x01`@Q\x80\x91\x03\x90\xA1PPP\x80a6b\x90aS=V[\x90Pa4\xD7V[PPPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a6\x92WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a7\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a7W\x82a2\x7FV[PPV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[b\x03K\xC0\x81\x11\x15a8\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`q`$\x82\x01R\x7FDelegationManager._setMinWithdra`D\x82\x01R\x7FwalDelayBlocks: _minWithdrawalDe`d\x82\x01R\x7FlayBlocks cannot be > MAX_WITHDR`\x84\x82\x01RpAWAL_DELAY_BLOCKS`x\x1B`\xA4\x82\x01R`\xC4\x01a\n\xBCV[`\x9DT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xAF\xA0\x03\xCDv\xF8\x7F\xF9\xD6+5\xBE\xEA\x88\x99 \xF3<\x0CB\xB8\xD4[t\x95Ma\xD5\x0FKki\x91\x01`@Q\x80\x91\x03\x90\xA1`\x9DUV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x98` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a9u\x90\x84\x90aW\x8AV[\x92PP\x81\x90UP\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x1E\xC0B\xC9e\xE2\xED\xD7\x10{Q\x18\x8E\xE0\xF3\x83\xE2.v\x17\x90A\xAB:\x9D\x18\xFF\x15\x14\x05\x16l\x84\x84\x84`@Qa\x0F0\x93\x92\x91\x90aW\xB9V[`\0a9\xC7a\x05\xF3\x87aX\x96V[`\0\x81\x81R`\x9E` R`@\x90 T\x90\x91P`\xFF\x16a:HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: action is not in qu`d\x82\x01Rbeue`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x9DTC\x90a:]`\xA0\x89\x01`\x80\x8A\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a:m\x91\x90aW\x8AV[\x11\x15a:\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`_`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: minWithdrawalDelayB`d\x82\x01R\x7Flocks period has not yet passed\0`\x84\x82\x01R`\xA4\x01a\n\xBCV[a;\x05``\x87\x01`@\x88\x01aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a;\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: only withdrawer can`d\x82\x01Ro\x101\xB7\xB6\xB862\xBA2\x900\xB1\xBA4\xB7\xB7`\x81\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x81\x15a<\x14Wa;\xA5`\xA0\x87\x01\x87aS\xAFV[\x85\x14\x90Pa<\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`D\x82\x01R\x7FdWithdrawal: input length mismat`d\x82\x01Ra\x0Cm`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\0\x81\x81R`\x9E` R`@\x90 \x80T`\xFF\x19\x16\x90U\x81\x15a=yW`\0[a<@`\xA0\x88\x01\x88aS\xAFV[\x90P\x81\x10\x15a=sWC`\xA1`\0a<[`\xA0\x8B\x01\x8BaS\xAFV[\x85\x81\x81\x10a<kWa<kaS\x11V[\x90P` \x02\x01` \x81\x01\x90a<\x80\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta<\xAA`\xA0\x8A\x01`\x80\x8B\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a<\xBA\x91\x90aW\x8AV[\x11\x15a<\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aX\xA2V[a=ka<\xE8` \x89\x01\x89aI\xFFV[3a<\xF6`\xA0\x8B\x01\x8BaS\xAFV[\x85\x81\x81\x10a=\x06Wa=\x06aS\x11V[\x90P` \x02\x01` \x81\x01\x90a=\x1B\x91\x90aI\xFFV[a=(`\xC0\x8C\x01\x8CaS\xAFV[\x86\x81\x81\x10a=8Wa=8aS\x11V[\x90P` \x02\x015\x8A\x8A\x87\x81\x81\x10a=QWa=QaS\x11V[\x90P` \x02\x01` \x81\x01\x90a=f\x91\x90aI\xFFV[aC]V[`\x01\x01a<3V[PaAhV[3`\0\x90\x81R`\x9A` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x90[a=\xA1`\xA0\x89\x01\x89aS\xAFV[\x90P\x81\x10\x15aAeWC`\xA1`\0a=\xBC`\xA0\x8C\x01\x8CaS\xAFV[\x85\x81\x81\x10a=\xCCWa=\xCCaS\x11V[\x90P` \x02\x01` \x81\x01\x90a=\xE1\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 Ta>\x0B`\xA0\x8B\x01`\x80\x8C\x01aW\xF6V[c\xFF\xFF\xFF\xFF\x16a>\x1B\x91\x90aW\x8AV[\x11\x15a>9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\xBC\x90aX\xA2V[s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0a>[`\xA0\x8A\x01\x8AaS\xAFV[\x83\x81\x81\x10a>kWa>kaS\x11V[\x90P` \x02\x01` \x81\x01\x90a>\x80\x91\x90aI\xFFV[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a?\xD0W`\0a>\x9E` \x8A\x01\x8AaI\xFFV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0E\x81\x07<\x83a>\xDF`\xC0\x8E\x01\x8EaS\xAFV[\x87\x81\x81\x10a>\xEFWa>\xEFaS\x11V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x87\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\x04\x85\x01R` \x02\x91\x90\x91\x015`$\x83\x01RP`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a?CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?g\x91\x90aV\\V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\0\x90\x81R`\x9A` R`@\x90 T\x91\x92P\x16\x80\x15a?\xC8Wa?\xC8\x81\x84a?\x9D`\xA0\x8F\x01\x8FaS\xAFV[\x88\x81\x81\x10a?\xADWa?\xADaS\x11V[\x90P` \x02\x01` \x81\x01\x90a?\xC2\x91\x90aI\xFFV[\x85a9>V[PPPaA]V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xC4b>\xA13\x89\x89\x85\x81\x81\x10a@\x12Wa@\x12aS\x11V[\x90P` \x02\x01` \x81\x01\x90a@'\x91\x90aI\xFFV[a@4`\xA0\x8D\x01\x8DaS\xAFV[\x86\x81\x81\x10a@DWa@DaS\x11V[\x90P` \x02\x01` \x81\x01\x90a@Y\x91\x90aI\xFFV[a@f`\xC0\x8E\x01\x8EaS\xAFV[\x87\x81\x81\x10a@vWa@vaS\x11V[`@Q`\xE0\x88\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16`\x04\x82\x01R\x94\x86\x16`$\x86\x01R\x92\x90\x94\x16`D\x84\x01R` \x90\x91\x02\x015`d\x82\x01R`\x84\x01\x90P`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a@\xD6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a@\xEAW=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15aA]WaA]\x823aA\x0F`\xA0\x8C\x01\x8CaS\xAFV[\x85\x81\x81\x10aA\x1FWaA\x1FaS\x11V[\x90P` \x02\x01` \x81\x01\x90aA4\x91\x90aI\xFFV[aAA`\xC0\x8D\x01\x8DaS\xAFV[\x86\x81\x81\x10aAQWaAQaS\x11V[\x90P` \x02\x015a9>V[`\x01\x01a=\x94V[PP[`@Q\x81\x81R\x7F\xC9p\x98\xC2\xF6X\x80\x0BM\xF2\x90\x01R\x7Fs$\xBC\xDF\xFC\xF6\xE8u\x1Ai\x9A\xB9 \xA1\xEC\xED[\x1D\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15aB\xBDW`@Qc\x0B\x13]?`\xE1\x1B\x80\x82R\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x16&\xBA~\x90aA\xE3\x90\x86\x90\x86\x90`\x04\x01aY*V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aB\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aB$\x91\x90aY\x87V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: ERC1271 signature `d\x82\x01Rr\x1D\x99\\\x9AY\x9AX\xD8]\x1A[\xDB\x88\x19\x98Z[\x19Y`j\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[\x82`\x01`\x01`\xA0\x1B\x03\x16aB\xD1\x83\x83aD\x9DV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x10\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEIP1271SignatureUtils.checkSigna`D\x82\x01R\x7Fture_EIP1271: signature not from`d\x82\x01Rf\x109\xB4\xB3\xB72\xB9`\xC9\x1B`\x84\x82\x01R`\xA4\x01a\n\xBCV[`\x01`\x01`\xA0\x1B\x03\x83\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14\x15aD\x08W`@Qb8{\x13`\xE8\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c8{\x13\0\x90aC\xD1\x90\x88\x90\x88\x90\x87\x90`\x04\x01aW\xB9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aC\xEBW`\0\x80\xFD[PZ\xF1\x15\x80\x15aC\xFFW=`\0\x80>=`\0\xFD[PPPPaD\x96V[`@Qc\xC6\x08\xC7\xF3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x84\x81\x16`$\x83\x01R`D\x82\x01\x84\x90R\x82\x81\x16`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xC6\x08\xC7\xF3\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15aD\x82W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13bW=`\0\x80>=`\0\xFD[PPPPPV[`\0\x80`\0aD\xAC\x85\x85aD\xB9V[\x91P\x91Pa\t\xC6\x81aE)V[`\0\x80\x82Q`A\x14\x15aD\xF0W` \x83\x01Q`@\x84\x01Q``\x85\x01Q`\0\x1AaD\xE4\x87\x82\x85\x85aF\xE4V[\x94P\x94PPPPaE\"V[\x82Q`@\x14\x15aE\x1AW` \x83\x01Q`@\x84\x01QaE\x0F\x86\x83\x83aG\xD1V[\x93P\x93PPPaE\"V[P`\0\x90P`\x02[\x92P\x92\x90PV[`\0\x81`\x04\x81\x11\x15aE=WaE=aY\xB1V[\x14\x15aEFWPV[`\x01\x81`\x04\x81\x11\x15aEZWaEZaY\xB1V[\x14\x15aE\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\n\xBCV[`\x02\x81`\x04\x81\x11\x15aE\xBCWaE\xBCaY\xB1V[\x14\x15aF\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\n\xBCV[`\x03\x81`\x04\x81\x11\x15aF\x1EWaF\x1EaY\xB1V[\x14\x15aFwW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\x04\x81`\x04\x81\x11\x15aF\x8BWaF\x8BaY\xB1V[\x14\x15a\x0F\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\n\xBCV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aG\x1BWP`\0\x90P`\x03aG\xC8V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aG3WP\x84`\xFF\x16`\x1C\x14\x15[\x15aGDWP`\0\x90P`\x04aG\xC8V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aG\x98W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aG\xC1W`\0`\x01\x92P\x92PPaG\xC8V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81aG\xEE`\xFF\x86\x90\x1C`\x1BaW\x8AV[\x90PaG\xFC\x87\x82\x88\x85aF\xE4V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12aH\x1CW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH3W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aE\"W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15aHaW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aHwW`\0\x80\xFD[aH\x83\x85\x82\x86\x01aH\nV[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\xEEW`\0\x80\xFD[\x805aH\xAF\x81aH\x8FV[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aH\xCCW`\0\x80\xFD[\x855aH\xD7\x81aH\x8FV[\x94P` \x86\x015aH\xE7\x81aH\x8FV[\x93P`@\x86\x015aH\xF7\x81aH\x8FV[\x94\x97\x93\x96P\x93\x94``\x81\x015\x94P`\x80\x015\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aIGW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aI+V[P\x90\x96\x95PPPPPPV[`\0``\x82\x84\x03\x12\x15aIeW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aI}W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x94W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aE\"W`\0\x80\xFD[`\0\x80`\0`\x80\x84\x86\x03\x12\x15aI\xC1W`\0\x80\xFD[aI\xCB\x85\x85aISV[\x92P``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xE6W`\0\x80\xFD[aI\xF2\x86\x82\x87\x01aIkV[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aJ\x11W`\0\x80\xFD[\x815aJ\x1C\x81aH\x8FV[\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15aJ8W`\0\x80\xFD[\x835aJC\x81aH\x8FV[\x92P` \x84\x015aJS\x81aH\x8FV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15aJvW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15aJ\x93W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\xAAW`\0\x80\xFD[aJ\xB6\x88\x83\x89\x01aH\nV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aJ\xCFW`\0\x80\xFD[PaJ\xDC\x87\x82\x88\x01aH\nV[\x95\x98\x94\x97P\x95PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aK\x04W`\0\x80\xFD[\x885aK\x0F\x81aH\x8FV[\x97P` \x89\x015aK\x1F\x81aH\x8FV[\x96P`@\x89\x015\x95P``\x89\x015\x94P`\x80\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aKIW`\0\x80\xFD[aKU\x8C\x83\x8D\x01aH\nV[\x90\x96P\x94P`\xA0\x8B\x015\x91P\x80\x82\x11\x15aKnW`\0\x80\xFD[PaK{\x8B\x82\x8C\x01aH\nV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\x80\x89\x8B\x03\x12\x15aK\xABW`\0\x80\xFD[\x885`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aK\xC2W`\0\x80\xFD[aK\xCE\x8C\x83\x8D\x01aH\nV[\x90\x9AP\x98P` \x8B\x015\x91P\x80\x82\x11\x15aK\xE7W`\0\x80\xFD[aK\xF3\x8C\x83\x8D\x01aH\nV[\x90\x98P\x96P`@\x8B\x015\x91P\x80\x82\x11\x15aL\x0CW`\0\x80\xFD[aL\x18\x8C\x83\x8D\x01aH\nV[\x90\x96P\x94P``\x8B\x015\x91P\x80\x82\x11\x15aKnW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aLiWaLiaL1V[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aLiWaLiaL1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aL\xB9WaL\xB9aL1V[`@R\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xEEW`\0\x80\xFD[\x805aH\xAF\x81aL\xC1V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xF7WaL\xF7aL1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aM\x12W`\0\x80\xFD[\x815` aM'aM\"\x83aL\xDEV[aL\x91V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aMFW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x805aM]\x81aH\x8FV[\x83R\x91\x83\x01\x91\x83\x01aMJV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aM\x86W`\0\x80\xFD[\x815` aM\x96aM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aM\xB5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x805\x83R\x91\x83\x01\x91\x83\x01aM\xB9V[`\0`\xE0\x82\x84\x03\x12\x15aM\xE2W`\0\x80\xFD[aM\xEAaLGV[\x90PaM\xF5\x82aH\xA4V[\x81RaN\x03` \x83\x01aH\xA4V[` \x82\x01RaN\x14`@\x83\x01aH\xA4V[`@\x82\x01R``\x82\x015``\x82\x01RaN/`\x80\x83\x01aL\xD3V[`\x80\x82\x01R`\xA0\x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aNNW`\0\x80\xFD[aNZ\x85\x83\x86\x01aM\x01V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15aNsW`\0\x80\xFD[PaN\x80\x84\x82\x85\x01aMuV[`\xC0\x83\x01RP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aN\x9EW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xB4W`\0\x80\xFD[aN\xC0\x84\x82\x85\x01aM\xD0V[\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aN\xDAW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14aJ\x1CW`\0\x80\xFD[\x80\x15\x15\x81\x14a\x0F\xEEW`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15aO\x11W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO(W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15aO<W`\0\x80\xFD[\x90\x95P` \x87\x015\x90\x80\x82\x11\x15aORW`\0\x80\xFD[PaO_\x88\x82\x89\x01aH\nV[\x90\x95P\x93PP`@\x86\x015\x91P``\x86\x015aOz\x81aN\xEBV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15aO\x9BW`\0\x80\xFD[\x825aO\xA6\x81aH\x8FV[\x91P` \x83\x015aO\xB6\x81aH\x8FV[\x80\x91PP\x92P\x92\x90PV[`\0`@\x82\x84\x03\x12\x15aO\xD3W`\0\x80\xFD[aO\xDBaLoV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aO\xF4W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aP\x08W`\0\x80\xFD[\x815` \x82\x82\x11\x15aP\x1CWaP\x1CaL1V[aP.`\x1F\x83\x01`\x1F\x19\x16\x82\x01aL\x91V[\x92P\x81\x83R\x86\x81\x83\x86\x01\x01\x11\x15aPDW`\0\x80\xFD[\x81\x81\x85\x01\x82\x85\x017`\0\x81\x83\x85\x01\x01R\x82\x85R\x80\x86\x015\x81\x86\x01RPPPP\x92\x91PPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15aP\x81W`\0\x80\xFD[\x855aP\x8C\x81aH\x8FV[\x94P` \x86\x015aP\x9C\x81aH\x8FV[\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aP\xB8W`\0\x80\xFD[aP\xC4\x89\x83\x8A\x01aO\xC1V[\x94P``\x88\x015\x91P\x80\x82\x11\x15aP\xDAW`\0\x80\xFD[PaP\xE7\x88\x82\x89\x01aO\xC1V[\x95\x98\x94\x97P\x92\x95`\x80\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15aQ\x0CW`\0\x80\xFD[\x825aQ\x17\x81aH\x8FV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ2W`\0\x80\xFD[aQ>\x85\x82\x86\x01aM\x01V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aQxW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aQ\\V[P\x94\x95\x94PPPPPV[` \x81R`\0aJ\x1C` \x83\x01\x84aQHV[`\0\x80` \x83\x85\x03\x12\x15aQ\xA9W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xBFW`\0\x80\xFD[aH\x83\x85\x82\x86\x01aIkV[`\0\x80`@\x83\x85\x03\x12\x15aQ\xDEW`\0\x80\xFD[\x825aQ\xE9\x81aH\x8FV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aR\rW`\0\x80\xFD[\x845aR\x18\x81aH\x8FV[\x93P` \x85\x015\x92P`@\x85\x015aR/\x81aH\x8FV[\x93\x96\x92\x95P\x92\x93``\x015\x92PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aQxW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aRSV[`@\x81R`\0aR\x8B`@\x83\x01\x85aR?V[\x82\x81\x03` \x84\x01Ra\x12 \x81\x85aQHV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\xB2W`\0\x80\xFD[\x835aR\xBD\x81aH\x8FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\xD8W`\0\x80\xFD[aR\xE4\x86\x82\x87\x01aO\xC1V[\x92PP`@\x84\x015\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15aS\x07W`\0\x80\xFD[aJ\x1C\x83\x83aISV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aSQWaSQaS'V[P`\x01\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aS\xA5W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aS\xC6W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aS\xE0W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\"W`\0\x80\xFD[``\x81\x01\x825aT\x07\x81aH\x8FV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x84\x015\x90aT#\x82aH\x8FV[\x16` \x83\x01R`@\x83\x015aT7\x81aL\xC1V[c\xFF\xFF\xFF\xFF\x81\x16`@\x84\x01RP\x92\x91PPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017`\0\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x91\x90PV[`\0` \x82\x84\x03\x12\x15aT\x8BW`\0\x80\xFD[\x81QaJ\x1C\x81aH\x8FV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`7\x90\x82\x01R\x7FDelegationManager: onlyStrategyM`@\x82\x01R\x7FanagerOrEigenPodManager\0\0\0\0\0\0\0\0\0``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aUOW`\0\x80\xFD[\x81QaJ\x1C\x81aN\xEBV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`\xDE\x19\x836\x03\x01\x81\x12aS\xA5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aU\xCAW`\0\x80\xFD[\x815aJ\x1C\x81aN\xEBV[`\0`\x01\x80`\xA0\x1B\x03\x80\x83Q\x16\x84R\x80` \x84\x01Q\x16` \x85\x01R\x80`@\x84\x01Q\x16`@\x85\x01RP``\x82\x01Q``\x84\x01Rc\xFF\xFF\xFF\xFF`\x80\x83\x01Q\x16`\x80\x84\x01R`\xA0\x82\x01Q`\xE0`\xA0\x85\x01RaV0`\xE0\x85\x01\x82aR?V[\x90P`\xC0\x83\x01Q\x84\x82\x03`\xC0\x86\x01Ra\x12 \x82\x82aQHV[` \x81R`\0aJ\x1C` \x83\x01\x84aU\xD5V[`\0` \x82\x84\x03\x12\x15aVnW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82`\x1F\x83\x01\x12aV\x86W`\0\x80\xFD[\x81Q` aV\x96aM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aV\xB5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aMjW\x80Q\x83R\x91\x83\x01\x91\x83\x01aV\xB9V[`\0\x80`@\x83\x85\x03\x12\x15aV\xE3W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aV\xFAW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aW\x0EW`\0\x80\xFD[\x81Q` aW\x1EaM\"\x83aL\xDEV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15aW=W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15aWdW\x85QaWU\x81aH\x8FV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90aWBV[\x91\x88\x01Q\x91\x96P\x90\x93PPP\x80\x82\x11\x15aW}W`\0\x80\xFD[PaQ>\x85\x82\x86\x01aVuV[`\0\x82\x19\x82\x11\x15aW\x9DWaW\x9DaS'V[P\x01\x90V[`\0\x82\x82\x10\x15aW\xB4WaW\xB4aS'V[P\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[\x82\x81R`@` \x82\x01R`\0aN\xC0`@\x83\x01\x84aU\xD5V[`\0` \x82\x84\x03\x12\x15aX\x08W`\0\x80\xFD[\x815aJ\x1C\x81aL\xC1V[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x815aX>\x81aH\x8FV[aXH\x81\x83aX\x13V[P`\x01\x81\x01` \x83\x015aX[\x81aH\x8FV[aXe\x81\x83aX\x13V[P`@\x83\x015aXt\x81aL\xC1V[\x81Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPPV[`\0a\x17Q6\x83aM\xD0V[` \x80\x82R`n\x90\x82\x01R`\0\x80Q` aY\xC8\x839\x81Q\x91R`@\x82\x01R\x7FdWithdrawal: withdrawalDelayBloc``\x82\x01R\x7Fks period has not yet passed for`\x80\x82\x01Rm this strategy`\x90\x1B`\xA0\x82\x01R`\xC0\x01\x90V[\x82\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15aY^W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01aYBV[\x81\x81\x11\x15aYpW`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aY\x99W`\0\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14aJ\x1CW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD\xFEDelegationManager._completeQueue\xA2dipfsX\"\x12 &\xB1\xFE\xD4\x84\x88\x18C\xA1\xD9\x81\x1EI?\xAB\xDFi?\x06\x8A-\xFA:\xF9(\x9C*\x7F\xBAt\x87>dsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MinWithdrawalDelayBlocksSet(uint256,uint256)` and selector `0xafa003cd76f87ff9d62b35beea889920f33c0c42b8d45b74954d61d50f4b6b69`.
    ```solidity
    event MinWithdrawalDelayBlocksSet(uint256 previousValue, uint256 newValue);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinWithdrawalDelayBlocksSet {
        #[allow(missing_docs)]
        pub previousValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MinWithdrawalDelayBlocksSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinWithdrawalDelayBlocksSet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    175u8, 160u8, 3u8, 205u8, 118u8, 248u8, 127u8, 249u8, 214u8, 43u8, 53u8, 190u8,
                    234u8, 136u8, 153u8, 32u8, 243u8, 60u8, 12u8, 66u8, 184u8, 212u8, 91u8, 116u8,
                    149u8, 77u8, 97u8, 213u8, 15u8, 75u8, 107u8, 105u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousValue: data.0,
                    newValue: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.previousValue,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newValue,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinWithdrawalDelayBlocksSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinWithdrawalDelayBlocksSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinWithdrawalDelayBlocksSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDetailsModified(address,(address,address,uint32))` and selector `0xfebe5cd24b2cbc7b065b9d0fdeb904461e4afcff57dd57acda1e7832031ba7ac`.
    ```solidity
    event OperatorDetailsModified(address indexed operator, IDelegationManager.OperatorDetails newOperatorDetails);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDetailsModified {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOperatorDetails:
            <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorDetailsModified {
            type DataTuple<'a> = (IDelegationManager::OperatorDetails,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorDetailsModified(address,(address,address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    254u8, 190u8, 92u8, 210u8, 75u8, 44u8, 188u8, 123u8, 6u8, 91u8, 157u8, 15u8,
                    222u8, 185u8, 4u8, 70u8, 30u8, 74u8, 252u8, 255u8, 87u8, 221u8, 87u8, 172u8,
                    218u8, 30u8, 120u8, 50u8, 3u8, 27u8, 167u8, 172u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    newOperatorDetails: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IDelegationManager::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorDetails,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDetailsModified {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDetailsModified> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDetailsModified) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorMetadataURIUpdated(address,string)` and selector `0x02a919ed0e2acad1dd90f17ef2fa4ae5462ee1339170034a8531cca4b6708090`.
    ```solidity
    event OperatorMetadataURIUpdated(address indexed operator, string metadataURI);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorMetadataURIUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorMetadataURIUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    2u8, 169u8, 25u8, 237u8, 14u8, 42u8, 202u8, 209u8, 221u8, 144u8, 241u8, 126u8,
                    242u8, 250u8, 74u8, 229u8, 70u8, 46u8, 225u8, 51u8, 145u8, 112u8, 3u8, 74u8,
                    133u8, 49u8, 204u8, 164u8, 182u8, 112u8, 128u8, 144u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    metadataURI: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorMetadataURIUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorMetadataURIUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorMetadataURIUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,(address,address,uint32))` and selector `0x8e8485583a2310d41f7c82b9427d0bd49bad74bb9cff9d3402a29d8f9b28a0e2`.
    ```solidity
    event OperatorRegistered(address indexed operator, IDelegationManager.OperatorDetails operatorDetails);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorDetails:
            <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = (IDelegationManager::OperatorDetails,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,(address,address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    142u8, 132u8, 133u8, 88u8, 58u8, 35u8, 16u8, 212u8, 31u8, 124u8, 130u8, 185u8,
                    66u8, 125u8, 11u8, 212u8, 155u8, 173u8, 116u8, 187u8, 156u8, 255u8, 157u8,
                    52u8, 2u8, 162u8, 157u8, 143u8, 155u8, 40u8, 160u8, 226u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    operatorDetails: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <IDelegationManager::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.operatorDetails,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesDecreased(address,address,address,uint256)` and selector `0x6909600037b75d7b4733aedd815442b5ec018a827751c832aaff64eba5d6d2dd`.
    ```solidity
    event OperatorSharesDecreased(address indexed operator, address staker, address strategy, uint256 shares);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesDecreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSharesDecreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorSharesDecreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    105u8, 9u8, 96u8, 0u8, 55u8, 183u8, 93u8, 123u8, 71u8, 51u8, 174u8, 221u8,
                    129u8, 84u8, 66u8, 181u8, 236u8, 1u8, 138u8, 130u8, 119u8, 81u8, 200u8, 50u8,
                    170u8, 255u8, 100u8, 235u8, 165u8, 214u8, 210u8, 221u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesDecreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesDecreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSharesDecreased) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSharesIncreased(address,address,address,uint256)` and selector `0x1ec042c965e2edd7107b51188ee0f383e22e76179041ab3a9d18ff151405166c`.
    ```solidity
    event OperatorSharesIncreased(address indexed operator, address staker, address strategy, uint256 shares);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSharesIncreased {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSharesIncreased {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorSharesIncreased(address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    30u8, 192u8, 66u8, 201u8, 101u8, 226u8, 237u8, 215u8, 16u8, 123u8, 81u8, 24u8,
                    142u8, 224u8, 243u8, 131u8, 226u8, 46u8, 118u8, 23u8, 144u8, 65u8, 171u8, 58u8,
                    157u8, 24u8, 255u8, 21u8, 20u8, 5u8, 22u8, 108u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    staker: data.0,
                    strategy: data.1,
                    shares: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSharesIncreased {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSharesIncreased> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSharesIncreased) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
    ```solidity
    event Paused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                    152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                    224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PauserRegistrySet(address,address)` and selector `0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6`.
    ```solidity
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauserRegistrySet {
        #[allow(missing_docs)]
        pub pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PauserRegistrySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    110u8, 159u8, 205u8, 83u8, 152u8, 150u8, 252u8, 166u8, 14u8, 139u8, 15u8, 1u8,
                    221u8, 88u8, 2u8, 51u8, 228u8, 138u8, 107u8, 15u8, 125u8, 240u8, 19u8, 184u8,
                    155u8, 167u8, 245u8, 101u8, 134u8, 154u8, 205u8, 182u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pauserRegistry: data.0,
                    newPauserRegistry: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PauserRegistrySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauserRegistrySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PauserRegistrySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerDelegated(address,address)` and selector `0xc3ee9f2e5fda98e8066a1f745b2df9285f416fe98cf2559cd21484b3d8743304`.
    ```solidity
    event StakerDelegated(address indexed staker, address indexed operator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerDelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerDelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerDelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    195u8, 238u8, 159u8, 46u8, 95u8, 218u8, 152u8, 232u8, 6u8, 106u8, 31u8, 116u8,
                    91u8, 45u8, 249u8, 40u8, 95u8, 65u8, 111u8, 233u8, 140u8, 242u8, 85u8, 156u8,
                    210u8, 20u8, 132u8, 179u8, 216u8, 116u8, 51u8, 4u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.staker.clone(),
                    self.operator.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerDelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerDelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerDelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerForceUndelegated(address,address)` and selector `0xf0eddf07e6ea14f388b47e1e94a0f464ecbd9eed4171130e0fc0e99fb4030a8a`.
    ```solidity
    event StakerForceUndelegated(address indexed staker, address indexed operator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerForceUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerForceUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerForceUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    240u8, 237u8, 223u8, 7u8, 230u8, 234u8, 20u8, 243u8, 136u8, 180u8, 126u8, 30u8,
                    148u8, 160u8, 244u8, 100u8, 236u8, 189u8, 158u8, 237u8, 65u8, 113u8, 19u8,
                    14u8, 15u8, 192u8, 233u8, 159u8, 180u8, 3u8, 10u8, 138u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.staker.clone(),
                    self.operator.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerForceUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerForceUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerForceUndelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StakerUndelegated(address,address)` and selector `0xfee30966a256b71e14bc0ebfc94315e28ef4a97a7131a9e2b7a310a73af44676`.
    ```solidity
    event StakerUndelegated(address indexed staker, address indexed operator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakerUndelegated {
        #[allow(missing_docs)]
        pub staker: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StakerUndelegated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "StakerUndelegated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    254u8, 227u8, 9u8, 102u8, 162u8, 86u8, 183u8, 30u8, 20u8, 188u8, 14u8, 191u8,
                    201u8, 67u8, 21u8, 226u8, 142u8, 244u8, 169u8, 122u8, 113u8, 49u8, 169u8,
                    226u8, 183u8, 163u8, 16u8, 167u8, 58u8, 244u8, 70u8, 118u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    staker: topics.1,
                    operator: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.staker.clone(),
                    self.operator.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.staker,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StakerUndelegated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakerUndelegated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakerUndelegated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyWithdrawalDelayBlocksSet(address,uint256,uint256)` and selector `0x0e7efa738e8b0ce6376a0c1af471655540d2e9a81647d7b09ed823018426576d`.
    ```solidity
    event StrategyWithdrawalDelayBlocksSet(address strategy, uint256 previousValue, uint256 newValue);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyWithdrawalDelayBlocksSet {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for StrategyWithdrawalDelayBlocksSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "StrategyWithdrawalDelayBlocksSet(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    14u8, 126u8, 250u8, 115u8, 142u8, 139u8, 12u8, 230u8, 55u8, 106u8, 12u8, 26u8,
                    244u8, 113u8, 101u8, 85u8, 64u8, 210u8, 233u8, 168u8, 22u8, 71u8, 215u8, 176u8,
                    158u8, 216u8, 35u8, 1u8, 132u8, 38u8, 87u8, 109u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    strategy: data.0,
                    previousValue: data.1,
                    newValue: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.previousValue,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newValue,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyWithdrawalDelayBlocksSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyWithdrawalDelayBlocksSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyWithdrawalDelayBlocksSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
    ```solidity
    event Unpaused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                    188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                    107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalCompleted(bytes32)` and selector `0xc97098c2f658800b4df29001527f7324bcdffcf6e8751a699ab920a1eced5b1d`.
    ```solidity
    event WithdrawalCompleted(bytes32 withdrawalRoot);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalCompleted {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WithdrawalCompleted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WithdrawalCompleted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    201u8, 112u8, 152u8, 194u8, 246u8, 88u8, 128u8, 11u8, 77u8, 242u8, 144u8, 1u8,
                    82u8, 127u8, 115u8, 36u8, 188u8, 223u8, 252u8, 246u8, 232u8, 117u8, 26u8,
                    105u8, 154u8, 185u8, 32u8, 161u8, 236u8, 237u8, 91u8, 29u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalRoot: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x9009ab153e8014fbfb02f2217f5cde7aa7f9ad734ae85ca3ee3f4ca2fdd499f9`.
    ```solidity
    event WithdrawalQueued(bytes32 withdrawalRoot, IDelegationManager.Withdrawal withdrawal);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalQueued {
        #[allow(missing_docs)]
        pub withdrawalRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub withdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WithdrawalQueued {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IDelegationManager::Withdrawal,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WithdrawalQueued(bytes32,(address,address,address,uint256,uint32,address[],uint256[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    144u8, 9u8, 171u8, 21u8, 62u8, 128u8, 20u8, 251u8, 251u8, 2u8, 242u8, 33u8,
                    127u8, 92u8, 222u8, 122u8, 167u8, 249u8, 173u8, 115u8, 74u8, 232u8, 92u8,
                    163u8, 238u8, 63u8, 76u8, 162u8, 253u8, 212u8, 153u8, 249u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    withdrawalRoot: data.0,
                    withdrawal: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalRoot),
                    <IDelegationManager::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalQueued {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalQueued> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalQueued) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _strategyManager, address _slasher, address _eigenPodManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _slasher: alloy::sol_types::private::Address,
        pub _eigenPodManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._strategyManager,
                        value._slasher,
                        value._eigenPodManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _strategyManager: tuple.0,
                        _slasher: tuple.1,
                        _eigenPodManager: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `DELEGATION_APPROVAL_TYPEHASH()` and selector `0x04a4f979`.
    ```solidity
    function DELEGATION_APPROVAL_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHCall {}
    ///Container type for the return parameters of the [`DELEGATION_APPROVAL_TYPEHASH()`](DELEGATION_APPROVAL_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DELEGATION_APPROVAL_TYPEHASHReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DELEGATION_APPROVAL_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DELEGATION_APPROVAL_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DELEGATION_APPROVAL_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DELEGATION_APPROVAL_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DELEGATION_APPROVAL_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DELEGATION_APPROVAL_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DELEGATION_APPROVAL_TYPEHASH()";
            const SELECTOR: [u8; 4] = [4u8, 164u8, 249u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `DOMAIN_TYPEHASH()` and selector `0x20606b70`.
    ```solidity
    function DOMAIN_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_TYPEHASHCall {}
    ///Container type for the return parameters of the [`DOMAIN_TYPEHASH()`](DOMAIN_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DOMAIN_TYPEHASHReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DOMAIN_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DOMAIN_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DOMAIN_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DOMAIN_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DOMAIN_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DOMAIN_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DOMAIN_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DOMAIN_TYPEHASH()";
            const SELECTOR: [u8; 4] = [32u8, 96u8, 107u8, 112u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()` and selector `0x4fc40b61`.
    ```solidity
    function MAX_STAKER_OPT_OUT_WINDOW_BLOCKS() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall {}
    ///Container type for the return parameters of the [`MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()`](MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_STAKER_OPT_OUT_WINDOW_BLOCKSReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_STAKER_OPT_OUT_WINDOW_BLOCKSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_STAKER_OPT_OUT_WINDOW_BLOCKSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_STAKER_OPT_OUT_WINDOW_BLOCKSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_STAKER_OPT_OUT_WINDOW_BLOCKSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_STAKER_OPT_OUT_WINDOW_BLOCKS()";
            const SELECTOR: [u8; 4] = [79u8, 196u8, 11u8, 97u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`.
    ```solidity
    function MAX_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WITHDRAWAL_DELAY_BLOCKSCall {}
    ///Container type for the return parameters of the [`MAX_WITHDRAWAL_DELAY_BLOCKS()`](MAX_WITHDRAWAL_DELAY_BLOCKSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WITHDRAWAL_DELAY_BLOCKSReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WITHDRAWAL_DELAY_BLOCKSCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WITHDRAWAL_DELAY_BLOCKSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WITHDRAWAL_DELAY_BLOCKSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WITHDRAWAL_DELAY_BLOCKSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WITHDRAWAL_DELAY_BLOCKSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WITHDRAWAL_DELAY_BLOCKSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WITHDRAWAL_DELAY_BLOCKSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_WITHDRAWAL_DELAY_BLOCKSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_WITHDRAWAL_DELAY_BLOCKS()";
            const SELECTOR: [u8; 4] = [202u8, 102u8, 28u8, 4u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `STAKER_DELEGATION_TYPEHASH()` and selector `0x43377382`.
    ```solidity
    function STAKER_DELEGATION_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKER_DELEGATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`STAKER_DELEGATION_TYPEHASH()`](STAKER_DELEGATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKER_DELEGATION_TYPEHASHReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKER_DELEGATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: STAKER_DELEGATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for STAKER_DELEGATION_TYPEHASHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKER_DELEGATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: STAKER_DELEGATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for STAKER_DELEGATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for STAKER_DELEGATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = STAKER_DELEGATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "STAKER_DELEGATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [67u8, 55u8, 115u8, 130u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `beaconChainETHStrategy()` and selector `0x9104c319`.
    ```solidity
    function beaconChainETHStrategy() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyCall {}
    ///Container type for the return parameters of the [`beaconChainETHStrategy()`](beaconChainETHStrategyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct beaconChainETHStrategyReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<beaconChainETHStrategyCall> for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beaconChainETHStrategyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<beaconChainETHStrategyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: beaconChainETHStrategyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for beaconChainETHStrategyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beaconChainETHStrategyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = beaconChainETHStrategyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beaconChainETHStrategy()";
            const SELECTOR: [u8; 4] = [145u8, 4u8, 195u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateCurrentStakerDelegationDigestHash(address,address,uint256)` and selector `0x1bbce091`.
    ```solidity
    function calculateCurrentStakerDelegationDigestHash(address staker, address operator, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCurrentStakerDelegationDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateCurrentStakerDelegationDigestHash(address,address,uint256)`](calculateCurrentStakerDelegationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCurrentStakerDelegationDigestHashReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateCurrentStakerDelegationDigestHashCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateCurrentStakerDelegationDigestHashCall) -> Self {
                    (value.staker, value.operator, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateCurrentStakerDelegationDigestHashCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        expiry: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateCurrentStakerDelegationDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateCurrentStakerDelegationDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateCurrentStakerDelegationDigestHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateCurrentStakerDelegationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateCurrentStakerDelegationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "calculateCurrentStakerDelegationDigestHash(address,address,uint256)";
            const SELECTOR: [u8; 4] = [27u8, 188u8, 224u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.expiry,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)` and selector `0x0b9f487a`.
    ```solidity
    function calculateDelegationApprovalDigestHash(address staker, address operator, address _delegationApprover, bytes32 approverSalt, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub _delegationApprover: alloy::sol_types::private::Address,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)`](calculateDelegationApprovalDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDelegationApprovalDigestHashReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateDelegationApprovalDigestHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateDelegationApprovalDigestHashCall) -> Self {
                    (
                        value.staker,
                        value.operator,
                        value._delegationApprover,
                        value.approverSalt,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateDelegationApprovalDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        _delegationApprover: tuple.2,
                        approverSalt: tuple.3,
                        expiry: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateDelegationApprovalDigestHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: calculateDelegationApprovalDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for calculateDelegationApprovalDigestHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateDelegationApprovalDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDelegationApprovalDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "calculateDelegationApprovalDigestHash(address,address,address,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 159u8, 72u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationApprover,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateStakerDelegationDigestHash(address,uint256,address,uint256)` and selector `0xc94b5111`.
    ```solidity
    function calculateStakerDelegationDigestHash(address staker, uint256 _stakerNonce, address operator, uint256 expiry) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateStakerDelegationDigestHashCall {
        pub staker: alloy::sol_types::private::Address,
        pub _stakerNonce: alloy::sol_types::private::primitives::aliases::U256,
        pub operator: alloy::sol_types::private::Address,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateStakerDelegationDigestHash(address,uint256,address,uint256)`](calculateStakerDelegationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateStakerDelegationDigestHashReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateStakerDelegationDigestHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateStakerDelegationDigestHashCall) -> Self {
                    (
                        value.staker,
                        value._stakerNonce,
                        value.operator,
                        value.expiry,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateStakerDelegationDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        _stakerNonce: tuple.1,
                        operator: tuple.2,
                        expiry: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateStakerDelegationDigestHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateStakerDelegationDigestHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateStakerDelegationDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateStakerDelegationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateStakerDelegationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "calculateStakerDelegationDigestHash(address,uint256,address,uint256)";
            const SELECTOR: [u8; 4] = [201u8, 75u8, 81u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._stakerNonce,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.expiry,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))` and selector `0x597b36da`.
    ```solidity
    function calculateWithdrawalRoot(IDelegationManager.Withdrawal memory withdrawal) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootCall {
        pub withdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))`](calculateWithdrawalRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateWithdrawalRootReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelegationManager::Withdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateWithdrawalRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootCall) -> Self {
                    (value.withdrawal,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateWithdrawalRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateWithdrawalRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateWithdrawalRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateWithdrawalRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateWithdrawalRootCall {
            type Parameters<'a> = (IDelegationManager::Withdrawal,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateWithdrawalRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateWithdrawalRoot((address,address,address,uint256,uint32,address[],uint256[]))";
            const SELECTOR: [u8; 4] = [89u8, 123u8, 54u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManager::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)` and selector `0x60d7faed`.
    ```solidity
    function completeQueuedWithdrawal(IDelegationManager.Withdrawal memory withdrawal, address[] memory tokens, uint256 middlewareTimesIndex, bool receiveAsTokens) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalCall {
        pub withdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
        pub tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub middlewareTimesIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub receiveAsTokens: bool,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)`](completeQueuedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IDelegationManager::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalCall) -> Self {
                    (
                        value.withdrawal,
                        value.tokens,
                        value.middlewareTimesIndex,
                        value.receiveAsTokens,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawal: tuple.0,
                        tokens: tuple.1,
                        middlewareTimesIndex: tuple.2,
                        receiveAsTokens: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalCall {
            type Parameters<'a> = (
                IDelegationManager::Withdrawal,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawal((address,address,address,uint256,uint32,address[],uint256[]),address[],uint256,bool)";
            const SELECTOR: [u8; 4] = [96u8, 215u8, 250u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManager::Withdrawal as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawal,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.middlewareTimesIndex),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.receiveAsTokens,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])` and selector `0x33404396`.
    ```solidity
    function completeQueuedWithdrawals(IDelegationManager.Withdrawal[] memory withdrawals, address[][] memory tokens, uint256[] memory middlewareTimesIndexes, bool[] memory receiveAsTokens) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsCall {
        pub withdrawals: alloy::sol_types::private::Vec<
            <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
        >,
        pub tokens: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub middlewareTimesIndexes:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        pub receiveAsTokens: alloy::sol_types::private::Vec<bool>,
    }
    ///Container type for the return parameters of the [`completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])`](completeQueuedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct completeQueuedWithdrawalsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManager::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<bool>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsCall) -> Self {
                    (
                        value.withdrawals,
                        value.tokens,
                        value.middlewareTimesIndexes,
                        value.receiveAsTokens,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawals: tuple.0,
                        tokens: tuple.1,
                        middlewareTimesIndexes: tuple.2,
                        receiveAsTokens: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<completeQueuedWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: completeQueuedWithdrawalsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for completeQueuedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for completeQueuedWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IDelegationManager::Withdrawal>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                >,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bool>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = completeQueuedWithdrawalsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "completeQueuedWithdrawals((address,address,address,uint256,uint32,address[],uint256[])[],address[][],uint256[],bool[])";
            const SELECTOR: [u8; 4] = [51u8, 64u8, 67u8, 150u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IDelegationManager::Withdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawals),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokens),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.middlewareTimesIndexes,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bool,
                    > as alloy_sol_types::SolType>::tokenize(&self.receiveAsTokens),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `cumulativeWithdrawalsQueued(address)` and selector `0xa1788484`.
    ```solidity
    function cumulativeWithdrawalsQueued(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeWithdrawalsQueued(address)`](cumulativeWithdrawalsQueuedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeWithdrawalsQueuedReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cumulativeWithdrawalsQueuedCall> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeWithdrawalsQueuedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<cumulativeWithdrawalsQueuedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeWithdrawalsQueuedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeWithdrawalsQueuedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeWithdrawalsQueuedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeWithdrawalsQueuedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeWithdrawalsQueued(address)";
            const SELECTOR: [u8; 4] = [161u8, 120u8, 132u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `decreaseDelegatedShares(address,address,uint256)` and selector `0x132d4967`.
    ```solidity
    function decreaseDelegatedShares(address staker, address strategy, uint256 shares) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`decreaseDelegatedShares(address,address,uint256)`](decreaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct decreaseDelegatedSharesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decreaseDelegatedSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesCall) -> Self {
                    (value.staker, value.strategy, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decreaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
                        shares: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<decreaseDelegatedSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: decreaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for decreaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for decreaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = decreaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "decreaseDelegatedShares(address,address,uint256)";
            const SELECTOR: [u8; 4] = [19u8, 45u8, 73u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegateTo(address,(bytes,uint256),bytes32)` and selector `0xeea9064b`.
    ```solidity
    function delegateTo(address operator, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToCall {
        pub operator: alloy::sol_types::private::Address,
        pub approverSignatureAndExpiry:
            <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateTo(address,(bytes,uint256),bytes32)`](delegateToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToCall) -> Self {
                    (
                        value.operator,
                        value.approverSignatureAndExpiry,
                        value.approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        approverSignatureAndExpiry: tuple.1,
                        approverSalt: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegateTo(address,(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [238u8, 169u8, 6u8, 75u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)` and selector `0x7f548071`.
    ```solidity
    function delegateToBySignature(address staker, address operator, ISignatureUtils.SignatureWithExpiry memory stakerSignatureAndExpiry, ISignatureUtils.SignatureWithExpiry memory approverSignatureAndExpiry, bytes32 approverSalt) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToBySignatureCall {
        pub staker: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
        pub stakerSignatureAndExpiry:
            <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSignatureAndExpiry:
            <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
        pub approverSalt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)`](delegateToBySignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateToBySignatureReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
                <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateToBySignatureCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToBySignatureCall) -> Self {
                    (
                        value.staker,
                        value.operator,
                        value.stakerSignatureAndExpiry,
                        value.approverSignatureAndExpiry,
                        value.approverSalt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToBySignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        operator: tuple.1,
                        stakerSignatureAndExpiry: tuple.2,
                        approverSignatureAndExpiry: tuple.3,
                        approverSalt: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateToBySignatureReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateToBySignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateToBySignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateToBySignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithExpiry,
                ISignatureUtils::SignatureWithExpiry,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateToBySignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "delegateToBySignature(address,address,(bytes,uint256),(bytes,uint256),bytes32)";
            const SELECTOR: [u8; 4] = [127u8, 84u8, 128u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.stakerSignatureAndExpiry,
                    ),
                    <ISignatureUtils::SignatureWithExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.approverSignatureAndExpiry,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.approverSalt),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegatedTo(address)` and selector `0x65da1264`.
    ```solidity
    function delegatedTo(address) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegatedTo(address)`](delegatedToCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedToReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatedToCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatedToReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatedToReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatedToReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedToCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedToReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegatedTo(address)";
            const SELECTOR: [u8; 4] = [101u8, 218u8, 18u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegationApprover(address)` and selector `0x3cdeb5e0`.
    ```solidity
    function delegationApprover(address operator) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegationApprover(address)`](delegationApproverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApprover(address)";
            const SELECTOR: [u8; 4] = [60u8, 222u8, 181u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegationApproverSaltIsSpent(address,bytes32)` and selector `0xbb45fef2`.
    ```solidity
    function delegationApproverSaltIsSpent(address, bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegationApproverSaltIsSpent(address,bytes32)`](delegationApproverSaltIsSpentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationApproverSaltIsSpentReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverSaltIsSpentCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverSaltIsSpentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegationApproverSaltIsSpentReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationApproverSaltIsSpentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationApproverSaltIsSpentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationApproverSaltIsSpentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationApproverSaltIsSpentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationApproverSaltIsSpent(address,bytes32)";
            const SELECTOR: [u8; 4] = [187u8, 69u8, 254u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
    ```solidity
    function domainSeparator() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall {}
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<domainSeparatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
    ```solidity
    function eigenPodManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall {}
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getDelegatableShares(address)` and selector `0xcf80873e`.
    ```solidity
    function getDelegatableShares(address staker) external view returns (address[] memory, uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDelegatableSharesCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getDelegatableShares(address)`](getDelegatableSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDelegatableSharesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _1:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDelegatableSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDelegatableSharesCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDelegatableSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDelegatableSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDelegatableSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDelegatableSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDelegatableSharesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDelegatableSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDelegatableShares(address)";
            const SELECTOR: [u8; 4] = [207u8, 128u8, 135u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorShares(address,address[])` and selector `0x90041347`.
    ```solidity
    function getOperatorShares(address operator, address[] memory strategies) external view returns (uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getOperatorShares(address,address[])`](getOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSharesReturn {
        pub _0:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSharesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorShares(address,address[])";
            const SELECTOR: [u8; 4] = [144u8, 4u8, 19u8, 71u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getWithdrawalDelay(address[])` and selector `0x0449ca39`.
    ```solidity
    function getWithdrawalDelay(address[] memory strategies) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalDelayCall {
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getWithdrawalDelay(address[])`](getWithdrawalDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalDelayReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getWithdrawalDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalDelayCall) -> Self {
                    (value.strategies,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawalDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getWithdrawalDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawalDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalDelayCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalDelayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalDelay(address[])";
            const SELECTOR: [u8; 4] = [4u8, 73u8, 202u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.strategies
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `increaseDelegatedShares(address,address,uint256)` and selector `0x28a573ae`.
    ```solidity
    function increaseDelegatedShares(address staker, address strategy, uint256 shares) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesCall {
        pub staker: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub shares: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`increaseDelegatedShares(address,address,uint256)`](increaseDelegatedSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct increaseDelegatedSharesReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<increaseDelegatedSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesCall) -> Self {
                    (value.staker, value.strategy, value.shares)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for increaseDelegatedSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        staker: tuple.0,
                        strategy: tuple.1,
                        shares: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<increaseDelegatedSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: increaseDelegatedSharesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for increaseDelegatedSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for increaseDelegatedSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = increaseDelegatedSharesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "increaseDelegatedShares(address,address,uint256)";
            const SELECTOR: [u8; 4] = [40u8, 165u8, 115u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.shares,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,address,uint256,uint256,address[],uint256[])` and selector `0x22bf40e4`.
    ```solidity
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, uint256 _minWithdrawalDelayBlocks, address[] memory _strategies, uint256[] memory _withdrawalDelayBlocks) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _minWithdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        pub _strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _withdrawalDelayBlocks:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint256,uint256,address[],uint256[])`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value._pauserRegistry,
                        value.initialPausedStatus,
                        value._minWithdrawalDelayBlocks,
                        value._strategies,
                        value._withdrawalDelayBlocks,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _pauserRegistry: tuple.1,
                        initialPausedStatus: tuple.2,
                        _minWithdrawalDelayBlocks: tuple.3,
                        _strategies: tuple.4,
                        _withdrawalDelayBlocks: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initialize(address,address,uint256,uint256,address[],uint256[])";
            const SELECTOR: [u8; 4] = [34u8, 191u8, 64u8, 228u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialPausedStatus),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._minWithdrawalDelayBlocks,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._withdrawalDelayBlocks,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isDelegated(address)` and selector `0x3e28391d`.
    ```solidity
    function isDelegated(address staker) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isDelegated(address)`](isDelegatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isDelegatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isDelegatedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isDelegatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDelegatedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isDelegatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDelegated(address)";
            const SELECTOR: [u8; 4] = [62u8, 40u8, 57u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isOperator(address)` and selector `0x6d70f7ae`.
    ```solidity
    function isOperator(address operator) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperator(address)`](isOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperator(address)";
            const SELECTOR: [u8; 4] = [109u8, 112u8, 247u8, 174u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `minWithdrawalDelayBlocks()` and selector `0xc448feb8`.
    ```solidity
    function minWithdrawalDelayBlocks() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksCall {}
    ///Container type for the return parameters of the [`minWithdrawalDelayBlocks()`](minWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minWithdrawalDelayBlocksReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minWithdrawalDelayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minWithdrawalDelayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minWithdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minWithdrawalDelayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minWithdrawalDelayBlocks()";
            const SELECTOR: [u8; 4] = [196u8, 72u8, 254u8, 184u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `modifyOperatorDetails((address,address,uint32))` and selector `0xf16172b0`.
    ```solidity
    function modifyOperatorDetails(IDelegationManager.OperatorDetails memory newOperatorDetails) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsCall {
        pub newOperatorDetails:
            <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`modifyOperatorDetails((address,address,uint32))`](modifyOperatorDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyOperatorDetailsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelegationManager::OperatorDetails,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyOperatorDetailsCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsCall) -> Self {
                    (value.newOperatorDetails,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyOperatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newOperatorDetails: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyOperatorDetailsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: modifyOperatorDetailsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyOperatorDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyOperatorDetailsCall {
            type Parameters<'a> = (IDelegationManager::OperatorDetails,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyOperatorDetailsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyOperatorDetails((address,address,uint32))";
            const SELECTOR: [u8; 4] = [241u8, 97u8, 114u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManager::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorDetails,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorDetails(address)` and selector `0xc5e480db`.
    ```solidity
    function operatorDetails(address operator) external view returns (IDelegationManager.OperatorDetails memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorDetailsCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorDetails(address)`](operatorDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorDetailsReturn {
        pub _0: <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorDetailsCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorDetailsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelegationManager::OperatorDetails,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorDetailsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorDetailsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorDetailsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorDetailsReturn;
            type ReturnTuple<'a> = (IDelegationManager::OperatorDetails,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorDetails(address)";
            const SELECTOR: [u8; 4] = [197u8, 228u8, 128u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `operatorShares(address,address)` and selector `0x778e55f3`.
    ```solidity
    function operatorShares(address, address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorShares(address,address)`](operatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSharesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorSharesCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorSharesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorSharesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSharesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSharesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorShares(address,address)";
            const SELECTOR: [u8; 4] = [119u8, 142u8, 85u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
    ```solidity
    function pause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall {}
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
    ```solidity
    function paused(uint8 index) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        pub index: u8,
    }
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call {}
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = paused_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall {}
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauserRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pendingWithdrawals(bytes32)` and selector `0xb7f06ebe`.
    ```solidity
    function pendingWithdrawals(bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`pendingWithdrawals(bytes32)`](pendingWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingWithdrawalsReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pendingWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pendingWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pendingWithdrawalsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingWithdrawals(bytes32)";
            const SELECTOR: [u8; 4] = [183u8, 240u8, 110u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `queueWithdrawals((address[],uint256[],address)[])` and selector `0x0dd8dd02`.
    ```solidity
    function queueWithdrawals(IDelegationManager.QueuedWithdrawalParams[] memory queuedWithdrawalParams) external returns (bytes32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsCall {
        pub queuedWithdrawalParams: alloy::sol_types::private::Vec<
            <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`queueWithdrawals((address[],uint256[],address)[])`](queueWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct queueWithdrawalsReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManager::QueuedWithdrawalParams>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsCall) -> Self {
                    (value.queuedWithdrawalParams,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        queuedWithdrawalParams: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<queueWithdrawalsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: queueWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for queueWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for queueWithdrawalsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IDelegationManager::QueuedWithdrawalParams>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = queueWithdrawalsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "queueWithdrawals((address[],uint256[],address)[])";
            const SELECTOR: [u8; 4] = [13u8, 216u8, 221u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IDelegationManager::QueuedWithdrawalParams,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.queuedWithdrawalParams,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerAsOperator((address,address,uint32),string)` and selector `0x0f589e59`.
    ```solidity
    function registerAsOperator(IDelegationManager.OperatorDetails memory registeringOperatorDetails, string memory metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorCall {
        pub registeringOperatorDetails:
            <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`registerAsOperator((address,address,uint32),string)`](registerAsOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerAsOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IDelegationManager::OperatorDetails,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAsOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorCall) -> Self {
                    (value.registeringOperatorDetails, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registeringOperatorDetails: tuple.0,
                        metadataURI: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerAsOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerAsOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerAsOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerAsOperatorCall {
            type Parameters<'a> = (
                IDelegationManager::OperatorDetails,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerAsOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerAsOperator((address,address,uint32),string)";
            const SELECTOR: [u8; 4] = [15u8, 88u8, 158u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IDelegationManager::OperatorDetails as alloy_sol_types::SolType>::tokenize(
                        &self.registeringOperatorDetails,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setMinWithdrawalDelayBlocks(uint256)` and selector `0x635bbd10`.
    ```solidity
    function setMinWithdrawalDelayBlocks(uint256 newMinWithdrawalDelayBlocks) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinWithdrawalDelayBlocksCall {
        pub newMinWithdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setMinWithdrawalDelayBlocks(uint256)`](setMinWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinWithdrawalDelayBlocksReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinWithdrawalDelayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMinWithdrawalDelayBlocksCall) -> Self {
                    (value.newMinWithdrawalDelayBlocks,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newMinWithdrawalDelayBlocks: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinWithdrawalDelayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMinWithdrawalDelayBlocksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinWithdrawalDelayBlocksCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinWithdrawalDelayBlocks(uint256)";
            const SELECTOR: [u8; 4] = [99u8, 91u8, 189u8, 16u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newMinWithdrawalDelayBlocks,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
    ```solidity
    function setPauserRegistry(address newPauserRegistry) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPauserRegistry(address)`](setPauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPauserRegistry: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPauserRegistry(address)";
            const SELECTOR: [u8; 4] = [16u8, 214u8, 122u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setStrategyWithdrawalDelayBlocks(address[],uint256[])` and selector `0x1522bf02`.
    ```solidity
    function setStrategyWithdrawalDelayBlocks(address[] memory strategies, uint256[] memory withdrawalDelayBlocks) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStrategyWithdrawalDelayBlocksCall {
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub withdrawalDelayBlocks:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`setStrategyWithdrawalDelayBlocks(address[],uint256[])`](setStrategyWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStrategyWithdrawalDelayBlocksReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStrategyWithdrawalDelayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStrategyWithdrawalDelayBlocksCall) -> Self {
                    (value.strategies, value.withdrawalDelayBlocks)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStrategyWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategies: tuple.0,
                        withdrawalDelayBlocks: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setStrategyWithdrawalDelayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStrategyWithdrawalDelayBlocksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStrategyWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStrategyWithdrawalDelayBlocksCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStrategyWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStrategyWithdrawalDelayBlocks(address[],uint256[])";
            const SELECTOR: [u8; 4] = [21u8, 34u8, 191u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalDelayBlocks),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `slasher()` and selector `0xb1344271`.
    ```solidity
    function slasher() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherCall {}
    ///Container type for the return parameters of the [`slasher()`](slasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slasherReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: slasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slasher()";
            const SELECTOR: [u8; 4] = [177u8, 52u8, 66u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stakerNonce(address)` and selector `0x29c77d4f`.
    ```solidity
    function stakerNonce(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerNonceCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`stakerNonce(address)`](stakerNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerNonceReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakerNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakerNonceCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakerNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakerNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakerNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakerNonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakerNonce(address)";
            const SELECTOR: [u8; 4] = [41u8, 199u8, 125u8, 79u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `stakerOptOutWindowBlocks(address)` and selector `0x16928365`.
    ```solidity
    function stakerOptOutWindowBlocks(address operator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerOptOutWindowBlocksCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`stakerOptOutWindowBlocks(address)`](stakerOptOutWindowBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakerOptOutWindowBlocksReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakerOptOutWindowBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakerOptOutWindowBlocksCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerOptOutWindowBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakerOptOutWindowBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakerOptOutWindowBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakerOptOutWindowBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakerOptOutWindowBlocksCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakerOptOutWindowBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakerOptOutWindowBlocks(address)";
            const SELECTOR: [u8; 4] = [22u8, 146u8, 131u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `strategyManager()` and selector `0x39b70e38`.
    ```solidity
    function strategyManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerCall {}
    ///Container type for the return parameters of the [`strategyManager()`](strategyManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyManagerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyManager()";
            const SELECTOR: [u8; 4] = [57u8, 183u8, 14u8, 56u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `strategyWithdrawalDelayBlocks(address)` and selector `0xc488375a`.
    ```solidity
    function strategyWithdrawalDelayBlocks(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyWithdrawalDelayBlocksCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`strategyWithdrawalDelayBlocks(address)`](strategyWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyWithdrawalDelayBlocksReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyWithdrawalDelayBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyWithdrawalDelayBlocksCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyWithdrawalDelayBlocksReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyWithdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyWithdrawalDelayBlocksCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyWithdrawalDelayBlocks(address)";
            const SELECTOR: [u8; 4] = [196u8, 136u8, 55u8, 90u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `undelegate(address)` and selector `0xda8be864`.
    ```solidity
    function undelegate(address staker) external returns (bytes32[] memory withdrawalRoots);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateCall {
        pub staker: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`undelegate(address)`](undelegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct undelegateReturn {
        pub withdrawalRoots:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<undelegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateCall) -> Self {
                    (value.staker,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { staker: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<undelegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: undelegateReturn) -> Self {
                    (value.withdrawalRoots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for undelegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        withdrawalRoots: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for undelegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = undelegateReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "undelegate(address)";
            const SELECTOR: [u8; 4] = [218u8, 139u8, 232u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.staker,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
    ```solidity
    function unpause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `updateOperatorMetadataURI(string)` and selector `0x99be81c8`.
    ```solidity
    function updateOperatorMetadataURI(string memory metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURICall {
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateOperatorMetadataURI(string)`](updateOperatorMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorMetadataURIReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURICall) -> Self {
                    (value.metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        metadataURI: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorMetadataURI(string)";
            const SELECTOR: [u8; 4] = [153u8, 190u8, 129u8, 200u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`DelegationManager`](self) function calls.
    pub enum DelegationManagerCalls {
        DELEGATION_APPROVAL_TYPEHASH(DELEGATION_APPROVAL_TYPEHASHCall),
        DOMAIN_TYPEHASH(DOMAIN_TYPEHASHCall),
        MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall),
        MAX_WITHDRAWAL_DELAY_BLOCKS(MAX_WITHDRAWAL_DELAY_BLOCKSCall),
        STAKER_DELEGATION_TYPEHASH(STAKER_DELEGATION_TYPEHASHCall),
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        calculateCurrentStakerDelegationDigestHash(calculateCurrentStakerDelegationDigestHashCall),
        calculateDelegationApprovalDigestHash(calculateDelegationApprovalDigestHashCall),
        calculateStakerDelegationDigestHash(calculateStakerDelegationDigestHashCall),
        calculateWithdrawalRoot(calculateWithdrawalRootCall),
        completeQueuedWithdrawal(completeQueuedWithdrawalCall),
        completeQueuedWithdrawals(completeQueuedWithdrawalsCall),
        cumulativeWithdrawalsQueued(cumulativeWithdrawalsQueuedCall),
        decreaseDelegatedShares(decreaseDelegatedSharesCall),
        delegateTo(delegateToCall),
        delegateToBySignature(delegateToBySignatureCall),
        delegatedTo(delegatedToCall),
        delegationApprover(delegationApproverCall),
        delegationApproverSaltIsSpent(delegationApproverSaltIsSpentCall),
        domainSeparator(domainSeparatorCall),
        eigenPodManager(eigenPodManagerCall),
        getDelegatableShares(getDelegatableSharesCall),
        getOperatorShares(getOperatorSharesCall),
        getWithdrawalDelay(getWithdrawalDelayCall),
        increaseDelegatedShares(increaseDelegatedSharesCall),
        initialize(initializeCall),
        isDelegated(isDelegatedCall),
        isOperator(isOperatorCall),
        minWithdrawalDelayBlocks(minWithdrawalDelayBlocksCall),
        modifyOperatorDetails(modifyOperatorDetailsCall),
        operatorDetails(operatorDetailsCall),
        operatorShares(operatorSharesCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        pendingWithdrawals(pendingWithdrawalsCall),
        queueWithdrawals(queueWithdrawalsCall),
        registerAsOperator(registerAsOperatorCall),
        renounceOwnership(renounceOwnershipCall),
        setMinWithdrawalDelayBlocks(setMinWithdrawalDelayBlocksCall),
        setPauserRegistry(setPauserRegistryCall),
        setStrategyWithdrawalDelayBlocks(setStrategyWithdrawalDelayBlocksCall),
        slasher(slasherCall),
        stakerNonce(stakerNonceCall),
        stakerOptOutWindowBlocks(stakerOptOutWindowBlocksCall),
        strategyManager(strategyManagerCall),
        strategyWithdrawalDelayBlocks(strategyWithdrawalDelayBlocksCall),
        transferOwnership(transferOwnershipCall),
        undelegate(undelegateCall),
        unpause(unpauseCall),
        updateOperatorMetadataURI(updateOperatorMetadataURICall),
    }
    #[automatically_derived]
    impl DelegationManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 73u8, 202u8, 57u8],
            [4u8, 164u8, 249u8, 121u8],
            [11u8, 159u8, 72u8, 122u8],
            [13u8, 216u8, 221u8, 2u8],
            [15u8, 88u8, 158u8, 89u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 45u8, 73u8, 103u8],
            [19u8, 100u8, 57u8, 221u8],
            [21u8, 34u8, 191u8, 2u8],
            [22u8, 146u8, 131u8, 101u8],
            [27u8, 188u8, 224u8, 145u8],
            [32u8, 96u8, 107u8, 112u8],
            [34u8, 191u8, 64u8, 228u8],
            [40u8, 165u8, 115u8, 174u8],
            [41u8, 199u8, 125u8, 79u8],
            [51u8, 64u8, 67u8, 150u8],
            [57u8, 183u8, 14u8, 56u8],
            [60u8, 222u8, 181u8, 224u8],
            [62u8, 40u8, 57u8, 29u8],
            [67u8, 55u8, 115u8, 130u8],
            [70u8, 101u8, 188u8, 218u8],
            [79u8, 196u8, 11u8, 97u8],
            [89u8, 92u8, 106u8, 103u8],
            [89u8, 123u8, 54u8, 218u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [96u8, 215u8, 250u8, 237u8],
            [99u8, 91u8, 189u8, 16u8],
            [101u8, 218u8, 18u8, 100u8],
            [109u8, 112u8, 247u8, 174u8],
            [113u8, 80u8, 24u8, 166u8],
            [119u8, 142u8, 85u8, 243u8],
            [127u8, 84u8, 128u8, 113u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [144u8, 4u8, 19u8, 71u8],
            [145u8, 4u8, 195u8, 25u8],
            [153u8, 190u8, 129u8, 200u8],
            [161u8, 120u8, 132u8, 132u8],
            [177u8, 52u8, 66u8, 113u8],
            [183u8, 240u8, 110u8, 190u8],
            [187u8, 69u8, 254u8, 242u8],
            [196u8, 72u8, 254u8, 184u8],
            [196u8, 136u8, 55u8, 90u8],
            [197u8, 228u8, 128u8, 219u8],
            [201u8, 75u8, 81u8, 17u8],
            [202u8, 102u8, 28u8, 4u8],
            [207u8, 128u8, 135u8, 62u8],
            [218u8, 139u8, 232u8, 100u8],
            [238u8, 169u8, 6u8, 75u8],
            [241u8, 97u8, 114u8, 176u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelegationManagerCalls {
        const NAME: &'static str = "DelegationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 54usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(_) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DOMAIN_TYPEHASH(_) => {
                    <DOMAIN_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(_) => {
                    <MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(_) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::STAKER_DELEGATION_TYPEHASH(_) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beaconChainETHStrategy(_) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateCurrentStakerDelegationDigestHash(_) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDelegationApprovalDigestHash(_) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateStakerDelegationDigestHash(_) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateWithdrawalRoot(_) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawal(_) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::completeQueuedWithdrawals(_) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeWithdrawalsQueued(_) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::decreaseDelegatedShares(_) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateTo(_) => {
                    <delegateToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegateToBySignature(_) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedTo(_) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApprover(_) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationApproverSaltIsSpent(_) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDelegatableShares(_) => {
                    <getDelegatableSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorShares(_) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalDelay(_) => {
                    <getWithdrawalDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::increaseDelegatedShares(_) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDelegated(_) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperator(_) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minWithdrawalDelayBlocks(_) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyOperatorDetails(_) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorDetails(_) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorShares(_) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingWithdrawals(_) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::queueWithdrawals(_) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerAsOperator(_) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinWithdrawalDelayBlocks(_) => {
                    <setMinWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStrategyWithdrawalDelayBlocks(_) => {
                    <setStrategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slasher(_) => <slasherCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::stakerNonce(_) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakerOptOutWindowBlocks(_) => {
                    <stakerOptOutWindowBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyWithdrawalDelayBlocks(_) => {
                    <strategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::undelegate(_) => {
                    <undelegateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateOperatorMetadataURI(_) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<DelegationManagerCalls>] = &[
                {
                    fn getWithdrawalDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getWithdrawalDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::getWithdrawalDelay)
                    }
                    getWithdrawalDelay
                },
                {
                    fn DELEGATION_APPROVAL_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::DELEGATION_APPROVAL_TYPEHASH)
                    }
                    DELEGATION_APPROVAL_TYPEHASH
                },
                {
                    fn calculateDelegationApprovalDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateDelegationApprovalDigestHash,
                            )
                    }
                    calculateDelegationApprovalDigestHash
                },
                {
                    fn queueWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::queueWithdrawals)
                    }
                    queueWithdrawals
                },
                {
                    fn registerAsOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::registerAsOperator)
                    }
                    registerAsOperator
                },
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn decreaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::decreaseDelegatedShares)
                    }
                    decreaseDelegatedShares
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn setStrategyWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <setStrategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::setStrategyWithdrawalDelayBlocks,
                            )
                    }
                    setStrategyWithdrawalDelayBlocks
                },
                {
                    fn stakerOptOutWindowBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <stakerOptOutWindowBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::stakerOptOutWindowBlocks)
                    }
                    stakerOptOutWindowBlocks
                },
                {
                    fn calculateCurrentStakerDelegationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateCurrentStakerDelegationDigestHash,
                            )
                    }
                    calculateCurrentStakerDelegationDigestHash
                },
                {
                    fn DOMAIN_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <DOMAIN_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::DOMAIN_TYPEHASH)
                    }
                    DOMAIN_TYPEHASH
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn increaseDelegatedShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::increaseDelegatedShares)
                    }
                    increaseDelegatedShares
                },
                {
                    fn stakerNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <stakerNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::stakerNonce)
                    }
                    stakerNonce
                },
                {
                    fn completeQueuedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::completeQueuedWithdrawals)
                    }
                    completeQueuedWithdrawals
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn delegationApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::delegationApprover)
                    }
                    delegationApprover
                },
                {
                    fn isDelegated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isDelegatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::isDelegated)
                    }
                    isDelegated
                },
                {
                    fn STAKER_DELEGATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::STAKER_DELEGATION_TYPEHASH)
                    }
                    STAKER_DELEGATION_TYPEHASH
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::MAX_STAKER_OPT_OUT_WINDOW_BLOCKS,
                            )
                    }
                    MAX_STAKER_OPT_OUT_WINDOW_BLOCKS
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn calculateWithdrawalRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::calculateWithdrawalRoot)
                    }
                    calculateWithdrawalRoot
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn completeQueuedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::completeQueuedWithdrawal)
                    }
                    completeQueuedWithdrawal
                },
                {
                    fn setMinWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <setMinWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::setMinWithdrawalDelayBlocks)
                    }
                    setMinWithdrawalDelayBlocks
                },
                {
                    fn delegatedTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegatedToCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::delegatedTo)
                    }
                    delegatedTo
                },
                {
                    fn isOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::isOperator)
                    }
                    isOperator
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn operatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <operatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::operatorShares)
                    }
                    operatorShares
                },
                {
                    fn delegateToBySignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::delegateToBySignature)
                    }
                    delegateToBySignature
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::getOperatorShares)
                    }
                    getOperatorShares
                },
                {
                    fn beaconChainETHStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::beaconChainETHStrategy)
                    }
                    beaconChainETHStrategy
                },
                {
                    fn updateOperatorMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::updateOperatorMetadataURI)
                    }
                    updateOperatorMetadataURI
                },
                {
                    fn cumulativeWithdrawalsQueued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::cumulativeWithdrawalsQueued)
                    }
                    cumulativeWithdrawalsQueued
                },
                {
                    fn slasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <slasherCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::slasher)
                    }
                    slasher
                },
                {
                    fn pendingWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::pendingWithdrawals)
                    }
                    pendingWithdrawals
                },
                {
                    fn delegationApproverSaltIsSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::delegationApproverSaltIsSpent)
                    }
                    delegationApproverSaltIsSpent
                },
                {
                    fn minWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::minWithdrawalDelayBlocks)
                    }
                    minWithdrawalDelayBlocks
                },
                {
                    fn strategyWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <strategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::strategyWithdrawalDelayBlocks)
                    }
                    strategyWithdrawalDelayBlocks
                },
                {
                    fn operatorDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <operatorDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::operatorDetails)
                    }
                    operatorDetails
                },
                {
                    fn calculateStakerDelegationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelegationManagerCalls::calculateStakerDelegationDigestHash,
                            )
                    }
                    calculateStakerDelegationDigestHash
                },
                {
                    fn MAX_WITHDRAWAL_DELAY_BLOCKS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelegationManagerCalls::MAX_WITHDRAWAL_DELAY_BLOCKS)
                    }
                    MAX_WITHDRAWAL_DELAY_BLOCKS
                },
                {
                    fn getDelegatableShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <getDelegatableSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::getDelegatableShares)
                    }
                    getDelegatableShares
                },
                {
                    fn undelegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <undelegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::undelegate)
                    }
                    undelegate
                },
                {
                    fn delegateTo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <delegateToCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::delegateTo)
                    }
                    delegateTo
                },
                {
                    fn modifyOperatorDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::modifyOperatorDetails)
                    }
                    modifyOperatorDetails
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(DelegationManagerCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelegationManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(DelegationManagerCalls::unpause)
                    }
                    unpause
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DOMAIN_TYPEHASH(inner) => {
                    <DOMAIN_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(inner) => {
                    <MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::STAKER_DELEGATION_TYPEHASH(inner) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateCurrentStakerDelegationDigestHash(inner) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateStakerDelegationDigestHash(inner) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateToBySignature(inner) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDelegatableShares(inner) => {
                    <getDelegatableSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalDelay(inner) => {
                    <getWithdrawalDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorDetails(inner) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinWithdrawalDelayBlocks(inner) => {
                    <setMinWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStrategyWithdrawalDelayBlocks(inner) => {
                    <setStrategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::stakerNonce(inner) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakerOptOutWindowBlocks(inner) => {
                    <stakerOptOutWindowBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyWithdrawalDelayBlocks(inner) => {
                    <strategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DELEGATION_APPROVAL_TYPEHASH(inner) => {
                    <DELEGATION_APPROVAL_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DOMAIN_TYPEHASH(inner) => {
                    <DOMAIN_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(inner) => {
                    <MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::STAKER_DELEGATION_TYPEHASH(inner) => {
                    <STAKER_DELEGATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateCurrentStakerDelegationDigestHash(inner) => {
                    <calculateCurrentStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateDelegationApprovalDigestHash(inner) => {
                    <calculateDelegationApprovalDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateStakerDelegationDigestHash(inner) => {
                    <calculateStakerDelegationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateWithdrawalRoot(inner) => {
                    <calculateWithdrawalRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawal(inner) => {
                    <completeQueuedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::completeQueuedWithdrawals(inner) => {
                    <completeQueuedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cumulativeWithdrawalsQueued(inner) => {
                    <cumulativeWithdrawalsQueuedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::decreaseDelegatedShares(inner) => {
                    <decreaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateTo(inner) => {
                    <delegateToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegateToBySignature(inner) => {
                    <delegateToBySignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegatedTo(inner) => {
                    <delegatedToCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApprover(inner) => {
                    <delegationApproverCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationApproverSaltIsSpent(inner) => {
                    <delegationApproverSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDelegatableShares(inner) => {
                    <getDelegatableSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorShares(inner) => {
                    <getOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalDelay(inner) => {
                    <getWithdrawalDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::increaseDelegatedShares(inner) => {
                    <increaseDelegatedSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isDelegated(inner) => {
                    <isDelegatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minWithdrawalDelayBlocks(inner) => {
                    <minWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyOperatorDetails(inner) => {
                    <modifyOperatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorDetails(inner) => {
                    <operatorDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorShares(inner) => {
                    <operatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pendingWithdrawals(inner) => {
                    <pendingWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::queueWithdrawals(inner) => {
                    <queueWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerAsOperator(inner) => {
                    <registerAsOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinWithdrawalDelayBlocks(inner) => {
                    <setMinWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setStrategyWithdrawalDelayBlocks(inner) => {
                    <setStrategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slasher(inner) => {
                    <slasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::stakerNonce(inner) => {
                    <stakerNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakerOptOutWindowBlocks(inner) => {
                    <stakerOptOutWindowBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyWithdrawalDelayBlocks(inner) => {
                    <strategyWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::undelegate(inner) => {
                    <undelegateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateOperatorMetadataURI(inner) => {
                    <updateOperatorMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DelegationManager`](self) events.
    pub enum DelegationManagerEvents {
        Initialized(Initialized),
        MinWithdrawalDelayBlocksSet(MinWithdrawalDelayBlocksSet),
        OperatorDetailsModified(OperatorDetailsModified),
        OperatorMetadataURIUpdated(OperatorMetadataURIUpdated),
        OperatorRegistered(OperatorRegistered),
        OperatorSharesDecreased(OperatorSharesDecreased),
        OperatorSharesIncreased(OperatorSharesIncreased),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        StakerDelegated(StakerDelegated),
        StakerForceUndelegated(StakerForceUndelegated),
        StakerUndelegated(StakerUndelegated),
        StrategyWithdrawalDelayBlocksSet(StrategyWithdrawalDelayBlocksSet),
        Unpaused(Unpaused),
        WithdrawalCompleted(WithdrawalCompleted),
        WithdrawalQueued(WithdrawalQueued),
    }
    #[automatically_derived]
    impl DelegationManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8, 169u8, 25u8, 237u8, 14u8, 42u8, 202u8, 209u8, 221u8, 144u8, 241u8, 126u8,
                242u8, 250u8, 74u8, 229u8, 70u8, 46u8, 225u8, 51u8, 145u8, 112u8, 3u8, 74u8, 133u8,
                49u8, 204u8, 164u8, 182u8, 112u8, 128u8, 144u8,
            ],
            [
                14u8, 126u8, 250u8, 115u8, 142u8, 139u8, 12u8, 230u8, 55u8, 106u8, 12u8, 26u8,
                244u8, 113u8, 101u8, 85u8, 64u8, 210u8, 233u8, 168u8, 22u8, 71u8, 215u8, 176u8,
                158u8, 216u8, 35u8, 1u8, 132u8, 38u8, 87u8, 109u8,
            ],
            [
                30u8, 192u8, 66u8, 201u8, 101u8, 226u8, 237u8, 215u8, 16u8, 123u8, 81u8, 24u8,
                142u8, 224u8, 243u8, 131u8, 226u8, 46u8, 118u8, 23u8, 144u8, 65u8, 171u8, 58u8,
                157u8, 24u8, 255u8, 21u8, 20u8, 5u8, 22u8, 108u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                105u8, 9u8, 96u8, 0u8, 55u8, 183u8, 93u8, 123u8, 71u8, 51u8, 174u8, 221u8, 129u8,
                84u8, 66u8, 181u8, 236u8, 1u8, 138u8, 130u8, 119u8, 81u8, 200u8, 50u8, 170u8,
                255u8, 100u8, 235u8, 165u8, 214u8, 210u8, 221u8,
            ],
            [
                110u8, 159u8, 205u8, 83u8, 152u8, 150u8, 252u8, 166u8, 14u8, 139u8, 15u8, 1u8,
                221u8, 88u8, 2u8, 51u8, 228u8, 138u8, 107u8, 15u8, 125u8, 240u8, 19u8, 184u8,
                155u8, 167u8, 245u8, 101u8, 134u8, 154u8, 205u8, 182u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                142u8, 132u8, 133u8, 88u8, 58u8, 35u8, 16u8, 212u8, 31u8, 124u8, 130u8, 185u8,
                66u8, 125u8, 11u8, 212u8, 155u8, 173u8, 116u8, 187u8, 156u8, 255u8, 157u8, 52u8,
                2u8, 162u8, 157u8, 143u8, 155u8, 40u8, 160u8, 226u8,
            ],
            [
                144u8, 9u8, 171u8, 21u8, 62u8, 128u8, 20u8, 251u8, 251u8, 2u8, 242u8, 33u8, 127u8,
                92u8, 222u8, 122u8, 167u8, 249u8, 173u8, 115u8, 74u8, 232u8, 92u8, 163u8, 238u8,
                63u8, 76u8, 162u8, 253u8, 212u8, 153u8, 249u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                175u8, 160u8, 3u8, 205u8, 118u8, 248u8, 127u8, 249u8, 214u8, 43u8, 53u8, 190u8,
                234u8, 136u8, 153u8, 32u8, 243u8, 60u8, 12u8, 66u8, 184u8, 212u8, 91u8, 116u8,
                149u8, 77u8, 97u8, 213u8, 15u8, 75u8, 107u8, 105u8,
            ],
            [
                195u8, 238u8, 159u8, 46u8, 95u8, 218u8, 152u8, 232u8, 6u8, 106u8, 31u8, 116u8,
                91u8, 45u8, 249u8, 40u8, 95u8, 65u8, 111u8, 233u8, 140u8, 242u8, 85u8, 156u8,
                210u8, 20u8, 132u8, 179u8, 216u8, 116u8, 51u8, 4u8,
            ],
            [
                201u8, 112u8, 152u8, 194u8, 246u8, 88u8, 128u8, 11u8, 77u8, 242u8, 144u8, 1u8,
                82u8, 127u8, 115u8, 36u8, 188u8, 223u8, 252u8, 246u8, 232u8, 117u8, 26u8, 105u8,
                154u8, 185u8, 32u8, 161u8, 236u8, 237u8, 91u8, 29u8,
            ],
            [
                240u8, 237u8, 223u8, 7u8, 230u8, 234u8, 20u8, 243u8, 136u8, 180u8, 126u8, 30u8,
                148u8, 160u8, 244u8, 100u8, 236u8, 189u8, 158u8, 237u8, 65u8, 113u8, 19u8, 14u8,
                15u8, 192u8, 233u8, 159u8, 180u8, 3u8, 10u8, 138u8,
            ],
            [
                254u8, 190u8, 92u8, 210u8, 75u8, 44u8, 188u8, 123u8, 6u8, 91u8, 157u8, 15u8, 222u8,
                185u8, 4u8, 70u8, 30u8, 74u8, 252u8, 255u8, 87u8, 221u8, 87u8, 172u8, 218u8, 30u8,
                120u8, 50u8, 3u8, 27u8, 167u8, 172u8,
            ],
            [
                254u8, 227u8, 9u8, 102u8, 162u8, 86u8, 183u8, 30u8, 20u8, 188u8, 14u8, 191u8,
                201u8, 67u8, 21u8, 226u8, 142u8, 244u8, 169u8, 122u8, 113u8, 49u8, 169u8, 226u8,
                183u8, 163u8, 16u8, 167u8, 58u8, 244u8, 70u8, 118u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DelegationManagerEvents {
        const NAME: &'static str = "DelegationManagerEvents";
        const COUNT: usize = 17usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(
                    <MinWithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <MinWithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::MinWithdrawalDelayBlocksSet),
                Some(<OperatorDetailsModified as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorDetailsModified as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorDetailsModified)
                }
                Some(<OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorMetadataURIUpdated)
                }
                Some(<OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorRegistered)
                }
                Some(<OperatorSharesDecreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSharesDecreased as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSharesDecreased)
                }
                Some(<OperatorSharesIncreased as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSharesIncreased as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSharesIncreased)
                }
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::PauserRegistrySet)
                }
                Some(<StakerDelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerDelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerDelegated)
                }
                Some(<StakerForceUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerForceUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerForceUndelegated)
                }
                Some(<StakerUndelegated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakerUndelegated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakerUndelegated)
                }
                Some(
                    <StrategyWithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyWithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyWithdrawalDelayBlocksSet)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Unpaused)
                }
                Some(<WithdrawalCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::WithdrawalCompleted)
                }
                Some(<WithdrawalQueued as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalQueued as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::WithdrawalQueued)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for DelegationManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinWithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDetailsModified(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyWithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::WithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinWithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDetailsModified(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesDecreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSharesIncreased(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerDelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerForceUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakerUndelegated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyWithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalQueued(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

    See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelegationManagerInstance<T, P, N> {
        DelegationManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _strategyManager: alloy::sol_types::private::Address,
        _slasher: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<DelegationManagerInstance<T, P, N>>>
    {
        DelegationManagerInstance::<T, P, N>::deploy(
            provider,
            _strategyManager,
            _slasher,
            _eigenPodManager,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _strategyManager: alloy::sol_types::private::Address,
        _slasher: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelegationManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _strategyManager,
            _slasher,
            _eigenPodManager,
        )
    }
    /**A [`DelegationManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`DelegationManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelegationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelegationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelegationManagerInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > DelegationManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`DelegationManager`](self) contract instance.

        See the [wrapper's documentation](`DelegationManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _strategyManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<DelegationManagerInstance<T, P, N>> {
            let call_builder =
                Self::deploy_builder(provider, _strategyManager, _slasher, _eigenPodManager);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _strategyManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _strategyManager,
                        _slasher,
                        _eigenPodManager,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> DelegationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DelegationManagerInstance<T, P, N> {
            DelegationManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > DelegationManagerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`DELEGATION_APPROVAL_TYPEHASH`] function.
        pub fn DELEGATION_APPROVAL_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DELEGATION_APPROVAL_TYPEHASHCall, N> {
            self.call_builder(&DELEGATION_APPROVAL_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`DOMAIN_TYPEHASH`] function.
        pub fn DOMAIN_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DOMAIN_TYPEHASHCall, N> {
            self.call_builder(&DOMAIN_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`MAX_STAKER_OPT_OUT_WINDOW_BLOCKS`] function.
        pub fn MAX_STAKER_OPT_OUT_WINDOW_BLOCKS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall, N>
        {
            self.call_builder(&MAX_STAKER_OPT_OUT_WINDOW_BLOCKSCall {})
        }
        ///Creates a new call builder for the [`MAX_WITHDRAWAL_DELAY_BLOCKS`] function.
        pub fn MAX_WITHDRAWAL_DELAY_BLOCKS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_WITHDRAWAL_DELAY_BLOCKSCall, N> {
            self.call_builder(&MAX_WITHDRAWAL_DELAY_BLOCKSCall {})
        }
        ///Creates a new call builder for the [`STAKER_DELEGATION_TYPEHASH`] function.
        pub fn STAKER_DELEGATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, STAKER_DELEGATION_TYPEHASHCall, N> {
            self.call_builder(&STAKER_DELEGATION_TYPEHASHCall {})
        }
        ///Creates a new call builder for the [`beaconChainETHStrategy`] function.
        pub fn beaconChainETHStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, beaconChainETHStrategyCall, N> {
            self.call_builder(&beaconChainETHStrategyCall {})
        }
        ///Creates a new call builder for the [`calculateCurrentStakerDelegationDigestHash`] function.
        pub fn calculateCurrentStakerDelegationDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateCurrentStakerDelegationDigestHashCall, N>
        {
            self.call_builder(&calculateCurrentStakerDelegationDigestHashCall {
                staker,
                operator,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculateDelegationApprovalDigestHash`] function.
        pub fn calculateDelegationApprovalDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            _delegationApprover: alloy::sol_types::private::Address,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateDelegationApprovalDigestHashCall, N>
        {
            self.call_builder(&calculateDelegationApprovalDigestHashCall {
                staker,
                operator,
                _delegationApprover,
                approverSalt,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculateStakerDelegationDigestHash`] function.
        pub fn calculateStakerDelegationDigestHash(
            &self,
            staker: alloy::sol_types::private::Address,
            _stakerNonce: alloy::sol_types::private::primitives::aliases::U256,
            operator: alloy::sol_types::private::Address,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateStakerDelegationDigestHashCall, N>
        {
            self.call_builder(&calculateStakerDelegationDigestHashCall {
                staker,
                _stakerNonce,
                operator,
                expiry,
            })
        }
        ///Creates a new call builder for the [`calculateWithdrawalRoot`] function.
        pub fn calculateWithdrawalRoot(
            &self,
            withdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateWithdrawalRootCall, N> {
            self.call_builder(&calculateWithdrawalRootCall { withdrawal })
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawal`] function.
        pub fn completeQueuedWithdrawal(
            &self,
            withdrawal: <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
            tokens: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            middlewareTimesIndex: alloy::sol_types::private::primitives::aliases::U256,
            receiveAsTokens: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalCall, N> {
            self.call_builder(&completeQueuedWithdrawalCall {
                withdrawal,
                tokens,
                middlewareTimesIndex,
                receiveAsTokens,
            })
        }
        ///Creates a new call builder for the [`completeQueuedWithdrawals`] function.
        pub fn completeQueuedWithdrawals(
            &self,
            withdrawals: alloy::sol_types::private::Vec<
                <IDelegationManager::Withdrawal as alloy::sol_types::SolType>::RustType,
            >,
            tokens: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            middlewareTimesIndexes: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            receiveAsTokens: alloy::sol_types::private::Vec<bool>,
        ) -> alloy_contract::SolCallBuilder<T, &P, completeQueuedWithdrawalsCall, N> {
            self.call_builder(&completeQueuedWithdrawalsCall {
                withdrawals,
                tokens,
                middlewareTimesIndexes,
                receiveAsTokens,
            })
        }
        ///Creates a new call builder for the [`cumulativeWithdrawalsQueued`] function.
        pub fn cumulativeWithdrawalsQueued(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeWithdrawalsQueuedCall, N> {
            self.call_builder(&cumulativeWithdrawalsQueuedCall { _0 })
        }
        ///Creates a new call builder for the [`decreaseDelegatedShares`] function.
        pub fn decreaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            shares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, decreaseDelegatedSharesCall, N> {
            self.call_builder(&decreaseDelegatedSharesCall {
                staker,
                strategy,
                shares,
            })
        }
        ///Creates a new call builder for the [`delegateTo`] function.
        pub fn delegateTo(
            &self,
            operator: alloy::sol_types::private::Address,
            approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToCall, N> {
            self.call_builder(&delegateToCall {
                operator,
                approverSignatureAndExpiry,
                approverSalt,
            })
        }
        ///Creates a new call builder for the [`delegateToBySignature`] function.
        pub fn delegateToBySignature(
            &self,
            staker: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
            stakerSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSignatureAndExpiry: <ISignatureUtils::SignatureWithExpiry as alloy::sol_types::SolType>::RustType,
            approverSalt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateToBySignatureCall, N> {
            self.call_builder(&delegateToBySignatureCall {
                staker,
                operator,
                stakerSignatureAndExpiry,
                approverSignatureAndExpiry,
                approverSalt,
            })
        }
        ///Creates a new call builder for the [`delegatedTo`] function.
        pub fn delegatedTo(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatedToCall, N> {
            self.call_builder(&delegatedToCall { _0 })
        }
        ///Creates a new call builder for the [`delegationApprover`] function.
        pub fn delegationApprover(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationApproverCall, N> {
            self.call_builder(&delegationApproverCall { operator })
        }
        ///Creates a new call builder for the [`delegationApproverSaltIsSpent`] function.
        pub fn delegationApproverSaltIsSpent(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationApproverSaltIsSpentCall, N> {
            self.call_builder(&delegationApproverSaltIsSpentCall { _0, _1 })
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
        }
        ///Creates a new call builder for the [`getDelegatableShares`] function.
        pub fn getDelegatableShares(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDelegatableSharesCall, N> {
            self.call_builder(&getDelegatableSharesCall { staker })
        }
        ///Creates a new call builder for the [`getOperatorShares`] function.
        pub fn getOperatorShares(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSharesCall, N> {
            self.call_builder(&getOperatorSharesCall {
                operator,
                strategies,
            })
        }
        ///Creates a new call builder for the [`getWithdrawalDelay`] function.
        pub fn getWithdrawalDelay(
            &self,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalDelayCall, N> {
            self.call_builder(&getWithdrawalDelayCall { strategies })
        }
        ///Creates a new call builder for the [`increaseDelegatedShares`] function.
        pub fn increaseDelegatedShares(
            &self,
            staker: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            shares: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, increaseDelegatedSharesCall, N> {
            self.call_builder(&increaseDelegatedSharesCall {
                staker,
                strategy,
                shares,
            })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _minWithdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
            _strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            _withdrawalDelayBlocks: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _pauserRegistry,
                initialPausedStatus,
                _minWithdrawalDelayBlocks,
                _strategies,
                _withdrawalDelayBlocks,
            })
        }
        ///Creates a new call builder for the [`isDelegated`] function.
        pub fn isDelegated(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isDelegatedCall, N> {
            self.call_builder(&isDelegatedCall { staker })
        }
        ///Creates a new call builder for the [`isOperator`] function.
        pub fn isOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorCall, N> {
            self.call_builder(&isOperatorCall { operator })
        }
        ///Creates a new call builder for the [`minWithdrawalDelayBlocks`] function.
        pub fn minWithdrawalDelayBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minWithdrawalDelayBlocksCall, N> {
            self.call_builder(&minWithdrawalDelayBlocksCall {})
        }
        ///Creates a new call builder for the [`modifyOperatorDetails`] function.
        pub fn modifyOperatorDetails(
            &self,
            newOperatorDetails: <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyOperatorDetailsCall, N> {
            self.call_builder(&modifyOperatorDetailsCall { newOperatorDetails })
        }
        ///Creates a new call builder for the [`operatorDetails`] function.
        pub fn operatorDetails(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorDetailsCall, N> {
            self.call_builder(&operatorDetailsCall { operator })
        }
        ///Creates a new call builder for the [`operatorShares`] function.
        pub fn operatorShares(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSharesCall, N> {
            self.call_builder(&operatorSharesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall {})
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(
            &self,
            index: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<T, &P, paused_1Call, N> {
            self.call_builder(&paused_1Call {})
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall {})
        }
        ///Creates a new call builder for the [`pendingWithdrawals`] function.
        pub fn pendingWithdrawals(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingWithdrawalsCall, N> {
            self.call_builder(&pendingWithdrawalsCall { _0 })
        }
        ///Creates a new call builder for the [`queueWithdrawals`] function.
        pub fn queueWithdrawals(
            &self,
            queuedWithdrawalParams: alloy::sol_types::private::Vec<
                <IDelegationManager::QueuedWithdrawalParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, queueWithdrawalsCall, N> {
            self.call_builder(&queueWithdrawalsCall {
                queuedWithdrawalParams,
            })
        }
        ///Creates a new call builder for the [`registerAsOperator`] function.
        pub fn registerAsOperator(
            &self,
            registeringOperatorDetails: <IDelegationManager::OperatorDetails as alloy::sol_types::SolType>::RustType,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerAsOperatorCall, N> {
            self.call_builder(&registerAsOperatorCall {
                registeringOperatorDetails,
                metadataURI,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setMinWithdrawalDelayBlocks`] function.
        pub fn setMinWithdrawalDelayBlocks(
            &self,
            newMinWithdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setMinWithdrawalDelayBlocksCall, N> {
            self.call_builder(&setMinWithdrawalDelayBlocksCall {
                newMinWithdrawalDelayBlocks,
            })
        }
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(&setPauserRegistryCall { newPauserRegistry })
        }
        ///Creates a new call builder for the [`setStrategyWithdrawalDelayBlocks`] function.
        pub fn setStrategyWithdrawalDelayBlocks(
            &self,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            withdrawalDelayBlocks: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStrategyWithdrawalDelayBlocksCall, N>
        {
            self.call_builder(&setStrategyWithdrawalDelayBlocksCall {
                strategies,
                withdrawalDelayBlocks,
            })
        }
        ///Creates a new call builder for the [`slasher`] function.
        pub fn slasher(&self) -> alloy_contract::SolCallBuilder<T, &P, slasherCall, N> {
            self.call_builder(&slasherCall {})
        }
        ///Creates a new call builder for the [`stakerNonce`] function.
        pub fn stakerNonce(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakerNonceCall, N> {
            self.call_builder(&stakerNonceCall { _0 })
        }
        ///Creates a new call builder for the [`stakerOptOutWindowBlocks`] function.
        pub fn stakerOptOutWindowBlocks(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakerOptOutWindowBlocksCall, N> {
            self.call_builder(&stakerOptOutWindowBlocksCall { operator })
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`strategyWithdrawalDelayBlocks`] function.
        pub fn strategyWithdrawalDelayBlocks(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyWithdrawalDelayBlocksCall, N> {
            self.call_builder(&strategyWithdrawalDelayBlocksCall { _0 })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`undelegate`] function.
        pub fn undelegate(
            &self,
            staker: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, undelegateCall, N> {
            self.call_builder(&undelegateCall { staker })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateOperatorMetadataURI`] function.
        pub fn updateOperatorMetadataURI(
            &self,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorMetadataURICall, N> {
            self.call_builder(&updateOperatorMetadataURICall { metadataURI })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > DelegationManagerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinWithdrawalDelayBlocksSet`] event.
        pub fn MinWithdrawalDelayBlocksSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinWithdrawalDelayBlocksSet, N> {
            self.event_filter::<MinWithdrawalDelayBlocksSet>()
        }
        ///Creates a new event filter for the [`OperatorDetailsModified`] event.
        pub fn OperatorDetailsModified_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDetailsModified, N> {
            self.event_filter::<OperatorDetailsModified>()
        }
        ///Creates a new event filter for the [`OperatorMetadataURIUpdated`] event.
        pub fn OperatorMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorMetadataURIUpdated, N> {
            self.event_filter::<OperatorMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSharesDecreased`] event.
        pub fn OperatorSharesDecreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesDecreased, N> {
            self.event_filter::<OperatorSharesDecreased>()
        }
        ///Creates a new event filter for the [`OperatorSharesIncreased`] event.
        pub fn OperatorSharesIncreased_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSharesIncreased, N> {
            self.event_filter::<OperatorSharesIncreased>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PauserRegistrySet`] event.
        pub fn PauserRegistrySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PauserRegistrySet, N> {
            self.event_filter::<PauserRegistrySet>()
        }
        ///Creates a new event filter for the [`StakerDelegated`] event.
        pub fn StakerDelegated_filter(&self) -> alloy_contract::Event<T, &P, StakerDelegated, N> {
            self.event_filter::<StakerDelegated>()
        }
        ///Creates a new event filter for the [`StakerForceUndelegated`] event.
        pub fn StakerForceUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerForceUndelegated, N> {
            self.event_filter::<StakerForceUndelegated>()
        }
        ///Creates a new event filter for the [`StakerUndelegated`] event.
        pub fn StakerUndelegated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakerUndelegated, N> {
            self.event_filter::<StakerUndelegated>()
        }
        ///Creates a new event filter for the [`StrategyWithdrawalDelayBlocksSet`] event.
        pub fn StrategyWithdrawalDelayBlocksSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyWithdrawalDelayBlocksSet, N> {
            self.event_filter::<StrategyWithdrawalDelayBlocksSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`WithdrawalCompleted`] event.
        pub fn WithdrawalCompleted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalCompleted, N> {
            self.event_filter::<WithdrawalCompleted>()
        }
        ///Creates a new event filter for the [`WithdrawalQueued`] event.
        pub fn WithdrawalQueued_filter(&self) -> alloy_contract::Event<T, &P, WithdrawalQueued, N> {
            self.event_filter::<WithdrawalQueued>()
        }
    }
}
