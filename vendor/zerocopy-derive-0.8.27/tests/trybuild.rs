// Copyright 2019 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

#![allow(clippy::uninlined_format_args)]

use std::env;

use testutil::set_rustflags_w_warnings;

fn test(subdir: &str) {
    let version = testutil::ToolchainVersion::extract_from_pwd().unwrap();
    // See the doc comment on this method for an explanation of what this does
    // and why we store source files in different directories.
    let source_files_dirname = version.get_ui_source_files_dirname_and_maybe_print_warning();

    // Set `-Wwarnings` in the `RUSTFLAGS` environment variable to ensure that
    // `.stderr` files reflect what the typical user would encounter.
    set_rustflags_w_warnings();

    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/{}/{}/*.rs", source_files_dirname, subdir));
}

#[test]
#[cfg_attr(miri, ignore)]
fn ui() {
    test("");

    // This tests the behavior when `--cfg zerocopy_derive_union_into_bytes` is
    // not present, so remove it. If this logic is wrong, that's fine - it will
    // exhibit as a test failure that we can debug at that point.
    let rustflags = env::var("RUSTFLAGS").unwrap();
    let new_rustflags = rustflags.replace("--cfg zerocopy_derive_union_into_bytes", "");

    // SAFETY: None of our code is concurrently accessinv env vars. It's
    // possible that the test framework has spawned other threads that are
    // concurrently accessing env vars, but we can't do anything about that.
    #[allow(unused_unsafe)] // `set_var` is safe on our MSRV.
    unsafe {
        env::set_var("RUSTFLAGS", new_rustflags)
    };

    test("union_into_bytes_cfg");

    // Reset RUSTFLAGS in case we later add other tests which rely on its value.
    // This isn't strictly necessary, but it's easier to add this now when we're
    // thinking about the semantics of these env vars than to debug later when
    // we've forgotten about it.
    //
    // SAFETY: See previous safety comment.
    #[allow(unused_unsafe)] // `set_var` is safe on our MSRV.
    unsafe {
        env::set_var("RUSTFLAGS", rustflags)
    };
}
