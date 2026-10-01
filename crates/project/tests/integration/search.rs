use std::io::{self, BufReader, Cursor};

use collections::HashMap;
use fs::FakeFs;
use futures::FutureExt as _;
use language::Buffer;
use project::{
    Project,
    search::{MatchPositionHint, SearchQuery},
};
use serde_json::json;
use text::Rope;
use util::{
    path,
    paths::{PathMatcher, PathStyle},
    rel_path::RelPath,
};

use crate::init_test;

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
        let mut input = reader(text.as_bytes());
        assert!(query.detect(&mut input).now_or_never().is_none());
        assert_eq!(input.inner.position(), 64 * 1024);
        let hint = smol::block_on(query.detect(&mut reader(text.as_bytes()))).unwrap();
        let expected = if query.as_str().contains('\n') {
            MatchPositionHint::default()
        } else {
            MatchPositionHint::Line(needle_line as u32)
        };
        assert_eq!(hint, Some(expected), "{:?}", query.as_str());

        if !query.as_str().contains('\n') {
            let mut invalid = format!("needle{}", "x".repeat(128 * 1024)).into_bytes();
            invalid.push(0xff);
            let error = smol::block_on(query.detect(&mut reader(&invalid))).unwrap_err();
            assert_eq!(
                error.downcast_ref::<io::Error>().map(|error| error.kind()),
                Some(io::ErrorKind::InvalidData)
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
            let mut input = reader(text.as_bytes());
            assert!(query.detect(&mut input).now_or_never().is_none());
            assert_eq!(input.inner.position(), BLOCK_BYTES as u64);
            assert_eq!(
                smol::block_on(query.detect(&mut reader(text.as_bytes()))).unwrap(),
                Some(expected),
                "split = {split}, case_sensitive = {case_sensitive}"
            );
        }
    }

    let needle = (0..9000)
        .map(|index| format!("{index:08x}"))
        .collect::<String>();
    let query = SearchQuery::text(
        &needle,
        false,
        true,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let text = format!("ignored\n{}x\n{needle}", &needle[..BLOCK_BYTES - 1]);
    for max_read in [257, BLOCK_BYTES] {
        let mut input = reader(text.as_bytes());
        input.max_read = max_read;
        assert!(query.detect(&mut input).now_or_never().is_none());
        assert_eq!(input.inner.position(), max_read as u64);
        let mut input = reader(text.as_bytes());
        input.max_read = max_read;
        assert_eq!(
            smol::block_on(query.detect(&mut input)).unwrap(),
            Some(MatchPositionHint::Line(2)),
        );
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
        let mut matched_text = text.clone();
        matched_text[..query.as_str().len()].copy_from_slice(query.as_str().as_bytes());
        matched_text.push(0xff);
        assert_eq!(
            smol::block_on(query.detect(&mut reader(&matched_text))).unwrap(),
            Some(MatchPositionHint::Line(0)),
            "matched split = {split}"
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

#[test]
fn detect_validates_matching_line_before_io_errors() {
    let query = SearchQuery::text(
        "n",
        false,
        true,
        false,
        PathMatcher::default(),
        PathMatcher::default(),
        false,
        None,
    )
    .unwrap();
    let found = Ok(Some(MatchPositionHint::Line(0)));
    let invalid = Err(io::ErrorKind::InvalidData);
    let failed = Err(io::ErrorKind::Other);
    for (bytes, at_eof, at_error) in [
        (b"".as_slice(), Ok(None), failed),
        (b"n", found, failed),
        (b"n\xff", invalid, failed),
        (b"\xff\nn", invalid, invalid),
        (b"n\xff\n", invalid, invalid),
        (b"n\n\xff", found, found),
        (b"n\xf0\x9f", invalid, failed),
        (b"n\n\xf0\x9f", found, found),
        (
            "x\n😀n".as_bytes(),
            Ok(Some(MatchPositionHint::Line(1))),
            failed,
        ),
    ] {
        for terminal_error in [None, Some(io::ErrorKind::Other)] {
            for max_read in [1, 64 * 1024] {
                let mut input = reader(bytes);
                input.max_read = max_read;
                input.terminal_error = terminal_error;
                let actual = smol::block_on(query.detect(&mut input))
                    .map_err(|error| error.downcast_ref::<io::Error>().unwrap().kind());
                let expected = if terminal_error.is_some() {
                    at_error
                } else {
                    at_eof
                };
                assert_eq!(
                    actual, expected,
                    "{bytes:?}, {terminal_error:?}, {max_read}"
                );
            }
        }
    }
}

#[gpui::test]
async fn searches_legacy_text_after_utf8_prefixes(cx: &mut gpui::TestAppContext) {
    init_test(cx);

    for (text, encoding, needle) in [
        (
            format!("Ã©\n{} où voilà un garçon à la maison ", "x".repeat(1021)),
            encoding_rs::WINDOWS_1252,
            "é",
        ),
        (
            format!(
                "Ã©\nÂ©{}{}",
                "x".repeat(70000),
                " déjà été à côté français ".repeat(1000)
            ),
            encoding_rs::WINDOWS_1252,
            "©",
        ),
        (
            format!(
                "\x1b[0m{}{}©",
                "x".repeat(1024 * 1024),
                " déjà été à côté français ".repeat(1000)
            ),
            encoding_rs::WINDOWS_1252,
            "©",
        ),
        (
            format!(
                "я{}{}ЖЕТОН",
                "x".repeat(1024 * 1024),
                "Съешь же ещё этих мягких французских булок, да выпей чаю.\n".repeat(1000)
            ),
            encoding_rs::WINDOWS_1251,
            "ЖЕТОН",
        ),
    ] {
        let (bytes, _, had_errors) = encoding.encode(&text);
        assert!(!had_errors);
        let expected = text
            .match_indices(needle)
            .map(|(offset, matched)| offset..offset + matched.len())
            .collect::<Vec<_>>();
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/dir"), json!({})).await;
        fs.insert_file(path!("/dir/legacy.txt"), bytes.into_owned())
            .await;
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let query = SearchQuery::text(
            needle,
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
            super::search(&project, query, cx).await.unwrap(),
            HashMap::from_iter(
                (!expected.is_empty()).then(|| (path!("dir/legacy.txt").to_string(), expected))
            ),
        );
    }
}

fn reader(bytes: &[u8]) -> InterruptingReader {
    InterruptingReader {
        inner: Cursor::new(bytes.to_vec()),
        interrupt_next_read: false,
        max_read: usize::MAX,
        terminal_error: None,
    }
}

struct InterruptingReader {
    inner: Cursor<Vec<u8>>,
    interrupt_next_read: bool,
    max_read: usize,
    terminal_error: Option<io::ErrorKind>,
}

impl io::Read for InterruptingReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.interrupt_next_read = !self.interrupt_next_read;
        if self.interrupt_next_read {
            Err(io::Error::from(io::ErrorKind::Interrupted))
        } else {
            let length = buffer.len().min(self.max_read);
            let read = self.inner.read(&mut buffer[..length])?;
            if read == 0
                && !buffer.is_empty()
                && let Some(error) = self.terminal_error
            {
                return Err(io::Error::from(error));
            }
            Ok(read)
        }
    }
}
