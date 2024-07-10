#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Operator,
    Num,
    LParen,
    RParen,
}

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Token {
    pub literal: String,
    pub token_kind: TokenKind,
    pub precidence: i32,
}

pub trait VecTokenToString {
    fn to_string(&self) -> String;
}

impl VecTokenToString for Vec<Token> {
    fn to_string(&self) -> String {
        // return self.literal but as string with spaces in between numbers and operators
        return self
            .iter()
            .map(|c| c.literal.as_str())
            .fold(String::new(), |mut string, c| {
                if !string.is_empty() && string.chars().last().unwrap() != '(' && c != ")" {
                    string.push(' ');
                }
                string.push_str(c);
                return string;
            })
    }
}


pub fn tokenizer(string: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    // clean up string
    let input_string: Vec<char> = string
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    let mut cur_pos: usize = 0;
    while cur_pos < input_string.len() {
        let start_pos: usize = cur_pos;
        let literal: String;
        let token_kind: TokenKind;
        let mut precidence: i32 = -1;
        match input_string[cur_pos] {
            c if c.is_numeric() => {
                let mut dec_count: u16 = 0;
                // keep including numbers and decimal place
                while cur_pos < input_string.len()
                    && (input_string[cur_pos].is_numeric() || input_string[cur_pos] == '.')
                {
                    if input_string[cur_pos] == '.' && dec_count == 0 {
                        dec_count += 1
                    } else if input_string[cur_pos] == '.' && dec_count > 0 {
                        return Err(format!("Number contains multiple decimal points at pos {0}", cur_pos).to_string());
                    }
                    cur_pos += 1;
                }
                literal = input_string[start_pos..cur_pos].iter().collect::<String>();
                token_kind = TokenKind::Num;
            }
            c if c == '-' => 'subtract: {
                // there is prev char
                if cur_pos > 0 && input_string.get(cur_pos - 1).is_some() {
                    let prev_char: char = input_string[cur_pos - 1];
                    match prev_char {
                        // prev char is operator
                        p if p == '+' || p == '-' || p == '*' || p == '/' => {
                            // therefore it is a unary minus
                            cur_pos += 1;
                            let mut dec_count: u16 = 0;
                            while cur_pos < input_string.len() && (input_string[cur_pos].is_numeric() || input_string[cur_pos] == '.') {
                                if input_string[cur_pos] == '.' && dec_count == 0 {
                                    dec_count += 1
                                } else if input_string[cur_pos] == '.' && dec_count > 0 {
                                    return Err(format!("Number contains multiple decimal points at pos {0}", cur_pos).to_string());
                                }
                                cur_pos += 1;
                            }
                            literal = input_string[start_pos..cur_pos].iter().collect::<String>();
                            token_kind = TokenKind::Num;
                            break 'subtract;
                        }
                        _ => {
                            // default to operator
                            literal = input_string[cur_pos].to_string();
                            token_kind = TokenKind::Operator;
                            precidence = 2;
                            cur_pos += 1;
                            break 'subtract;  // break out of match
                        }
                        
                    }
                } else {  // no prev char
                    cur_pos += 1;
                    let mut dec_count: u16 = 0;
                    while cur_pos < input_string.len() && (input_string[cur_pos].is_numeric() || input_string[cur_pos] == '.') {
                        if input_string[cur_pos] == '.' && dec_count == 0 {
                            dec_count += 1
                        } else if input_string[cur_pos] == '.' && dec_count > 0 {
                            return Err(format!("Number contains multiple decimal points at pos {0}", cur_pos).to_string());
                        }
                        cur_pos += 1;
                    }
                    literal = input_string[start_pos..cur_pos].iter().collect::<String>();
                    token_kind = TokenKind::Num;
                }
            }
            c if c == '+' || c == '*' || c == '/' => {
                // no prev and next char
                if input_string.get(cur_pos + 1).is_none() || input_string.get(cur_pos - 1).is_none() {
                    return Err(format!("Invalid operator ({0}) position at pos {1}", c, cur_pos).to_string());
                }
                literal = input_string[cur_pos].to_string();
                match c {
                    c if c == '+' => {
                        token_kind = TokenKind::Operator;
                        precidence = 2;
                    }
                    c if c == '*' || c == '/' => {
                        token_kind = TokenKind::Operator;
                        precidence = 3;
                    }
                    '(' => token_kind = TokenKind::LParen,
                    ')' => token_kind = TokenKind::RParen,
                    _ => return Err("Invalid character".to_string()),
                }
                cur_pos += 1;
            }
            '(' => {
                literal = input_string[cur_pos].to_string();
                token_kind = TokenKind::LParen;
                cur_pos += 1;
            }
            ')' => {
                literal = input_string[cur_pos].to_string();
                token_kind = TokenKind::RParen;
                cur_pos += 1;
            }
            _ => return Err("Invalid character".to_string()),
        }
        tokens.push(Token {
            literal,
            token_kind,
            precidence,
        });
    }
    return Ok(tokens);
}
