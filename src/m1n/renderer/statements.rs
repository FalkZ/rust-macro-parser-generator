use crate::m1n::grammar::{
    argument, arguments, enum_version, function, maybe_arguments, modifier, modifiers, name,
    statement, statements, variable,
};

use crate::{
    m1n::{command::prettier_format, grammar::Parser},
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
    },
};

use super::modifiers::Modifiers;
use super::{Context, FileType};

impl Render<Context> for statements {
    fn render(&self, context: &mut RenderContext<Context>) {}
}

impl Render<Context> for Vec<statements> {
    fn render(&self, context: &mut RenderContext<Context>) {
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
            let c = context.borrow_context();
            if c.file_type != FileType::Class {
                panic!("file name should be uppercase for enum type: {}", c.name);
            }

            use crate::parser_generator::tokens::RawToken;

            let names: Vec<String> = enum_versions.iter().map(|v| v.name.as_str()).collect();

            c.file_type = FileType::Enum(names);
        }

        context.join_boxed(&imports, "\n").str("\n\n");

        let c = context.get_context();

        match c.file_type {
            FileType::Class => {
                context
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

                context
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
                context
                    .str(format!("export abstract class {} {{\n", c.name))
                    .join_boxed(&variables, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&functions, "\n\n")
                    .str("\n\n\n")
                    .join_boxed(&raw_functions, "\n\n")
                    .str("\n};")
                    .str("\n\n");

                context.borrow_context().file_type = FileType::Class;

                context.join_boxed(&enum_versions, "\n\n");
            }
        };
    }
}
