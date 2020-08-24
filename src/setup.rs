use iced::{
    button, scrollable, text_input, Align, Button, Column, Command, Container, Element,
    HorizontalAlignment, Length, Radio, Row, Scrollable, Text, TextInput,
};

use crate::data::{Account, Entropy};
use crate::utils::{generate_entropy, mnemonic};

use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::Network;

fn button<'a, Message>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(HorizontalAlignment::Center),
    )
    .padding(12)
    .min_width(100)
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Back,
    SetupComplete(Account),
    NetworkSelected(Network),
    Name(String),
    HowManyWords(Entropy),
}

#[derive(Debug, Clone)]
pub struct Page {
    scroll: scrollable::State,
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
        input_state: text_input::State,
        network: Network,
        name: String,
    },
    HowManyWords {
        network: Network,
        name: String,
        how_many: Option<Entropy>,
    },
    DisplayWords {
        network: Network,
        name: String,
        how_many: Entropy,
        words: Vec<String>,
        xprv: ExtendedPrivKey,
    },
    //CreateAccount {
    //name: String,
    //how_many: u8,
    //words: Vec<String>,
    //account: Account,
    //},
}

fn generate(entropy: Entropy, network: Network) -> (Vec<String>, ExtendedPrivKey) {
    let data = generate_entropy(entropy);
    let words = mnemonic(&data).expect("couldn't create mnemonic");
    let xprv = ExtendedPrivKey::new_master(network, &data).unwrap();
    (words, xprv)
}

impl<'a> Page {
    pub fn new() -> Self {
        Self {
            next_button: button::State::new(),
            back_button: button::State::new(),
            scroll: scrollable::State::new(),
            step: Step::Network { network: None },
        }
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next => match self.step.clone() {
                Step::Network { network } => {
                    if let Some(network) = network {
                        self.step = Step::Name { input_state: text_input::State::new(), name: "".to_string(), network };
                    }
                },
                Step::Name { network, name, .. } => self.step = Step::HowManyWords { network, name, how_many: None },
                Step::HowManyWords { network, name, how_many } => {
                    if let Some(how_many) = how_many {
                        let (words, xprv) = generate(how_many, network);
                        self.step = Step::DisplayWords {
                            network,
                            name,
                            how_many,
                            words,
                            xprv,
                        }
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
                    self.step = Step::Name {
                        network,
                        name,
                        input_state: text_input::State::new(),
                    }
                }
                Step::DisplayWords { network, name, .. } => {
                    self.step = Step::HowManyWords {
                        network,
                        name,
                        how_many: None,
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
            Message::Name(name) => {
                if let Step::Name {
                    network,
                    input_state,
                    ..
                } = self.step.clone()
                {
                    self.step = Step::Name {
                        network,
                        input_state,
                        name,
                    };
                }
            }
            Message::HowManyWords(how_many) => {
                if let Step::HowManyWords { network, name, .. } = self.step.clone() {
                    self.step = Step::HowManyWords {
                        network,
                        name,
                        how_many: Some(how_many),
                    };
                }
            }
            // This is handled above
            Message::SetupComplete(_) => {}
        };

        Command::none()
    }
    pub fn view(&mut self) -> Element<Message> {
        let mut controls = Row::new();

        // Back button
        match self.step.clone() {
            // First step can't go back
            Step::Network { .. } => {}
            _ => {
                controls =
                    controls.push(button(&mut self.back_button, "Back").on_press(Message::Back));
            }
        }

        // Next button
        match self.step.clone() {
            Step::DisplayWords { name, .. } => {
                let account = Account::new(name);
                controls = controls.push(
                    //Button::new(self.next_button, Text::new("Finish"))
                    //.on_press(Message::SetupComplete(account)),
                    button(&mut self.next_button, "Finish")
                        .on_press(Message::SetupComplete(account)),
                );
            }
            _ => {
                controls =
                    controls.push(button(&mut self.next_button, "Next").on_press(Message::Next));
            }
        }

        let content = match &mut self.step {
            Step::Network { network } => Self::network(*network),
            Step::Name {
                input_state, name, ..
            } => Self::name(input_state, name),
            Step::HowManyWords { how_many, .. } => Self::how_many_words(*how_many),
            Step::DisplayWords { words, .. } => Self::display_words(words.clone()),
        };

        // TODO: put the controls outside the scrollable
        let all_content = Column::new().push(content).push(controls);
        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(all_content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }
    fn network(selection: Option<Network>) -> Element<'a, Message> {
        let networks = [
            (Network::Regtest, "Regtest"),
            (Network::Testnet, "Testnet"),
            (Network::Bitcoin, "Mainnet"),
        ];
        let question = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(networks.iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, (network, string)| {
                    choices.push(Radio::new(
                        network,
                        string,
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
            .into()
    }
    fn name(input_state: &'a mut text_input::State, name: &str) -> Element<'a, Message> {
        let input = TextInput::new(input_state, "Name", name, Message::Name)
            .padding(15)
            .size(30);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Name").size(50))
            .push(input)
            .into()
    }
    fn how_many_words(how_many_words: Option<Entropy>) -> Element<'a, Message> {
        let radio = Column::new()
            .padding(20)
            .spacing(10)
            .push(Text::new("Iced is written in...").size(24))
            .push(Entropy::all().iter().cloned().fold(
                Column::new().padding(10).spacing(20),
                |choices, network| {
                    choices.push(Radio::new(
                        network,
                        network,
                        how_many_words,
                        Message::HowManyWords,
                    ))
                },
            ));
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("How Many Words").size(50))
            .push(radio)
            .into()
    }
    fn display_words(words: Vec<String>) -> Element<'a, Message> {
        let word_list = Column::new().padding(20).spacing(10).push(
            words
                .iter()
                .cloned()
                .fold(Column::new().padding(10).spacing(20), |choices, word| {
                    choices.push(Text::new(word))
                }),
        );
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new("Display Words").size(50))
            .push(word_list)
            .into()
    }
}
