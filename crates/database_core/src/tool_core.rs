use std::fmt::Write;

use crate::connection::DatabaseType;
use crate::connection_registry::{get_connection, list_connections};
use crate::export::{generate_ddl_from_schema, generate_table_ddl};

pub fn format_connection_not_found(connection_name: &str) -> String {
    let available = list_connections();
    if available.is_empty() {
        format!(
            "Connection '{}' not found. No database connections are available.\n\n\
            **To connect to a database:**\n\
            1. Open the Database panel (View > Database Panel)\n\
            2. Click the \"+\" button to add a new connection\n\
            3. Enter your connection details and click Connect",
            connection_name
        )
    } else {
        let entries: Vec<String> = available
            .iter()
            .map(|(name, config)| format!("- **{}** ({})", name, config.database_type))
            .collect();
        format!(
            "Connection '{}' not found.\n\n**Available connections:**\n{}",
            connection_name,
            entries.join("\n")
        )
    }
}

pub fn build_explain_sql(sql: &str, analyze: bool, database_type: &DatabaseType) -> String {
    let sql = sql.trim().trim_end_matches(';');

    match database_type {
        DatabaseType::Sqlite => {
            format!("EXPLAIN QUERY PLAN {sql}")
        }
        DatabaseType::PostgreSql => {
            if analyze {
                format!("EXPLAIN ANALYZE {sql}")
            } else {
                format!("EXPLAIN {sql}")
            }
        }
        DatabaseType::MySql => {
            if analyze {
                format!("EXPLAIN ANALYZE {sql}")
            } else {
                format!("EXPLAIN {sql}")
            }
        }
    }
}

pub fn execute_query_core(sql: &str, connection_name: &str, limit: usize) -> Result<String, String> {
    let (connection, _config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    let limit = if limit == 0 { 100 } else { limit };

    let limited_sql = if !sql.to_uppercase().contains("LIMIT") {
        format!("{} LIMIT {}", sql.trim_end_matches(';'), limit)
    } else {
        sql.to_string()
    };

    let result = connection.execute_query(&limited_sql).map_err(|error| {
        let mut message = format!("Query execution failed: {error}\n\n");
        message.push_str("**SQL:**\n```sql\n");
        message.push_str(&limited_sql);
        message.push_str("\n```\n");

        if let Some(ref schema) = schema {
            message.push_str("\n**Available tables:** ");
            let names: Vec<&str> = schema.tables.iter().map(|t| t.name.as_str()).collect();
            message.push_str(&names.join(", "));
        }

        message
    })?;

    let mut output = String::new();

    if result.columns.is_empty() {
        if let Some(affected) = result.affected_rows {
            write!(output, "{affected} row(s) affected.").ok();
        } else {
            write!(output, "Query executed successfully (no results).").ok();
        }
        return Ok(output);
    }

    writeln!(output, "| {} |", result.columns.join(" | ")).ok();

    let separator: Vec<&str> = result.columns.iter().map(|_| "---").collect();
    writeln!(output, "| {} |", separator.join(" | ")).ok();

    for row in &result.rows {
        let cells: Vec<String> = row
            .iter()
            .map(|cell| {
                let value = cell.to_string();
                value.replace('|', "\\|").replace('\n', " ")
            })
            .collect();
        writeln!(output, "| {} |", cells.join(" | ")).ok();
    }

    writeln!(output).ok();
    write!(
        output,
        "{} row(s) returned in {:.1}ms",
        result.rows.len(),
        result.execution_time.as_secs_f64() * 1000.0
    )
    .ok();

    if let Some(total) = result.total_row_count {
        if total > result.rows.len() as u64 {
            write!(output, " (total: {total} rows)").ok();
        }
    }

    Ok(output)
}

pub fn describe_object_core(object_name: &str, connection_name: &str) -> Result<String, String> {
    let (_connection, _config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    let schema = schema.ok_or_else(|| {
        format!(
            "Schema not available for connection '{}'. The connection may still be loading.",
            connection_name
        )
    })?;

    let table = schema
        .tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(object_name))
        .ok_or_else(|| {
            let table_names: Vec<&str> =
                schema.tables.iter().map(|table| table.name.as_str()).collect();
            format!(
                "Object '{}' not found. Available objects: {}",
                object_name,
                table_names.join(", ")
            )
        })?;

    let mut output = String::new();

    writeln!(output, "# {}", table.name).ok();
    writeln!(output).ok();

    if let Some(row_count) = table.row_count {
        writeln!(output, "Row count: {row_count}").ok();
    }
    if table.is_virtual {
        writeln!(output, "Type: Virtual table").ok();
    }

    writeln!(output).ok();
    writeln!(output, "## Columns").ok();
    writeln!(output).ok();
    writeln!(output, "| Name | Type | Nullable | Primary Key | Default |").ok();
    writeln!(output, "| --- | --- | --- | --- | --- |").ok();

    for column in &table.columns {
        let nullable = if column.nullable { "YES" } else { "NO" };
        let primary_key = if column.primary_key { "YES" } else { "" };
        let default = column.default_value.as_deref().unwrap_or("");
        writeln!(
            output,
            "| {} | {} | {} | {} | {} |",
            column.name, column.data_type, nullable, primary_key, default
        )
        .ok();
    }

    if !table.indexes.is_empty() {
        writeln!(output).ok();
        writeln!(output, "## Indexes").ok();
        writeln!(output).ok();
        for index in &table.indexes {
            let unique_label = if index.unique { " (UNIQUE)" } else { "" };
            writeln!(
                output,
                "- **{}**{}: ({})",
                index.name,
                unique_label,
                index.columns.join(", ")
            )
            .ok();
        }
    }

    if !table.foreign_keys.is_empty() {
        writeln!(output).ok();
        writeln!(output, "## Foreign Keys").ok();
        writeln!(output).ok();
        for foreign_key in &table.foreign_keys {
            writeln!(
                output,
                "- {} -> {}.{}",
                foreign_key.from_column, foreign_key.to_table, foreign_key.to_column
            )
            .ok();
        }
    }

    Ok(output)
}

pub fn list_objects_core(
    connection_name: &str,
    object_type: Option<&str>,
) -> Result<String, String> {
    let (_connection, config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    let schema = schema.ok_or_else(|| {
        format!(
            "Schema not available for connection '{}'. The connection may still be loading.",
            connection_name
        )
    })?;

    let filter = object_type.unwrap_or("all").to_lowercase();

    let mut output = String::new();

    writeln!(
        output,
        "# Database: {} ({})",
        connection_name, config.database_type
    )
    .ok();
    writeln!(output).ok();

    let tables: Vec<_> = schema
        .tables
        .iter()
        .filter(|table| !table.is_virtual)
        .collect();
    let views: Vec<_> = schema
        .tables
        .iter()
        .filter(|table| table.is_virtual)
        .collect();

    let show_tables = filter == "all" || filter == "tables";
    let show_views = filter == "all" || filter == "views";

    if show_tables {
        writeln!(output, "## Tables ({} total)", tables.len()).ok();
        writeln!(output).ok();
        if tables.is_empty() {
            writeln!(output, "No tables found.").ok();
        } else {
            for table in &tables {
                let row_info = table
                    .row_count
                    .map(|count| format!(" ({count} rows)"))
                    .unwrap_or_default();
                let col_count = table.columns.len();
                writeln!(
                    output,
                    "- **{}** ({col_count} columns){row_info}",
                    table.name
                )
                .ok();
            }
        }
    }

    if show_views {
        writeln!(output).ok();
        writeln!(output, "## Views ({} total)", views.len()).ok();
        writeln!(output).ok();
        if views.is_empty() {
            writeln!(output, "No views found.").ok();
        } else {
            for view in &views {
                let col_count = view.columns.len();
                writeln!(output, "- **{}** ({col_count} columns)", view.name).ok();
            }
        }
    }

    Ok(output)
}

pub fn explain_query_core(
    sql: &str,
    connection_name: &str,
    analyze: bool,
) -> Result<String, String> {
    let (connection, config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    let explain_sql = build_explain_sql(sql, analyze, &config.database_type);

    let result = connection.execute_query(&explain_sql).map_err(|error| {
        let mut message = format!("EXPLAIN failed: {error}\n\n");
        message.push_str("**SQL:**\n```sql\n");
        message.push_str(&explain_sql);
        message.push_str("\n```\n");

        if let Some(ref schema) = schema {
            message.push_str("\n**Available tables:** ");
            let names: Vec<&str> = schema.tables.iter().map(|t| t.name.as_str()).collect();
            message.push_str(&names.join(", "));
        }

        message
    })?;

    let mut output = String::new();

    let mode = if analyze {
        "EXPLAIN ANALYZE"
    } else {
        "EXPLAIN"
    };
    writeln!(output, "## {mode} Result").ok();
    writeln!(output).ok();
    writeln!(output, "```").ok();

    if result.columns.is_empty() && result.rows.is_empty() {
        writeln!(output, "(no output)").ok();
    } else {
        for row in &result.rows {
            let line: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
            writeln!(output, "{}", line.join(" | ")).ok();
        }
    }

    writeln!(output, "```").ok();

    Ok(output)
}

pub fn modify_data_core(sql: &str, connection_name: &str) -> Result<String, String> {
    let (connection, config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    if config.read_only {
        return Err(format!(
            "Connection '{}' is in read-only mode. Data modification is not allowed.",
            connection_name
        ));
    }

    let result = connection.execute_query(sql).map_err(|error| {
        let mut message = format!("Statement execution failed: {error}\n\n");
        message.push_str("**SQL:**\n```sql\n");
        message.push_str(sql);
        message.push_str("\n```\n");

        if let Some(ref schema) = schema {
            message.push_str("\n**Available tables:** ");
            let names: Vec<&str> = schema.tables.iter().map(|t| t.name.as_str()).collect();
            message.push_str(&names.join(", "));
        }

        message
    })?;

    let mut output = String::new();
    write!(
        output,
        "Statement executed successfully on `{}`.",
        connection_name
    )
    .ok();

    if let Some(affected) = result.affected_rows {
        write!(output, " {affected} row(s) affected.").ok();
    }

    writeln!(output).ok();
    write!(
        output,
        "Execution time: {:.1}ms",
        result.execution_time.as_secs_f64() * 1000.0
    )
    .ok();

    Ok(output)
}

pub fn get_schema_core(
    connection_name: &str,
    tables: &[String],
) -> Result<String, String> {
    let (_connection, config, schema) =
        get_connection(connection_name).ok_or_else(|| format_connection_not_found(connection_name))?;

    let schema = schema.ok_or_else(|| {
        format!(
            "Schema not available for connection '{}'. The connection may still be loading.",
            connection_name
        )
    })?;

    let mut output = String::new();

    writeln!(
        output,
        "-- Database: {} ({})",
        connection_name, config.database_type
    )
    .ok();
    writeln!(
        output,
        "-- {} tables, {} views",
        schema.tables.iter().filter(|t| !t.is_virtual).count(),
        schema.tables.iter().filter(|t| t.is_virtual).count()
    )
    .ok();
    writeln!(output).ok();

    if tables.is_empty() {
        output.push_str(&generate_ddl_from_schema(&schema));
    } else {
        let mut found_any = false;
        for table_name in tables {
            if let Some(table) = schema
                .tables
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(table_name))
            {
                generate_table_ddl(table, &mut output);
                output.push('\n');
                found_any = true;
            } else {
                writeln!(output, "-- Table '{}' not found", table_name).ok();
            }
        }

        if !found_any {
            let table_names: Vec<&str> =
                schema.tables.iter().map(|t| t.name.as_str()).collect();
            return Err(format!(
                "None of the specified tables were found. Available tables: {}",
                table_names.join(", ")
            ));
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection_registry::clear_all_connections;

    #[test]
    fn test_format_connection_not_found_empty() {
        clear_all_connections();
        let result = format_connection_not_found("test_db");
        assert!(result.contains("test_db"));
        assert!(result.contains("No database connections are available"));
    }

    #[test]
    fn test_build_explain_sql_sqlite() {
        let result = build_explain_sql("SELECT * FROM users", false, &DatabaseType::Sqlite);
        assert_eq!(result, "EXPLAIN QUERY PLAN SELECT * FROM users");
    }

    #[test]
    fn test_build_explain_sql_postgres_no_analyze() {
        let result = build_explain_sql("SELECT * FROM users", false, &DatabaseType::PostgreSql);
        assert_eq!(result, "EXPLAIN SELECT * FROM users");
    }

    #[test]
    fn test_build_explain_sql_postgres_analyze() {
        let result = build_explain_sql("SELECT * FROM users", true, &DatabaseType::PostgreSql);
        assert_eq!(result, "EXPLAIN ANALYZE SELECT * FROM users");
    }

    #[test]
    fn test_build_explain_sql_mysql_no_analyze() {
        let result = build_explain_sql("SELECT * FROM users", false, &DatabaseType::MySql);
        assert_eq!(result, "EXPLAIN SELECT * FROM users");
    }

    #[test]
    fn test_build_explain_sql_mysql_analyze() {
        let result = build_explain_sql("SELECT * FROM users;", true, &DatabaseType::MySql);
        assert_eq!(result, "EXPLAIN ANALYZE SELECT * FROM users");
    }

    #[test]
    fn test_execute_query_core_connection_not_found() {
        clear_all_connections();
        let result = execute_query_core("SELECT 1", "nonexistent", 100);
        assert!(result.is_err());
        assert!(result.err().map_or(false, |e| e.contains("nonexistent")));
    }

    #[test]
    fn test_modify_data_core_connection_not_found() {
        clear_all_connections();
        let result = modify_data_core("INSERT INTO t VALUES (1)", "nonexistent");
        assert!(result.is_err());
        assert!(result.err().map_or(false, |e| e.contains("nonexistent")));
    }

    #[test]
    fn test_describe_object_core_connection_not_found() {
        clear_all_connections();
        let result = describe_object_core("users", "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_objects_core_connection_not_found() {
        clear_all_connections();
        let result = list_objects_core("nonexistent", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_schema_core_connection_not_found() {
        clear_all_connections();
        let result = get_schema_core("nonexistent", &[]);
        assert!(result.is_err());
    }
}
