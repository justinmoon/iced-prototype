use iced::{button, Align, Button, Column, Element, Radio, Sandbox, Settings, Text};

use crate::data::{Account, Network};

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Back,
    SetupComplete(Account),
    NetworkSelected(Network),
}

#[derive(Debug, Clone)]
pub struct Page {
    next_button: button::State,
    back_button: button::State,
    step: Step,
}

#[derive(Debug, Clone)]
enum Step {
    Network {
        network: Option<Network>,
    },
    Name {
        network: Network,
        name: String,
    },
    HowManyWords {
        network: Network,
        name: String,
        how_many: u8,
    },
    DisplayWords {
        network: Network,
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
            step: Step::Network { network: None },
        }
    }
    pub fn update(&mut self, message: Message) {
        println!("got message: {:?}", message);
        println!("start step: {:?}", self.step);
        match message {
            Message::Next => match self.step.clone() {
                Step::Network { network } => {
                    if let Some(network) = network {
                        self.step = Step::Name { name: "".to_string(), network };
                    }
                },
                Step::Name { network, name } => self.step = Step::HowManyWords { network, name, how_many: 24 },
                Step::HowManyWords { network, name, how_many } => {
                    self.step = Step::DisplayWords {
                        network,
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
                Step::Network { .. } => println!("Can't go back from step 1"),
                Step::Name { network, .. } => {
                    self.step = Step::Network {
                        network: Some(network),
                    }
                }
                Step::HowManyWords { network, name, .. } => {
                    self.step = Step::Name { network, name }
                }
                Step::DisplayWords {
                    network,
                    name,
                    how_many,
                    ..
                } => {
                    self.step = Step::HowManyWords {
                        network,
                        name,
                        how_many,
                    }
                }
            },
            Message::NetworkSelected(network) => {
                if let Step::Network { .. } = self.step {
                    self.step = Step::Network {
                        network: Some(network),
                    }
                }
            }
            // This is handled above
            Message::SetupComplete(_) => {}
        };
        println!("end step: {:?}", self.step);
    }
    pub fn view(&mut self) -> Element<Message> {
        match self.step.clone() {
            Step::Network { network } => self.network(network),
            Step::Name { name, .. } => self.name(name),
            Step::HowManyWords { how_many, .. } => self.how_many_words(how_many),
            Step::DisplayWords { name, words, .. } => self.display_words(words, name),
        }
    }
    fn network(&mut self, selection: Option<Network>) -> Element<Message> {
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(Network::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, network| {
                    choices.push(Radio::new(
                        network,
                        network,
                        selection,
                        Message::NetworkSelected,
                    ))
                },
            ));

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Choose Network").size(50))
            .push(question)
            .push(Button::new(&mut self.next_button, Text::new("Next")).on_press(Message::Next))
            .into()
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
