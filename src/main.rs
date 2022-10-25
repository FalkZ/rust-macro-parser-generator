mod lexer;

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






#[derive(Debug)]
enum ParserError {
    Mismatch,
    Err(anyhow::Error),
}


use std::error::Error;
use std::fmt;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&dyn Error> {
        match &self {
            ParserError::Err(e) => Some(self),
            _ => None,

        }
    }
}







type ParserResult<T> =  Result<T, ParserError>;

impl ParserError {
    fn error(message: &str) -> ParserError {
        ParserError::Err(anyhow::anyhow!(message))
    }
}


use core::slice::Iter;





trait T<V> {
    fn or_message(self, str: &str) -> ParserResult<V>;
}

impl<A> T<A> for Option<A> {
    fn or_message(self, message: &str) -> ParserResult<A> {
        match self {
            Some(v) => Ok(v),
            None => Err(ParserError::error(message)),
        }
    }
}

trait T2<V> {
    fn or_err(self) -> ParserResult<V>;
}

impl<A, E: Into<anyhow::Error>> T2<A> for Result<A, E> {
    fn or_err(self) -> ParserResult<A> {
        let res: anyhow::Result<Lexer> = self.into();
        match res {
            Ok(v) => Ok(v),
            Err(e) => ParserError::Err(e),
        }
    }
}



impl Parser {
    fn new(str: &str) -> Result<Self, String> {
        let tokens = Lexer::lex(str)?;

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

    fn next(&mut self) -> ParserResult<Lexer> {
        if self.tokens.is_empty() {
            Err("next on EOF".into())
        } else {
            Ok(self.tokens.remove(0))
        }
    }

    fn root(mut self) -> Result<(), String> {
        while let Ok(t) = self.peek() {
            println!("{:?}", &t);

            match t {
                &Lexer::NUMBER(i64) => {
                    let a = self.next().unwrap();
                    self.stack.push(a);
                }
                _ => {
                    self.next().unwrap();
                }
            };
        }
        println!("{:?}", &self);

        // eat!(self, Lexer::T123(_));
        // eat!(self, Lexer::PLUS);

        Ok(())
    }

    fn parse_term(tokens: Iter<Lexer>) -> ParserResult<()> {
         eat!(tokens, NUMBER);

        Ok(())
    }

    fn start(self) {
        let iter: Iter<Lexer> = self.tokens.iter();

        Self::parse_term(iter.clone());
    }
}

// op = PLUS | MINUS | POWER | DIV
// term = NUMBER op { term | NUMBER }

fn run() -> Result<(), String> {
    let a = "123 \n ^+ 345 add 12 'text' ";

    let t = Parser::new(a)?;

    t.root()?;

    Ok(())
}

fn main() {
    println!("{:?}", run());
}
