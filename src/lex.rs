use miette::{Error, LabeledSpan, WrapErr};

use std::borrow::Cow;
use std::fmt;

pub struct Lexer<'de> {
    source_code: &'de str,
    index: usize,
}

impl<'de> Lexer<'de> {
    #[inline]
    pub fn from(input: &'de str) -> Self {
        Self {
            source_code: input,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'de> {
    text: &'de str,
    token_kind: TokenKind,
}

impl<'de> Token<'de> {
    #[inline]
    fn from(text: &'de str, token_kind: TokenKind) -> Self {
        Self { text, token_kind }
    }

    #[inline]
    fn some_ok_from(text: &'de str, token_kind: TokenKind) -> Option<Result<Self, Error>> {
        Some(Ok(Self { text, token_kind }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Bang,
    Colon,
    SemiColon,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Period,
    Minus,
    Plus,
    Star,
    Slash,
    Equal,
    Less,
    Greater,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    BangEqual,
    None,
    Let,
    And,
    Or,
    If,
    Else,
    Elif,
    For,
    While,
    Function,
    Return,
    Struct,
    Self_,
    String,
    Ident,
    Number,
}

impl<'de> fmt::Display for Token<'de> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match self.token_kind {
            TokenKind::Colon => "COLON",
            TokenKind::SemiColon => "SEMICOLON",
            TokenKind::LeftParenthesis => "L_PAREN",
            TokenKind::RightParenthesis => "R_PAREN",
            TokenKind::LeftBrace => "L_BRACE",
            TokenKind::RightBrace => "R_BRACE",
            TokenKind::Comma => "COMMA",
            TokenKind::Period => "PERIOD",
            TokenKind::Minus => "MINUS",
            TokenKind::Plus => "PLUS",
            TokenKind::Star => "STAR",
            TokenKind::Slash => "SLASH",
            TokenKind::Equal => "EQUAL",
            TokenKind::Greater => "GREATER",
            TokenKind::Less => "LESS",
            TokenKind::EqualEqual => "EQUAL_EQUAL",
            TokenKind::LessEqual => "LESS_EQUAL",
            TokenKind::GreaterEqual => "GREATER_EQUAL",
            TokenKind::BangEqual => "BANG_EQUAL",
            TokenKind::None => "NONE",
            TokenKind::Let => "LET",
            TokenKind::And => "AND",
            TokenKind::Bang => "BANG",
            TokenKind::Or => "OR",
            TokenKind::If => "IF",
            TokenKind::Else => "ELSE",
            TokenKind::Elif => "ELSE_IF",
            TokenKind::For => "FOR",
            TokenKind::While => "WHILE",
            TokenKind::Function => "FUNCTION",
            TokenKind::Return => "RETURN",
            TokenKind::Struct => "STRUCT",
            TokenKind::Self_ => "SELF",
            TokenKind::String => &Token::unescape(self.text),
            TokenKind::Ident => self.text,
            TokenKind::Number => self.text,
        };
        f.write_str(token)
    }
}

impl Token<'_> {
    fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        todo!()
    }
}

enum Started {
    String,
    Number,
    Ident,
    IfEqualElse(TokenKind, TokenKind),
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token<'de>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut char_indices = self.source_code[self.index..].char_indices();

        loop {
            let (index, c) = char_indices.next()?;
            let c_str = &self.source_code[self.index + index..self.index + index + c.len_utf8()];
            self.index += c.len_utf8();

            let group_started = match c {
                ':' => return Token::some_ok_from(c_str, TokenKind::Colon),
                ';' => return Token::some_ok_from(c_str, TokenKind::SemiColon),
                '(' => return Token::some_ok_from(c_str, TokenKind::LeftParenthesis),
                ')' => return Token::some_ok_from(c_str, TokenKind::RightParenthesis),
                '{' => return Token::some_ok_from(c_str, TokenKind::LeftBrace),
                '}' => return Token::some_ok_from(c_str, TokenKind::RightBrace),
                ',' => return Token::some_ok_from(c_str, TokenKind::Comma),
                '.' => return Token::some_ok_from(c_str, TokenKind::Period),
                '+' => return Token::some_ok_from(c_str, TokenKind::Plus),
                '-' => return Token::some_ok_from(c_str, TokenKind::Minus),
                '*' => return Token::some_ok_from(c_str, TokenKind::Star),
                '/' => return Token::some_ok_from(c_str, TokenKind::Slash),
                '"' => Started::String,
                '<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
                '>' => Started::IfEqualElse(TokenKind::GreaterEqual, TokenKind::Greater),
                '!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
                '=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
                '0'..='9' => Started::Number,
                'a'..='z' => Started::Ident,
                _ => {
                    return Some(Err(miette::miette! {labels = vec![
                        LabeledSpan::at(self.index - c.len_utf8()..self.index, "This character"),
                    ],
                        "Unrecognised token '{c}'",
                    }
                    .with_source_code(self.source_code.to_string())));
                }
            };

            match group_started {
                Started::IfEqualElse(token_kind_yes, token_kind_no) => {
                    if self.source_code[self.index..].starts_with('=') {
                        self.index += '='.len_utf8();
                        return Token::some_ok_from(
                            &self.source_code[self.index - 2..self.index],
                            token_kind_yes,
                        );
                    } else {
                        return Token::some_ok_from(
                            &self.source_code[self.index - 1..self.index],
                            token_kind_no,
                        );
                    }
                }
                _ => {
                    todo!()
                }
            }
        }
    }
}

mod tests {

    use super::*;

    #[test]
    fn test_single_symbol_tokens() {
        let source = String::from("!:;(){},.-+*/=<>");
        let mut tokens = Lexer::from(&source).into_iter().map(|t| t.unwrap());

        assert_eq!(tokens.next().unwrap(), Token::from("!", TokenKind::Bang));
        assert_eq!(tokens.next().unwrap(), Token::from(":", TokenKind::Colon));
        assert_eq!(
            tokens.next().unwrap(),
            Token::from(";", TokenKind::SemiColon)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("(", TokenKind::LeftParenthesis)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from(")", TokenKind::RightParenthesis)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("{", TokenKind::LeftBrace)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("}", TokenKind::RightBrace)
        );
        assert_eq!(tokens.next().unwrap(), Token::from(",", TokenKind::Comma));
        assert_eq!(tokens.next().unwrap(), Token::from(".", TokenKind::Period));
        assert_eq!(tokens.next().unwrap(), Token::from("-", TokenKind::Minus));
        assert_eq!(tokens.next().unwrap(), Token::from("+", TokenKind::Plus));
        assert_eq!(tokens.next().unwrap(), Token::from("*", TokenKind::Star));
        assert_eq!(tokens.next().unwrap(), Token::from("/", TokenKind::Slash));
        assert_eq!(tokens.next().unwrap(), Token::from("=", TokenKind::Equal));
        assert_eq!(tokens.next().unwrap(), Token::from("<", TokenKind::Less));
        assert_eq!(tokens.next().unwrap(), Token::from(">", TokenKind::Greater));
    }

    #[test]
    fn test_multi_symbol_tokens() {
        let source = String::from("=>=<====!=");
        let mut tokens = Lexer::from(&source).into_iter().map(|t| t.unwrap());

        assert_eq!(tokens.next().unwrap(), Token::from("=", TokenKind::Equal));
        assert_eq!(
            tokens.next().unwrap(),
            Token::from(">=", TokenKind::GreaterEqual)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("<=", TokenKind::LessEqual)
        );
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("==", TokenKind::EqualEqual)
        );
        assert_eq!(tokens.next().unwrap(), Token::from("=", TokenKind::Equal));
        assert_eq!(
            tokens.next().unwrap(),
            Token::from("!=", TokenKind::BangEqual)
        );
    }
}
