use std::io::prelude::*;

#[derive(Debug)]
pub enum QuoteStates {
    Left,
    Right,
    Ignore,
}

#[derive(Debug)]
pub struct CollectQuotes {
    pub buffer: String,
    pub saved: Vec<String>,
    pub state: QuoteStates,
}
impl CollectQuotes {
    pub fn is_left(&self, item: &str) -> bool {
        let left = vec!["\"".to_string()]; //to get the quotes
        let mut result = false;

        for quotes in left {
            if quotes.eq(item) {
                result = true;
            }
        }
        result
    }

    pub fn is_right(&self, item: &str) -> bool {
        let right = vec!["\"".to_string()];
        let mut result = false;
        for quotes in right {
            if quotes.eq(item) {
                result = true;
            }
        }
        result
    }

    pub fn new() -> Self {
        CollectQuotes {
            buffer: String::new(),
            saved: vec![],
            state: QuoteStates::Ignore, // intial state of the dfa
        }
    }

    pub fn process(&mut self, item: &str) {
        println!("debug: item: {} -- {:?}", item, &self);

        match self.state {
            QuoteStates::Ignore => match self.is_left(item) {
                true => {
                    self.state = QuoteStates::Left;
                    self.buffer.push_str(item);
                }
                false => {}
            },
            QuoteStates::Left => {
                match self.is_right(item) {
                    true => {
                        self.state = QuoteStates::Right;
                        self.buffer.push_str(item);
                        // saving in to the vector saved
                        self.saved.push(self.buffer.clone());
                        // clear buffer

                        self.buffer = String::new();
                    }
                    false => {
                        //else keep pushing into the buffer
                        self.buffer.push_str(item);
                    }
                }
            }
            QuoteStates::Right => match self.is_left(item) {
                true => {
                    self.state = QuoteStates::Left;
                    self.buffer.push_str(item);
                }
                false => {
                    self.state = QuoteStates::Ignore;
                }
            },
        }
    }
}

#[allow(dead_code)] // detect unused imports
fn process_string(string: String) -> CollectQuotes {
    // process the strings as bytes to handle utf8
    let string_as_bytes = string.as_bytes();

    // initialize the state machine
    let mut dfa = CollectQuotes::new();

    // temp storage for bytes that not yet valid utf8 strings
    let mut temp = Vec::new(); // just a new vector for temp storage

    // processing one byte at a time
    for byte in string_as_bytes.bytes() {
        let item = byte.unwrap();

        temp.push(item);

        // take a clone to match and prevent taking ownership
        match String::from_utf8(temp.clone()) {
            Ok(Character) => {
                dfa.process(&Character); // process that one Character

                // clear temp buffer
                temp.clear();
            }
            Err(_) => {
                println!("Note yet a valid utf8 string: {:?}", temp);
            }
        }
    }
    return dfa;
}

fn main() {
    let egg_string =
        "Mau said, \"Sleep now , please !! \" . Pusha replies , \" Meowww nooo !! \"".to_string();
    let dfa = process_string(egg_string);
    println!("dfa : {:?}", dfa);
}
