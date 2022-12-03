use crate::m1n::grammar::{name, statement, statements};

use crate::parser_generator::render::{OutputBuilder, Render};

use super::modifiers::Modifiers;
use super::{Context, FileType};

impl Render<Context> for statements {
    fn render(&self, _context: &mut OutputBuilder<Context>) {}
}

impl Render<Context> for Vec<statements> {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        let mut imports = vec![];
        let mut variables = vec![];
        let mut functions = vec![];
        let mut raw_functions = vec![];
        let mut enum_versions = vec![];

        self.iter().for_each(|statement| {
            match *statement.statement.to_owned() {
                statement::function(v) => {
                    match *v.name {
                        name::RAWIDENT(_) => raw_functions.push(v),
                        name::IDENT(_) => functions.push(v),
                    };
                }
                statement::variable(v) => {
                    variables.push(v);
                }
                statement::definition(_) => {}
                statement::imports(v) => {
                    imports.push(v);
                }
                statement::enum_version(v) => {
                    enum_versions.push(v);
                }
            };
        });

        if enum_versions.len() > 0 {
            let c = builder.borrow_context();
            if c.file_type != FileType::Class {
                panic!("file name should be uppercase for enum type: {}", c.name);
            }

            use crate::parser_generator::tokens::RawToken;

            let names: Vec<String> = enum_versions.iter().map(|v| v.name.as_str()).collect();

            c.file_type = FileType::Enum(names);
        }

        builder.join_boxed(&imports, "\n").str("\n\n");

        let c = builder.get_context();

        match c.file_type {
            FileType::Class => {
                builder
                    .str(format!("export class {} {{\n", c.name))
                    .join_boxed(&variables, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&functions, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&raw_functions, "\n\n")
                    .str("\n};");
            }
            FileType::Singleton => {
                let names = functions
                    .iter()
                    .filter(|v| Modifiers::new(&v.modifiers).public)
                    .map(|v| match &*v.name {
                        name::RAWIDENT(_) => unreachable!(),
                        name::IDENT(v) => v.as_str(),
                    })
                    .collect::<Vec<&str>>()
                    .join(", ");

                builder
                    .join_boxed(&variables, "\n\n")
                    .str("\n\n\n")
                    .str("\n const self = {")
                    .join_boxed(&raw_functions, ",\n\n")
                    .str("}")
                    .str("\n\n\n")
                    .join_boxed(&functions, "\n\n")
                    .str(format!("export default {{ ...self, {}}}", names));
            }
            FileType::Enum(_) => {
                builder
                    .str(format!("export abstract class {} {{\n", c.name))
                    .join_boxed(&variables, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&functions, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&raw_functions, "\n\n")
                    .str("\n};")
                    .str("\n\n");

                builder.borrow_context().file_type = FileType::Class;

                builder.join_boxed(&enum_versions, "\n\n");
            }
        };
    }
}
