use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<f32, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    for ref token in tokens {
        match &token.token_kind {
            TokenKind::Num => output.push(token.clone()),
            TokenKind::LParen => operator_stack.push(token.clone()),
            TokenKind::RParen => {
                while let Some(last_op) = operator_stack.pop() {
                    if last_op.token_kind != TokenKind::LParen {
                        output.push(last_op);
                    } else {
                        break
                    }
                }
            }
            TokenKind::Operator => {
                while let Some(last_op) = operator_stack.pop() {
                    if last_op.precidence >= token.precidence && last_op.token_kind != TokenKind::LParen {
                        println!("token        :  {:?}", &token);
                        println!("last operator: {:?}", &last_op);
                        output.push(last_op);
                    } else {
                        operator_stack.push(last_op);
                        break
                    }
                }
                operator_stack.push(token.clone());
            }
        }
        // println!("output:   {:?}", &output);
        println!("op stack: {:?}", &operator_stack);
    }
    while !operator_stack.is_empty() {
        if let Some(operator) = operator_stack.pop() {
            output.push(operator);
        }
    }
    for t in output {
        println!("{}", t.literal)
    }
    return Ok(1.0);
}