mod dummy_book;

use crate::dummy_book::{assert_contains_strings, assert_doesnt_contain_strings, DummyBook};

use anyhow::Context;
use mdbook::config::Config;
use mdbook::errors::*;
use mdbook::utils::fs::write_file;
use mdbook::MDBook;
use pretty_assertions::assert_eq;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::str::FromStr;
use tempfile::Builder as TempFileBuilder;
use walkdir::{DirEntry, WalkDir};

const BOOK_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/dummy_book");
const TOC_TOP_LEVEL: &[&str] = &[
    "1. First Chapter",
    "2. Second Chapter",
    "Conclusion",
    "Dummy Book",
    "Introduction",
];
const TOC_SECOND_LEVEL: &[&str] = &[
    "1.1. Nested Chapter",
    "1.2. Includes",
    "1.3. Recursive",
    "1.4. Markdown",
    "1.5. Unicode",
    "1.6. No Headers",
    "1.7. Duplicate Headers",
    "1.8. Heading Attributes",
    "2.1. Nested Chapter",
];

/// Make sure you can load the dummy book and build it without panicking.
#[test]
fn build_the_dummy_book() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();

    md.build().unwrap();
}

#[test]
fn by_default_mdbook_generates_rendered_content_in_the_book_directory() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();

    assert!(!temp.path().join("book").exists());
    md.build().unwrap();

    assert!(temp.path().join("book").exists());
    let index_file = md.build_dir_for("html").join("index.html");
    assert!(index_file.exists());
}

#[test]
fn make_sure_bottom_level_files_contain_links_to_chapters() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let dest = temp.path().join("book");
    let links = vec![
        r#"href="intro.html""#,
        r#"href="first/index.html""#,
        r#"href="first/nested.html""#,
        r#"href="second.html""#,
        r#"href="conclusion.html""#,
    ];

    let files_in_bottom_dir = vec!["index.html", "intro.html", "second.html", "conclusion.html"];

    for filename in files_in_bottom_dir {
        assert_contains_strings(dest.join(filename), &links);
    }
}

#[test]
fn check_correct_cross_links_in_nested_dir() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let first = temp.path().join("book").join("first");
    let links = vec![
        r#"href="../intro.html""#,
        r#"href="../first/index.html""#,
        r#"href="../first/nested.html""#,
        r#"href="../second.html""#,
        r#"href="../conclusion.html""#,
    ];

    let files_in_nested_dir = vec!["index.html", "nested.html"];

    for filename in files_in_nested_dir {
        assert_contains_strings(first.join(filename), &links);
    }

    assert_contains_strings(
        first.join("index.html"),
        &[r##"<h2 id="some-section"><a class="header" href="#some-section">"##],
    );

    assert_contains_strings(
        first.join("nested.html"),
        &[r##"<h2 id="some-section"><a class="header" href="#some-section">"##],
    );
}

#[test]
fn check_correct_relative_links_in_print_page() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let first = temp.path().join("book");

    assert_contains_strings(
        first.join("print.html"),
        &[
            r##"<a href="second/../first/nested.html">the first section</a>,"##,
            r##"<a href="second/../../std/foo/bar.html">outside</a>"##,
            r##"<img src="second/../images/picture.png" alt="Some image" />"##,
            r##"<a href="second/nested.html#some-section">fragment link</a>"##,
            r##"<a href="second/../first/markdown.html">HTML Link</a>"##,
            r##"<img src="second/../images/picture.png" alt="raw html">"##,
        ],
    );
}

#[test]
fn rendered_code_has_playground_stuff() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let nested = temp.path().join("book/first/nested.html");
    let playground_class = vec![r#"class="playground""#];

    assert_contains_strings(nested, &playground_class);

    let book_js = temp.path().join("book/book.js");
    assert_contains_strings(book_js, &[".playground"]);
}

#[test]
fn rendered_code_does_not_have_playground_stuff_in_html_when_disabled_in_config() {
    let temp = DummyBook::new().build().unwrap();
    let config = Config::from_str(
        "
    [output.html.playground]
    runnable = false
    ",
    )
    .unwrap();
    let md = MDBook::load_with_config(temp.path(), config).unwrap();
    md.build().unwrap();

    let nested = temp.path().join("book/first/nested.html");
    let playground_class = vec![r#"class="playground""#];

    assert_doesnt_contain_strings(nested, &playground_class);
}

#[test]
fn anchors_include_text_between_but_not_anchor_comments() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let nested = temp.path().join("book/first/nested.html");
    let text_between_anchors = vec!["unique-string-for-anchor-test"];
    let anchor_text = vec!["ANCHOR"];

    assert_contains_strings(nested.clone(), &text_between_anchors);
    assert_doesnt_contain_strings(nested, &anchor_text);
}

#[test]
fn rustdoc_include_hides_the_unspecified_part_of_the_file() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let nested = temp.path().join("book/first/nested.html");
    let text = vec![
        "<span class=\"boring\">fn some_function() {",
        "<span class=\"boring\">fn some_other_function() {",
    ];

    assert_contains_strings(nested, &text);
}

#[test]
fn chapter_content_appears_in_rendered_document() {
    let content = vec![
        ("index.html", "This file is just here to cause the"),
        ("intro.html", "Here's some interesting text"),
        ("second.html", "Second Chapter"),
        ("first/nested.html", "testable code"),
        ("first/index.html", "more text"),
        ("conclusion.html", "Conclusion"),
    ];

    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let destination = temp.path().join("book");

    for (filename, text) in content {
        let path = destination.join(filename);
        assert_contains_strings(path, &[text]);
    }
}

/// Apply a series of predicates to some root predicate, where each
/// successive predicate is the descendant of the last one. Similar to how you
/// might do `ul.foo li a` in CSS to access all anchor tags in the `foo` list.
macro_rules! descendants {
    ($root:expr, $($child:expr),*) => {
        $root
        $(
            .descendant($child)
        )*
    };
}

/// Make sure that all `*.md` files (excluding `SUMMARY.md`) were rendered
/// and placed in the `book` directory with their extensions set to `*.html`.
#[test]
fn chapter_files_were_rendered_to_html() {
    let temp = DummyBook::new().build().unwrap();
    let src = Path::new(BOOK_ROOT).join("src");

    let chapter_files = WalkDir::new(&src)
        .into_iter()
        .filter_entry(|entry| entry_ends_with(entry, ".md"))
        .filter_map(std::result::Result::ok)
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.file_name().and_then(OsStr::to_str) != Some("SUMMARY.md"));

    for chapter in chapter_files {
        let rendered_location = temp
            .path()
            .join(chapter.strip_prefix(&src).unwrap())
            .with_extension("html");
        assert!(
            rendered_location.exists(),
            "{} doesn't exits",
            rendered_location.display()
        );
    }
}

fn entry_ends_with(entry: &DirEntry, ending: &str) -> bool {
    entry.file_name().to_string_lossy().ends_with(ending)
}

/// Read the main page (`book/index.html`) and expose it as a DOM which we
/// can search with the `select` crate
fn root_index_html() -> Result<Document> {
    let temp = DummyBook::new()
        .build()
        .with_context(|| "Couldn't create the dummy book")?;
    MDBook::load(temp.path())?
        .build()
        .with_context(|| "Book building failed")?;

    let index_page = temp.path().join("book").join("index.html");
    let html = fs::read_to_string(index_page).with_context(|| "Unable to read index.html")?;

    Ok(Document::from(html.as_str()))
}

#[test]
fn check_second_toc_level() {
    let doc = root_index_html().unwrap();
    let mut should_be = Vec::from(TOC_SECOND_LEVEL);
    should_be.sort_unstable();

    let pred = descendants!(
        Class("chapter"),
        Name("li"),
        Name("li"),
        Name("a").and(Class("toggle").not())
    );

    let mut children_of_children: Vec<_> = doc
        .find(pred)
        .map(|elem| elem.text().trim().to_string())
        .collect();
    children_of_children.sort();

    assert_eq!(children_of_children, should_be);
}

#[test]
fn check_first_toc_level() {
    let doc = root_index_html().unwrap();
    let mut should_be = Vec::from(TOC_TOP_LEVEL);

    should_be.extend(TOC_SECOND_LEVEL);
    should_be.sort_unstable();

    let pred = descendants!(
        Class("chapter"),
        Name("li"),
        Name("a").and(Class("toggle").not())
    );

    let mut children: Vec<_> = doc
        .find(pred)
        .map(|elem| elem.text().trim().to_string())
        .collect();
    children.sort();

    assert_eq!(children, should_be);
}

#[test]
fn check_spacers() {
    let doc = root_index_html().unwrap();
    let should_be = 2;

    let num_spacers = doc
        .find(Class("chapter").descendant(Name("li").and(Class("spacer"))))
        .count();
    assert_eq!(num_spacers, should_be);
}

/// Ensure building fails if `create-missing` is false and one of the files does
/// not exist.
#[test]
fn failure_on_missing_file() {
    let temp = DummyBook::new().build().unwrap();
    fs::remove_file(temp.path().join("src").join("intro.md")).unwrap();

    let mut cfg = Config::default();
    cfg.build.create_missing = false;

    let got = MDBook::load_with_config(temp.path(), cfg);
    assert!(got.is_err());
}

/// Ensure a missing file is created if `create-missing` is true.
#[test]
fn create_missing_file_with_config() {
    let temp = DummyBook::new().build().unwrap();
    fs::remove_file(temp.path().join("src").join("intro.md")).unwrap();

    let mut cfg = Config::default();
    cfg.build.create_missing = true;

    assert!(!temp.path().join("src").join("intro.md").exists());
    let _md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    assert!(temp.path().join("src").join("intro.md").exists());
}

/// This makes sure you can include a Rust file with `{{#playground example.rs}}`.
/// Specification is in `guide/src/format/rust.md`
#[test]
fn able_to_include_playground_files_in_chapters() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let second = temp.path().join("book/second.html");

    let playground_strings = &[r#"class="playground""#, r#"println!("Hello World!");"#];

    assert_contains_strings(&second, playground_strings);
    assert_doesnt_contain_strings(&second, &["{{#playground example.rs}}"]);
}

/// This makes sure you can include a Rust file with `{{#include ../SUMMARY.md}}`.
#[test]
fn able_to_include_files_in_chapters() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let includes = temp.path().join("book/first/includes.html");

    let summary_strings = &[
        r##"<h1 id="summary"><a class="header" href="#summary">Summary</a></h1>"##,
        ">First Chapter</a>",
    ];
    assert_contains_strings(&includes, summary_strings);

    assert_doesnt_contain_strings(&includes, &["{{#include ../SUMMARY.md::}}"]);
}

/// Ensure cyclic includes are capped so that no exceptions occur
#[test]
fn recursive_includes_are_capped() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let recursive = temp.path().join("book/first/recursive.html");
    let content = &["Around the world, around the world
Around the world, around the world
Around the world, around the world"];
    assert_contains_strings(recursive, content);
}

#[test]
fn example_book_can_build() {
    let example_book_dir = dummy_book::new_copy_of_example_book().unwrap();

    let md = MDBook::load(example_book_dir.path()).unwrap();

    md.build().unwrap();
}

#[test]
fn book_with_a_reserved_filename_does_not_build() {
    let tmp_dir = TempFileBuilder::new().prefix("mdBook").tempdir().unwrap();
    let src_path = tmp_dir.path().join("src");
    fs::create_dir(&src_path).unwrap();

    let summary_path = src_path.join("SUMMARY.md");
    let print_path = src_path.join("print.md");

    fs::File::create(print_path).unwrap();
    let mut summary_file = fs::File::create(summary_path).unwrap();
    writeln!(summary_file, "[print](print.md)").unwrap();

    let md = MDBook::load(tmp_dir.path()).unwrap();
    let got = md.build();
    assert!(got.is_err());
}

#[test]
fn by_default_mdbook_use_index_preprocessor_to_convert_readme_to_index() {
    let temp = DummyBook::new().build().unwrap();
    let mut cfg = Config::default();
    cfg.set("book.src", "src2")
        .expect("Couldn't set config.book.src to \"src2\".");
    let md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    md.build().unwrap();

    let first_index = temp.path().join("book").join("first").join("index.html");
    let expected_strings = vec![
        r#"href="../first/index.html""#,
        r#"href="../second/index.html""#,
        "First README",
    ];
    assert_contains_strings(&first_index, &expected_strings);
    assert_doesnt_contain_strings(&first_index, &["README.html"]);

    let second_index = temp.path().join("book").join("second").join("index.html");
    let unexpected_strings = vec!["Second README"];
    assert_doesnt_contain_strings(second_index, &unexpected_strings);
}

#[test]
fn first_chapter_is_copied_as_index_even_if_not_first_elem() {
    let temp = DummyBook::new().build().unwrap();
    let mut cfg = Config::default();
    cfg.set("book.src", "index_html_test")
        .expect("Couldn't set config.book.src to \"index_html_test\"");
    let md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    md.build().unwrap();

    let root = temp.path().join("book");
    let chapter = fs::read_to_string(root.join("chapter_1.html")).expect("read chapter 1");
    let index = fs::read_to_string(root.join("index.html")).expect("read index");
    pretty_assertions::assert_eq!(chapter, index);
}

#[test]
fn theme_dir_overrides_work_correctly() {
    let book_dir = dummy_book::new_copy_of_example_book().unwrap();
    let book_dir = book_dir.path();
    let theme_dir = book_dir.join("theme");

    let mut index = mdbook::theme::INDEX.to_vec();
    index.extend_from_slice(b"\n<!-- This is a modified index.hbs! -->");

    write_file(&theme_dir, "index.hbs", &index).unwrap();

    let md = MDBook::load(book_dir).unwrap();
    md.build().unwrap();

    let built_index = book_dir.join("book").join("index.html");
    dummy_book::assert_contains_strings(built_index, &["This is a modified index.hbs!"]);
}

#[test]
fn no_index_for_print_html() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let print_html = temp.path().join("book/print.html");
    assert_contains_strings(print_html, &[r##"noindex"##]);

    let index_html = temp.path().join("book/index.html");
    assert_doesnt_contain_strings(index_html, &[r##"noindex"##]);
}

#[test]
fn markdown_options() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let path = temp.path().join("book/first/markdown.html");
    assert_contains_strings(
        &path,
        &[
            "<th>foo</th>",
            "<th>bar</th>",
            "<td>baz</td>",
            "<td>bim</td>",
        ],
    );
    assert_contains_strings(
        &path,
        &[
            r##"<sup class="footnote-reference"><a href="#1">1</a></sup>"##,
            r##"<sup class="footnote-reference"><a href="#word">2</a></sup>"##,
            r##"<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>"##,
            r##"<div class="footnote-definition" id="word"><sup class="footnote-definition-label">2</sup>"##,
        ],
    );
    assert_contains_strings(&path, &["<del>strikethrough example</del>"]);
    assert_contains_strings(
        &path,
        &[
            "<li><input disabled=\"\" type=\"checkbox\" checked=\"\"/>\nApples",
            "<li><input disabled=\"\" type=\"checkbox\" checked=\"\"/>\nBroccoli",
            "<li><input disabled=\"\" type=\"checkbox\"/>\nCarrots",
        ],
    );
}

#[test]
fn redirects_are_emitted_correctly() {
    let temp = DummyBook::new().build().unwrap();
    let mut md = MDBook::load(temp.path()).unwrap();

    // override the "outputs.html.redirect" table
    let redirects: HashMap<PathBuf, String> = vec![
        (PathBuf::from("/overview.html"), String::from("index.html")),
        (
            PathBuf::from("/nexted/page.md"),
            String::from("https://rust-lang.org/"),
        ),
    ]
    .into_iter()
    .collect();
    md.config.set("output.html.redirect", &redirects).unwrap();

    md.build().unwrap();

    for (original, redirect) in &redirects {
        let mut redirect_file = md.build_dir_for("html");
        // append everything except the bits that make it absolute
        // (e.g. "/" or "C:\")
        redirect_file.extend(remove_absolute_components(original));
        let contents = fs::read_to_string(&redirect_file).unwrap();
        assert!(contents.contains(redirect));
    }
}

#[test]
fn edit_url_has_default_src_dir_edit_url() {
    let temp = DummyBook::new().build().unwrap();
    let book_toml = r#"
        [book]
        title = "implicit"

        [output.html]
        edit-url-template = "https://github.com/rust-lang/mdBook/edit/master/guide/{path}"    
        "#;

    write_file(temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(
        index_html,
        &[
            r#"href="https://github.com/rust-lang/mdBook/edit/master/guide/src/README.md" title="Suggest an edit""#,
        ],
    );
}

#[test]
fn edit_url_has_configured_src_dir_edit_url() {
    let temp = DummyBook::new().build().unwrap();
    let book_toml = r#"
        [book]
        title = "implicit"
        src = "src2"

        [output.html]
        edit-url-template = "https://github.com/rust-lang/mdBook/edit/master/guide/{path}"    
        "#;

    write_file(temp.path(), "book.toml", book_toml.as_bytes()).unwrap();

    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let index_html = temp.path().join("book").join("index.html");
    assert_contains_strings(
        index_html,
        &[
            r#"href="https://github.com/rust-lang/mdBook/edit/master/guide/src2/README.md" title="Suggest an edit""#,
        ],
    );
}

fn remove_absolute_components(path: &Path) -> impl Iterator<Item = Component> + '_ {
    path.components()
        .skip_while(|c| matches!(c, Component::Prefix(_) | Component::RootDir))
}

/// Checks formatting of summary names with inline elements.
#[test]
fn summary_with_markdown_formatting() {
    let temp = DummyBook::new().build().unwrap();
    let mut cfg = Config::default();
    cfg.set("book.src", "summary-formatting").unwrap();
    let md = MDBook::load_with_config(temp.path(), cfg).unwrap();
    md.build().unwrap();

    let rendered_path = temp.path().join("book/formatted-summary.html");
    assert_contains_strings(
        rendered_path,
        &[
            r#"<a href="formatted-summary.html" class="active"><strong aria-hidden="true">1.</strong> Italic code *escape* `escape2`</a>"#,
            r#"<a href="soft.html"><strong aria-hidden="true">2.</strong> Soft line break</a>"#,
            r#"<a href="escaped-tag.html"><strong aria-hidden="true">3.</strong> &lt;escaped tag&gt;</a>"#,
        ],
    );

    let generated_md = temp.path().join("summary-formatting/formatted-summary.md");
    assert_eq!(
        fs::read_to_string(generated_md).unwrap(),
        "# Italic code *escape* `escape2`\n"
    );
    let generated_md = temp.path().join("summary-formatting/soft.md");
    assert_eq!(
        fs::read_to_string(generated_md).unwrap(),
        "# Soft line break\n"
    );
    let generated_md = temp.path().join("summary-formatting/escaped-tag.md");
    assert_eq!(
        fs::read_to_string(generated_md).unwrap(),
        "# &lt;escaped tag&gt;\n"
    );
}

/// Ensure building fails if `[output.html].theme` points to a non-existent directory
#[test]
fn failure_on_missing_theme_directory() {
    // 1. Using default theme should work
    let temp = DummyBook::new().build().unwrap();
    let book_toml = r#"
        [book]
        title = "implicit"
        src = "src"
        "#;

    write_file(temp.path(), "book.toml", book_toml.as_bytes()).unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    let got = md.build();
    assert!(got.is_ok());

    // 2. Pointing to a normal directory should work
    let temp = DummyBook::new().build().unwrap();
    let created = fs::create_dir(temp.path().join("theme-directory"));
    assert!(created.is_ok());
    let book_toml = r#"
        [book]
        title = "implicit"
        src = "src"

        [output.html]
        theme = "./theme-directory"
        "#;

    write_file(temp.path(), "book.toml", book_toml.as_bytes()).unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    let got = md.build();
    assert!(got.is_ok());

    // 3. Pointing to a non-existent directory should fail
    let temp = DummyBook::new().build().unwrap();
    let book_toml = r#"
        [book]
        title = "implicit"
        src = "src"

        [output.html]
        theme = "./non-existent-directory"
        "#;

    write_file(temp.path(), "book.toml", book_toml.as_bytes()).unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    let got = md.build();
    assert!(got.is_err());
}

#[cfg(feature = "search")]
mod search {
    use crate::dummy_book::DummyBook;
    use mdbook::MDBook;
    use std::fs::{self, File};
    use std::path::Path;

    fn read_book_index(root: &Path) -> serde_json::Value {
        let index = root.join("book/searchindex.js");
        let index = fs::read_to_string(index).unwrap();
        let index = index.trim_start_matches("Object.assign(window.search, ");
        let index = index.trim_end_matches(");");
        serde_json::from_str(index).unwrap()
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn book_creates_reasonable_search_index() {
        let temp = DummyBook::new().build().unwrap();
        let md = MDBook::load(temp.path()).unwrap();
        md.build().unwrap();

        let index = read_book_index(temp.path());

        let doc_urls = index["doc_urls"].as_array().unwrap();
        eprintln!("doc_urls={doc_urls:#?}",);
        let get_doc_ref =
            |url: &str| -> String { doc_urls.iter().position(|s| s == url).unwrap().to_string() };

        let first_chapter = get_doc_ref("first/index.html#first-chapter");
        let introduction = get_doc_ref("intro.html#introduction");
        let some_section = get_doc_ref("first/index.html#some-section");
        let summary = get_doc_ref("first/includes.html#summary");
        let no_headers = get_doc_ref("first/no-headers.html");
        let duplicate_headers_1 = get_doc_ref("first/duplicate-headers.html#header-text-1");
        let conclusion = get_doc_ref("conclusion.html#conclusion");
        let heading_attrs = get_doc_ref("first/heading-attributes.html#both");

        let bodyidx = &index["index"]["index"]["body"]["root"];
        let textidx = &bodyidx["t"]["e"]["x"]["t"];
        assert_eq!(textidx["df"], 5);
        assert_eq!(textidx["docs"][&first_chapter]["tf"], 1.0);
        assert_eq!(textidx["docs"][&introduction]["tf"], 1.0);

        let docs = &index["index"]["documentStore"]["docs"];
        assert_eq!(docs[&first_chapter]["body"], "more text.");
        assert_eq!(docs[&some_section]["body"], "");
        assert_eq!(
            docs[&summary]["body"],
            "Dummy Book Introduction First Chapter Nested Chapter Includes Recursive Markdown Unicode No Headers Duplicate Headers Heading Attributes Second Chapter Nested Chapter Conclusion"
        );
        assert_eq!(
            docs[&summary]["breadcrumbs"],
            "First Chapter » Includes » Summary"
        );
        // See note about InlineHtml in search.rs. Ideally the `alert()` part
        // should not be in the index, but we don't have a way to scrub inline
        // html.
        assert_eq!(docs[&conclusion]["body"], "I put &lt;HTML&gt; in here! Sneaky inline event alert(\"inline\");. But regular inline is indexed.");
        assert_eq!(
            docs[&no_headers]["breadcrumbs"],
            "First Chapter » No Headers"
        );
        assert_eq!(
            docs[&duplicate_headers_1]["breadcrumbs"],
            "First Chapter » Duplicate Headers » Header Text"
        );
        assert_eq!(
            docs[&no_headers]["body"],
            "Capybara capybara capybara. Capybara capybara capybara. ThisLongWordIsIncludedSoWeCanCheckThatSufficientlyLongWordsAreOmittedFromTheSearchIndex."
        );
        assert_eq!(
            docs[&heading_attrs]["breadcrumbs"],
            "First Chapter » Heading Attributes » Heading with id and classes"
        );
    }

    // Setting this to `true` may cause issues with `cargo watch`,
    // since it may not finish writing the fixture before the tests
    // are run again.
    const GENERATE_FIXTURE: bool = false;

    fn get_fixture() -> serde_json::Value {
        if GENERATE_FIXTURE {
            let temp = DummyBook::new().build().unwrap();
            let md = MDBook::load(temp.path()).unwrap();
            md.build().unwrap();

            let src = read_book_index(temp.path());

            let dest = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/searchindex_fixture.json");
            let dest = File::create(dest).unwrap();
            serde_json::to_writer_pretty(dest, &src).unwrap();

            src
        } else {
            let json = include_str!("searchindex_fixture.json");
            serde_json::from_str(json).expect("Unable to deserialize the fixture")
        }
    }

    // So you've broken the test. If you changed dummy_book, it's probably
    // safe to regenerate the fixture. If you haven't then make sure that the
    // search index still works. Run `cargo run -- serve tests/dummy_book`
    // and try some searches. Are you getting results? Do the teasers look OK?
    // Are there new errors in the JS console?
    //
    // If you're pretty sure you haven't broken anything, change `GENERATE_FIXTURE`
    // above to `true`, and run `cargo test` to generate a new fixture. Then
    // **change it back to `false`**. Include the changed `searchindex_fixture.json` in your commit.
    #[test]
    fn search_index_hasnt_changed_accidentally() {
        let temp = DummyBook::new().build().unwrap();
        let md = MDBook::load(temp.path()).unwrap();
        md.build().unwrap();

        let book_index = read_book_index(temp.path());

        let fixture_index = get_fixture();

        // Uncomment this if you're okay with pretty-printing 32KB of JSON
        //assert_eq!(fixture_index, book_index);

        if book_index != fixture_index {
            panic!("The search index has changed from the fixture");
        }
    }
}

#[test]
fn custom_fonts() {
    // Tests to ensure custom fonts are copied as expected.
    let builtin_fonts = [
        "OPEN-SANS-LICENSE.txt",
        "SOURCE-CODE-PRO-LICENSE.txt",
        "fonts.css",
        "open-sans-v17-all-charsets-300.woff2",
        "open-sans-v17-all-charsets-300italic.woff2",
        "open-sans-v17-all-charsets-600.woff2",
        "open-sans-v17-all-charsets-600italic.woff2",
        "open-sans-v17-all-charsets-700.woff2",
        "open-sans-v17-all-charsets-700italic.woff2",
        "open-sans-v17-all-charsets-800.woff2",
        "open-sans-v17-all-charsets-800italic.woff2",
        "open-sans-v17-all-charsets-italic.woff2",
        "open-sans-v17-all-charsets-regular.woff2",
        "source-code-pro-v11-all-charsets-500.woff2",
    ];
    let actual_files = |path: &Path| -> Vec<String> {
        let mut actual: Vec<_> = path
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        actual.sort();
        actual
    };
    let has_fonts_css = |path: &Path| -> bool {
        let contents = fs::read_to_string(path.join("book/index.html")).unwrap();
        contents.contains("fonts/fonts.css")
    };

    // No theme:
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).build().unwrap();
    MDBook::load(p).unwrap().build().unwrap();
    assert_eq!(actual_files(&p.join("book/fonts")), &builtin_fonts);
    assert!(has_fonts_css(p));

    // Full theme.
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).copy_theme(true).build().unwrap();
    assert_eq!(actual_files(&p.join("theme/fonts")), &builtin_fonts);
    MDBook::load(p).unwrap().build().unwrap();
    assert_eq!(actual_files(&p.join("book/fonts")), &builtin_fonts);
    assert!(has_fonts_css(p));

    // Mixed with copy-fonts=true
    // Should ignore the copy-fonts setting since the user has provided their own fonts.css.
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).build().unwrap();
    write_file(&p.join("theme/fonts"), "fonts.css", b"/*custom*/").unwrap();
    write_file(&p.join("theme/fonts"), "myfont.woff", b"").unwrap();
    MDBook::load(p).unwrap().build().unwrap();
    assert!(has_fonts_css(p));
    assert_eq!(
        actual_files(&p.join("book/fonts")),
        ["fonts.css", "myfont.woff"]
    );

    // copy-fonts=false, no theme
    // This should generate a deprecation warning.
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).build().unwrap();
    let config = Config::from_str("output.html.copy-fonts = false").unwrap();
    MDBook::load_with_config(p, config)
        .unwrap()
        .build()
        .unwrap();
    assert!(!has_fonts_css(p));
    assert!(!p.join("book/fonts").exists());

    // copy-fonts=false with empty fonts.css
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).build().unwrap();
    write_file(&p.join("theme/fonts"), "fonts.css", b"").unwrap();
    let config = Config::from_str("output.html.copy-fonts = false").unwrap();
    MDBook::load_with_config(p, config)
        .unwrap()
        .build()
        .unwrap();
    assert!(!has_fonts_css(p));
    assert!(!p.join("book/fonts").exists());

    // copy-fonts=false with fonts theme
    let temp = TempFileBuilder::new().prefix("mdbook").tempdir().unwrap();
    let p = temp.path();
    MDBook::init(p).build().unwrap();
    write_file(&p.join("theme/fonts"), "fonts.css", b"/*custom*/").unwrap();
    write_file(&p.join("theme/fonts"), "myfont.woff", b"").unwrap();
    let config = Config::from_str("output.html.copy-fonts = false").unwrap();
    MDBook::load_with_config(p, config)
        .unwrap()
        .build()
        .unwrap();
    assert!(has_fonts_css(p));
    assert_eq!(
        actual_files(&p.join("book/fonts")),
        &["fonts.css", "myfont.woff"]
    );
}

#[test]
fn custom_header_attributes() {
    let temp = DummyBook::new().build().unwrap();
    let md = MDBook::load(temp.path()).unwrap();
    md.build().unwrap();

    let contents = temp.path().join("book/first/heading-attributes.html");

    let summary_strings = &[
        r##"<h1 id="attrs"><a class="header" href="#attrs">Heading Attributes</a></h1>"##,
        r##"<h2 id="heading-with-classes" class="class1 class2"><a class="header" href="#heading-with-classes">Heading with classes</a></h2>"##,
        r##"<h2 id="both" class="class1 class2"><a class="header" href="#both">Heading with id and classes</a></h2>"##,
    ];
    assert_contains_strings(&contents, summary_strings);
}
