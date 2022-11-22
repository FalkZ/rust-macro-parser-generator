use crate::{Lexer, Parser};

Lexer!(
    {
        {'0'..='9' =>} => NUMBER,
        {'A'..='Z' | 'a'..='z' =>} => IDENT,
        {'[' => ']'} => RAWIDENT,
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL,
        {'#' => '#'} => TYPESCRIPT,
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
    operator = (PLUS | MINUS | DIVISION | IDENT ),

    modifier = ( MUT | PUB | CR ),
    modifiers = [#modifier => modifier, *],

    float = {NUMBER => whole, DOT, NUMBER => float},
    value = ( #function_call | #float | NUMBER | TEXTLITERAL | IDENT | TYPESCRIPT ),

    path = [ IDENT => path, DOT, * ],
    function_call = { *path => path, IDENT => name, #calls => arguments },

    call =  [#body => arg,  COMMA, *],
    calls = {BRACKETOPEN, *call => arguments, #body => last,  BRACKETCLOSE},


    expressions = [ #operator => operator, #value => value, * ],
    body = {#value => value, *expressions => expressions},

    argument =  [IDENT => arg,  COMMA, *],
    arguments = {BRACKETOPEN, *argument => arguments, IDENT => last,  BRACKETCLOSE},

    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    name = (RAWIDENT | IDENT),

    function = { *modifiers => modifiers, #name => name, #maybe_arguments => arguments, EQUAL, #body => body, SEMI},
    variable = { *modifiers => modifiers, IDENT => name, EQUAL, #body => body, SEMI},
    statement = (#function | #variable),
    statements = [#statement => statement,  *]
);
