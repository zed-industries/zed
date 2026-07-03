use anyhow::{Context as _, bail};

use crate::{ColumnInfo, RowDelete, RowInsert, RowKey, RowUpdate, TableRef};

pub struct BuiltStatement {
    pub sql: String,
    pub params: Vec<String>,
}

/// Renders a `$index` parameter placeholder cast to the column's type.
///
/// Every parameter is bound as a Rust `String` (which `postgres-types` only
/// accepts for text-like types), so the value is cast through `text` to the
/// column's schema-qualified type server-side. The schema qualification lets
/// types outside `search_path` (e.g. an extension type in a dedicated schema)
/// still resolve.
fn param_cast(column: &ColumnInfo, index: usize) -> String {
    format!(
        "${index}::text::{}.{}",
        quote_ident(&column.udt_schema),
        quote_ident(&column.udt_name)
    )
}

/// Looks up a column by name, erroring if it does not exist.
fn find_column<'a>(columns: &'a [ColumnInfo], name: &str) -> anyhow::Result<&'a ColumnInfo> {
    columns
        .iter()
        .find(|column| column.name == name)
        .with_context(|| format!("unknown column: {name}"))
}

/// Builds the `WHERE` clause matching a row by its `RowKey`, pushing one typed
/// parameter per key column. Errors on an empty key or an unknown column.
fn build_key_predicate(
    columns: &[ColumnInfo],
    key: &RowKey,
    params: &mut Vec<String>,
) -> anyhow::Result<String> {
    if key.columns.is_empty() {
        bail!("row key must not be empty");
    }
    // Guard against a malformed key whose columns and values differ in length:
    // `zip` below would silently stop at the shorter, producing a WHERE clause
    // over only a prefix of the primary key, which could match/modify the wrong
    // rows on an UPDATE/DELETE. RowKey should be equal-length by construction,
    // so this is defensive.
    if key.columns.len() != key.values.len() {
        bail!("row key columns and values length mismatch");
    }
    let mut predicates = Vec::with_capacity(key.columns.len());
    for (name, value) in key.columns.iter().zip(&key.values) {
        let column = find_column(columns, name)?;
        let Some(value) = value else {
            bail!("row key value for column {name} must not be null");
        };
        params.push(value.clone());
        predicates.push(format!(
            "{} = {}",
            quote_ident(&column.name),
            param_cast(column, params.len())
        ));
    }
    Ok(predicates.join(" AND "))
}

/// Builds an `UPDATE` statement. The `SET` clause covers the columns in
/// `update.set` (rejecting primary-key columns and an empty set); the `WHERE`
/// clause matches the row by `update.key`. `SET` value parameters are ordered
/// first, followed by the `WHERE` key parameters.
pub fn build_update(
    table: &TableRef,
    columns: &[ColumnInfo],
    update: &RowUpdate,
) -> anyhow::Result<BuiltStatement> {
    if update.set.is_empty() {
        bail!("update must set at least one column");
    }
    let mut params = Vec::new();
    let mut assignments = Vec::with_capacity(update.set.len());
    for (name, cell) in &update.set {
        let column = find_column(columns, name)?;
        if column.is_primary_key {
            bail!("cannot update primary key column: {name}");
        }
        assignments.push(format!(
            "{} = {}",
            quote_ident(&column.name),
            render_cell(column, cell, &mut params)
        ));
    }
    let where_clause = build_key_predicate(columns, &update.key, &mut params)?;
    let sql = format!(
        "UPDATE {}.{} SET {} WHERE {}",
        quote_ident(&table.schema),
        quote_ident(&table.name),
        assignments.join(", "),
        where_clause,
    );
    Ok(BuiltStatement { sql, params })
}

/// Builds an `INSERT` statement covering only the columns in `insert.values`.
///
/// When `insert.values` is empty (the user added a row and left every cell
/// unset), this emits `INSERT INTO ... DEFAULT VALUES` so every column takes its
/// default. Columns without a default (e.g. a `NOT NULL` column) still error at
/// runtime and roll the batch back, surfacing the Postgres error to the user.
pub fn build_insert(
    table: &TableRef,
    columns: &[ColumnInfo],
    insert: &RowInsert,
) -> anyhow::Result<BuiltStatement> {
    if insert.values.is_empty() {
        let sql = format!(
            "INSERT INTO {}.{} DEFAULT VALUES",
            quote_ident(&table.schema),
            quote_ident(&table.name),
        );
        return Ok(BuiltStatement {
            sql,
            params: vec![],
        });
    }
    let mut params = Vec::new();
    let mut idents = Vec::with_capacity(insert.values.len());
    let mut values = Vec::with_capacity(insert.values.len());
    for (name, cell) in &insert.values {
        let column = find_column(columns, name)?;
        idents.push(quote_ident(&column.name));
        values.push(render_cell(column, cell, &mut params));
    }
    let sql = format!(
        "INSERT INTO {}.{} ({}) VALUES ({})",
        quote_ident(&table.schema),
        quote_ident(&table.name),
        idents.join(", "),
        values.join(", "),
    );
    Ok(BuiltStatement { sql, params })
}

/// Builds a `DELETE` statement matching the row by `delete.key` (rejecting an
/// empty key).
pub fn build_delete(
    table: &TableRef,
    columns: &[ColumnInfo],
    delete: &RowDelete,
) -> anyhow::Result<BuiltStatement> {
    let mut params = Vec::new();
    let where_clause = build_key_predicate(columns, &delete.key, &mut params)?;
    let sql = format!(
        "DELETE FROM {}.{} WHERE {}",
        quote_ident(&table.schema),
        quote_ident(&table.name),
        where_clause,
    );
    Ok(BuiltStatement { sql, params })
}

/// Renders an `EditCell`: `Value` pushes a text parameter cast to the column's
/// type and returns the placeholder; `Null` returns a literal `NULL`.
fn render_cell(column: &ColumnInfo, cell: &crate::EditCell, params: &mut Vec<String>) -> String {
    match cell {
        crate::EditCell::Value(value) => {
            params.push(value.clone());
            param_cast(column, params.len())
        }
        crate::EditCell::Null => "NULL".to_string(),
    }
}

/// Returns true if `sql` contains a top-level `RETURNING` keyword (a whole
/// word, case-insensitive, outside string/quoted-identifier/dollar-quoted
/// spans and comments). Used to decide whether [`with_returning`] needs to
/// append a clause.
///
/// Scans by `char_indices()` (never slices at a byte offset that wasn't
/// yielded as a char boundary by the iterator), so it is safe on any UTF-8
/// input, including multi-byte characters inside string literals.
///
/// Recognizes and skips over: `'...'` string literals (with `''`
/// escaping), `"..."` quoted identifiers, `$$...$$` and `$tag$...$tag$`
/// dollar-quoted strings, `-- ...` line comments, and `/* ... */` block
/// comments. Does not attempt to parse subquery structure, so a
/// `RETURNING` that is only valid inside a nested subquery (rare, and
/// generally not valid Postgres syntax there anyway) is out of scope.
fn has_top_level_returning(sql: &str) -> bool {
    const KEYWORD: &str = "RETURNING";
    let chars: Vec<(usize, char)> = sql.char_indices().collect();
    let mut position = 0;
    while position < chars.len() {
        let (_, ch) = chars[position];
        match ch {
            '\'' => {
                position += 1;
                position = skip_past_delimited(&chars, position, '\'');
            }
            '"' => {
                position += 1;
                position = skip_past_delimited(&chars, position, '"');
            }
            '$' => {
                if let Some(next_position) = skip_dollar_quoted(&chars, position) {
                    position = next_position;
                } else {
                    position += 1;
                }
            }
            '-' if chars.get(position + 1).is_some_and(|&(_, c)| c == '-') => {
                position += 2;
                while position < chars.len() && chars[position].1 != '\n' {
                    position += 1;
                }
            }
            '/' if chars.get(position + 1).is_some_and(|&(_, c)| c == '*') => {
                position += 2;
                while position < chars.len()
                    && !(chars[position].1 == '*'
                        && chars.get(position + 1).is_some_and(|&(_, c)| c == '/'))
                {
                    position += 1;
                }
                // Skip the closing `*/` itself, if present.
                position = (position + 2).min(chars.len());
            }
            _ => {
                let keyword_len = KEYWORD.chars().count();
                let matches_keyword = position + keyword_len <= chars.len()
                    && chars[position..position + keyword_len]
                        .iter()
                        .map(|&(_, c)| c)
                        .zip(KEYWORD.chars())
                        .all(|(a, b)| a.eq_ignore_ascii_case(&b));
                let preceded_by_word_char = position > 0 && is_word_char(chars[position - 1].1);
                let followed_by_word_char = chars
                    .get(position + keyword_len)
                    .is_some_and(|&(_, c)| is_word_char(c));
                if matches_keyword && !preceded_by_word_char && !followed_by_word_char {
                    return true;
                }
                position += 1;
            }
        }
    }
    false
}

/// Returns true if `ch` counts as part of a SQL identifier/keyword for the
/// purposes of word-boundary checks (so `RETURNINGX`/`xRETURNING` are not
/// mistaken for the keyword).
fn is_word_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// Advances `position` past a `'...'`/`"..."`-style delimited span whose
/// opening delimiter has already been consumed, honoring the SQL convention
/// that a doubled delimiter (`''`/`""`) inside the span is an escaped
/// literal delimiter rather than the closing one. Returns the index just
/// past the closing delimiter, or `chars.len()` if the span is unterminated.
fn skip_past_delimited(chars: &[(usize, char)], mut position: usize, delimiter: char) -> usize {
    while position < chars.len() {
        if chars[position].1 == delimiter {
            if chars
                .get(position + 1)
                .is_some_and(|&(_, c)| c == delimiter)
            {
                position += 2;
                continue;
            }
            return position + 1;
        }
        position += 1;
    }
    position
}

/// If a dollar-quoted string (`$$...$$` or `$tag$...$tag$`) starts at
/// `position` (which must point at a `$`), returns the index just past its
/// closing delimiter (or past the end of input, if unterminated). Returns
/// `None` if `position` is not the start of a valid dollar-quote opening
/// delimiter (e.g. a bare `$` used as an operator, or `$1` parameter
/// placeholder syntax, which is not a dollar-quote).
fn skip_dollar_quoted(chars: &[(usize, char)], position: usize) -> Option<usize> {
    let mut tag_end = position + 1;
    while chars
        .get(tag_end)
        .is_some_and(|&(_, c)| c.is_ascii_alphabetic() || c == '_')
    {
        tag_end += 1;
    }
    if chars.get(tag_end).is_none_or(|&(_, c)| c != '$') {
        return None;
    }
    let opening_end = tag_end + 1;
    let tag: Vec<char> = chars[position + 1..tag_end]
        .iter()
        .map(|&(_, c)| c)
        .collect();
    let mut search_position = opening_end;
    while search_position < chars.len() {
        if chars[search_position].1 == '$' {
            let candidate_end = search_position + 1 + tag.len() + 1;
            if candidate_end <= chars.len() {
                let candidate: Vec<char> = chars[search_position..candidate_end]
                    .iter()
                    .map(|&(_, c)| c)
                    .collect();
                let mut expected = vec!['$'];
                expected.extend(&tag);
                expected.push('$');
                if candidate == expected {
                    return Some(candidate_end);
                }
            }
        }
        search_position += 1;
    }
    Some(chars.len())
}

/// Applies the deterministic transform used by both `preview_write` and
/// `commit_write` so the previewed and committed statements always match: if
/// `sql` already has a top-level `RETURNING` clause, it is used as-is;
/// otherwise ` RETURNING *` is appended.
pub fn with_returning(sql: &str) -> String {
    if has_top_level_returning(sql) {
        sql.to_string()
    } else {
        format!("{sql} RETURNING *")
    }
}

/// Escapes a string as a single-quoted SQL literal, doubling inner single
/// quotes.
///
/// Correctness relies on the session having `standard_conforming_strings=on`,
/// which `postgres::session_options` pins for every session this crate opens;
/// without it, backslashes in `value` would be reinterpreted as escapes by the
/// server instead of being taken literally.
fn escape_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// Renders a value as a literal cast to the column's type, for embedding in a
/// statement run through the simple query protocol (which has no bind
/// parameters). Mirrors [`param_cast`]'s `::text::type` cast but with the
/// value inlined via [`escape_literal`] instead of a `$n` placeholder.
fn literal_cast(column: &ColumnInfo, value: &str) -> String {
    format!(
        "{}::text::{}.{}",
        escape_literal(value),
        quote_ident(&column.udt_schema),
        quote_ident(&column.udt_name)
    )
}

/// Builds a `SELECT * FROM "schema"."table" WHERE (pk1, pk2, ...) IN
/// ((v11, v12, ...), ...)` statement fetching the before-image of rows by
/// their primary-key values, casting each value through the PK column's type
/// the same way [`param_cast`] does for edits. `pk_rows` is one tuple of PK
/// values per row, in `pk_columns` order. Returns `None` if `pk_rows` is empty
/// (nothing to fetch).
///
/// Unlike the other statement builders in this module, the returned SQL has
/// every value inlined as an escaped literal (via [`literal_cast`]) rather
/// than a bind parameter: the before-image fetch runs through the simple
/// query protocol (see `postgres::fetch_update_before_image`) so its result
/// columns — of whatever type the target table happens to have — come back as
/// text, matching how every other arbitrary-shaped result set (`run_query`,
/// write previews) is decoded in this crate. The extended protocol used
/// elsewhere in this module always has a compile-time-known Rust type for
/// each result column, which does not hold for `SELECT *` on a user table.
pub fn build_pk_in_select(
    table: &TableRef,
    pk_columns: &[&ColumnInfo],
    pk_rows: &[Vec<Option<String>>],
) -> anyhow::Result<Option<String>> {
    if pk_columns.is_empty() || pk_rows.is_empty() {
        return Ok(None);
    }
    let mut tuples = Vec::with_capacity(pk_rows.len());
    for pk_values in pk_rows {
        if pk_values.len() != pk_columns.len() {
            bail!("primary key row width mismatch");
        }
        let mut literals = Vec::with_capacity(pk_columns.len());
        for (column, value) in pk_columns.iter().zip(pk_values) {
            let Some(value) = value else {
                // A NULL primary-key value cannot match any row via `=`, so
                // this row can never be found by the before-fetch; the caller's
                // row-count check will then discard `before` and note why.
                bail!("primary key value must not be null");
            };
            literals.push(literal_cast(column, value));
        }
        tuples.push(format!("({})", literals.join(", ")));
    }
    let pk_idents: Vec<String> = pk_columns
        .iter()
        .map(|column| quote_ident(&column.name))
        .collect();
    let sql = format!(
        "SELECT * FROM {}.{} WHERE ({}) IN ({})",
        quote_ident(&table.schema),
        quote_ident(&table.name),
        pk_idents.join(", "),
        tuples.join(", "),
    );
    Ok(Some(sql))
}

pub const LIST_DATABASES_SQL: &str =
    "SELECT datname FROM pg_database WHERE NOT datistemplate AND datallowconn ORDER BY datname";

pub const LIST_SCHEMAS_SQL: &str = "SELECT schema_name FROM information_schema.schemata \
     WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast') \
     ORDER BY schema_name";

pub const LIST_TABLES_SQL: &str = "SELECT table_name, table_type FROM information_schema.tables \
     WHERE table_schema = $1 AND table_type IN ('BASE TABLE', 'VIEW') \
     ORDER BY table_name";

pub const COLUMNS_SQL: &str = "SELECT c.column_name, c.data_type, c.udt_name, c.udt_schema, \
     c.is_nullable = 'YES' AS is_nullable, c.column_default, \
     EXISTS (SELECT 1 FROM information_schema.table_constraints tc \
       JOIN information_schema.key_column_usage kcu \
         ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
       WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_schema = c.table_schema \
         AND tc.table_name = c.table_name AND kcu.column_name = c.column_name) AS is_primary_key \
     FROM information_schema.columns c \
     WHERE c.table_schema = $1 AND c.table_name = $2 \
     ORDER BY c.ordinal_position";

pub const FOREIGN_KEYS_SQL: &str = "SELECT kcu.column_name, ccu.table_schema, ccu.table_name, ccu.column_name \
     FROM information_schema.table_constraints tc \
     JOIN information_schema.key_column_usage kcu \
       ON tc.constraint_name = kcu.constraint_name AND tc.table_schema = kcu.table_schema \
     JOIN information_schema.constraint_column_usage ccu \
       ON tc.constraint_name = ccu.constraint_name AND tc.table_schema = ccu.table_schema \
     WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema = $1 AND tc.table_name = $2 \
     ORDER BY kcu.ordinal_position";

pub const INDEXES_SQL: &str = "SELECT indexname, indexdef FROM pg_indexes \
     WHERE schemaname = $1 AND tablename = $2 ORDER BY indexname";

/// Quotes a PostgreSQL identifier: wraps in double quotes, doubles inner quotes.
pub fn quote_ident(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 2);
    out.push('"');
    for ch in ident.chars() {
        if ch == '"' {
            out.push('"');
        }
        out.push(ch);
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EditCell, RowDelete, RowInsert, RowKey, RowUpdate};

    fn col(name: &str, udt: &str) -> ColumnInfo {
        ColumnInfo {
            name: name.to_string(),
            data_type: udt.to_string(),
            udt_name: udt.to_string(),
            udt_schema: "pg_catalog".to_string(),
            is_nullable: true,
            default: None,
            is_primary_key: false,
        }
    }

    fn pk_col(name: &str, udt: &str) -> ColumnInfo {
        ColumnInfo {
            is_primary_key: true,
            ..col(name, udt)
        }
    }

    fn users_table() -> TableRef {
        TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        }
    }

    #[test]
    fn quote_ident_wraps_and_doubles_quotes() {
        assert_eq!(quote_ident("users"), "\"users\"");
        assert_eq!(quote_ident("weird\"name"), "\"weird\"\"name\"");
        assert_eq!(quote_ident("Mixed Case"), "\"Mixed Case\"");
    }

    fn edit_columns() -> Vec<ColumnInfo> {
        vec![
            pk_col("id", "int4"),
            col("name", "text"),
            col("age", "int4"),
        ]
    }

    #[test]
    fn build_update_sets_and_where() {
        let columns = edit_columns();
        let update = RowUpdate {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("7".into())],
            },
            set: vec![
                ("name".into(), EditCell::Value("Ann".into())),
                ("age".into(), EditCell::Null),
            ],
        };
        let built = build_update(&users_table(), &columns, &update).unwrap();
        assert_eq!(
            built.sql,
            "UPDATE \"public\".\"users\" SET \"name\" = $1::text::\"pg_catalog\".\"text\", \"age\" = NULL WHERE \"id\" = $2::text::\"pg_catalog\".\"int4\""
        );
        assert_eq!(built.params, vec!["Ann".to_string(), "7".to_string()]);
    }

    #[test]
    fn build_insert_only_given_columns() {
        let columns = edit_columns();
        let insert = RowInsert {
            values: vec![
                ("name".into(), EditCell::Value("Bob".into())),
                ("id".into(), EditCell::Value("9".into())),
            ],
        };
        let built = build_insert(&users_table(), &columns, &insert).unwrap();
        assert_eq!(
            built.sql,
            "INSERT INTO \"public\".\"users\" (\"name\", \"id\") VALUES ($1::text::\"pg_catalog\".\"text\", $2::text::\"pg_catalog\".\"int4\")"
        );
        assert_eq!(built.params, vec!["Bob".to_string(), "9".to_string()]);
    }

    #[test]
    fn build_delete_by_pk() {
        let columns = edit_columns();
        let delete = RowDelete {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("3".into())],
            },
        };
        let built = build_delete(&users_table(), &columns, &delete).unwrap();
        assert_eq!(
            built.sql,
            "DELETE FROM \"public\".\"users\" WHERE \"id\" = $1::text::\"pg_catalog\".\"int4\""
        );
        assert_eq!(built.params, vec!["3".to_string()]);
    }

    #[test]
    fn build_update_rejects_pk_in_set() {
        let columns = edit_columns();
        let update = RowUpdate {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("7".into())],
            },
            set: vec![("id".into(), EditCell::Value("8".into()))],
        };
        assert!(build_update(&users_table(), &columns, &update).is_err());
    }

    #[test]
    fn build_update_rejects_empty_set() {
        let columns = edit_columns();
        let update = RowUpdate {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("7".into())],
            },
            set: vec![],
        };
        assert!(build_update(&users_table(), &columns, &update).is_err());
    }

    #[test]
    fn build_insert_empty_uses_default_values() {
        let columns = edit_columns();
        let insert = RowInsert { values: vec![] };
        let built = build_insert(&users_table(), &columns, &insert).unwrap();
        assert_eq!(built.sql, "INSERT INTO \"public\".\"users\" DEFAULT VALUES");
        assert!(built.params.is_empty());
    }

    #[test]
    fn build_key_predicate_rejects_length_mismatch() {
        // A key whose columns and values differ in length must error rather than
        // silently building a WHERE clause over only a prefix of the key.
        let columns = edit_columns();
        let delete = RowDelete {
            key: RowKey {
                columns: vec!["id".into(), "name".into()],
                values: vec![Some("3".into())],
            },
        };
        assert!(build_delete(&users_table(), &columns, &delete).is_err());

        let update = RowUpdate {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("7".into()), Some("extra".into())],
            },
            set: vec![("name".into(), EditCell::Value("Ann".into()))],
        };
        assert!(build_update(&users_table(), &columns, &update).is_err());
    }

    #[test]
    fn build_update_rejects_unknown_column() {
        let columns = edit_columns();
        let update = RowUpdate {
            key: RowKey {
                columns: vec!["id".into()],
                values: vec![Some("7".into())],
            },
            set: vec![("nope".into(), EditCell::Value("x".into()))],
        };
        assert!(build_update(&users_table(), &columns, &update).is_err());
    }

    #[test]
    fn with_returning_appends_when_absent() {
        assert_eq!(
            with_returning("DELETE FROM t WHERE id=1"),
            "DELETE FROM t WHERE id=1 RETURNING *"
        );
    }

    #[test]
    fn with_returning_respects_existing_clause() {
        assert_eq!(
            with_returning("INSERT INTO t VALUES (1) RETURNING id"),
            "INSERT INTO t VALUES (1) RETURNING id"
        );
    }

    #[test]
    fn with_returning_is_case_insensitive() {
        assert_eq!(
            with_returning("update t set a=1 where id=1 returning a"),
            "update t set a=1 where id=1 returning a"
        );
    }

    #[test]
    fn with_returning_does_not_match_word_containing_returning() {
        // A column or alias literally named e.g. `xreturning` must not be
        // mistaken for the keyword.
        assert_eq!(
            with_returning("UPDATE t SET xreturning = 1 WHERE id=1"),
            "UPDATE t SET xreturning = 1 WHERE id=1 RETURNING *"
        );
    }

    #[test]
    fn with_returning_ignores_returning_inside_string_literal() {
        assert_eq!(
            with_returning("UPDATE t SET note = 'see RETURNING docs' WHERE id=1"),
            "UPDATE t SET note = 'see RETURNING docs' WHERE id=1 RETURNING *"
        );
    }

    #[test]
    fn with_returning_does_not_panic_on_non_ascii_literal() {
        // A UTF-8 multi-byte character (in a string literal value) must not
        // land the byte-index scanner mid-codepoint and panic on a
        // non-char-boundary slice.
        assert_eq!(
            with_returning("UPDATE t SET name = 'café' WHERE id = 1"),
            "UPDATE t SET name = 'café' WHERE id = 1 RETURNING *"
        );
    }

    #[test]
    fn with_returning_does_not_panic_on_cyrillic_literal() {
        assert_eq!(
            with_returning("INSERT INTO t (name) VALUES ('привет')"),
            "INSERT INTO t (name) VALUES ('привет') RETURNING *"
        );
    }

    #[test]
    fn with_returning_does_not_panic_on_emoji_literal() {
        assert_eq!(
            with_returning("UPDATE t SET note = '🎉party' WHERE id = 1"),
            "UPDATE t SET note = '🎉party' WHERE id = 1 RETURNING *"
        );
    }

    #[test]
    fn has_top_level_returning_true_for_insert_returning_id() {
        assert!(has_top_level_returning(
            "INSERT INTO t VALUES (1) RETURNING id"
        ));
    }

    #[test]
    fn has_top_level_returning_false_for_returning_inside_literal() {
        assert!(!has_top_level_returning(
            "UPDATE t SET note='say RETURNING now' WHERE id=1"
        ));
    }

    #[test]
    fn has_top_level_returning_false_for_dollar_quoted_string() {
        assert!(!has_top_level_returning(
            "INSERT INTO t VALUES ($$has RETURNING inside$$)"
        ));
    }

    #[test]
    fn has_top_level_returning_false_for_tagged_dollar_quoted_string() {
        assert!(!has_top_level_returning(
            "INSERT INTO t VALUES ($tag$has RETURNING inside$tag$)"
        ));
    }

    #[test]
    fn has_top_level_returning_false_for_leading_block_comment() {
        assert!(!has_top_level_returning("/* RETURNING */ DELETE FROM t"));
    }

    #[test]
    fn build_pk_in_select_single_column() {
        let columns = edit_columns();
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        let rows = vec![vec![Some("1".into())], vec![Some("2".into())]];
        let sql = build_pk_in_select(&users_table(), &pk, &rows)
            .unwrap()
            .unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM \"public\".\"users\" WHERE (\"id\") IN (('1'::text::\"pg_catalog\".\"int4\"), ('2'::text::\"pg_catalog\".\"int4\"))"
        );
    }

    #[test]
    fn build_pk_in_select_composite_key() {
        let columns = [pk_col("a", "int4"), pk_col("b", "text"), col("c", "text")];
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        let rows = vec![vec![Some("1".into()), Some("x".into())]];
        let sql = build_pk_in_select(&users_table(), &pk, &rows)
            .unwrap()
            .unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM \"public\".\"users\" WHERE (\"a\", \"b\") IN (('1'::text::\"pg_catalog\".\"int4\", 'x'::text::\"pg_catalog\".\"text\"))"
        );
    }

    #[test]
    fn build_pk_in_select_escapes_single_quotes_in_pk_value() {
        let columns = edit_columns();
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        let rows = vec![vec![Some("o'brien".into())]];
        let sql = build_pk_in_select(&users_table(), &pk, &rows)
            .unwrap()
            .unwrap();
        assert_eq!(
            sql,
            "SELECT * FROM \"public\".\"users\" WHERE (\"id\") IN (('o''brien'::text::\"pg_catalog\".\"int4\"))"
        );
    }

    #[test]
    fn build_pk_in_select_empty_rows_returns_none() {
        let columns = edit_columns();
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        assert!(
            build_pk_in_select(&users_table(), &pk, &[])
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn build_pk_in_select_rejects_width_mismatch() {
        let columns = edit_columns();
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        let rows = vec![vec![Some("1".into()), Some("extra".into())]];
        assert!(build_pk_in_select(&users_table(), &pk, &rows).is_err());
    }

    #[test]
    fn build_pk_in_select_rejects_null_pk_value() {
        let columns = edit_columns();
        let pk = columns
            .iter()
            .filter(|c| c.is_primary_key)
            .collect::<Vec<_>>();
        let rows = vec![vec![None]];
        assert!(build_pk_in_select(&users_table(), &pk, &rows).is_err());
    }
}
