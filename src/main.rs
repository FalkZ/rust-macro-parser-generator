mod lexer;
mod matchers;
mod parser;
mod result;
mod tokens;
mod traits;
mod file;
mod renderer;
mod visitor;

use file::compile_file;
use result::{ParserError, ParserResult};
use traits::*;

use crate::tokens::Tokens;

Lexer!(
    {
        {'0'..='9' =>} => NUMBER(String),
        {'A'..='Z' | 'a'..='z' =>} => IDENT(String),
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL(String),
        {';'} => SEMI,
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
        IDENT => {
            {"if"} => IF,
            {"mut"} => MUT,
            {"pub"} => PUB,
            {"cr"} => CR
        }
    }
);



Parser!(
    operator = (PLUS | MINUS | DIVISION | IDENT(String) ),

    modifier = ( MUT | PUB | CR ),
    modifiers = [#modifier => modifier, *],

    float = {NUMBER(String) => whole, DOT, NUMBER(String) => float},
    value = ( #float | NUMBER(String) | TEXTLITERAL(String) | IDENT(String)),

    expressions = [ #value => value, #operator => operator, * ],
    ex = {#expressions => ex, #value => v, SEMI},

    argument =  [IDENT(String) => arg,  COMMA, *],
    arguments = {BRACKETOPEN, #argument => arguments, IDENT(String) => last,  BRACKETCLOSE},

    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    function = { #modifiers => modifiers, IDENT(String) => name, #maybe_arguments => arguments, EQUAL, #ex => body},
    variable = { #modifiers => modifiers, IDENT(String) => name, EQUAL, #ex => body },
    statement = (#function | #variable),
    statements = [#statement => statement,  *]
);



fn main() {
    compile_file("./examples/Class.m1n");
    compile_file("./examples/file.m1n");
}
