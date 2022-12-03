use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{
        body, calls, expressions, match_arm, match_operation, match_statement, operator, path,
        value,
    },
    parser_generator::{
        render::{OutputBuilder, Render, RenderBoxed},
        tokens::RawToken,
    },
};

impl Render<Context> for match_arm {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder
            .str("(_ ")
            .render_boxed(&self.operator)
            .render_boxed(&self.value)
            .str("){ return ")
            .render_boxed(&self.body)
            .str("}");
    }
}

impl Render<Context> for match_statement {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder
            .str("(_) => { if ")
            .join(&self.statements, " else if ")
            .str("}");
    }
}

impl Render<Context> for match_operation {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(value) = builder.get_context().single_expression.clone() {
            builder
                .str("util['match'](")
                .render_boxed(&value)
                .str(",")
                .render_boxed(&self.body)
                .str(")");
        } else {
            builder.str("util['match'], ").render_boxed(&self.body);
        }
    }
}
