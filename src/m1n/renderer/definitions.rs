use super::substring::Substring;
use super::Context;
use crate::{
    m1n::grammar::{import, import_items, imports, IDENT, RAWIDENT},
    parser_generator::render::{Render, RenderContext},
};

impl Render<Context> for import_items {
    fn render(&self, context: &mut RenderContext<Context>) {
        let mut raw: Vec<RAWIDENT> = vec![];
        let mut norm: Vec<IDENT> = vec![];

        self.items.iter().for_each(|v| match *v.item.clone() {
            crate::m1n::grammar::name::RAWIDENT(r) => raw.push(RAWIDENT(r)),
            crate::m1n::grammar::name::IDENT(i) => norm.push(IDENT(i)),
        });

        if norm.len() > 0 {
            context.str(", { ").join_raw(&norm, ", ").str(" }");
        }
    }
}

impl Render<Context> for import {
    fn render(&self, context: &mut RenderContext<Context>) {
        let path: String = self.path.0.as_str().substring(1, -1);

        let name = path.split("/").last().unwrap();

        context
            .str("import ")
            .apply(&self.path, |_v: &RAWIDENT| &name);

        if let Some(args) = &self.import_items {
            context.render_boxed(args);
        }

        context
            .str(" from ")
            .apply(&self.path, |_v: &RAWIDENT| format!("'{}'", &path))
            .str(";");
    }
}

impl Render<Context> for imports {
    fn render(&self, context: &mut RenderContext<Context>) {
        context.join(&self.imports, "\n");
    }
}
