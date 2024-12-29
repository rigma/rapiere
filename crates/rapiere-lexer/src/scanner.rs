use crate::{
    error::Error,
    token::{Token, TokenKind},
    tokenizer::Tokenizer,
};
use std::fmt;

pub struct Scanner {
    column: usize,
    line: usize,
    mark: (usize, usize, usize),
    offset: usize,
    tokenizer: Tokenizer,
}

impl Scanner {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            column: 1,
            line: 1,
            mark: (0, 0, 0),
            offset: 0,
            tokenizer: Tokenizer::new(),
        }
    }

    #[inline(always)]
    pub fn column(&self) -> usize {
        self.column
    }

    #[inline]
    fn consume(&mut self, input: &[u8], amount: usize) {
        for b in &input[..amount] {
            if *b == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        self.offset += amount;
    }

    #[inline(always)]
    pub fn line(&self) -> usize {
        self.line
    }

    #[inline(always)]
    pub fn mark_current_position(&mut self) {
        self.mark = (self.offset, self.line, self.column);
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.column = 1;
        self.line = 1;
        self.mark = (0, 0, 0);
        self.offset = 0;
    }

    #[inline(always)]
    pub fn reset_to_mark(&mut self) {
        (self.offset, self.line, self.column) = self.mark;
    }

    pub fn scan(&mut self, input: &[u8]) -> Result<Option<Token>, Error> {
        loop {
            // We have reached the input's end, nothing more to do
            if self.offset >= input.len() {
                return Ok(None);
            }

            let window = &input[self.offset..];
            match self.tokenizer.tokenize(window) {
                Ok((Some((kind, word)), length)) => {
                    self.consume(input, length);

                    let token = match kind {
                        TokenKind::Undefined => {
                            unreachable!("undefined token should result into an error");
                        }
                        TokenKind::Identifier => Token::new(kind, self.line, self.column, length)
                            .with_value(String::from_utf8_lossy(word)),
                        TokenKind::Literal => {
                            Token::new(kind, self.line, self.column, length).with_value(word)
                        }
                        _ => Token::new(kind, self.line, self.column, length),
                    };

                    return Ok(Some(token));
                }
                Ok((None, length)) => {
                    // If a length is returned, we'll consume it and start another loop turn
                    if length > 0 {
                        self.consume(input, length);
                        continue;
                    }
                }
                Err(mut err) => {
                    err.set_position(self.line, self.column);

                    return Err(err);
                }
            }

            return Ok(None);
        }
    }
}

impl fmt::Debug for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scanner")
            .field("column", &self.column)
            .field("line", &self.line)
            .field("mark", &self.mark)
            .field("offset", &self.offset)
            .finish()
    }
}
