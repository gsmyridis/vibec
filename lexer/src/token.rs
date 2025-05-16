#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32,
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, len: u32) -> Self {
        Self { kind, len }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// An identifier or keyword, e.g. 'ident' or 'continue'.
    Ident,

    /// Any whitespace character sequence.
    Whitespace,

    /// Literal values
    Literal { kind: LiteralKind },

    /// '!'
    Bang,
    /// ':'
    Colon,
    /// ';'
    SemiColon,
    /// '('
    OpenParen,
    /// ')'
    CloseParen,
    /// '{'
    OpenBrace,
    /// '}'
    CloseBrace,
    /// ','
    Comma,
    /// '.'
    Dot,
    /// '-'
    Minus,
    /// '+'
    Plus,
    /// '*'
    Star,
    /// '/'
    Slash,
    /// '='
    Equal,
    /// '<'
    Less,
    /// '>'
    Greater,
    /// '=='
    EqualEqual,
    /// '<='
    LessEqual,
    /// '>='
    GreaterEqual,
    /// '!='
    BangEqual,
    /// '&'
    And,
    /// '|'
    Or,

    /// End of input.
    EoF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    /// `10`, `0b101`, `0o81`, `0x1A`
    Int { base: Base, empty_int: bool },

    /// `10.1`
    Float { base: Base, empty_exponent: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base {
    /// Literal starts with "0b"
    Binary = 2,
    /// Literal starts with "0o"
    Octal = 8,
    /// Literal doesn't have a prefix
    Decimal = 10,
    /// Literal starts with "0x"
    Hexadecimal = 16,
}
