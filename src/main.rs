mod lexer;
mod matchers;
mod parser;
mod result;
mod tokens;
mod traits;
mod file;
//od renderer;
//mod visitor;
mod grammar;
mod command;
mod sourcemap;
mod source;
mod render_macro;
mod new_renderer;

use file::compile_file;

use crate::sourcemap::test_source_map;

use crate::sourcemap::{create_source_map};





fn main() {
    // test_source_map("./test/Class.js", "PI = a + b + 0.012;\n\npub mut Pi = 123;\n\nfn(a, b) = a + b;\n\none(b) = a + b + c;\n\npub zero() = #(()=>{throw new Error(\"hello 2\")})()#;\n\nmultiline() = console.log(\"test space\", \"multi\n\n    line\");\n\nnesting() = one(fn(a + b - c, 123.2));")
  // compile_file("./examples/m1n/math.m1n");
  // compile_file("./examples/m1n/std.m1n");
   //compile_file("./examples/m1n/Class.m1n");
   compile_file("./examples/m1n/file.m1n");   


}
