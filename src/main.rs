mod lexer;
mod matchers;
mod parser;
mod result;
mod tokens;
mod traits;
mod visitor;
//mod example_parser;

use result::{ParserError, ParserResult};
use traits::*;

use crate::tokens::Tokens;

Lexer!(
    {
        {'0'..='9' =>} => NUMBER(i64),
        {'A'..='Z' | 'a'..='z' =>} => IDENT(String),
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL(String),
        {'.'} => DOT,
        {'+'} => PLUS,
        {'-'} => MINUS,
        {'='} => EQUAL,
        {','} => COMMA,
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
    operator = {PLUS | MINUS | DIVISION | IDENT(String) },
    value = { NUMBER(i64) | TEXTLITERAL(String) | IDENT(String)},
    expression = (#value, #operator, #expressions),
    expressions = {#expression | #value},
    argument =  (IDENT(String), COMMA, #arguments),
    arguments = {#argument | IDENT(String)},

    function = (IDENT(String), BRACKETOPEN, #arguments, BRACKETCLOSE, EQUAL, #expressions),
    assignment = (IDENT(String), EQUAL, #expressions),
    statement = {#function | #assignment}
);

struct V;

#[derive(Debug)]
enum ASTVariants {
    Arguments(Vec<String>),
    Function(Function),
    None,
}

#[derive(Debug)]
struct Function {
    name: String,
    args: Vec<String>,
}

impl Visitor<ASTVariants> for V {
    fn arguments(&self, args: &arguments) -> ASTVariants {
        let mut vec: Vec<String> = vec![];

        let mut a = args;

        loop {
            match a {
                arguments::argument(ar) => {
                    vec.push(ar.0.0.clone());
                    a = &ar.2
                }
                arguments::IDENT(str) => {
                    vec.push(str.to_owned());
                    break;
                }
            }
        }

        ASTVariants::Arguments(vec)
    }

    fn function(&self, f: &function) -> ASTVariants {
        let name = f.0.0.to_owned();
        match f.2.visit(self) {
            ASTVariants::Arguments(args) => ASTVariants::Function(Function {
                name,
                args,
            }),
            _ => ASTVariants::None,
        }
    }

    fn statement(&self, statement: &statement) -> ASTVariants {
        match statement {
            statement::function(fun) => fun.visit(self),
            statement::assignment(_) => todo!(),
        }
    }
}

fn run() -> ParserResult<Box<statement>> {
    let a = "test(a, b) = a + b";

    // let a = "test = a";

    let t = Parser::new(a)?;

    let t = Parser::statement(&t.tokens)?;

    let v = V {};

    println!("{:?}", t.visit(&v));

    Ok(t)
}

fn main() {
    println!("{:?}", run());
}
