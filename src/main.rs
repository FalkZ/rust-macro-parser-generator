mod lexer;
mod matchers;
mod parser;
mod result;
mod tokens;
mod traits;
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

//expression = [#value => value, #operator => operator * | ],

Parser!(
    operator = (PLUS | MINUS | DIVISION | IDENT(String) ),
    value = ( NUMBER(i64) | TEXTLITERAL(String) | IDENT(String)),

    expressions = [#value => value, #operator => operator, * | #value ],

    argument =  {IDENT(String) => arg,  COMMA, #arguments => rest},
    arguments = (#argument | IDENT(String)),

    function = { IDENT(String) => name, BRACKETOPEN, #arguments => arguments,  BRACKETCLOSE, EQUAL, #expressions => body},
    variable = { IDENT(String) => name, EQUAL, #expressions => body },
    statement = (#function | #variable),
    statements = [#statement => statement,  * | #statement]
);

struct V;



#[derive(Debug)]
struct Statements {
    functions: Vec<Function>,
    variables: Vec<Variable>,
}

impl Default for Statements {
    fn default() -> Self {
        Statements {
            functions: vec![],
            variables: vec![]
        }
    }
}

#[derive(Debug)]
struct Function {
    name: String,
    args: Vec<String>,
}

#[derive(Debug)]
struct Variable {
    name: String
}

macro_rules! map_nested {
    (match $value:ident {
        $($pattern:pat => $block:block)+
    }
    rest: $continue:path) => {
        let mut __iter = $value;

        loop {
            match __iter {
                $($pattern => $block)+
            };

            match __iter {
                $continue(v) => { __iter = &v.rest; },
                _ => { break; }
            };
        }

    };
}

impl V {
    fn arguments(&self, args: &arguments) -> Vec<String> {
        let mut vec: Vec<String> = vec![];

        map_nested!(
            match args {
                arguments::argument(ar) => {
                    vec.push(ar.arg.0.clone());
                }
                arguments::IDENT(str) => {
                    vec.push(str.to_owned());
                }

            }
            rest: arguments::argument
        );

        vec
    }

    fn function(&self, f: &function) -> Function {
        let name = f.name.0.to_owned();

       let args = self.arguments(&f.arguments);
        
        Function { name, args }
    }

    fn variable(&self, f: &variable) -> Variable {
        let name = f.name.0.to_owned();
      
        Variable { name }
    }

    fn statement(&self, s: &mut Statements, statement: &statement){

        match statement {
            statement::function(fun) => {
                s.functions.push(self.function(&fun));
            },
            statement::variable(var) => {s.variables.push(self.variable(&var));},
        };
    }

    fn statements(&self, statements: &statements) -> Statements {
        let mut s = Statements::default();

        
        statements.items.iter().for_each(|v|{ self.statement(&mut s, &v.statement)});
       
        let end = statements.end.as_ref().expect("no end at statement");
        self.statement(&mut s, &end);
  
        s
    }
}


#[derive(Debug, Clone)]
struct Recursive<R, B> {
    items: Vec<R>,
    end: Option<B>
}

impl<R, B> Recursive<R, B> {
    fn prepend_0(self, element: Recursive<R, B>) -> Self {

        let mut items: Vec<R> = vec![];
        
        let end = element.end.or(self.end);

        items.extend(element.items);
        items.extend(self.items);


        Self{ items, end }
    }

    fn prepend(self, element: R) -> Self {

        let mut items: Vec<R> = vec![element];
        
        let end = self.end;

        items.extend(self.items);


        Self{ items, end }
    }
}








fn run() -> ParserResult<Statements> {
    let a = "
    test(a, b) = 
        a + b

    PI = 3

    ";

    // let a = "test = a";

    let t = Parser::new(a)?;

    let t = Parser::statements(&t.tokens)?;


    let v = V {};

    let r = v.statements(&t);

    
    Ok(r)
}

fn main() {
    println!("{:?}", run());
}
