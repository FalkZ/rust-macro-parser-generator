use std::fs::ReadDir;

use crate::{
    m1n::grammar::{expression, expression_variant, expression_variant_wrapper},
    parser_generator::render::{Render, RenderContext},
};

use super::Context;

impl Render<Context> for expression_variant_wrapper {
    fn render(&self, context: &mut RenderContext<Context>) {
        match &*self.expression {
            expression_variant::newline_expression(v) => {
                context.render_boxed(&v.expressions);
            }
            expression_variant::bracket_expression(v) => {
                context.render_boxed(&v.expressions);
            }
        };
    }
}
