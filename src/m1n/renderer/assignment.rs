use crate::{
    m1n::grammar::assingment_operation,
    parser_generator::render::{Render, RenderContext},
};

use super::Context;

impl Render<Context> for assingment_operation {
    fn render(&self, context: &mut RenderContext<Context>) {
        if let Some(value) = context.get_context().single_expression.clone() {
            context
                .str("util['assign'](")
                .render_boxed(&value)
                .str(", (_) => { ")
                .render_raw(&self.identifier)
                .str(" = _; })");
        } else {
            context
                .str("util['assign'], (_) => { ")
                .render_raw(&self.identifier)
                .str(" = _; }");
        }
    }
}
