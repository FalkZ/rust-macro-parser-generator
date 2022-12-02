mod body;
mod definitions;
mod enums;
mod functions;
mod match_statement;
mod modifiers;
mod statements;
pub mod substring;
mod values;
mod variables;

use std::{
    fs::{self},
    path::Path,
};

use crate::{
    m1n::{command::prettier_format, grammar::Parser},
    parser_generator::{
        render::{Render, RenderContext},
        result::ParserResult,
        tokens::Tokens,
    },
};

#[derive(Clone, PartialEq)]
enum FileType {
    Class,
    Enum(Vec<String>),
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

pub struct Return {
    pub tokens: Tokens<super::grammar::Lexer>,
    pub statements: Vec<super::grammar::statements>,
    pub typescript: String,
}

pub fn render(source_path: &str) -> ParserResult<Return> {
    let source_content = fs::read_to_string(source_path).expect("couldn't read file");

    let source_content = format!(
        "
import: 
    `@std/util`{{ pipe, match }},
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

    let out_file_name = source_path.replace(".m1n", ".ts");
    prettier_format(&out_file_name);

    let typescript = fs::read_to_string(&out_file_name).expect("couldn't read file");

    Ok(Return {
        typescript,
        tokens: t.tokens,
        statements,
    })
}
