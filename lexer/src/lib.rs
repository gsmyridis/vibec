mod token;
use token::{
    Base,
    LiteralKind::{self, *},
    Token,
    TokenKind::{self, *},
};

mod cursor;
use cursor::Cursor;

#[cfg(test)]
mod tests;

/// Checks if `c` is whitespace.
#[inline]
pub fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// Checks if `c` is a valid non-starting character of an identifier.
///
/// Valid characters are ASCII characters and `_`.
#[inline]
pub fn is_id_start(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z')
}

/// Checks if `c` is a valid non-starting character of an identifier.
///
/// Valid characters are ASCII characters, numbers and `_`.
#[inline]
pub fn is_id_continue(c: char) -> bool {
    matches!(c, '_' | 'a'..='z' | 'A'..='Z' | '0'..='9')
}

/// Checks whether `string` is a valid identifier.
///
/// A valid identifier can start with a letter or a `_`, and continues with
/// letters, numbers or `_`.
pub fn is_ident(string: &str) -> bool {
    let mut chars = string.chars();
    if let Some(start) = chars.next() {
        is_id_start(start) && chars.all(is_id_continue)
    } else {
        false
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = cursor.next_token();
        if token.kind != TokenKind::EoF {
            Some(token)
        } else {
            None
        }
    })
}

impl<'a> Cursor<'a> {
    #[inline]
    fn whitespace(&mut self) -> TokenKind {
        self.next_while(is_whitespace);
        Whitespace
    }

    fn number(&mut self, first_digit: char) -> LiteralKind {
        // If first_digit is 0, then check for basis.
        let mut base = Base::Decimal;
        if first_digit == '0' {
            match self.peek_first() {
                'b' => {
                    base = Base::Binary;
                    self.next_char();
                    if !self.eat_decimal_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'o' => {
                    base = Base::Octal;
                    self.next_char();
                    if !self.eat_decimal_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                'x' => {
                    base = Base::Hexadecimal;
                    self.next_char();
                    if !self.eat_hexadecimal_digits() {
                        return Int {
                            base,
                            empty_int: true,
                        };
                    }
                }
                // Not a base prefix; consume additional digits.
                '0'..='9' | '_' => {
                    self.eat_decimal_digits();
                }
                // Not a base prefix; nothing to do.
                '.' | 'e' | 'E' => {}

                // Just a '0'.
                _ => {
                    return Int {
                        base,
                        empty_int: false,
                    };
                }
            }
        } else {
            self.eat_decimal_digits();
        }

        match self.peek_first() {
            // Maybe it is a integer range or a field/method access: (0..2) or '12.foo()')
            '.' if self.peek_second() != '.' && !is_id_start(self.peek_second()) => {
                // If there is anything after '.', it must be a number.
                self.next_char();
                let mut empty_exponent = false;
                if self.peek_first().is_ascii_digit() {
                    self.eat_decimal_digits();
                    if matches!(self.peek_first(), 'e' | 'E') {
                        self.next_char();
                        empty_exponent = !self.eat_float_exponent();
                    }
                }

                Float {
                    base,
                    empty_exponent,
                }
            }
            'e' | 'E' => {
                self.next_char();
                let empty_exponent = self.eat_float_exponent();
                Float {
                    base,
                    empty_exponent,
                }
            }
            _ => Int {
                base,
                empty_int: false,
            },
        }
    }

    /// Consumes the decimal digits and returns if digits were consumed.
    fn eat_decimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek_first() {
                '_' => {
                    self.next_char();
                }
                '0'..='9' => {
                    has_digits = true;
                    self.next_char();
                }
                _ => break,
            }
        }
        has_digits
    }

    /// Consumes the hexadecimal digits and returns if digits were consumed.
    fn eat_hexadecimal_digits(&mut self) -> bool {
        let mut has_digits = false;
        loop {
            match self.peek_first() {
                '_' => {
                    self.next_char();
                }
                '0'..='9' | 'A'..='F' | 'a'..='f' => {
                    has_digits = true;
                    self.next_char();
                }
                _ => break,
            }
        }
        has_digits
    }

    /// Consumes the float exponent and returns if digits were consumed.
    fn eat_float_exponent(&mut self) -> bool {
        if self.peek_first() == '-' || self.peek_first() == '+' {
            self.next_char();
        }
        self.eat_decimal_digits()
    }

    fn next_token(&mut self) -> Token {
        let first_char = match self.next_char() {
            Some(c) => c,
            None => return Token::new(EoF, 0),
        };

        let token_kind = match first_char {
            // Slash, comment or block comment.

            // Whitespace sequence.
            c if is_whitespace(c) => self.whitespace(),

            // One-symbol tokens.
            ';' => SemiColon,
            ':' => Colon,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            ',' => Comma,
            '.' => Dot,
            '!' => Bang,
            '=' => Equal,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '>' => Greater,
            '<' => Less,
            '&' => And,
            '|' => Or,

            // Number Literal
            c @ '0'..='9' => {
                let literal_kind = self.number(c);
                Literal { kind: literal_kind }
            }
            _ => todo!(),
        };

        let token = Token::new(token_kind, self.position_within_token());
        self.reset_position_within_token();
        token
    }
}
