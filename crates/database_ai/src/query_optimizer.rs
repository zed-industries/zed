use database_core::{explain_query_core, get_schema_core};

/// Builds a prompt for AI-assisted query optimization.
///
/// Includes the original query, its EXPLAIN plan, and schema context so the model
/// can suggest targeted improvements.
pub fn build_optimization_prompt(
    sql: &str,
    connection_name: &str,
) -> Result<String, String> {
    let explain_output = explain_query_core(sql, connection_name, false)?;
    let schema = get_schema_core(connection_name, &[]).unwrap_or_default();

    Ok(format!(
        "You are a database performance expert. Analyze this SQL query and suggest optimizations.\n\n\
        ## Query\n\n\
        ```sql\n{sql}\n```\n\n\
        ## Execution Plan\n\n\
        ```\n{explain_output}\n```\n\n\
        {schema_section}\
        ## Instructions\n\n\
        - Identify performance bottlenecks in the execution plan.\n\
        - Suggest specific index additions if sequential scans are detected.\n\
        - Propose query rewrites that would improve performance.\n\
        - Explain the expected impact of each suggestion.\n",
        schema_section = if schema.is_empty() {
            String::new()
        } else {
            format!("## Schema Context\n\n{schema}\n\n")
        }
    ))
}

/// Analyzes a query's structure and returns static optimization hints without
/// requiring a live database connection.
///
/// These hints are heuristic and should be combined with real EXPLAIN output
/// when a connection is available.
pub fn static_optimization_hints(sql: &str) -> Vec<String> {
    let upper = sql.to_uppercase();
    let mut hints = Vec::new();

    if upper.contains("SELECT *") {
        hints.push(
            "Avoid SELECT *: specify only the columns you need to reduce data transfer and \
            enable covering indexes."
                .to_string(),
        );
    }

    if upper.contains("LIKE '%") {
        hints.push(
            "Leading wildcard in LIKE '%...' prevents index usage. Consider full-text search \
            or restructuring the predicate."
                .to_string(),
        );
    }

    if upper.contains("NOT IN") {
        hints.push(
            "NOT IN with a subquery can be slow. Consider rewriting as LEFT JOIN / IS NULL \
            or NOT EXISTS."
                .to_string(),
        );
    }

    if upper.contains("ORDER BY") && !upper.contains("LIMIT") {
        hints.push(
            "ORDER BY without LIMIT sorts the entire result set. Add LIMIT if you only need \
            the top N rows."
                .to_string(),
        );
    }

    if upper.contains("DISTINCT") {
        hints.push(
            "DISTINCT forces a sort or hash deduplication pass. Ensure it is truly necessary \
            and consider whether a GROUP BY would be clearer."
                .to_string(),
        );
    }

    hints
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_hints_select_star() {
        let hints = static_optimization_hints("SELECT * FROM users");
        assert!(hints.iter().any(|h| h.contains("SELECT *")));
    }

    #[test]
    fn test_static_hints_leading_wildcard() {
        let hints = static_optimization_hints("SELECT id FROM t WHERE name LIKE '%foo'");
        assert!(hints.iter().any(|h| h.contains("LIKE")));
    }

    #[test]
    fn test_static_hints_not_in() {
        let hints = static_optimization_hints("SELECT id FROM t WHERE id NOT IN (SELECT id FROM s)");
        assert!(hints.iter().any(|h| h.contains("NOT IN")));
    }

    #[test]
    fn test_static_hints_order_without_limit() {
        let hints = static_optimization_hints("SELECT id FROM t ORDER BY created_at");
        assert!(hints.iter().any(|h| h.contains("ORDER BY")));
    }

    #[test]
    fn test_static_hints_clean_query() {
        let hints = static_optimization_hints("SELECT id, name FROM users WHERE id = 1 LIMIT 10");
        assert!(hints.is_empty());
    }

    #[test]
    fn test_build_optimization_prompt_missing_connection() {
        let result = build_optimization_prompt("SELECT 1", "nonexistent_xyz");
        assert!(result.is_err());
    }
}
