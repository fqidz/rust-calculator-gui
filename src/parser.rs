use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<f32, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    for ref token in tokens {
        match &token.token_kind {
            TokenKind::Num => output.push(token.clone()),
            TokenKind::LParen => output.push(token.clone()),
            TokenKind::RParen => {
                if let Some(last_operator) = operator_stack.last() {
                    while last_operator.token_kind != TokenKind::LParen {
                        output.push(last_operator.clone());
                    } // else last_operator.token_kind == TokenKind::LParen
                    Some(operator_stack.pop());
                }
            }
            t if *t == TokenKind::Add || *t == TokenKind::Subtract => {
                if let Some(last_operator) = operator_stack.last() {
                    while 
                        last_operator.token_kind == TokenKind::Add ||
                        last_operator.token_kind == TokenKind::Subtract ||
                        last_operator.token_kind == TokenKind::Multiply ||
                        last_operator.token_kind == TokenKind::Divide {
                            output.push(last_operator.clone());
                        }
                }
                operator_stack.push(token.clone());
            }
            t if *t == TokenKind::Multiply || *t == TokenKind::Divide => {
                if let Some(last_operator) = operator_stack.last() {
                    while 
                        last_operator.token_kind == TokenKind::Multiply ||
                        last_operator.token_kind == TokenKind::Divide {
                            output.push(last_operator.clone());
                        }
                }
                operator_stack.push(token.clone());
            }
            _ => return Err("Error".to_string()),
        }
        println!("{:?}", operator_stack);
        println!("{:?}", token);
    }
    println!("{:?}", output);
    return Ok(1.0);
}