// Take a look at the license at the top of the repository in the LICENSE file.

use super::utils::{show_error, TestResult};
use std::ffi::OsStr;
use std::path::Path;

fn to_correct_name(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_uppercase() {
            if !out.is_empty() {
                out.push('_');
            }
            out.push_str(c.to_lowercase().to_string().as_str());
        } else {
            out.push(c);
        }
    }
    out
}

fn check_md_doc_path(p: &Path, md_line: &str, ty_line: &str) -> bool {
    let parts = md_line.split('/').collect::<Vec<_>>();
    if let Some(md_name) = parts.last().and_then(|n| n.split(".md").next()) {
        if let Some(name) = ty_line.split_whitespace().filter(|s| !s.is_empty()).nth(2) {
            if let Some(name) = name
                .split('<')
                .next()
                .and_then(|n| n.split('{').next())
                .and_then(|n| n.split('(').next())
                .and_then(|n| n.split(';').next())
            {
                let correct = to_correct_name(name);
                if correct.as_str() == md_name {
                    return true;
                }
                show_error(
                    p,
                    &format!(
                        "Invalid markdown file name `{md_name}`, should have been `{correct}`",
                    ),
                );
                return false;
            }
        }
        show_error(p, &format!("Cannot extract type name from `{ty_line}`"));
    } else {
        show_error(p, &format!("Cannot extract md name from `{md_line}`"));
    }
    false
}

fn check_doc_comments_before(p: &Path, lines: &[&str], start: usize) -> bool {
    let mut found_docs = false;

    for pos in (0..start).rev() {
        let trimmed = lines[pos].trim();
        if trimmed.starts_with("///") {
            if !lines[start].trim().starts_with("pub enum ThreadStatus {") {
                show_error(
                    p,
                    &format!(
                        "Types should use common documentation by using `#[doc = include_str!(` \
                         and by putting the markdown file in the `md_doc` folder instead of `{}`",
                        &lines[pos],
                    ),
                );
                return false;
            }
            return true;
        } else if trimmed.starts_with("#[doc = include_str!(") {
            found_docs = true;
            if !check_md_doc_path(p, trimmed, lines[start]) {
                return false;
            }
        } else if !trimmed.starts_with("#[") && !trimmed.starts_with("//") {
            break;
        }
    }
    if !found_docs {
        show_error(
            p,
            &format!(
                "Missing documentation for public item: `{}` (if it's not supposed to be a public \
                 item, use `pub(crate)` instead)",
                lines[start],
            ),
        );
        return false;
    }
    true
}

pub fn check_docs(content: &str, p: &Path) -> TestResult {
    let mut res = TestResult {
        nb_tests: 0,
        nb_errors: 0,
    };

    // No need to check if we are in the `src` or `src/common` folder or if we are in a `ffi.rs`
    // file.
    if p.file_name().unwrap() == OsStr::new("ffi.rs") {
        return res;
    }
    let path = format!(
        "/{}",
        p.parent().unwrap().display().to_string().replace('\\', "/")
    );
    if path.ends_with("/src") || path.ends_with("src/common") {
        return res;
    }
    let lines = content.lines().collect::<Vec<_>>();

    for pos in 1..lines.len() {
        let line = lines[pos];
        let trimmed = line.trim();
        if trimmed.starts_with("//!") {
            show_error(p, "There shouln't be inner doc comments (`//!`)");
            res.nb_tests += 1;
            res.nb_errors += 1;
            continue;
        } else if !line.starts_with("pub fn ")
            && !trimmed.starts_with("pub struct ")
            && !trimmed.starts_with("pub enum ")
        {
            continue;
        }
        res.nb_tests += 1;
        if !check_doc_comments_before(p, &lines, pos) {
            res.nb_errors += 1;
        }
    }
    res
}
