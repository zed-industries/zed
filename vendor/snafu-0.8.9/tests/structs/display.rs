use snafu::prelude::*;

#[test]
fn defaults_to_name_of_struct() {
    #[derive(Debug, Snafu)]
    struct Error;

    let e = Snafu.build();
    let display = e.to_string();
    assert_eq!(display, "Error");
}

#[test]
fn doc_comment_used_as_display() {
    /// This is a bad thing
    ///
    /// It's an error!
    #[derive(Debug, Snafu)]
    struct Error;

    let e = Snafu.build();
    let display = e.to_string();
    assert_eq!(display, "This is a bad thing");
}

#[test]
fn attribute_is_stronger_than_doc_comment() {
    /// This is ignored
    #[derive(Debug, Snafu)]
    #[snafu(display("In favor of this"))]
    struct Error;

    let e = Snafu.build();
    let display = e.to_string();
    assert_eq!(display, "In favor of this");
}
