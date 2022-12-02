use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{body, calls, expressions, operator, path, value},
    parser_generator::{
        render::{Render, RenderContext},
        tokens::RawToken,
    },
};

impl Render<Context> for path {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_raw(&self.path).str(".");
    }
}

impl Render<Context> for calls {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_boxed(&self.argument);
    }
}

impl Render<Context> for operator {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            operator::equals(_) => context.str("=="),
            operator::PLUS(_) => context.str("math['+']"),
            operator::MINUS(_) => context.str("math['-']"),
            operator::DIVISION(_) => context.str("math['/']"),
            operator::IDENT(v) => context.render_raw(v),
        };
    }
}

impl Render<Context> for expressions {
    fn render(&self, context: &mut RenderContext<Context>) {
        context
            .str(".op(")
            .render_boxed(&self.operator)
            .str(", ")
            .render_boxed(&self.value)
            .str(")");
    }
}

impl Render<Context> for body {
    fn render(&self, context: &mut RenderContext<Context>) {
        if self.expressions.len() == 0 {
            context.render_boxed(&self.value);
        } else {
            context
                .str("pipe(")
                .render_boxed(&self.value)
                .str(")")
                .join(&self.expressions, "")
                .str(".end");
        }
    }
}
