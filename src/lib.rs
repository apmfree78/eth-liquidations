pub mod exchanges {
    pub mod aave_v3 {
        pub mod decode_events;
        pub mod events;
        pub mod get_user_api;
        pub mod get_user_from_contract;
        pub mod update_user;
        pub mod user_structs;
        pub mod implementations {
            pub mod aave_user_data;
            pub mod aave_users_hash;
        }
    }
}

pub mod mempool {
    pub mod detect_price_update;
    pub mod liquidations;
    pub mod update_token_price;
}

pub mod utils {
    pub mod type_conversion;
}

pub mod data {
    pub mod address;
    pub mod erc20;
    pub mod token_price_hash;
}

pub mod abi {
    pub mod aave_oracle;
    pub mod aave_v3_data_provider;
    pub mod aave_v3_pool;
    pub mod erc20;
}

pub mod events {
    pub mod aave_events;
}
