use database_client::{Filter, FilterOp, Sort, SortDirection, TableRef, quote_ident};

/// The source of rows for a [`QueryState`]: either a concrete table/view, or
/// custom SQL text typed by the user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryBase {
    /// Rows come from a table or view, addressed by schema-qualified name.
    Table(TableRef),
    /// Rows come from arbitrary SQL text entered by the user.
    Custom(String),
}

/// The full state needed to render a SELECT query: what to select from, plus
/// an optional overlay of filters, sort, and paging applied on top of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryState {
    /// What the query selects from.
    pub base: QueryBase,
    /// Predicates ANDed together into a WHERE clause.
    pub filters: Vec<Filter>,
    /// Optional single-column ORDER BY.
    pub sort: Option<Sort>,
    /// Row limit; always `Some` for a table-backed query, `None` for a fresh
    /// custom query that hasn't been paged yet.
    pub limit: Option<usize>,
    /// Row offset, applied together with `limit`.
    pub offset: usize,
}

impl QueryState {
    /// Creates a query over a table with an empty overlay and the given page size.
    pub fn for_table(table: TableRef, page_size: usize) -> Self {
        Self {
            base: QueryBase::Table(table),
            filters: Vec::new(),
            sort: None,
            limit: Some(page_size),
            offset: 0,
        }
    }

    /// Creates a query from custom SQL text with no overlay applied yet.
    pub fn for_custom(text: String) -> Self {
        Self {
            base: QueryBase::Custom(text),
            filters: Vec::new(),
            sort: None,
            limit: None,
            offset: 0,
        }
    }

    /// Whether this query's base is custom SQL text rather than a table.
    pub fn is_custom(&self) -> bool {
        matches!(self.base, QueryBase::Custom(_))
    }

    /// Whether any filter, sort, or paging is applied on top of the base query.
    pub fn has_overlay(&self) -> bool {
        !self.filters.is_empty() || self.sort.is_some() || self.limit.is_some() || self.offset > 0
    }
}

/// Escapes a string as a single-quoted SQL literal, doubling inner single quotes.
pub fn escape_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// Escapes `\`, `%`, and `_` for use inside a `LIKE`/`ILIKE` pattern.
fn escape_like_pattern(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// Renders a single filter as a WHERE-clause fragment (no leading/trailing whitespace).
fn filter_fragment(filter: &Filter) -> String {
    let column = quote_ident(&filter.column);
    match filter.op {
        FilterOp::Eq => format!("{column} = {}", escape_literal(&filter.value)),
        FilterOp::NotEq => format!("{column} <> {}", escape_literal(&filter.value)),
        FilterOp::Gt => format!("{column} > {}", escape_literal(&filter.value)),
        FilterOp::Lt => format!("{column} < {}", escape_literal(&filter.value)),
        FilterOp::Contains => format!(
            "{column}::text ILIKE {}",
            escape_literal(&format!("%{}%", escape_like_pattern(&filter.value)))
        ),
        FilterOp::IsNull => format!("{column} IS NULL"),
        FilterOp::IsNotNull => format!("{column} IS NOT NULL"),
    }
}

/// Renders the WHERE/ORDER BY/LIMIT overlay for a query state, with a leading
/// space before each clause that is present (empty string if there is no overlay).
fn overlay_clauses(state: &QueryState) -> String {
    let mut sql = String::new();
    if !state.filters.is_empty() {
        let predicates: Vec<String> = state.filters.iter().map(filter_fragment).collect();
        sql.push_str(" WHERE ");
        sql.push_str(&predicates.join(" AND "));
    }
    if let Some(sort) = &state.sort {
        let direction = match sort.direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        sql.push_str(&format!(
            " ORDER BY {} {}",
            quote_ident(&sort.column),
            direction
        ));
    }
    if let Some(limit) = state.limit {
        sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, state.offset));
    }
    sql
}

/// Deterministically renders a [`QueryState`] to SQL text.
///
/// A table-backed query always renders as `SELECT * FROM "schema"."table"`
/// plus the overlay. A custom query with no overlay renders verbatim as typed;
/// once a filter, sort, or paging is applied, the custom text is wrapped as a
/// subquery so the overlay can be layered on top of it.
pub fn render_sql(state: &QueryState) -> String {
    match &state.base {
        QueryBase::Table(table) => format!(
            "SELECT * FROM {}.{}{};",
            quote_ident(&table.schema),
            quote_ident(&table.name),
            overlay_clauses(state)
        ),
        QueryBase::Custom(text) => {
            if !state.has_overlay() {
                return text.clone();
            }
            let inner = text.trim().trim_end_matches(';').trim_end();
            format!(
                "SELECT * FROM (\n{inner}\n) AS zed_sub{};",
                overlay_clauses(state)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn orders() -> TableRef {
        TableRef {
            database: "shop".into(),
            schema: "public".into(),
            name: "orders".into(),
        }
    }

    #[test]
    fn renders_plain_table_query() {
        let state = QueryState::for_table(orders(), 100);
        assert_eq!(
            render_sql(&state),
            "SELECT * FROM \"public\".\"orders\" LIMIT 100 OFFSET 0;"
        );
    }

    #[test]
    fn renders_filters_sort_and_paging() {
        let mut state = QueryState::for_table(orders(), 50);
        state.filters = vec![
            Filter {
                column: "status".into(),
                op: FilterOp::Eq,
                value: "active".into(),
            },
            Filter {
                column: "total".into(),
                op: FilterOp::Gt,
                value: "10".into(),
            },
        ];
        state.sort = Some(Sort {
            column: "total".into(),
            direction: SortDirection::Desc,
        });
        state.offset = 100;
        assert_eq!(
            render_sql(&state),
            "SELECT * FROM \"public\".\"orders\" WHERE \"status\" = 'active' AND \"total\" > '10' \
             ORDER BY \"total\" DESC LIMIT 50 OFFSET 100;"
        );
    }

    #[test]
    fn renders_all_operators() {
        let column = "c".to_string();

        let eq = Filter {
            column: column.clone(),
            op: FilterOp::Eq,
            value: "v".into(),
        };
        assert_eq!(filter_fragment(&eq), "\"c\" = 'v'");

        let not_eq = Filter {
            column: column.clone(),
            op: FilterOp::NotEq,
            value: "v".into(),
        };
        assert_eq!(filter_fragment(&not_eq), "\"c\" <> 'v'");

        let gt = Filter {
            column: column.clone(),
            op: FilterOp::Gt,
            value: "v".into(),
        };
        assert_eq!(filter_fragment(&gt), "\"c\" > 'v'");

        let lt = Filter {
            column: column.clone(),
            op: FilterOp::Lt,
            value: "v".into(),
        };
        assert_eq!(filter_fragment(&lt), "\"c\" < 'v'");

        let contains = Filter {
            column: column.clone(),
            op: FilterOp::Contains,
            value: "v".into(),
        };
        assert_eq!(filter_fragment(&contains), "\"c\"::text ILIKE '%v%'");

        let is_null = Filter {
            column: column.clone(),
            op: FilterOp::IsNull,
            value: String::new(),
        };
        assert_eq!(filter_fragment(&is_null), "\"c\" IS NULL");

        let is_not_null = Filter {
            column,
            op: FilterOp::IsNotNull,
            value: String::new(),
        };
        assert_eq!(filter_fragment(&is_not_null), "\"c\" IS NOT NULL");
    }

    #[test]
    fn escapes_quotes_in_literals_and_idents() {
        let mut state = QueryState::for_table(orders(), 10);
        state.filters = vec![Filter {
            column: "we\"ird".into(),
            op: FilterOp::Eq,
            value: "it's".into(),
        }];
        let sql = render_sql(&state);
        assert!(sql.contains("\"we\"\"ird\" = 'it''s'"));
    }

    #[test]
    fn escapes_like_metacharacters_in_contains() {
        let mut state = QueryState::for_table(orders(), 10);
        state.filters = vec![Filter {
            column: "notes".into(),
            op: FilterOp::Contains,
            value: "50%_a\\b".into(),
        }];
        assert!(render_sql(&state).contains(r#""notes"::text ILIKE '%50\%\_a\\b%'"#));
    }

    #[test]
    fn custom_without_overlay_is_verbatim() {
        let state = QueryState::for_custom("SELECT 1;".into());
        assert_eq!(render_sql(&state), "SELECT 1;");
    }

    #[test]
    fn custom_with_overlay_wraps_subquery_and_strips_trailing_semicolon() {
        let mut state = QueryState::for_custom("SELECT o.id FROM orders o;  ".into());
        state.sort = Some(Sort {
            column: "id".into(),
            direction: SortDirection::Asc,
        });
        state.limit = Some(100);
        state.offset = 200;
        assert_eq!(
            render_sql(&state),
            "SELECT * FROM (\nSELECT o.id FROM orders o\n) AS zed_sub \
             ORDER BY \"id\" ASC LIMIT 100 OFFSET 200;"
        );
    }

    #[test]
    fn custom_with_only_paging_wraps() {
        let mut state = QueryState::for_custom("SELECT 1".into());
        state.limit = Some(100);
        assert_eq!(
            render_sql(&state),
            "SELECT * FROM (\nSELECT 1\n) AS zed_sub LIMIT 100 OFFSET 0;"
        );
    }
}
