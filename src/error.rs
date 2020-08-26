use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Error {
    BroadcastError,
    Mnemonic(String),
    AddressDerivationError,

    // Hack b/c Iced needs cloneable Error, so we can't wrap non-cloneable library errors
    Library(String),
}

// The macro isn't working, so doing this hack for now ...
pub fn consume_library_error(error: impl Debug) -> Error {
    Error::Library(format!("{:?}", error))
}

macro_rules! impl_error {
    ( $from:ty, $to:ident ) => {
        impl std::convert::From<$from> for Error {
            fn from(err: $from) -> Self {
                Error::Library(format!("{:?}", err))
            }
        }
    };
}

impl_error!(magical_bitcoin_wallet::error::Error, Magical);
impl_error!(magical_bitcoin_wallet::electrum_client::Error, Electrum);
impl_error!(magical_bitcoin_wallet::sled::Error, Sled);
