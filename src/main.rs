mod lexer;
mod matchers;
mod parser;
mod result;
mod tokens;
mod traits;
mod file;
mod renderer;
mod visitor;
mod grammar;
mod command;

use file::compile_file;





fn main() {
    compile_file("./examples/Class.m1n");
    compile_file("./examples/file.m1n");
}
