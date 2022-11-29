use sourcemap::SourceMapBuilder;

use crate::m1n::grammar::{
    argument, arguments, function, maybe_arguments, modifier, modifiers, name, statement,
    statements, variable,
};

use crate::{
    m1n::grammar::Parser,
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
    },
};

use super::substring::Substring;
use super::{Context, FileType, StatementType};

impl Render<Context> for argument {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_raw(&self.arg).str(", ");
    }
}

impl Render<Context> for arguments {
    fn render(&self, context: &mut RenderContext<Context>) {
        context
            .str("(")
            .join(&self.arguments, "")
            .render_raw(&self.last)
            .str(")");
    }
}

#[derive(Debug, Default)]
pub struct Modifiers {
    pub mutable: bool,
    pub public: bool,
    pub create: bool,
}

impl Modifiers {
    pub fn new(m: &Vec<modifiers>) -> Self {
        let mut r = Modifiers::default();

        m.iter().for_each(|v| match *v.modifier {
            modifier::MUT(_) => {
                r.mutable = true;
            }
            modifier::PUB(_) => {
                r.public = true;
            }
            modifier::CR(_) => {
                r.create = true;
            }
        });

        r
    }
}

impl Render<Context> for Vec<modifiers> {
    fn render(&self, context: &mut RenderContext<Context>) {
        let mut m = Modifiers::new(&self);

        let c = context.get_context();

        match c.file_type {
            FileType::Class => match c.statement_type {
                StatementType::Variable => {
                    if m.public {
                        context.str("public ");
                    } else {
                        context.str("private ");
                    }
                    if !m.mutable {
                        context.str("readonly ");
                    }
                }
                StatementType::RawFunction | StatementType::Function => {
                    if m.public {
                        context.str("public ");
                    } else {
                        context.str("private ");
                    }
                }
                StatementType::None => unreachable!(),
            },
            FileType::Singleton => match c.statement_type {
                StatementType::Variable => {
                    if m.public {
                        context.str("export ");
                    }
                    if m.mutable {
                        context.str("let ");
                    } else {
                        context.str("const ");
                    }
                }
                StatementType::Function => {
                    if m.public {
                        context.str("export ");
                    }

                    context.str("function ");
                }

                StatementType::RawFunction => {}
                StatementType::None => unreachable!(),
            },
        }
    }
}

impl Render<Context> for name {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            name::RAWIDENT(v) => {
                let val: &str = v.as_str();

                context.str(format!("['{}']", &val.substring(1, -1)))
            }
            name::IDENT(r) => context.render_raw(r),
        };
    }
}

impl Render<Context> for function {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.borrow_context().statement_type = match *self.name {
            name::RAWIDENT(_) => StatementType::RawFunction,
            name::IDENT(_) => StatementType::Function,
        };

        context.render(&self.modifiers).render_boxed(&self.name);

        match &self.arguments {
            Some(v) => context.render_boxed(v),
            None => context.str("()"),
        };

        context.str(" { return ").render_boxed(&self.body).str(";}");
    }
}

impl Render<Context> for variable {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.borrow_context().statement_type = StatementType::Variable;

        context.render(&self.modifiers);
        context.render_raw(&self.name);

        context.str(" = ").render_boxed(&self.body).str(";");
    }
}
