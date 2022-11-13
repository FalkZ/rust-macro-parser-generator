
use std::{fs, path::Path};

use crate::{result::ParserResult, grammar::Parser, visitor::Visitor, renderer::{RenderContext, Render}};


pub fn compile_file(file_path: &str) -> ParserResult<()> {
    let contents = fs::read_to_string(file_path).expect("couldn't read file");

    let name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();

    let t = Parser::new(&contents)?;

    println!("{:?}", &t.tokens);

    let t = Parser::statements(&t.tokens)?;

    let v = Visitor {};

    let r = v.statements(&t);

    let context = if name.chars().nth(0).unwrap().is_uppercase() {
        RenderContext::Class
    } else {
        RenderContext::Singleton
    };

    let out = r.render(&context);

    fs::write(file_path.replace(".m1n", ".ts"), out).unwrap();

    Ok(())
}