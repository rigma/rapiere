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
    use super::*;
    use crate::token::{TokenKind, TokenValue};
    use rstest::rstest;

    #[rstest]
    #[case::and_keyword(b"AND", TokenKind::And, None)]
    #[case::or_keyword(b"OR", TokenKind::Or, None)]
    #[case::not_keyword(b"NOT", TokenKind::Not, None)]
    #[case::identifier(
        b"foo_bar",
        TokenKind::Identifier,
        Some(TokenValue::String("foo_bar".to_owned())),
    )]
    #[case::left_parenthesis(b"(", TokenKind::LeftParenthesis, None)]
    #[case::right_parenthesis(b")", TokenKind::RightParenthesis, None)]
    #[case::colon(b":", TokenKind::Colon, None)]
    #[case::comma(b",", TokenKind::Comma, None)]
    #[case::dot(b".", TokenKind::Dot, None)]
    #[case::minus(b"-", TokenKind::Minus, None)]
    #[case::equals(b"=", TokenKind::Equals, None)]
    #[case::not_equals(b"!=", TokenKind::NotEquals, None)]
    #[case::greater_than(b">", TokenKind::GreaterThan, None)]
    #[case::greater_than_equals(b">=", TokenKind::GreaterThanEquals, None)]
    #[case::lesser_than(b"<", TokenKind::LesserThan, None)]
    #[case::lesser_than_equals(b"<=", TokenKind::LesserThanEquals, None)]
    #[case::float_literal(b"3.1415", TokenKind::Literal, Some(TokenValue::Float(3.1415)))]
    #[case::negative_float_literal(b"-1.618", TokenKind::Literal, Some(TokenValue::Float(-1.618)))]
    #[case::scientific_notation_float_literal(
        b"1e30",
        TokenKind::Literal,
        Some(TokenValue::Float(1e30))
    )]
    #[case::scientific_notation_float_literal_with_capital_exponent(
        b"1E30",
        TokenKind::Literal,
        Some(TokenValue::Float(1e30))
    )]
    #[case::scientific_notation_with_fractional_part_float_literal(
        b"6.62607015e-34",
        TokenKind::Literal,
        Some(TokenValue::Float(6.62607015e-34))
    )]
    #[case::scientific_notation_with_fractional_part_float_literal_with_capital_exponent(
        b"6.62607015E-34",
        TokenKind::Literal,
        Some(TokenValue::Float(6.62607015e-34))
    )]
    #[case::integer_literal(b"42", TokenKind::Literal, Some(TokenValue::Integer(42)))]
    #[case::negative_integer_literal(b"-12", TokenKind::Literal, Some(TokenValue::Integer(-12)))]
    #[case::hex_integer_literal_1(b"0x2a", TokenKind::Literal, Some(TokenValue::Integer(42)))]
    #[case::hex_integer_literal_2(b"0x2A", TokenKind::Literal, Some(TokenValue::Integer(42)))]
    #[case::hex_integer_literal_3(b"0X2a", TokenKind::Literal, Some(TokenValue::Integer(42)))]
    #[case::hex_integer_literal_4(b"0X2A", TokenKind::Literal, Some(TokenValue::Integer(42)))]
    #[case::string_literal(
        b"\"hello world\"",
        TokenKind::Literal,
        Some(TokenValue::String("hello world".to_owned())),
    )]
    #[case::true_value(b"true", TokenKind::True, None)]
    #[case::false_value(b"false", TokenKind::False, None)]
    #[case::null_value(b"null", TokenKind::Null, None)]
    #[case::whitespace_token(b" ", TokenKind::Whitespace, None)]
    #[case::tab_whitespace(b"\t", TokenKind::Whitespace, None)]
    #[case::line_feed_whitespace(&[0xa], TokenKind::Whitespace, None)]
    #[case::form_feed_whitespace(&[0xd], TokenKind::Whitespace, None)]
    #[case::eof(b"", TokenKind::EOF, None)]
    fn it_parses_a_token(
        #[case] input: &[u8],
        #[case] expected_kind: TokenKind,
        #[case] expected_value: Option<TokenValue>,
    ) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert!(token.is_ok());

        let token = token.unwrap();
        assert!(token.is_some());

        let token = token.unwrap();
        assert_eq!(token.kind, expected_kind);
        assert_eq!(token.value, expected_value);
    }

    #[test]
    fn it_read_an_input_of_tokens() {
        let input = b" ():,.-=!=>>=<<=42\"hello world\"3.1415truefalsenullANDORNOTfoo_bar";
        let expected_tokens = vec![
            (TokenKind::Whitespace, None),
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
            (TokenKind::True, None),
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

            match (token.unwrap(), iterator.next()) {
                (Some(token), Some((expected_token, expected_value))) => {
                    assert_eq!(token.kind, *expected_token);
                    assert_eq!(token.value, *expected_value);
                }
                _ => break,
            }
        }
    }
}
