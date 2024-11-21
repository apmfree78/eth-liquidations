use crate::abi::qualifyuser::{TopProfitUserAccount, User, QUALIFY_USER};
use crate::data::address::CONTRACT;
use anyhow::Result;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::SignerMiddleware,
    providers::{Middleware, Provider, Ws},
    signers::{Signer, Wallet},
    types::{Address, Chain, Transaction, U64},
    utils::{Anvil, AnvilInstance},
};
use log::{debug, info, warn};
use std::sync::Arc;

use super::simulation::convert_transaction_to_typed_transaction; // from ethers-rs

pub struct AnvilSimulator {
    pub client: Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
    pub anvil: AnvilInstance,
    pub from_address: Address,
}

impl AnvilSimulator {
    pub async fn new(rpc_url: &str) -> Result<Self> {
        // Main network provider   // Configure Anvil with forking
        let anvil = Anvil::new()
            .fork(rpc_url) // URL of your Geth node
            .spawn();

        // setup mock sender
        let from_address: Address = anvil.addresses()[0];
        let private_keys = anvil.keys();
        let from_private_key = private_keys[0].clone();

        // Connect to Anvil
        let anvil_ws_url = anvil.ws_endpoint();
        let provider = Provider::<Ws>::connect(anvil_ws_url).await?;

        // Create a wallet with the private key
        let wallet = Wallet::from(from_private_key).with_chain_id(Chain::Mainnet);

        // Create the SignerMiddleware
        let client = Arc::new(SignerMiddleware::new(provider, wallet));

        Ok(Self {
            client,
            anvil,
            from_address,
        })
    }

    pub async fn simulate_oracle_plus_qualify_user_tx(
        &self,
        mempool_tx: &Transaction,
        qualify_user_data: Vec<User>,
    ) -> anyhow::Result<TopProfitUserAccount> {
        let qualify_user_address: Address = CONTRACT.get_address().qualify_user.parse()?;
        let qualify_user = QUALIFY_USER::new(qualify_user_address, self.client.clone());

        // Take a snapshot
        let snapshot_id: U64 = self.client.provider().request("evm_snapshot", ()).await?;

        // Impersonate the sender of mempool_tx
        let sender_address = mempool_tx.from;
        self.client
            .provider()
            .request::<_, ()>("anvil_impersonateAccount", [sender_address])
            .await?;

        // Convert and send the first transaction
        let mempool_tx_typed = convert_transaction_to_typed_transaction(&mempool_tx);

        debug!("calculating oracle update on anvil");
        // Send the transaction and get the PendingTransaction
        let pending_tx = self.client.send_transaction(mempool_tx_typed, None).await?;

        // Await the transaction receipt immediately to avoid capturing `pending_tx` in the async state
        let _receipt = pending_tx.await?;
        debug!("calculating oracle update complete!");

        // Stop impersonating the account
        self.client
            .provider()
            .request::<_, ()>("anvil_stopImpersonatingAccount", [sender_address])
            .await?;

        // Read the updated topProfitAccount public variable
        let top_profit_account: TopProfitUserAccount = qualify_user
            .check_user_accounts(qualify_user_data)
            .call()
            .await?;

        let success = self
            .client
            .provider()
            .request::<_, bool>("evm_revert", (snapshot_id,))
            .await?;
        if success {
            info!("State reverted to snapshot {:#?}", snapshot_id);
        } else {
            warn!("Failed to revert state");
        }

        Ok(top_profit_account)
    }
}
