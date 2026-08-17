use crate::cli::cmd::mdbook_cmd;
use crate::dummy_book::DummyBook;

#[test]
fn mdbook_cli_dummy_book_generates_index_html() {
    let temp = DummyBook::new().build().unwrap();

    // doesn't exist before
    assert!(!temp.path().join("book").exists());

    let mut cmd = mdbook_cmd();
    cmd.arg("build").current_dir(temp.path());
    cmd.assert()
        .success()
        .stderr(
            predicates::str::is_match(r##"Stack depth exceeded in first[\\/]recursive.md."##)
                .unwrap(),
        )
        .stderr(predicates::str::contains(
            r##"[INFO] (mdbook::book): Running the html backend"##,
        ));

    // exists afterward
    assert!(temp.path().join("book").exists());

    let index_file = temp.path().join("book/index.html");
    assert!(index_file.exists());
}
