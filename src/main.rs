use iced::{
    button, executor, Application, Button, Column, Command, Element, HorizontalAlignment, Row,
    Settings, Text,
};
use log::error;

mod account;
mod data;
mod error;
mod mocks;
mod receive;
mod send;
mod setup;
mod tasks;
mod utils;

use data::Account;

#[derive(Debug, Clone)]
pub enum Message {
    Setup(setup::Message),
    // TODO: this should just take an account UUID ...
    ChangeAccount(Account),
    CreateAccount,
    AccountMessage(account::Message),
}

#[derive(Debug, Clone)]
enum Page {
    Setup(setup::Page),
    Account(account::Page),
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
                page: Page::Account(account::Page::new(accounts[0].clone())),
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
                self.page = Page::Account(account::Page::new(account));
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
                        self.page = Page::Account(account::Page::new(account));
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
