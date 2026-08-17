use std::fmt::Debug;

// `--features trace` code names the return type, so doesn't work with `impl Trait`
#[cfg(not(feature = "trace"))]
peg::parser!{
    grammar g() for str {
        pub rule returns_impl_trait() -> impl Debug
            = "" { Box::new(5) }
    }
}

fn main() {
    #[cfg(not(feature = "trace"))]
    assert_eq!(format!("{:?}", g::returns_impl_trait("")), "Ok(5)");
}
