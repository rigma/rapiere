use crate::{error::Error, token::TokenKind};

pub(crate) struct Tokenizer;
pub(crate) type RawToken<'i> = (TokenKind, &'i [u8]);

impl Tokenizer {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize<'i>(&self, input: &'i [u8]) -> Result<(Option<RawToken<'i>>, usize), Error> {
        if input[0].is_ascii_whitespace() {
            return Ok((
                None,
                match input.iter().skip(1).position(|&b| !b.is_ascii_whitespace()) {
                    Some(pos) => pos + 1,
                    None => input.len(),
                },
            ));
        }

        match input[0] {
            b'!' => {
                if let Some(b) = input.get(1) {
                    if *b == b'=' {
                        Ok((Some((TokenKind::NotEquals, &input[..2])), 2))
                    } else {
                        Err(Error::UnrecognizedToken(None))
                    }
                } else {
                    Err(Error::UnrecognizedToken(None))
                }
            }
            b'(' => Ok((Some((TokenKind::LeftParenthesis, &input[..1])), 1)),
            b')' => Ok((Some((TokenKind::RightParenthesis, &input[..1])), 1)),
            b'>' => {
                if let Some(b) = input.get(1) {
                    if *b == b'=' {
                        Ok((Some((TokenKind::GreaterThanEquals, &input[..2])), 2))
                    } else {
                        Ok((Some((TokenKind::GreaterThan, &input[..1])), 1))
                    }
                } else {
                    Ok((Some((TokenKind::GreaterThan, &input[..1])), 1))
                }
            }
            b'<' => {
                if let Some(b) = input.get(1) {
                    if *b == b'=' {
                        Ok((Some((TokenKind::LesserThanEquals, &input[..2])), 2))
                    } else {
                        Ok((Some((TokenKind::LesserThan, &input[..1])), 1))
                    }
                } else {
                    Ok((Some((TokenKind::LesserThan, &input[..1])), 1))
                }
            }
            b'=' => Ok((Some((TokenKind::Equals, &input[..1])), 1)),
            b'-' => {
                if let Some(b) = input.get(1) {
                    if b.is_ascii_digit() {
                        number(input)
                    } else {
                        Ok((Some((TokenKind::Minus, &input[..1])), 1))
                    }
                } else {
                    Ok((Some((TokenKind::Minus, &input[..1])), 1))
                }
            }
            b'"' => string_literal(input),
            b':' => Ok((Some((TokenKind::Colon, &input[..1])), 1)),
            b'.' => {
                if let Some(b) = input.get(1) {
                    if b.is_ascii_digit() {
                        fractional_part(input, 0)
                    } else {
                        Ok((Some((TokenKind::Dot, &input[..1])), 1))
                    }
                } else {
                    Ok((Some((TokenKind::Dot, &input[..1])), 1))
                }
            }
            b',' => Ok((Some((TokenKind::Comma, &input[..1])), 1)),
            b'0'..=b'9' => number(input),
            b => {
                if let Some(token) = boolean(input) {
                    Ok(token)
                } else if let Some(token) = null(input) {
                    Ok(token)
                } else if let Some(token) = keyword(input) {
                    Ok(token)
                } else {
                    if is_identifier_byte(b) {
                        identifier(input)
                    } else {
                        Err(Error::UnrecognizedToken(None))
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn boolean(input: &[u8]) -> Option<(Option<RawToken<'_>>, usize)> {
    if &input[..4] == b"true" {
        Some((Some((TokenKind::True, &input[..4])), 4))
    } else if &input[..5] == b"false" {
        Some((Some((TokenKind::False, &input[..5])), 5))
    } else {
        None
    }
}

fn exponential_part(input: &[u8], position: usize) -> Result<(Option<RawToken<'_>>, usize), Error> {
    if let Some(b) = input.get(position + 1) {
        let position = if *b == b'+' || *b == b'-' {
            position + 1
        } else {
            position
        };

        if let Some((idx, b)) = find_end_of_number(input, position + 1, u8::is_ascii_digit)? {
            if is_identifier_byte(b) || idx == position + 1 {
                return Err(Error::BadNumber(None));
            }

            Ok((Some((TokenKind::Literal, &input[..idx])), idx))
        } else {
            if input.len() == position + 1 {
                return Err(Error::BadNumber(None));
            }

            Ok((Some((TokenKind::Literal, input)), input.len()))
        }
    } else {
        Err(Error::BadNumber(None))
    }
}

fn find_end_of_number(
    input: &[u8],
    position: usize,
    test: fn(&u8) -> bool,
) -> Result<Option<(usize, u8)>, Error> {
    for (idx, b) in input.iter().enumerate().skip(position) {
        if test(b) {
            continue;
        } else if *b == b'_' {
            if idx >= 1
                && input.get(idx - 1).map_or(false, test)
                && input.get(idx + 1).map_or(false, test)
            {
                continue;
            }

            return Err(Error::BadNumber(None));
        } else {
            return Ok(Some((idx, *b)));
        }
    }

    Ok(None)
}

fn fractional_part(input: &[u8], position: usize) -> Result<(Option<RawToken<'_>>, usize), Error> {
    if let Some((idx, b)) = find_end_of_number(input, position + 1, u8::is_ascii_digit)? {
        if b == b'E' || b == b'e' {
            exponential_part(input, position)
        } else {
            Ok((Some((TokenKind::Literal, &input[..idx])), idx))
        }
    } else {
        Ok((Some((TokenKind::Literal, input)), input.len()))
    }
}

fn hex_integer(input: &[u8]) -> Result<(Option<RawToken<'_>>, usize), Error> {
    if let Some((idx, b)) = find_end_of_number(input, 2, u8::is_ascii_hexdigit)? {
        if is_identifier_byte(b) || idx == 2 {
            return Err(Error::MalformatedHexNumber(None));
        }

        Ok((Some((TokenKind::Literal, &input[..idx])), idx))
    } else {
        if input.len() == 2 {
            return Err(Error::MalformatedHexNumber(None));
        }

        Ok((Some((TokenKind::Literal, input)), input.len()))
    }
}

fn identifier(input: &[u8]) -> Result<(Option<RawToken<'_>>, usize), Error> {
    let end = input.iter().skip(1).position(|&b| !is_identifier_byte(b));
    let idx = if let Some(end) = end {
        end + 1
    } else {
        input.len()
    };

    let word = &input[..idx];
    let kind = match word {
        b"AND" => TokenKind::And,
        b"OR" => TokenKind::Or,
        b"NOT" => TokenKind::Not,
        b"true" => TokenKind::True,
        b"false" => TokenKind::False,
        b"null" => TokenKind::Null,
        _ => TokenKind::Identifier,
    };

    Ok((Some((kind, word)), idx))
}

#[inline(always)]
fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte > b'\x7f' || byte == b'_'
}

#[inline(always)]
fn keyword(input: &[u8]) -> Option<(Option<RawToken<'_>>, usize)> {
    if &input[..2] == b"OR" {
        Some((Some((TokenKind::Or, &input[..2])), 2))
    } else if &input[..3] == b"AND" {
        Some((Some((TokenKind::And, &input[..3])), 3))
    } else if &input[..3] == b"NOT" {
        Some((Some((TokenKind::Not, &input[..3])), 3))
    } else {
        None
    }
}

#[inline(always)]
fn null(input: &[u8]) -> Option<(Option<RawToken<'_>>, usize)> {
    if &input[..4] == b"null" {
        Some((Some((TokenKind::Null, &input[..4])), 4))
    } else {
        None
    }
}

fn number(input: &[u8]) -> Result<(Option<RawToken<'_>>, usize), Error> {
    if input[0] == b'0' {
        if let Some(b) = input.get(1) {
            if *b == b'X' || *b == b'x' {
                return hex_integer(input);
            }
        } else {
            return Ok((Some((TokenKind::Literal, input)), input.len()));
        }
    }

    if let Some((idx, b)) = find_end_of_number(input, 1, u8::is_ascii_digit)? {
        if b == b'E' || b == b'e' {
            exponential_part(input, idx)
        } else if b == b'.' {
            fractional_part(input, idx)
        } else {
            Ok((Some((TokenKind::Literal, &input[..idx])), idx))
        }
    } else {
        Ok((Some((TokenKind::Literal, input)), input.len()))
    }
}

fn string_literal(input: &[u8]) -> Result<(Option<RawToken<'_>>, usize), Error> {
    let mut end = None;
    let mut previous = 0;

    for (idx, current) in input.iter().enumerate().skip(1) {
        // Escaped quote case
        if *current == b'"' && previous == *current {
            previous = 0;
            continue;
        } else if previous == b'"' {
            end = Some(idx);
            break;
        }

        previous = *current;
    }

    if end.is_some() || previous == b'"' {
        let idx = if let Some(end) = end {
            end
        } else {
            input.len()
        };

        Ok((Some((TokenKind::Literal, &input[..idx])), idx))
    } else {
        Err(Error::UnterminatedStringLiteral(None))
    }
}
