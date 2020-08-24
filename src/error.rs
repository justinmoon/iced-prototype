#[derive(Debug, Clone)]
pub enum Error {
    BroadcastError,
    Mnemonic(String),
}
