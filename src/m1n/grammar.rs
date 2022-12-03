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
        {'!'} => NOT,
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

    // operators
    equals = {EQUAL, EQUAL},
    operator = ( #equals | PLUS | MINUS | DIVISION | IDENT  ),
    unary_operator = ( NOT | IDENT  ),

    modifier = ( MUT | PUB | CR ),
    modifiers = [#modifier => modifier, *],

    arrow = {MINUS, MORETHAN},
    assignment = {EQUAL, MORETHAN},

    match_arm = [ #operator => operator, #primitive_value => value, #arrow, #body => body, (COMMA)*],
    match_statement = {BRACKETOPEN, *match_arm => statements, BRACKETCLOSE},

    // Values
    negative = { MINUS },
    number = { ?negative => negative, NUMBER => whole},
    float = {#number => whole, DOT, NUMBER => float},
    primitive_value = ( #float | #number | TEXTLITERAL | IDENT | TYPESCRIPT ),
    value = ( #function_call  | UNDERLINE | #primitive_value | #bracket_expression),


    // Function calls
    path = [ IDENT => path, DOT, * ],
    calls =  [#body => argument, (COMMA) *],
    function_call = { *path => path, IDENT => name, BRACKETOPEN, *calls => arguments, BRACKETCLOSE },



    // Operators
    binary_operation = { #operator => operator, #value => value },
    unary_operation = { #unary_operator => operator },
    match_operation = { MATCH, #match_statement => body },
    assingment_operation = { #assignment, IDENT => identifier },

    expression = ( #match_operation | #assingment_operation | #binary_operation  | #unary_operation ),
    expressions = [ #newline_expression => expression, * ],
    body = {#value => value, ?expression => first, *expressions => expressions },

    newline_expression = {_ NEWLINE, #expression => expressions },
    bracket_expression = { BRACKETOPEN, #body => expressions,  BRACKETCLOSE },


    // Arguments
    argument =  [IDENT => arg, COMMA, *],
    arguments = {*argument => arguments, IDENT => last},
    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    name = (RAWIDENT | IDENT),

    function = { *modifiers => modifiers, #name => name, BRACKETOPEN, ?arguments => arguments, BRACKETCLOSE, EQUAL, #body => body, SEMI},
    variable = { *modifiers => modifiers, IDENT => name, EQUAL, #body => body, SEMI},

    definition = {IDENT => name, COL, *calls=>arguments, SEMI},

    // import
    import_item = [ #name => item, (COMMA) *],
    import_items = { CBRACKETOPEN, *import_item=>items, CBRACKETCLOSE },
    import = [ RAWIDENT=>path, ?import_items=>import_items, (COMMA) * ],
    imports = { IMPORT, COL, *import=>imports, SEMI },

    // Enum
    enum_statement = (#function | #variable),
    enum_statements = [#enum_statement => statement,  *],
    enum_version = { PIPE, IDENT => name, BRACKETOPEN, *enum_statements => statements, BRACKETCLOSE, SEMI },


    statement = (#imports | #definition | #enum_version | #function | #variable),
    statements = [#statement => statement,  *]
);
