use crate::m1n::grammar::{modifier, modifiers};

use crate::parser_generator::render::{OutputBuilder, Render};

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
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        let m = Modifiers::new(&self);

        let c = builder.get_context();

        match c.file_type {
            FileType::Class | FileType::Enum(_) => match c.statement_type {
                StatementType::Variable => {
                    if m.public {
                        builder.str("public ");
                    } else {
                        builder.str(if c.file_type == FileType::Class {
                            "private "
                        } else {
                            "protected "
                        });
                    }
                    if !m.mutable {
                        builder.str("readonly ");
                    }
                }
                StatementType::RawFunction | StatementType::Function => {
                    if m.public {
                        builder.str("public ");
                    } else {
                        builder.str(if c.file_type == FileType::Class {
                            "private "
                        } else {
                            "protected "
                        });
                    }
                }
                StatementType::None => unreachable!(),
            },
            FileType::Singleton => match c.statement_type {
                StatementType::Variable => {
                    if m.public {
                        builder.str("export ");
                    }
                    if m.mutable {
                        builder.str("let ");
                    } else {
                        builder.str("const ");
                    }
                }
                StatementType::Function => {
                    if m.public {
                        builder.str("export ");
                    }

                    builder.str("function ");
                }

                StatementType::RawFunction => {}
                StatementType::None => unreachable!(),
            },
        }
    }
}
