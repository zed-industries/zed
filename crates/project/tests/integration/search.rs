use std::io::{self, BufReader, Cursor};

use language::Buffer;
use project::search::{MatchPositionHint, SearchQuery};
use text::Rope;
use util::{
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

#[test]
fn path_matcher_creation_for_valid_paths() {
    for valid_path in [
        "file",
        "Cargo.toml",
        ".DS_Store",
        "~/dir/another_dir/",
        "./dir/file",
        "dir/[a-z].txt",
    ] {
        let path_matcher = PathMatcher::new(&[valid_path.to_owned()], PathStyle::local())
            .unwrap_or_else(|e| panic!("Valid path {valid_path} should be accepted, but got: {e}"));
        assert!(
            path_matcher.is_match(&RelPath::new(valid_path.as_ref(), PathStyle::local()).unwrap()),
            "Path matcher for valid path {valid_path} should match itself"
        )
    }
}

#[test]
fn path_matcher_creation_for_globs() {
    for invalid_glob in ["dir/[].txt", "dir/[a-z.txt", "dir/{file"] {
        match PathMatcher::new(&[invalid_glob.to_owned()], PathStyle::local()) {
            Ok(_) => panic!("Invalid glob {invalid_glob} should not be accepted"),
            Err(_expected) => {}
        }
    }

    for valid_glob in [
        "dir/?ile",
        "dir/*.txt",
        "dir/**/file",
        "dir/[a-z].txt",
        "{dir,file}",
    ] {
        match PathMatcher::new(&[valid_glob.to_owned()], PathStyle::local()) {
            Ok(_expected) => {}
            Err(e) => panic!("Valid glob should be accepted, but got: {e}"),
        }
    }
}

#[test]
fn test_case_sensitive_pattern_items() {
    let case_sensitive = false;
    let search_query = SearchQuery::regex(
        "test\\C",
        false,
        case_sensitive,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    assert_eq!(
        search_query.case_sensitive(),
        true,
        "Case sensitivity should be enabled when \\C pattern item is present in the query."
    );

    let case_sensitive = true;
    let search_query = SearchQuery::regex(
        "test\\c",
        true,
        case_sensitive,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    assert_eq!(
        search_query.case_sensitive(),
        false,
        "Case sensitivity should be disabled when \\c pattern item is present, even if initially set to true."
    );

    let case_sensitive = false;
    let search_query = SearchQuery::regex(
        "test\\c\\C",
        false,
        case_sensitive,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    assert_eq!(
        search_query.case_sensitive(),
        true,
        "Case sensitivity should be enabled when \\C is the last pattern item, even after a \\c."
    );

    let case_sensitive = false;
    let search_query = SearchQuery::regex(
        "tests\\\\C",
        false,
        case_sensitive,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    assert_eq!(
        search_query.case_sensitive(),
        false,
        "Case sensitivity should not be enabled when \\C pattern item is preceded by a backslash."
    );
}

#[gpui::test]
async fn test_multiline_regex_crlf(cx: &mut gpui::TestAppContext) {
    let search_query = SearchQuery::regex(
        "^hello$\r?\n",
        false,
        false,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    let text = Rope::from("hello\r\nworld\r\nhello\r\nworld");
    let snapshot = cx
        .update(|app| Buffer::build_snapshot(text, None, None, None, app))
        .await;

    let results = search_query.search(&snapshot, None).await;
    assert_eq!(results, vec![0..7, 14..21]);
}

#[gpui::test]
async fn test_multiline_regex(cx: &mut gpui::TestAppContext) {
    let search_query = SearchQuery::regex(
        "^hello$\n",
        false,
        false,
        false,
        false,
        Default::default(),
        Default::default(),
        false,
        None,
    )
    .expect("Should be able to create a regex SearchQuery");

    let text = Rope::from("hello\nworld\nhello\nworld");
    let snapshot = cx
        .update(|app| Buffer::build_snapshot(text, None, None, None, app))
        .await;

    let results = search_query.search(&snapshot, None).await;
    assert_eq!(results, vec![0..6, 12..18]);
}

#[gpui::test]
async fn regex_with_eol_detects_lines() {
    let re = SearchQuery::regex(
        "Bool$",
        false,
        false,
        false,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let input = " Bool\nsomething else";
    let result = re
        .detect(&mut BufReader::new(input.as_bytes()))
        .await
        .unwrap();
    assert!(result.is_some());
}

#[gpui::test]
async fn multi_line_regex_detects_matches() {
    let re = SearchQuery::regex(
        "Bool$\nbool",
        false,
        false,
        false,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let input = " Bool\nbool";
    let result = re
        .detect(&mut BufReader::new(input.as_bytes()))
        .await
        .unwrap();
    assert!(result.is_some());
}

#[test]
fn detect_reports_line_across_block_boundaries() {
    let line_len = 1000;
    let filler = format!("{}\n", "x".repeat(line_len - 1));
    let mut text = filler.repeat(70);
    let needle_line = text.lines().count();
    text.push_str("prefix needle suffix\n");
    text.push_str(&filler.repeat(3));

    for (query, case_sensitive) in [("needle", true), ("NEEDLE", false), ("x\nx", true)] {
        let query = SearchQuery::text(
            query,
            false,
            case_sensitive,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            false,
            None,
        )
        .unwrap();
        let hint = smol::block_on(query.detect(&mut reader(text.as_bytes()))).unwrap();
        let expected = if query.as_str().contains('\n') {
            MatchPositionHint::default()
        } else {
            MatchPositionHint::Line(needle_line as u32)
        };
        assert_eq!(hint, Some(expected), "{:?}", query.as_str());
    }

    let query = SearchQuery::text(
        "absent",
        false,
        true,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    assert_eq!(
        smol::block_on(query.detect(&mut reader(text.as_bytes()))).unwrap(),
        None
    );

    let mut invalid = text.into_bytes();
    invalid.extend_from_slice(b"\xff\xfe tail\n");
    let error = smol::block_on(query.detect(&mut reader(&invalid))).unwrap_err();
    assert_eq!(
        error.downcast_ref::<io::Error>().map(|error| error.kind()),
        Some(io::ErrorKind::InvalidData)
    );
}

#[test]
fn detect_handles_block_boundary_splits() {
    const BLOCK_BYTES: usize = 64 * 1024;
    let needle = "néédle";
    for split in 1..needle.len() {
        let mut text = "a".repeat(BLOCK_BYTES - split);
        text.push_str(needle);
        text.push_str("\ntail\n");
        for case_sensitive in [true, false] {
            let query = SearchQuery::text(
                needle,
                false,
                case_sensitive,
                false,
                PathMatcher::default(),
                PathMatcher::default(),
                false,
                None,
            )
            .unwrap();
            let expected = if query.is_regex() {
                MatchPositionHint::ByteOffset(BLOCK_BYTES - split)
            } else {
                MatchPositionHint::Line(0)
            };
            assert_eq!(
                smol::block_on(query.detect(&mut reader(text.as_bytes()))).unwrap(),
                Some(expected),
                "split = {split}, case_sensitive = {case_sensitive}"
            );
        }
    }

    let query = SearchQuery::text(
        "absent",
        false,
        true,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let emoji = "\u{1f600}";
    for split in 1..emoji.len() {
        let mut text = "a".repeat(BLOCK_BYTES - split).into_bytes();
        text.extend_from_slice(emoji.as_bytes());
        text.extend_from_slice(b"\nline\n");
        assert_eq!(
            smol::block_on(query.detect(&mut reader(&text))).unwrap(),
            None,
            "split = {split}"
        );
        text.truncate(BLOCK_BYTES - split + 2);
        let error = smol::block_on(query.detect(&mut reader(&text))).unwrap_err();
        assert_eq!(
            error.downcast_ref::<io::Error>().map(|error| error.kind()),
            Some(io::ErrorKind::InvalidData),
            "truncated split = {split}"
        );
    }
}

fn reader(bytes: &[u8]) -> Cursor<Vec<u8>> {
    Cursor::new(bytes.to_vec())
}
