

use crate::m1n::grammar::{
    variable,
};

use crate::{
    parser_generator::{
        render::{Render, RenderContext},
    },
};


use super::{Context, StatementType};

impl Render<Context> for variable {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.borrow_context().statement_type = StatementType::Variable;

        context.render(&self.modifiers);
        context.render_raw(&self.name);

        context.str(" = ").render_boxed(&self.body).str(";");
    }
}
