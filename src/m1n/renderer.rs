mod assignment;
mod body;
mod definitions;
mod enums;
mod expressions;
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
        render::{OutputBuilder, Render},
        result::ParserResult,
        tokens::Tokens,
    },
};

use super::grammar::value;

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
    single_expression: Option<Box<value>>,
}

pub struct Return {
    pub tokens: Tokens<super::grammar::Lexer>,
    pub statements: Vec<super::grammar::statements>,
    pub typescript: String,
    pub sourcemap: String,
}

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct SourceMapJson {
    mappings: String,
}

pub fn render(source_path: &str) -> ParserResult<Return> {
    let source_content = fs::read_to_string(source_path).expect("couldn't read file");

    let source_content = format!(
        "
import: 
    `@std/util`{{ pipe }},
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
        single_expression: None,
    };

    let mut src = OutputBuilder::new(context);

    statements.render(&mut src);

    src.write_files(source_path, Some(&source_content));

    let out_file_name = source_path.replace(".m1n", ".ts");
    let out_file_name_map = source_path.replace(".m1n", ".ts.map");
    let out_file_name_pretty = source_path.replace(".m1n", ".pretty.ts");

    fs::copy(&out_file_name, &out_file_name_pretty).expect("couldn't copy pretty");

    prettier_format(&out_file_name_pretty);

    let sourcemap = fs::read_to_string(&out_file_name_map).expect("couldn't read file");

    let typescript = fs::read_to_string(&out_file_name_pretty).expect("couldn't read file");

    let typescript_ugly = fs::read_to_string(&out_file_name).expect("couldn't read file");

    let s: SourceMapJson = serde_json::from_str(&sourcemap).expect("couldn't parse sourcemap");

    let s = s.mappings.replace(";", "\n");

    let sourcemap = format!("{}\n\n\n{}", typescript_ugly, s);

    Ok(Return {
        typescript,
        sourcemap,
        tokens: t.tokens,
        statements,
    })
}
