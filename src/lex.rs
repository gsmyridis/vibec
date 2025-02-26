use miette::{Error, WrapErr};

use std::fmt;

struct Lexer<'de> {
    source: &'de str,
    index: usize,
}

impl<'de> Lexer<'de> {
    #[inline]
    fn from(input: &'de str) -> Self {
        Self {
            source: input,
            index: 0,
        }
    }

    #[inline]
    fn rest(&self) -> &'de str {
        &self.source[self.index..]
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
        let c = self.rest().chars().next()?;
        self.index += 1;
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
            _ => None,
        }
    }
}

mod tests {
    use super::*;

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
