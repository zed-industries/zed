use std::str::FromStr;

pub struct GitDiff {
    files: Vec<GitDiffFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDiffFile {
    pub old_path: String,
    pub new_path: String,
    pub is_binary: bool,
    pub is_renamed: bool,
    pub is_deleted: bool,
    pub is_new: bool,
    pub hunks: Vec<GitDiffHunk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDiffHunk {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<GitDiffLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitDiffLine {
    pub line_type: GitDiffLineType,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitDiffLineType {
    Context,
    Addition,
    Deletion,
}

impl GitDiff {
    pub fn files(&self) -> &[GitDiffFile] {
        &self.files
    }
}

impl FromStr for GitDiff {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut lines = s.lines().peekable();

        while let Some(line) = lines.next() {
            if line.starts_with("diff --git ") {
                let file = parse_diff_file(line, &mut lines)?;
                files.push(file);
            }
        }

        Ok(GitDiff { files })
    }
}

fn parse_diff_file(
    diff_line: &str,
    lines: &mut std::iter::Peekable<std::str::Lines>,
) -> Result<GitDiffFile, anyhow::Error> {
    // Parse "diff --git a/path/to/file b/path/to/file"
    let paths = diff_line
        .strip_prefix("diff --git ")
        .ok_or_else(|| anyhow::anyhow!("Invalid diff header: {}", diff_line))?;

    let mut parts = paths.split(' ');
    let old_path = parts
        .next()
        .and_then(|p| p.strip_prefix("a/"))
        .unwrap_or("")
        .to_string();
    let new_path = parts
        .next()
        .and_then(|p| p.strip_prefix("b/"))
        .unwrap_or("")
        .to_string();

    let mut is_binary = false;
    let mut is_renamed = false;
    let mut is_deleted = false;
    let mut is_new = false;
    let mut hunks = Vec::new();

    // Parse metadata lines (index, ---, +++, etc.)
    while let Some(line) = lines.peek() {
        if line.starts_with("index ")
            || line.starts_with("new file mode ")
            || line.starts_with("deleted file mode ")
            || line.starts_with("similarity index ")
            || line.starts_with("rename from ")
            || line.starts_with("rename to ")
            || line.starts_with("old mode ")
            || line.starts_with("new mode ")
        {
            let line = lines.next().unwrap();

            if line.starts_with("deleted file mode ") {
                is_deleted = true;
            } else if line.starts_with("new file mode ") {
                is_new = true;
            } else if line.starts_with("rename from ") || line.starts_with("rename to ") {
                is_renamed = true;
            }

            continue;
        }

        if line.starts_with("Binary files ") || line.contains("binary file") {
            is_binary = true;
            lines.next();
            continue;
        }

        if line.starts_with("--- ") {
            lines.next(); // Skip "--- a/file"
            if let Some(line) = lines.next() {
                if !line.starts_with("+++ ") {
                    return Err(anyhow::anyhow!("Expected '+++ b/file' after '--- a/file'"));
                }
            }
            break;
        }

        // If we're not at metadata lines anymore, break
        if line.starts_with("@@ ") {
            break;
        }

        // If an unexpected line appears, move on
        lines.next();
    }

    // Parse hunks
    while let Some(line) = lines.peek() {
        if line.starts_with("@@ ") {
            let hunk = parse_hunk(lines)?;
            hunks.push(hunk);
        } else if line.starts_with("diff --git ") {
            break;
        } else {
            lines.next();
        }
    }

    Ok(GitDiffFile {
        old_path,
        new_path,
        is_binary,
        is_renamed,
        is_deleted,
        is_new,
        hunks,
    })
}

fn parse_hunk(
    lines: &mut std::iter::Peekable<std::str::Lines>,
) -> Result<GitDiffHunk, anyhow::Error> {
    let hunk_header = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("Expected hunk header"))?;

    // Parse "@@ -start,count +start,count @@" format
    let (old_range, new_range) = parse_hunk_header(hunk_header)?;

    let old_start = old_range.0;
    let old_lines = old_range.1;
    let new_start = new_range.0;
    let new_lines = new_range.1;

    let mut hunk_lines = Vec::new();

    while let Some(line) = lines.peek() {
        if line.starts_with("@@ ") || line.starts_with("diff --git ") {
            break;
        }

        let line = lines.next().unwrap();
        let line_type = if line.is_empty() {
            GitDiffLineType::Context
        } else {
            match line.chars().next() {
                Some('+') => GitDiffLineType::Addition,
                Some('-') => GitDiffLineType::Deletion,
                _ => GitDiffLineType::Context,
            }
        };

        let content = if line.is_empty() {
            " ".to_string()
        } else {
            line.to_string()
        };

        hunk_lines.push(GitDiffLine { line_type, content });
    }

    Ok(GitDiffHunk {
        old_start,
        old_lines,
        new_start,
        new_lines,
        lines: hunk_lines,
    })
}

fn parse_hunk_header(header: &str) -> Result<((u32, u32), (u32, u32)), anyhow::Error> {
    let header = header
        .strip_prefix("@@ ")
        .and_then(|h| h.strip_suffix(" @@"))
        .or_else(|| {
            header
                .strip_prefix("@@ ")
                .and_then(|h| h.strip_suffix(" @@ "))
        })
        .ok_or_else(|| anyhow::anyhow!("Invalid hunk header: {}", header))?;

    let mut parts = header.split(' ');

    let old_range = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing old range in hunk header"))?;
    let new_range = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Missing new range in hunk header"))?;

    let old_range = parse_range(old_range.strip_prefix('-').unwrap_or(old_range))?;
    let new_range = parse_range(new_range.strip_prefix('+').unwrap_or(new_range))?;

    Ok((old_range, new_range))
}

fn parse_range(range: &str) -> Result<(u32, u32), anyhow::Error> {
    if let Some((start, count)) = range.split_once(',') {
        let start = start.parse::<u32>()?;
        let count = count.parse::<u32>()?;
        Ok((start, count))
    } else {
        // If no count is provided, it defaults to 1
        let start = range.parse::<u32>()?;
        Ok((start, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_diff() {
        let diff = GitDiff::from_str("").unwrap();
        assert_eq!(diff.files(), &[]);
    }

    #[test]
    fn test_parse_simple_diff() {
        let diff_str = r#"diff --git a/file.txt b/file.txt
index 1234567..abcdefg 100644
--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,4 @@
 Context line
-Deleted line
+Added line
 Another context line
+One more added line
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "file.txt".to_string(),
                new_path: "file.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![GitDiffHunk {
                    old_start: 1,
                    old_lines: 3,
                    new_start: 1,
                    new_lines: 4,
                    lines: vec![
                        GitDiffLine {
                            line_type: GitDiffLineType::Context,
                            content: " Context line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Deleted line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+Added line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Context,
                            content: " Another context line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+One more added line".to_string(),
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn test_parse_new_file() {
        let diff_str = r#"diff --git a/new_file.txt b/new_file.txt
new file mode 100644
index 0000000..1234567
--- /dev/null
+++ b/new_file.txt
@@ -0,0 +1,2 @@
+First line
+Second line
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "new_file.txt".to_string(),
                new_path: "new_file.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: true,
                hunks: vec![GitDiffHunk {
                    old_start: 0,
                    old_lines: 0,
                    new_start: 1,
                    new_lines: 2,
                    lines: vec![
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+First line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+Second line".to_string(),
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn test_parse_deleted_file() {
        let diff_str = r#"diff --git a/deleted_file.txt b/deleted_file.txt
deleted file mode 100644
index 1234567..0000000
--- a/deleted_file.txt
+++ /dev/null
@@ -1,3 +0,0 @@
-Line 1
-Line 2
-Line 3
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "deleted_file.txt".to_string(),
                new_path: "deleted_file.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: true,
                is_new: false,
                hunks: vec![GitDiffHunk {
                    old_start: 1,
                    old_lines: 3,
                    new_start: 0,
                    new_lines: 0,
                    lines: vec![
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Line 1".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Line 2".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Line 3".to_string(),
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn test_parse_renamed_file() {
        let diff_str = r#"diff --git a/old_name.txt b/new_name.txt
similarity index 100%
rename from old_name.txt
rename to new_name.txt
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "old_name.txt".to_string(),
                new_path: "new_name.txt".to_string(),
                is_binary: false,
                is_renamed: true,
                is_deleted: false,
                is_new: false,
                hunks: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_binary_file() {
        let diff_str = r#"diff --git a/binary.bin b/binary.bin
index 1234567..abcdefg 100644
Binary files a/binary.bin and b/binary.bin differ
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "binary.bin".to_string(),
                new_path: "binary.bin".to_string(),
                is_binary: true,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_multiple_files() {
        let diff_str = r#"diff --git a/file1.txt b/file1.txt
index 1234567..abcdefg 100644
--- a/file1.txt
+++ b/file1.txt
@@ -1,2 +1,3 @@
 Context
+Added
diff --git a/file2.txt b/file2.txt
index 7654321..gfedcba 100644
--- a/file2.txt
+++ b/file2.txt
@@ -5,2 +5,1 @@
 Context
-Removed
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[
                GitDiffFile {
                    old_path: "file1.txt".to_string(),
                    new_path: "file1.txt".to_string(),
                    is_binary: false,
                    is_renamed: false,
                    is_deleted: false,
                    is_new: false,
                    hunks: vec![GitDiffHunk {
                        old_start: 1,
                        old_lines: 2,
                        new_start: 1,
                        new_lines: 3,
                        lines: vec![
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Addition,
                                content: "+Added".to_string(),
                            },
                        ],
                    }],
                },
                GitDiffFile {
                    old_path: "file2.txt".to_string(),
                    new_path: "file2.txt".to_string(),
                    is_binary: false,
                    is_renamed: false,
                    is_deleted: false,
                    is_new: false,
                    hunks: vec![GitDiffHunk {
                        old_start: 5,
                        old_lines: 2,
                        new_start: 5,
                        new_lines: 1,
                        lines: vec![
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Deletion,
                                content: "-Removed".to_string(),
                            },
                        ],
                    }],
                },
            ]
        );
    }

    #[test]
    fn test_parse_multiple_hunks() {
        let diff_str = r#"diff --git a/multi_hunk.txt b/multi_hunk.txt
index 1234567..abcdefg 100644
--- a/multi_hunk.txt
+++ b/multi_hunk.txt
@@ -1,3 +1,4 @@
 Context 1
+Added 1
 Context 2
 Context 3
@@ -10,2 +11,1 @@
 Context 4
-Deleted 1
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "multi_hunk.txt".to_string(),
                new_path: "multi_hunk.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![
                    GitDiffHunk {
                        old_start: 1,
                        old_lines: 3,
                        new_start: 1,
                        new_lines: 4,
                        lines: vec![
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context 1".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Addition,
                                content: "+Added 1".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context 2".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context 3".to_string(),
                            },
                        ],
                    },
                    GitDiffHunk {
                        old_start: 10,
                        old_lines: 2,
                        new_start: 11,
                        new_lines: 1,
                        lines: vec![
                            GitDiffLine {
                                line_type: GitDiffLineType::Context,
                                content: " Context 4".to_string(),
                            },
                            GitDiffLine {
                                line_type: GitDiffLineType::Deletion,
                                content: "-Deleted 1".to_string(),
                            },
                        ],
                    },
                ],
            }]
        );
    }

    #[test]
    fn test_parse_range_formats() {
        // Test hunk header with single line format (implying count=1)
        let diff_str = r#"diff --git a/single_line.txt b/single_line.txt
index 1234567..abcdefg 100644
--- a/single_line.txt
+++ b/single_line.txt
@@ -5 +6 @@
-Old line
+New line
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "single_line.txt".to_string(),
                new_path: "single_line.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![GitDiffHunk {
                    old_start: 5,
                    old_lines: 1,
                    new_start: 6,
                    new_lines: 1,
                    lines: vec![
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Old line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+New line".to_string(),
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn test_parse_empty_lines() {
        let diff_str = r#"diff --git a/file_with_empty.txt b/file_with_empty.txt
index 1234567..abcdefg 100644
--- a/file_with_empty.txt
+++ b/file_with_empty.txt
@@ -1,4 +1,5 @@
 First line

-Deleted line
+Added line
 Last line
+
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "file_with_empty.txt".to_string(),
                new_path: "file_with_empty.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![GitDiffHunk {
                    old_start: 1,
                    old_lines: 4,
                    new_start: 1,
                    new_lines: 5,
                    lines: vec![
                        GitDiffLine {
                            line_type: GitDiffLineType::Context,
                            content: " First line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Context,
                            content: " ".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Deletion,
                            content: "-Deleted line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+Added line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Context,
                            content: " Last line".to_string(),
                        },
                        GitDiffLine {
                            line_type: GitDiffLineType::Addition,
                            content: "+".to_string(),
                        },
                    ],
                }],
            }]
        );
    }

    #[test]
    fn test_parse_mode_changes() {
        let diff_str = r#"diff --git a/file.txt b/file.txt
old mode 100644
new mode 100755
"#;
        let diff = GitDiff::from_str(diff_str).unwrap();
        assert_eq!(
            diff.files(),
            &[GitDiffFile {
                old_path: "file.txt".to_string(),
                new_path: "file.txt".to_string(),
                is_binary: false,
                is_renamed: false,
                is_deleted: false,
                is_new: false,
                hunks: vec![],
            }]
        );
    }
}
