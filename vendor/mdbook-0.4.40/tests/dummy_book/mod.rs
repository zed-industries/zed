//! This will create an entire book in a temporary directory using some
//! dummy contents from the `tests/dummy-book/` directory.

// Not all features are used in all test crates, so...
#![allow(dead_code, unused_variables, unused_imports, unused_extern_crates)]

use anyhow::Context;
use mdbook::errors::*;
use mdbook::MDBook;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use tempfile::{Builder as TempFileBuilder, TempDir};
use walkdir::WalkDir;

/// Create a dummy book in a temporary directory, using the contents of
/// `SUMMARY_MD` as a guide.
///
/// The "Nested Chapter" file contains a code block with a single
/// `assert!($TEST_STATUS)`. If you want to check MDBook's testing
/// functionality, `$TEST_STATUS` can be substitute for either `true` or
/// `false`. This is done using the `passing_test` parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct DummyBook {
    passing_test: bool,
}

impl DummyBook {
    /// Create a new `DummyBook` with all the defaults.
    pub fn new() -> DummyBook {
        DummyBook { passing_test: true }
    }

    /// Whether the doc-test included in the "Nested Chapter" should pass or
    /// fail (it passes by default).
    pub fn with_passing_test(&mut self, test_passes: bool) -> &mut DummyBook {
        self.passing_test = test_passes;
        self
    }

    /// Write a book to a temporary directory using the provided settings.
    pub fn build(&self) -> Result<TempDir> {
        let temp = TempFileBuilder::new()
            .prefix("dummy_book-")
            .tempdir()
            .with_context(|| "Unable to create temp directory")?;

        let dummy_book_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/dummy_book");
        recursive_copy(&dummy_book_root, temp.path()).with_context(|| {
            "Couldn't copy files into a \
             temporary directory"
        })?;

        let sub_pattern = if self.passing_test { "true" } else { "false" };
        let files_containing_tests = [
            "src/first/nested.md",
            "src/first/nested-test.rs",
            "src/first/nested-test-with-anchors.rs",
            "src/first/partially-included-test.rs",
            "src/first/partially-included-test-with-anchors.rs",
        ];
        for file in &files_containing_tests {
            let path_containing_tests = temp.path().join(file);
            replace_pattern_in_file(&path_containing_tests, "$TEST_STATUS", sub_pattern)?;
        }

        Ok(temp)
    }
}

fn replace_pattern_in_file(filename: &Path, from: &str, to: &str) -> Result<()> {
    let contents = fs::read_to_string(filename)?;
    File::create(filename)?.write_all(contents.replace(from, to).as_bytes())?;

    Ok(())
}

/// Read the contents of the provided file into memory and then iterate through
/// the list of strings asserting that the file contains all of them.
pub fn assert_contains_strings<P: AsRef<Path>>(filename: P, strings: &[&str]) {
    let filename = filename.as_ref();
    let content = fs::read_to_string(filename).expect("Couldn't read the file's contents");

    for s in strings {
        assert!(
            content.contains(s),
            "Searching for {:?} in {}\n\n{}",
            s,
            filename.display(),
            content
        );
    }
}

pub fn assert_doesnt_contain_strings<P: AsRef<Path>>(filename: P, strings: &[&str]) {
    let filename = filename.as_ref();
    let content = fs::read_to_string(filename).expect("Couldn't read the file's contents");

    for s in strings {
        assert!(
            !content.contains(s),
            "Found {:?} in {}\n\n{}",
            s,
            filename.display(),
            content
        );
    }
}

/// Recursively copy an entire directory tree to somewhere else (a la `cp -r`).
fn recursive_copy<A: AsRef<Path>, B: AsRef<Path>>(from: A, to: B) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    for entry in WalkDir::new(from) {
        let entry = entry.with_context(|| "Unable to inspect directory entry")?;

        let original_location = entry.path();
        let relative = original_location
            .strip_prefix(from)
            .expect("`original_location` is inside the `from` directory");
        let new_location = to.join(relative);

        if original_location.is_file() {
            if let Some(parent) = new_location.parent() {
                fs::create_dir_all(parent).with_context(|| "Couldn't create directory")?;
            }

            fs::copy(original_location, &new_location)
                .with_context(|| "Unable to copy file contents")?;
        }
    }

    Ok(())
}

pub fn new_copy_of_example_book() -> Result<TempDir> {
    let temp = TempFileBuilder::new().prefix("guide").tempdir()?;

    let guide = Path::new(env!("CARGO_MANIFEST_DIR")).join("guide");

    recursive_copy(guide, temp.path())?;

    Ok(temp)
}
