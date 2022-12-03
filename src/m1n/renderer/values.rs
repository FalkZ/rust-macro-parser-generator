use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{body, calls, expressions, number, operator, path, primitive_value, value},
    parser_generator::{
        render::{OutputBuilder, Render},
        tokens::RawToken,
    },
};

impl Render<Context> for number {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(_) = self.negative {
            builder.str("-");
        }
        builder.render_raw(&self.whole);
    }
}

impl Render<Context> for primitive_value {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            primitive_value::float(v) => {
                builder.render_boxed(&v.whole).str(".").render_raw(&v.float);
            }
            primitive_value::number(v) => {
                builder.render_boxed(v);
            }
            primitive_value::TEXTLITERAL(v) => {
                builder.apply(v, |v| format!("`{}`", v.as_str().substring(1, -1)));
            }
            primitive_value::IDENT(v) => {
                builder.render_raw(v);
            }
            primitive_value::TYPESCRIPT(v) => {
                builder.apply(v, |v| format!("({})", v.as_str().substring(1, -1)));
            }
        };
    }
}

impl Render<Context> for value {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            value::function_call(v) => {
                builder
                    .join(&v.path, "")
                    .render_raw(&v.name)
                    .str("(")
                    .join(&v.arguments, ", ")
                    .str(")");
            }

            value::primitive_value(v) => {
                builder.render_boxed(v);
            }
            value::UNDERLINE(v) => {
                builder.render_raw(v);
            }
            value::bracket_expression(v) => {
                builder.render_boxed(v);
            }
        };
    }
}
