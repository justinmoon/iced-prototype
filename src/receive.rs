use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Element, HorizontalAlignment,
    Row, Text, TextInput,
};

use crate::data::Account;
use crate::error::Error;
use crate::tasks;
use bitcoin::util::address::Address as BitcoinAddress;

use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug, Clone)]
pub enum Message {
    Copy,
}

#[derive(Debug, Clone)]
pub struct Address {
    account: Account,
    address: BitcoinAddress,
    copy_button: button::State,
}

impl Address {
    // mut?
    pub fn new(mut account: Account) -> Self {
        let address = account.address().expect("Couldn't derive address"); // FIXME
        Self {
            account,
            address,
            copy_button: button::State::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Copy => {
                // TODO: emit event updating UI depending on whether or not this worked ...
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(self.address.to_string()).unwrap();
            }
        };
        Command::none()
    }
    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(Text::new(self.address.to_string()))
            .push(Button::new(&mut self.copy_button, Text::new("Copy")).on_press(Message::Copy))
            .into()
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
