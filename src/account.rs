use iced::{
    button, Align, Button, Column, Command, Element, HorizontalAlignment, Length, Row, Text,
};

use crate::data::Account;
use crate::{receive, send};

#[derive(Debug, Clone)]
pub enum Message {
    SendView,
    ReceiveView,
    TransactionsView,
    SettingsView,
    Send(send::Message),
    Receive(receive::Message),
}

#[derive(Debug, Clone)]
pub enum MainView {
    Send(send::Page),
    Receive(receive::Page),
    Transactions,
    Settings,
}

#[derive(Debug, Clone)]
pub struct Nav {
    transactions_view_button: button::State,
    send_view_button: button::State,
    receive_view_button: button::State,
    settings_view_button: button::State,
}

impl<'a> Nav {
    pub fn new() -> Self {
        Self {
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
    pub fn view(&mut self) -> Element<Message> {
        let buttons = Column::new()
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
            account,
            view: MainView::Transactions,
            nav: Nav::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
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
                self.view = MainView::Transactions;
                Command::none()
            }
            Message::SettingsView => {
                self.view = MainView::Settings;
                Command::none()
            }
            Message::Send(msg) => {
                // FIXME: this is verbose ... maybe have self.view should just map .update to all
                // active child like I did for the Node enum in druid ... ???
                if let MainView::Send(ref mut view) = &mut self.view {
                    view.update(msg).map(Message::Send)
                } else {
                    Command::none()
                }
            }
            Message::Receive(msg) => {
                // FIXME: this is verbose ... maybe have self.view should just map .update to all
                // active child like I did for the Node enum in druid ... ???
                if let MainView::Receive(ref mut view) = &mut self.view {
                    view.update(msg).map(Message::Receive)
                } else {
                    Command::none()
                }
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let content: Element<_> = match self.view {
            MainView::Transactions => Text::new("Transactions").into(),
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

