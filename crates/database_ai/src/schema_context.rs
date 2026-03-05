use database_core::{describe_object_core, get_schema_core, list_objects_core};

/// Formats the full schema of a connection for inclusion in an AI context window.
///
/// Returns a markdown-formatted string with all tables, columns, and constraints.
pub fn format_schema_context(connection_name: &str) -> Result<String, String> {
    get_schema_core(connection_name, &[])
}

/// Formats the schema of specific tables for inclusion in an AI context window.
pub fn format_tables_context(connection_name: &str, table_names: &[&str]) -> Result<String, String> {
    let owned: Vec<String> = table_names.iter().map(|s| s.to_string()).collect();
    get_schema_core(connection_name, &owned)
}

/// Formats the schema of a single table for AI context.
pub fn format_table_context(connection_name: &str, table_name: &str) -> Result<String, String> {
    describe_object_core(table_name, connection_name)
}

/// Lists all database objects (tables and views) as a summary for AI context.
pub fn format_objects_summary(connection_name: &str) -> Result<String, String> {
    list_objects_core(connection_name, Some("all"))
}

/// Builds a prompt-ready schema section header for the given connection.
pub fn schema_section_header(connection_name: &str) -> String {
    format!("## Database schema for `{connection_name}`\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_section_header() {
        let header = schema_section_header("my_db");
        assert!(header.contains("my_db"));
        assert!(header.starts_with("## Database schema"));
    }

    #[test]
    fn test_format_schema_context_missing_connection() {
        let result = format_schema_context("nonexistent_connection_xyz");
        assert!(result.is_err());
    }

    #[test]
    fn test_format_table_context_missing_connection() {
        let result = format_table_context("nonexistent_connection_xyz", "users");
        assert!(result.is_err());
    }

    #[test]
    fn test_format_objects_summary_missing_connection() {
        let result = format_objects_summary("nonexistent_connection_xyz");
        assert!(result.is_err());
    }
}
