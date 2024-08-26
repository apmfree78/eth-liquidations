pub use chainlink_aggregator::*;
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
pub mod chainlink_aggregator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_link"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract LinkTokenInterface",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_minAnswer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("int192"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_maxAnswer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("int192"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_billingAccessController",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract AccessControllerInterface",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_requesterAccessController",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract AccessControllerInterface",),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_decimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("description"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("acceptPayeeship"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("acceptPayeeship"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transmitter"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addAccess"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkEnabled"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("description"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("description"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("disableAccessCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("disableAccessCheck"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("enableAccessCheck"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("enableAccessCheck"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAnswer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAnswer"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_roundId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBilling"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBilling"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maximumGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reasonableGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("observationPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmissionPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accountingGas"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint24"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getBillingAccessController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getBillingAccessController",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "contract AccessControllerInterface",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLinkToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLinkToken"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("linkToken"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract LinkTokenInterface",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRequesterAccessController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRequesterAccessController",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "contract AccessControllerInterface",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getRoundData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getRoundData"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_roundId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint80"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("roundId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint80"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("answer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("startedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updatedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("answeredInRound"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint80"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTimestamp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTimestamp"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_roundId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTransmitters"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTransmitters"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getValidatorConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getValidatorConfig"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("validator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "contract AggregatorValidatorInterface",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("gasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hasAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hasAccess"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_calldata"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestAnswer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestAnswer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestConfigDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestConfigDetails",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("configCount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestConfigDigestAndEpoch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestConfigDigestAndEpoch",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("scanLogs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestRound"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestRound"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestRoundData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestRoundData"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("roundId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint80"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("answer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("startedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updatedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("answeredInRound"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint80"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestTimestamp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestTimestamp"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestTransmissionDetails"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("latestTransmissionDetails",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("round"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("latestAnswer_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int192"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("latestTimestamp_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("linkAvailableForPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("linkAvailableForPayment",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("availableBalance"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("maxAnswer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("maxAnswer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int192"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minAnswer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minAnswer"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int192"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("oracleObservationCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("oracleObservationCount",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transmitterAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owedPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owedPayment"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transmitterAddress",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeAccess"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestNewRound"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("requestNewRound"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(80usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint80"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBilling"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBilling"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maximumGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reasonableGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("observationPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmissionPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accountingGas"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint24"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setBillingAccessController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setBillingAccessController",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_billingAccessController",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "contract AccessControllerInterface",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("signers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmitters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("f"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("onchainConfig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("offchainConfigVersion",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("offchainConfig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setLinkToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setLinkToken"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("linkToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("contract LinkTokenInterface",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPayees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPayees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmitters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payees"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setRequesterAccessController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setRequesterAccessController",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("requesterAccessController",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "contract AccessControllerInterface",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setValidatorConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setValidatorConfig"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newValidator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "contract AggregatorValidatorInterface",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newGasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("to"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferPayeeship"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferPayeeship"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proposed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transmit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transmit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reportContext"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                    3usize,
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[3]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("report"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ss"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rawVs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("typeAndVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("typeAndVersion"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("version"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawFunds"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawPayment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdrawPayment"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("transmitter"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddedAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddedAccess"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AnswerUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AnswerUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("current"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("roundId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("updatedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BillingAccessControllerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BillingAccessControllerSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("old"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("current"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BillingSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("BillingSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("maximumGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reasonableGasPriceGwei",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("observationPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmissionPaymentGjuels",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("accountingGas"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CheckAccessDisabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CheckAccessDisabled",),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CheckAccessEnabled"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CheckAccessEnabled"),
                        inputs: ::std::vec![],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConfigSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConfigSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousConfigBlockNumber",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("configCount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("signers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitters"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("f"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("onchainConfig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("offchainConfigVersion",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("offchainConfig"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LinkTokenSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LinkTokenSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldLinkToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newLinkToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewRound"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewRound"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("roundId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("startedBy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("startedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewTransmission"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewTransmission"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("aggregatorRoundId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("answer"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("observationsTimestamp",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("observations"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("observers"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("juelsPerFeeCoin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(192usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("epochAndRound"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OraclePaid"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OraclePaid"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("payee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("linkToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferRequested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferRequested",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnershipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PayeeshipTransferRequested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PayeeshipTransferRequested",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("current"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("proposed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PayeeshipTransferred"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PayeeshipTransferred",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("transmitter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previous"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("current"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RemovedAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RemovedAccess"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("user"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RequesterAccessControllerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RequesterAccessControllerSet",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("old"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("current"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RoundRequested"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("RoundRequested"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("requester"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("epoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("round"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transmitted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transmitted"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("configDigest"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("epoch"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValidatorConfigSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ValidatorConfigSet"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousValidator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousGasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("currentValidator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("currentGasLimit"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CHAINLINK_AGGREGATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct CHAINLINK_AGGREGATOR<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CHAINLINK_AGGREGATOR<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CHAINLINK_AGGREGATOR<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CHAINLINK_AGGREGATOR<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CHAINLINK_AGGREGATOR<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CHAINLINK_AGGREGATOR))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CHAINLINK_AGGREGATOR<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CHAINLINK_AGGREGATOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptPayeeship` (0xb121e147) function
        pub fn accept_payeeship(
            &self,
            transmitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 33, 225, 71], transmitter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAccess` (0xa118f249) function
        pub fn add_access(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 24, 242, 73], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkEnabled` (0xdc7f0124) function
        pub fn check_enabled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([220, 127, 1, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disableAccessCheck` (0x0a756983) function
        pub fn disable_access_check(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 117, 105, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enableAccessCheck` (0x8038e4a1) function
        pub fn enable_access_check(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 56, 228, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAnswer` (0xb5ab58dc) function
        pub fn get_answer(
            &self,
            round_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([181, 171, 88, 220], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBilling` (0x29937268) function
        pub fn get_billing(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u32, u32, u32, u32, u32)> {
            self.0
                .method_hash([41, 147, 114, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBillingAccessController` (0xc4c92b37) function
        pub fn get_billing_access_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([196, 201, 43, 55], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLinkToken` (0xe76d5168) function
        pub fn get_link_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([231, 109, 81, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRequesterAccessController` (0xdaffc4b5) function
        pub fn get_requester_access_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([218, 255, 196, 181], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundData` (0x9a6fc8f5) function
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTimestamp` (0xb633620c) function
        pub fn get_timestamp(
            &self,
            round_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 51, 98, 12], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTransmitters` (0x666cab8d) function
        pub fn get_transmitters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([102, 108, 171, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorConfig` (0x9bd2c0b1) function
        pub fn get_validator_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::Address, u32)>
        {
            self.0
                .method_hash([155, 210, 192, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasAccess` (0x6b14daf8) function
        pub fn has_access(
            &self,
            user: ::ethers::core::types::Address,
            calldata: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([107, 20, 218, 248], (user, calldata))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestAnswer` (0x50d25bcd) function
        pub fn latest_answer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([80, 210, 91, 205], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestConfigDetails` (0x81ff7048) function
        pub fn latest_config_details(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u32, u32, [u8; 32])> {
            self.0
                .method_hash([129, 255, 112, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestConfigDigestAndEpoch` (0xafcb95d7) function
        pub fn latest_config_digest_and_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, [u8; 32], u32)> {
            self.0
                .method_hash([175, 203, 149, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRound` (0x668a0f02) function
        pub fn latest_round(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 138, 15, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRoundData` (0xfeaf968c) function
        pub fn latest_round_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestTimestamp` (0x8205bf6a) function
        pub fn latest_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([130, 5, 191, 106], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestTransmissionDetails` (0xe5fe4577) function
        pub fn latest_transmission_details(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ([u8; 32], u32, u8, ::ethers::core::types::I256, u64),
        > {
            self.0
                .method_hash([229, 254, 69, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `linkAvailableForPayment` (0xd09dc339) function
        pub fn link_available_for_payment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 157, 195, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxAnswer` (0x70da2f67) function
        pub fn max_answer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([112, 218, 47, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minAnswer` (0x22adbc78) function
        pub fn min_answer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([34, 173, 188, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracleObservationCount` (0xe4902f82) function
        pub fn oracle_observation_count(
            &self,
            transmitter_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([228, 144, 47, 130], transmitter_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owedPayment` (0x0eafb25b) function
        pub fn owed_payment(
            &self,
            transmitter_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([14, 175, 178, 91], transmitter_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeAccess` (0x8823da6c) function
        pub fn remove_access(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 35, 218, 108], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestNewRound` (0x98e5b12a) function
        pub fn request_new_round(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([152, 229, 177, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBilling` (0x643dc105) function
        pub fn set_billing(
            &self,
            maximum_gas_price_gwei: u32,
            reasonable_gas_price_gwei: u32,
            observation_payment_gjuels: u32,
            transmission_payment_gjuels: u32,
            accounting_gas: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 61, 193, 5],
                    (
                        maximum_gas_price_gwei,
                        reasonable_gas_price_gwei,
                        observation_payment_gjuels,
                        transmission_payment_gjuels,
                        accounting_gas,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBillingAccessController` (0xfbffd2c1) function
        pub fn set_billing_access_controller(
            &self,
            billing_access_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 255, 210, 193], billing_access_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0xe3d0e712) function
        pub fn set_config(
            &self,
            signers: ::std::vec::Vec<::ethers::core::types::Address>,
            transmitters: ::std::vec::Vec<::ethers::core::types::Address>,
            f: u8,
            onchain_config: ::ethers::core::types::Bytes,
            offchain_config_version: u64,
            offchain_config: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 208, 231, 18],
                    (
                        signers,
                        transmitters,
                        f,
                        onchain_config,
                        offchain_config_version,
                        offchain_config,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLinkToken` (0x4fb17470) function
        pub fn set_link_token(
            &self,
            link_token: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 177, 116, 112], (link_token, recipient))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPayees` (0x9c849b30) function
        pub fn set_payees(
            &self,
            transmitters: ::std::vec::Vec<::ethers::core::types::Address>,
            payees: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 132, 155, 48], (transmitters, payees))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRequesterAccessController` (0x9e3ceeab) function
        pub fn set_requester_access_controller(
            &self,
            requester_access_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 60, 238, 171], requester_access_controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setValidatorConfig` (0xeb457163) function
        pub fn set_validator_config(
            &self,
            new_validator: ::ethers::core::types::Address,
            new_gas_limit: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 69, 113, 99], (new_validator, new_gas_limit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferPayeeship` (0xeb5dcd6c) function
        pub fn transfer_payeeship(
            &self,
            transmitter: ::ethers::core::types::Address,
            proposed: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 93, 205, 108], (transmitter, proposed))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transmit` (0xb1dc65a4) function
        pub fn transmit(
            &self,
            report_context: [[u8; 32]; 3],
            report: ::ethers::core::types::Bytes,
            rs: ::std::vec::Vec<[u8; 32]>,
            ss: ::std::vec::Vec<[u8; 32]>,
            raw_vs: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [177, 220, 101, 164],
                    (report_context, report, rs, ss, raw_vs),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `typeAndVersion` (0x181f5a77) function
        pub fn type_and_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([24, 31, 90, 119], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFunds` (0xc1075329) function
        pub fn withdraw_funds(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 7, 83, 41], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawPayment` (0x8ac28d5a) function
        pub fn withdraw_payment(
            &self,
            transmitter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 194, 141, 90], transmitter)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AddedAccess` event
        pub fn added_access_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddedAccessFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AnswerUpdated` event
        pub fn answer_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AnswerUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `BillingAccessControllerSet` event
        pub fn billing_access_controller_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BillingAccessControllerSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BillingSet` event
        pub fn billing_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, BillingSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `CheckAccessDisabled` event
        pub fn check_access_disabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CheckAccessDisabledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `CheckAccessEnabled` event
        pub fn check_access_enabled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CheckAccessEnabledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConfigSet` event
        pub fn config_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConfigSetFilter> {
            self.0.event()
        }
        ///Gets the contract's `LinkTokenSet` event
        pub fn link_token_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LinkTokenSetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewRound` event
        pub fn new_round_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewRoundFilter> {
            self.0.event()
        }
        ///Gets the contract's `NewTransmission` event
        pub fn new_transmission_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewTransmissionFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OraclePaid` event
        pub fn oracle_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OraclePaidFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferRequested` event
        pub fn ownership_transfer_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferRequestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PayeeshipTransferRequested` event
        pub fn payeeship_transfer_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PayeeshipTransferRequestedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PayeeshipTransferred` event
        pub fn payeeship_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PayeeshipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RemovedAccess` event
        pub fn removed_access_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RemovedAccessFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RequesterAccessControllerSet` event
        pub fn requester_access_controller_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequesterAccessControllerSetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RoundRequested` event
        pub fn round_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RoundRequestedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transmitted` event
        pub fn transmitted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransmittedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ValidatorConfigSet` event
        pub fn validator_config_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ValidatorConfigSetFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CHAINLINK_AGGREGATOREvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for CHAINLINK_AGGREGATOR<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(name = "AddedAccess", abi = "AddedAccess(address)")]
    pub struct AddedAccessFilter {
        pub user: ::ethers::core::types::Address,
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
    #[ethevent(name = "AnswerUpdated", abi = "AnswerUpdated(int256,uint256,uint256)")]
    pub struct AnswerUpdatedFilter {
        #[ethevent(indexed)]
        pub current: ::ethers::core::types::I256,
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
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
        name = "BillingAccessControllerSet",
        abi = "BillingAccessControllerSet(address,address)"
    )]
    pub struct BillingAccessControllerSetFilter {
        pub old: ::ethers::core::types::Address,
        pub current: ::ethers::core::types::Address,
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
        name = "BillingSet",
        abi = "BillingSet(uint32,uint32,uint32,uint32,uint24)"
    )]
    pub struct BillingSetFilter {
        pub maximum_gas_price_gwei: u32,
        pub reasonable_gas_price_gwei: u32,
        pub observation_payment_gjuels: u32,
        pub transmission_payment_gjuels: u32,
        pub accounting_gas: u32,
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
    #[ethevent(name = "CheckAccessDisabled", abi = "CheckAccessDisabled()")]
    pub struct CheckAccessDisabledFilter;
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
    #[ethevent(name = "CheckAccessEnabled", abi = "CheckAccessEnabled()")]
    pub struct CheckAccessEnabledFilter;
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
        name = "ConfigSet",
        abi = "ConfigSet(uint32,bytes32,uint64,address[],address[],uint8,bytes,uint64,bytes)"
    )]
    pub struct ConfigSetFilter {
        pub previous_config_block_number: u32,
        pub config_digest: [u8; 32],
        pub config_count: u64,
        pub signers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub transmitters: ::std::vec::Vec<::ethers::core::types::Address>,
        pub f: u8,
        pub onchain_config: ::ethers::core::types::Bytes,
        pub offchain_config_version: u64,
        pub offchain_config: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "LinkTokenSet", abi = "LinkTokenSet(address,address)")]
    pub struct LinkTokenSetFilter {
        #[ethevent(indexed)]
        pub old_link_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_link_token: ::ethers::core::types::Address,
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
    #[ethevent(name = "NewRound", abi = "NewRound(uint256,address,uint256)")]
    pub struct NewRoundFilter {
        #[ethevent(indexed)]
        pub round_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub started_by: ::ethers::core::types::Address,
        pub started_at: ::ethers::core::types::U256,
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
        name = "NewTransmission",
        abi = "NewTransmission(uint32,int192,address,uint32,int192[],bytes,int192,bytes32,uint40)"
    )]
    pub struct NewTransmissionFilter {
        #[ethevent(indexed)]
        pub aggregator_round_id: u32,
        pub answer: ::ethers::core::types::I256,
        pub transmitter: ::ethers::core::types::Address,
        pub observations_timestamp: u32,
        pub observations: ::std::vec::Vec<::ethers::core::types::I256>,
        pub observers: ::ethers::core::types::Bytes,
        pub juels_per_fee_coin: ::ethers::core::types::I256,
        pub config_digest: [u8; 32],
        pub epoch_and_round: u64,
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
        name = "OraclePaid",
        abi = "OraclePaid(address,address,uint256,address)"
    )]
    pub struct OraclePaidFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payee: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub link_token: ::ethers::core::types::Address,
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
        name = "OwnershipTransferRequested",
        abi = "OwnershipTransferRequested(address,address)"
    )]
    pub struct OwnershipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
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
        name = "PayeeshipTransferRequested",
        abi = "PayeeshipTransferRequested(address,address,address)"
    )]
    pub struct PayeeshipTransferRequestedFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub current: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proposed: ::ethers::core::types::Address,
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
        name = "PayeeshipTransferred",
        abi = "PayeeshipTransferred(address,address,address)"
    )]
    pub struct PayeeshipTransferredFilter {
        #[ethevent(indexed)]
        pub transmitter: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub previous: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub current: ::ethers::core::types::Address,
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
    #[ethevent(name = "RemovedAccess", abi = "RemovedAccess(address)")]
    pub struct RemovedAccessFilter {
        pub user: ::ethers::core::types::Address,
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
        name = "RequesterAccessControllerSet",
        abi = "RequesterAccessControllerSet(address,address)"
    )]
    pub struct RequesterAccessControllerSetFilter {
        pub old: ::ethers::core::types::Address,
        pub current: ::ethers::core::types::Address,
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
        name = "RoundRequested",
        abi = "RoundRequested(address,bytes32,uint32,uint8)"
    )]
    pub struct RoundRequestedFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        pub config_digest: [u8; 32],
        pub epoch: u32,
        pub round: u8,
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
    #[ethevent(name = "Transmitted", abi = "Transmitted(bytes32,uint32)")]
    pub struct TransmittedFilter {
        pub config_digest: [u8; 32],
        pub epoch: u32,
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
        name = "ValidatorConfigSet",
        abi = "ValidatorConfigSet(address,uint32,address,uint32)"
    )]
    pub struct ValidatorConfigSetFilter {
        #[ethevent(indexed)]
        pub previous_validator: ::ethers::core::types::Address,
        pub previous_gas_limit: u32,
        #[ethevent(indexed)]
        pub current_validator: ::ethers::core::types::Address,
        pub current_gas_limit: u32,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CHAINLINK_AGGREGATOREvents {
        AddedAccessFilter(AddedAccessFilter),
        AnswerUpdatedFilter(AnswerUpdatedFilter),
        BillingAccessControllerSetFilter(BillingAccessControllerSetFilter),
        BillingSetFilter(BillingSetFilter),
        CheckAccessDisabledFilter(CheckAccessDisabledFilter),
        CheckAccessEnabledFilter(CheckAccessEnabledFilter),
        ConfigSetFilter(ConfigSetFilter),
        LinkTokenSetFilter(LinkTokenSetFilter),
        NewRoundFilter(NewRoundFilter),
        NewTransmissionFilter(NewTransmissionFilter),
        OraclePaidFilter(OraclePaidFilter),
        OwnershipTransferRequestedFilter(OwnershipTransferRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PayeeshipTransferRequestedFilter(PayeeshipTransferRequestedFilter),
        PayeeshipTransferredFilter(PayeeshipTransferredFilter),
        RemovedAccessFilter(RemovedAccessFilter),
        RequesterAccessControllerSetFilter(RequesterAccessControllerSetFilter),
        RoundRequestedFilter(RoundRequestedFilter),
        TransmittedFilter(TransmittedFilter),
        ValidatorConfigSetFilter(ValidatorConfigSetFilter),
    }
    impl ::ethers::contract::EthLogDecode for CHAINLINK_AGGREGATOREvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AddedAccessFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::AddedAccessFilter(decoded));
            }
            if let Ok(decoded) = AnswerUpdatedFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::AnswerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = BillingAccessControllerSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::BillingAccessControllerSetFilter(decoded));
            }
            if let Ok(decoded) = BillingSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::BillingSetFilter(decoded));
            }
            if let Ok(decoded) = CheckAccessDisabledFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::CheckAccessDisabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = CheckAccessEnabledFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::CheckAccessEnabledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ConfigSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::ConfigSetFilter(decoded));
            }
            if let Ok(decoded) = LinkTokenSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::LinkTokenSetFilter(decoded));
            }
            if let Ok(decoded) = NewRoundFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::NewRoundFilter(decoded));
            }
            if let Ok(decoded) = NewTransmissionFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::NewTransmissionFilter(decoded));
            }
            if let Ok(decoded) = OraclePaidFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::OraclePaidFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferRequestedFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::OwnershipTransferRequestedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PayeeshipTransferRequestedFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::PayeeshipTransferRequestedFilter(decoded));
            }
            if let Ok(decoded) = PayeeshipTransferredFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::PayeeshipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RemovedAccessFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::RemovedAccessFilter(decoded));
            }
            if let Ok(decoded) = RequesterAccessControllerSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::RequesterAccessControllerSetFilter(decoded));
            }
            if let Ok(decoded) = RoundRequestedFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::RoundRequestedFilter(decoded));
            }
            if let Ok(decoded) = TransmittedFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::TransmittedFilter(decoded));
            }
            if let Ok(decoded) = ValidatorConfigSetFilter::decode_log(log) {
                return Ok(CHAINLINK_AGGREGATOREvents::ValidatorConfigSetFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CHAINLINK_AGGREGATOREvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddedAccessFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AnswerUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::BillingAccessControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BillingSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckAccessDisabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckAccessEnabledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LinkTokenSetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewRoundFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewTransmissionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OraclePaidFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayeeshipTransferRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayeeshipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovedAccessFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequesterAccessControllerSetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RoundRequestedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransmittedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatorConfigSetFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddedAccessFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: AddedAccessFilter) -> Self {
            Self::AddedAccessFilter(value)
        }
    }
    impl ::core::convert::From<AnswerUpdatedFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: AnswerUpdatedFilter) -> Self {
            Self::AnswerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<BillingAccessControllerSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: BillingAccessControllerSetFilter) -> Self {
            Self::BillingAccessControllerSetFilter(value)
        }
    }
    impl ::core::convert::From<BillingSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: BillingSetFilter) -> Self {
            Self::BillingSetFilter(value)
        }
    }
    impl ::core::convert::From<CheckAccessDisabledFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: CheckAccessDisabledFilter) -> Self {
            Self::CheckAccessDisabledFilter(value)
        }
    }
    impl ::core::convert::From<CheckAccessEnabledFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: CheckAccessEnabledFilter) -> Self {
            Self::CheckAccessEnabledFilter(value)
        }
    }
    impl ::core::convert::From<ConfigSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: ConfigSetFilter) -> Self {
            Self::ConfigSetFilter(value)
        }
    }
    impl ::core::convert::From<LinkTokenSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: LinkTokenSetFilter) -> Self {
            Self::LinkTokenSetFilter(value)
        }
    }
    impl ::core::convert::From<NewRoundFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: NewRoundFilter) -> Self {
            Self::NewRoundFilter(value)
        }
    }
    impl ::core::convert::From<NewTransmissionFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: NewTransmissionFilter) -> Self {
            Self::NewTransmissionFilter(value)
        }
    }
    impl ::core::convert::From<OraclePaidFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: OraclePaidFilter) -> Self {
            Self::OraclePaidFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferRequestedFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: OwnershipTransferRequestedFilter) -> Self {
            Self::OwnershipTransferRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PayeeshipTransferRequestedFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: PayeeshipTransferRequestedFilter) -> Self {
            Self::PayeeshipTransferRequestedFilter(value)
        }
    }
    impl ::core::convert::From<PayeeshipTransferredFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: PayeeshipTransferredFilter) -> Self {
            Self::PayeeshipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<RemovedAccessFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: RemovedAccessFilter) -> Self {
            Self::RemovedAccessFilter(value)
        }
    }
    impl ::core::convert::From<RequesterAccessControllerSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: RequesterAccessControllerSetFilter) -> Self {
            Self::RequesterAccessControllerSetFilter(value)
        }
    }
    impl ::core::convert::From<RoundRequestedFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: RoundRequestedFilter) -> Self {
            Self::RoundRequestedFilter(value)
        }
    }
    impl ::core::convert::From<TransmittedFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: TransmittedFilter) -> Self {
            Self::TransmittedFilter(value)
        }
    }
    impl ::core::convert::From<ValidatorConfigSetFilter> for CHAINLINK_AGGREGATOREvents {
        fn from(value: ValidatorConfigSetFilter) -> Self {
            Self::ValidatorConfigSetFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `acceptPayeeship` function with signature `acceptPayeeship(address)` and selector `0xb121e147`
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
    #[ethcall(name = "acceptPayeeship", abi = "acceptPayeeship(address)")]
    pub struct AcceptPayeeshipCall {
        pub transmitter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addAccess` function with signature `addAccess(address)` and selector `0xa118f249`
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
    #[ethcall(name = "addAccess", abi = "addAccess(address)")]
    pub struct AddAccessCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `checkEnabled` function with signature `checkEnabled()` and selector `0xdc7f0124`
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
    #[ethcall(name = "checkEnabled", abi = "checkEnabled()")]
    pub struct CheckEnabledCall;
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
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
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `disableAccessCheck` function with signature `disableAccessCheck()` and selector `0x0a756983`
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
    #[ethcall(name = "disableAccessCheck", abi = "disableAccessCheck()")]
    pub struct DisableAccessCheckCall;
    ///Container type for all input parameters for the `enableAccessCheck` function with signature `enableAccessCheck()` and selector `0x8038e4a1`
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
    #[ethcall(name = "enableAccessCheck", abi = "enableAccessCheck()")]
    pub struct EnableAccessCheckCall;
    ///Container type for all input parameters for the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    #[ethcall(name = "getAnswer", abi = "getAnswer(uint256)")]
    pub struct GetAnswerCall {
        pub round_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getBilling` function with signature `getBilling()` and selector `0x29937268`
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
    #[ethcall(name = "getBilling", abi = "getBilling()")]
    pub struct GetBillingCall;
    ///Container type for all input parameters for the `getBillingAccessController` function with signature `getBillingAccessController()` and selector `0xc4c92b37`
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
        name = "getBillingAccessController",
        abi = "getBillingAccessController()"
    )]
    pub struct GetBillingAccessControllerCall;
    ///Container type for all input parameters for the `getLinkToken` function with signature `getLinkToken()` and selector `0xe76d5168`
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
    #[ethcall(name = "getLinkToken", abi = "getLinkToken()")]
    pub struct GetLinkTokenCall;
    ///Container type for all input parameters for the `getRequesterAccessController` function with signature `getRequesterAccessController()` and selector `0xdaffc4b5`
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
        name = "getRequesterAccessController",
        abi = "getRequesterAccessController()"
    )]
    pub struct GetRequesterAccessControllerCall;
    ///Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(uint256)")]
    pub struct GetTimestampCall {
        pub round_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTransmitters` function with signature `getTransmitters()` and selector `0x666cab8d`
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
    #[ethcall(name = "getTransmitters", abi = "getTransmitters()")]
    pub struct GetTransmittersCall;
    ///Container type for all input parameters for the `getValidatorConfig` function with signature `getValidatorConfig()` and selector `0x9bd2c0b1`
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
    #[ethcall(name = "getValidatorConfig", abi = "getValidatorConfig()")]
    pub struct GetValidatorConfigCall;
    ///Container type for all input parameters for the `hasAccess` function with signature `hasAccess(address,bytes)` and selector `0x6b14daf8`
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
    #[ethcall(name = "hasAccess", abi = "hasAccess(address,bytes)")]
    pub struct HasAccessCall {
        pub user: ::ethers::core::types::Address,
        pub calldata: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    #[ethcall(name = "latestAnswer", abi = "latestAnswer()")]
    pub struct LatestAnswerCall;
    ///Container type for all input parameters for the `latestConfigDetails` function with signature `latestConfigDetails()` and selector `0x81ff7048`
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
    #[ethcall(name = "latestConfigDetails", abi = "latestConfigDetails()")]
    pub struct LatestConfigDetailsCall;
    ///Container type for all input parameters for the `latestConfigDigestAndEpoch` function with signature `latestConfigDigestAndEpoch()` and selector `0xafcb95d7`
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
        name = "latestConfigDigestAndEpoch",
        abi = "latestConfigDigestAndEpoch()"
    )]
    pub struct LatestConfigDigestAndEpochCall;
    ///Container type for all input parameters for the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    #[ethcall(name = "latestRound", abi = "latestRound()")]
    pub struct LatestRoundCall;
    ///Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    ///Container type for all input parameters for the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    #[ethcall(name = "latestTimestamp", abi = "latestTimestamp()")]
    pub struct LatestTimestampCall;
    ///Container type for all input parameters for the `latestTransmissionDetails` function with signature `latestTransmissionDetails()` and selector `0xe5fe4577`
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
        name = "latestTransmissionDetails",
        abi = "latestTransmissionDetails()"
    )]
    pub struct LatestTransmissionDetailsCall;
    ///Container type for all input parameters for the `linkAvailableForPayment` function with signature `linkAvailableForPayment()` and selector `0xd09dc339`
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
    #[ethcall(name = "linkAvailableForPayment", abi = "linkAvailableForPayment()")]
    pub struct LinkAvailableForPaymentCall;
    ///Container type for all input parameters for the `maxAnswer` function with signature `maxAnswer()` and selector `0x70da2f67`
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
    #[ethcall(name = "maxAnswer", abi = "maxAnswer()")]
    pub struct MaxAnswerCall;
    ///Container type for all input parameters for the `minAnswer` function with signature `minAnswer()` and selector `0x22adbc78`
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
    #[ethcall(name = "minAnswer", abi = "minAnswer()")]
    pub struct MinAnswerCall;
    ///Container type for all input parameters for the `oracleObservationCount` function with signature `oracleObservationCount(address)` and selector `0xe4902f82`
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
        name = "oracleObservationCount",
        abi = "oracleObservationCount(address)"
    )]
    pub struct OracleObservationCountCall {
        pub transmitter_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owedPayment` function with signature `owedPayment(address)` and selector `0x0eafb25b`
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
    #[ethcall(name = "owedPayment", abi = "owedPayment(address)")]
    pub struct OwedPaymentCall {
        pub transmitter_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `removeAccess` function with signature `removeAccess(address)` and selector `0x8823da6c`
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
    #[ethcall(name = "removeAccess", abi = "removeAccess(address)")]
    pub struct RemoveAccessCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `requestNewRound` function with signature `requestNewRound()` and selector `0x98e5b12a`
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
    #[ethcall(name = "requestNewRound", abi = "requestNewRound()")]
    pub struct RequestNewRoundCall;
    ///Container type for all input parameters for the `setBilling` function with signature `setBilling(uint32,uint32,uint32,uint32,uint24)` and selector `0x643dc105`
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
        name = "setBilling",
        abi = "setBilling(uint32,uint32,uint32,uint32,uint24)"
    )]
    pub struct SetBillingCall {
        pub maximum_gas_price_gwei: u32,
        pub reasonable_gas_price_gwei: u32,
        pub observation_payment_gjuels: u32,
        pub transmission_payment_gjuels: u32,
        pub accounting_gas: u32,
    }
    ///Container type for all input parameters for the `setBillingAccessController` function with signature `setBillingAccessController(address)` and selector `0xfbffd2c1`
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
        name = "setBillingAccessController",
        abi = "setBillingAccessController(address)"
    )]
    pub struct SetBillingAccessControllerCall {
        pub billing_access_controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig(address[],address[],uint8,bytes,uint64,bytes)` and selector `0xe3d0e712`
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
        name = "setConfig",
        abi = "setConfig(address[],address[],uint8,bytes,uint64,bytes)"
    )]
    pub struct SetConfigCall {
        pub signers: ::std::vec::Vec<::ethers::core::types::Address>,
        pub transmitters: ::std::vec::Vec<::ethers::core::types::Address>,
        pub f: u8,
        pub onchain_config: ::ethers::core::types::Bytes,
        pub offchain_config_version: u64,
        pub offchain_config: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setLinkToken` function with signature `setLinkToken(address,address)` and selector `0x4fb17470`
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
    #[ethcall(name = "setLinkToken", abi = "setLinkToken(address,address)")]
    pub struct SetLinkTokenCall {
        pub link_token: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPayees` function with signature `setPayees(address[],address[])` and selector `0x9c849b30`
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
    #[ethcall(name = "setPayees", abi = "setPayees(address[],address[])")]
    pub struct SetPayeesCall {
        pub transmitters: ::std::vec::Vec<::ethers::core::types::Address>,
        pub payees: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `setRequesterAccessController` function with signature `setRequesterAccessController(address)` and selector `0x9e3ceeab`
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
        name = "setRequesterAccessController",
        abi = "setRequesterAccessController(address)"
    )]
    pub struct SetRequesterAccessControllerCall {
        pub requester_access_controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setValidatorConfig` function with signature `setValidatorConfig(address,uint32)` and selector `0xeb457163`
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
        name = "setValidatorConfig",
        abi = "setValidatorConfig(address,uint32)"
    )]
    pub struct SetValidatorConfigCall {
        pub new_validator: ::ethers::core::types::Address,
        pub new_gas_limit: u32,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferPayeeship` function with signature `transferPayeeship(address,address)` and selector `0xeb5dcd6c`
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
    #[ethcall(name = "transferPayeeship", abi = "transferPayeeship(address,address)")]
    pub struct TransferPayeeshipCall {
        pub transmitter: ::ethers::core::types::Address,
        pub proposed: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transmit` function with signature `transmit(bytes32[3],bytes,bytes32[],bytes32[],bytes32)` and selector `0xb1dc65a4`
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
        name = "transmit",
        abi = "transmit(bytes32[3],bytes,bytes32[],bytes32[],bytes32)"
    )]
    pub struct TransmitCall {
        pub report_context: [[u8; 32]; 3],
        pub report: ::ethers::core::types::Bytes,
        pub rs: ::std::vec::Vec<[u8; 32]>,
        pub ss: ::std::vec::Vec<[u8; 32]>,
        pub raw_vs: [u8; 32],
    }
    ///Container type for all input parameters for the `typeAndVersion` function with signature `typeAndVersion()` and selector `0x181f5a77`
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
    #[ethcall(name = "typeAndVersion", abi = "typeAndVersion()")]
    pub struct TypeAndVersionCall;
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `withdrawFunds` function with signature `withdrawFunds(address,uint256)` and selector `0xc1075329`
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
    #[ethcall(name = "withdrawFunds", abi = "withdrawFunds(address,uint256)")]
    pub struct WithdrawFundsCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawPayment` function with signature `withdrawPayment(address)` and selector `0x8ac28d5a`
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
    #[ethcall(name = "withdrawPayment", abi = "withdrawPayment(address)")]
    pub struct WithdrawPaymentCall {
        pub transmitter: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CHAINLINK_AGGREGATORCalls {
        AcceptOwnership(AcceptOwnershipCall),
        AcceptPayeeship(AcceptPayeeshipCall),
        AddAccess(AddAccessCall),
        CheckEnabled(CheckEnabledCall),
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        DisableAccessCheck(DisableAccessCheckCall),
        EnableAccessCheck(EnableAccessCheckCall),
        GetAnswer(GetAnswerCall),
        GetBilling(GetBillingCall),
        GetBillingAccessController(GetBillingAccessControllerCall),
        GetLinkToken(GetLinkTokenCall),
        GetRequesterAccessController(GetRequesterAccessControllerCall),
        GetRoundData(GetRoundDataCall),
        GetTimestamp(GetTimestampCall),
        GetTransmitters(GetTransmittersCall),
        GetValidatorConfig(GetValidatorConfigCall),
        HasAccess(HasAccessCall),
        LatestAnswer(LatestAnswerCall),
        LatestConfigDetails(LatestConfigDetailsCall),
        LatestConfigDigestAndEpoch(LatestConfigDigestAndEpochCall),
        LatestRound(LatestRoundCall),
        LatestRoundData(LatestRoundDataCall),
        LatestTimestamp(LatestTimestampCall),
        LatestTransmissionDetails(LatestTransmissionDetailsCall),
        LinkAvailableForPayment(LinkAvailableForPaymentCall),
        MaxAnswer(MaxAnswerCall),
        MinAnswer(MinAnswerCall),
        OracleObservationCount(OracleObservationCountCall),
        OwedPayment(OwedPaymentCall),
        Owner(OwnerCall),
        RemoveAccess(RemoveAccessCall),
        RequestNewRound(RequestNewRoundCall),
        SetBilling(SetBillingCall),
        SetBillingAccessController(SetBillingAccessControllerCall),
        SetConfig(SetConfigCall),
        SetLinkToken(SetLinkTokenCall),
        SetPayees(SetPayeesCall),
        SetRequesterAccessController(SetRequesterAccessControllerCall),
        SetValidatorConfig(SetValidatorConfigCall),
        TransferOwnership(TransferOwnershipCall),
        TransferPayeeship(TransferPayeeshipCall),
        Transmit(TransmitCall),
        TypeAndVersion(TypeAndVersionCall),
        Version(VersionCall),
        WithdrawFunds(WithdrawFundsCall),
        WithdrawPayment(WithdrawPaymentCall),
    }
    impl ::ethers::core::abi::AbiDecode for CHAINLINK_AGGREGATORCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) =
                <AcceptPayeeshipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptPayeeship(decoded));
            }
            if let Ok(decoded) = <AddAccessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddAccess(decoded));
            }
            if let Ok(decoded) = <CheckEnabledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckEnabled(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded) =
                <DisableAccessCheckCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DisableAccessCheck(decoded));
            }
            if let Ok(decoded) =
                <EnableAccessCheckCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnableAccessCheck(decoded));
            }
            if let Ok(decoded) = <GetAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAnswer(decoded));
            }
            if let Ok(decoded) = <GetBillingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBilling(decoded));
            }
            if let Ok(decoded) =
                <GetBillingAccessControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetBillingAccessController(decoded));
            }
            if let Ok(decoded) = <GetLinkTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLinkToken(decoded));
            }
            if let Ok(decoded) =
                <GetRequesterAccessControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRequesterAccessController(decoded));
            }
            if let Ok(decoded) = <GetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRoundData(decoded));
            }
            if let Ok(decoded) = <GetTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <GetTransmittersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTransmitters(decoded));
            }
            if let Ok(decoded) =
                <GetValidatorConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetValidatorConfig(decoded));
            }
            if let Ok(decoded) = <HasAccessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasAccess(decoded));
            }
            if let Ok(decoded) = <LatestAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestAnswer(decoded));
            }
            if let Ok(decoded) =
                <LatestConfigDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestConfigDetails(decoded));
            }
            if let Ok(decoded) =
                <LatestConfigDigestAndEpochCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestConfigDigestAndEpoch(decoded));
            }
            if let Ok(decoded) = <LatestRoundCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestRound(decoded));
            }
            if let Ok(decoded) =
                <LatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestRoundData(decoded));
            }
            if let Ok(decoded) =
                <LatestTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestTransmissionDetailsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestTransmissionDetails(decoded));
            }
            if let Ok(decoded) =
                <LinkAvailableForPaymentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LinkAvailableForPayment(decoded));
            }
            if let Ok(decoded) = <MaxAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxAnswer(decoded));
            }
            if let Ok(decoded) = <MinAnswerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MinAnswer(decoded));
            }
            if let Ok(decoded) =
                <OracleObservationCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OracleObservationCount(decoded));
            }
            if let Ok(decoded) = <OwedPaymentCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwedPayment(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemoveAccessCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveAccess(decoded));
            }
            if let Ok(decoded) =
                <RequestNewRoundCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestNewRound(decoded));
            }
            if let Ok(decoded) = <SetBillingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBilling(decoded));
            }
            if let Ok(decoded) =
                <SetBillingAccessControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetBillingAccessController(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetLinkTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetLinkToken(decoded));
            }
            if let Ok(decoded) = <SetPayeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetPayees(decoded));
            }
            if let Ok(decoded) =
                <SetRequesterAccessControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetRequesterAccessController(decoded));
            }
            if let Ok(decoded) =
                <SetValidatorConfigCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetValidatorConfig(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TransferPayeeshipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferPayeeship(decoded));
            }
            if let Ok(decoded) = <TransmitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transmit(decoded));
            }
            if let Ok(decoded) =
                <TypeAndVersionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TypeAndVersion(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WithdrawFundsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawFunds(decoded));
            }
            if let Ok(decoded) =
                <WithdrawPaymentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WithdrawPayment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CHAINLINK_AGGREGATORCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptPayeeship(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddAccess(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckEnabled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Description(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DisableAccessCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnableAccessCheck(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAnswer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBilling(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBillingAccessController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLinkToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetRequesterAccessController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTimestamp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetTransmitters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetValidatorConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasAccess(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestAnswer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestConfigDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestConfigDigestAndEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRound(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestRoundData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestTimestamp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LatestTransmissionDetails(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LinkAvailableForPayment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxAnswer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinAnswer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleObservationCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwedPayment(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveAccess(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestNewRound(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBilling(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetBillingAccessController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetLinkToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPayees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRequesterAccessController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetValidatorConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferPayeeship(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transmit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TypeAndVersion(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WithdrawPayment(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CHAINLINK_AGGREGATORCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptPayeeship(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckEnabled(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisableAccessCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnableAccessCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBilling(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBillingAccessController(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLinkToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRequesterAccessController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTransmitters(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatorConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfigDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestConfigDigestAndEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestTransmissionDetails(element) => ::core::fmt::Display::fmt(element, f),
                Self::LinkAvailableForPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAnswer(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleObservationCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwedPayment(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestNewRound(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBilling(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBillingAccessController(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLinkToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPayees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRequesterAccessController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetValidatorConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferPayeeship(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transmit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TypeAndVersion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawPayment(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AcceptPayeeshipCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: AcceptPayeeshipCall) -> Self {
            Self::AcceptPayeeship(value)
        }
    }
    impl ::core::convert::From<AddAccessCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: AddAccessCall) -> Self {
            Self::AddAccess(value)
        }
    }
    impl ::core::convert::From<CheckEnabledCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: CheckEnabledCall) -> Self {
            Self::CheckEnabled(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<DisableAccessCheckCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: DisableAccessCheckCall) -> Self {
            Self::DisableAccessCheck(value)
        }
    }
    impl ::core::convert::From<EnableAccessCheckCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: EnableAccessCheckCall) -> Self {
            Self::EnableAccessCheck(value)
        }
    }
    impl ::core::convert::From<GetAnswerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetAnswerCall) -> Self {
            Self::GetAnswer(value)
        }
    }
    impl ::core::convert::From<GetBillingCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetBillingCall) -> Self {
            Self::GetBilling(value)
        }
    }
    impl ::core::convert::From<GetBillingAccessControllerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetBillingAccessControllerCall) -> Self {
            Self::GetBillingAccessController(value)
        }
    }
    impl ::core::convert::From<GetLinkTokenCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetLinkTokenCall) -> Self {
            Self::GetLinkToken(value)
        }
    }
    impl ::core::convert::From<GetRequesterAccessControllerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetRequesterAccessControllerCall) -> Self {
            Self::GetRequesterAccessController(value)
        }
    }
    impl ::core::convert::From<GetRoundDataCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetRoundDataCall) -> Self {
            Self::GetRoundData(value)
        }
    }
    impl ::core::convert::From<GetTimestampCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetTimestampCall) -> Self {
            Self::GetTimestamp(value)
        }
    }
    impl ::core::convert::From<GetTransmittersCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetTransmittersCall) -> Self {
            Self::GetTransmitters(value)
        }
    }
    impl ::core::convert::From<GetValidatorConfigCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: GetValidatorConfigCall) -> Self {
            Self::GetValidatorConfig(value)
        }
    }
    impl ::core::convert::From<HasAccessCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: HasAccessCall) -> Self {
            Self::HasAccess(value)
        }
    }
    impl ::core::convert::From<LatestAnswerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestAnswerCall) -> Self {
            Self::LatestAnswer(value)
        }
    }
    impl ::core::convert::From<LatestConfigDetailsCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestConfigDetailsCall) -> Self {
            Self::LatestConfigDetails(value)
        }
    }
    impl ::core::convert::From<LatestConfigDigestAndEpochCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestConfigDigestAndEpochCall) -> Self {
            Self::LatestConfigDigestAndEpoch(value)
        }
    }
    impl ::core::convert::From<LatestRoundCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestRoundCall) -> Self {
            Self::LatestRound(value)
        }
    }
    impl ::core::convert::From<LatestRoundDataCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestRoundDataCall) -> Self {
            Self::LatestRoundData(value)
        }
    }
    impl ::core::convert::From<LatestTimestampCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestTimestampCall) -> Self {
            Self::LatestTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestTransmissionDetailsCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LatestTransmissionDetailsCall) -> Self {
            Self::LatestTransmissionDetails(value)
        }
    }
    impl ::core::convert::From<LinkAvailableForPaymentCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: LinkAvailableForPaymentCall) -> Self {
            Self::LinkAvailableForPayment(value)
        }
    }
    impl ::core::convert::From<MaxAnswerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: MaxAnswerCall) -> Self {
            Self::MaxAnswer(value)
        }
    }
    impl ::core::convert::From<MinAnswerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: MinAnswerCall) -> Self {
            Self::MinAnswer(value)
        }
    }
    impl ::core::convert::From<OracleObservationCountCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: OracleObservationCountCall) -> Self {
            Self::OracleObservationCount(value)
        }
    }
    impl ::core::convert::From<OwedPaymentCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: OwedPaymentCall) -> Self {
            Self::OwedPayment(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemoveAccessCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: RemoveAccessCall) -> Self {
            Self::RemoveAccess(value)
        }
    }
    impl ::core::convert::From<RequestNewRoundCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: RequestNewRoundCall) -> Self {
            Self::RequestNewRound(value)
        }
    }
    impl ::core::convert::From<SetBillingCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetBillingCall) -> Self {
            Self::SetBilling(value)
        }
    }
    impl ::core::convert::From<SetBillingAccessControllerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetBillingAccessControllerCall) -> Self {
            Self::SetBillingAccessController(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetLinkTokenCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetLinkTokenCall) -> Self {
            Self::SetLinkToken(value)
        }
    }
    impl ::core::convert::From<SetPayeesCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetPayeesCall) -> Self {
            Self::SetPayees(value)
        }
    }
    impl ::core::convert::From<SetRequesterAccessControllerCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetRequesterAccessControllerCall) -> Self {
            Self::SetRequesterAccessController(value)
        }
    }
    impl ::core::convert::From<SetValidatorConfigCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: SetValidatorConfigCall) -> Self {
            Self::SetValidatorConfig(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TransferPayeeshipCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: TransferPayeeshipCall) -> Self {
            Self::TransferPayeeship(value)
        }
    }
    impl ::core::convert::From<TransmitCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: TransmitCall) -> Self {
            Self::Transmit(value)
        }
    }
    impl ::core::convert::From<TypeAndVersionCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: TypeAndVersionCall) -> Self {
            Self::TypeAndVersion(value)
        }
    }
    impl ::core::convert::From<VersionCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawFundsCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: WithdrawFundsCall) -> Self {
            Self::WithdrawFunds(value)
        }
    }
    impl ::core::convert::From<WithdrawPaymentCall> for CHAINLINK_AGGREGATORCalls {
        fn from(value: WithdrawPaymentCall) -> Self {
            Self::WithdrawPayment(value)
        }
    }
    ///Container type for all return fields from the `checkEnabled` function with signature `checkEnabled()` and selector `0xdc7f0124`
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
    pub struct CheckEnabledReturn(pub bool);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
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
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getAnswer` function with signature `getAnswer(uint256)` and selector `0xb5ab58dc`
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
    pub struct GetAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getBilling` function with signature `getBilling()` and selector `0x29937268`
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
    pub struct GetBillingReturn {
        pub maximum_gas_price_gwei: u32,
        pub reasonable_gas_price_gwei: u32,
        pub observation_payment_gjuels: u32,
        pub transmission_payment_gjuels: u32,
        pub accounting_gas: u32,
    }
    ///Container type for all return fields from the `getBillingAccessController` function with signature `getBillingAccessController()` and selector `0xc4c92b37`
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
    pub struct GetBillingAccessControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getLinkToken` function with signature `getLinkToken()` and selector `0xe76d5168`
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
    pub struct GetLinkTokenReturn {
        pub link_token: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getRequesterAccessController` function with signature `getRequesterAccessController()` and selector `0xdaffc4b5`
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
    pub struct GetRequesterAccessControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
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
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `getTimestamp` function with signature `getTimestamp(uint256)` and selector `0xb633620c`
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
    pub struct GetTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getTransmitters` function with signature `getTransmitters()` and selector `0x666cab8d`
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
    pub struct GetTransmittersReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getValidatorConfig` function with signature `getValidatorConfig()` and selector `0x9bd2c0b1`
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
    pub struct GetValidatorConfigReturn {
        pub validator: ::ethers::core::types::Address,
        pub gas_limit: u32,
    }
    ///Container type for all return fields from the `hasAccess` function with signature `hasAccess(address,bytes)` and selector `0x6b14daf8`
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
    pub struct HasAccessReturn(pub bool);
    ///Container type for all return fields from the `latestAnswer` function with signature `latestAnswer()` and selector `0x50d25bcd`
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
    pub struct LatestAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `latestConfigDetails` function with signature `latestConfigDetails()` and selector `0x81ff7048`
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
    pub struct LatestConfigDetailsReturn {
        pub config_count: u32,
        pub block_number: u32,
        pub config_digest: [u8; 32],
    }
    ///Container type for all return fields from the `latestConfigDigestAndEpoch` function with signature `latestConfigDigestAndEpoch()` and selector `0xafcb95d7`
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
    pub struct LatestConfigDigestAndEpochReturn {
        pub scan_logs: bool,
        pub config_digest: [u8; 32],
        pub epoch: u32,
    }
    ///Container type for all return fields from the `latestRound` function with signature `latestRound()` and selector `0x668a0f02`
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
    pub struct LatestRoundReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
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
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `latestTimestamp` function with signature `latestTimestamp()` and selector `0x8205bf6a`
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
    pub struct LatestTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestTransmissionDetails` function with signature `latestTransmissionDetails()` and selector `0xe5fe4577`
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
    pub struct LatestTransmissionDetailsReturn {
        pub config_digest: [u8; 32],
        pub epoch: u32,
        pub round: u8,
        pub latest_answer: ::ethers::core::types::I256,
        pub latest_timestamp: u64,
    }
    ///Container type for all return fields from the `linkAvailableForPayment` function with signature `linkAvailableForPayment()` and selector `0xd09dc339`
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
    pub struct LinkAvailableForPaymentReturn {
        pub available_balance: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `maxAnswer` function with signature `maxAnswer()` and selector `0x70da2f67`
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
    pub struct MaxAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `minAnswer` function with signature `minAnswer()` and selector `0x22adbc78`
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
    pub struct MinAnswerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `oracleObservationCount` function with signature `oracleObservationCount(address)` and selector `0xe4902f82`
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
    pub struct OracleObservationCountReturn(pub u32);
    ///Container type for all return fields from the `owedPayment` function with signature `owedPayment(address)` and selector `0x0eafb25b`
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
    pub struct OwedPaymentReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `requestNewRound` function with signature `requestNewRound()` and selector `0x98e5b12a`
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
    pub struct RequestNewRoundReturn(pub u128);
    ///Container type for all return fields from the `typeAndVersion` function with signature `typeAndVersion()` and selector `0x181f5a77`
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
    pub struct TypeAndVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}
