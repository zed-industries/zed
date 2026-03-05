use database_core::get_schema_core;

/// Builds a prompt for generating SQL from a natural language description.
///
/// The returned string is ready to be sent to a language model as the user message.
/// It includes the schema context for the specified tables so the model can generate
/// accurate, schema-aware queries.
pub fn build_nl_to_sql_prompt(
    natural_language: &str,
    connection_name: &str,
    table_hints: &[&str],
) -> Result<String, String> {
    let owned: Vec<String> = table_hints.iter().map(|s| s.to_string()).collect();
    let schema = get_schema_core(connection_name, &owned)?;

    Ok(format!(
        "You are a SQL expert. Generate a SQL query for the following request.\n\n\
        ## Database Schema\n\n\
        {schema}\n\n\
        ## Request\n\n\
        {natural_language}\n\n\
        ## Instructions\n\n\
        - Write a single valid SQL query that answers the request.\n\
        - Use proper JOIN syntax when multiple tables are needed.\n\
        - Include a LIMIT clause when the result set could be large.\n\
        - Return only the SQL query, no explanation.\n"
    ))
}

/// Builds a prompt for generating a SQL query when no schema is available locally.
///
/// Used when the connection is not yet established or schema introspection failed.
pub fn build_schemaless_nl_to_sql_prompt(natural_language: &str) -> String {
    format!(
        "You are a SQL expert. Generate a SQL query for the following request.\n\n\
        Note: No database schema is available. Write a generic SQL query based on the request.\n\n\
        ## Request\n\n\
        {natural_language}\n\n\
        ## Instructions\n\n\
        - Write a single valid SQL query.\n\
        - Use common table and column naming conventions.\n\
        - Return only the SQL query, no explanation.\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_nl_to_sql_prompt_missing_connection() {
        let result = build_nl_to_sql_prompt("find all users", "nonexistent_xyz", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_schemaless_prompt_contains_request() {
        let prompt = build_schemaless_nl_to_sql_prompt("find active users");
        assert!(prompt.contains("find active users"));
        assert!(prompt.contains("SQL query"));
    }
}
