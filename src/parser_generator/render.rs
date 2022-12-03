use std::{
    cell::RefCell,
    fmt::Display,
    fs::{self, File},
    path::Path,
    rc::Rc,
};

use sourcemap::SourceMapBuilder;

use super::{
    position::{Position, TryGetPosition},
    static_context::StaticContext,
    tokens::RawToken,
};

pub trait Render<Context: Clone>: TryGetPosition {
    fn render(&self, context: &mut OutputBuilder<Context>);
}

#[derive(Clone)]
pub struct OutputBuilder<C: Clone> {
    context: Box<C>,
    static_context: Rc<RefCell<StaticContext>>,
}

impl<C: Clone> OutputBuilder<C> {
    pub fn new(context: C) -> Self {
        Self {
            context: Box::new(context),
            static_context: Rc::new(RefCell::new(StaticContext::new())),
        }
    }

    pub fn get_context(&self) -> &C {
        &*self.context
    }

    pub fn borrow_context(&mut self) -> &mut C {
        &mut *self.context
    }

    pub fn set_context(&mut self, context: C) {
        self.context = Box::new(context)
    }

    fn add_string(&self, string: &str) {
        self.static_context.borrow_mut().add_string(string)
    }
    fn add_posititon(&self, pos: Option<Position>) {
        self.static_context.borrow_mut().add_posititon(pos)
    }

    pub fn str<D: Display>(&mut self, content: D) -> &mut Self {
        self.add_string(&format!("{}", content));

        self
    }

    pub fn render_raw<R: RawToken>(&mut self, content: &R) -> &mut Self {
        let t = content.raw_token();
        self.add_posititon(Some(t.position));
        self.add_string(&t.raw);

        self
    }

    pub fn render<T: Render<C> + Clone>(&mut self, content: &T) -> &mut Self {
        let c = content.clone();
        self.add_posititon(c.try_position());
        c.render(&mut self.clone());

        self
    }

    pub fn join<T: Render<C>, S: Display>(&mut self, content: &Vec<T>, separator: S) -> &mut Self {
        use crate::parser_generator::position::TryGetPosition;

        self.add_posititon(content.try_position());

        let last = content.len();
        content.iter().enumerate().for_each(|(i, v)| {
            v.render(&mut self.clone());

            if (i + 1) != last {
                self.str(format!("{}", separator));
            }
        });

        self
    }

    pub fn join_boxed<T: Render<C> + Clone, S: Display>(
        &mut self,
        content: &Vec<Box<T>>,
        separator: S,
    ) -> &mut Self {
        let last = content.len();
        content.iter().enumerate().for_each(|(i, v)| {
            self.render_boxed(&v);

            if (i + 1) != last {
                self.str(format!("{}", separator));
            }
        });

        self
    }

    pub fn join_raw<T: RawToken, S: Display>(
        &mut self,
        content: &Vec<T>,
        separator: S,
    ) -> &mut Self {
        use crate::parser_generator::position::TryGetPosition;

        self.add_posititon(content.try_position());

        let last = content.len();
        content.iter().enumerate().for_each(|(i, v)| {
            self.render_raw(v);

            if (i + 1) != last {
                self.str(format!("{}", separator));
            }
        });

        self
    }

    pub fn apply<'p, T: TryGetPosition, D: Display, F: Fn(&'p T) -> D>(
        &mut self,
        v: &'p T,
        f: F,
    ) -> &mut Self {
        self.add_posititon(v.try_position());

        self.str(f(v));

        self
    }

    pub fn write_files(&self, src_file_path: &str, source_content: Option<&str>) {
        self.static_context
            .borrow()
            .write_files(src_file_path, source_content);
    }
}

pub trait RenderBoxed<T: Clone> {
    fn render_boxed(&mut self, content: &T) -> &mut Self;
}

impl<C: Clone, T: Render<C> + Clone> RenderBoxed<Box<T>> for OutputBuilder<C> {
    fn render_boxed(&mut self, content: &Box<T>) -> &mut Self {
        let c = &(**content);
        self.render(c);

        self
    }
}
