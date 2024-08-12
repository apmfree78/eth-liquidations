pub use liquidate_user::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod liquidate_user {
    // const _: () = {
    //     ::core::include_bytes!(
    //         "/Users/apmfree/Desktop/rust/learn_ethers/examples/abi/LiquidateUser.json",
    //     );
    // };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aavePoolAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aaveDataProviderAddress",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aavePriceOracleAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aavePoolAddressProviderAddress",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("swapRouterAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("walletAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ADDRESSES_PROVIDER"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPoolAddressesProvider",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POOL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("POOL"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IPool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("executeOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("executeOperation"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("premium"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("findAndLiquidateAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("findAndLiquidateAccount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("users"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct LiquidateUser.User[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("LiquidateAccount"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("LiquidateAccount"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("liquidator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("beneficiary"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("collateralToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("profit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceToPayLoan"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalanceToPayLoan",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoCollateralToken"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoCollateralToken"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoUserAccountQualifiedForLiquidation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NoUserAccountQualifiedForLiquidation",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotEnoughDebtTokenToCoverLiquidation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NotEnoughDebtTokenToCoverLiquidation",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATE_USER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@Qa \x0C8\x03\x80a \x0C\x839\x81\x01`@\x81\x90Ra\x000\x91a\0\x85V[`\0\x80T`\xFF\x19\x16\x90U`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x80R\x93\x85\x16`\xA0R\x91\x84\x16`\xC0R\x83\x16`\xE0R\x82\x16a\x01\0R\x16a\x01 Ra\0\xF9V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x80W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\0\x9EW`\0\x80\xFD[a\0\xA7\x87a\0iV[\x95Pa\0\xB5` \x88\x01a\0iV[\x94Pa\0\xC3`@\x88\x01a\0iV[\x93Pa\0\xD1``\x88\x01a\0iV[\x92Pa\0\xDF`\x80\x88\x01a\0iV[\x91Pa\0\xED`\xA0\x88\x01a\0iV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x1E\x9Ca\x01p`\09`\0\x81\x81a\x06%\x01R\x81\x81a\r\xFC\x01Ra\x0E\xFB\x01R`\0a\ty\x01R`\0`S\x01R`\0a\x10W\x01R`\0\x81\x81a\x106\x01R\x81\x81a\x14\x04\x01Ra\x14\xBB\x01R`\0\x81\x81`\xB5\x01R\x81\x81a\x03\x1B\x01Ra\x07\r\x01Ra\x1E\x9C`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x05B\x97\\\x14a\0QW\x80c\x1B\x11\xD0\xFF\x14a\0\x90W\x80cu5\xD2F\x14a\0\xB3W\x80c\xA2U\xDF\x16\x14a\0\xD9W[`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA3a\0\x9E6`\x04a\x16(V[a\0\xEEV[`@Q\x90\x15\x15\x81R` \x01a\0\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0sV[a\0\xECa\0\xE76`\x04a\x16\xD5V[a\x02\x8CV[\0[`\0\x80\x80a\0\xFE\x84\x86\x01\x86a\x17LV[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82\x84\x01\x90\x81R\x8E\x82\x16``\x84\x01R\x90\x85\x16`\x80\x83\x01R\x81R` \x80\x82\x01\x8D\x90R\x82Q\x80\x84\x01\x90\x93R`\x14\x83Rsliquidating account!``\x1B\x90\x83\x01R\x92\x94P\x90\x92Pa\x01j\x90a\x06\xA6V[a\x01s\x81a\x06\xECV[\x82`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x8C\x16\x14a\x01\xBCWa\x01\xA8`@Q\x80``\x01`@R\x80`4\x81R` \x01a\x1E3`4\x919a\x06\xA6V[a\x01\xBC\x84\x8Ca\x01\xB7\x8C\x8Ea\x17\x9BV[a\t`V[a\x02F`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x1C\xEF`'\x919`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02A\x91\x90a\x17\xAEV[a\x0C\xA4V[a\x02g`@Q\x80``\x01`@R\x80`;\x81R` \x01a\x1D\xA8`;\x919a\x06\xA6V[a\x02{\x84\x8Ca\x02v\x8C\x8Ea\x17\x9BV[a\x0C\xEDV[P`\x01\x9A\x99PPPPPPPPPPV[`\0T`\xFF\x16\x15a\x02\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x81U`@\x80Q`\xA0\x81\x01\x82R\x90\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R\x81R` \x81\x01\x82\x90R\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0[\x84\x81\x10\x15a\x04\xCEW`\0\x87\x87\x83\x81\x81\x10a\x03ZWa\x03Za\x17\xC7V[a\x03p\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x17\xDDV[`@Qc/\xE4\xA1_`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P`\0\x91\x87\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE0\x91\x90a\x18\x01V[\x95PPPPPPa\x04\x19`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o42\xB0\xB6:4\x1030\xB1\xBA7\xB9\x10\x1E\x9F`\x81\x1B\x81RP\x82a\x0C\xA4V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x80\x15a\x047WPg\x01cEx]\x8A\0\0\x81\x11[\x15a\x04\xC4W`\0\x80a\x04`\x8B\x8B\x87\x81\x81\x10a\x04TWa\x04Ta\x17\xC7V[\x90P``\x02\x01\x84a\x101V[\x91P\x91Ph\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x82\x11\x80\x15a\x04{WP\x86\x82\x11[\x15a\x04\xC1W\x81\x96P`@Q\x80`@\x01`@R\x80\x8C\x8C\x88\x81\x81\x10a\x04\xA0Wa\x04\xA0a\x17\xC7V[\x90P``\x02\x01\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a\x18KV[\x81R` \x01\x82\x90R\x95P[PP[PP`\x01\x01a\x03>V[P\x80QQ`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{Wa\x05\x0C`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\x83\x93{3K\xA1\x01\xE9\xF1`\xB5\x1B\x81RP\x83a\x0C\xA4V[a\x05C`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x03#+\x13\xA1\x03\xA3y\x02\x1B{\xB3+\x91\x01\xE9\xF1`}\x1B\x81RP\x82` \x01Qa\x0C\xA4V[\x80Q`@\x80\x82\x01Q\x91Q\x81Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x80\x83\x01\x91\x90\x91R\x91\x84\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x93\x84\x90R\x85Q\x83\x01Q\x92\x86\x01Qc\x10\xAC-\xDF`\xE2\x1B\x90\x94R\x93\x87\x16\x92cB\xB0\xB7|\x92a\x05\xAD\x920\x92\x90\x87\x90`\0\x90`d\x01a\x19\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPPa\x06\0`@Q\x80``\x01`@R\x80`;\x81R` \x01a\x1Dm`;\x919a\x06\xA6V[\x81Q`@\x80\x82\x01Q\x91Q\x81Q\x86\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x82\x01R\x92\x81\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x910\x91\x7F\xD3\x97\x14\xF5\xA9-\xFC\xC8\x91\xCB6\x8B\xF8\xC2\xA1\xEE\xDDc\xE7\xAE\xAD\x0F&\xCB\xA8\xBES^\xA4\x02\x05\xFF\x91\x01`@Q\x80\x91\x03\x90\xA4Pa\x06\x94V[`@Qc<\x10\xB0\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\0\x80T`\xFF\x19\x16\x90UPPPPV[a\x06\xE9\x81`@Q`$\x01a\x06\xBA\x91\x90a\x19\\V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x13\xD1V[PV[\x80Q` \x90\x81\x01Q\x90\x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x84\x91\x90a\x17\xAEV[\x10a\t&W\x80`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x83\x85` \x01Q`\x03a\x07\xAA\x91\x90a\x19oV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x19\x91\x90a\x19\x9BV[P\x82Q`@\x80\x82\x01Q` \x80\x84\x01Q\x93Q\x90\x87\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x93\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R`d\x83\x01\x91\x90\x91R`\0`\x84\x83\x01R\x83\x16\x90b\xA7\x18\xA9\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPPa\x08\xEC`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x1E\x0C`'\x919`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01a\x02\0V[a\t!`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01sLiqudation executed!``\x1B\x81RPa\x06\xA6V[PPPV[a\tG`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x1DD`)\x919a\x06\xA6V[`@Qc\x03\xDCq\xB7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEE\x91\x90a\x17\xAEV[`\0\x03a\n\x0EW`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ny\x91\x90a\x17\xAEV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x8A\x81\x1B\x82\x16` \x84\x01Ra\x01w`\xEB\x1B`4\x84\x01R\x89\x90\x1B\x16`7\x82\x01R\x90\x91P`\0\x90`K\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\xA0\x83\x01\x82R\x80\x83R0` \x84\x01R\x92P`\0\x91\x90\x81\x01a\n\xE3Ba\x01,a\x17\x9BV[\x81R` \x81\x01\x89\x90R`@\x90\x81\x01\x85\x90RQc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x91\x92P\x90\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bi\x91\x90a\x19\x9BV[P`@Qc\x1EQ\x80\x93`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xF2\x8C\x04\x98\x90a\x0B\x96\x90\x84\x90`\x04\x01a\x19\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD9\x91\x90a\x17\xAEV[Pa\x0C\x10`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x1D\x1B\xDA\xD9[\x88\x1C\xDD\xD8\\\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`Z\x1B\x81RPa\x06\xA6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x87\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cz\x91\x90a\x17\xAEV[\x10\x15a\x0C\x99W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[a\x0C\xE9\x82\x82`@Q`$\x01a\x0C\xBA\x92\x91\x90a\x1A\x0EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\tq\n\x9D`\xE4\x1B\x17\x90Ra\x13\xD1V[PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\\\x91\x90a\x17\xAEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCA\x91\x90a\x17\xAEV[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9BW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0ETW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ex\x91\x90a\x19\x9BV[Pa\x0E\x9B`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x1D\xE3`)\x919\x83a\x0C\xA4V[\x84\x81\x11\x15a\x10(W`\0a\x0E\xAF\x86\x83a\x1A0V[\x90Pa\x0E\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01premaining balance`x\x1B\x81RP\x82a\x0C\xA4V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x86\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0FSW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fw\x91\x90a\x19\x9BV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE3\x91\x90a\x17\xAEV[\x90P\x86\x81\x10\x15a\x10\x06W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x99`@Q\x80``\x01`@R\x80`.\x81R` \x01a\x1D\x16`.\x919\x83a\x0C\xA4V[PPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\x9Ca\x10\x8A`@\x89\x01` \x8A\x01a\x17\xDDV[a\x10\x97` \x8A\x01\x8Aa\x17\xDDV[a\x13\xDAV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x84\x16c(\xDD-\x01a\x10\xBF``\x8B\x01`@\x8C\x01a\x17\xDDV[a\x10\xCC` \x8C\x01\x8Ca\x17\xDDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11<\x91\x90a\x1ACV[\x98PPPPPPPPP`\0g\x06\xF0[Y\xD3\xB2\0\0\x90Pg\r/\x13\xF7x\x9F\0\0\x88\x10\x15a\x11nWPg\r\xE0\xB6\xB3\xA7d\0\0[`\0a\x11\x88a\x11\x83``\x8C\x01`@\x8D\x01a\x17\xDDV[a\x14\x99V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x86\x16c\xB3Yo\x07a\x11\xAB`@\x8E\x01` \x8F\x01a\x17\xDDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x13\x91\x90a\x17\xAEV[\x90P`\0a\x12/a\x12*`@\x8E\x01` \x8F\x01a\x17\xDDV[a\x15>V[\x90Pa\x12s`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x03#+\x13\xA1\x03\xA3{[+q\x01\xE9\xF1`\x95\x1B\x81RP\x8D` \x01` \x81\x01\x90a\x12n\x91\x90a\x17\xDDV[a\x15\xADV[a\x12\xB6`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x03\x1B{cc\x0B\xA3+\x93\x0Ba\x03\xA3{[+q\x01\xE9\xF1`e\x1B\x81RP\x8D`@\x01` \x81\x01\x90a\x12n\x91\x90a\x17\xDDV[a\x12\xE5`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l:7\xBA0\xB6\x1022\xB1:\x10\x1E\x9F`\x99\x1B\x81RP\x87a\x0C\xA4V[a\x13\x1A`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r22\xB1:\x10:7\xB5\xB2\xB7\x10(94\xB1\xB2\x90\x1E\x9F`i\x1B\x81RP\x83a\x0C\xA4V[`\0a\x13&\x85\x88a\x19oV[\x90Pa\x13:g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x1A\xF0V[\x90P`\0\x84\x11\x80\x15a\x13IWP\x85[\x15a\x13\xB9W`\0\x82a\x13[\x85\x84a\x19oV[a\x13e\x91\x90a\x1A\xF0V[\x90Pa\x13q\x85\x82a\x19oV[\x90Pa\x13\x7Fa'\x10\x86a\x1A0V[a\x13\x89\x90\x82a\x19oV[\x90Pa\x13\x97a'\x10\x82a\x1A\xF0V[\x90Pa\x13\xA5a'\x10\x82a\x1A\xF0V[\x9BP\x90\x99Pa\x13\xCA\x98PPPPPPPPPV[`\0\x80\x9AP\x9APPPPPPPPPP[\x92P\x92\x90PV[a\x06\xE9\x81a\x15\xF2V[`@Qc(\xDD-\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x82\x91\x84\x16\x90c(\xDD-\x01\x90`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14v\x91\x90a\x1ACV[PPPPPP\x92P\x92PP\x80\x82a\x14\x8D\x91\x90a\x17\x9BV[\x93PPPP[\x92\x91PPV[`@Qc>\x15\x01A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c>\x15\x01A\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15,\x91\x90a\x1B\x12V[P\x94\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA2\x91\x90a\x1B\xABV[a\x14\x93\x90`\na\x1C\xB5V[a\x0C\xE9\x82\x82`@Q`$\x01a\x15\xC3\x92\x91\x90a\x1C\xC4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x13\xD1V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xE9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x16AW`\0\x80\xFD[\x865a\x16L\x81a\x16\x13V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a\x16j\x81a\x16\x13V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x86W`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x16\x97W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xAEW`\0\x80\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x16\xC0W`\0\x80\xFD[` \x82\x01\x93P\x80\x92PPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80` \x83\x85\x03\x12\x15a\x16\xE8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xFFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x17\x10W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17'W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x17<W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17_W`\0\x80\xFD[\x825a\x17j\x81a\x16\x13V[\x91P` \x83\x015a\x17z\x81a\x16\x13V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x14\x93Wa\x14\x93a\x17\x85V[`\0` \x82\x84\x03\x12\x15a\x17\xC0W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xEFW`\0\x80\xFD[\x815a\x17\xFA\x81a\x16\x13V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18\x1AW`\0\x80\xFD[PP\x84Q` \x86\x01Q`@\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x90\x99\x01Q\x93\x9A\x92\x99P\x90\x97\x90\x96P\x94P\x90\x92P\x90PV[`\0``\x82\x84\x03\x12\x80\x15a\x18^W`\0\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\x90WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x18\x9E\x81a\x16\x13V[\x81R` \x83\x015a\x18\xAE\x81a\x16\x13V[` \x82\x01R`@\x83\x015a\x18\xC1\x81a\x16\x13V[`@\x82\x01R\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\xF3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xD7V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R`\xA0``\x82\x01\x81\x90R`\0\x90a\x19F\x90\x83\x01\x85a\x18\xCDV[\x90Pa\xFF\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[` \x81R`\0a\x17\xFA` \x83\x01\x84a\x18\xCDV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x14\x93Wa\x14\x93a\x17\x85V[\x80Q\x80\x15\x15\x81\x14a\x19\x96W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x19\xADW`\0\x80\xFD[a\x17\xFA\x82a\x19\x86V[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x19\xD2`\xC0\x84\x01\x82a\x18\xCDV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`@\x81R`\0a\x1A!`@\x83\x01\x85a\x18\xCDV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x14\x93Wa\x14\x93a\x17\x85V[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x1AbW`\0\x80\xFD[`\0\x8AQ\x90P\x80\x99PP`\0` \x8B\x01Q\x90P\x80\x98PP`\0`@\x8B\x01Q\x90P\x80\x97PP`\0``\x8B\x01Q\x90P\x80\x96PP`\0`\x80\x8B\x01Q\x90P\x80\x95PP`\0`\xA0\x8B\x01Q\x90P\x80\x94PP`\0`\xC0\x8B\x01Q\x90P\x80\x93PP`\xE0\x8A\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xD2W`\0\x80\xFD[\x91Pa\x1A\xE1a\x01\0\x8B\x01a\x19\x86V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x82a\x1B\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\x1B2W`\0\x80\xFD[\x8AQ` \x8C\x01Q`@\x8D\x01Q``\x8E\x01Q`\x80\x8F\x01Q\x93\x9DP\x91\x9BP\x99P\x97P\x95Pa\x1B``\xA0\x8C\x01a\x19\x86V[\x94Pa\x1Bn`\xC0\x8C\x01a\x19\x86V[\x93Pa\x1B|`\xE0\x8C\x01a\x19\x86V[\x92Pa\x1B\x8Ba\x01\0\x8C\x01a\x19\x86V[\x91Pa\x1B\x9Aa\x01 \x8C\x01a\x19\x86V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\x1B\xBDW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x17\xFAW`\0\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\x1C\tW\x80\x85\x04\x81\x11\x15a\x1B\xEDWa\x1B\xEDa\x17\x85V[`\x01\x84\x16\x15a\x1B\xFBW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x1B\xD2V[\x93P\x93\x91PPV[`\0\x82a\x1C WP`\x01a\x14\x93V[\x81a\x1C-WP`\0a\x14\x93V[\x81`\x01\x81\x14a\x1CCW`\x02\x81\x14a\x1CMWa\x1CiV[`\x01\x91PPa\x14\x93V[`\xFF\x84\x11\x15a\x1C^Wa\x1C^a\x17\x85V[PP`\x01\x82\x1Ba\x14\x93V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1C\x8CWP\x81\x81\na\x14\x93V[a\x1C\x99`\0\x19\x84\x84a\x1B\xCEV[\x80`\0\x19\x04\x82\x11\x15a\x1C\xADWa\x1C\xADa\x17\x85V[\x02\x93\x92PPPV[`\0a\x17\xFA`\xFF\x84\x16\x83a\x1C\x11V[`@\x81R`\0a\x1C\xD7`@\x83\x01\x85a\x18\xCDV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV\xFEcollateral token balance AFTER swap => profit taken of amount (outstading debt token)Not enough tokens to cover the liqudationflashloan successfully executed and liquidation successful!transfering remaining collateral token to liquidator walletprofit taken of amount (collateral Token)debt Token balance after liquidation =>swapping collateral token debt token to pay off debt\xA2dipfsX\"\x12 c\xFEe=\x05\xD8\x96,\x0EN\x06\xAD\xCF\xA5\xDE\x91\x9FTE\r{?\xC9?\x90\xA9\xAE\xDF\xB8\x9E\xBA\x03dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATE_USER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x05B\x97\\\x14a\0QW\x80c\x1B\x11\xD0\xFF\x14a\0\x90W\x80cu5\xD2F\x14a\0\xB3W\x80c\xA2U\xDF\x16\x14a\0\xD9W[`\0\x80\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xA3a\0\x9E6`\x04a\x16(V[a\0\xEEV[`@Q\x90\x15\x15\x81R` \x01a\0\x87V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0sV[a\0\xECa\0\xE76`\x04a\x16\xD5V[a\x02\x8CV[\0[`\0\x80\x80a\0\xFE\x84\x86\x01\x86a\x17LV[`@\x80Q`\xA0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82\x84\x01\x90\x81R\x8E\x82\x16``\x84\x01R\x90\x85\x16`\x80\x83\x01R\x81R` \x80\x82\x01\x8D\x90R\x82Q\x80\x84\x01\x90\x93R`\x14\x83Rsliquidating account!``\x1B\x90\x83\x01R\x92\x94P\x90\x92Pa\x01j\x90a\x06\xA6V[a\x01s\x81a\x06\xECV[\x82`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x8C\x16\x14a\x01\xBCWa\x01\xA8`@Q\x80``\x01`@R\x80`4\x81R` \x01a\x1E3`4\x919a\x06\xA6V[a\x01\xBC\x84\x8Ca\x01\xB7\x8C\x8Ea\x17\x9BV[a\t`V[a\x02F`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x1C\xEF`'\x919`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02A\x91\x90a\x17\xAEV[a\x0C\xA4V[a\x02g`@Q\x80``\x01`@R\x80`;\x81R` \x01a\x1D\xA8`;\x919a\x06\xA6V[a\x02{\x84\x8Ca\x02v\x8C\x8Ea\x17\x9BV[a\x0C\xEDV[P`\x01\x9A\x99PPPPPPPPPPV[`\0T`\xFF\x16\x15a\x02\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x81U`@\x80Q`\xA0\x81\x01\x82R\x90\x81\x01\x82\x81R``\x82\x01\x83\x90R`\x80\x82\x01\x83\x90R\x81R` \x81\x01\x82\x90R\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\0[\x84\x81\x10\x15a\x04\xCEW`\0\x87\x87\x83\x81\x81\x10a\x03ZWa\x03Za\x17\xC7V[a\x03p\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x17\xDDV[`@Qc/\xE4\xA1_`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P`\0\x91\x87\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE0\x91\x90a\x18\x01V[\x95PPPPPPa\x04\x19`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o42\xB0\xB6:4\x1030\xB1\xBA7\xB9\x10\x1E\x9F`\x81\x1B\x81RP\x82a\x0C\xA4V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x80\x15a\x047WPg\x01cEx]\x8A\0\0\x81\x11[\x15a\x04\xC4W`\0\x80a\x04`\x8B\x8B\x87\x81\x81\x10a\x04TWa\x04Ta\x17\xC7V[\x90P``\x02\x01\x84a\x101V[\x91P\x91Ph\x02\xB5\xE3\xAF\x16\xB1\x88\0\0\x82\x11\x80\x15a\x04{WP\x86\x82\x11[\x15a\x04\xC1W\x81\x96P`@Q\x80`@\x01`@R\x80\x8C\x8C\x88\x81\x81\x10a\x04\xA0Wa\x04\xA0a\x17\xC7V[\x90P``\x02\x01\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a\x18KV[\x81R` \x01\x82\x90R\x95P[PP[PP`\x01\x01a\x03>V[P\x80QQ`\x01`\x01`\xA0\x1B\x03\x16\x15a\x06{Wa\x05\x0C`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\x83\x93{3K\xA1\x01\xE9\xF1`\xB5\x1B\x81RP\x83a\x0C\xA4V[a\x05C`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p\x03#+\x13\xA1\x03\xA3y\x02\x1B{\xB3+\x91\x01\xE9\xF1`}\x1B\x81RP\x82` \x01Qa\x0C\xA4V[\x80Q`@\x80\x82\x01Q\x91Q\x81Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x80\x83\x01\x91\x90\x91R\x91\x84\x16\x81\x84\x01R\x82Q\x80\x82\x03\x84\x01\x81R``\x82\x01\x93\x84\x90R\x85Q\x83\x01Q\x92\x86\x01Qc\x10\xAC-\xDF`\xE2\x1B\x90\x94R\x93\x87\x16\x92cB\xB0\xB7|\x92a\x05\xAD\x920\x92\x90\x87\x90`\0\x90`d\x01a\x19\x13V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xC7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xDBW=`\0\x80>=`\0\xFD[PPPPa\x06\0`@Q\x80``\x01`@R\x80`;\x81R` \x01a\x1Dm`;\x919a\x06\xA6V[\x81Q`@\x80\x82\x01Q\x91Q\x81Q\x86\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16` \x82\x01R\x92\x81\x16\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x910\x91\x7F\xD3\x97\x14\xF5\xA9-\xFC\xC8\x91\xCB6\x8B\xF8\xC2\xA1\xEE\xDDc\xE7\xAE\xAD\x0F&\xCB\xA8\xBES^\xA4\x02\x05\xFF\x91\x01`@Q\x80\x91\x03\x90\xA4Pa\x06\x94V[`@Qc<\x10\xB0\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\0\x80T`\xFF\x19\x16\x90UPPPPV[a\x06\xE9\x81`@Q`$\x01a\x06\xBA\x91\x90a\x19\\V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x13\xD1V[PV[\x80Q` \x90\x81\x01Q\x90\x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x84\x91\x90a\x17\xAEV[\x10a\t&W\x80`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x83\x85` \x01Q`\x03a\x07\xAA\x91\x90a\x19oV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x19\x91\x90a\x19\x9BV[P\x82Q`@\x80\x82\x01Q` \x80\x84\x01Q\x93Q\x90\x87\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x93\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R`d\x83\x01\x91\x90\x91R`\0`\x84\x83\x01R\x83\x16\x90b\xA7\x18\xA9\x90`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x8CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA0W=`\0\x80>=`\0\xFD[PPPPa\x08\xEC`@Q\x80``\x01`@R\x80`'\x81R` \x01a\x1E\x0C`'\x919`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01a\x02\0V[a\t!`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01sLiqudation executed!``\x1B\x81RPa\x06\xA6V[PPPV[a\tG`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x1DD`)\x919a\x06\xA6V[`@Qc\x03\xDCq\xB7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xEE\x91\x90a\x17\xAEV[`\0\x03a\n\x0EW`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nUW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ny\x91\x90a\x17\xAEV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x8A\x81\x1B\x82\x16` \x84\x01Ra\x01w`\xEB\x1B`4\x84\x01R\x89\x90\x1B\x16`7\x82\x01R\x90\x91P`\0\x90`K\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\xA0\x83\x01\x82R\x80\x83R0` \x84\x01R\x92P`\0\x91\x90\x81\x01a\n\xE3Ba\x01,a\x17\x9BV[\x81R` \x81\x01\x89\x90R`@\x90\x81\x01\x85\x90RQc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x91\x92P\x90\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0BEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bi\x91\x90a\x19\x9BV[P`@Qc\x1EQ\x80\x93`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xF2\x8C\x04\x98\x90a\x0B\x96\x90\x84\x90`\x04\x01a\x19\xB6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD9\x91\x90a\x17\xAEV[Pa\x0C\x10`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x1D\x1B\xDA\xD9[\x88\x1C\xDD\xD8\\\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`Z\x1B\x81RPa\x06\xA6V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x87\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cz\x91\x90a\x17\xAEV[\x10\x15a\x0C\x99W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[a\x0C\xE9\x82\x82`@Q`$\x01a\x0C\xBA\x92\x91\x90a\x1A\x0EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\tq\n\x9D`\xE4\x1B\x17\x90Ra\x13\xD1V[PPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\\\x91\x90a\x17\xAEV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xCA\x91\x90a\x17\xAEV[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x9BW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0ETW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ex\x91\x90a\x19\x9BV[Pa\x0E\x9B`@Q\x80``\x01`@R\x80`)\x81R` \x01a\x1D\xE3`)\x919\x83a\x0C\xA4V[\x84\x81\x11\x15a\x10(W`\0a\x0E\xAF\x86\x83a\x1A0V[\x90Pa\x0E\xE4`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01premaining balance`x\x1B\x81RP\x82a\x0C\xA4V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x86\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0FSW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fw\x91\x90a\x19\x9BV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE3\x91\x90a\x17\xAEV[\x90P\x86\x81\x10\x15a\x10\x06W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x99`@Q\x80``\x01`@R\x80`.\x81R` \x01a\x1D\x16`.\x919\x83a\x0C\xA4V[PPPPPPPV[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x10\x9Ca\x10\x8A`@\x89\x01` \x8A\x01a\x17\xDDV[a\x10\x97` \x8A\x01\x8Aa\x17\xDDV[a\x13\xDAV[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x84\x16c(\xDD-\x01a\x10\xBF``\x8B\x01`@\x8C\x01a\x17\xDDV[a\x10\xCC` \x8C\x01\x8Ca\x17\xDDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11<\x91\x90a\x1ACV[\x98PPPPPPPPP`\0g\x06\xF0[Y\xD3\xB2\0\0\x90Pg\r/\x13\xF7x\x9F\0\0\x88\x10\x15a\x11nWPg\r\xE0\xB6\xB3\xA7d\0\0[`\0a\x11\x88a\x11\x83``\x8C\x01`@\x8D\x01a\x17\xDDV[a\x14\x99V[\x90P`\0`\x01`\x01`\xA0\x1B\x03\x86\x16c\xB3Yo\x07a\x11\xAB`@\x8E\x01` \x8F\x01a\x17\xDDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x13\x91\x90a\x17\xAEV[\x90P`\0a\x12/a\x12*`@\x8E\x01` \x8F\x01a\x17\xDDV[a\x15>V[\x90Pa\x12s`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x03#+\x13\xA1\x03\xA3{[+q\x01\xE9\xF1`\x95\x1B\x81RP\x8D` \x01` \x81\x01\x90a\x12n\x91\x90a\x17\xDDV[a\x15\xADV[a\x12\xB6`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x03\x1B{cc\x0B\xA3+\x93\x0Ba\x03\xA3{[+q\x01\xE9\xF1`e\x1B\x81RP\x8D`@\x01` \x81\x01\x90a\x12n\x91\x90a\x17\xDDV[a\x12\xE5`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l:7\xBA0\xB6\x1022\xB1:\x10\x1E\x9F`\x99\x1B\x81RP\x87a\x0C\xA4V[a\x13\x1A`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r22\xB1:\x10:7\xB5\xB2\xB7\x10(94\xB1\xB2\x90\x1E\x9F`i\x1B\x81RP\x83a\x0C\xA4V[`\0a\x13&\x85\x88a\x19oV[\x90Pa\x13:g\r\xE0\xB6\xB3\xA7d\0\0\x82a\x1A\xF0V[\x90P`\0\x84\x11\x80\x15a\x13IWP\x85[\x15a\x13\xB9W`\0\x82a\x13[\x85\x84a\x19oV[a\x13e\x91\x90a\x1A\xF0V[\x90Pa\x13q\x85\x82a\x19oV[\x90Pa\x13\x7Fa'\x10\x86a\x1A0V[a\x13\x89\x90\x82a\x19oV[\x90Pa\x13\x97a'\x10\x82a\x1A\xF0V[\x90Pa\x13\xA5a'\x10\x82a\x1A\xF0V[\x9BP\x90\x99Pa\x13\xCA\x98PPPPPPPPPV[`\0\x80\x9AP\x9APPPPPPPPPP[\x92P\x92\x90PV[a\x06\xE9\x81a\x15\xF2V[`@Qc(\xDD-\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x82\x91\x84\x16\x90c(\xDD-\x01\x90`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14RW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14v\x91\x90a\x1ACV[PPPPPP\x92P\x92PP\x80\x82a\x14\x8D\x91\x90a\x17\x9BV[\x93PPPP[\x92\x91PPV[`@Qc>\x15\x01A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c>\x15\x01A\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15,\x91\x90a\x1B\x12V[P\x94\x9C\x9BPPPPPPPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA2\x91\x90a\x1B\xABV[a\x14\x93\x90`\na\x1C\xB5V[a\x0C\xE9\x82\x82`@Q`$\x01a\x15\xC3\x92\x91\x90a\x1C\xC4V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x13\xD1V[\x80Qjconsole.log` \x83\x01`\0\x80\x84\x83\x85Z\xFAPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xE9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a\x16AW`\0\x80\xFD[\x865a\x16L\x81a\x16\x13V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a\x16j\x81a\x16\x13V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\x86W`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x16\x97W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xAEW`\0\x80\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x16\xC0W`\0\x80\xFD[` \x82\x01\x93P\x80\x92PPP\x92\x95P\x92\x95P\x92\x95V[`\0\x80` \x83\x85\x03\x12\x15a\x16\xE8W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xFFW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x17\x10W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17'W`\0\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x17<W`\0\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x17_W`\0\x80\xFD[\x825a\x17j\x81a\x16\x13V[\x91P` \x83\x015a\x17z\x81a\x16\x13V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x14\x93Wa\x14\x93a\x17\x85V[`\0` \x82\x84\x03\x12\x15a\x17\xC0W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x17\xEFW`\0\x80\xFD[\x815a\x17\xFA\x81a\x16\x13V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18\x1AW`\0\x80\xFD[PP\x84Q` \x86\x01Q`@\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x90\x99\x01Q\x93\x9A\x92\x99P\x90\x97\x90\x96P\x94P\x90\x92P\x90PV[`\0``\x82\x84\x03\x12\x80\x15a\x18^W`\0\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x18\x90WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x825a\x18\x9E\x81a\x16\x13V[\x81R` \x83\x015a\x18\xAE\x81a\x16\x13V[` \x82\x01R`@\x83\x015a\x18\xC1\x81a\x16\x13V[`@\x82\x01R\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\xF3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xD7V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R`\xA0``\x82\x01\x81\x90R`\0\x90a\x19F\x90\x83\x01\x85a\x18\xCDV[\x90Pa\xFF\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[` \x81R`\0a\x17\xFA` \x83\x01\x84a\x18\xCDV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x14\x93Wa\x14\x93a\x17\x85V[\x80Q\x80\x15\x15\x81\x14a\x19\x96W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x19\xADW`\0\x80\xFD[a\x17\xFA\x82a\x19\x86V[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x19\xD2`\xC0\x84\x01\x82a\x18\xCDV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`@\x81R`\0a\x1A!`@\x83\x01\x85a\x18\xCDV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x14\x93Wa\x14\x93a\x17\x85V[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15a\x1AbW`\0\x80\xFD[`\0\x8AQ\x90P\x80\x99PP`\0` \x8B\x01Q\x90P\x80\x98PP`\0`@\x8B\x01Q\x90P\x80\x97PP`\0``\x8B\x01Q\x90P\x80\x96PP`\0`\x80\x8B\x01Q\x90P\x80\x95PP`\0`\xA0\x8B\x01Q\x90P\x80\x94PP`\0`\xC0\x8B\x01Q\x90P\x80\x93PP`\xE0\x8A\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1A\xD2W`\0\x80\xFD[\x91Pa\x1A\xE1a\x01\0\x8B\x01a\x19\x86V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0\x82a\x1B\rWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\x1B2W`\0\x80\xFD[\x8AQ` \x8C\x01Q`@\x8D\x01Q``\x8E\x01Q`\x80\x8F\x01Q\x93\x9DP\x91\x9BP\x99P\x97P\x95Pa\x1B``\xA0\x8C\x01a\x19\x86V[\x94Pa\x1Bn`\xC0\x8C\x01a\x19\x86V[\x93Pa\x1B|`\xE0\x8C\x01a\x19\x86V[\x92Pa\x1B\x8Ba\x01\0\x8C\x01a\x19\x86V[\x91Pa\x1B\x9Aa\x01 \x8C\x01a\x19\x86V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\x1B\xBDW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x17\xFAW`\0\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\x1C\tW\x80\x85\x04\x81\x11\x15a\x1B\xEDWa\x1B\xEDa\x17\x85V[`\x01\x84\x16\x15a\x1B\xFBW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x1B\xD2V[\x93P\x93\x91PPV[`\0\x82a\x1C WP`\x01a\x14\x93V[\x81a\x1C-WP`\0a\x14\x93V[\x81`\x01\x81\x14a\x1CCW`\x02\x81\x14a\x1CMWa\x1CiV[`\x01\x91PPa\x14\x93V[`\xFF\x84\x11\x15a\x1C^Wa\x1C^a\x17\x85V[PP`\x01\x82\x1Ba\x14\x93V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1C\x8CWP\x81\x81\na\x14\x93V[a\x1C\x99`\0\x19\x84\x84a\x1B\xCEV[\x80`\0\x19\x04\x82\x11\x15a\x1C\xADWa\x1C\xADa\x17\x85V[\x02\x93\x92PPPV[`\0a\x17\xFA`\xFF\x84\x16\x83a\x1C\x11V[`@\x81R`\0a\x1C\xD7`@\x83\x01\x85a\x18\xCDV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV\xFEcollateral token balance AFTER swap => profit taken of amount (outstading debt token)Not enough tokens to cover the liqudationflashloan successfully executed and liquidation successful!transfering remaining collateral token to liquidator walletprofit taken of amount (collateral Token)debt Token balance after liquidation =>swapping collateral token debt token to pay off debt\xA2dipfsX\"\x12 c\xFEe=\x05\xD8\x96,\x0EN\x06\xAD\xCF\xA5\xDE\x91\x9FTE\r{?\xC9?\x90\xA9\xAE\xDF\xB8\x9E\xBA\x03dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATE_USER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LIQUIDATE_USER<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LIQUIDATE_USER<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LIQUIDATE_USER<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LIQUIDATE_USER<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LIQUIDATE_USER<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LIQUIDATE_USER))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LIQUIDATE_USER<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIQUIDATE_USER_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIQUIDATE_USER_ABI.clone(),
                LIQUIDATE_USER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ADDRESSES_PROVIDER` (0x0542975c) function
        pub fn addresses_provider(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([5, 66, 151, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POOL` (0x7535d246) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([117, 53, 210, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeOperation` (0x1b11d0ff) function
        pub fn execute_operation(
            &self,
            asset: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            premium: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Address,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([27, 17, 208, 255], (asset, amount, premium, p3, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findAndLiquidateAccount` (0xa255df16) function
        pub fn find_and_liquidate_account(
            &self,
            users: ::std::vec::Vec<User>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 85, 223, 22], users)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LiquidateAccount` event
        pub fn liquidate_account_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidateAccountFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidateAccountFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LIQUIDATE_USER<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientBalanceToPayLoan` with signature `InsufficientBalanceToPayLoan()` and selector `0x1455dea1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsufficientBalanceToPayLoan",
        abi = "InsufficientBalanceToPayLoan()"
    )]
    pub struct InsufficientBalanceToPayLoan;
    ///Custom Error type `NoCollateralToken` with signature `NoCollateralToken()` and selector `0x687e296b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoCollateralToken", abi = "NoCollateralToken()")]
    pub struct NoCollateralToken;
    ///Custom Error type `NoUserAccountQualifiedForLiquidation` with signature `NoUserAccountQualifiedForLiquidation()` and selector `0x3c10b0ed`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "NoUserAccountQualifiedForLiquidation",
        abi = "NoUserAccountQualifiedForLiquidation()"
    )]
    pub struct NoUserAccountQualifiedForLiquidation;
    ///Custom Error type `NotEnoughDebtTokenToCoverLiquidation` with signature `NotEnoughDebtTokenToCoverLiquidation()` and selector `0x07b8e36e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "NotEnoughDebtTokenToCoverLiquidation",
        abi = "NotEnoughDebtTokenToCoverLiquidation()"
    )]
    pub struct NotEnoughDebtTokenToCoverLiquidation;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LIQUIDATE_USERErrors {
        InsufficientBalanceToPayLoan(InsufficientBalanceToPayLoan),
        NoCollateralToken(NoCollateralToken),
        NoUserAccountQualifiedForLiquidation(NoUserAccountQualifiedForLiquidation),
        NotEnoughDebtTokenToCoverLiquidation(NotEnoughDebtTokenToCoverLiquidation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LIQUIDATE_USERErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalanceToPayLoan as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalanceToPayLoan(decoded));
            }
            if let Ok(decoded) = <NoCollateralToken as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoCollateralToken(decoded));
            }
            if let Ok(decoded) =
                <NoUserAccountQualifiedForLiquidation as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NoUserAccountQualifiedForLiquidation(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughDebtTokenToCoverLiquidation as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NotEnoughDebtTokenToCoverLiquidation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LIQUIDATE_USERErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientBalanceToPayLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoCollateralToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoUserAccountQualifiedForLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotEnoughDebtTokenToCoverLiquidation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LIQUIDATE_USERErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientBalanceToPayLoan as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoCollateralToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoUserAccountQualifiedForLiquidation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotEnoughDebtTokenToCoverLiquidation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LIQUIDATE_USERErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientBalanceToPayLoan(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoCollateralToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoUserAccountQualifiedForLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotEnoughDebtTokenToCoverLiquidation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LIQUIDATE_USERErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceToPayLoan> for LIQUIDATE_USERErrors {
        fn from(value: InsufficientBalanceToPayLoan) -> Self {
            Self::InsufficientBalanceToPayLoan(value)
        }
    }
    impl ::core::convert::From<NoCollateralToken> for LIQUIDATE_USERErrors {
        fn from(value: NoCollateralToken) -> Self {
            Self::NoCollateralToken(value)
        }
    }
    impl ::core::convert::From<NoUserAccountQualifiedForLiquidation> for LIQUIDATE_USERErrors {
        fn from(value: NoUserAccountQualifiedForLiquidation) -> Self {
            Self::NoUserAccountQualifiedForLiquidation(value)
        }
    }
    impl ::core::convert::From<NotEnoughDebtTokenToCoverLiquidation> for LIQUIDATE_USERErrors {
        fn from(value: NotEnoughDebtTokenToCoverLiquidation) -> Self {
            Self::NotEnoughDebtTokenToCoverLiquidation(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "LiquidateAccount",
        abi = "LiquidateAccount(address,address,address,uint256,address)"
    )]
    pub struct LiquidateAccountFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub beneficiary: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub collateral_token: ::ethers::core::types::Address,
        pub profit: ::ethers::core::types::U256,
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "ADDRESSES_PROVIDER", abi = "ADDRESSES_PROVIDER()")]
    pub struct AddressesProviderCall;
    ///Container type for all input parameters for the `POOL` function with signature `POOL()` and selector `0x7535d246`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "POOL", abi = "POOL()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `0x1b11d0ff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "executeOperation",
        abi = "executeOperation(address,uint256,uint256,address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub premium: ::ethers::core::types::U256,
        pub p3: ::ethers::core::types::Address,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `findAndLiquidateAccount` function with signature `findAndLiquidateAccount((address,address,address)[])` and selector `0xa255df16`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "findAndLiquidateAccount",
        abi = "findAndLiquidateAccount((address,address,address)[])"
    )]
    pub struct FindAndLiquidateAccountCall {
        pub users: ::std::vec::Vec<User>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LIQUIDATE_USERCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        FindAndLiquidateAccount(FindAndLiquidateAccountCall),
    }
    impl ::ethers::core::abi::AbiDecode for LIQUIDATE_USERCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AddressesProviderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressesProvider(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExecuteOperation(decoded));
            }
            if let Ok(decoded) =
                <FindAndLiquidateAccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindAndLiquidateAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LIQUIDATE_USERCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteOperation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindAndLiquidateAccount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LIQUIDATE_USERCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindAndLiquidateAccount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for LIQUIDATE_USERCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LIQUIDATE_USERCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for LIQUIDATE_USERCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<FindAndLiquidateAccountCall> for LIQUIDATE_USERCalls {
        fn from(value: FindAndLiquidateAccountCall) -> Self {
            Self::FindAndLiquidateAccount(value)
        }
    }
    ///Container type for all return fields from the `ADDRESSES_PROVIDER` function with signature `ADDRESSES_PROVIDER()` and selector `0x0542975c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddressesProviderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `POOL` function with signature `POOL()` and selector `0x7535d246`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `executeOperation` function with signature `executeOperation(address,uint256,uint256,address,bytes)` and selector `0x1b11d0ff`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExecuteOperationReturn(pub bool);
    ///`User(address,address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct User {
        pub id: ::ethers::core::types::Address,
        pub debt_token: ::ethers::core::types::Address,
        pub collateral_token: ::ethers::core::types::Address,
    }
}
