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
        IDENT => { {"if"} => IF }
        //NUMBER => { { 123 } => T123(i64) }
    }
);

// op = PLUS | MINUS | POWER | DIV
// _term3 = term | NUMBER(String)
// term = NUMBER(String) op _term3

//expression = [#value => value, #operator => operator * | ],

Parser!(
    operator = (PLUS | MINUS | DIVISION | IDENT(String) ),
    float = {NUMBER(String) => whole, DOT, NUMBER(String) => float},
    value = ( #float | NUMBER(String) | TEXTLITERAL(String) | IDENT(String)),

    expressions = [ #value => value, #operator => operator, * ],
    ex = {#expressions => ex, #value => v, SEMI},

    argument =  [IDENT(String) => arg,  COMMA, *],
    arguments = {BRACKETOPEN, #argument => arguments, IDENT(String) => last,  BRACKETCLOSE},

    no_arguments = {BRACKETOPEN, BRACKETCLOSE},
    maybe_arguments =  (#arguments | #no_arguments),

    function = { IDENT(String) => name, #maybe_arguments => arguments, EQUAL, #ex => body},
    variable = { IDENT(String) => name, EQUAL, #ex => body },
    statement = (#function | #variable),
    statements = [#statement => statement,  *]
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



impl V {
    fn arguments(&self, args: &arguments) -> Vec<String> {
        let mut vec: Vec<String> = args.arguments.iter().map(|a| a.arg.0.clone()).collect();

        vec.push(args.last.0.clone());
      

        vec
    }

    fn function(&self, f: &function) -> Function {
        let name = f.name.0.to_owned();

        let args =  match *f.arguments.clone() {
            maybe_arguments::arguments(a) => self.arguments(&a),
            maybe_arguments::no_arguments(_) => vec![],
        };

      
        
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

        
        statements.iter().for_each(|v|{ self.statement(&mut s, &v.statement)});
  
        s
    }
}











fn run() -> ParserResult<Statements> {
    let a = "
    PI = a + b + 0.012;

    Pi = 123;

    fn(a, b) = a + b;

    one(b) = a + b;

    zero() = a + b;
    
    ";

    // let a = "test = a";

    let t = Parser::new(a)?;

    let t = Parser::statements(&t.tokens)?;


    let v = V {};

    println!("{:?}", &t);

    let r = v.statements(&t);

    
    Ok(r)
}

fn main() {
    println!("{:?}", run());
}
