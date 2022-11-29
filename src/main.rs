#![allow(dead_code, unused_imports)]

mod m1n;
mod parser_generator;

use m1n::renderer::render;

fn main() {
    // compile_file("./examples/m1n/math.m1n");
    // compile_file("./examples/m1n/std.m1n");
    //compile_file("./examples/m1n/Class.m1n");
    render("./examples/m1n/file.m1n").expect("couldn't compile file");
    render("./examples/m1n/std/math.m1n").expect("couldn't compile file");
}
