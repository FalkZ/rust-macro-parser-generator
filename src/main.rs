mod lexer;
mod result;
mod traits;

use core::slice::Iter;
use result::{ParserResult, ParserError};
use traits::*;


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
        IDENT => { {"^"} => POWER },
        NUMBER => { { 123 } => T123(i64) }
    }
);

macro_rules! eat {
    ($self:ident, $($pattern : tt)+) => {
        let next = $self.next().or_message("next on EOF")?;

        if(!matches!(&next, &$($pattern)+)){
            return ParserError::error(&format!("tried to eat {} but was {:?}", stringify!($($pattern)+), &next));
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
    tokens: Vec<Lexer>,
    stack: Vec<Lexer>,
}

impl Parser {
    fn new(str: &str) -> ParserResult<Self> {
        let tokens = Lexer::lex(str).or_err()?;
        return Ok(Self {
            tokens,
            stack: vec![],
        });
    }

    fn peek(&self) -> Result<&Lexer, String> {
        if let Some(t) = self.tokens.get(0) {
            Ok(t)
        } else {
            Err("peek on EOF".to_string())
        }
    }

    fn root(mut self) -> Result<(), String> {
        // eat!(self, Lexer::T123(_));
        // eat!(self, Lexer::PLUS);

        Ok(())
    }

    fn parse_term(mut tokens: Iter<Lexer>) -> ParserResult<()> {
        eat!(tokens, Lexer::NUMBER(_));
        eat!(tokens, Lexer::NUMBER(_));

        Ok(())
    }

    fn start(self) -> ParserResult<()> {
        let iter: Iter<Lexer> = self.tokens.iter();

        Self::parse_term(iter.clone())?;

        Ok(())
    }
}

// op = PLUS | MINUS | POWER | DIV
// _term3 = term | NUMBER(i64)
// term = NUMBER(i64) op _term3

enum op {
    PLUS,
    MINUS,
    POWER,
    DIV,
}

struct NUMBER(i64);

enum _term3 {
    op(op),
    NUMBER(NUMBER),
}

struct term(NUMBER, op, _term3);

fn run() -> ParserResult<()> {
    let a = "123 \n ^+ 345 add 12 'text' ";

    let t = Parser::new(a)?;

    t.start()?;

    Ok(())
}

fn main() {
    println!("{:?}", run());
}
