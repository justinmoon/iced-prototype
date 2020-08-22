use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};
use log::error;

mod data;
mod setup;

pub fn main() {
    env_logger::init();
    Junction::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Setup(setup::Message),
}

#[derive(Debug, Clone)]
struct Account {
    navigate_button: button::State,
    account: data::Account,
}

impl Account {
    pub fn new(account: data::Account) -> Self {
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
    Account(Account),
}

struct Junction {
    page: Page,
}

impl Sandbox for Junction {
    type Message = Message;

    fn new() -> Self {
        Self {
            page: Page::Setup(setup::Page::new()),
        }
    }

    fn title(&self) -> String {
        String::from("Junction")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Setup(msg) => match self.page {
                Page::Setup(ref mut page) => match msg {
                    // Intercept account completion
                    setup::Message::SetupComplete(account) => {
                        self.page = Page::Account(Account::new(account))
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
            Page::Setup(ref mut setup) => setup.view().map(Message::Setup),
            Page::Account(ref mut account) => account.view(),
        }
    }
}

