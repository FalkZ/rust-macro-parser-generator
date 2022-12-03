use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{
        binary_operation, body, calls, expression, expressions, operator, path, unary_operation,
        unary_operator, value,
    },
    parser_generator::{
        render::{OutputBuilder, Render},
        tokens::RawToken,
    },
};

impl Render<Context> for path {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.render_raw(&self.path).str(".");
    }
}

impl Render<Context> for calls {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.render_boxed(&self.argument);
    }
}

impl Render<Context> for operator {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            operator::equals(_) => builder.str("=="),
            operator::PLUS(_) => builder.str("math['+']"),
            operator::MINUS(_) => builder.str("math['-']"),
            operator::DIVISION(_) => builder.str("math['/']"),
            operator::IDENT(v) => builder.render_raw(v),
        };
    }
}

impl Render<Context> for binary_operation {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(value) = builder.get_context().single_expression.clone() {
            builder
                .render_boxed(&self.operator)
                .str("(")
                .render_boxed(&value)
                .str(",")
                .render_boxed(&self.value)
                .str(")");
        } else {
            builder
                .render_boxed(&self.operator)
                .str(", ")
                .render_boxed(&self.value);
        }
    }
}

impl Render<Context> for unary_operator {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            unary_operator::NOT(_) => builder.str("math['!']"),
            unary_operator::IDENT(v) => builder.render_raw(v),
        };
    }
}

impl Render<Context> for unary_operation {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(value) = builder.get_context().single_expression.clone() {
            builder
                .render_boxed(&self.operator)
                .str("(")
                .render_boxed(&value)
                .str(")");
        } else {
            builder.render_boxed(&self.operator);
        }
    }
}

impl Render<Context> for expression {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            expression::assingment_operation(v) => builder.render_boxed(&v),
            expression::match_operation(v) => builder.render_boxed(&v),
            expression::binary_operation(v) => builder.render_boxed(&v),
            expression::unary_operation(v) => builder.render_boxed(&v),
        };
    }
}

impl Render<Context> for expressions {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.str(".op(").render_boxed(&self.expression).str(")");
    }
}

impl Render<Context> for body {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        if let Some(first) = &self.first {
            if self.expressions.len() == 0 {
                builder.borrow_context().single_expression = Some(self.value.clone());

                builder.render_boxed(first);
            } else {
                builder.borrow_context().single_expression = None;
                builder
                    .str("pipe(")
                    .render_boxed(&self.value)
                    .str(").op(")
                    .render_boxed(first)
                    .str(")")
                    .join(&self.expressions, "")
                    .str(".end()");
            }
        } else {
            builder.render_boxed(&self.value);
        }
    }
}
