use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{
        body, calls, expressions, match_arm, match_operation, match_statement, operator, path,
        value,
    },
    parser_generator::{
        render::{Render, RenderContext},
        tokens::RawToken,
    },
};

impl Render<Context> for match_arm {
    fn render(&self, context: &mut RenderContext<Context>) {
        context
            .str("(_ ")
            .render_boxed(&self.operator)
            .render_boxed(&self.value)
            .str("){ return ")
            .render_boxed(&self.body)
            .str("}");
    }
}

impl Render<Context> for match_statement {
    fn render(&self, context: &mut RenderContext<Context>) {
        context
            .str("(_) => { if ")
            .join(&self.statements, " else if ")
            .str("}");
    }
}

impl Render<Context> for match_operation {
    fn render(&self, context: &mut RenderContext<Context>) {
        if let Some(value) = context.get_context().single_expression.clone() {
            context
                .str("util['match'](")
                .render_boxed(&value)
                .str(",")
                .render_boxed(&self.body)
                .str(")");
        } else {
            context.str("util['match'], ").render_boxed(&self.body);
        }
    }
}
