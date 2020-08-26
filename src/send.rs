use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Element, HorizontalAlignment,
    Text, TextInput,
};

use crate::data::Account;
use crate::error::Error;
use crate::tasks;

#[derive(Debug, Clone)]
pub enum Message {
    Address(String),
    Amount(String),
    Broadcast,
    BroadcastResult(Result<String, Error>),
    AccountUpdated(Account),
}

#[derive(Debug, Clone)]
pub struct AddressAndAmount {
    broadcast_button: button::State,
    address_input: text_input::State,
    address: String,
    amount_input: text_input::State,
    amount: String,
    txid: Option<String>,
    error: Option<String>,
    broadcasting: bool,
    account: Account,
}

impl AddressAndAmount {
    pub fn new(account: Account) -> Self {
        Self {
            broadcast_button: button::State::new(),
            address: "".to_string(),
            address_input: text_input::State::new(),
            amount: "".to_string(),
            amount_input: text_input::State::new(),
            txid: None,
            error: None,
            broadcasting: false,
            account,
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Address(address) => {
                self.address = address;
                Command::none()
            }
            Message::Amount(amount) => {
                self.amount = amount;
                Command::none()
            }
            Message::Broadcast => {
                self.broadcasting = true;
                self.error = None;
                Command::perform(tasks::dummy_broadcast(), Message::BroadcastResult)
            }

            Message::BroadcastResult(result) => {
                self.broadcasting = false;
                match result {
                    Ok(txid) => self.txid = Some(txid),
                    Err(_) => self.error = Some("Couldn't broadcast".to_string()),
                };
                Command::none()
            }
            Message::AccountUpdated(account) => {
                self.account = account;
                Command::none()
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        if let Some(txid) = self.txid.clone() {
            Text::new(txid).into()
        } else {
            let address_input = TextInput::new(
                &mut self.address_input,
                "Address",
                &self.address,
                Message::Address,
            )
            .padding(15);

            let amount_input = TextInput::new(
                &mut self.amount_input,
                "Amount",
                &self.amount,
                Message::Amount,
            )
            .padding(15);

            let button: Element<_> = if self.broadcasting {
                Text::new("Broadcasting").into()
            } else {
                Button::new(
                    &mut self.broadcast_button,
                    Text::new("Send").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::Broadcast)
                .into()
            };

            let mut content = Column::new()
                .padding(20)
                .align_items(Align::Center)
                .push(address_input)
                .push(amount_input);

            if let Some(error) = self.error.clone() {
                content = content.push(Text::new(error));
            };

            content.push(button).into()
        }
    }
}

#[derive(Debug, Clone)]
enum Step {
    AddressAndAmount(AddressAndAmount),
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
            step: Step::AddressAndAmount(AddressAndAmount::new(account.clone())),
            account,
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self.step {
            Step::AddressAndAmount(ref mut step) => step.update(message),
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        match self.step {
            Step::AddressAndAmount(ref mut step) => step.view(),
        }
    }
}
