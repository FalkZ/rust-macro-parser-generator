Lexer!(
    {
        NUMBER(i32): '0'..='9' => continue,
        IDENT(String): 'A'..='z' => continue,
        TEXTLITERAL(String): '"' | '\'' => until('"' | '\''),
        PLUS: '+',
        MINUS: '-',
        DIVISION: '/',
        MULTIPLICATION: '*',
        POWER: '^',
        BRACKETOPEN: '(',
        BRACKETCLOSE: ')',
        CBRACKETOPEN: '{',
        CBRACKETCLOSE: '}',
        _: '\n' | ' '
    }
    {
        T123: Lexer::NUMBER(123 | 12),
        IF: { IF } Lexer::IDENT(IF)
    } 
);