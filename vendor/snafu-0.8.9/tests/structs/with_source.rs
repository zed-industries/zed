use snafu::prelude::*;
use std::{
    error::Error as StdError,
    fs, io,
    path::{Path, PathBuf},
    ptr,
};

#[derive(Debug, Snafu)]
#[snafu(display("filename: {}, source: {source}", filename.display()))]
struct Error {
    filename: PathBuf,
    source: io::Error,
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn example(filename: impl AsRef<Path>) -> Result<()> {
    let filename = filename.as_ref();

    let _config = fs::read(filename).context(Snafu { filename })?;

    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: StdError>() {}
    check::<Error>();

    let path = "/some/directory/that/does/not/exist";
    let e = example(path).unwrap_err();
    assert_eq!(e.filename, Path::new(path));

    let source = e.source().expect("Source must be present");
    let source_as_io_error = source
        .downcast_ref::<io::Error>()
        .expect("Source must be io::Error");
    assert!(ptr::eq(source_as_io_error, &e.source));

    let display = e.to_string();
    assert!(
        display.starts_with("filename: /some/directory/that/does/not/exist, source: "),
        "Display string incorrect, is: {}",
        display,
    );
}
