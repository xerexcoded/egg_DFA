use std::io::prelude::*;

#[derive(Debug)]
pub enum QuoteStates {
    Left,
    Right,
    Ignore,
}
pub struct CollectQuotes {
    pub buffer: String,
    pub saved: Vec<String>,
    pub state: QuoteStates,
}
fn main() {
    println!("Hello, world!");
}
