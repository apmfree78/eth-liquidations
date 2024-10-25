use ethers::types::Chain;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, fs};

//*****************************************
//*****************************************
//*****************************************
//*****************************************
//*****************************************
// CHANGE THIS VALUE TO SET CHAIN FOR BUILD
pub const CHAIN: Chain = Chain::Mainnet;
//*****************************************
//*****************************************
//*****************************************
//*****************************************
//*****************************************
pub const ETH: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
pub const BTC: &str = "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB";
pub const USD: &str = "0x0000000000000000000000000000000000000348";

pub struct ContractAddresses {
    pub aave_oracle: String,
    pub aave_v3_pool: String,
    pub uniswap_factory: String,
    pub aave_v3_data_provider: String,
    pub liquidate_user: String,
    pub weth: String,
    pub chain_link_feed_registry: String,
    pub qualify_user: String,
}

pub struct ContractAddressMap {
    pub addresses: HashMap<Chain, ContractAddresses>,
}

impl ContractAddressMap {
    fn new(chains: Chains) -> Self {
        let mut addresses = HashMap::<Chain, ContractAddresses>::new();
        addresses.insert(
            Chain::Mainnet,
            ContractAddresses {
                aave_oracle: chains.mainnet.aave_oracle,
                aave_v3_pool: chains.mainnet.aave_v3_pool,
                uniswap_factory: chains.mainnet.uniswap_factory,
                aave_v3_data_provider: chains.mainnet.aave_v3_data_provider,
                liquidate_user: chains.mainnet.liquidate_user,
                chain_link_feed_registry: chains.mainnet.chain_link_feed_registry,
                weth: chains.mainnet.weth,
                qualify_user: chains.mainnet.qualify_user,
            },
        );

        addresses.insert(
            Chain::Sepolia,
            ContractAddresses {
                aave_oracle: chains.sepolia.aave_oracle,
                aave_v3_pool: chains.sepolia.aave_v3_pool,
                uniswap_factory: chains.sepolia.uniswap_factory,
                aave_v3_data_provider: chains.sepolia.aave_v3_data_provider,
                liquidate_user: chains.sepolia.liquidate_user,
                chain_link_feed_registry: chains.sepolia.chain_link_feed_registry,
                weth: chains.sepolia.weth,
                qualify_user: chains.sepolia.qualify_user,
            },
        );
        Self { addresses }
    }

    pub fn get_address(&self) -> &ContractAddresses {
        self.addresses.get(&CHAIN).expect("Chain not supported")
    }
}

#[derive(Deserialize)]
struct Chains {
    mainnet: ChainContracts,
    sepolia: ChainContracts,
}

#[derive(Deserialize)]
struct ChainContracts {
    aave_v3_pool: String,
    uniswap_factory: String,
    aave_oracle: String,
    weth: String,
    aave_v3_data_provider: String,
    liquidate_user: String,
    chain_link_feed_registry: String,
    qualify_user: String,
}

impl Chains {
    fn load() -> Self {
        let config = fs::read_to_string("contracts.toml").expect("failed to read toml file");
        toml::from_str(&config).expect("failed to parse toml to chain config")
    }
}

// CREATE GLOBAL INSTANCE OF ALL CONTRACT ADDRESS FOR A GIVEN CHAIN
pub static CONTRACT: Lazy<ContractAddressMap> = Lazy::new(|| {
    let chains = Chains::load();

    ContractAddressMap::new(chains)
});
