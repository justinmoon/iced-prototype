use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::{Address, Amount, Txid};
use magical_bitcoin_wallet::TxBuilder;

use crate::data::Account;
use crate::error::{consume_library_error, Error};

pub async fn update_account(account: Account) -> Result<Account, Error> {
    let mut clone = account.clone();
    account.sync(None)?;
    clone.balance = Some(account.get_balance()?);
    clone.transactions = Some(account.list_transactions()?);
    Ok(clone)
}

pub async fn create_psbt(
    account: Account,
    address: Address,
    amount: Amount,
) -> Result<PartiallySignedTransaction, Error> {
    let addressees = vec![(address, amount.as_sat())];
    let builder = TxBuilder::from_addressees(addressees);
    let psbt = account.get_wallet()?.create_tx(builder)?.0;
    Ok(psbt)
}

pub async fn sign_psbt(
    account: Account,
    psbt: PartiallySignedTransaction,
) -> Result<(PartiallySignedTransaction, bool), Error> {
    // FIXME: assume_height, assert that we have private key ???
    let assume_height = None;
    account
        .get_wallet()?
        .sign(psbt, assume_height)
        .map_err(consume_library_error)
}

pub async fn broadcast_psbt(
    account: Account,
    psbt: PartiallySignedTransaction,
) -> Result<Txid, Error> {
    let tx = psbt.extract_tx();
    account
        .get_wallet()?
        .broadcast(tx)
        .map_err(consume_library_error)
}

pub async fn send_money(account: Account, address: Address, amount: Amount) -> Result<Txid, Error> {
    let psbt = create_psbt(account.clone(), address, amount).await?;
    let (psbt, finalized) = sign_psbt(account.clone(), psbt).await?;
    if finalized {
        broadcast_psbt(account, psbt).await
    } else {
        Err(Error::CouldNotFinalize)
    }
}
