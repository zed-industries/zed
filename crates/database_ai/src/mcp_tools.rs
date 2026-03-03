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
            idempotent_hint: Some(true),
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
            idempotent_hint: Some(true),
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
            idempotent_hint: Some(true),
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
            idempotent_hint: Some(true),
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
            idempotent_hint: Some(true),
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
