use crate::m1n::grammar::{argument, arguments, function, name};

use crate::parser_generator::render::{OutputBuilder, Render, RenderBoxed};

use super::substring::Substring;
use super::{Context, FileType, StatementType};

impl Render<Context> for argument {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.render_raw(&self.arg).str(", ");
    }
}

impl Render<Context> for arguments {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        let c = builder.get_context().clone();

        builder.str("(");

        if let FileType::Enum(names) = c.file_type {
            builder.str(format!("this: {}, ", names.join(" | ")));
        }

        builder
            .join(&self.arguments, "")
            .render_raw(&self.last)
            .str(")");
    }
}

impl Render<Context> for name {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        match self {
            name::RAWIDENT(v) => {
                let val: &str = v.as_str();

                builder.str(format!("['{}']", &val.substring(1, -1)))
            }
            name::IDENT(r) => builder.render_raw(r),
        };
    }
}

impl Render<Context> for function {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.borrow_context().statement_type = match *self.name {
            name::RAWIDENT(_) => StatementType::RawFunction,
            name::IDENT(_) => StatementType::Function,
        };

        builder.render(&self.modifiers).render_boxed(&self.name);

        match &self.arguments {
            Some(v) => builder.render_boxed(v),
            None => builder.str("()"),
        };

        builder.str(" { return ").render_boxed(&self.body).str(";}");
    }
}
