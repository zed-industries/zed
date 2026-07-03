use anyhow::{Context as _, bail};

use crate::{
    ColumnInfo, FilterOp, RowDelete, RowInsert, RowKey, RowUpdate, SelectSpec, SortDirection,
    TableRef,
};

pub struct BuiltSelect {
    pub sql: String,
    pub params: Vec<String>,
}

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

pub fn build_select(
    table: &TableRef,
    columns: &[ColumnInfo],
    spec: &SelectSpec,
) -> anyhow::Result<BuiltSelect> {
    if spec.limit == 0 {
        bail!("page size must be greater than zero");
    }

    let select_list = columns
        .iter()
        .map(|column| format!("{}::text", quote_ident(&column.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let mut sql = format!(
        "SELECT {} FROM {}.{}",
        select_list,
        quote_ident(&table.schema),
        quote_ident(&table.name),
    );

    let mut params = Vec::new();
    let mut predicates = Vec::new();
    for filter in &spec.filters {
        let column = find_column(columns, &filter.column)?;
        let ident = quote_ident(&column.name);
        match filter.op {
            FilterOp::IsNull => predicates.push(format!("{ident} IS NULL")),
            FilterOp::Contains => {
                params.push(format!("%{}%", escape_like(&filter.value)));
                // `E'\\'` is a single backslash regardless of the server's
                // `standard_conforming_strings` setting; a bare `'\'` literal
                // breaks parsing when that GUC is off. Backslash is already the
                // default LIKE escape, so this preserves `escape_like`'s meaning.
                predicates.push(format!(
                    "{ident}::text ILIKE ${} ESCAPE E'\\\\'",
                    params.len()
                ));
            }
            FilterOp::Eq | FilterOp::NotEq | FilterOp::Gt | FilterOp::Lt => {
                let op = match filter.op {
                    FilterOp::Eq => "=",
                    FilterOp::NotEq => "<>",
                    FilterOp::Gt => ">",
                    FilterOp::Lt => "<",
                    FilterOp::Contains | FilterOp::IsNull => unreachable!(),
                };
                params.push(filter.value.clone());
                // Bind the parameter as text and let the server cast it to the
                // column's type (see `param_cast`). This keeps typed comparison
                // semantics for Gt/Lt while avoiding a bind-time `WrongType`
                // error on non-text columns.
                predicates.push(format!("{ident} {op} {}", param_cast(column, params.len())));
            }
        }
    }
    if !predicates.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }

    if let Some(sort) = &spec.sort {
        let column = find_column(columns, &sort.column)?;
        let direction = match sort.direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        sql.push_str(&format!(
            " ORDER BY {} {direction}",
            quote_ident(&column.name)
        ));
    }

    sql.push_str(&format!(" LIMIT {} OFFSET {}", spec.limit + 1, spec.offset));
    Ok(BuiltSelect { sql, params })
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

/// Escapes `\`, `%`, `_` for use in `ILIKE ... ESCAPE '\'` patterns.
pub fn escape_like(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(ch, '\\' | '%' | '_') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        EditCell, Filter, FilterOp, RowDelete, RowInsert, RowKey, RowUpdate, Sort, SortDirection,
    };

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
    fn build_select_plain_page() {
        let columns = vec![col("id", "int4"), col("name", "text")];
        let spec = SelectSpec {
            filters: vec![],
            sort: None,
            limit: 100,
            offset: 0,
        };
        let built = build_select(&users_table(), &columns, &spec).unwrap();
        assert_eq!(
            built.sql,
            "SELECT \"id\"::text, \"name\"::text FROM \"public\".\"users\" LIMIT 101 OFFSET 0"
        );
        assert!(built.params.is_empty());
    }

    #[test]
    fn build_select_with_filters_sort_offset() {
        let columns = vec![col("id", "int4"), col("name", "text")];
        let spec = SelectSpec {
            filters: vec![
                Filter {
                    column: "id".into(),
                    op: FilterOp::Gt,
                    value: "5".into(),
                },
                Filter {
                    column: "name".into(),
                    op: FilterOp::Contains,
                    value: "a%b".into(),
                },
                Filter {
                    column: "name".into(),
                    op: FilterOp::IsNull,
                    value: String::new(),
                },
            ],
            sort: Some(Sort {
                column: "name".into(),
                direction: SortDirection::Desc,
            }),
            limit: 50,
            offset: 100,
        };
        let built = build_select(&users_table(), &columns, &spec).unwrap();
        assert_eq!(
            built.sql,
            "SELECT \"id\"::text, \"name\"::text FROM \"public\".\"users\" \
             WHERE \"id\" > $1::text::\"pg_catalog\".\"int4\" \
             AND \"name\"::text ILIKE $2 ESCAPE E'\\\\' AND \"name\" IS NULL \
             ORDER BY \"name\" DESC LIMIT 51 OFFSET 100"
        );
        assert_eq!(built.params, vec!["5".to_string(), "%a\\%b%".to_string()]);
    }

    #[test]
    fn build_select_casts_filter_params_through_text_schema_qualified() {
        // Non-text columns must be compared by casting the text-bound parameter
        // server-side to the column's schema-qualified type, so binding the
        // parameter as a Rust String does not fail and typed comparison is kept.
        let mut price = col("price", "numeric");
        price.udt_schema = "pg_catalog".into();
        let mut email = col("email", "citext");
        email.udt_schema = "extensions".into();
        let columns = vec![col("id", "int4"), price, email];
        let spec = SelectSpec {
            filters: vec![
                Filter {
                    column: "id".into(),
                    op: FilterOp::Eq,
                    value: "5".into(),
                },
                Filter {
                    column: "price".into(),
                    op: FilterOp::Lt,
                    value: "9.99".into(),
                },
                Filter {
                    column: "email".into(),
                    op: FilterOp::NotEq,
                    value: "a@b.com".into(),
                },
            ],
            sort: None,
            limit: 10,
            offset: 0,
        };
        let built = build_select(&users_table(), &columns, &spec).unwrap();
        assert_eq!(
            built.sql,
            "SELECT \"id\"::text, \"price\"::text, \"email\"::text FROM \"public\".\"users\" \
             WHERE \"id\" = $1::text::\"pg_catalog\".\"int4\" \
             AND \"price\" < $2::text::\"pg_catalog\".\"numeric\" \
             AND \"email\" <> $3::text::\"extensions\".\"citext\" \
             LIMIT 11 OFFSET 0"
        );
        assert_eq!(built.params, vec!["5", "9.99", "a@b.com"]);
    }

    #[test]
    fn build_select_rejects_unknown_columns_and_zero_limit() {
        let columns = vec![col("id", "int4")];
        let bad_filter = SelectSpec {
            filters: vec![Filter {
                column: "nope".into(),
                op: FilterOp::Eq,
                value: "1".into(),
            }],
            sort: None,
            limit: 10,
            offset: 0,
        };
        assert!(build_select(&users_table(), &columns, &bad_filter).is_err());

        let bad_sort = SelectSpec {
            filters: vec![],
            sort: Some(Sort {
                column: "nope".into(),
                direction: SortDirection::Asc,
            }),
            limit: 10,
            offset: 0,
        };
        assert!(build_select(&users_table(), &columns, &bad_sort).is_err());

        let zero = SelectSpec {
            filters: vec![],
            sort: None,
            limit: 0,
            offset: 0,
        };
        assert!(build_select(&users_table(), &columns, &zero).is_err());
    }

    #[test]
    fn quote_ident_wraps_and_doubles_quotes() {
        assert_eq!(quote_ident("users"), "\"users\"");
        assert_eq!(quote_ident("weird\"name"), "\"weird\"\"name\"");
        assert_eq!(quote_ident("Mixed Case"), "\"Mixed Case\"");
    }

    #[test]
    fn escape_like_escapes_metacharacters() {
        assert_eq!(escape_like("100%"), "100\\%");
        assert_eq!(escape_like("a_b"), "a\\_b");
        assert_eq!(escape_like("back\\slash"), "back\\\\slash");
        assert_eq!(escape_like("plain"), "plain");
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
}
