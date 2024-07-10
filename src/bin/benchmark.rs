extern crate rust_calculator_gui;
use std::time::Instant;
use rust_calculator_gui::lexer::{Token, tokenizer};
use rust_calculator_gui::parser::parse;

fn main() {
    let mut now = Instant::now();
    let tokens: Vec<Token> = tokenizer("2+3/-2-3--4-(5*2+(3.00+20)/10000.0)".to_string()).unwrap();
    let mut elapsed = now.elapsed();
    println!("Tokenizer took: {:?}", elapsed);

    now = Instant::now();
    let result: f32 = parse(tokens).unwrap();
    elapsed = now.elapsed();
    println!("Parse took: {:?}", elapsed);
    println!("Result: {:?}", result);
}