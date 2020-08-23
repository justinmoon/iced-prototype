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

