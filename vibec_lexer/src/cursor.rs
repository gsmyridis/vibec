use std::str::Chars;

pub const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    pub len_remaining: usize,
    pub chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            len_remaining: input.len(),
            chars: input.chars(),
        }
    }

    /// Returns the remaining input stream as str.
    pub(crate) fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    /// Peeks the next symbol from the input stream without consuming it.
    ///
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    #[inline]
    pub(crate) fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Peeks the second next symbol from the input strea without consuming it.
    ///
    /// If requested position doesn't exist, `EOF_CHAR` is returned.
    pub(crate) fn peek_second(&self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    /// Checks if there is nothign more to consume, i.e. reached the end-of-file.
    #[inline]
    pub(crate) fn is_eof(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Returns the next character of the input stream.
    #[inline]
    pub(crate) fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Goes to the next character while the predicate returns true or the end-of-file has
    /// been reached.
    #[inline]
    pub(crate) fn next_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.chars.next();
        }
    }

    /// Returns the number of bytes already consumed for the current token.
    #[inline]
    pub(crate) fn position_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    /// Resets the number of bytes consumed to 0.
    #[inline]
    pub(crate) fn reset_position_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }
}
