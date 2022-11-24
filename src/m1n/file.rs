
use std::{fs::{self}};



use crate::parser_generator::{result::ParserResult};

use super::{grammar::Parser, renderer::render};



pub fn compile_file(file_path: &str) -> ParserResult<()> {
    let contents = fs::read_to_string(file_path).expect("couldn't read file");



    //let name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();

    let t = Parser::new(&contents)?;

    println!("{:?}", &t.tokens);

    let t = Parser::statements(&t.tokens)?;

    /* 
    let v = Visitor {};

    let r = v.statements(&t);

    let context = if name.chars().nth(0).unwrap().is_uppercase() {
        RenderContext::Class(name.replace(".m1n", ""))
    } else {
        RenderContext::Singleton(name.replace(".m1n", ""))
    };

    let mut out = r.render(&context);
    let out_path = file_path.replace(".m1n", ".ts");

    out = format!("{}\n//# sourceMappingURL={}.map", &out, &name);

    fs::write(&out_path, out).unwrap();

    



    esbuild(&out_path);
    prettier_format(&out_path);

    */

    render(&file_path, &contents, &t);





    Ok(())
}