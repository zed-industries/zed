use agent::AgentTool as _;
use agent_client_protocol as acp;

use crate::tools::execute_query::{ExecuteQueryTool, ExecuteQueryToolInput};
use crate::tools::explain_query::ExplainQueryToolInput;
use crate::tools::get_schema::GetSchemaToolInput;
use crate::tools::list_objects::ListObjectsToolInput;
use crate::tools::modify_data::{ModifyDataTool, ModifyDataToolInput};

#[test]
fn test_execute_query_input_default_limit() {
    let json = serde_json::json!({
        "sql": "SELECT * FROM users",
        "connection": "my_db"
    });
    let input: ExecuteQueryToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.sql, "SELECT * FROM users");
    assert_eq!(input.connection, "my_db");
    assert_eq!(input.limit, 100);
}

#[test]
fn test_execute_query_input_custom_limit() {
    let json = serde_json::json!({
        "sql": "SELECT 1",
        "connection": "db",
        "limit": 50
    });
    let input: ExecuteQueryToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.limit, 50);
}

#[test]
fn test_list_objects_input_default_type() {
    let json = serde_json::json!({
        "connection": "my_db"
    });
    let input: ListObjectsToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.object_type, Some("all".to_string()));
}

#[test]
fn test_list_objects_input_filter_tables() {
    let json = serde_json::json!({
        "connection": "db",
        "object_type": "tables"
    });
    let input: ListObjectsToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.object_type, Some("tables".to_string()));
}

#[test]
fn test_explain_query_input_defaults_to_no_analyze() {
    let json = serde_json::json!({
        "sql": "SELECT 1",
        "connection": "db"
    });
    let input: ExplainQueryToolInput = serde_json::from_value(json).unwrap();
    assert!(!input.analyze);
}

#[test]
fn test_explain_query_input_with_analyze() {
    let json = serde_json::json!({
        "sql": "SELECT 1",
        "connection": "db",
        "analyze": true
    });
    let input: ExplainQueryToolInput = serde_json::from_value(json).unwrap();
    assert!(input.analyze);
}

#[test]
fn test_get_schema_input_empty_tables() {
    let json = serde_json::json!({
        "connection": "db"
    });
    let input: GetSchemaToolInput = serde_json::from_value(json).unwrap();
    assert!(input.tables.is_empty());
}

#[test]
fn test_get_schema_input_with_specific_tables() {
    let json = serde_json::json!({
        "connection": "db",
        "tables": ["users", "orders"]
    });
    let input: GetSchemaToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.tables, vec!["users", "orders"]);
}

#[test]
fn test_modify_data_input_deserialization() {
    let json = serde_json::json!({
        "sql": "INSERT INTO t VALUES (1)",
        "connection": "db"
    });
    let input: ModifyDataToolInput = serde_json::from_value(json).unwrap();
    assert_eq!(input.sql, "INSERT INTO t VALUES (1)");
    assert_eq!(input.connection, "db");
}

#[test]
fn test_execute_query_missing_connection_returns_error() {
    let result = database_core::execute_query_core("SELECT 1", "nonexistent_xyz", 100);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.contains("nonexistent_xyz"));
}

#[test]
fn test_explain_query_missing_connection_returns_error() {
    let result = database_core::explain_query_core("SELECT 1", "nonexistent_xyz", false);
    assert!(result.is_err());
}

#[test]
fn test_list_objects_missing_connection_returns_error() {
    let result = database_core::list_objects_core("nonexistent_xyz", Some("all"));
    assert!(result.is_err());
}

#[test]
fn test_get_schema_missing_connection_returns_error() {
    let result = database_core::get_schema_core("nonexistent_xyz", &[]);
    assert!(result.is_err());
}

#[test]
fn test_execute_query_tool_name() {
    assert_eq!(ExecuteQueryTool::NAME, "database_execute_query");
}

#[test]
fn test_execute_query_tool_kind_is_other() {
    assert_eq!(ExecuteQueryTool::kind(), acp::ToolKind::Other);
}

#[test]
fn test_modify_data_tool_kind_is_write() {
    assert_eq!(ModifyDataTool::kind(), acp::ToolKind::Edit);
}

#[test]
fn test_allowlist_blocks_delete_by_default() {
    assert!(
        database_core::ReadOnlyGuard::check("DELETE FROM users").is_err(),
        "DELETE must be blocked by ReadOnlyGuard"
    );
    assert!(
        database_core::ReadOnlyGuard::check("DROP TABLE users").is_err(),
        "DROP must be blocked by ReadOnlyGuard"
    );
    assert!(
        database_core::ReadOnlyGuard::check("SELECT id FROM users").is_ok(),
        "SELECT must be allowed by ReadOnlyGuard"
    );
}
