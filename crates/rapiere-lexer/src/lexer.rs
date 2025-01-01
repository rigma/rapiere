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

#[cfg(test)]
mod tests {
    pub use super::*;
    pub use crate::token::{TokenKind, TokenValue};

    #[test]
    fn it_parses_raw_input_into_tokens() {
        let input = b"():,.-=!=>>=<<=42\"hello world\"3.1415-1.618true-12falsenullANDORNOTfoo_bar";
        let expected_tokens = vec![
            (TokenKind::LeftParenthesis, None),
            (TokenKind::RightParenthesis, None),
            (TokenKind::Colon, None),
            (TokenKind::Comma, None),
            (TokenKind::Dot, None),
            (TokenKind::Minus, None),
            (TokenKind::Equals, None),
            (TokenKind::NotEquals, None),
            (TokenKind::GreaterThan, None),
            (TokenKind::GreaterThanEquals, None),
            (TokenKind::LesserThan, None),
            (TokenKind::LesserThanEquals, None),
            (TokenKind::Literal, Some(TokenValue::Integer(42))),
            (
                TokenKind::Literal,
                Some(TokenValue::String("hello world".to_owned())),
            ),
            (TokenKind::Literal, Some(TokenValue::Float(3.1415))),
            (TokenKind::Literal, Some(TokenValue::Float(-1.618))),
            (TokenKind::True, None),
            (TokenKind::Literal, Some(TokenValue::Integer(-12))),
            (TokenKind::False, None),
            (TokenKind::Null, None),
            (TokenKind::And, None),
            (TokenKind::Or, None),
            (TokenKind::Not, None),
            (
                TokenKind::Identifier,
                Some(TokenValue::String("foo_bar".to_owned())),
            ),
            (TokenKind::EOF, None),
        ];

        let mut lexer = Lexer::new(input);
        let mut iterator = expected_tokens.iter();

        loop {
            let token = lexer.next_token();
            assert!(token.is_ok());
            let token = token.unwrap();

            match (token, iterator.next()) {
                (Some(token), Some((expected_token, expected_value))) => {
                    assert_eq!(token.kind, *expected_token);
                    assert_eq!(token.value, *expected_value);
                }
                _ => break,
            }
        }
    }

    #[test]
    fn it_ignores_ascii_whitespaces() {
        let input = [0x9, 0xa, 0xc, b'\n', b' '];

        let mut lexer = Lexer::new(&input);
        let token = lexer.next_token();
        assert!(token.is_ok());

        let token = token.unwrap();
        assert!(token.is_some());

        let token = token.unwrap();
        assert_eq!(token.kind, TokenKind::EOF);
    }
}
