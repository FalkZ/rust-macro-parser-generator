macro_rules! tests {
    (@debug: $name:ident, $t:ident, $str:literal) => {
        #[test]
        fn $name() {
            let r = crate::m1n::renderer::render($str).expect("couldn't compile file");
            insta::assert_debug_snapshot!(&r.$t);
        }
    };
    (@display: $name:ident, $t:ident, $str:literal) => {
        #[test]
        fn $name() {
            let r = crate::m1n::renderer::render($str).expect("couldn't compile file");
            insta::assert_display_snapshot!(&r.$t);
        }
    };
    (@run: $name:ident, $t:ident, $str:literal) => {
        pub fn $name() {
           crate::m1n::renderer::render($str).expect("couldn't compile file");
        }
    };
    // `()` indicates that the macro takes no argument.
    ($($name:ident: $str:literal),*) => {
        pub mod run {
            $(
                tests!(@run: $name, run, $str);
            )*

            pub fn run() {
                $(
                self::$name();
                )*
            }
        }
        mod tokens {
            $(
                tests!(@debug: $name, tokens, $str);
            )*
        }
        mod statements {
            $(
                tests!(@debug: $name, statements, $str);
            )*
        }
        mod typescript {
            $(
                tests!(@display: $name, typescript, $str);
            )*
        }
        mod sourcemap {
            $(
                tests!(@display: $name, sourcemap, $str);
            )*
        }
    };
}

tests!(
    enums: "./examples/m1n/Enum.m1n",
    class: "./examples/m1n/Class.m1n",
    file: "./examples/m1n/file.m1n",
    math: "./examples/m1n/std/math.m1n",
    matcher: "./examples/m1n/match.m1n",
    assignment: "./examples/m1n/assignment.m1n",
    operator: "./examples/m1n/operator.m1n"
);
