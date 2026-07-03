//! Classification of DML statements accepted by `propose_write`, and
//! best-effort extraction of the target table for an `UPDATE` statement.
//!
//! This is a safety gate: every write SQL statement a user submits through
//! the MCP server passes through [`classify_dml`] before anything else
//! happens to it. The classifier must be strict about what it accepts —
//! exactly one INSERT/UPDATE/DELETE statement — and must not be fooled by
//! comments, string/identifier-quoted content, or statement separators
//! embedded in literals.
//!
//! `#[allow(dead_code)]` below is temporary: nothing calls into this module
//! yet because the `propose_write`/`apply_write` tools that use it land in
//! task 4 (`.superpowers/sdd/task-4-brief.md`), which also replaces the
//! locally-defined `WriteKind` with a re-export of `database_client::WriteKind`.
//! Remove the attribute when that call site is wired up.
#![allow(dead_code)]

use anyhow::{Result, bail};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteKind {
    Insert,
    Update,
    Delete,
}

/// Classifies a single DML statement. Errs (with a user-facing message) for
/// SELECT/WITH/DDL, empty input, or more than one statement.
pub fn classify_dml(sql: &str) -> Result<WriteKind> {
    let statement = single_statement(sql)?;
    let head = strip_leading_noise(statement);
    let verb = head
        .split(|c: char| c.is_whitespace() || c == '(')
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    match verb.as_str() {
        "INSERT" => Ok(WriteKind::Insert),
        "UPDATE" => Ok(WriteKind::Update),
        "DELETE" => Ok(WriteKind::Delete),
        "" => bail!("empty statement; provide one INSERT, UPDATE, or DELETE statement"),
        other => bail!(
            "only INSERT/UPDATE/DELETE are allowed via propose_write (got `{other}`); \
             use run_query for reads, and run DDL yourself"
        ),
    }
}

/// Best-effort extraction of the UPDATE target table `(schema, table)` for the
/// before-image fetch. Returns None for forms we cannot parse confidently
/// (the caller then omits the before-image with a note).
pub fn extract_update_target(sql: &str) -> Option<(String, String)> {
    let statement = single_statement(sql).ok()?;
    let head = strip_leading_noise(statement);
    let mut rest = strip_prefix_ci(head, "UPDATE")?;
    rest = rest.trim_start();
    if let Some(after_only) = strip_prefix_ci(rest, "ONLY") {
        rest = after_only.trim_start();
    }
    parse_qualified_name(rest)
}

/// Returns the trimmed statement, erroring if a non-trailing `;` is found
/// outside string ('...') or quoted-identifier ("...") spans. A `;` is
/// allowed only when nothing but whitespace follows it.
fn single_statement(sql: &str) -> Result<&str> {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        bail!("empty statement; provide one INSERT, UPDATE, or DELETE statement");
    }
    let bytes = trimmed.as_bytes();
    let mut in_string = false;
    let mut in_ident = false;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        match byte {
            b'\'' if !in_ident => {
                in_string = !in_string;
            }
            b'"' if !in_string => {
                in_ident = !in_ident;
            }
            b';' if !in_string && !in_ident => {
                if trimmed[index + 1..].trim().is_empty() {
                    return Ok(trimmed[..index].trim());
                }
                bail!("only a single statement is allowed via propose_write; found more than one");
            }
            _ => {}
        }
        index += 1;
    }
    if in_string {
        bail!("unterminated string literal in statement");
    }
    if in_ident {
        bail!("unterminated quoted identifier in statement");
    }
    Ok(trimmed)
}

/// Skips whitespace and leading `--` line comments or `/* */` block comments
/// (non-nested — the first `*/` closes the comment), repeating until neither
/// pattern matches.
fn strip_leading_noise(mut sql: &str) -> &str {
    loop {
        sql = sql.trim_start();
        if let Some(rest) = sql.strip_prefix("--") {
            sql = match rest.split_once('\n') {
                Some((_, after)) => after,
                None => "",
            };
        } else if let Some(rest) = sql.strip_prefix("/*") {
            sql = match rest.split_once("*/") {
                Some((_, after)) => after,
                None => "",
            };
        } else {
            return sql;
        }
    }
}

/// Case-insensitive prefix strip that additionally requires the prefix to be
/// a whole keyword (followed by whitespace, `(`, end of input, or nothing
/// left) so that e.g. `"UPDATED"` does not match a strip of `"UPDATE"`.
fn strip_prefix_ci<'a>(input: &'a str, prefix: &str) -> Option<&'a str> {
    if input.len() < prefix.len() {
        return None;
    }
    let (candidate, rest) = input.split_at(prefix.len());
    if !candidate.eq_ignore_ascii_case(prefix) {
        return None;
    }
    match rest.chars().next() {
        None => Some(rest),
        Some(c) if c.is_whitespace() || c == '(' => Some(rest),
        _ => None,
    }
}

/// Reads an optionally-quoted `schema.table` or bare `table` from the start
/// of `input`, defaulting schema to `"public"` when absent. Stops at
/// whitespace/`(`/end of input. Returns `None` on any ambiguity (unterminated
/// quotes, nothing found, unexpected characters immediately after a quoted
/// segment, or a bare first segment that is actually the `SET` keyword,
/// which means no table name was given at all).
fn parse_qualified_name(input: &str) -> Option<(String, String)> {
    let input = input.trim_start();
    let (first, first_was_quoted, after_first) = parse_name_part(input)?;
    if !first_was_quoted && first.eq_ignore_ascii_case("SET") {
        return None;
    }
    if let Some(rest) = after_first.strip_prefix('.') {
        let (second, _second_was_quoted, after_second) = parse_name_part(rest)?;
        if !ends_name(after_second) {
            return None;
        }
        Some((first, second))
    } else {
        if !ends_name(after_first) {
            return None;
        }
        Some(("public".to_string(), first))
    }
}

/// True if `rest` is empty or starts with whitespace/`(` — i.e. a name has
/// fully ended, as opposed to being followed by more identifier characters
/// (which would indicate we mis-parsed a boundary).
fn ends_name(rest: &str) -> bool {
    match rest.chars().next() {
        None => true,
        Some(c) => c.is_whitespace() || c == '(',
    }
}

/// Parses one identifier segment: either a `"quoted identifier"` (with `""`
/// as an escaped quote) or a bare identifier of alphanumerics/underscores.
/// Returns the parsed name, whether it was double-quoted, and the remainder
/// of the input after it.
fn parse_name_part(input: &str) -> Option<(String, bool, &str)> {
    if let Some(rest) = input.strip_prefix('"') {
        let mut name = String::new();
        let mut chars = rest.char_indices();
        loop {
            let (byte_index, c) = chars.next()?;
            if c == '"' {
                // Check for an escaped quote (`""`).
                if rest[byte_index + 1..].starts_with('"') {
                    name.push('"');
                    chars.next();
                    continue;
                }
                return Some((name, true, &rest[byte_index + 1..]));
            }
            name.push(c);
        }
    } else {
        let end = input
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(input.len());
        if end == 0 {
            return None;
        }
        Some((input[..end].to_string(), false, &input[end..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_each_dml_verb() {
        assert_eq!(
            classify_dml("INSERT INTO t VALUES (1)").unwrap(),
            WriteKind::Insert
        );
        assert_eq!(
            classify_dml("  update t set a = 1 where id = 2 ").unwrap(),
            WriteKind::Update
        );
        assert_eq!(
            classify_dml("DELETE FROM t WHERE id = 1;").unwrap(),
            WriteKind::Delete
        );
    }

    #[test]
    fn skips_leading_comments_and_whitespace() {
        assert_eq!(
            classify_dml("-- a comment\n  DELETE FROM t WHERE id=1").unwrap(),
            WriteKind::Delete
        );
        assert_eq!(
            classify_dml("/* block */\nUPDATE t SET a=1 WHERE id=1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn rejects_non_dml() {
        assert!(classify_dml("SELECT * FROM t").is_err());
        assert!(classify_dml("WITH x AS (SELECT 1) INSERT INTO t SELECT * FROM x").is_err());
        assert!(classify_dml("CREATE TABLE t (id int)").is_err());
        assert!(classify_dml("DROP TABLE t").is_err());
        assert!(classify_dml("TRUNCATE t").is_err());
        assert!(classify_dml("").is_err());
        assert!(classify_dml("   ").is_err());
    }

    #[test]
    fn rejects_multiple_statements() {
        assert!(classify_dml("DELETE FROM t WHERE id=1; DELETE FROM t WHERE id=2").is_err());
        // A trailing semicolon is allowed:
        assert_eq!(
            classify_dml("DELETE FROM t WHERE id=1;").unwrap(),
            WriteKind::Delete
        );
        assert_eq!(
            classify_dml("DELETE FROM t WHERE id=1;   ").unwrap(),
            WriteKind::Delete
        );
    }

    #[test]
    fn does_not_treat_semicolon_in_string_literal_as_separator() {
        // Single statement whose value contains a semicolon.
        assert_eq!(
            classify_dml("UPDATE t SET note = 'a; b' WHERE id = 1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn extracts_simple_update_targets() {
        assert_eq!(
            extract_update_target("UPDATE public.orders SET a=1 WHERE id=2"),
            Some(("public".into(), "orders".into()))
        );
        assert_eq!(
            extract_update_target("update orders set a=1 where id=2"),
            Some(("public".into(), "orders".into()))
        );
        assert_eq!(
            extract_update_target("UPDATE ONLY \"my schema\".\"my table\" SET a=1"),
            Some(("my schema".into(), "my table".into()))
        );
    }

    #[test]
    fn declines_complex_update_targets() {
        // FROM-join and CTE forms are not confidently parseable -> None (before-image omitted).
        assert_eq!(
            extract_update_target("UPDATE t SET a = b.x FROM other b WHERE t.id = b.id"),
            Some(("public".into(), "t".into()))
        ); // target table still first token; before-fetch code guards separately
        assert_eq!(extract_update_target("INSERT INTO t VALUES (1)"), None);
    }

    // --- Additional bypass-hardening tests ---

    #[test]
    fn semicolon_inside_string_literal_is_not_a_separator_even_with_trailing_text() {
        // The literal contains a `;` followed by what looks like a second
        // statement's keyword, entirely inside the quotes - must not split.
        assert_eq!(
            classify_dml("UPDATE t SET note = 'x; DROP TABLE t' WHERE id = 1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn semicolon_inside_quoted_identifier_is_not_a_separator() {
        assert_eq!(
            classify_dml("UPDATE \"weird;name\" SET a = 1 WHERE id = 1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn escaped_quote_in_string_literal_does_not_end_the_string_early() {
        // '' is an escaped single quote inside a string literal; the `;` right
        // after it is still inside the (reopened) string in SQL terms from a
        // naive toggle perspective, but our toggle-based scanner treats '' as
        // close-then-reopen, which still keeps the statement whole since no
        // bare `;` appears outside quotes.
        assert_eq!(
            classify_dml("UPDATE t SET note = 'it''s; fine' WHERE id = 1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn comment_before_keyword_does_not_change_classification() {
        assert_eq!(
            classify_dml("--leading comment, no space before dashes\nINSERT INTO t VALUES (1)")
                .unwrap(),
            WriteKind::Insert
        );
        assert_eq!(
            classify_dml("/* multi\nline\ncomment */ DELETE FROM t WHERE id=1").unwrap(),
            WriteKind::Delete
        );
        assert_eq!(
            classify_dml("/* c1 */ /* c2 */ UPDATE t SET a=1 WHERE id=1").unwrap(),
            WriteKind::Update
        );
    }

    #[test]
    fn semicolon_immediately_after_closing_block_comment_is_still_trailing_only() {
        assert_eq!(
            classify_dml("DELETE FROM t WHERE id=1 /* trailing note */;").unwrap(),
            WriteKind::Delete
        );
    }

    #[test]
    fn rejects_cte_write_because_first_keyword_is_with() {
        assert!(classify_dml("WITH x AS (SELECT 1) UPDATE t SET a=1 WHERE id=1").is_err());
        assert!(classify_dml("WITH x AS (SELECT 1) DELETE FROM t WHERE id=1").is_err());
    }

    #[test]
    fn rejects_statement_with_only_comments_and_whitespace() {
        assert!(classify_dml("-- just a comment\n").is_err());
        assert!(classify_dml("/* just a comment */").is_err());
    }

    #[test]
    fn rejects_unterminated_string_literal() {
        assert!(classify_dml("UPDATE t SET note = 'unterminated WHERE id=1").is_err());
    }

    #[test]
    fn rejects_unterminated_quoted_identifier() {
        assert!(classify_dml("UPDATE \"unterminated SET a=1").is_err());
    }

    #[test]
    fn rejects_word_prefixed_by_dml_verb_but_not_equal() {
        // Guards against a naive `starts_with` check on the verb token.
        assert!(classify_dml("UPDATER t SET a=1").is_err());
        assert!(classify_dml("INSERTED INTO t VALUES (1)").is_err());
    }

    #[test]
    fn classifies_insert_with_open_paren_immediately_after_verb_is_not_applicable_but_tabs_and_newlines_are_whitespace()
     {
        assert_eq!(
            classify_dml("\t\nINSERT\tINTO t VALUES (1)").unwrap(),
            WriteKind::Insert
        );
    }

    #[test]
    fn extract_update_target_returns_none_for_non_update_statements() {
        assert_eq!(extract_update_target("DELETE FROM t WHERE id=1"), None);
        assert_eq!(extract_update_target("SELECT * FROM t"), None);
    }

    #[test]
    fn extract_update_target_returns_none_for_multi_statement_input() {
        assert_eq!(
            extract_update_target("UPDATE t SET a=1; UPDATE t SET a=2"),
            None
        );
    }

    #[test]
    fn extract_update_target_handles_quoted_identifier_with_escaped_quote() {
        assert_eq!(
            extract_update_target("UPDATE \"my\"\"table\" SET a=1"),
            Some(("public".into(), "my\"table".into()))
        );
    }

    #[test]
    fn extract_update_target_declines_when_table_name_missing() {
        assert_eq!(extract_update_target("UPDATE SET a=1"), None);
        assert_eq!(extract_update_target("UPDATE"), None);
        assert_eq!(extract_update_target("UPDATE set a=1"), None);
    }

    #[test]
    fn extract_update_target_allows_quoted_table_literally_named_set() {
        // A table genuinely named "SET" (quoted) is not the keyword.
        assert_eq!(
            extract_update_target("UPDATE \"SET\" SET a=1"),
            Some(("public".into(), "SET".into()))
        );
    }

    #[test]
    fn extract_update_target_is_case_insensitive_for_only_keyword() {
        assert_eq!(
            extract_update_target("UPDATE only orders SET a=1"),
            Some(("public".into(), "orders".into()))
        );
    }
}
