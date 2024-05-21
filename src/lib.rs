pub mod exchanges {
    pub mod aave_v3 {
        pub mod decode_events;
        pub mod events;
        pub mod get_user_from_contract;
        pub mod get_users;
        pub mod update_user;
        pub mod user_data;
    }
}

pub mod mempool {
    pub mod detect_price_update;
}

pub mod utils {
    pub mod type_conversion;
}

pub mod data {
    pub mod address;
    pub mod erc20;
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
