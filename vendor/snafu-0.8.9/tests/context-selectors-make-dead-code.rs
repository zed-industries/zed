#![deny(dead_code)]

// This tests our workaround for
// https://github.com/rust-lang/rust/issues/141005

macro_rules! define_error {
    () => {
        #[derive(Debug, snafu::Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub struct TheError;
    };
}

define_error!();

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<TheError>();
}
