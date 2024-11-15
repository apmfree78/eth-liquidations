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

pub mod backrun {
    pub mod flashbots;
    pub mod simulation;
}

pub mod interest {
    pub mod calculate_interest;
}

pub mod mempool {
    pub mod decode_new_price;
    pub mod detect_price_update;
    pub mod liquidations;
    pub mod update_token_price;
}

pub mod utils {
    pub mod connection_pool;
    pub mod logging;
    pub mod type_conversion;
}

pub mod data {
    pub mod address;
    pub mod chainlink_data;
    pub mod chainlink_feed_map;
    pub mod erc20;
    pub mod token_data_hash;
    pub mod token_price_hash;
    pub mod tokens_by_chain;
    pub mod users_to_track;
}

pub mod abi {
    pub mod aave_oracle;
    pub mod aave_v3_data_provider;
    pub mod aave_v3_pool;
    pub mod chainlink_aggregator;
    pub mod chainlink_registry;
    pub mod erc20;
    pub mod liquidate_user;
    pub mod qualifyuser;
    pub mod wsteth;
}

pub mod events {
    pub mod aave_events;
}

pub mod validate {
    pub mod aave_users;
}
