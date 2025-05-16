use super::Base::*;
use super::*;

#[test]
fn test_one_symbol_tokens() {
    let tokens: Vec<Token> = tokenize(";:()}{,.!=+-*/><&|").collect();
    assert_eq!(
        tokens,
        [
            Token::new(SemiColon, 1),
            Token::new(Colon, 1),
            Token::new(OpenParen, 1),
            Token::new(CloseParen, 1),
            Token::new(CloseBrace, 1),
            Token::new(OpenBrace, 1),
            Token::new(Comma, 1),
            Token::new(Dot, 1),
            Token::new(Bang, 1),
            Token::new(Equal, 1),
            Token::new(Plus, 1),
            Token::new(Minus, 1),
            Token::new(Star, 1),
            Token::new(Slash, 1),
            Token::new(Greater, 1),
            Token::new(Less, 1),
            Token::new(And, 1),
            Token::new(Or, 1),
        ]
    );
}

#[test]
fn test_whitespaces() {
    let tokens: Vec<Token> = tokenize(" .\r.\t.\n. \r\t\n").collect();
    assert_eq!(
        tokens,
        [
            Token::new(Whitespace, 1),
            Token::new(Dot, 1),
            Token::new(Whitespace, 1),
            Token::new(Dot, 1),
            Token::new(Whitespace, 1),
            Token::new(Dot, 1),
            Token::new(Whitespace, 1),
            Token::new(Dot, 1),
            Token::new(Whitespace, 4),
        ]
    );
}

#[test]
fn test_decimal_numbers() {
    let mut tokens = tokenize("1 0b10 0b 0o18 0o 0xFA 0x");
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Decimal,
                    empty_int: false
                }
            },
            1
        ),
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Binary,
                    empty_int: false
                }
            },
            4
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Binary,
                    empty_int: true
                }
            },
            2
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Octal,
                    empty_int: false,
                }
            },
            4
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Octal,
                    empty_int: true
                }
            },
            2
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Hexadecimal,
                    empty_int: false
                }
            },
            4
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Int {
                    base: Hexadecimal,
                    empty_int: true
                }
            },
            2
        )
    );
    assert_eq!(tokens.next(), None);
}

#[test]
fn test_float_numbers() {
    let mut tokens = tokenize("1.2 10. 0b10.1");
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Float {
                    base: Decimal,
                    empty_exponent: false
                }
            },
            3
        )
    );

    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Float {
                    base: Decimal,
                    empty_exponent: false
                }
            },
            3
        )
    );
    assert_eq!(tokens.next().unwrap(), Token::new(Whitespace, 1));
    assert_eq!(
        tokens.next().unwrap(),
        Token::new(
            Literal {
                kind: Float {
                    base: Binary,
                    empty_exponent: false
                }
            },
            6
        )
    );
}
