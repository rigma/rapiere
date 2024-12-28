use chrono::{DateTime, FixedOffset, TimeDelta, TimeZone};
use std::{borrow::Cow, fmt};

#[derive(Clone, Debug, Default)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub value: Option<TokenValue>,
}

impl Token {
    #[inline(always)]
    pub fn new(kind: TokenKind, line: usize, column: usize, length: usize) -> Self {
        Self {
            kind,
            line,
            column,
            length,
            value: None,
        }
    }

    #[inline]
    pub fn with_value(mut self, value: impl Into<TokenValue>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl Eq for Token {}

impl PartialEq for Token {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

impl fmt::Display for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.kind, &self.value) {
            (TokenKind::Identifier, Some(value)) => write!(f, "Token::ID({value})"),
            (TokenKind::Literal, Some(TokenValue::DateTime(value))) => {
                write!(f, "Token::DateTime({})", value.to_rfc3339())
            }
            (TokenKind::Literal, Some(TokenValue::Duration(value))) => {
                let duration = value.num_seconds() as f32;
                let duration = duration + (value.num_seconds() as f32 / 1e9);

                write!(f, "Token::Duration({duration}s)")
            }
            (TokenKind::Literal, Some(TokenValue::Float(value))) => {
                write!(f, "Token::Float({value})")
            }
            (TokenKind::Literal, Some(TokenValue::Integer(value))) => {
                write!(f, "Token::Integer({value})")
            }
            (TokenKind::Literal, Some(TokenValue::String(value))) => {
                write!(f, "Token::String({value})")
            }
            (kind, _) => write!(f, "Token::{kind}"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TokenKind {
    /// (
    LeftParenthesis,

    /// )
    RightParenthesis,

    /// :
    Colon,

    /// ,
    Comma,

    /// .
    Dot,

    /// -
    Minus,

    /// =
    Equals,

    /// !=
    NotEquals,

    /// >
    GreaterThan,

    /// >=
    GreaterThanEquals,

    /// <
    LesserThan,

    /// <=
    LesserThanEquals,

    /// Identifier
    Identifier,

    /// Literal
    Literal,

    /// AND keyword
    And,

    /// OR keyword
    Or,

    /// NOT keyword
    Not,

    /// true
    True,

    /// false
    False,

    /// null
    Null,

    /// End of file
    EOF,

    /// Undefined
    #[default]
    Undefined,
}

impl fmt::Display for TokenKind {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LeftParenthesis => write!(f, "LeftParenthesis"),
            Self::RightParenthesis => write!(f, "RightParenthesis"),
            Self::Colon => write!(f, "Colon"),
            Self::Comma => write!(f, "Comma"),
            Self::Dot => write!(f, "Dot"),
            Self::Minus => write!(f, "Minus"),
            Self::Equals => write!(f, "Equals"),
            Self::NotEquals => write!(f, "NotEquals"),
            Self::GreaterThan => write!(f, "GreaterThan"),
            Self::GreaterThanEquals => write!(f, "GreaterThanEquals"),
            Self::LesserThan => write!(f, "LesserThan"),
            Self::LesserThanEquals => write!(f, "LesserThanEquals"),
            Self::Identifier => write!(f, "Identifier"),
            Self::Literal => write!(f, "Literal"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Not => write!(f, "Not"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::Null => write!(f, "Null"),
            Self::EOF => write!(f, "EOF"),
            Self::Undefined => write!(f, "Undefined"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenValue {
    DateTime(DateTime<FixedOffset>),
    Duration(TimeDelta),
    Float(f32),
    Integer(i64),
    String(String),
}

impl From<Cow<'_, str>> for TokenValue {
    fn from(value: Cow<'_, str>) -> Self {
        Self::String(value.to_string())
    }
}

impl<T> From<DateTime<T>> for TokenValue
where
    T: TimeZone,
{
    #[inline(always)]
    fn from(value: DateTime<T>) -> Self {
        Self::DateTime(value.fixed_offset())
    }
}

impl From<String> for TokenValue {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<TimeDelta> for TokenValue {
    #[inline(always)]
    fn from(value: TimeDelta) -> Self {
        Self::Duration(value)
    }
}

impl From<f32> for TokenValue {
    #[inline(always)]
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<i64> for TokenValue {
    #[inline(always)]
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<&str> for TokenValue {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<&[u8]> for TokenValue {
    fn from(value: &[u8]) -> Self {
        let value = String::from_utf8_lossy(value);

        if let Ok(value) = value.parse::<f32>() {
            return Self::Float(value);
        }

        if &value[..2] == "0X" || &value[..2] == "0x" {
            if let Ok(value) = i64::from_str_radix(&value[2..], 16) {
                Self::Integer(value)
            } else {
                Self::String(value.to_string())
            }
        } else {
            if let Ok(value) = value.parse::<i64>() {
                Self::Integer(value)
            } else {
                Self::String(value.to_string())
            }
        }
    }
}

impl fmt::Display for TokenValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DateTime(value) => write!(f, "{}", value.to_rfc3339()),
            Self::Duration(value) => {
                let duration = value.num_seconds() as f32;
                let duration = duration + (value.subsec_nanos() as f32 / 1e9);

                write!(f, "{duration}s")
            }
            Self::Float(value) => write!(f, "{value}"),
            Self::Integer(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{value}"),
        }
    }
}
