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
            FileType::Class | FileType::Enum(_) => match c.statement_type {
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
