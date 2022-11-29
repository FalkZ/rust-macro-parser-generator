mod body;
mod definitions;
mod shell;
mod statements;
pub mod substring;

use std::{
    fs::{self, File},
    path::Path,
};

use sourcemap::SourceMapBuilder;

use super::grammar::{
    argument, arguments, function, maybe_arguments, modifiers, name, statement, variable,
};

use crate::{
    m1n::{command::prettier_format, grammar::Parser},
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
    },
};

#[derive(Clone)]
enum FileType {
    Class,
    Singleton,
}

#[derive(Clone)]
enum StatementType {
    Variable,
    RawFunction,
    Function,
    None,
}

#[derive(Clone)]
struct Context {
    statement_type: StatementType,
    file_type: FileType,
    name: String,
}

pub fn render(source_path: &str) -> ParserResult<()> {
    let source_content = fs::read_to_string(source_path).expect("couldn't read file");

    let source_content = format!(
        "
import: `@std/util`{{ pipe }},
    `@std/math`;
{}",
        source_content
    );

    let t = Parser::new(&source_content)?;

    println!("{}", &t.tokens);

    let statements = Parser::statements(&t.tokens)?;

    println!("{:#?}", &statements); // AST

    let name = Path::new(&source_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".m1n", "");

    let file_type: FileType = if name.chars().nth(0).unwrap().is_uppercase() {
        FileType::Class
    } else {
        FileType::Singleton
    };

    //esbuild(&out_path);

    let context = Context {
        file_type,
        name,
        statement_type: StatementType::None,
    };

    let mut src = RenderContext::new(context);

    statements.render(&mut src);

    src.write_files(source_path, Some(&source_content));

    prettier_format(&source_path.replace(".m1n", ".ts"));

    Ok(())
}
