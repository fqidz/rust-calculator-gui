use std::{io::Error, thread::current};
fn main() {
    // println!("{}", parse("1/3+(2+3)*(4+5)".to_string()));
    // tokenize("1.003/3+(2+3)*(4+5)".to_string());
    println!("{:?}", tokenizer("111123232h993".to_string()));
}

#[derive(Debug)]
enum TokenKind {
    Operator,
    Num,
    LParen,
    RParen,
    Whitespace,
}

#[derive(Debug)]
struct Token {
    literal: String,
    token_type: TokenKind,
}

fn tokenizer(string: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let input_string: Vec<char> = string
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let mut cur_pos: usize = 0;
    while cur_pos < input_string.len() {
        println!("{cur_pos}");
        let start_pos: usize = cur_pos;
        match input_string[cur_pos] {
            c if c.is_numeric() => {
                while cur_pos < input_string.len() && input_string[cur_pos].is_numeric() {
                    cur_pos += 1;
                }
                tokens.push(Token{
                    literal: input_string[start_pos..cur_pos]
                        .iter()
                        .collect::<String>(),
                    token_type: TokenKind::Num,
                });
            }
            c if
                c == '+' ||
                c == '-' ||
                c == '*' ||
                c == '/' => {
                    tokens.push(Token{
                        literal: input_string[cur_pos].to_string(),
                        token_type: TokenKind::Operator,
                    });
                }
            c if c == '(' => {
                tokens.push(Token{
                    literal: input_string[cur_pos].to_string(),
                    token_type: TokenKind::LParen,
                });
            }
            _ => return Err("Invalid Character".to_string()),
        }
        cur_pos += 1;
    }
    return Ok(tokens);
}


trait Tokenize {
    fn tokenize(&self);
}

impl Tokenize for String {
    fn tokenize(&self) {
        println!("{self}");
    }
}