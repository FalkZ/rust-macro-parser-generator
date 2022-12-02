use std::fs::ReadDir;

use crate::{
    m1n::grammar::{
        bracket_expression, expression, expression_variant, expression_variant_wrapper,
        newline_expression,
    },
    parser_generator::render::{Render, RenderContext},
};

use super::Context;

impl Render<Context> for newline_expression {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_boxed(&self.expressions);
    }
}

impl Render<Context> for bracket_expression {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_boxed(&self.expressions);
    }
}

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
