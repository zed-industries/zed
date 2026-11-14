use anyhow::{Context as _, Result, ensure};
use diffy::{Line, Patch};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PatchFile {
    pub(super) old_path: Option<String>,
    pub(super) new_path: Option<String>,
    pub(super) hunks: Vec<PatchHunk>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PatchHunk {
    pub(super) old_start: u32,
    pub(super) old_count: u32,
    pub(super) new_start: u32,
    pub(super) new_count: u32,
    pub(super) old_text: String,
    pub(super) new_text: String,
}

pub(super) fn parse_patch(text: &str) -> Result<Vec<PatchFile>> {
    ensure!(
        text.starts_with("diff --git ") || text.starts_with("--- ") || text.starts_with("@@ "),
        "expected a Git or unified patch without a commit envelope"
    );
    let boundaries = std::iter::once(0)
        .chain(
            text.match_indices("\ndiff --git ")
                .map(|(offset, _)| offset + 1),
        )
        .chain(std::iter::once(text.len()))
        .collect::<Vec<_>>();
    let mut files = Vec::new();
    for section in boundaries.windows(2) {
        let section = &text[section[0]..section[1]];
        check_coordinate_bounds(section)?;
        let section = normalize_quoted_paths(section)?;
        let patch = Patch::from_str(&section).context("invalid Git patch")?;
        if patch.hunks().is_empty() {
            // File operations, including binary and rename-only changes, come from ACP's
            // structured changes. They do not need to be inferred from Git's preamble.
            continue;
        }
        let (old_path, new_path) = if patch.original().is_none() && patch.modified().is_none() {
            (None, None)
        } else {
            let old_path = patch_path(patch.original(), "a/")?;
            let new_path = patch_path(patch.modified(), "b/")?;
            ensure!(
                old_path.is_some() || new_path.is_some(),
                "both patch paths are /dev/null"
            );
            (old_path, new_path)
        };
        let hunks = patch
            .hunks()
            .iter()
            .map(|hunk| {
                let mut old_text = String::new();
                let mut new_text = String::new();
                for line in hunk.lines() {
                    match line {
                        Line::Context(text) => {
                            old_text.push_str(text);
                            new_text.push_str(text);
                        }
                        Line::Delete(text) => old_text.push_str(text),
                        Line::Insert(text) => new_text.push_str(text),
                    }
                }
                Ok(PatchHunk {
                    old_start: u32::try_from(hunk.old_range().start())?,
                    old_count: u32::try_from(hunk.old_range().len())?,
                    new_start: u32::try_from(hunk.new_range().start())?,
                    new_count: u32::try_from(hunk.new_range().len())?,
                    old_text,
                    new_text,
                })
            })
            .collect::<Result<_>>()?;
        files.push(PatchFile {
            old_path,
            new_path,
            hunks,
        });
    }
    Ok(files)
}

fn normalize_quoted_paths(section: &str) -> Result<Cow<'_, str>> {
    if section.starts_with("@@ ") {
        return Ok(Cow::Borrowed(section));
    }
    let header_end = section
        .find("\n@@ ")
        .map_or(section.len(), |offset| offset + 1);
    let (headers, hunks) = section.split_at(header_end);
    if !headers.contains("--- \"") && !headers.contains("+++ \"") {
        return Ok(Cow::Borrowed(section));
    }

    // diffy understands escaped quotes, backslashes and whitespace, but not Git's
    // octal byte escapes or \a, \b, \f and \v. Normalize only filename headers.
    let mut normalized = Vec::with_capacity(section.len());
    for line in headers.split_inclusive('\n') {
        let Some((prefix, filename)) = line.split_once(' ').filter(|(prefix, filename)| {
            matches!(*prefix, "---" | "+++") && filename.starts_with('"')
        }) else {
            normalized.extend_from_slice(line.as_bytes());
            continue;
        };
        let filename_end = filename.find(['\t', '\n']).unwrap_or(filename.len());
        let (filename, suffix) = filename.split_at(filename_end);
        let filename = filename
            .strip_prefix('"')
            .and_then(|filename| filename.strip_suffix('"'))
            .context("unterminated quoted patch filename")?;
        normalized.extend_from_slice(prefix.as_bytes());
        normalized.extend_from_slice(b" \"");
        let mut bytes = filename.bytes();
        while let Some(byte) = bytes.next() {
            if byte != b'\\' {
                normalized.push(byte);
                continue;
            }
            let byte = match bytes.next().context("unterminated patch filename escape")? {
                digit @ b'0'..=b'7' => {
                    let mut value = digit - b'0';
                    for _ in 0..2 {
                        let digit = bytes
                            .next()
                            .filter(|digit| matches!(digit, b'0'..=b'7'))
                            .context("invalid octal patch filename escape")?
                            - b'0';
                        value = value
                            .checked_mul(8)
                            .and_then(|value| value.checked_add(digit))
                            .context("octal patch filename escape exceeds one byte")?;
                    }
                    value
                }
                b'a' => 0x07,
                b'b' => 0x08,
                b'f' => 0x0c,
                b'v' => 0x0b,
                b'n' => b'\n',
                b'r' => b'\r',
                b't' => b'\t',
                b'\\' => b'\\',
                b'"' => b'"',
                _ => anyhow::bail!("invalid patch filename escape"),
            };
            match byte {
                b'\0' => normalized.extend_from_slice(b"\\0"),
                b'\n' => normalized.extend_from_slice(b"\\n"),
                b'\r' => normalized.extend_from_slice(b"\\r"),
                b'\t' => normalized.extend_from_slice(b"\\t"),
                b'\\' => normalized.extend_from_slice(b"\\\\"),
                b'"' => normalized.extend_from_slice(b"\\\""),
                byte => normalized.push(byte),
            }
        }
        normalized.push(b'"');
        normalized.extend_from_slice(suffix.as_bytes());
    }
    normalized.extend_from_slice(hunks.as_bytes());
    Ok(Cow::Owned(
        String::from_utf8(normalized).context("patch filename is not UTF-8")?,
    ))
}

fn patch_path(path: Option<&str>, prefix: &str) -> Result<Option<String>> {
    let path = path.context("missing patch filename")?;
    if path == "/dev/null" {
        return Ok(None);
    }
    let path = path.strip_prefix(prefix).unwrap_or(path);
    ensure!(
        !path.is_empty() && !path.split(['/', '\\']).any(|component| component == ".."),
        "empty patch path or parent traversal"
    );
    ensure!(!path.contains('\0'), "patch path contains NUL");
    Ok(Some(path.to_owned()))
}

fn check_coordinate_bounds(section: &str) -> Result<()> {
    // diffy adds usize coordinates while checking hunk ordering. Bound them before
    // parsing so an untrusted header cannot overflow there, even in a debug build.
    for line in section.lines().filter(|line| line.starts_with("@@ ")) {
        for range in line.split_whitespace().skip(1).take(2) {
            let range = range.trim_start_matches(['-', '+']);
            let (start, count) = range.split_once(',').unwrap_or((range, "1"));
            let start = start.parse::<u32>().context("invalid hunk start")?;
            let count = count.parse::<u32>().context("invalid hunk length")?;
            ensure!(start > 0 || count == 0, "nonempty hunk starts at line zero");
            start
                .checked_add(count)
                .context("hunk coordinate overflow")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parses_sparse_multi_file_hunks() {
        let files = parse_patch(indoc! {"
            diff --git a//project/one.rs b//project/one.rs
            index abc..def 100644
            --- a//project/one.rs
            +++ b//project/one.rs
            @@ -100,2 +100,3 @@ function
             context
            -old
            +new
            +added
            @@ -900,1 +901,1 @@
            -before
            +after
            diff --git /project/two.rs /project/two.rs
            --- /project/two.rs
            +++ /project/two.rs
            @@ -1 +1 @@
            -old
            +new
        "})
        .expect("multi-file patch");
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].old_path.as_deref(), Some("/project/one.rs"));
        assert_eq!(files[0].hunks[0].old_start, 100);
        assert_eq!(files[0].hunks[0].new_count, 3);
        assert_eq!(files[0].hunks[0].old_text, "context\nold\n");
        assert_eq!(files[0].hunks[0].new_text, "context\nnew\nadded\n");
        assert_eq!(files[0].hunks[1].new_start, 901);
        assert_eq!(files[1].new_path.as_deref(), Some("/project/two.rs"));
    }

    #[test]
    fn parses_add_delete_and_missing_newline() {
        let files = parse_patch(indoc! {r#"
            diff --git a//new b//new
            new file mode 100644
            --- /dev/null
            +++ b//new
            @@ -0,0 +1 @@
            +added
            \ No newline at end of file
            diff --git a//old b//old
            deleted file mode 100644
            --- a//old
            +++ /dev/null
            @@ -1 +0,0 @@
            -deleted
            \ No newline at end of file
        "#})
        .expect("add and delete");
        assert_eq!(files[0].old_path, None);
        assert_eq!(files[0].hunks[0].old_text, "");
        assert_eq!(files[0].hunks[0].new_text, "added");
        assert_eq!(files[1].new_path, None);
        assert_eq!(files[1].hunks[0].old_text, "deleted");
        assert_eq!(files[1].hunks[0].new_text, "");
    }

    #[test]
    fn parses_git_quoted_paths_and_file_operations() {
        let files = parse_patch(indoc! {r#"
            diff --git "a//project/a\tfile" "b//project/a\tfile"
            --- "a//project/a\tfile"
            +++ "b//project/a\tfile"
            @@ -1 +1 @@
            -old
            +new
            diff --git a//project/a file b//project/a file
            --- a//project/a file
            +++ b//project/a file
            @@ -1 +1 @@
            -old
            +new
            diff --git "/project/\303\251t\303\251.rs" "/project/\346\226\260.rs"
            similarity index 75%
            rename from "/project/\303\251t\303\251.rs"
            rename to "/project/\346\226\260.rs"
            --- "/project/\303\251t\303\251.rs"
            +++ "/project/\346\226\260.rs"
            @@ -1 +1 @@
            -old
            +new
            diff --git /project/source /project/copy
            similarity index 75%
            copy from /project/source
            copy to /project/copy
            --- /project/source
            +++ /project/copy
            @@ -1 +1 @@
            --- "this is hunk text, not a filename"
            +new
            diff --git "/project/\a\b\f\n\r\t\v\\\"\303\251" "/project/\a\b\f\n\r\t\v\\\"\303\251"
            --- "/project/\a\b\f\n\r\t\v\\\"\303\251"
            +++ "/project/\007\010\014\012\015\011\013\134\042\303\251"
            @@ -1 +1 @@
            -old
            +new
            diff --git a/src/file.rs b/src/file.rs
            --- a/src/file.rs
            +++ b/src/file.rs
            @@ -1 +1 @@
            -old
            +new
        "#})
        .expect("quoted and spaced paths");
        assert_eq!(files[0].old_path.as_deref(), Some("/project/a\tfile"));
        assert_eq!(files[1].old_path.as_deref(), Some("/project/a file"));
        assert_eq!(files[2].old_path.as_deref(), Some("/project/été.rs"));
        assert_eq!(files[2].new_path.as_deref(), Some("/project/新.rs"));
        assert_eq!(files[3].old_path.as_deref(), Some("/project/source"));
        assert_eq!(files[3].new_path.as_deref(), Some("/project/copy"));
        assert_eq!(
            files[3].hunks[0].old_text,
            "-- \"this is hunk text, not a filename\"\n"
        );
        assert_eq!(
            files[4].old_path.as_deref(),
            Some("/project/\u{7}\u{8}\u{c}\n\r\t\u{b}\\\"é")
        );
        assert_eq!(files[4].new_path, files[4].old_path);
        assert_eq!(files[5].old_path.as_deref(), Some("src/file.rs"));
        assert_eq!(files[5].new_path.as_deref(), Some("src/file.rs"));
    }

    #[test]
    fn leaves_nontext_operations_to_structured_changes() {
        assert!(
            parse_patch(indoc! {"
            diff --git /old /new
            similarity index 100%
            rename from /old
            rename to /new
            diff --git /source /copy
            similarity index 100%
            copy from /source
            copy to /copy
            diff --git a//image b//image
            Binary files a//image and b//image differ
        "})
            .expect("file-level changes")
            .is_empty()
        );
    }

    #[test]
    fn parses_unified_patches_without_git_headers() {
        let files = parse_patch("--- /file\n+++ /file\n@@ -10 +10 @@\n-before\n+after\n")
            .expect("unified patch");
        assert_eq!(files[0].old_path.as_deref(), Some("/file"));
        assert_eq!(files[0].hunks[0].new_text, "after\n");
        let files = parse_patch("@@ -10 +10 @@\n-before\n+after\n").expect("bare hunks");
        assert_eq!(
            (files[0].old_path.as_ref(), files[0].new_path.as_ref()),
            (None, None)
        );
        assert_eq!(files[0].hunks[0].old_start, 10);
        let files = parse_patch("@@ -1 +1 @@\n--- \"not a header\"\n+after\n")
            .expect("bare hunk with header-like content");
        assert_eq!(files[0].hunks[0].old_text, "-- \"not a header\"\n");
    }

    #[test]
    fn preserves_unicode_and_documents_diffy_boundaries() {
        let unified = "--- /été.rs\n+++ /été.rs\n@@ -1 +1 @@\n-été 🦀\n+新しい 🦀\n";
        let files = parse_patch(unified).expect("Unicode patch");
        assert_eq!(files[0].old_path.as_deref(), Some("/été.rs"));
        assert_eq!(files[0].hunks[0].old_text, "été 🦀\n");
        assert_eq!(files[0].hunks[0].new_text, "新しい 🦀\n");
        assert!(
            parse_patch(&unified.repeat(2)).is_err(),
            "multiple files need Git section boundaries"
        );
        assert!(
            parse_patch("diff --git /one /one\nunrecognized preamble\n")
                .expect("diffy skips non-hunk preamble")
                .is_empty(),
            "structured changes, not Git metadata, provide non-text file rows"
        );
        assert!(
            parse_patch("--- /one\n@@ -1 +1 @@\n-old\n+new\n").is_err(),
            "a partial filename pair is not a bare hunk"
        );
    }

    #[test]
    fn rejects_malformed_or_unsupported_text_patches() {
        for patch in [
            "commit abc\ndiff --git /old /new\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -1,2 +1 @@\n-old\n+new\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -2 +2 @@\n-a\n+b\n@@ -1 +1 @@\n-c\n+d\n",
            "diff --git a/../relative b/../relative\n--- a/../relative\n+++ b/../relative\n@@ -1 +1 @@\n-a\n+b\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -1 +1 @@\n-a\n+b\ntrailing garbage\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -18446744073709551615 +1 @@\n-a\n+b\n@@ -2 +2 @@\n-c\n+d\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -0 +0 @@\n-a\n+b\n",
            "diff --git /old /new\n--- \"a//\\377\"\n+++ /new\n@@ -1 +1 @@\n-a\n+b\n",
            "diff --git /old /new\n--- \"a//\\777\"\n+++ /new\n@@ -1 +1 @@\n-a\n+b\n",
            "diff --git /old /new\n--- \"a//\\30\"\n+++ /new\n@@ -1 +1 @@\n-a\n+b\n",
            "diff --git /old /new\n--- \"a//\\000\"\n+++ /new\n@@ -1 +1 @@\n-a\n+b\n",
        ] {
            assert!(parse_patch(patch).is_err(), "{patch}");
        }
    }
}
