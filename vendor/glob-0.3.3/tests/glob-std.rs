// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-windows TempDir may cause IoError on windows: #10462

#![cfg_attr(test, deny(warnings))]

extern crate glob;
extern crate tempdir;

use glob::{glob, glob_with};
use std::env;
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;

#[test]
fn main() {
    fn mk_file(path: &str, directory: bool) {
        if directory {
            fs::create_dir(path).unwrap();
        } else {
            fs::File::create(path).unwrap();
        }
    }

    fn mk_symlink_file(original: &str, link: &str) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            symlink(original, link).unwrap();
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::symlink_file;
            symlink_file(original, link).unwrap();
        }
    }

    fn mk_symlink_dir(original: &str, link: &str) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            symlink(original, link).unwrap();
        }
        #[cfg(windows)]
        {
            use std::os::windows::fs::symlink_dir;
            symlink_dir(original, link).unwrap();
        }
    }

    fn glob_vec(pattern: &str) -> Vec<PathBuf> {
        glob(pattern).unwrap().map(|r| r.unwrap()).collect()
    }

    fn glob_with_vec(pattern: &str, options: glob::MatchOptions) -> Vec<PathBuf> {
        glob_with(pattern, options)
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    }

    let root = TempDir::new("glob-tests");
    let root = root.ok().expect("Should have created a temp directory");
    assert!(env::set_current_dir(root.path()).is_ok());

    mk_file("aaa", true);
    mk_file("aaa/apple", true);
    mk_file("aaa/orange", true);
    mk_file("aaa/tomato", true);
    mk_file("aaa/tomato/tomato.txt", false);
    mk_file("aaa/tomato/tomoto.txt", false);
    mk_file("bbb", true);
    mk_file("bbb/specials", true);
    mk_file("bbb/specials/!", false);
    // a valid symlink
    mk_symlink_file("aaa/apple", "aaa/green_apple");
    // a broken symlink
    mk_symlink_file("aaa/setsuna", "aaa/kazusa");

    // windows does not allow `*` or `?` characters to exist in filenames
    if env::consts::FAMILY != "windows" {
        mk_file("bbb/specials/*", false);
        mk_file("bbb/specials/?", false);
    }

    mk_file("bbb/specials/[", false);
    mk_file("bbb/specials/]", false);
    mk_file("ccc", true);
    mk_file("xyz", true);
    mk_file("xyz/x", false);
    mk_file("xyz/y", false);
    mk_file("xyz/z", false);

    mk_file("r", true);
    mk_file("r/current_dir.md", false);
    mk_file("r/one", true);
    mk_file("r/one/a.md", false);
    mk_file("r/one/another", true);
    mk_file("r/one/another/a.md", false);
    mk_file("r/one/another/deep", true);
    mk_file("r/one/another/deep/spelunking.md", false);
    mk_file("r/another", true);
    mk_file("r/another/a.md", false);
    mk_file("r/two", true);
    mk_file("r/two/b.md", false);
    mk_file("r/three", true);
    mk_file("r/three/c.md", false);

    mk_file("dirsym", true);
    mk_symlink_dir(root.path().join("r").to_str().unwrap(), "dirsym/link");

    assert_eq!(
        glob_vec("dirsym/**/*.md"),
        vec!(
            PathBuf::from("dirsym/link/another/a.md"),
            PathBuf::from("dirsym/link/current_dir.md"),
            PathBuf::from("dirsym/link/one/a.md"),
            PathBuf::from("dirsym/link/one/another/a.md"),
            PathBuf::from("dirsym/link/one/another/deep/spelunking.md"),
            PathBuf::from("dirsym/link/three/c.md"),
            PathBuf::from("dirsym/link/two/b.md")
        )
    );

    // all recursive entities
    assert_eq!(
        glob_vec("r/**"),
        vec!(
            PathBuf::from("r/another"),
            PathBuf::from("r/one"),
            PathBuf::from("r/one/another"),
            PathBuf::from("r/one/another/deep"),
            PathBuf::from("r/three"),
            PathBuf::from("r/two")
        )
    );

    // std-canonicalized windows verbatim disk paths should work
    if env::consts::FAMILY == "windows" {
        let r_verbatim = PathBuf::from("r").canonicalize().unwrap();
        assert_eq!(
            glob_vec(&format!("{}\\**", r_verbatim.display().to_string()))
                .into_iter()
                .map(|p| p.strip_prefix(&r_verbatim).unwrap().to_owned())
                .collect::<Vec<_>>(),
            vec!(
                PathBuf::from("another"),
                PathBuf::from("one"),
                PathBuf::from("one\\another"),
                PathBuf::from("one\\another\\deep"),
                PathBuf::from("three"),
                PathBuf::from("two")
            )
        );
    }

    // collapse consecutive recursive patterns
    assert_eq!(
        glob_vec("r/**/**"),
        vec!(
            PathBuf::from("r/another"),
            PathBuf::from("r/one"),
            PathBuf::from("r/one/another"),
            PathBuf::from("r/one/another/deep"),
            PathBuf::from("r/three"),
            PathBuf::from("r/two")
        )
    );

    assert_eq!(
        glob_vec("r/**/*"),
        vec!(
            PathBuf::from("r/another"),
            PathBuf::from("r/another/a.md"),
            PathBuf::from("r/current_dir.md"),
            PathBuf::from("r/one"),
            PathBuf::from("r/one/a.md"),
            PathBuf::from("r/one/another"),
            PathBuf::from("r/one/another/a.md"),
            PathBuf::from("r/one/another/deep"),
            PathBuf::from("r/one/another/deep/spelunking.md"),
            PathBuf::from("r/three"),
            PathBuf::from("r/three/c.md"),
            PathBuf::from("r/two"),
            PathBuf::from("r/two/b.md")
        )
    );

    // followed by a wildcard
    assert_eq!(
        glob_vec("r/**/*.md"),
        vec!(
            PathBuf::from("r/another/a.md"),
            PathBuf::from("r/current_dir.md"),
            PathBuf::from("r/one/a.md"),
            PathBuf::from("r/one/another/a.md"),
            PathBuf::from("r/one/another/deep/spelunking.md"),
            PathBuf::from("r/three/c.md"),
            PathBuf::from("r/two/b.md")
        )
    );

    // followed by a precise pattern
    assert_eq!(
        glob_vec("r/one/**/a.md"),
        vec!(
            PathBuf::from("r/one/a.md"),
            PathBuf::from("r/one/another/a.md")
        )
    );

    // followed by another recursive pattern
    // collapses consecutive recursives into one
    assert_eq!(
        glob_vec("r/one/**/**/a.md"),
        vec!(
            PathBuf::from("r/one/a.md"),
            PathBuf::from("r/one/another/a.md")
        )
    );

    // followed by two precise patterns
    assert_eq!(
        glob_vec("r/**/another/a.md"),
        vec!(
            PathBuf::from("r/another/a.md"),
            PathBuf::from("r/one/another/a.md")
        )
    );

    assert_eq!(glob_vec(""), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("."), vec!(PathBuf::from(".")));
    assert_eq!(glob_vec(".."), vec!(PathBuf::from("..")));

    assert_eq!(glob_vec("aaa"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aaa/"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("a"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aa"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aaaa"), Vec::<PathBuf>::new());

    assert_eq!(glob_vec("aaa/apple"), vec!(PathBuf::from("aaa/apple")));
    assert_eq!(glob_vec("aaa/apple/nope"), Vec::<PathBuf>::new());

    // windows should support both / and \ as directory separators
    if env::consts::FAMILY == "windows" {
        assert_eq!(glob_vec("aaa\\apple"), vec!(PathBuf::from("aaa/apple")));
    }

    assert_eq!(
        glob_vec("???/"),
        vec!(
            PathBuf::from("aaa"),
            PathBuf::from("bbb"),
            PathBuf::from("ccc"),
            PathBuf::from("xyz")
        )
    );

    assert_eq!(
        glob_vec("aaa/tomato/tom?to.txt"),
        vec!(
            PathBuf::from("aaa/tomato/tomato.txt"),
            PathBuf::from("aaa/tomato/tomoto.txt")
        )
    );

    assert_eq!(
        glob_vec("xyz/?"),
        vec!(
            PathBuf::from("xyz/x"),
            PathBuf::from("xyz/y"),
            PathBuf::from("xyz/z")
        )
    );

    assert_eq!(glob_vec("a*"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("*a*"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("a*a"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aaa*"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("*aaa"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("*aaa*"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("*a*a*a*"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aaa*/"), vec!(PathBuf::from("aaa")));

    assert_eq!(
        glob_vec("aaa/*"),
        vec!(
            PathBuf::from("aaa/apple"),
            PathBuf::from("aaa/green_apple"),
            PathBuf::from("aaa/kazusa"),
            PathBuf::from("aaa/orange"),
            PathBuf::from("aaa/tomato"),
        )
    );

    assert_eq!(
        glob_vec("aaa/*a*"),
        vec!(
            PathBuf::from("aaa/apple"),
            PathBuf::from("aaa/green_apple"),
            PathBuf::from("aaa/kazusa"),
            PathBuf::from("aaa/orange"),
            PathBuf::from("aaa/tomato")
        )
    );

    assert_eq!(
        glob_vec("*/*/*.txt"),
        vec!(
            PathBuf::from("aaa/tomato/tomato.txt"),
            PathBuf::from("aaa/tomato/tomoto.txt")
        )
    );

    assert_eq!(
        glob_vec("*/*/t[aob]m?to[.]t[!y]t"),
        vec!(
            PathBuf::from("aaa/tomato/tomato.txt"),
            PathBuf::from("aaa/tomato/tomoto.txt")
        )
    );

    assert_eq!(glob_vec("./aaa"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("./*"), glob_vec("*"));
    assert_eq!(glob_vec("*/..").pop().unwrap(), PathBuf::from("xyz/.."));
    assert_eq!(glob_vec("aaa/../bbb"), vec!(PathBuf::from("aaa/../bbb")));
    assert_eq!(glob_vec("nonexistent/../bbb"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aaa/tomato/tomato.txt/.."), Vec::<PathBuf>::new());

    assert_eq!(glob_vec("aaa/tomato/tomato.txt/"), Vec::<PathBuf>::new());

    // Ensure to find a broken symlink.
    assert_eq!(glob_vec("aaa/kazusa"), vec!(PathBuf::from("aaa/kazusa")));

    assert_eq!(glob_vec("aa[a]"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aa[abc]"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("a[bca]a"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aa[b]"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aa[xyz]"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aa[]]"), Vec::<PathBuf>::new());

    assert_eq!(glob_vec("aa[!b]"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aa[!bcd]"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("a[!bcd]a"), vec!(PathBuf::from("aaa")));
    assert_eq!(glob_vec("aa[!a]"), Vec::<PathBuf>::new());
    assert_eq!(glob_vec("aa[!abc]"), Vec::<PathBuf>::new());

    assert_eq!(
        glob_vec("bbb/specials/[[]"),
        vec!(PathBuf::from("bbb/specials/["))
    );
    assert_eq!(
        glob_vec("bbb/specials/!"),
        vec!(PathBuf::from("bbb/specials/!"))
    );
    assert_eq!(
        glob_vec("bbb/specials/[]]"),
        vec!(PathBuf::from("bbb/specials/]"))
    );

    mk_file("i", true);
    mk_file("i/qwe", true);
    mk_file("i/qwe/.aaa", false);
    mk_file("i/qwe/.bbb", true);
    mk_file("i/qwe/.bbb/ccc", false);
    mk_file("i/qwe/.bbb/.ddd", false);
    mk_file("i/qwe/eee", false);

    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: true,
        require_literal_leading_dot: true,
    };
    assert_eq!(glob_with_vec("i/**/*a*", options), Vec::<PathBuf>::new());
    assert_eq!(glob_with_vec("i/**/*c*", options), Vec::<PathBuf>::new());
    assert_eq!(glob_with_vec("i/**/*d*", options), Vec::<PathBuf>::new());
    assert_eq!(
        glob_with_vec("i/**/*e*", options),
        vec!(PathBuf::from("i/qwe"), PathBuf::from("i/qwe/eee"))
    );

    if env::consts::FAMILY != "windows" {
        assert_eq!(
            glob_vec("bbb/specials/[*]"),
            vec!(PathBuf::from("bbb/specials/*"))
        );
        assert_eq!(
            glob_vec("bbb/specials/[?]"),
            vec!(PathBuf::from("bbb/specials/?"))
        );
    }

    if env::consts::FAMILY == "windows" {
        assert_eq!(
            glob_vec("bbb/specials/[![]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/]")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!]]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/[")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!!]"),
            vec!(
                PathBuf::from("bbb/specials/["),
                PathBuf::from("bbb/specials/]")
            )
        );
    } else {
        assert_eq!(
            glob_vec("bbb/specials/[![]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/*"),
                PathBuf::from("bbb/specials/?"),
                PathBuf::from("bbb/specials/]")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!]]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/*"),
                PathBuf::from("bbb/specials/?"),
                PathBuf::from("bbb/specials/[")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!!]"),
            vec!(
                PathBuf::from("bbb/specials/*"),
                PathBuf::from("bbb/specials/?"),
                PathBuf::from("bbb/specials/["),
                PathBuf::from("bbb/specials/]")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!*]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/?"),
                PathBuf::from("bbb/specials/["),
                PathBuf::from("bbb/specials/]")
            )
        );

        assert_eq!(
            glob_vec("bbb/specials/[!?]"),
            vec!(
                PathBuf::from("bbb/specials/!"),
                PathBuf::from("bbb/specials/*"),
                PathBuf::from("bbb/specials/["),
                PathBuf::from("bbb/specials/]")
            )
        );
    }
}
