mod lexer;
mod parser;
mod result;
mod traits;
mod tokens;

use core::slice::Iter;

use result::{ParserResult, ParserError};
use traits::*;

use crate::tokens::Tokens;


Lexer!(
    {
        {'0'..='9' =>} => NUMBER(i64),
        {'A'..='z' =>} => IDENT(String),
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL(String),
        {'.'} => DOT,
        {'+'} => PLUS,
        {'-'} => MINUS,
        {'/'} => DIVISION,
        {'*'} => MULTIPLICATION,
        {'('} => BRACKETOPEN,
        {')'} => BRACKETCLOSE,
        {'{'} => CBRACKETOPEN,
        {'}'} => CBRACKETCLOSE
    }

    { ' ' | '\n' } => _

    {
        IDENT => { {"^"} => POWER }
       // NUMBER => { { 123 } => T123(i64) }
    }
);


// op = PLUS | MINUS | POWER | DIV
// _term3 = term | NUMBER(i64)
// term = NUMBER(i64) op _term3

Parser!({PLUS, MINUS, POWER, DIV,  NUMBER(i64)}
{
    op = { PLUS | MINUS | POWER | DIV },
    _term3 = { #term | NUMBER(i64) },
    term = (NUMBER(i64), #op, #_term3)
}
);

macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target { // #1
                a
            } else {
                panic!(
                    "mismatch variant when cast to {}", 
                    stringify!($pat)); // #2
            }
        }
    };
}

macro_rules! eat {
    ($self:ident, $pattern : pat) => {
        let next = $self.next().or_message("next on EOF")?;

    
        match &next {
            $pattern => Ok(n),            
            _ => Err(ParserError::Unreachable)
        }

    };
}


macro_rules! eat2 {
    ($self:ident, $name:ident ($type:ty)) => {
        {
            let next = $self.next().or_message("next on EOF")?;
     
            match &next {
                Lexer::$name (val) => {

                    Ok($name(*val))
                },            
                _ => Err(ParserError::Mismatch)
            }
        }
    };
}

macro_rules! eat3 {
    ($self:ident, $name:ident ($type:ty)) => {
        {
            let next = $self.peek().or_message("next on EOF")?;
     
            match &next {
                Lexer::$name (val) => {
                    $self.next();
                    Ok($name(*val))
                },            
                _ => Err(ParserError::Mismatch)
            }
        }
    };
}

macro_rules! mat {
    ($self:ident, $name:ident, $name_out:path) => {
        {
            let next = $self.peek().or_message("next on EOF")?;

            println!("match token {:?} to {:?}", next, stringify!($name));
     
            match &next {
                Lexer::$name => {
                    $self.next();
                    return Ok(Box::new($name_out));
                },            
                _ => ()
            }
        }
    };
    ($self:ident, $name:ident (), $name_out:path) => {
        {
            let next = $self.peek().or_message("next on EOF")?;

            println!("match token {:?} to {:?}", next, stringify!($name));
     
            match &next {
                Lexer::$name (val) => {
                    $self.next();
                    return Ok(Box::new(($name_out(*val))));
                },            
                _ => ()
            }
        }
    };

    ($pinned_tokens:ident, #$name:ident, $name_out:path) => {
        {
            match Self::$name($pinned_tokens.get_pinned()) {
                Ok(val) => return Ok(Box::new($name_out(val))),
                _ => (),
            };
        }
    };
}

macro_rules! peek {
    ($self:ident, $($pattern : tt)+) => {
        {
            let next = $self.peek()?;
            if(!matches!(&next, &$($pattern)+)){
                return Mismatch;
            }
        }
    };
}



#[derive(Debug)]
struct Parser {
    tokens: Tokens<Lexer>,
}

impl Parser {
    fn new(str: &str) -> ParserResult<Self> {
        let tokens = Lexer::lex(str).or_err()?;

        
        return Ok(Self {
            tokens: Tokens::new(tokens),
        });
    }

    fn _term3(tokens: & Tokens<Lexer>) -> ParserResult<Box<_term3>> {

        let pin = tokens.pin();
       
        mat!(pin, #term, _term3::term);

        let a = pin.get_pinned();
        mat!(a, NUMBER(), _term3::NUMBER);
        Err(ParserError::UnreachableAt("parse_term".to_string()))
    }

    fn op(tokens: & Tokens<Lexer>) -> ParserResult<Box<op>> {
         mat!(tokens, PLUS, op::PLUS);
         mat!(tokens, MINUS, op::MINUS);

         Err(ParserError::UnreachableAt("parse_op".to_string()))
    }

    fn term(tokens: & Tokens<Lexer>) -> ParserResult<Box<term>> {
        println!("parse_term: {:?}", tokens);
        let r = term(
        eat3!(tokens, NUMBER(n))?,
        Self::op(tokens)?,
        Self::_term3(tokens)?
        );
     

        Ok(Box::new(r))
    }

}





fn run() -> ParserResult<Box<term>> {
    let a = "123 \n - 345 add 12 'text' ^";

    let b = "123 \n - 345 + 12 ";

    let t = Parser::new(b)?;

    Parser::term(&t.tokens)
}

fn main() {

    let a = term(NUMBER(123),Box::new(op::MINUS), Box::new(_term3::NUMBER(123)));


    let b: i64 = a.0.0;

    let c = *a.2;
    let d = match c {
        _term3::NUMBER(val) => {

        }
        _ => {}
    };

    println!("{:?}", run());
}
