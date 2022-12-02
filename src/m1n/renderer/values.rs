use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{body, calls, expressions, number, operator, path, primitive_value, value},
    parser_generator::{
        render::{Render, RenderContext},
        tokens::RawToken,
    },
};

impl Render<Context> for number {
    fn render(&self, context: &mut RenderContext<Context>) {
        if let Some(_) = self.negative {
            context.str("-");
        }
        context.render_raw(&self.whole);
    }
}

impl Render<Context> for primitive_value {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            primitive_value::float(v) => {
                context.render_boxed(&v.whole).str(".").render_raw(&v.float);
            }
            primitive_value::number(v) => {
                context.render_boxed(v);
            }
            primitive_value::TEXTLITERAL(v) => {
                context.apply(v, |v| format!("`{}`", v.as_str().substring(1, -1)));
            }
            primitive_value::IDENT(v) => {
                context.render_raw(v);
            }
            primitive_value::TYPESCRIPT(v) => {
                context.apply(v, |v| format!("({})", v.as_str().substring(1, -1)));
            }
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
                    .join(&v.arguments, ", ")
                    .str(")");
            }

            value::primitive_value(v) => {
                context.render_boxed(v);
            }
            value::UNDERLINE(v) => {
                context.render_raw(v);
            }
        };
    }
}
