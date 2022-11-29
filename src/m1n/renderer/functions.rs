use std::fs::File;

use sourcemap::SourceMapBuilder;

use crate::m1n::grammar::{
    argument, arguments, function, maybe_arguments, modifier, modifiers, name, statement,
    statements, variable,
};

use crate::{
    m1n::grammar::Parser,
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
    },
};

use super::substring::Substring;
use super::{Context, FileType, StatementType};

impl Render<Context> for argument {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.render_raw(&self.arg).str(", ");
    }
}

impl Render<Context> for arguments {
    fn render(&self, context: &mut RenderContext<Context>) {
        let c = context.get_context().clone();

        context.str("(");

        if let FileType::Enum(names) = c.file_type {
            context.str(format!("this: {}, ", names.join(" | ")));
        }

        context
            .join(&self.arguments, "")
            .render_raw(&self.last)
            .str(")");
    }
}

impl Render<Context> for name {
    fn render(&self, context: &mut RenderContext<Context>) {
        match self {
            name::RAWIDENT(v) => {
                let val: &str = v.as_str();

                context.str(format!("['{}']", &val.substring(1, -1)))
            }
            name::IDENT(r) => context.render_raw(r),
        };
    }
}

impl Render<Context> for function {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.borrow_context().statement_type = match *self.name {
            name::RAWIDENT(_) => StatementType::RawFunction,
            name::IDENT(_) => StatementType::Function,
        };

        context.render(&self.modifiers).render_boxed(&self.name);

        match &self.arguments {
            Some(v) => context.render_boxed(v),
            None => context.str("()"),
        };

        context.str(" { return ").render_boxed(&self.body).str(";}");
    }
}
