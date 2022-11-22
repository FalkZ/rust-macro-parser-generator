use super::{result::{ParserError, ParserResult}, sourcemap::{RenderContext, Token}, render::Render, position::{GetPosition, Position}};



pub trait OrMessage<V> {
    fn or_message(self, str: &str) -> ParserResult<V>;
}

impl<A> OrMessage<A> for Option<A> {
    fn or_message(self, message: &str) -> ParserResult<A> {
        match self {
            Some(v) => Ok(v),
            None => ParserError::error(message),
        }
    }
}

pub trait OrErr<V> {
    fn or_err(self) -> ParserResult<V>;
}

impl<A, E: Into<anyhow::Error>> OrErr<A> for Result<A, E> {
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



pub trait OrErrString<V> {
    fn or_err(self) -> ParserResult<V>;
}

 impl<A, E: Into<String>> OrErrString<A> for Result<A, E> {
    fn or_err(self) -> ParserResult<A> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => ParserError::error(&e.into()),
        }
    }
}




pub trait RawToken {
    fn raw_token(&self) -> Token;
}

impl <T: RawToken> GetPosition for T {
    fn position(&self)-> Position {
        self.raw_token().position()
    }
}

impl <T: RawToken> Render for T {
    fn render(&self, context: &mut RenderContext){
        let raw = self.raw_token();
        context.add_token(raw.as_str(), &raw.position)

    }  
}