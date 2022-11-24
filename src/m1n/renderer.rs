use super::{grammar::{statements, statement, function, variable, maybe_arguments, arguments, argument }};

use crate::parser_generator::{sourcemap::{RenderContext}, render::{Render}};
use crate::render;


impl Render for argument {
    fn render(&self, context: &mut RenderContext) {
        render!(self, context, "{}", self.arg);
    }
}

impl Render for Vec<argument> {
    fn render(&self, context: &mut RenderContext) {
        self.iter().for_each(|v| render!(self, context, "{},", v));
    }
}

impl Render for arguments {
    fn render(&self, context: &mut RenderContext) {
        render!(self, context, "{}{}", self.arguments, self.last);
    }
}

impl Render for maybe_arguments {
    fn render(&self, context: &mut RenderContext) {
        match self {
            maybe_arguments::no_arguments(_) => render!(self, context, "()"),
            maybe_arguments::arguments(v) => render!(self, context, "({})", v)
        }
    }
}

impl Render for function  {
    fn render(&self, context: &mut RenderContext) {      
        context.str("fn{} \n");
    }
}

impl Render for variable  {
    fn render(&self, context: &mut RenderContext) {
       context.str("var \n");
    }
}

impl Render for statements  {
    fn render(&self, context: &mut RenderContext) {
        
        match *self.statement.to_owned() {
            statement::function(v) => 
                context.render(*v),
            statement::variable(v) =>  context.render(*v),
        };

    }
}

impl Render for Vec<statements>  {
    fn render(&self, context: &mut RenderContext) {

        context.join(self, &"\n\n");
        
    }
}



pub fn render(source_path: &str, source_content: &str,s: &Vec<statements>){

    let mut src = RenderContext::new(source_path);

    s.render(&mut src);

    src.write_file(Some(source_content));
}