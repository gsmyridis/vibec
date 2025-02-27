use miette::{Error, LabeledSpan, WrapErr};

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

macro_rules! define_tokens {
    (
        $(
            ($token_id:ident, $token_string:expr, $token_name:expr),
        )+
    ) => {
        #[derive(Debug, PartialEq, Eq)]
        pub enum Token {
            $($token_id,)+
        }

        impl fmt::Display for Token {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self {
                    $(Token::$token_id => write!(f, $token_name),)+
                }
            }
        }
    };
}

define_tokens! {
    (LeftParenthesis, "(", "L_PAREN"),
    (RightParenthesis, ")", "R_PAREN"),
    (LeftBrace, "{{", "L_BRACE"),
    (RightBrace, "}}", "R_BRACE"),
    (Comma, ",", "COMMA"),
    (Period, ".", "PERIOD"),
    (Minus, "-", "MINUS"),
    (Plus, "+", "PLUS"),
    (Star, "*", "STAR"),
    (Slash, "/", "SLASH"),
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.source_code[self.index..].chars().next()?;
        self.index += c.len_utf8();
        match c {
            '(' => Some(Ok(Token::LeftParenthesis)),
            ')' => Some(Ok(Token::RightParenthesis)),
            '{' => Some(Ok(Token::LeftBrace)),
            '}' => Some(Ok(Token::RightBrace)),
            ',' => Some(Ok(Token::Comma)),
            '.' => Some(Ok(Token::Period)),
            '+' => Some(Ok(Token::Plus)),
            '-' => Some(Ok(Token::Minus)),
            '*' => Some(Ok(Token::Star)),
            '/' => Some(Ok(Token::Slash)),
            _ => Some(Err(miette::miette! {labels = vec![
                LabeledSpan::at(self.index - c.len_utf8()..self.index, "This character"),
            ],
                "Unrecognised token",
            }
            .with_source_code(self.source_code.to_string()))),
        }
    }
}

mod tests {

    use super::{Lexer, Token};

    #[test]
    fn test_single_tokens() {
        let source = String::from("(){},.+-*/");
        let mut lexer = Lexer::from(&source);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::LeftParenthesis);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::RightParenthesis);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::LeftBrace);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::RightBrace);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Comma);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Period);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Plus);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Minus);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Star);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Slash);
    }
}
