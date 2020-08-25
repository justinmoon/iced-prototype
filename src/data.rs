use iced::button;

#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub descriptor: String,
    pub tab_button: button::State,
}

impl Account {
    pub fn new(name: String, descriptor: String) -> Self {
        Self {
            name,
            descriptor,
            tab_button: button::State::new(),
        }
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
