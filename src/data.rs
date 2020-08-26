use std::str::FromStr;
use std::sync::Arc;

use bitcoin::{util::address::Address, Network};
use iced::button;
use magical_bitcoin_wallet::blockchain::{
    Blockchain, Capability, ElectrumBlockchain, OnlineBlockchain, Progress,
};
use magical_bitcoin_wallet::database::BatchDatabase;
use magical_bitcoin_wallet::descriptor::ExtendedDescriptor;
use magical_bitcoin_wallet::sled;
use magical_bitcoin_wallet::Client as ElectrumClient;
use magical_bitcoin_wallet::Wallet;

use crate::error::Error;

fn get_wallet(account: &Account) -> Result<Wallet<ElectrumBlockchain, sled::Tree>, Error> {
    let host = "tcp://localhost:51401";
    let proxy = None;

    let client = ElectrumClient::new(&host, proxy)?;
    let blockchain = ElectrumBlockchain::from(client);

    let db = sled::open("/home/justin/.iced")?;

    // TODO: uuid
    let tree = db.open_tree(account.name.clone())?; // TODO: handle this

    Wallet::new(
        &account.descriptor,
        None,
        // HELP: account network or node network?
        account.network.clone(),
        tree,
        blockchain,
    )
    .map_err(Error::Magical)
}

#[derive(Debug, Clone)]
pub struct Account {
    // TODO: uuid, receive/change descriptors, wallet<...>?
    pub name: String,
    pub descriptor: String,
    pub tab_button: button::State,
    pub network: Network,
}

impl Account {
    pub fn new(name: String, descriptor: String) -> Self {
        let network = Network::Regtest;
        Self {
            name,
            descriptor,
            tab_button: button::State::new(),
            network,
        }
    }
}

impl Account {
    pub fn address(&self) -> Result<Address, Error> {
        let wallet: Wallet<ElectrumBlockchain, sled::Tree> = get_wallet(&self)?;
        wallet.get_new_address().map_err(Error::Magical)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entropy {
    Low,
    Medium,
    High,
}

impl Entropy {
    pub fn all() -> [Entropy; 3] {
        [Entropy::Low, Entropy::Medium, Entropy::High]
    }
    pub fn words(&self) -> u8 {
        match self {
            Self::Low => 12,
            Self::Medium => 18,
            Self::High => 24,
        }
    }
}

impl From<Entropy> for String {
    fn from(entropy: Entropy) -> String {
        String::from(match entropy {
            Entropy::Low => "Regtest",
            Entropy::Medium => "Medium",
            Entropy::High => "High",
        })
    }
}
