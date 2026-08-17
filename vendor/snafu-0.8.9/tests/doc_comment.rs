use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    /// No user available.
    /// You may need to specify one.
    ///
    /// Here is a more detailed description.
    MissingUser,

    /// This is just a doc comment.
    #[snafu(display("This is {stronger}"))]
    Stronger { stronger: &'static str },

    #[doc(hidden)]
    _Hidden,
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
}

#[test]
fn uses_doc_comment() {
    assert_eq!(
        Error::MissingUser.to_string(),
        "No user available. You may need to specify one.",
    );
}

#[test]
fn display_is_stronger_than_doc_comment() {
    assert_eq!(
        Error::Stronger {
            stronger: "always stronger!"
        }
        .to_string(),
        "This is always stronger!",
    );
}
