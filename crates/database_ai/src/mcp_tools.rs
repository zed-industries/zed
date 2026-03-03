use std::future::Future;

use anyhow::Result;
use context_server::listener::{McpServerTool, ToolResponse};
use context_server::types::{ToolAnnotations, ToolResponseContent};
use gpui::AsyncApp;

use crate::tools::describe_object::DescribeObjectToolInput;
use crate::tools::execute_query::ExecuteQueryToolInput;
use crate::tools::explain_query::ExplainQueryToolInput;
use crate::tools::get_schema::GetSchemaToolInput;
use crate::tools::list_objects::ListObjectsToolInput;
use crate::tools::modify_data::ModifyDataToolInput;

fn text_response(text: String) -> ToolResponse<()> {
    ToolResponse {
        content: vec![ToolResponseContent::Text { text }],
        structured_content: (),
    }
}

fn error_response(error: String) -> anyhow::Error {
    anyhow::anyhow!(error)
}

#[derive(Clone)]
pub struct McpExecuteQuery;

impl McpServerTool for McpExecuteQuery {
    type Input = ExecuteQueryToolInput;
    type Output = ();

    const NAME: &'static str = "database_execute_query";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Execute SQL Query".into()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text = database_core::execute_query_core(&input.sql, &input.connection, input.limit)
                .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[derive(Clone)]
pub struct McpDescribeObject;

impl McpServerTool for McpDescribeObject {
    type Input = DescribeObjectToolInput;
    type Output = ();

    const NAME: &'static str = "database_describe_object";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Describe DB Object".into()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text =
                database_core::describe_object_core(&input.object_name, &input.connection)
                    .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[derive(Clone)]
pub struct McpListObjects;

impl McpServerTool for McpListObjects {
    type Input = ListObjectsToolInput;
    type Output = ();

    const NAME: &'static str = "database_list_objects";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("List DB Objects".into()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text = database_core::list_objects_core(
                &input.connection,
                input.object_type.as_deref(),
            )
            .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[derive(Clone)]
pub struct McpExplainQuery;

impl McpServerTool for McpExplainQuery {
    type Input = ExplainQueryToolInput;
    type Output = ();

    const NAME: &'static str = "database_explain_query";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Explain SQL Query".into()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text =
                database_core::explain_query_core(&input.sql, &input.connection, input.analyze)
                    .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[derive(Clone)]
pub struct McpModifyData;

impl McpServerTool for McpModifyData {
    type Input = ModifyDataToolInput;
    type Output = ();

    const NAME: &'static str = "database_modify_data";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Modify DB Data".into()),
            read_only_hint: Some(false),
            destructive_hint: Some(true),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text = database_core::modify_data_core(&input.sql, &input.connection)
                .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[derive(Clone)]
pub struct McpGetSchema;

impl McpServerTool for McpGetSchema {
    type Input = GetSchemaToolInput;
    type Output = ();

    const NAME: &'static str = "database_get_schema";

    fn annotations(&self) -> ToolAnnotations {
        ToolAnnotations {
            title: Some("Get DB Schema".into()),
            read_only_hint: Some(true),
            destructive_hint: Some(false),
            idempotent_hint: Some(false),
            open_world_hint: Some(false),
        }
    }

    fn run(
        &self,
        input: Self::Input,
        _cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<ToolResponse<()>>> {
        async move {
            let text = database_core::get_schema_core(&input.connection, &input.tables)
                .map_err(error_response)?;
            Ok(text_response(text))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use context_server::listener::McpServerTool;

    #[test]
    fn test_tool_names_match_agent_tools() {
        assert_eq!(McpExecuteQuery::NAME, "database_execute_query");
        assert_eq!(McpDescribeObject::NAME, "database_describe_object");
        assert_eq!(McpListObjects::NAME, "database_list_objects");
        assert_eq!(McpExplainQuery::NAME, "database_explain_query");
        assert_eq!(McpModifyData::NAME, "database_modify_data");
        assert_eq!(McpGetSchema::NAME, "database_get_schema");
    }

    #[test]
    fn test_read_only_tools_annotations() {
        let read_only_tools: Vec<ToolAnnotations> = vec![
            McpExecuteQuery.annotations(),
            McpDescribeObject.annotations(),
            McpListObjects.annotations(),
            McpExplainQuery.annotations(),
            McpGetSchema.annotations(),
        ];

        for annotations in &read_only_tools {
            assert_eq!(annotations.read_only_hint, Some(true));
            assert_eq!(annotations.destructive_hint, Some(false));
            assert_eq!(annotations.idempotent_hint, Some(false));
        }
    }

    #[test]
    fn test_modify_data_annotations() {
        let annotations = McpModifyData.annotations();
        assert_eq!(annotations.read_only_hint, Some(false));
        assert_eq!(annotations.destructive_hint, Some(true));
        assert_eq!(annotations.idempotent_hint, Some(false));
    }

    #[test]
    fn test_execute_query_input_deserialization() {
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
    fn test_execute_query_input_with_custom_limit() {
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
    fn test_explain_query_input_defaults() {
        let json = serde_json::json!({
            "sql": "SELECT 1",
            "connection": "db"
        });
        let input: ExplainQueryToolInput = serde_json::from_value(json).unwrap();
        assert!(!input.analyze);
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
    fn test_get_schema_input_with_tables() {
        let json = serde_json::json!({
            "connection": "db",
            "tables": ["users", "orders"]
        });
        let input: GetSchemaToolInput = serde_json::from_value(json).unwrap();
        assert_eq!(input.tables, vec!["users", "orders"]);
    }

    #[test]
    fn test_text_response_structure() {
        let response = text_response("hello".to_string());
        assert_eq!(response.content.len(), 1);
        match &response.content[0] {
            ToolResponseContent::Text { text } => assert_eq!(text, "hello"),
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_error_response_preserves_message() {
        let error = error_response("something failed".to_string());
        assert_eq!(format!("{}", error), "something failed");
    }
}
