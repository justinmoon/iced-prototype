use iced::{
    button, Align, Button, Column, Element, HorizontalAlignment, Length, Row, Sandbox, Settings,
    Text,
};
use log::error;

mod data;
mod mocks;
mod setup;

use data::Account;

//
// Accounts
//

#[derive(Debug, Clone)]
pub enum AccountMessage {
    ChangeView(AccountView),
}

#[derive(Debug, Clone)]
pub enum AccountView {
    Send,
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
        account_view: AccountView,
    ) -> Element<'a, AccountMessage> {
        Button::new(
            button_state,
            Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
        )
        .on_press(AccountMessage::ChangeView(account_view))
        .into()
    }
    pub fn view(&mut self) -> Element<AccountMessage> {
        let buttons = Column::new()
            .push(Self::button(
                "Transactions",
                &mut self.transactions_view_button,
                AccountView::Transactions,
            ))
            .push(Self::button(
                "Send",
                &mut self.send_view_button,
                AccountView::Send,
            ))
            .push(Self::button(
                "Receive",
                &mut self.receive_view_button,
                AccountView::Receive,
            ))
            .push(Self::button(
                "Settings",
                &mut self.settings_view_button,
                AccountView::Settings,
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
    fn update(&mut self, message: AccountMessage) {
        match message {
            AccountMessage::ChangeView(view) => self.view = view,
        }
    }
    pub fn _main(&self) -> Element<AccountMessage> {
        let content = match self.view {
            AccountView::Transactions => Text::new("Transactions"),
            AccountView::Send => Text::new("Send"),
            AccountView::Receive => Text::new("Receive"),
            AccountView::Settings => Text::new("Settings"),
        };
        let account_name = Text::new(format!("Account: {}", self.account.name)).size(50);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(content)
            .into()
    }
    pub fn main(view: &AccountView, account: &Account) -> Element<'a, AccountMessage> {
        let content = match view {
            AccountView::Transactions => Text::new("Transactions"),
            AccountView::Send => Text::new("Send"),
            AccountView::Receive => Text::new("Receive"),
            AccountView::Settings => Text::new("Settings"),
        };
        //let account_name = Text::new(format!("Account: {}", self.account.name)).size(50);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(content)
            .into()
    }
    pub fn view(&mut self) -> Element<AccountMessage> {
        Row::new()
            .push(self.nav.view())
            .push(AccountPage::main(&self.view, &self.account))
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
            Message::AccountMessage(msg) => {
                if let Page::Account(ref mut account_page) = self.page {
                    account_page.update(msg);
                }
            }
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
                .push(account_page.view().map(Message::AccountMessage))
                .into(),
        }
    }
}

pub fn main() {
    env_logger::init();
    Junction::run(Settings::default())
}
