//! Provides methods for transactions

use anyhow::{Context, Ok};
use aptos_sdk::{coin_client::CoinClient, rest_client::PendingTransaction, types::LocalAccount};

use crate::{client::AptosClient, utils::wrap_coin_amount};

/// create a vanity account
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// crate_txn_hash(aptos_client,from address,to address,amount)
/// ```
pub async fn create_txn_hash(
    aptos_client: &mut AptosClient,
    from: &mut LocalAccount,
    to: &mut LocalAccount,
    amount: u64,
) -> Result<PendingTransaction, String> {
    let mut coin_client = CoinClient::new(&aptos_client.rest_client().clone().unwrap());
    let txn_hash: Result<aptos_sdk::rest_client::PendingTransaction, anyhow::Error> = coin_client
        .transfer(from, to.address(), wrap_coin_amount(amount), None)
        .await;
    Err("123".to_string())
}
