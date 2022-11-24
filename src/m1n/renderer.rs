use super::{grammar::{statements, statement, function, variable, maybe_arguments, arguments, argument }};

use crate::parser_generator::{sourcemap::{RenderContext}, render::{Render}};


impl Render for argument {
    fn render(&self, context: &mut RenderContext) {
       context.render_raw(&self.arg).str(", ");
    }
}


impl Render for arguments {
    fn render(&self, context: &mut RenderContext) {

        context.str("(")
        .join(&self.arguments, "")
        .render_raw(&self.last)
        .str(")");    
    }
}



impl Render for function  {
    fn render(&self, context: &mut RenderContext) {      
        context.str("fn{} \n");
        
        match &self.arguments {
            Some(v) => context.render(*v.clone()),
            None => context.str("()")
        };
        
        
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