
pub enum RenderContext {
    Class(String),
    Singleton,
}

pub trait Render {
    fn render(&self, context: &RenderContext) -> String;
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub modifiers: Modifiers,
    pub args: Vec<String>,
}

impl Render for Function {
    fn render(&self, context: &RenderContext) -> String {
        match context {
            RenderContext::Class(_) => format!(
                "{modifiers} {name}({args}){{}}",
                modifiers = self.modifiers.render(context),
                name = self.name,
                args = self.args.join(", ")
            ),
            RenderContext::Singleton => format!(
                "{modifiers} const {name} = ({args}) => {{}}",
                modifiers = self.modifiers.render(context),
                name = self.name,
                args = self.args.join(", ")
            ),
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

impl Render for Statements {
    fn render(&self, context: &RenderContext) -> String {
        let body = self.functions
            .iter()
            .map(|v| v.render(context))
            .collect::<Vec<String>>()
            .join("\n\n");


            match context {
                RenderContext::Class(name) => format!("export class {name} {{{body}}}", name=name, body=body),
                RenderContext::Singleton => body,
            }
    }
}