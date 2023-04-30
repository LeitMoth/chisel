use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Message(String),
    TrailingCharacters,
    Eof,
    ExpectedBoolean,
    ExpectedInteger,
    ExpectedString,
    BadStruct,
    BadParse,
    ExpectedFieldName,
    StructNameChanged,
    ExpectedClosingQuote,
    EndOfSequence,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::TrailingCharacters => todo!(),
            Error::Eof => todo!(),
            Error::ExpectedBoolean => todo!(),
            Error::ExpectedInteger => todo!(),
            Error::ExpectedString => todo!(),
            Error::BadStruct => todo!(),
            Error::BadParse => todo!(),
            Error::ExpectedFieldName => todo!(),
            Error::StructNameChanged => todo!(),
            Error::ExpectedClosingQuote => todo!(),
            Error::EndOfSequence => todo!(),
        }
    }
}

impl std::error::Error for Error {}
