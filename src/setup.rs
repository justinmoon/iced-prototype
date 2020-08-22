use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

use crate::data::Account;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Back,
    SetupComplete(Account),
}

#[derive(Debug, Clone)]
pub struct Page {
    next_button: button::State,
    back_button: button::State,
    step: Step,
}

#[derive(Debug, Clone)]
enum Step {
    Name {
        name: String,
    },
    HowManyWords {
        name: String,
        how_many: u8,
    },
    DisplayWords {
        name: String,
        how_many: u8,
        words: Vec<String>,
    },
    //CreateAccount {
    //name: String,
    //how_many: u8,
    //words: Vec<String>,
    //account: Account,
    //},
}

fn generate_words() -> Vec<String> {
    vec![
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
        String::from("potato"),
    ]
}

impl Page {
    pub fn new() -> Self {
        Self {
            next_button: button::State::new(),
            back_button: button::State::new(),
            step: Step::Name {
                name: "".to_string(),
            },
        }
    }
    pub fn update(&mut self, message: Message) {
        println!("got message: {:?}", message);
        println!("start step: {:?}", self.step);
        match message {
            Message::Next => match self.step.clone() {
                Step::Name { name } => self.step = Step::HowManyWords { name, how_many: 24 },
                Step::HowManyWords { name, how_many } => {
                    self.step = Step::DisplayWords {
                        name,
                        how_many,
                        words: generate_words(),
                    }
                }
                Step::DisplayWords {
                    ..
                    //name,
                    //how_many,
                    //words,
                    // TODO: intercept this in main
                } => println!("Setup Done"),
            },
            Message::Back => match self.step.clone() {
                Step::Name { .. } => println!("Can't go back from step 1"),
                Step::HowManyWords { name, .. } => self.step = Step::Name { name },
                Step::DisplayWords { name, how_many, .. } => {
                    self.step = Step::HowManyWords { name, how_many }
                }
            },
            // Don't handle SetupComplete
            _ => {}
        };
        println!("end step: {:?}", self.step);
    }
    pub fn view(&mut self) -> Element<Message> {
        match self.step.clone() {
            Step::Name { name } => self.name(name),
            Step::HowManyWords { how_many, .. } => self.how_many_words(how_many),
            Step::DisplayWords { name, words, .. } => self.display_words(words, name),
        }
    }
    fn name(&mut self, name: String) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Name").size(50))
            .push(Button::new(&mut self.next_button, Text::new("Next")).on_press(Message::Next))
            .into()
    }
    fn how_many_words(&mut self, how_many_words: u8) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("How Many Words").size(50))
            .push(Button::new(&mut self.back_button, Text::new("Back")).on_press(Message::Back))
            .push(Button::new(&mut self.next_button, Text::new("Next")).on_press(Message::Next))
            .into()
    }
    fn display_words(&mut self, words: Vec<String>, name: String) -> Element<Message> {
        let account = Account { name };
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Display Words").size(50))
            .push(Button::new(&mut self.back_button, Text::new("Back")).on_press(Message::Back))
            .push(
                Button::new(&mut self.next_button, Text::new("Finish"))
                    .on_press(Message::SetupComplete(account)),
            )
            .into()
    }
}
