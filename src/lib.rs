pub mod exchanges {
    pub mod aave_v3 {
        pub mod data;
        pub mod events;
        pub mod get_users;
    }
}

pub mod data {
    pub mod address;
    pub mod erc20;
}

pub mod abi {
    pub mod aave_oracle;
    pub mod aave_v3_pool;
    pub mod erc20;
}

pub mod events {
    pub mod aave_events;
}
