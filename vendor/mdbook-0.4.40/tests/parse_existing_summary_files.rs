//! Some integration tests to make sure the `SUMMARY.md` parser can deal with
//! some real-life examples.

use mdbook::book;
use std::fs::File;
use std::io::Read;
use std::path::Path;

macro_rules! summary_md_test {
    ($name:ident, $filename:expr) => {
        #[test]
        fn $name() {
            env_logger::try_init().ok();

            let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("summary_md_files")
                .join($filename);

            if !filename.exists() {
                panic!("{} Doesn't exist", filename.display());
            }

            let mut content = String::new();
            File::open(&filename)
                .unwrap()
                .read_to_string(&mut content)
                .unwrap();

            if let Err(e) = book::parse_summary(&content) {
                eprintln!("Error parsing {}", filename.display());
                eprintln!();
                eprintln!("{:?}", e);
                panic!();
            }
        }
    };
}

summary_md_test!(rust_by_example, "rust_by_example.md");
summary_md_test!(rust_ffi_guide, "rust_ffi_guide.md");
summary_md_test!(example_book, "example_book.md");
summary_md_test!(the_book_2nd_edition, "the_book-2nd_edition.md");
