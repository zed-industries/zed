#![no_main]
use libfuzzer_sys::fuzz_target;
use std::path::PathBuf;

extern crate util;

fuzz_target!(|data: Vec<(PathBuf, bool)>| {
    // Convert each tuple of (bytes, is_file) into (PathBuf, bool)
    // Attempt to sort the vector of path pairs
    let mut sorted_pairs = data.clone();
    sorted_pairs.sort_by(|(a_path, a_file), (b_path, b_file)| {
        util::paths::compare_paths((a_path, *a_file), (b_path, *b_file))
    });

    // The fuzzer will monitor for any panic during sorting, indicating a weak total ordering violation.
});
