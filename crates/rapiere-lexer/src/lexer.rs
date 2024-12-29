use crate::{
    error::Error,
    scanner::Scanner,
    token::{Token, TokenKind},
};
use std::fmt;

#[derive(Debug)]
pub struct Lexer<'i> {
    eof: bool,
    input: &'i [u8],
    scanner: Scanner,
}

impl<'i> Lexer<'i> {
    #[inline(always)]
    pub fn new(input: &'i [u8]) -> Self {
        Self {
            eof: false,
            input,
            scanner: Scanner::new(),
        }
    }

    #[inline(always)]
    pub fn column(&self) -> u64 {
        self.scanner.column()
    }

    #[inline(always)]
    pub fn line(&self) -> u64 {
        self.scanner.line()
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, Error> {
        if self.eof {
            return Ok(None);
        }

        let token = if let Some(token) = self.scanner.scan(self.input)? {
            token
        } else {
            self.eof = true;

            Token::new(
                TokenKind::EOF,
                self.scanner.line(),
                self.scanner.column(),
                0,
            )
        };

        Ok(Some(token))
    }

    #[inline(always)]
    pub fn reset(&mut self, input: &'i [u8]) {
        self.eof = false;
        self.input = input;
        self.scanner.reset();
    }
}

impl<'i> From<&'i [u8]> for Lexer<'i> {
    #[inline(always)]
    fn from(value: &'i [u8]) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for Lexer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer(line = {}, column = {}, eof = {})",
            self.scanner.line(),
            self.scanner.column(),
            self.eof
        )
    }
}
