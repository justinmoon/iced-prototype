use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

pub fn main() {
    Junction::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Navigate,
}

#[derive(Default)]
struct Setup {
    navigate_button: button::State,
}

impl Setup {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Setup").size(50))
            .push(
                Button::new(&mut self.navigate_button, Text::new("Navigate"))
                    .on_press(Message::Navigate),
            )
            .into()
    }
}

#[derive(Default)]
struct Account {
    navigate_button: button::State,
}

impl Account {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Account").size(50))
            .push(
                Button::new(&mut self.navigate_button, Text::new("Navigate"))
                    .on_press(Message::Navigate),
            )
            .into()
    }
}

enum Page {
    Setup(Setup),
    Account(Account),
}

struct Junction {
    page: Page,
}

impl Junction {
    fn navigate(&mut self) {
        match self.page {
            Page::Account(_) => self.page = Page::Setup(Setup::default()),
            Page::Setup(_) => self.page = Page::Account(Account::default()),
        }
    }
}

impl Sandbox for Junction {
    type Message = Message;

    fn new() -> Self {
        Self {
            page: Page::Setup(Setup::default()),
        }
    }

    fn title(&self) -> String {
        String::from("Junction")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Navigate => self.navigate(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self.page {
            Page::Setup(ref mut setup) => setup.view(),
            Page::Account(ref mut account) => account.view(),
        }
    }
}

