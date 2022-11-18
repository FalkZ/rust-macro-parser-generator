use std::fmt::Display;


use crate::grammar::{
    body, function_call, name, operator, path_single, value,
};

pub struct Import {
    pub file: String,
    pub content: String,
}

#[derive(Default)]
pub struct Imports(pub Vec<Import>);

impl Imports {
    fn add_import(&mut self, file: &str, content: &str) {
        self.0.push(Import {
            file: file.to_string(),
            content: content.to_string(),
        })
    }
}

impl Render for Imports {
    fn render(&self, context: &RenderContext) -> String {
        let own_file = match context {
            RenderContext::Class(v) => v,
            RenderContext::Singleton(v) => v,
        };

        let s: Vec<String> = self
            .0
            .iter()
            .filter(|i| &i.file != own_file)
            .map(|i| {
                format!(
                    "import {content} from './{file}'",
                    content = i.content,
                    file = i.file
                )
            })
            .collect();

        s.join("\n")
    }
}

pub enum RenderContext {
    Class(String),
    Singleton(String),
}

pub trait Render {
    fn render(&self, context: &RenderContext) -> String;
}

use crate::grammar::NUMBER;
impl Display for NUMBER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl Render for function_call {
    fn render(&self, context: &RenderContext) -> String {
        let mut args: Vec<String> = self
            .arguments
            .arguments
            .iter()
            .map(|a| a.arg.render(&context))
            .collect();

        args.push(self.arguments.last.render(&context));

        let path: Vec<String> = self.path.iter().map(|v: &path_single| v.path.0.as_str().to_string()).collect();

        let mut p =  path.join(".");
        if p.len()>0 {p += "."}
        format!(
            "{path}{name}({args})",
            path = p,
            name = &self.name.0.as_str(),
            args = args.join(", ")
        )
    }
}

impl Render for value {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            value::float(v) => format!("{}.{}", &v.whole, &v.float),
            value::NUMBER(v) => format!("{}", v),
            value::TEXTLITERAL(r) => {
               let v = r.as_str();
                format!("`{}`", &v[1..v.len() - 1])
            },
            value::IDENT(v) => format!("{}", &v),
            value::TYPESCRIPT(r) => {          
                let v = r.as_str();
                format!("({})", &v[1..v.len() - 1])},
            value::function_call(f) => f.render(context),
        }
    }
}

impl Render for operator {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            operator::PLUS(_) => "math['+']".to_string(),
            operator::MINUS(_) => "math['-']".to_string(),
            operator::DIVISION(_) => "math['/']".to_string(),
            operator::IDENT(str) => str.as_str().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Body(pub body);

impl Render for Body {
    fn render(&self, context: &RenderContext) -> String {
        self.0.render(&context)
    }
}

impl Render for body {
    fn render(&self, context: &RenderContext) -> String {
        use crate::grammar::expressions_single;

        let r: Vec<expressions_single> = *self.expressions.to_owned();

        let mut v = self.value.render(context);

        r.iter().for_each(|a: &expressions_single| {
            let value = &a.value;
            let op = &a.operator;

            v = format!(
                "op({}, {}, {})",
                &v,
                &op.render(&context),
                &value.render(&context)
            );
        });

        v
    }
}

impl Render for name {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            name::RAWIDENT(r) =>  {          
                let v = r.as_str();
                format!("['{}']", &v[1..v.len() - 1])
            },
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
                "{modifiers} {name}({args}){{return {body}}}",
                modifiers = self.modifiers.render(context),
                name = self.name.render(context),
                args = self.args.join(", "),
                body = self.body.render(&context)
            ),
            RenderContext::Singleton(_) => {
                let name = match &self.name {
                    name::RAWIDENT(v) => format!("_{} = function", self.name.render(context)),
                    name::IDENT(v) => format!(
                        "{modifiers} function {name}",
                        modifiers = self.modifiers.render(context),
                        name = v,
                    ),
                };

                format!(
                    "{name}({args}){{return ({body})}}",
                    name = name,
                    args = self.args.join(", "),
                    body = self.body.render(&context)
                )
            }
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
            RenderContext::Singleton(_) => {
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
    let keys: Vec<String> = functions
        .iter()
        .filter(|v| v.modifiers.public)
        .map(|v| match v.name {
            name::RAWIDENT(_) => format!("{name}: _{name}", name = v.name.render(context)),
            name::IDENT(_) => v.name.render(context),
        })
        .collect();

    format!("export default {{{}}}", keys.join(", "))
}

impl Render for Statements {
    fn render(&self, context: &RenderContext) -> String {
        let body = self
            .functions
            .iter()
            .map(|v| v.render(context))
            .collect::<Vec<String>>()
            .join("\n\n");

        let stm = match context {
            RenderContext::Class(name) => {
                format!("export class {name} {{{body}}}", name = name, body = body)
            }
            RenderContext::Singleton(_) => format!(
                "const _ = {{}}\n\n{body}\n\n {export}",
                body = &body,
                export = export_default(&self.functions, context)
            ),
        };

        let mut imports = Imports::default();

        imports.add_import("math", "math");
        imports.add_import("std", "{ op }");

        format!(
            "
            {}


            {}",
            imports.render(context),
            stm
        )
    }
}
