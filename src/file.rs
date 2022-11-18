
use std::{fs::{self}, path::Path, thread::sleep, time::Duration};



use crate::{result::ParserResult, grammar::Parser, visitor::Visitor, renderer::{RenderContext, Render}, command::{prettier_format, esbuild}};



pub fn compile_file(file_path: &str) -> ParserResult<()> {
    let contents = fs::read_to_string(file_path).expect("couldn't read file");



    let name = Path::new(&file_path).file_name().unwrap().to_str().unwrap();

    let t = Parser::new(&contents)?;

    println!("{:?}", &t.tokens);

    let t = Parser::statements(&t.tokens)?;

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

    sleep(Duration::from_secs(1));


    let p = format!("{}.map", out_path.replace(".ts", ".js"));

    let c = fs::read_to_string(&p).expect("couldn't read file");

    let src = format!("\"sourcesContent\":[\"{}\"]", &contents.replace("\n", "\\n").replace("\"", "\\\""));

    let c = c.replace("\"sourcesContent\": [null]", &src );


    fs::write(&p, &c).unwrap();


    Ok(())
}