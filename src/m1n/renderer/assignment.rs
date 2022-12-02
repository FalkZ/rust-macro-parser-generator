use crate::{
    m1n::grammar::assingment_operation,
    parser_generator::render::{Render, RenderContext},
};

use super::Context;

impl Render<Context> for assingment_operation {
    fn render(&self, context: &mut RenderContext<Context>) {
        context
            .str("util['assign'], (_) => { ")
            .render_raw(&self.identifier)
            .str(" = _; }");
    }
}
