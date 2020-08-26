use iced::{
    button, Align, Button, Column, Command, Element, HorizontalAlignment, Length, Row, Text,
};

use crate::data::Account;
use crate::{error::Error, receive, send, transactions};

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    SendView,
    ReceiveView,
    TransactionsView,
    SettingsView,

    // Nexted views
    Send(send::Message),
    Receive(receive::Message),
    Transactions(transactions::Message),

    // Account updates
    AccountUpdated(Result<Account, Error>),
}

#[derive(Debug, Clone)]
pub enum MainView {
    Send(send::Page),
    Receive(receive::Page),
    Transactions(transactions::Page),
    Settings,
}

#[derive(Debug, Clone)]
pub struct Nav {
    transactions_view_button: button::State,
    send_view_button: button::State,
    receive_view_button: button::State,
    settings_view_button: button::State,
    account: Account,
}

impl<'a> Nav {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            transactions_view_button: button::State::new(),
            send_view_button: button::State::new(),
            receive_view_button: button::State::new(),
            settings_view_button: button::State::new(),
        }
    }
    pub fn button(
        label: &'a str,
        button_state: &'a mut button::State,
        message: Message,
    ) -> Element<'a, Message> {
        Button::new(
            button_state,
            Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(message)
        .into()
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AccountUpdated(result) => {
                self.account = result.unwrap();
            }
            _ => {}
        };
        Command::none()
    }
    pub fn view(&mut self) -> Element<Message> {
        let balance = match self.account.balance {
            Some(balance) => Text::new(format!("{}: {}", self.account.name, balance)),
            None => Text::new("..."),
        };
        let buttons = Column::new()
            .push(balance)
            .push(Self::button(
                "Transactions",
                &mut self.transactions_view_button,
                Message::TransactionsView,
            ))
            .push(Self::button(
                "Send",
                &mut self.send_view_button,
                Message::SendView,
            ))
            .push(Self::button(
                "Receive",
                &mut self.receive_view_button,
                Message::ReceiveView,
            ))
            .push(Self::button(
                "Settings",
                &mut self.settings_view_button,
                Message::SettingsView,
            ));
        Column::new().push(buttons).width(Length::Units(200)).into()
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    navigate_button: button::State,
    account: Account,
    nav: Nav,
    view: MainView,
}

impl<'a> Page {
    pub fn new(account: Account) -> Self {
        Self {
            navigate_button: button::State::new(),
            account: account.clone(),
            view: MainView::Transactions(transactions::Page::new(account.clone())),
            nav: Nav::new(account),
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message.clone() {
            Message::SendView => {
                // FIXME: clone
                self.view = MainView::Send(send::Page::new(self.account.clone()));
                Command::none()
            }
            Message::ReceiveView => {
                self.view = MainView::Receive(receive::Page::new(self.account.clone()));
                Command::none()
            }
            Message::TransactionsView => {
                self.view = MainView::Transactions(transactions::Page::new(self.account.clone()));
                Command::none()
            }
            Message::SettingsView => {
                self.view = MainView::Settings;
                Command::none()
            }
            // FIXME: these are verbose ... maybe have self.view should just map .update to all
            // active child like I did for the Node enum in druid ... ???
            Message::Send(msg) => {
                if let MainView::Send(ref mut view) = &mut self.view {
                    view.update(msg).map(Message::Send)
                } else {
                    Command::none()
                }
            }
            Message::Receive(msg) => {
                if let MainView::Receive(ref mut view) = &mut self.view {
                    view.update(msg).map(Message::Receive)
                } else {
                    Command::none()
                }
            }
            Message::Transactions(msg) => {
                if let MainView::Transactions(ref mut view) = &mut self.view {
                    view.update(msg).map(Message::Transactions)
                } else {
                    Command::none()
                }
            }
            Message::AccountUpdated(result) => {
                // FIXME: record error state
                if let Ok(account) = result {
                    self.account = account.clone();

                    // Handle the result here. Below they just get an account, not Result<Account,
                    // Error>
                    match &mut self.view {
                        MainView::Send(ref mut view) => {
                            view.update(send::Message::AccountUpdated(account));
                        }
                        MainView::Receive(ref mut view) => {
                            view.update(receive::Message::AccountUpdated(account));
                        }
                        MainView::Transactions(ref mut view) => {
                            view.update(transactions::Message::AccountUpdated(account));
                        }
                        _ => {}
                    }
                }
                self.nav.update(message);
                Command::none()
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let content: Element<_> = match self.view {
            MainView::Transactions(ref mut transactions) => {
                transactions.view().map(Message::Transactions)
            }
            MainView::Send(ref mut send) => send.view().map(Message::Send),
            MainView::Receive(ref mut receive) => receive.view().map(Message::Receive),
            MainView::Settings => Text::new("Settings").into(),
        };

        let content: Element<_> = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(content)
            .into();

        Row::new()
            .push(self.nav.view())
            //.push(Page::main(&self.view, &self.account))
            .push(content)
            .into()
    }
}
