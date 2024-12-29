#[derive(Debug)]
pub enum Error {
    BadNumber(Option<(usize, usize)>),

    MalformatedHexNumber(Option<(usize, usize)>),

    UnrecognizedToken(Option<(usize, usize)>),

    UnterminatedStringLiteral(Option<(usize, usize)>),
}

impl Error {
    pub fn position(&self) -> (Option<usize>, Option<usize>) {
        #[inline(always)]
        fn unwrap_position(pos: &Option<(usize, usize)>) -> (Option<usize>, Option<usize>) {
            if let Some((line, column)) = pos {
                (Some(*line), Some(*column))
            } else {
                (None, None)
            }
        }

        match *self {
            Self::BadNumber(ref pos) => unwrap_position(pos),
            Self::MalformatedHexNumber(ref pos) => unwrap_position(pos),
            Self::UnrecognizedToken(ref pos) => unwrap_position(pos),
            Self::UnterminatedStringLiteral(ref pos) => unwrap_position(pos),
        }
    }

    pub fn set_position(&mut self, line: usize, column: usize) {
        match *self {
            Self::BadNumber(ref mut pos) => *pos = Some((line, column)),
            Self::MalformatedHexNumber(ref mut pos) => *pos = Some((line, column)),
            Self::UnrecognizedToken(ref mut pos) => *pos = Some((line, column)),
            Self::UnterminatedStringLiteral(ref mut pos) => *pos = Some((line, column)),
        }
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::BadNumber(_) => "bad number format",
            Self::MalformatedHexNumber(_) => "malformatted hexadecimal integer",
            Self::UnrecognizedToken(_) => "unrecognized token",
            Self::UnterminatedStringLiteral(_) => "non-terminated string literal",
        };
        let (line, column) = self.position();

        write!(
            f,
            "lexer error occured: {msg} (line: {line}, column: {column})",
            line = line.map_or_else(|| "unknown".to_owned(), |n| n.to_string()),
            column = column.map_or_else(|| "unknown".to_owned(), |n| n.to_string()),
        )
    }
}
