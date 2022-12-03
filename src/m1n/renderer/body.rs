use super::{substring::Substring, Context};
use crate::{
    m1n::grammar::{binary_operation, body, calls, expression, expressions, operator, path, value},
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
            operator::assignment(_) => unreachable!(),
        };
    }
}

impl Render<Context> for binary_operation {
    fn render(&self, context: &mut RenderContext<Context>) {
        if let Some(value) = context.get_context().single_expression.clone() {
            context
                .render_boxed(&self.operator)
                .str("(")
                .render_boxed(&value)
                .str(",")
                .render_boxed(&self.value)
                .str(")");
        } else {
            context
                .render_boxed(&self.operator)
                .str(", ")
                .render_boxed(&self.value);
        }
    }
}

impl Render<Context> for expression {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            expression::assingment_operation(v) => context.render_boxed(&v),
            expression::match_operation(v) => context.render_boxed(&v),
            expression::binary_operation(v) => context.render_boxed(&v),
        };
    }
}

impl Render<Context> for expressions {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.str(".op(").render_boxed(&self.expression).str(")");
    }
}

impl Render<Context> for body {
    fn render(&self, context: &mut RenderContext<Context>) {
        if let Some(first) = &self.first {
            if self.expressions.len() == 0 {
                context.borrow_context().single_expression = Some(self.value.clone());

                context.render_boxed(first);
            } else {
                context.borrow_context().single_expression = None;
                context
                    .str("pipe(")
                    .render_boxed(&self.value)
                    .str(").op(")
                    .render_boxed(first)
                    .str(")")
                    .join(&self.expressions, "")
                    .str(".end()");
            }
        } else {
            context.render_boxed(&self.value);
        }
    }
}
