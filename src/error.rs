use std::error;
use std::fmt::{self, Display};

use serde;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<E>(e: E) -> Self
        where E: Into<String>
    {
        Error {
            msg: e.into(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.msg.fmt(formatter)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl serde::ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error {
            msg: msg.into(),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error {
            msg: msg.into(),
        }
    }

    fn end_of_stream() -> Self {
        Error {
            msg: "end of stream".to_string(),
        }
    }
}
