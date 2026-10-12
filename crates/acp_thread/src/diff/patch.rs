use anyhow::{Context as _, Result, ensure};
use diffy::{Line, Patch};
use util::paths::{PathStyle, is_absolute};

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
        let patch = Patch::from_str(section).context("invalid Git patch")?;
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

fn patch_path(path: Option<&str>, prefix: &str) -> Result<Option<String>> {
    let path = path.context("missing patch filename")?;
    if path == "/dev/null" {
        return Ok(None);
    }
    let path = path.strip_prefix(prefix).unwrap_or(path);
    ensure!(
        is_absolute(path, PathStyle::Windows),
        "patch path must be absolute: {path}"
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
    fn delegates_quoted_and_spaced_paths_to_diffy() {
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
        "#})
        .expect("quoted and spaced paths");
        assert_eq!(files[0].old_path.as_deref(), Some("/project/a\tfile"));
        assert_eq!(files[1].old_path.as_deref(), Some("/project/a file"));
    }

    #[test]
    fn leaves_nontext_operations_to_structured_changes() {
        assert!(
            parse_patch(indoc! {"
            diff --git a//old b//new
            similarity index 100%
            rename from /old
            rename to /new
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
            "diff --git a/relative b/relative\n--- a/relative\n+++ b/relative\n@@ -1 +1 @@\n-a\n+b\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -1 +1 @@\n-a\n+b\ntrailing garbage\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -18446744073709551615 +1 @@\n-a\n+b\n@@ -2 +2 @@\n-c\n+d\n",
            "diff --git /old /new\n--- /old\n+++ /new\n@@ -0 +0 @@\n-a\n+b\n",
            "diff --git /old /new\n--- \"a//\\303\\251\"\n+++ /new\n@@ -1 +1 @@\n-a\n+b\n",
        ] {
            assert!(parse_patch(patch).is_err(), "{patch}");
        }
    }
}
