use crate::{
    m1n::grammar::{enum_statement, enum_statements, enum_version},
    parser_generator::render::{OutputBuilder, Render},
};

use super::Context;

impl Render<Context> for enum_statements {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match &*self.statement {
            enum_statement::function(v) => builder.render_boxed(&v),
            enum_statement::variable(v) => builder.render_boxed(&v),
        };
    }
}
impl Render<Context> for enum_version {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        let c = builder.get_context().clone();

        builder
            .str("export class ")
            .render_raw(&self.name)
            .str(format!(" extends {} {{\n", c.name))
            .join(&self.statements, "\n\n")
            .str("\n};");
    }
}
