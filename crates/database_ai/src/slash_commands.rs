use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use anyhow::Result;
use assistant_slash_command::{
    AfterCompletion, ArgumentCompletion, SlashCommand, SlashCommandOutput,
    SlashCommandOutputSection, SlashCommandResult,
};
use database_core::{explain_query_core, get_schema_core, list_connections};
use gpui::{Task, WeakEntity};
use language::{BufferSnapshot, CodeLabel, LspAdapterDelegate};
use ui::{IconName, prelude::*};
use workspace::Workspace;

/// `/db-schema [connection] [table]`
///
/// Inserts the DDL schema for a database connection or a specific table.
/// With no arguments, lists available connections.
/// With one argument, inserts the full schema for that connection.
/// With two arguments, inserts the schema for the named table.
pub struct DbSchemaSlashCommand;

impl SlashCommand for DbSchemaSlashCommand {
    fn name(&self) -> String {
        "db-schema".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::DatabaseZap
    }

    fn label(&self, _cx: &App) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }

    fn description(&self) -> String {
        "Insert database schema into context".to_string()
    }

    fn menu_text(&self) -> String {
        "Insert Database Schema".to_string()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn accepts_arguments(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let connections: Vec<ArgumentCompletion> = if arguments.len() <= 1 {
            list_connections()
                .into_iter()
                .map(|(name, _)| ArgumentCompletion {
                    label: CodeLabel::plain(name.clone(), None),
                    new_text: name,
                    after_completion: AfterCompletion::Continue,
                    replace_previous_arguments: false,
                })
                .collect()
        } else {
            Vec::new()
        };
        Task::ready(Ok(connections))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let arguments = arguments.to_vec();
        Task::ready((|| {
            let (text, label) = if arguments.is_empty() {
                let connections = list_connections();
                if connections.is_empty() {
                    let text = "No database connections available. Open the Database panel to add a connection.".to_string();
                    (text.clone(), SharedString::from(text))
                } else {
                    let names: Vec<String> =
                        connections.into_iter().map(|(name, _)| name).collect();
                    let text = format!(
                        "Available database connections:\n{}",
                        names
                            .iter()
                            .map(|n| format!("- {n}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                    (text, "Available connections".into())
                }
            } else if arguments.len() == 1 {
                let connection = &arguments[0];
                let schema = get_schema_core(connection, &[])
                    .map_err(|error| anyhow::anyhow!(error))?;
                let label: SharedString = format!("Schema: {connection}").into();
                (schema, label)
            } else {
                let connection = &arguments[0];
                let table = &arguments[1];
                let schema = get_schema_core(connection, std::slice::from_ref(table))
                    .map_err(|error| anyhow::anyhow!(error))?;
                let label: SharedString = format!("Schema: {connection}/{table}").into();
                (schema, label)
            };

            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::DatabaseZap,
                    label,
                    metadata: None,
                }],
                run_commands_in_text: false,
            }
            .into_event_stream())
        })())
    }
}

/// `/db-query <connection> <sql>`
///
/// Executes a SQL query and inserts the results as a markdown table.
pub struct DbQuerySlashCommand;

impl SlashCommand for DbQuerySlashCommand {
    fn name(&self) -> String {
        "db-query".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::DatabaseZap
    }

    fn label(&self, _cx: &App) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }

    fn description(&self) -> String {
        "Execute a SQL query and insert results".to_string()
    }

    fn menu_text(&self) -> String {
        "Execute Database Query".to_string()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let completions = if arguments.is_empty() || (arguments.len() == 1 && arguments[0].is_empty()) {
            list_connections()
                .into_iter()
                .map(|(name, _)| ArgumentCompletion {
                    label: CodeLabel::plain(name.clone(), None),
                    new_text: name,
                    after_completion: AfterCompletion::Continue,
                    replace_previous_arguments: false,
                })
                .collect()
        } else {
            Vec::new()
        };
        Task::ready(Ok(completions))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let arguments = arguments.to_vec();
        Task::ready((|| {
            if arguments.len() < 2 {
                return Err(anyhow::anyhow!(
                    "Usage: /db-query <connection> <sql>\nExample: /db-query mydb SELECT * FROM users LIMIT 10"
                ));
            }
            let connection = &arguments[0];
            let sql = arguments[1..].join(" ");
            let text = database_core::execute_query_core(&sql, connection, 100)
                .map_err(|error| anyhow::anyhow!(error))?;
            let range = 0..text.len();
            let label: SharedString = format!("Query on {connection}").into();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::DatabaseZap,
                    label,
                    metadata: None,
                }],
                run_commands_in_text: false,
            }
            .into_event_stream())
        })())
    }
}

/// `/db-explain <connection> <sql>`
///
/// Runs EXPLAIN on a SQL query and inserts the execution plan.
pub struct DbExplainSlashCommand;

impl SlashCommand for DbExplainSlashCommand {
    fn name(&self) -> String {
        "db-explain".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::DatabaseZap
    }

    fn label(&self, _cx: &App) -> CodeLabel {
        CodeLabel::plain(self.name(), None)
    }

    fn description(&self) -> String {
        "Explain a SQL query execution plan".to_string()
    }

    fn menu_text(&self) -> String {
        "Explain Database Query".to_string()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakEntity<Workspace>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        let completions = if arguments.is_empty() || (arguments.len() == 1 && arguments[0].is_empty()) {
            list_connections()
                .into_iter()
                .map(|(name, _)| ArgumentCompletion {
                    label: CodeLabel::plain(name.clone(), None),
                    new_text: name,
                    after_completion: AfterCompletion::Continue,
                    replace_previous_arguments: false,
                })
                .collect()
        } else {
            Vec::new()
        };
        Task::ready(Ok(completions))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakEntity<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Task<SlashCommandResult> {
        let arguments = arguments.to_vec();
        Task::ready((|| {
            if arguments.len() < 2 {
                return Err(anyhow::anyhow!(
                    "Usage: /db-explain <connection> <sql>\nExample: /db-explain mydb SELECT * FROM users"
                ));
            }
            let connection = &arguments[0];
            let sql = arguments[1..].join(" ");
            let text = explain_query_core(&sql, connection, false)
                .map_err(|error| anyhow::anyhow!(error))?;
            let range = 0..text.len();
            let label: SharedString = format!("Explain on {connection}").into();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::DatabaseZap,
                    label,
                    metadata: None,
                }],
                run_commands_in_text: false,
            }
            .into_event_stream())
        })())
    }
}
