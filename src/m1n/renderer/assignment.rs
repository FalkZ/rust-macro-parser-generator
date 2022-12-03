use crate::{
    m1n::grammar::assingment_operation,
    parser_generator::render::{OutputBuilder, Render, RenderBoxed},
};

use super::Context;

impl Render<Context> for assingment_operation {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(value) = builder.get_context().single_expression.clone() {
            builder
                .str("util['assign'](")
                .render_boxed(&value)
                .str(", (_) => { ")
                .render_raw(&self.identifier)
                .str(" = _; })");
        } else {
            builder
                .str("util['assign'], (_) => { ")
                .render_raw(&self.identifier)
                .str(" = _; }");
        }
    }
}
