use rust_calculator_gui::lexer::{tokenizer, Token, TokenKind};

#[test]
fn tokenizer_single_test() {
    assert_eq!(vec![
        Token{literal: "2".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "+".to_string(), token_kind: TokenKind::Operator, precidence: 2},
        Token{literal: "2".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("2+2".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "5".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "-".to_string(), token_kind: TokenKind::Operator, precidence: 2},
        Token{literal: "3".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("5-3".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "7".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "*".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "7".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("7*7".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "9".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "/".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "13".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("9/13".to_string()).unwrap());
}

#[test]
fn tokenizer_float_test() {
    assert_eq!(vec![
        Token{literal: "8.389".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "+".to_string(), token_kind: TokenKind::Operator, precidence: 2},
        Token{literal: "2.00".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("8.389+2.00".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "1.8989".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "-".to_string(), token_kind: TokenKind::Operator, precidence: 2},
        Token{literal: "3.333".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("1.8989-3.333".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "3.0001".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "*".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "2.0002".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("3.0001*2.0002".to_string()).unwrap());
    assert_eq!(vec![
        Token{literal: "9.9".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "/".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "2.0".to_string(), token_kind: TokenKind::Num, precidence: -1},
        ], tokenizer("9.9/2.0".to_string()).unwrap());
}

#[test]
fn tokenizer_multi_test() {
    assert_eq!(vec![
        Token{literal: "2".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "/".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "5".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "+".to_string(), token_kind: TokenKind::Operator, precidence: 2},
        Token{literal: "(".to_string(), token_kind: TokenKind::LParen, precidence: -1},
        Token{literal: "2".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: "*".to_string(), token_kind: TokenKind::Operator, precidence: 3},
        Token{literal: "3".to_string(), token_kind: TokenKind::Num, precidence: -1},
        Token{literal: ")".to_string(), token_kind: TokenKind::RParen, precidence: -1},
        ], tokenizer("2/5+(2*3)".to_string()).unwrap());
}