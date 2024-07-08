use std::ops::Div;

use crate::lexer::{Token, TokenKind};

pub fn parse(tokens: Vec<Token>) -> Result<f32, String> {
    let postfix: Vec<Token> = infix_to_postfix(tokens)?;
    let mut stack: Vec<f32> = Vec::new();
    for ref token in postfix {
        match &token.token_kind {
            TokenKind::Num => {
                let num: f32 = match token.literal.parse::<f32>() {
                    Ok(val) => val,
                    Err(e) => return Err(e.to_string()),
                };
                stack.push(num);
            },
            TokenKind::Operator => {
                if stack.len() < 2 {
                    return Err("Expected 2 numbers".to_string());
                }
                let y: f32 = stack.pop().unwrap();
                let x: f32 = stack.pop().unwrap();
                match &token.literal {
                    o if *o == "+".to_string() => stack.push(x + y),
                    o if *o == "-".to_string() => stack.push(x - y),
                    o if *o == "*".to_string() => stack.push(x * y),
                    o if *o == "/".to_string() => {
                        let ans: f32 = match x.div(y) {
                            a if a.is_nan() => return Err("Division by zero".to_string()),
                            val => val,
                        };
                        stack.push(ans);
                    }
                    _ => return Err("Invalid Operator".to_string()),
                }
            },
            _ => return Err("Not an operator or number".to_string()),
        }
    }
    if stack.len() == 1 {
        return Ok(stack[0]);
    } else {
        return Err("Invalid stack size".to_string());
    }
}

fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();
    for ref token in tokens {
        match &token.token_kind {
            TokenKind::Num => output.push(token.to_owned()),
            TokenKind::LParen => operator_stack.push(token.to_owned()),
            TokenKind::RParen => {
                while let Some(last_op) = operator_stack.pop() {
                    if last_op.token_kind != TokenKind::LParen {
                        output.push(last_op);
                    } else {
                        break
                    }
                }
            },
            TokenKind::Operator => {
                while let Some(last_op) = operator_stack.pop() {
                    if last_op.precidence >= token.precidence && last_op.token_kind != TokenKind::LParen {
                        output.push(last_op);
                    } else {
                        operator_stack.push(last_op);
                        break
                    }
                }
                operator_stack.push(token.to_owned());
            },
        }
    }
    while !operator_stack.is_empty() {
        if let Some(operator) = operator_stack.pop() {
            output.push(operator);
        }
    }
    return Ok(output);

}