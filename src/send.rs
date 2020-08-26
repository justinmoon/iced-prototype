use std::str::FromStr;

use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Element, HorizontalAlignment,
    Text, TextInput,
};

use crate::data::Account;
use crate::error::Error;
use crate::tasks;
use bitcoin::util::amount::Denomination;
use bitcoin::{Address, Amount, Txid};

#[derive(Debug, Clone)]
pub enum Message {
    Address(String),
    Amount(String),
    //Broadcast,
    //BroadcastResult(Result<Txid, Error>),
    AccountUpdated(Account),
    Send,
    SendResult(Result<Txid, Error>),
}

#[derive(Debug, Clone)]
pub struct AddressAndAmount {
    broadcast_button: button::State,
    address_input: text_input::State,
    address: String,
    amount_input: text_input::State,
    amount: String,
    txid: Option<Txid>,
    error: Option<String>,
    sending: bool,
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
            sending: false,
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
                self.error = None;
                Command::none()
            }
            Message::AccountUpdated(account) => {
                self.account = account;
                self.error = None;
                Command::none()
            }
            Message::Send => {
                self.sending = true;
                if let Ok(address) = Address::from_str(&self.address) {
                    if let Ok(amount) = Amount::from_str_in(&self.amount, Denomination::Satoshi) {
                        return Command::perform(
                            tasks::send_money(self.account.clone(), address, amount),
                            Message::SendResult,
                        );
                    } else {
                        self.error = Some("Bad amount".to_string());
                    }
                } else {
                    self.error = Some("Bad address".to_string())
                }
                Command::none()
            }
            Message::SendResult(result) => {
                self.sending = false;
                match result {
                    Ok(txid) => self.txid = Some(txid),
                    Err(_) => self.error = Some("Could not sign".to_string()),
                }
                Command::none()
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        if let Some(txid) = self.txid.clone() {
            Text::new(txid.to_string()).into()
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

            let button: Element<_> = if self.sending {
                Text::new("Sending").into()
            } else {
                // TODO: maybe validate address / amount here?
                Button::new(
                    &mut self.broadcast_button,
                    Text::new("Send").horizontal_alignment(HorizontalAlignment::Center),
                )
                .on_press(Message::Send)
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
