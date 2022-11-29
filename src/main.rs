//#![allow(dead_code, unused_imports)]

mod m1n;
mod parser_generator;



fn main() {}

#[test]
fn test_enum() {
    let r = render("./examples/m1n/Enum.m1n").expect("couldn't compile file");

    insta::assert_debug_snapshot!(&r.tokens);
    insta::assert_debug_snapshot!(&r.statements);
    insta::assert_display_snapshot!(&r.typescript);
}

#[test]
fn test_class() {
    let r = render("./examples/m1n/Class.m1n").expect("couldn't compile file");

    insta::assert_debug_snapshot!(&r.tokens);
    insta::assert_debug_snapshot!(&r.statements);
    insta::assert_display_snapshot!(&r.typescript);
}

#[test]
fn test_file() {
    let r = render("./examples/m1n/file.m1n").expect("couldn't compile file");

    insta::assert_debug_snapshot!(&r.tokens);
    insta::assert_debug_snapshot!(&r.statements);
    insta::assert_display_snapshot!(&r.typescript);
}

#[test]
fn test_math() {
    let r = render("./examples/m1n/std/math.m1n").expect("couldn't compile file");

    insta::assert_debug_snapshot!(&r.tokens);
    insta::assert_debug_snapshot!(&r.statements);
    insta::assert_display_snapshot!(&r.typescript);
}
