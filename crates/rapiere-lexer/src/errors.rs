use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("parsing error has occured during token scanning: {error} (l. {line}, c. {column})")]
    ScannerError {
        #[source]
        error: TokenizerError,
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("bad number format")]
    BadNumber,

    #[error("malformatted hexadecimal integer")]
    MalformatedHexNumber,

    #[error("unrecognized token")]
    UnrecognizedToken,

    #[error("non-terminated string literal")]
    UnterminatedStringLiteral,
}
