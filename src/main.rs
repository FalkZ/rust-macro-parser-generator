mod lexer;
mod parser;
mod result;
mod traits;
mod tokens;
mod matchers;
mod visitor;
mod example_parser;


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
        //NUMBER => { { 123 } => T123(i64) }
    }
);


// op = PLUS | MINUS | POWER | DIV
// _term3 = term | NUMBER(i64)
// term = NUMBER(i64) op _term3

Parser!(
    op = { PLUS | MINUS | POWER },
    _term3 = { #term | NUMBER(i64) },
    term = (NUMBER(i64), #op, #_term3)
);


struct V;

impl Visitor<()> for V {
   fn term(&self, t: &term) -> () {

 
    let mut last = *t.2.clone();

    println!("op: {:?}", t.1);

    while let _term3::term(t0) = last {
        println!("op: {:?}", t0.1);
        last = *t0.2;
    }
    
    

      
   }
  
}

fn run() -> ParserResult<Box<term>> {
    let a = "123 \n - 345 add 12 'text' ^";

    let b = "123 \n - 345 + 12 ";

    let t = Parser::new(b)?;

    let t = Parser::term(&t.tokens)?;

    let v = V{};

    t.visit(v);

    Ok(t)
}



fn main() {
    println!("{:?}", run());
}
