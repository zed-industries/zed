use snafu::prelude::*;
use std::error::Error as StdError;

#[derive(Debug, Snafu)]
#[snafu(display("name: [{name}]"))]
struct Error {
    name: String,
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn example(name: &str) -> Result<()> {
    ensure!(name.is_empty(), Snafu { name });
    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: StdError>() {}
    check::<Error>();

    let name = "must be empty";
    let e = example(name).unwrap_err();
    assert_eq!(e.name, name);

    assert!(e.source().is_none());

    let display = e.to_string();
    assert_eq!(display, "name: [must be empty]");
}
