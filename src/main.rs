use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, HorizontalAlignment,
    Length, Row, Settings, Text,
};
use log::error;

mod data;
mod error;
mod mocks;
mod send;
mod setup;
mod tasks;
mod utils;

use data::Account;

//
// Accounts
//

#[derive(Debug, Clone)]
pub enum AccountMessage {
    SendView,
    ReceiveView,
    TransactionsView,
    SettingsView,
    Send(send::Message),
}

#[derive(Debug, Clone)]
pub enum AccountView {
    Send(send::Page),
    Receive,
    Transactions,
    Settings,
}

#[derive(Debug, Clone)]
struct AccountNav {
    transactions_view_button: button::State,
    send_view_button: button::State,
    receive_view_button: button::State,
    settings_view_button: button::State,
}

impl<'a> AccountNav {
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
        message: AccountMessage,
    ) -> Element<'a, AccountMessage> {
        Button::new(
            button_state,
            Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(message)
        .into()
    }
    pub fn view(&mut self) -> Element<AccountMessage> {
        let buttons = Column::new()
            .push(Self::button(
                "Transactions",
                &mut self.transactions_view_button,
                AccountMessage::TransactionsView,
            ))
            .push(Self::button(
                "Send",
                &mut self.send_view_button,
                AccountMessage::SendView,
            ))
            .push(Self::button(
                "Receive",
                &mut self.receive_view_button,
                AccountMessage::ReceiveView,
            ))
            .push(Self::button(
                "Settings",
                &mut self.settings_view_button,
                AccountMessage::SettingsView,
            ));
        Column::new().push(buttons).width(Length::Units(200)).into()
    }
}

#[derive(Debug, Clone)]
struct AccountPage {
    navigate_button: button::State,
    account: Account,
    nav: AccountNav,
    view: AccountView,
}

impl<'a> AccountPage {
    pub fn new(account: Account) -> Self {
        Self {
            navigate_button: button::State::new(),
            account,
            view: AccountView::Transactions,
            nav: AccountNav::new(),
        }
    }
    fn update(&mut self, message: AccountMessage) -> Command<AccountMessage> {
        match message {
            AccountMessage::SendView => {
                // FIXME: clone
                self.view = AccountView::Send(send::Page::new(self.account.clone()));
                Command::none()
            }
            AccountMessage::ReceiveView => {
                self.view = AccountView::Receive;
                Command::none()
            }
            AccountMessage::TransactionsView => {
                self.view = AccountView::Transactions;
                Command::none()
            }
            AccountMessage::SettingsView => {
                self.view = AccountView::Settings;
                Command::none()
            }
            AccountMessage::Send(msg) => {
                // FIXME: this is verbose ... maybe have self.view should just map .update to all
                // active child like I did for the Node enum in druid ... ???
                if let AccountView::Send(ref mut view) = &mut self.view {
                    view.update(msg).map(AccountMessage::Send)
                } else {
                    Command::none()
                }
            }
        }
    }
    pub fn view(&mut self) -> Element<AccountMessage> {
        let content: Element<_> = match self.view {
            AccountView::Transactions => Text::new("Transactions").into(),
            AccountView::Send(ref mut send) => send.view().map(AccountMessage::Send),
            AccountView::Receive => Text::new("Receive").into(),
            AccountView::Settings => Text::new("Settings").into(),
        };

        let content: Element<_> = Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(content)
            .into();

        Row::new()
            .push(self.nav.view())
            //.push(AccountPage::main(&self.view, &self.account))
            .push(content)
            .into()
    }
}

//
// Application
//

#[derive(Debug, Clone)]
pub enum Message {
    Setup(setup::Message),
    // TODO: this should just take an account UUID ...
    ChangeAccount(Account),
    CreateAccount,
    AccountMessage(AccountMessage),
}

#[derive(Debug, Clone)]
enum Page {
    Setup(setup::Page),
    Account(AccountPage),
}

struct Junction {
    page: Page,
    accounts: Vec<Account>,
    new_account_button: button::State,
}

impl<'a> Junction {
    fn tabs(
        accounts: &'a mut Vec<Account>,
        new_account_button: &'a mut button::State,
    ) -> Element<'a, Message> {
        let mut tabs = Row::new();

        // Existing accounts
        for account in accounts {
            // FIXME: don't clone this
            let cloned = Account::clone(account);
            let tab = Button::new(
                &mut account.tab_button,
                Text::new(account.name.clone()).horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::ChangeAccount(cloned));
            tabs = tabs.push(tab);
        }

        // New account button
        tabs = tabs.push(
            Button::new(
                new_account_button,
                Text::new("+").horizontal_alignment(HorizontalAlignment::Center),
            )
            .on_press(Message::CreateAccount),
        );

        tabs.into()
    }
}

impl Application for Junction {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let accounts = mocks::make_accounts(3);
        (
            Self {
                //page: Page::Setup(setup::Page::new()),
                page: Page::Account(AccountPage::new(accounts[0].clone())),
                accounts,
                new_account_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Junction")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AccountMessage(msg) => {
                if let Page::Account(ref mut account_page) = self.page {
                    account_page.update(msg).map(Message::AccountMessage)
                } else {
                    Command::none()
                }
            }
            Message::ChangeAccount(account) => {
                self.page = Page::Account(AccountPage::new(account));
                Command::none()
            }
            Message::CreateAccount => {
                self.page = Page::Setup(setup::Page::new());
                Command::none()
            }
            Message::Setup(msg) => match self.page {
                Page::Setup(ref mut page) => match msg {
                    // Intercept account completion
                    setup::Message::SetupComplete(account) => {
                        self.page = Page::Account(AccountPage::new(account));
                        Command::none()
                    }
                    // Forward all other setup wizard events
                    _ => page.update(msg).map(Message::Setup),
                },
                _ => {
                    error!("Receive setup message outside setup page");
                    Command::none()
                }
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.page {
            Page::Setup(ref mut setup_page) => setup_page.view().map(Message::Setup),
            Page::Account(ref mut account_page) => Column::new()
                .push(Self::tabs(&mut self.accounts, &mut self.new_account_button))
                .push(account_page.view().map(Message::AccountMessage))
                .into(),
        }
    }
}

pub fn main() {
    env_logger::init();
    Junction::run(Settings::default())
}
