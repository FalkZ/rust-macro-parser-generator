#[feature(specialization)]

mod m1n;
mod parser_generator;


use m1n::file::compile_file;


fn main() {
  // compile_file("./examples/m1n/math.m1n");
  // compile_file("./examples/m1n/std.m1n");
   //compile_file("./examples/m1n/Class.m1n");
   compile_file("./examples/m1n/file.m1n");   

}
