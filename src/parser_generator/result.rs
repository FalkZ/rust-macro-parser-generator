use std::error::Error;
use std::fmt;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub enum ParserError {
    Mismatch,
    Unreachable,
    UnreachableAt(String),
    Err(TextError),
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
            ParserError::Err(_) => Some(self),
            _ => None,
        }
    }
}


#[derive(Debug, Clone)]
pub struct TextError {
    message: String
}


impl Error for TextError {
    fn description(&self) -> &str {
       &self.message
    }

    fn cause(&self) -> Option<&dyn Error> {
        None 
    }
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}


impl ParserError {
    pub fn error<T>(message: &str) -> ParserResult<T> {
        let err = TextError{ message: message.to_string() };
        Err(ParserError::Err(err))
    }
}


