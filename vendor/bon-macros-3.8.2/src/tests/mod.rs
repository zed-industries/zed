mod attr_setters;
mod syntax_errors;

use crate::util::prelude::*;
use expect_test::{expect_file, ExpectFile};

fn snapshot(test_name: &str) -> ExpectFile {
    let snapshot_path = format!(
        "{}/tests/snapshots/{test_name}.rs",
        env!("CARGO_MANIFEST_DIR")
    );
    expect_file![snapshot_path]
}

#[track_caller]
fn assert_snapshot(test_name: &'static str, actual: &dyn ToTokens) {
    let actual = prettyplease::unparse(&syn::parse2(actual.to_token_stream()).unwrap());
    snapshot(test_name).assert_eq(&actual);
}
