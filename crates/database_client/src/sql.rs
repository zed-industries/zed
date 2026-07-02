use anyhow::{Context as _, bail};

use crate::{ColumnInfo, FilterOp, SelectSpec, SortDirection, TableRef};

pub struct BuiltSelect {
    pub sql: String,
    pub params: Vec<String>,
}

pub fn build_select(
    table: &TableRef,
    columns: &[ColumnInfo],
    spec: &SelectSpec,
) -> anyhow::Result<BuiltSelect> {
    if spec.limit == 0 {
        bail!("page size must be greater than zero");
    }
    let find_column = |name: &str| -> anyhow::Result<&ColumnInfo> {
        columns
            .iter()
            .find(|column| column.name == name)
            .with_context(|| format!("unknown column: {name}"))
    };

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
        let column = find_column(&filter.column)?;
        let ident = quote_ident(&column.name);
        match filter.op {
            FilterOp::IsNull => predicates.push(format!("{ident} IS NULL")),
            FilterOp::Contains => {
                params.push(format!("%{}%", escape_like(&filter.value)));
                predicates.push(format!("{ident}::text ILIKE ${} ESCAPE '\\'", params.len()));
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
                predicates.push(format!(
                    "{ident} {op} ${}::{}",
                    params.len(),
                    quote_ident(&column.udt_name)
                ));
            }
        }
    }
    if !predicates.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }

    if let Some(sort) = &spec.sort {
        let column = find_column(&sort.column)?;
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

pub const LIST_DATABASES_SQL: &str =
    "SELECT datname FROM pg_database WHERE NOT datistemplate AND datallowconn ORDER BY datname";

pub const LIST_SCHEMAS_SQL: &str = "SELECT schema_name FROM information_schema.schemata \
     WHERE schema_name NOT IN ('pg_catalog', 'information_schema', 'pg_toast') \
     ORDER BY schema_name";

pub const LIST_TABLES_SQL: &str = "SELECT table_name, table_type FROM information_schema.tables \
     WHERE table_schema = $1 AND table_type IN ('BASE TABLE', 'VIEW') \
     ORDER BY table_name";

pub const COLUMNS_SQL: &str = "SELECT c.column_name, c.data_type, c.udt_name, \
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
    use crate::{Filter, FilterOp, Sort, SortDirection};

    fn col(name: &str, udt: &str) -> ColumnInfo {
        ColumnInfo {
            name: name.to_string(),
            data_type: udt.to_string(),
            udt_name: udt.to_string(),
            is_nullable: true,
            default: None,
            is_primary_key: false,
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
             WHERE \"id\" > $1::\"int4\" AND \"name\"::text ILIKE $2 ESCAPE '\\' AND \"name\" IS NULL \
             ORDER BY \"name\" DESC LIMIT 51 OFFSET 100"
        );
        assert_eq!(built.params, vec!["5".to_string(), "%a\\%b%".to_string()]);
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
}
