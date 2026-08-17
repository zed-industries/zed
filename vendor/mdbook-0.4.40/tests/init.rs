use mdbook::config::Config;
use mdbook::MDBook;
use pretty_assertions::assert_eq;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tempfile::Builder as TempFileBuilder;

/// Run `mdbook init` in an empty directory and make sure the default files
/// are created.
#[test]
fn base_mdbook_init_should_create_default_content() {
    let created_files = vec!["book", "src", "src/SUMMARY.md", "src/chapter_1.md"];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    for file in &created_files {
        assert!(!temp.path().join(file).exists());
    }

    MDBook::init(temp.path()).build().unwrap();

    for file in &created_files {
        let target = temp.path().join(file);
        println!("{}", target.display());
        assert!(target.exists(), "{} doesn't exist", file);
    }

    let contents = fs::read_to_string(temp.path().join("book.toml")).unwrap();
    assert_eq!(
        contents,
        "[book]\nauthors = []\nlanguage = \"en\"\nmultilingual = false\nsrc = \"src\"\n"
    );
}

/// Run `mdbook init` in a directory containing a SUMMARY.md should create the
/// files listed in the summary.
#[test]
fn run_mdbook_init_should_create_content_from_summary() {
    let created_files = vec!["intro.md", "first.md", "outro.md"];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let src_dir = temp.path().join("src");
    fs::create_dir_all(src_dir.clone()).unwrap();
    static SUMMARY: &str = r#"# Summary

[intro](intro.md)

- [First chapter](first.md)

[outro](outro.md)

"#;

    let mut summary = File::create(src_dir.join("SUMMARY.md")).unwrap();
    summary.write_all(SUMMARY.as_bytes()).unwrap();
    MDBook::init(temp.path()).build().unwrap();

    for file in &created_files {
        let target = src_dir.join(file);
        println!("{}", target.display());
        assert!(target.exists(), "{} doesn't exist", file);
    }
}

/// Set some custom arguments for where to place the source and destination
/// files, then call `mdbook init`.
#[test]
fn run_mdbook_init_with_custom_book_and_src_locations() {
    let created_files = vec!["out", "in", "in/SUMMARY.md", "in/chapter_1.md"];

    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    for file in &created_files {
        assert!(
            !temp.path().join(file).exists(),
            "{} shouldn't exist yet!",
            file
        );
    }

    let mut cfg = Config::default();
    cfg.book.src = PathBuf::from("in");
    cfg.build.build_dir = PathBuf::from("out");

    MDBook::init(temp.path()).with_config(cfg).build().unwrap();

    for file in &created_files {
        let target = temp.path().join(file);
        assert!(
            target.exists(),
            "{} should have been created by `mdbook init`",
            file
        );
    }

    let contents = fs::read_to_string(temp.path().join("book.toml")).unwrap();
    assert_eq!(
        contents,
        "[book]\nauthors = []\nlanguage = \"en\"\nmultilingual = false\nsrc = \"in\"\n\n[build]\nbuild-dir = \"out\"\ncreate-missing = true\nextra-watch-dirs = []\nuse-default-preprocessors = true\n"
    );
}

#[test]
fn book_toml_isnt_required() {
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let md = MDBook::init(temp.path()).build().unwrap();

    let _ = fs::remove_file(temp.path().join("book.toml"));

    md.build().unwrap();
}

#[test]
fn copy_theme() {
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    MDBook::init(temp.path()).copy_theme(true).build().unwrap();
    let expected = vec![
        "book.js",
        "css/chrome.css",
        "css/general.css",
        "css/print.css",
        "css/variables.css",
        "favicon.png",
        "favicon.svg",
        "fonts/OPEN-SANS-LICENSE.txt",
        "fonts/SOURCE-CODE-PRO-LICENSE.txt",
        "fonts/fonts.css",
        "fonts/open-sans-v17-all-charsets-300.woff2",
        "fonts/open-sans-v17-all-charsets-300italic.woff2",
        "fonts/open-sans-v17-all-charsets-600.woff2",
        "fonts/open-sans-v17-all-charsets-600italic.woff2",
        "fonts/open-sans-v17-all-charsets-700.woff2",
        "fonts/open-sans-v17-all-charsets-700italic.woff2",
        "fonts/open-sans-v17-all-charsets-800.woff2",
        "fonts/open-sans-v17-all-charsets-800italic.woff2",
        "fonts/open-sans-v17-all-charsets-italic.woff2",
        "fonts/open-sans-v17-all-charsets-regular.woff2",
        "fonts/source-code-pro-v11-all-charsets-500.woff2",
        "highlight.css",
        "highlight.js",
        "index.hbs",
    ];
    let theme_dir = temp.path().join("theme");
    let mut actual: Vec<_> = walkdir::WalkDir::new(&theme_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .map(|e| {
            e.path()
                .strip_prefix(&theme_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/")
        })
        .collect();
    actual.sort();
    assert_eq!(actual, expected);
}
