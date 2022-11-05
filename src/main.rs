mod lexer;
mod parser;
mod result;
mod traits;
mod tokens;
mod matchers;

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
    op = { PLUS | MINUS | POWER },
    _term3 = { #term | NUMBER(i64) },
    term = (NUMBER(i64), #op, #_term3)
}
);







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
       
        let a = mat!(pin.get_pinned(), #term, _term3::term);
        return_if_match!(a);
        let a = mat!(pin.get_pinned(), NUMBER(i32), _term3::NUMBER);
        return_if_match!(a);

        Err(ParserError::UnreachableAt("parse_term".to_string()))
    }

    fn op(tokens: & Tokens<Lexer>) -> ParserResult<Box<op>> {
        
        let pin = tokens.pin();

        let a = mat!(pin.get_pinned(), PLUS, op::PLUS);
        return_if_match!(a);
        let a = mat!(pin.get_pinned(), MINUS, op::MINUS);
        return_if_match!(a);

         Err(ParserError::UnreachableAt("parse_op".to_string()))
    }

    fn term(tokens: & Tokens<Lexer>) -> ParserResult<Box<term>> {
        println!("parse_term: {:?}", tokens);
        let r = term(
        mat!(tokens, NUMBER(i32), NUMBER)?,
        mat!(tokens, #op)?,
        mat!(tokens, #_term3)?,
        );
     

        Ok(Box::new(r))
    }

}





fn run() -> ParserResult<Box<term>> {
    let a = "123 \n - 345 add 12 'text' ^";

    let b = "123 \n - 345 + 12 ";

    let t = Parser2::new(b)?;

    Parser2::term(&t.tokens)
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
