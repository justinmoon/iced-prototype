use std::time::Duration;

use tokio;
use tokio::time::delay_for;

use crate::data::Account;
use crate::error::Error;

pub async fn dummy_broadcast() -> Result<String, Error> {
    // Sleep one second to simulate p2p activity
    delay_for(Duration::from_secs(1)).await;

    // Return a txid sometimes and error other times
    if rand::random() {
        let txid = "99761825a9cfb0e2a361df43ce67ec5cb986168a5aadd517e2bceb3147ff8c1b".to_string();
        Ok(txid)
    } else {
        Err(Error::BroadcastError)
    }
}

pub async fn update_account(account: Account) -> Result<Account, Error> {
    let mut clone = account.clone();
    account.sync(None)?;
    clone.balance = Some(account.get_balance()?);
    clone.transactions = Some(account.list_transactions()?);
    Ok(clone)
}
