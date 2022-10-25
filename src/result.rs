use std::error::Error;
use std::fmt;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Mismatch,
    Err(anyhow::Error),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        "Error while parsing"
    }

    fn cause(&self) -> Option<&dyn Error> {
        match &self {
            ParserError::Err(e) => Some(self),
            _ => None,
        }
    }
}

impl ParserError {
    pub fn error<T>(message: &str) -> ParserResult<T> {
        let err = anyhow::anyhow!("{}", message);
        Err(ParserError::Err(err))
    }
}
