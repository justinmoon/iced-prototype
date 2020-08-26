macro_rules! impl_error {
    ( $from:ty, $to:ident ) => {
        impl std::convert::From<$from> for Error {
            fn from(err: $from) -> Self {
                Error::$to(err)
            }
        }
    };
}

#[derive(Debug)]
pub enum Error {
    BroadcastError,
    Mnemonic(String),
    AddressDerivationError,

    // Dependencies
    Magical(magical_bitcoin_wallet::error::Error),
    Electrum(magical_bitcoin_wallet::electrum_client::Error),
    Sled(magical_bitcoin_wallet::sled::Error),
}

impl_error!(magical_bitcoin_wallet::error::Error, Magical);
impl_error!(magical_bitcoin_wallet::electrum_client::Error, Electrum);
impl_error!(magical_bitcoin_wallet::sled::Error, Sled);
