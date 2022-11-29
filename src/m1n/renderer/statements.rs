use crate::m1n::grammar::{
    argument, arguments, function, maybe_arguments, modifier, modifiers, name, statement,
    statements, variable,
};

use crate::{
    m1n::{command::prettier_format, grammar::Parser},
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
    },
};

use super::shell::Modifiers;
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
            };
        });

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
        };
    }
}
