use crate::result::{ParserResult, ParserError};

pub trait T<V> {
    fn or_message(self, str: &str) -> ParserResult<V>;
}

impl<A> T<A> for Option<A> {
    fn or_message(self, message: &str) -> ParserResult<A> {
        match self {
            Some(v) => Ok(v),
            None => ParserError::error(message),
        }
    }
}

pub trait T2<V> {
    fn or_err(self) -> ParserResult<V>;
}

impl<A, E: Into<anyhow::Error>> T2<A> for Result<A, E> {
    fn or_err(self) -> ParserResult<A> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let er : anyhow::Error = e.into();
                
                ParserError::error( &er.to_string())
            },
        }
    }
}


pub trait T3<V> {
    fn or_err(self) -> ParserResult<V>;
}

 impl<A, E: Into<String>> T3<A> for Result<A, E> {
    fn or_err(self) -> ParserResult<A> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => ParserError::error(&e.into()),
        }
    }
}
