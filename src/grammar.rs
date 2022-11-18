use crate::{Lexer, Parser};

use crate::result::{ParserError, ParserResult};
use crate::traits::*;

use crate::tokens::Tokens;

use crate::{mat, return_end_if_missmatch, return_if_match};


Lexer!(
    {
        {'0'..='9' =>} => NUMBER(String),
        {'A'..='Z' | 'a'..='z' =>} => IDENT(String),
        {'[' => ']'} => RAWIDENT(String),
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL(String),
        {'#' => '#'} => TYPESCRIPT(String),
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
    value = ( #function_call | #float | NUMBER(String) | TEXTLITERAL(String) | IDENT(String) | TYPESCRIPT(String)),

    path = [ IDENT(String) => path, DOT, * ],
    function_call = { #path => path, IDENT(String) => name, #calls => arguments },

    call =  [#body => arg,  COMMA, *],
    calls = {BRACKETOPEN, #call => arguments, #body => last,  BRACKETCLOSE},


    expressions = [ #operator => operator, #value => value, * ],
    body = {#value => value, #expressions => expressions},

    argument =  [IDENT(String) => arg,  COMMA, *],
    arguments = {BRACKETOPEN, #argument => arguments, IDENT(String) => last,  BRACKETCLOSE},

    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    name = (RAWIDENT(String) | IDENT(String)),

    function = { #modifiers => modifiers, #name => name, #maybe_arguments => arguments, EQUAL, #body => body, SEMI},
    variable = { #modifiers => modifiers, IDENT(String) => name, EQUAL, #body => body, SEMI},
    statement = (#function | #variable),
    statements = [#statement => statement,  *]
);
