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

impl Render<Context> for variable {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.borrow_context().statement_type = StatementType::Variable;

        context.render(&self.modifiers);
        context.render_raw(&self.name);

        context.str(" = ").render_boxed(&self.body).str(";");
    }
}
