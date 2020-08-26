use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Element, HorizontalAlignment,
    Row, Text, TextInput,
};

use crate::data::Account;
use crate::error::Error;
use crate::tasks;
use bitcoin::util::address::Address as BitcoinAddress;

#[derive(Debug, Clone)]
pub enum Message {
    AccountUpdated(Account),
}

#[derive(Debug, Clone)]
pub struct Page {
    account: Account,
}

impl<'a> Page {
    pub fn new(account: Account) -> Self {
        Self { account }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AccountUpdated(account) => {
                self.account = account;
            }
        };
        Command::none()
    }
    pub fn view(&mut self) -> Element<Message> {
        let mut view = Column::new();

        if let Some(transactions) = &self.account.transactions {
            for tx in transactions {
                let delta: i64 = tx.received as i64 - tx.sent as i64;
                let row = Text::new(format!("{} {}", delta, tx.txid));
                view = view.push(row);
            }
        }

        view.into()
    }
}
