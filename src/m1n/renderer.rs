use std::{fs::{File, self}, path::Path};

use sourcemap::SourceMapBuilder;

use super::{grammar::{statements, statement, function, variable, maybe_arguments, arguments, argument }};

use crate::{parser_generator::{render::{Render, RenderContext}, result::ParserResult}, m1n::grammar::Parser};

 

impl Render<Context> for argument {
    fn render(&self, context: &mut RenderContext<Context>) {
       context.render_raw(&self.arg).str(", ");
    }
}


impl Render<Context> for arguments {
    fn render(&self, context: &mut RenderContext<Context>) {

        context.str("(")
        .join(&self.arguments, "")
        .render_raw(&self.last)
        .str(")");    
    }
}



impl Render<Context> for function  {
    fn render(&self, context: &mut RenderContext<Context>) {      
        context.str("fn{} \n");
        
        match &self.arguments {
            Some(v) => context.render(*v.clone()),
            None => context.str("()")
        };
        
        
    }
}

impl Render<Context> for variable  {
    fn render(&self, context: &mut RenderContext<Context>) {
       context.str("var \n");
    }
}

impl Render<Context> for statements  {
    fn render(&self, context: &mut RenderContext<Context>) {
        
        match *self.statement.to_owned() {
            statement::function(v) => 
                context.render(*v),
            statement::variable(v) =>  context.render(*v),
        };

    }
}

impl Render<Context> for Vec<statements>  {
    fn render(&self, context: &mut RenderContext<Context>) {

        context.join(self, &"\n\n");
        
    }
}


#[derive(Clone)]
enum FileType {
    Class,
    Singleton
}

#[derive(Clone)]
struct Context {
    file_type: FileType,
    name: String

}


pub fn render(source_path: &str) -> ParserResult<()> {

    let source_content = fs::read_to_string(source_path).expect("couldn't read file");


    let t = Parser::new(&source_content)?;

    println!("{}", &t.tokens);

    let statements = Parser::statements(&t.tokens)?;

    //println!("{:#?}", &t); // AST

    

    let name = Path::new(&source_path).file_name().unwrap().to_str().unwrap().replace(".m1n", "");

    let file_type: FileType = if name.chars().nth(0).unwrap().is_uppercase() {
        FileType::Class
    } else { 
        FileType::Singleton   
    };  


    //esbuild(&out_path);
    //prettier_format(&out_path);


    let context = Context{file_type, name};

    let mut src = RenderContext::new(source_path, context);

    statements.render(&mut src);

    src.write_files(source_path, Some(&source_content));

    Ok(())
}