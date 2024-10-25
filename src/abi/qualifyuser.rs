pub use qualify_user::*;
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
pub mod qualify_user {
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
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkUserAccounts"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkUserAccounts"),
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
                                ::std::borrow::ToOwned::to_owned("struct QualifyUser.User[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("topProfitAccount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("topProfitAccount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("debtToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("collateralToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("profit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("NoUserAccountQualifiedForLiquidation"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("NoUserAccountQualifiedForLiquidation",),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static QUALIFY_USER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x0C\x868\x03\x80a\x0C\x86\x839\x81\x01`@\x81\x90Ra\0.\x91a\0fV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x16`\xC0Ra\0\xA6V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0aW_\x80\xFD[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a\0xW_\x80\xFD[a\0\x81\x84a\0KV[\x92Pa\0\x8F` \x85\x01a\0KV[\x91Pa\0\x9D`@\x85\x01a\0KV[\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x0B\xA9a\0\xDD_9_a\x03E\x01R_\x81\x81a\x03$\x01R\x81\x81a\x05\xF2\x01Ra\x06\xA6\x01R_`\xDB\x01Ra\x0B\xA9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cQt\x0E'\x14a\08W\x80c~\xB4\xBE\x89\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x07\xA4V[a\0\xAEV[\0[_T`\x01T`\x02T`\x03T`\x04Ta\0t\x94`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x81\x16\x93\x16\x91\x90\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x86\x16` \x86\x01R\x92\x90\x94\x16\x83\x83\x01R``\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90Q\x90\x81\x90\x03`\xA0\x01\x90\xF3[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91_[\x84\x81\x10\x15a\x02\xC2W_\x87\x87\x83\x81\x81\x10a\x01\x18Wa\x01\x18a\x08\x15V[a\x01.\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08)V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P_\x91\x87\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x9B\x91\x90a\x08OV[\x95PPPPPPg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x80\x15a\x01\xC0WPg\x01cEx]\x8A\0\0\x81\x11[\x15a\x02\xB8W_\x80a\x01\xE8\x8B\x8B\x87\x81\x81\x10a\x01\xDCWa\x01\xDCa\x08\x15V[\x90P``\x02\x01\x84a\x03 V[\x91P\x91P\x86\x82\x11\x15a\x02\xB5W\x81\x96P`@Q\x80`\xA0\x01`@R\x80\x8C\x8C\x88\x81\x81\x10a\x02\x14Wa\x02\x14a\x08\x15V[a\x02*\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x8C\x88\x81\x81\x10a\x02JWa\x02Ja\x08\x15V[\x90P``\x02\x01` \x01` \x81\x01\x90a\x02b\x91\x90a\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x8C\x88\x81\x81\x10a\x02\x82Wa\x02\x82a\x08\x15V[\x90P``\x02\x01`@\x01` \x81\x01\x90a\x02\x9A\x91\x90a\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x83\x81RP\x95P[PP[PP`\x01\x01a\0\xFDV[P\x80Q_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U` \x83\x01Q`\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x83\x01Q`\x02\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x81\x01Q`\x03U`\x80\x01Q`\x04UPPPPPV[_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x03\x8Aa\x03x`@\x89\x01` \x8A\x01a\x08)V[a\x03\x85` \x8A\x01\x8Aa\x08)V[a\x05\xC9V[\x90P_`\x01`\x01`\xA0\x1B\x03\x84\x16c(\xDD-\x01a\x03\xAC``\x8B\x01`@\x8C\x01a\x08)V[a\x03\xB9` \x8C\x01\x8Ca\x08)V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04'\x91\x90a\x08\xA9V[\x98PPPPPPPPP_g\x06\xF0[Y\xD3\xB2\0\0\x90Pg\r/\x13\xF7x\x9F\0\0\x88\x10\x15a\x04XWPg\r\xE0\xB6\xB3\xA7d\0\0[_a\x04qa\x04l``\x8C\x01`@\x8D\x01a\x08)V[a\x06\x85V[\x90P_`\x01`\x01`\xA0\x1B\x03\x86\x16c\xB3Yo\x07a\x04\x93`@\x8E\x01` \x8F\x01a\x08)V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF9\x91\x90a\tHV[\x90P_a\x05\x14a\x05\x0F`@\x8E\x01` \x8F\x01a\x08)V[a\x07'V[\x90P_a\x05!\x85\x88a\tsV[\x90Pa\x055g\r\xE0\xB6\xB3\xA7d\0\0\x82a\t\x8AV[\x90P_\x84\x11\x80\x15a\x05CWP\x85[\x15a\x05\xB2W_\x82a\x05T\x85\x84a\tsV[a\x05^\x91\x90a\t\x8AV[\x90Pa\x05j\x85\x82a\tsV[\x90Pa\x05xa'\x10\x86a\t\xA9V[a\x05\x82\x90\x82a\tsV[\x90Pa\x05\x90a'\x10\x82a\t\x8AV[\x90Pa\x05\x9Ea'\x10\x82a\t\x8AV[\x9BP\x90\x99Pa\x05\xC2\x98PPPPPPPPPV[_\x80\x9AP\x9APPPPPPPPPP[\x92P\x92\x90PV[`@Qc(\xDD-\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x82\x91\x84\x16\x90c(\xDD-\x01\x90`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06>W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06b\x91\x90a\x08\xA9V[PPPPPP\x92P\x92PP\x80\x82a\x06y\x91\x90a\t\xBCV[\x93PPPP[\x92\x91PPV[`@Qc>\x15\x01A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c>\x15\x01A\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a\t\xCFV[P\x94\x9C\x9BPPPPPPPPPPPPV[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x07\x83WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07\x80\x91\x81\x01\x90a\nbV[`\x01[a\x07\x8FWP`\x12a\x07\x92V[\x90P[a\x07\x9D\x81`\na\x0BeV[\x93\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x07\xB5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xCBW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x07\xDBW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF1W_\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x08\x05W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x089W_\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x9DW_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x08dW_\x80\xFD[PP\x84Q` \x86\x01Q`@\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x90\x99\x01Q\x93\x9A\x92\x99P\x90\x97\x90\x96P\x94P\x90\x92P\x90PV[\x80Q\x80\x15\x15\x81\x14a\x08\xA4W_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15a\x08\xC2W_\x80\xFD[_\x8AQ\x90P\x80\x99PP_` \x8B\x01Q\x90P\x80\x98PP_`@\x8B\x01Q\x90P\x80\x97PP_``\x8B\x01Q\x90P\x80\x96PP_`\x80\x8B\x01Q\x90P\x80\x95PP_`\xA0\x8B\x01Q\x90P\x80\x94PP_`\xC0\x8B\x01Q\x90P\x80\x93PP`\xE0\x8A\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t*W_\x80\xFD[\x91Pa\t9a\x01\0\x8B\x01a\x08\x95V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_` \x82\x84\x03\x12\x15a\tXW_\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x7FWa\x06\x7Fa\t_V[_\x82a\t\xA4WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x06\x7FWa\x06\x7Fa\t_V[\x80\x82\x01\x80\x82\x11\x15a\x06\x7FWa\x06\x7Fa\t_V[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\t\xE9W_\x80\xFD[\x8AQ` \x8C\x01Q`@\x8D\x01Q``\x8E\x01Q`\x80\x8F\x01Q\x93\x9DP\x91\x9BP\x99P\x97P\x95Pa\n\x17`\xA0\x8C\x01a\x08\x95V[\x94Pa\n%`\xC0\x8C\x01a\x08\x95V[\x93Pa\n3`\xE0\x8C\x01a\x08\x95V[\x92Pa\nBa\x01\0\x8C\x01a\x08\x95V[\x91Pa\nQa\x01 \x8C\x01a\x08\x95V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[_` \x82\x84\x03\x12\x15a\nrW_\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07\x9DW_\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\xBDW\x80\x85\x04\x81\x11\x15a\n\xA1Wa\n\xA1a\t_V[`\x01\x84\x16\x15a\n\xAFW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\n\x86V[\x93P\x93\x91PPV[_\x82a\n\xD3WP`\x01a\x06\x7FV[\x81a\n\xDFWP_a\x06\x7FV[\x81`\x01\x81\x14a\n\xF5W`\x02\x81\x14a\n\xFFWa\x0B\x1BV[`\x01\x91PPa\x06\x7FV[`\xFF\x84\x11\x15a\x0B\x10Wa\x0B\x10a\t_V[PP`\x01\x82\x1Ba\x06\x7FV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B>WP\x81\x81\na\x06\x7FV[a\x0BJ_\x19\x84\x84a\n\x82V[\x80_\x19\x04\x82\x11\x15a\x0B]Wa\x0B]a\t_V[\x02\x93\x92PPPV[_a\x07\x9D`\xFF\x84\x16\x83a\n\xC5V\xFE\xA2dipfsX\"\x12 \x15\xBB\xE4\x81\xCF\xFEU}\n3\xB3\x11\xFA\x95\x05\xBA\x9C>GH\x95\xBB\x9A\x890\x17X\xFE\x0B\x89\xF9\xA1dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static QUALIFY_USER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cQt\x0E'\x14a\08W\x80c~\xB4\xBE\x89\x14a\0MW[_\x80\xFD[a\0Ka\0F6`\x04a\x07\xA4V[a\0\xAEV[\0[_T`\x01T`\x02T`\x03T`\x04Ta\0t\x94`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x94\x81\x16\x93\x16\x91\x90\x85V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x86\x16` \x86\x01R\x92\x90\x94\x16\x83\x83\x01R``\x83\x01R`\x80\x82\x01\x92\x90\x92R\x90Q\x90\x81\x90\x03`\xA0\x01\x90\xF3[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91_[\x84\x81\x10\x15a\x02\xC2W_\x87\x87\x83\x81\x81\x10a\x01\x18Wa\x01\x18a\x08\x15V[a\x01.\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08)V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P_\x91\x87\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x9B\x91\x90a\x08OV[\x95PPPPPPg\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x80\x15a\x01\xC0WPg\x01cEx]\x8A\0\0\x81\x11[\x15a\x02\xB8W_\x80a\x01\xE8\x8B\x8B\x87\x81\x81\x10a\x01\xDCWa\x01\xDCa\x08\x15V[\x90P``\x02\x01\x84a\x03 V[\x91P\x91P\x86\x82\x11\x15a\x02\xB5W\x81\x96P`@Q\x80`\xA0\x01`@R\x80\x8C\x8C\x88\x81\x81\x10a\x02\x14Wa\x02\x14a\x08\x15V[a\x02*\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x8C\x88\x81\x81\x10a\x02JWa\x02Ja\x08\x15V[\x90P``\x02\x01` \x01` \x81\x01\x90a\x02b\x91\x90a\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x8C\x88\x81\x81\x10a\x02\x82Wa\x02\x82a\x08\x15V[\x90P``\x02\x01`@\x01` \x81\x01\x90a\x02\x9A\x91\x90a\x08)V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x82\x81R` \x01\x83\x81RP\x95P[PP[PP`\x01\x01a\0\xFDV[P\x80Q_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U` \x83\x01Q`\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x83\x01Q`\x02\x80T\x91\x90\x93\x16\x91\x16\x17\x90U``\x81\x01Q`\x03U`\x80\x01Q`\x04UPPPPPV[_\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x03\x8Aa\x03x`@\x89\x01` \x8A\x01a\x08)V[a\x03\x85` \x8A\x01\x8Aa\x08)V[a\x05\xC9V[\x90P_`\x01`\x01`\xA0\x1B\x03\x84\x16c(\xDD-\x01a\x03\xAC``\x8B\x01`@\x8C\x01a\x08)V[a\x03\xB9` \x8C\x01\x8Ca\x08)V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04'\x91\x90a\x08\xA9V[\x98PPPPPPPPP_g\x06\xF0[Y\xD3\xB2\0\0\x90Pg\r/\x13\xF7x\x9F\0\0\x88\x10\x15a\x04XWPg\r\xE0\xB6\xB3\xA7d\0\0[_a\x04qa\x04l``\x8C\x01`@\x8D\x01a\x08)V[a\x06\x85V[\x90P_`\x01`\x01`\xA0\x1B\x03\x86\x16c\xB3Yo\x07a\x04\x93`@\x8E\x01` \x8F\x01a\x08)V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xF9\x91\x90a\tHV[\x90P_a\x05\x14a\x05\x0F`@\x8E\x01` \x8F\x01a\x08)V[a\x07'V[\x90P_a\x05!\x85\x88a\tsV[\x90Pa\x055g\r\xE0\xB6\xB3\xA7d\0\0\x82a\t\x8AV[\x90P_\x84\x11\x80\x15a\x05CWP\x85[\x15a\x05\xB2W_\x82a\x05T\x85\x84a\tsV[a\x05^\x91\x90a\t\x8AV[\x90Pa\x05j\x85\x82a\tsV[\x90Pa\x05xa'\x10\x86a\t\xA9V[a\x05\x82\x90\x82a\tsV[\x90Pa\x05\x90a'\x10\x82a\t\x8AV[\x90Pa\x05\x9Ea'\x10\x82a\t\x8AV[\x9BP\x90\x99Pa\x05\xC2\x98PPPPPPPPPV[_\x80\x9AP\x9APPPPPPPPPP[\x92P\x92\x90PV[`@Qc(\xDD-\x01`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x82\x81\x16`$\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x82\x91\x84\x16\x90c(\xDD-\x01\x90`D\x01a\x01 `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06>W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06b\x91\x90a\x08\xA9V[PPPPPP\x92P\x92PP\x80\x82a\x06y\x91\x90a\t\xBCV[\x93PPPP[\x92\x91PPV[`@Qc>\x15\x01A`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R_\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x83\x91\x90\x83\x16\x90c>\x15\x01A\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x15\x91\x90a\t\xCFV[P\x94\x9C\x9BPPPPPPPPPPPPV[_\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x07\x83WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07\x80\x91\x81\x01\x90a\nbV[`\x01[a\x07\x8FWP`\x12a\x07\x92V[\x90P[a\x07\x9D\x81`\na\x0BeV[\x93\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x07\xB5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xCBW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x07\xDBW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xF1W_\x80\xFD[\x85` ``\x83\x02\x84\x01\x01\x11\x15a\x08\x05W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x089W_\x80\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\x9DW_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x08dW_\x80\xFD[PP\x84Q` \x86\x01Q`@\x87\x01Q``\x88\x01Q`\x80\x89\x01Q`\xA0\x90\x99\x01Q\x93\x9A\x92\x99P\x90\x97\x90\x96P\x94P\x90\x92P\x90PV[\x80Q\x80\x15\x15\x81\x14a\x08\xA4W_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80_\x80_a\x01 \x8A\x8C\x03\x12\x15a\x08\xC2W_\x80\xFD[_\x8AQ\x90P\x80\x99PP_` \x8B\x01Q\x90P\x80\x98PP_`@\x8B\x01Q\x90P\x80\x97PP_``\x8B\x01Q\x90P\x80\x96PP_`\x80\x8B\x01Q\x90P\x80\x95PP_`\xA0\x8B\x01Q\x90P\x80\x94PP_`\xC0\x8B\x01Q\x90P\x80\x93PP`\xE0\x8A\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t*W_\x80\xFD[\x91Pa\t9a\x01\0\x8B\x01a\x08\x95V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[_` \x82\x84\x03\x12\x15a\tXW_\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06\x7FWa\x06\x7Fa\t_V[_\x82a\t\xA4WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x06\x7FWa\x06\x7Fa\t_V[\x80\x82\x01\x80\x82\x11\x15a\x06\x7FWa\x06\x7Fa\t_V[_\x80_\x80_\x80_\x80_\x80a\x01@\x8B\x8D\x03\x12\x15a\t\xE9W_\x80\xFD[\x8AQ` \x8C\x01Q`@\x8D\x01Q``\x8E\x01Q`\x80\x8F\x01Q\x93\x9DP\x91\x9BP\x99P\x97P\x95Pa\n\x17`\xA0\x8C\x01a\x08\x95V[\x94Pa\n%`\xC0\x8C\x01a\x08\x95V[\x93Pa\n3`\xE0\x8C\x01a\x08\x95V[\x92Pa\nBa\x01\0\x8C\x01a\x08\x95V[\x91Pa\nQa\x01 \x8C\x01a\x08\x95V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[_` \x82\x84\x03\x12\x15a\nrW_\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x07\x9DW_\x80\xFD[`\x01\x81[`\x01\x84\x11\x15a\n\xBDW\x80\x85\x04\x81\x11\x15a\n\xA1Wa\n\xA1a\t_V[`\x01\x84\x16\x15a\n\xAFW\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\n\x86V[\x93P\x93\x91PPV[_\x82a\n\xD3WP`\x01a\x06\x7FV[\x81a\n\xDFWP_a\x06\x7FV[\x81`\x01\x81\x14a\n\xF5W`\x02\x81\x14a\n\xFFWa\x0B\x1BV[`\x01\x91PPa\x06\x7FV[`\xFF\x84\x11\x15a\x0B\x10Wa\x0B\x10a\t_V[PP`\x01\x82\x1Ba\x06\x7FV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x0B>WP\x81\x81\na\x06\x7FV[a\x0BJ_\x19\x84\x84a\n\x82V[\x80_\x19\x04\x82\x11\x15a\x0B]Wa\x0B]a\t_V[\x02\x93\x92PPPV[_a\x07\x9D`\xFF\x84\x16\x83a\n\xC5V\xFE\xA2dipfsX\"\x12 \x15\xBB\xE4\x81\xCF\xFEU}\n3\xB3\x11\xFA\x95\x05\xBA\x9C>GH\x95\xBB\x9A\x890\x17X\xFE\x0B\x89\xF9\xA1dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static QUALIFY_USER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct QUALIFY_USER<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for QUALIFY_USER<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for QUALIFY_USER<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for QUALIFY_USER<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for QUALIFY_USER<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(QUALIFY_USER))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> QUALIFY_USER<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                QUALIFY_USER_ABI.clone(),
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
                QUALIFY_USER_ABI.clone(),
                QUALIFY_USER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkUserAccounts` (0x51740e27) function
        pub fn check_user_accounts(
            &self,
            users: ::std::vec::Vec<User>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 116, 14, 39], users)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `topProfitAccount` (0x7eb4be89) function
        pub fn top_profit_account(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([126, 180, 190, 137], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for QUALIFY_USER<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all input parameters for the `checkUserAccounts` function with signature `checkUserAccounts((address,address,address)[])` and selector `0x51740e27`
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
        name = "checkUserAccounts",
        abi = "checkUserAccounts((address,address,address)[])"
    )]
    pub struct CheckUserAccountsCall {
        pub users: ::std::vec::Vec<User>,
    }
    ///Container type for all input parameters for the `topProfitAccount` function with signature `topProfitAccount()` and selector `0x7eb4be89`
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
    #[ethcall(name = "topProfitAccount", abi = "topProfitAccount()")]
    pub struct TopProfitAccountCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum QUALIFY_USERCalls {
        CheckUserAccounts(CheckUserAccountsCall),
        TopProfitAccount(TopProfitAccountCall),
    }
    impl ::ethers::core::abi::AbiDecode for QUALIFY_USERCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <CheckUserAccountsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckUserAccounts(decoded));
            }
            if let Ok(decoded) =
                <TopProfitAccountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TopProfitAccount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for QUALIFY_USERCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckUserAccounts(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TopProfitAccount(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for QUALIFY_USERCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckUserAccounts(element) => ::core::fmt::Display::fmt(element, f),
                Self::TopProfitAccount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckUserAccountsCall> for QUALIFY_USERCalls {
        fn from(value: CheckUserAccountsCall) -> Self {
            Self::CheckUserAccounts(value)
        }
    }
    impl ::core::convert::From<TopProfitAccountCall> for QUALIFY_USERCalls {
        fn from(value: TopProfitAccountCall) -> Self {
            Self::TopProfitAccount(value)
        }
    }
    ///Container type for all return fields from the `topProfitAccount` function with signature `topProfitAccount()` and selector `0x7eb4be89`
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
    pub struct TopProfitAccountReturn {
        pub user_id: ::ethers::core::types::Address,
        pub debt_token: ::ethers::core::types::Address,
        pub collateral_token: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub profit: ::ethers::core::types::U256,
    }
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
