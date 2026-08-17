use crate::cli::cmd::mdbook_cmd;
use crate::dummy_book::DummyBook;

use predicates::boolean::PredicateBooleanExt;

#[test]
fn mdbook_cli_can_correctly_test_a_passing_book() {
    let temp = DummyBook::new().with_passing_test(true).build().unwrap();

    let mut cmd = mdbook_cmd();
    cmd.arg("test").current_dir(temp.path());
    cmd.assert().success()
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "README.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "intro.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "first[\\/]index.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "first[\\/]nested.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"returned an error:\n\n"##).unwrap().not())
      .stderr(predicates::str::is_match(r##"Nested_Chapter::Rustdoc_include_works_with_anchors_too \(line \d+\) ... FAILED"##).unwrap().not());
}

#[test]
fn mdbook_cli_detects_book_with_failing_tests() {
    let temp = DummyBook::new().with_passing_test(false).build().unwrap();

    let mut cmd = mdbook_cmd();
    cmd.arg("test").current_dir(temp.path());
    cmd.assert().failure()
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "README.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "intro.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "first[\\/]index.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"Testing chapter [^:]*: "first[\\/]nested.md""##).unwrap())
      .stderr(predicates::str::is_match(r##"returned an error:\n\n"##).unwrap())
      .stderr(predicates::str::is_match(r##"Nested_Chapter::Rustdoc_include_works_with_anchors_too \(line \d+\) ... FAILED"##).unwrap());
}
