use crate::{Lexer, Parser};

Lexer!(
    {
        {'0'..='9' =>} => NUMBER,

        {'A'..='Z' | 'a'..='z' =>} => IDENT,
        {'`' => '`'} => RAWIDENT,
        {'"' | '\'' => '"' | '\''} => TEXTLITERAL,
        {'#' => '#'} => TYPESCRIPT,
        {'_'} => UNDERLINE,
        {';'} => SEMI,
        {':'} => COL,
        {'.'} => DOT,
        {'+'} => PLUS,
        {'-'} => MINUS,
        {'='} => EQUAL,
        {'>'} => MORETHAN,
        {','} => COMMA,
        {'/'} => DIVISION,
        {'|'} => PIPE,
        {'*'} => MULTIPLICATION,
        {'('} => BRACKETOPEN,
        {')'} => BRACKETCLOSE,
        {'{'} => CBRACKETOPEN,
        {'}'} => CBRACKETCLOSE,
        {' '} => SPACE,
        {'\n'} => NEWLINE
    }

    ( SPACE | NEWLINE ) => _

    {
        IDENT => {
            {"if"} => IF,
            {"match"} => MATCH,
            {"import"} => IMPORT,
            {"mut"} => MUT,
            {"pub"} => PUB,
            {"cr"} => CR
        }
    }
);

Parser!(

    equals = {EQUAL, EQUAL},
    operator = (#assignment | #equals | PLUS | MINUS | DIVISION | IDENT  ),

    modifier = ( MUT | PUB | CR ),
    modifiers = [#modifier => modifier, *],

    arrow = {MINUS, MORETHAN},
    assignment = {EQUAL, MORETHAN},

    match_arm = [ #operator => operator, #primitive_value => value, #arrow, #body => body, (COMMA)*],
    match_statement = {BRACKETOPEN, *match_arm => statements, BRACKETCLOSE},

    negative = { MINUS },
    number = { ?negative => negative, NUMBER => whole},
    float = {#number => whole, DOT, NUMBER => float},
    primitive_value = ( #float | #number | TEXTLITERAL | IDENT | TYPESCRIPT ),
    value = ( #function_call  | UNDERLINE | #primitive_value | #bracket_expression),



    path = [ IDENT => path, DOT, * ],
    function_call = { *path => path, IDENT => name, BRACKETOPEN, *calls => arguments, BRACKETCLOSE },

    calls =  [#body => argument, (COMMA) *],

    binary_operation = { #operator => operator, #value => value },
    match_operation = { MATCH, #match_statement => body },
    assingment_operation = { #assignment, IDENT => identifier },

    expression = ( #match_operation | #assingment_operation | #binary_operation ),
    expressions = [ #newline_expression => expression, * ],
    body = {#value => value, ?expression => first, *expressions => expressions },

    newline_expression = {_ NEWLINE, #expression => expressions },
    bracket_expression = { BRACKETOPEN, #body => expressions,  BRACKETCLOSE },


    argument =  [IDENT => arg, COMMA, *],
    arguments = {*argument => arguments, IDENT => last},

    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    name = (RAWIDENT | IDENT),

    function = { *modifiers => modifiers, #name => name, BRACKETOPEN, ?arguments => arguments, BRACKETCLOSE, EQUAL, #body => body, SEMI},
    variable = { *modifiers => modifiers, IDENT => name, EQUAL, #body => body, SEMI},

    definition = {IDENT => name, COL, *calls=>arguments, SEMI},

    import_item = [ #name => item, (COMMA) *],
    import_items = { CBRACKETOPEN, *import_item=>items, CBRACKETCLOSE },
    import = [ RAWIDENT=>path, ?import_items=>import_items, (COMMA) * ],
    imports = { IMPORT, COL, *import=>imports, SEMI },

    enum_statement = (#function | #variable),
    enum_statements = [#enum_statement => statement,  *],
    enum_version = { PIPE, IDENT => name, BRACKETOPEN, *enum_statements => statements, BRACKETCLOSE, SEMI },

    statement = (#imports | #definition | #enum_version | #function | #variable),
    statements = [#statement => statement,  *]
);
