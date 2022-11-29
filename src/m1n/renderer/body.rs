use std::ops::Sub;

use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{arguments, body, call, calls, expressions, operator, path, value},
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

impl Render<Context> for call {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_boxed(&self.arg).str(", ");
    }
}

impl Render<Context> for calls {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.join(&self.arguments, "").render_boxed(&self.last);
    }
}

impl Render<Context> for operator {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            operator::PLUS(_) => context.str("math['+']"),
            operator::MINUS(_) => context.str("math['-']"),
            operator::DIVISION(_) => context.str("math['/']"),
            operator::IDENT(v) => context.render_raw(v),
        };
    }
}

impl Render<Context> for value {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            value::function_call(v) => {
                context
                    .join(&v.path, "")
                    .render_raw(&v.name)
                    .str("(")
                    .render_boxed(&v.arguments)
                    .str(")");
            }
            value::float(v) => {
                context.render_raw(&v.whole).str(".").render_raw(&v.float);
            }
            value::NUMBER(v) => {
                context.render_raw(v);
            }
            value::TEXTLITERAL(v) => {
                context.apply(v, |v| format!("`{}`", v.as_str().substring(1, -1)));
            }
            value::IDENT(v) => {
                context.render_raw(v);
            }
            value::TYPESCRIPT(v) => {
                context.apply(v, |v| format!("({})", v.as_str().substring(1, -1)));
            }
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
