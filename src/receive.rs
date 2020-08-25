use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Element, HorizontalAlignment,
    Text, TextInput,
};

use crate::data::Account;
use crate::error::Error;
use crate::tasks;

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug, Clone)]
pub struct Address {
    account: Account,
    address: String,
}

impl Address {
    // mut?
    pub fn new(mut account: Account) -> Self {
        let address = account.address();
        Self { account, address }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {}
    }
    pub fn view(&mut self) -> Element<Message> {
        Text::new(self.address.clone()).into()
    }
}

#[derive(Debug, Clone)]
enum Step {
    Address(Address),
}

#[derive(Debug, Clone)]
pub struct Page {
    scroll: scrollable::State,
    next_button: button::State,
    back_button: button::State,
    step: Step,
    // FIXME: is it jank to include this here? am i duplicating data?
    account: Account,
}

impl<'a> Page {
    pub fn new(account: Account) -> Self {
        Self {
            next_button: button::State::new(),
            back_button: button::State::new(),
            scroll: scrollable::State::new(),
            step: Step::Address(Address::new(account.clone())),
            account,
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self.step {
            Step::Address(ref mut step) => step.update(message),
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        match self.step {
            Step::Address(ref mut step) => step.view(),
        }
    }
}
