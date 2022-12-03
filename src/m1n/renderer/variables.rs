use crate::m1n::grammar::variable;

use crate::parser_generator::render::{OutputBuilder, Render};

use super::{Context, StatementType};

impl Render<Context> for variable {
    fn render(&self, builder: &mut OutputBuilder<Context>) {
        builder.borrow_context().statement_type = StatementType::Variable;

        builder.render(&self.modifiers);
        builder.render_raw(&self.name);

        builder.str(" = ").render_boxed(&self.body).str(";");
    }
}
