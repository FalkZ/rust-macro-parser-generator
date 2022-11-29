use crate::{
    m1n::grammar::{enum_statement, enum_statements, enum_version},
    parser_generator::render::{Render, RenderContext},
};

use super::Context;

impl Render<Context> for enum_statements {
    fn render(&self, context: &mut RenderContext<Context>) {
        match &*self.statement {
            enum_statement::function(v) => context.render_boxed(&v),
            enum_statement::variable(v) => context.render_boxed(&v),
        };
    }
}
impl Render<Context> for enum_version {
    fn render(&self, context: &mut RenderContext<Context>) {
        let c = context.get_context().clone();

        context
            .str("export class ")
            .render_raw(&self.name)
            .str(format!(" extends {} {{\n", c.name))
            .join(&self.statements, "\n\n")
            .str("\n};");
    }
}
