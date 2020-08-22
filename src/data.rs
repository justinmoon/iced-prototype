#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    Regtest,
    Mainnet,
    Testnet,
}

impl Network {
    pub fn all() -> [Network; 3] {
        [Network::Regtest, Network::Testnet, Network::Mainnet]
    }
}

impl From<Network> for String {
    fn from(network: Network) -> String {
        String::from(match network {
            Network::Regtest => "Regtest",
            Network::Testnet => "Testnet",
            Network::Mainnet => "Mainnet",
        })
    }
}

