use bitcoin::{util::address::Address, Network};
use iced::button;
use magical_bitcoin_wallet::blockchain::ElectrumBlockchain;
use magical_bitcoin_wallet::sled;
use magical_bitcoin_wallet::types::TransactionDetails;
use magical_bitcoin_wallet::Client as ElectrumClient;
use magical_bitcoin_wallet::Wallet;

use crate::error::{consume_library_error, Error};

#[derive(Debug, Clone)]
pub struct Account {
    // TODO: uuid, receive/change descriptors, wallet<...>?
    pub name: String,
    pub descriptor: String,
    pub tab_button: button::State,
    pub network: Network,
    pub balance: Option<u64>,
    pub transactions: Option<Vec<TransactionDetails>>,
}

impl Account {
    pub fn new(name: String, descriptor: String) -> Self {
        let network = Network::Regtest;
        Self {
            name,
            descriptor,
            tab_button: button::State::new(),
            network,
            balance: None,
            transactions: None,
        }
    }
}

impl Account {
    pub fn get_wallet(&self) -> Result<Wallet<ElectrumBlockchain, sled::Tree>, Error> {
        let host = "tcp://localhost:51401";
        let proxy = None;

        let client = ElectrumClient::new(&host, proxy)?;
        let blockchain = ElectrumBlockchain::from(client);

        let db = sled::open("/home/justin/.iced")?;

        // TODO: uuid
        let tree = db.open_tree(self.name.clone())?; // TODO: handle this

        println!("{}", &self.descriptor);
        Wallet::new(
            &self.descriptor,
            None,
            // HELP: self network or node network?
            self.network.clone(),
            tree,
            blockchain,
        )
        .map_err(consume_library_error)
    }
    pub fn sync(&self, max_address_param: Option<u32>) -> Result<(), Error> {
        let wallet = self.get_wallet()?;
        wallet
            .sync(max_address_param)
            .map_err(consume_library_error)
    }
    pub fn address(&self) -> Result<Address, Error> {
        let wallet = self.get_wallet()?;
        wallet.get_new_address().map_err(consume_library_error)
    }
    pub fn get_balance(&self) -> Result<u64, Error> {
        let wallet = self.get_wallet()?;
        wallet.get_balance().map_err(consume_library_error)
    }
    pub fn list_transactions(&self) -> Result<Vec<TransactionDetails>, Error> {
        let wallet = self.get_wallet()?;
        wallet
            .list_transactions(false)
            .map_err(consume_library_error)
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
