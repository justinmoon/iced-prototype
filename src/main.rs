use iced::{
    button, Align, Button, Column, Element, HorizontalAlignment, Row, Sandbox, Settings, Text,
};
use log::error;

mod data;
mod mocks;
mod setup;

use data::Account;

pub fn main() {
    env_logger::init();
    Junction::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Setup(setup::Message),
    // TODO: this should just take an account UUID ...
    ChangeAccount(Account),
    CreateAccount,
}

#[derive(Debug, Clone)]
struct AccountPage {
    navigate_button: button::State,
    account: Account,
}

impl AccountPage {
    pub fn new(account: Account) -> Self {
        Self {
            navigate_button: button::State::new(),
            account,
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(format!("Account: {}", self.account.name)).size(50))
            .into()
    }
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

impl Sandbox for Junction {
    type Message = Message;

    fn new() -> Self {
        let accounts = mocks::make_accounts(3);
        Self {
            //page: Page::Setup(setup::Page::new()),
            page: Page::Account(AccountPage::new(accounts[0].clone())),
            accounts,
            new_account_button: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Junction")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeAccount(account) => self.page = Page::Account(AccountPage::new(account)),
            Message::CreateAccount => self.page = Page::Setup(setup::Page::new()),
            Message::Setup(msg) => match self.page {
                Page::Setup(ref mut page) => match msg {
                    // Intercept account completion
                    setup::Message::SetupComplete(account) => {
                        self.page = Page::Account(AccountPage::new(account))
                    }
                    // Forward all other setup wizard events
                    _ => page.update(msg),
                },
                _ => error!("Receive setup message outside setup page"),
            },
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.page {
            Page::Setup(ref mut setup_page) => setup_page.view().map(Message::Setup),
            Page::Account(ref mut account_page) => Column::new()
                .push(Self::tabs(&mut self.accounts, &mut self.new_account_button))
                .push(account_page.view())
                .into(),
        }
    }
}
