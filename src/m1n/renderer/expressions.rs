use std::fs::ReadDir;

use crate::{
    m1n::grammar::{bracket_expression, expression, newline_expression},
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
