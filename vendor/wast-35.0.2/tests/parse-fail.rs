//! A test suite to parse everything in `parse-fail` and assert that it matches
//! the `*.err` file it generates.
//!
//! Use `BLESS=1` in the environment to auto-update `*.err` files. Be sure to
//! look at the diff!

use rayon::prelude::*;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let mut tests = Vec::new();
    find_tests("tests/parse-fail".as_ref(), &mut tests);
    let filter = std::env::args().nth(1);

    let bless = env::var("BLESS").is_ok();
    let tests = tests
        .iter()
        .filter(|test| {
            if let Some(filter) = &filter {
                if let Some(s) = test.file_name().and_then(|s| s.to_str()) {
                    if !s.contains(filter) {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();

    println!("running {} tests\n", tests.len());

    let errors = tests
        .par_iter()
        .filter_map(|test| run_test(test, bless).err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for msg in errors.iter() {
            eprintln!("{}", msg);
        }

        panic!("{} tests failed", errors.len())
    }

    println!("test result: ok. {} passed\n", tests.len());
}

fn run_test(test: &Path, bless: bool) -> anyhow::Result<()> {
    let err = match wat::parse_file(test) {
        Ok(_) => anyhow::bail!("{} parsed successfully", test.display()),
        Err(e) => e.to_string() + "\n",
    };
    let assert = test.with_extension("wat.err");
    if bless {
        std::fs::write(assert, err.to_string())?;
        return Ok(());
    }

    // Ignore CRLF line ending and force always `\n`
    let assert = std::fs::read_to_string(assert)
        .unwrap_or(String::new())
        .replace("\r\n", "\n");

    // Compare normalize verisons which handles weirdness like path differences
    if normalize(&assert) == normalize(&err) {
        return Ok(());
    }

    anyhow::bail!(
        "errors did not match:\n\nexpected:\n\t{}\nactual:\n\t{}\n",
        tab(&assert),
        tab(&err),
    );

    fn normalize(s: &str) -> String {
        s.replace("\\", "/")
    }

    fn tab(s: &str) -> String {
        s.replace("\n", "\n\t")
    }
}

fn find_tests(path: &Path, tests: &mut Vec<PathBuf>) {
    for f in path.read_dir().unwrap() {
        let f = f.unwrap();
        if f.file_type().unwrap().is_dir() {
            find_tests(&f.path(), tests);
            continue;
        }
        match f.path().extension().and_then(|s| s.to_str()) {
            Some("wat") => {}
            _ => continue,
        }
        tests.push(f.path());
    }
}
