fn main() {
    // println!("{}", parse("1/3+(2+3)*(4+5)".to_string()));
    tokenize("1.003/3+(2+3)*(4+5)".to_string());
}

enum TokenType {
    Operator,
    Num,
    LParen,
    RParen,
    Whitespace
}

struct Token {
    literal: String,
    token_type: TokenType
}

fn tokenize(input: String) -> Vec<(Token, TokenType)> {
    let result: Vec<(Token, TokenType)> = Vec::new();
    let input_vec: Vec<char> = input.chars().collect::<Vec<char>>();
    for index in 0..input.len() {
        let mut peek_index: usize = 1;
        let mut token: String;
        let mut token_type: TokenType;
        loop {
            match input_vec[index + peek_index] {
                '.' => peek_index += 1,
                c if c.is_whitespace() => {
                    token = input_vec[index..peek_index - 1].into_iter().collect::<String>();
                },
            }
        }
    }
    let mut index: usize;
    while index < input.len() {
        let starting_index: usize = index;
        let mut token_type: TokenType;
        match input_vec[index] {
            c if c.is_numeric() => token_type = TokenType::Num,
            c if 
                c == '+' ||
                c == '-' || 
                c == '+' || 
                c == '/' => token_type = TokenType::Operator,
            c if c.is_whitespace() => token_type = TokenType::Whitespace,
            '(' => token_type = TokenType::LParen,
            ')' => token_type = TokenType::RParen,
            _ => panic!()
        }
        index += 1;
        loop {
            match input_vec[index] {
                c if c.is_whitespace() => break,
                '.' => index += 1,
                c if c.is_numeric() => index += 1
                _ => panic!()
            }
        }
        let token: String;
    }
    

    return result;
}

fn parse(input: String) -> String {
    let input: String = input
        .trim()
        .parse::<String>()
        .unwrap();

    return "".to_string();
}