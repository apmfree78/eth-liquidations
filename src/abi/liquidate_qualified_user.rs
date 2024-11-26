pub use liquidate_qualified_user::*;
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
pub mod liquidate_qualified_user {
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
                        name: ::std::borrow::ToOwned::to_owned("wethAddress"),
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
                    ::std::borrow::ToOwned::to_owned("liquidateAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidateAccount"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LiquidateQualifiedUser.User",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
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
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATE_QUALIFIED_USER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15a\0\x10W_\x80\xFD[P`@Qa\x1A\xF18\x03\x80a\x1A\xF1\x839\x81\x01`@\x81\x90Ra\0/\x91a\0\x82V[_\x80T`\xFF\x19\x16\x90U`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x80R\x93\x85\x16`\xA0R\x91\x84\x16`\xC0R\x83\x16`\xE0R\x82\x16a\x01\0R\x16a\x01 Ra\0\xF2V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\0\x97W_\x80\xFD[a\0\xA0\x87a\0gV[\x95Pa\0\xAE` \x88\x01a\0gV[\x94Pa\0\xBC`@\x88\x01a\0gV[\x93Pa\0\xCA``\x88\x01a\0gV[\x92Pa\0\xD8`\x80\x88\x01a\0gV[\x91Pa\0\xE6`\xA0\x88\x01a\0gV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x19fa\x01\x8B_9_\x81\x81a\x04d\x01R\x81\x81a\x0EH\x01R\x81\x81a\x0F\x0C\x01Ra\x0FU\x01R_\x81\x81a\x01\x85\x01R\x81\x81a\n\x96\x01R\x81\x81a\n\xEC\x01R\x81\x81a\x0B1\x01R\x81\x81a\r\x11\x01Ra\r\xFF\x01R_\x81\x81a\x07\x11\x01Ra\nm\x01R_`P\x01R_\x81\x81a\x103\x01Ra\x11\xB0\x01R_\x81\x81`\xCA\x01R\x81\x81a\x02\xEC\x01Ra\x05I\x01Ra\x19f_\xF3\xFE`\x80`@R`\x046\x10a\0>W_5`\xE0\x1C\x80c\x05B\x97\\\x14a\0BW\x80c\x1B\x11\xD0\xFF\x14a\0\x8DW\x80cu5\xD2F\x14a\0\xBCW\x80c\xBB\xE8\xEA\xB3\x14a\0\xEEW[_\x80\xFD[4\x80\x15a\0MW_\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x98W_\x80\xFD[Pa\0\xACa\0\xA76`\x04a\x15oV[a\x01\x03V[`@Q\x90\x15\x15\x81R` \x01a\0\x84V[4\x80\x15a\0\xC7W_\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0pV[a\x01\x01a\0\xFC6`\x04a\x16\x14V[a\x02\x87V[\0[_\x80\x80a\x01\x12\x84\x86\x01\x86a\x16-V[`@\x80Q`\xC0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x82\x01\x92\x83R\x8D\x81\x16``\x83\x01R\x84\x16`\x80\x82\x01R`\xA0\x81\x01\x8C\x90R\x90\x81R` \x81\x01\x8B\x90R\x91\x93P\x91Pa\x01[\x81a\x05(V[\x82`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x8C\x16\x14a\x01\x83Wa\x01\x83\x84\x8Ca\x01~\x8C\x8Ea\x16xV[a\x06\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02bW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02$\x91\x90a\x16\x8BV[\x90P\x8B`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02VWa\x02I\x8A\x8Ca\x16xV[a\x02S\x90\x82a\x16\xA2V[\x90P[a\x02`\x85\x82a\njV[P[a\x02v\x84\x8Ca\x02q\x8C\x8Ea\x16xV[a\x0C\xF8V[P`\x01\x9A\x99PPPPPPPPPPV[_T`\xFF\x16\x15a\x02\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x81U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x03\x19` \x84\x01\x84a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x033WP``\x82\x015\x15\x15[\x80\x15a\x03WWP_a\x03K``\x84\x01`@\x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x03{WP_a\x03o`@\x84\x01` \x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x05\x02W_a\x03\x91``\x84\x01`@\x85\x01a\x16\xB5V[a\x03\x9E` \x85\x01\x85a\x16\xB5V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x82\x01R\x92\x90\x91\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16cB\xB0\xB7|0\x85` \x01` \x81\x01\x90a\x03\xF1\x91\x90a\x16\xB5V[\x86``\x015\x85_`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x18\x95\x94\x93\x92\x91\x90a\x16\xD0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04/W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04AW=_\x80>=_\xFD[Pa\x04V\x92PPP``\x84\x01`@\x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x7F\xAF\xBB\xB2\x9D\xE0\xC3\xCB\x07J\xB3\xAAL\xB9O C\xCE\xC2Y\xDAM\xD7[\0\x9F3\xF5fF\xFFP\x15a\x04\xB3` \x88\x01\x88a\x16\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA4`@QG\x90A\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\xFAW=_\x80>=_\xFD[PPPa\x05\x1BV[`@Qc<\x10\xB0\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_\x80T`\xFF\x19\x16\x90UV[\x80Q` \x90\x81\x01Q\x90\x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x16\x8BV[\x10a\x06\xDAW\x80`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x83\x85` \x01Q`\x03a\x05\xE4\x91\x90a\x175V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06,W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06P\x91\x90a\x17LV[P\x82Q`@\x80\x82\x01Q` \x80\x84\x01Q\x93Q\x90\x87\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x93\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R`d\x83\x01\x91\x90\x91R_`\x84\x83\x01R\x83\x16\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xD1W=_\x80>=_\xFD[PPPPPPPV[`@Qc\x03\xDCq\xB7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x84\x91\x90a\x16\x8BV[_\x03a\x07\xA3W`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x16\x8BV[\x90P_a\x08\x19\x88\x88\x88a\x10\x12V[\x90P\x81\x81\x11\x15a\x08&WP\x80[`@\x80Qa\x01\0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x82R\x89\x16` \x82\x01Ra\x0B\xB8\x91\x81\x01\x91\x90\x91R0``\x82\x01R_\x90`\x80\x81\x01a\x08gBa\x01,a\x16xV[\x81R` \x81\x01\x89\x90R`@\x80\x82\x01\x85\x90R_``\x90\x92\x01\x91\x90\x91RQcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`$\x83\x01R\x91\x92P\x83\x91\x87\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF3\x91\x90a\x16\x8BV[\x10\x15a\tkW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ti\x91\x90a\x17LV[P[`@Qc\x1Bg\xC43`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xDB>!\x98\x90a\t\x97\x90\x84\x90`\x04\x01a\x17\xDBV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xB3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD7\x91\x90a\x16\x8BV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x87\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n@\x91\x90a\x16\x8BV[\x10\x15a\n_W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x83\x16\x03a\n\xC5WPPPPV[\x82_\x03a\n\xE5W`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B\x11\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x11\x8FV[\x90P_`@Q\x80a\x01\0\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B\xB8b\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01Ba\x01,a\x0B\x88\x91\x90a\x16xV[\x81R` \x81\x01\x87\x90R`@\x80\x82\x01\x85\x90R_``\x90\x92\x01\x91\x90\x91RQcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x91\x92P\x86\x91\x86\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a\x16\x8BV[\x10\x15a\x0C\x8CW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R\x85\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0CfW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8A\x91\x90a\x17LV[P[`@QcAK\xF3\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cAK\xF3\x89\x90a\x0C\xB8\x90\x84\x90`\x04\x01a\x17\xDBV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90a\x16\x8BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90_\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rbW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x86\x91\x90a\x16\x8BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF1\x91\x90a\x16\x8BV[\x90P_\x81\x11\x80\x15a\x0E4WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0EmWa\x0Em`\x01`\x01`\xA0\x1B\x03\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD5\x91\x90a\x16\x8BV[\x90P_\x81\x11\x80\x15a\x0E\xF8WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0F1Wa\x0F1`\x01`\x01`\xA0\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[\x86\x83\x11\x15a\n_W_a\x0FD\x88\x85a\x16\xA2V[\x90Pa\x0Fz`\x01`\x01`\xA0\x1B\x03\x88\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE2\x91\x90a\x16\x8BV[\x90P\x88\x81\x10\x15a\x10\x05W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA1\x91\x90a\x16\x8BV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x92P_\x91\x84\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xEAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x0E\x91\x90a\x16\x8BV[\x90P_a\x11\x1A\x88a\x13MV[\x90P_a\x11&\x88a\x13MV[\x90P_\x84a\x114\x85\x8Aa\x175V[a\x11>\x91\x90a\x17\xEAV[\x90P\x81a\x11K\x84\x83a\x175V[a\x11U\x91\x90a\x17\xEAV[\x90P_b\x0FB@a\x11h\x83aN a\x175V[a\x11r\x91\x90a\x17\xEAV[\x90Pa\x11~\x81\x83a\x16xV[\x97PPPPPPPP[\x93\x92PPPV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xFAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1E\x91\x90a\x16\x8BV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x92P_\x91\x84\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12gW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x8B\x91\x90a\x16\x8BV[\x90P_a\x12\x97\x88a\x13MV[\x90P_a\x12\xA3\x88a\x13MV[\x90P_\x83a\x12\xB1\x86\x8Aa\x175V[a\x12\xBB\x91\x90a\x17\xEAV[\x90P\x82a\x12\xC8\x83\x83a\x175V[a\x12\xD2\x91\x90a\x17\xEAV[\x90P_b\x0FB@a\x12\xE5\x83aN a\x175V[a\x12\xEF\x91\x90a\x17\xEAV[\x90Pa\x11~\x81\x83a\x16\xA2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x06\xF3\x90\x84\x90a\x13\xC3V[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x13\xA9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x13\xA6\x91\x81\x01\x90a\x18\tV[`\x01[a\x13\xB5WP`\x12a\x13\xB8V[\x90P[a\x11\x88\x81`\na\x19\x0CV[_a\x13\xD7`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x14$V[\x90P\x80Q_\x14\x15\x80\x15a\x13\xFBWP\x80\x80` \x01\x90Q\x81\x01\x90a\x13\xF9\x91\x90a\x17LV[\x15[\x15a\x06\xF3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x02\xD5V[``a\x141\x83\x83_a\x14:V[\x90P[\x92\x91PPV[``\x81G\x10\x15a\x14_W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x02\xD5V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x14z\x91\x90a\x19\x1AV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x14\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\xB9V[``\x91P[P\x91P\x91Pa\x14\xC9\x86\x83\x83a\x14\xD3V[\x96\x95PPPPPPV[``\x82a\x14\xE8Wa\x14\xE3\x82a\x15/V[a\x11\x88V[\x81Q\x15\x80\x15a\x14\xFFWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x15(W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xD5V[P\x80a\x11\x88V[\x80Q\x15a\x15?W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15XW_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a\x15\x84W_\x80\xFD[\x865a\x15\x8F\x81a\x15[V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a\x15\xAD\x81a\x15[V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xC8W_\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x15\xD8W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xEEW_\x80\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x15\xFFW_\x80\xFD[` \x82\x01\x93P\x80\x92PPP\x92\x95P\x92\x95P\x92\x95V[_`\x80\x82\x84\x03\x12\x80\x15a\x16%W_\x80\xFD[P\x90\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x16>W_\x80\xFD[\x825a\x16I\x81a\x15[V[\x91P` \x83\x015a\x16Y\x81a\x15[V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x16\x9BW_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x16\xC5W_\x80\xFD[\x815a\x11\x88\x81a\x15[V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R`\x01\x80`\xA0\x1B\x03\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R_\x83Q\x80`\xA0\x84\x01R\x80` \x86\x01`\xC0\x85\x01^_`\xC0\x82\x85\x01\x01R`\xC0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\xFF\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x17\\W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\x88W_\x80\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Qb\xFF\xFF\xFF\x16\x90\x84\x01R``\x80\x83\x01Q\x91\x82\x16\x90\x84\x01RP`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Qa\x06\xF3`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a\x01\0\x81\x01a\x144\x82\x84a\x17kV[_\x82a\x18\x04WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x18\x19W_\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x11\x88W_\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\x18dW\x80\x85\x04\x81\x11\x15a\x18HWa\x18Ha\x16dV[`\x01\x84\x16\x15a\x18VW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x18-V[\x93P\x93\x91PPV[_\x82a\x18zWP`\x01a\x144V[\x81a\x18\x86WP_a\x144V[\x81`\x01\x81\x14a\x18\x9CW`\x02\x81\x14a\x18\xA6Wa\x18\xC2V[`\x01\x91PPa\x144V[`\xFF\x84\x11\x15a\x18\xB7Wa\x18\xB7a\x16dV[PP`\x01\x82\x1Ba\x144V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x18\xE5WP\x81\x81\na\x144V[a\x18\xF1_\x19\x84\x84a\x18)V[\x80_\x19\x04\x82\x11\x15a\x19\x04Wa\x19\x04a\x16dV[\x02\x93\x92PPPV[_a\x141`\xFF\x84\x16\x83a\x18lV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 QT\xC1\xEF@N{\x9A%\x90\xF28\x12\x1A\xE6X|i\xC0s&\x91a\x92\x9D\xFEN\x81\xD69\xFF\xCDdsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATE_QUALIFIED_USER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0>W_5`\xE0\x1C\x80c\x05B\x97\\\x14a\0BW\x80c\x1B\x11\xD0\xFF\x14a\0\x8DW\x80cu5\xD2F\x14a\0\xBCW\x80c\xBB\xE8\xEA\xB3\x14a\0\xEEW[_\x80\xFD[4\x80\x15a\0MW_\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\x98W_\x80\xFD[Pa\0\xACa\0\xA76`\x04a\x15oV[a\x01\x03V[`@Q\x90\x15\x15\x81R` \x01a\0\x84V[4\x80\x15a\0\xC7W_\x80\xFD[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\0pV[a\x01\x01a\0\xFC6`\x04a\x16\x14V[a\x02\x87V[\0[_\x80\x80a\x01\x12\x84\x86\x01\x86a\x16-V[`@\x80Q`\xC0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x82\x01\x92\x83R\x8D\x81\x16``\x83\x01R\x84\x16`\x80\x82\x01R`\xA0\x81\x01\x8C\x90R\x90\x81R` \x81\x01\x8B\x90R\x91\x93P\x91Pa\x01[\x81a\x05(V[\x82`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x90\x8C\x16\x14a\x01\x83Wa\x01\x83\x84\x8Ca\x01~\x8C\x8Ea\x16xV[a\x06\xF8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14a\x02bW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02$\x91\x90a\x16\x8BV[\x90P\x8B`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02VWa\x02I\x8A\x8Ca\x16xV[a\x02S\x90\x82a\x16\xA2V[\x90P[a\x02`\x85\x82a\njV[P[a\x02v\x84\x8Ca\x02q\x8C\x8Ea\x16xV[a\x0C\xF8V[P`\x01\x9A\x99PPPPPPPPPPV[_T`\xFF\x16\x15a\x02\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x81U\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x03\x19` \x84\x01\x84a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x033WP``\x82\x015\x15\x15[\x80\x15a\x03WWP_a\x03K``\x84\x01`@\x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x03{WP_a\x03o`@\x84\x01` \x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x05\x02W_a\x03\x91``\x84\x01`@\x85\x01a\x16\xB5V[a\x03\x9E` \x85\x01\x85a\x16\xB5V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x82\x01R\x92\x90\x91\x16\x90\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x81`\x01`\x01`\xA0\x1B\x03\x16cB\xB0\xB7|0\x85` \x01` \x81\x01\x90a\x03\xF1\x91\x90a\x16\xB5V[\x86``\x015\x85_`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x18\x95\x94\x93\x92\x91\x90a\x16\xD0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04/W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04AW=_\x80>=_\xFD[Pa\x04V\x92PPP``\x84\x01`@\x85\x01a\x16\xB5V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x7F\xAF\xBB\xB2\x9D\xE0\xC3\xCB\x07J\xB3\xAAL\xB9O C\xCE\xC2Y\xDAM\xD7[\0\x9F3\xF5fF\xFFP\x15a\x04\xB3` \x88\x01\x88a\x16\xB5V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA4`@QG\x90A\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x04\xFAW=_\x80>=_\xFD[PPPa\x05\x1BV[`@Qc<\x10\xB0\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP_\x80T`\xFF\x19\x16\x90UV[\x80Q` \x90\x81\x01Q\x90\x82\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xBE\x91\x90a\x16\x8BV[\x10a\x06\xDAW\x80`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x83\x85` \x01Q`\x03a\x05\xE4\x91\x90a\x175V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06,W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06P\x91\x90a\x17LV[P\x82Q`@\x80\x82\x01Q` \x80\x84\x01Q\x93Q\x90\x87\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x93\x82\x16`$\x85\x01R\x81\x16`D\x84\x01R`d\x83\x01\x91\x90\x91R_`\x84\x83\x01R\x83\x16\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xBFW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xD1W=_\x80>=_\xFD[PPPPPPPV[`@Qc\x03\xDCq\xB7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x84\x91\x90a\x16\x8BV[_\x03a\x07\xA3W`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x0B\x91\x90a\x16\x8BV[\x90P_a\x08\x19\x88\x88\x88a\x10\x12V[\x90P\x81\x81\x11\x15a\x08&WP\x80[`@\x80Qa\x01\0\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x82R\x89\x16` \x82\x01Ra\x0B\xB8\x91\x81\x01\x91\x90\x91R0``\x82\x01R_\x90`\x80\x81\x01a\x08gBa\x01,a\x16xV[\x81R` \x81\x01\x89\x90R`@\x80\x82\x01\x85\x90R_``\x90\x92\x01\x91\x90\x91RQcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`$\x83\x01R\x91\x92P\x83\x91\x87\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF3\x91\x90a\x16\x8BV[\x10\x15a\tkW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\tEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ti\x91\x90a\x17LV[P[`@Qc\x1Bg\xC43`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xDB>!\x98\x90a\t\x97\x90\x84\x90`\x04\x01a\x17\xDBV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xB3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD7\x91\x90a\x16\x8BV[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x87\x90`\x01`\x01`\xA0\x1B\x03\x88\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n@\x91\x90a\x16\x8BV[\x10\x15a\n_W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPV[\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x83\x16\x03a\n\xC5WPPPPV[\x82_\x03a\n\xE5W`@Qch~)k`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B\x11\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x11\x8FV[\x90P_`@Q\x80a\x01\0\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x0B\xB8b\xFF\xFF\xFF\x16\x81R` \x010`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01Ba\x01,a\x0B\x88\x91\x90a\x16xV[\x81R` \x81\x01\x87\x90R`@\x80\x82\x01\x85\x90R_``\x90\x92\x01\x91\x90\x91RQcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`$\x83\x01R\x91\x92P\x86\x91\x86\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a\x16\x8BV[\x10\x15a\x0C\x8CW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R\x85\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0CfW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x8A\x91\x90a\x17LV[P[`@QcAK\xF3\x89`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cAK\xF3\x89\x90a\x0C\xB8\x90\x84\x90`\x04\x01a\x17\xDBV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD1\x91\x90a\x16\x8BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x90\x84\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90_\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rbW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x86\x91\x90a\x16\x8BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF1\x91\x90a\x16\x8BV[\x90P_\x81\x11\x80\x15a\x0E4WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0EmWa\x0Em`\x01`\x01`\xA0\x1B\x03\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD5\x91\x90a\x16\x8BV[\x90P_\x81\x11\x80\x15a\x0E\xF8WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x89`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0F1Wa\x0F1`\x01`\x01`\xA0\x1B\x03\x86\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[\x86\x83\x11\x15a\n_W_a\x0FD\x88\x85a\x16\xA2V[\x90Pa\x0Fz`\x01`\x01`\xA0\x1B\x03\x88\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x12\xFBV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x89\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xE2\x91\x90a\x16\x8BV[\x90P\x88\x81\x10\x15a\x10\x05W`@Qc\x14U\xDE\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xA1\x91\x90a\x16\x8BV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x92P_\x91\x84\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xEAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x0E\x91\x90a\x16\x8BV[\x90P_a\x11\x1A\x88a\x13MV[\x90P_a\x11&\x88a\x13MV[\x90P_\x84a\x114\x85\x8Aa\x175V[a\x11>\x91\x90a\x17\xEAV[\x90P\x81a\x11K\x84\x83a\x175V[a\x11U\x91\x90a\x17\xEAV[\x90P_b\x0FB@a\x11h\x83aN a\x175V[a\x11r\x91\x90a\x17\xEAV[\x90Pa\x11~\x81\x83a\x16xV[\x97PPPPPPPP[\x93\x92PPPV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xFAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x1E\x91\x90a\x16\x8BV[`@Qc\xB3Yo\x07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x91\x92P_\x91\x84\x16\x90c\xB3Yo\x07\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12gW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x8B\x91\x90a\x16\x8BV[\x90P_a\x12\x97\x88a\x13MV[\x90P_a\x12\xA3\x88a\x13MV[\x90P_\x83a\x12\xB1\x86\x8Aa\x175V[a\x12\xBB\x91\x90a\x17\xEAV[\x90P\x82a\x12\xC8\x83\x83a\x175V[a\x12\xD2\x91\x90a\x17\xEAV[\x90P_b\x0FB@a\x12\xE5\x83aN a\x175V[a\x12\xEF\x91\x90a\x17\xEAV[\x90Pa\x11~\x81\x83a\x16\xA2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x06\xF3\x90\x84\x90a\x13\xC3V[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x13\xA9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x13\xA6\x91\x81\x01\x90a\x18\tV[`\x01[a\x13\xB5WP`\x12a\x13\xB8V[\x90P[a\x11\x88\x81`\na\x19\x0CV[_a\x13\xD7`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x14$V[\x90P\x80Q_\x14\x15\x80\x15a\x13\xFBWP\x80\x80` \x01\x90Q\x81\x01\x90a\x13\xF9\x91\x90a\x17LV[\x15[\x15a\x06\xF3W`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x02\xD5V[``a\x141\x83\x83_a\x14:V[\x90P[\x92\x91PPV[``\x81G\x10\x15a\x14_W`@Qc\xCDx`Y`\xE0\x1B\x81R0`\x04\x82\x01R`$\x01a\x02\xD5V[_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x14z\x91\x90a\x19\x1AV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x14\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\xB9V[``\x91P[P\x91P\x91Pa\x14\xC9\x86\x83\x83a\x14\xD3V[\x96\x95PPPPPPV[``\x82a\x14\xE8Wa\x14\xE3\x82a\x15/V[a\x11\x88V[\x81Q\x15\x80\x15a\x14\xFFWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x15(W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xD5V[P\x80a\x11\x88V[\x80Q\x15a\x15?W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15XW_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a\x15\x84W_\x80\xFD[\x865a\x15\x8F\x81a\x15[V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015a\x15\xAD\x81a\x15[V[\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xC8W_\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\x15\xD8W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xEEW_\x80\xFD[\x89` \x82\x84\x01\x01\x11\x15a\x15\xFFW_\x80\xFD[` \x82\x01\x93P\x80\x92PPP\x92\x95P\x92\x95P\x92\x95V[_`\x80\x82\x84\x03\x12\x80\x15a\x16%W_\x80\xFD[P\x90\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\x16>W_\x80\xFD[\x825a\x16I\x81a\x15[V[\x91P` \x83\x015a\x16Y\x81a\x15[V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x16\x9BW_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x16\xC5W_\x80\xFD[\x815a\x11\x88\x81a\x15[V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R`\x01\x80`\xA0\x1B\x03\x85\x16` \x82\x01R\x83`@\x82\x01R`\xA0``\x82\x01R_\x83Q\x80`\xA0\x84\x01R\x80` \x86\x01`\xC0\x85\x01^_`\xC0\x82\x85\x01\x01R`\xC0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\xFF\xFF\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x144Wa\x144a\x16dV[_` \x82\x84\x03\x12\x15a\x17\\W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x11\x88W_\x80\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Qb\xFF\xFF\xFF\x16\x90\x84\x01R``\x80\x83\x01Q\x91\x82\x16\x90\x84\x01RP`\x80\x81\x01Q`\x80\x83\x01R`\xA0\x81\x01Q`\xA0\x83\x01R`\xC0\x81\x01Q`\xC0\x83\x01R`\xE0\x81\x01Qa\x06\xF3`\xE0\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[a\x01\0\x81\x01a\x144\x82\x84a\x17kV[_\x82a\x18\x04WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a\x18\x19W_\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x11\x88W_\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\x18dW\x80\x85\x04\x81\x11\x15a\x18HWa\x18Ha\x16dV[`\x01\x84\x16\x15a\x18VW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x18-V[\x93P\x93\x91PPV[_\x82a\x18zWP`\x01a\x144V[\x81a\x18\x86WP_a\x144V[\x81`\x01\x81\x14a\x18\x9CW`\x02\x81\x14a\x18\xA6Wa\x18\xC2V[`\x01\x91PPa\x144V[`\xFF\x84\x11\x15a\x18\xB7Wa\x18\xB7a\x16dV[PP`\x01\x82\x1Ba\x144V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x18\xE5WP\x81\x81\na\x144V[a\x18\xF1_\x19\x84\x84a\x18)V[\x80_\x19\x04\x82\x11\x15a\x19\x04Wa\x19\x04a\x16dV[\x02\x93\x92PPPV[_a\x141`\xFF\x84\x16\x83a\x18lV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 QT\xC1\xEF@N{\x9A%\x90\xF28\x12\x1A\xE6X|i\xC0s&\x91a\x92\x9D\xFEN\x81\xD69\xFF\xCDdsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATE_QUALIFIED_USER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LIQUIDATE_QUALIFIED_USER<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LIQUIDATE_QUALIFIED_USER<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LIQUIDATE_QUALIFIED_USER<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LIQUIDATE_QUALIFIED_USER<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LIQUIDATE_QUALIFIED_USER<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LIQUIDATE_QUALIFIED_USER))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LIQUIDATE_QUALIFIED_USER<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIQUIDATE_QUALIFIED_USER_ABI.clone(),
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
                LIQUIDATE_QUALIFIED_USER_ABI.clone(),
                LIQUIDATE_QUALIFIED_USER_BYTECODE.clone().into(),
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
        ///Calls the contract's `liquidateAccount` (0xbbe8eab3) function
        pub fn liquidate_account(
            &self,
            user: User,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([187, 232, 234, 179], (user,))
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
        for LIQUIDATE_QUALIFIED_USER<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
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
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
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
        name = "AddressInsufficientBalance",
        abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
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
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
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
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
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
        name = "SafeERC20FailedOperation",
        abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LIQUIDATE_QUALIFIED_USERErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        InsufficientBalanceToPayLoan(InsufficientBalanceToPayLoan),
        NoCollateralToken(NoCollateralToken),
        NoUserAccountQualifiedForLiquidation(NoUserAccountQualifiedForLiquidation),
        NotEnoughDebtTokenToCoverLiquidation(NotEnoughDebtTokenToCoverLiquidation),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LIQUIDATE_QUALIFIED_USERErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) =
                <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
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
            if let Ok(decoded) =
                <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LIQUIDATE_QUALIFIED_USERErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LIQUIDATE_QUALIFIED_USERErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LIQUIDATE_QUALIFIED_USERErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceToPayLoan> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: InsufficientBalanceToPayLoan) -> Self {
            Self::InsufficientBalanceToPayLoan(value)
        }
    }
    impl ::core::convert::From<NoCollateralToken> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: NoCollateralToken) -> Self {
            Self::NoCollateralToken(value)
        }
    }
    impl ::core::convert::From<NoUserAccountQualifiedForLiquidation>
        for LIQUIDATE_QUALIFIED_USERErrors
    {
        fn from(value: NoUserAccountQualifiedForLiquidation) -> Self {
            Self::NoUserAccountQualifiedForLiquidation(value)
        }
    }
    impl ::core::convert::From<NotEnoughDebtTokenToCoverLiquidation>
        for LIQUIDATE_QUALIFIED_USERErrors
    {
        fn from(value: NotEnoughDebtTokenToCoverLiquidation) -> Self {
            Self::NotEnoughDebtTokenToCoverLiquidation(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for LIQUIDATE_QUALIFIED_USERErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
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
        abi = "LiquidateAccount(address,address,address,address)"
    )]
    pub struct LiquidateAccountFilter {
        #[ethevent(indexed)]
        pub liquidator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub beneficiary: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub collateral_token: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `liquidateAccount` function with signature `liquidateAccount((address,address,address,uint256))` and selector `0xbbe8eab3`
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
        name = "liquidateAccount",
        abi = "liquidateAccount((address,address,address,uint256))"
    )]
    pub struct LiquidateAccountCall {
        pub user: User,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LIQUIDATE_QUALIFIED_USERCalls {
        AddressesProvider(AddressesProviderCall),
        Pool(PoolCall),
        ExecuteOperation(ExecuteOperationCall),
        LiquidateAccount(LiquidateAccountCall),
    }
    impl ::ethers::core::abi::AbiDecode for LIQUIDATE_QUALIFIED_USERCalls {
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
                <LiquidateAccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidateAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LIQUIDATE_QUALIFIED_USERCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddressesProvider(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteOperation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidateAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LIQUIDATE_QUALIFIED_USERCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressesProvider(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteOperation(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidateAccount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddressesProviderCall> for LIQUIDATE_QUALIFIED_USERCalls {
        fn from(value: AddressesProviderCall) -> Self {
            Self::AddressesProvider(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LIQUIDATE_QUALIFIED_USERCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<ExecuteOperationCall> for LIQUIDATE_QUALIFIED_USERCalls {
        fn from(value: ExecuteOperationCall) -> Self {
            Self::ExecuteOperation(value)
        }
    }
    impl ::core::convert::From<LiquidateAccountCall> for LIQUIDATE_QUALIFIED_USERCalls {
        fn from(value: LiquidateAccountCall) -> Self {
            Self::LiquidateAccount(value)
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
    ///`User(address,address,address,uint256)`
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
        pub debt_to_cover: ::ethers::core::types::U256,
    }
}
