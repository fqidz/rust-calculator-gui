#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Operator,
    Num,
    LParen,
    RParen,
}


#[derive(Clone, Debug)]
pub struct Token {
    pub literal: String,
    pub token_kind: TokenKind,
    pub precidence: i32,
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
                while cur_pos < input_string.len() && 
                    (input_string[cur_pos].is_numeric() || input_string[cur_pos] == '.') {
                        if input_string[cur_pos] == '.' && dec_count == 0 {
                            dec_count += 1 
                        } else if input_string[cur_pos] == '.' && dec_count > 0 {
                            return Err("Number contains multiple decimal points".to_string())
                        }
                        cur_pos += 1;
                }
                literal = input_string[start_pos..cur_pos].iter().collect::<String>();
                token_kind = TokenKind::Num;
            }
            c if
                c == '+' || c == '-' ||
                c == '*' || c == '/' || 
                c == '(' || c == ')' => {
                    literal = input_string[cur_pos].to_string();
                    cur_pos += 1;
                    match c {
                        c if c == '+' || c == '-' => {
                            token_kind = TokenKind::Operator;
                            precidence = 2;
                        },
                        c if c == '*' || c == '/' => {
                            token_kind = TokenKind::Operator;
                            precidence = 3;
                        },
                        '(' => token_kind = TokenKind::LParen,
                        ')' => token_kind = TokenKind::RParen,
                        _ => return Err("Invalid character".to_string()),
                    }
                }
            _ => return Err("Invalid character".to_string()),
        }
        tokens.push(Token{
            literal,
            token_kind,
            precidence,
        });
    }
    return Ok(tokens);
}