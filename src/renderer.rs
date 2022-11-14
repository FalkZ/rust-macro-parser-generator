use std::fmt::Display;

use crate::grammar::{body, value, operator, name};


pub enum RenderContext {
    Class(String),
    Singleton,
}

pub trait Render {
    fn render(&self, context: &RenderContext) -> String;
}

use crate::grammar::NUMBER;
impl Display for NUMBER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Render for value {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            value::float(v) => format!("{}.{}", &v.whole, &v.float),
            value::NUMBER(v) => format!("{}", &v),
            value::TEXTLITERAL(v) =>  format!("'{}'", &v),
            value::IDENT(v) => format!("{}", &v),
            value::TYPESCRIPT(v) => format!("({})", &v[1..v.len() - 1]),
        }
    }
}

impl Render for operator {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            operator::PLUS => "math['+']".to_string(),
            operator::MINUS =>"math['-']".to_string(),
            operator::DIVISION => "math['/']".to_string(),
            operator::IDENT(str) => str.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Body(pub body);
impl Render for Body {
    fn render(&self, context: &RenderContext) -> String {

        use crate::grammar::expressions_single;
        
        let r: Vec<expressions_single> = *self.0.expressions.to_owned();

        

        let mut v = self.0.value.render(context);

        r.iter().rev().for_each(|a: &expressions_single| {

            let value = &a.value;
            let op = &a.operator;

            v = format!("op({}, {}, {})", &value.render(&context), &op.render(&context), &v);
        });

        v
    }
}


impl Render for name {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            name::RAWIDENT(v) => format!("['{}']", &v[1..v.len() - 1]),
            name::IDENT(v) => format!("{}", v),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: name,
    pub modifiers: Modifiers,
    pub args: Vec<String>,
    pub body: Body,
}

impl Render for Function {
    fn render(&self, context: &RenderContext) -> String {
        match context {
            RenderContext::Class(_) => format!(
                "{modifiers} {name}({args}){{{body}}}",
                modifiers = self.modifiers.render(context),
                name = self.name.render(context),
                args = self.args.join(", "),
                body = self.body.render(&context)
            ),
            RenderContext::Singleton => {
                let name = match &self.name {
                    name::RAWIDENT(v) => format!("_{}", self.name.render(context)),
                    name::IDENT(v) =>    format!(
                        "{modifiers} const {name}",
                        modifiers = self.modifiers.render(context),
                        name = v,
                    ),
                };

                format!(
                "{name} = ({args}) => ({body})",
                name = name,
                args = self.args.join(", "),
                body = self.body.render(&context)
            )
        },
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub modifiers: Modifiers,
}

#[derive(Debug, Default)]
pub struct Modifiers {
    pub cr: bool,
    pub public: bool,
    pub mutable: bool,
}

impl Render for Modifiers {
    fn render(&self, context: &RenderContext) -> String {
        match context {
            RenderContext::Class(_) => {
                let mut s = vec![];
                if self.public {
                    s.push("public")
                } else {
                    s.push("private")
                }
                if !self.mutable {
                    s.push("readonly")
                };

                s.join(" ")
            }
            RenderContext::Singleton => {
                let mut s = vec![];
                if self.public {
                    s.push("export")
                };
                if !self.mutable {
                    s.push("readonly")
                };

                s.join(" ")
            }
        }
    }
}


#[derive(Debug)]
pub struct Statements {
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

impl Default for Statements {
    fn default() -> Self {
        Statements {
            functions: vec![],
            variables: vec![],
        }
    }
}

fn export_default(functions: &Vec<Function>, context: &RenderContext) -> String {
    let keys: Vec<String> = 
    functions.iter().filter(|v|{
        v.modifiers.public
    }).map(|v|{
     match v.name {
            name::RAWIDENT(_) => format!("{name}: _{name}", name=v.name.render(context)),
            name::IDENT(_) => v.name.render(context),
        }      
    }).collect();

    format!("export default {{{}}}", keys.join(", "))
}

impl Render for Statements {
    fn render(&self, context: &RenderContext) -> String {
        let body = self.functions
            .iter()
            .map(|v| v.render(context))
            .collect::<Vec<String>>()
            .join("\n\n");


            let stm = match context {
                RenderContext::Class(name) => format!("export class {name} {{{body}}}", name=name, body=body),
                RenderContext::Singleton => format!("const _ = {{}}\n\n{body}\n\n {export}", body = &body, export = export_default(&self.functions, context)),
            };

            format!("
            import math from './math'
            import {{op}} from './std'


            {}", stm)
    }
}