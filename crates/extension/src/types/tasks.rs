use crate::ExtensionWorktreeProxy;
use anyhow::anyhow;
use gpui::AppContext;
use std::sync::Arc;

use language::ToOffset;
use task::{HideStrategy, RevealStrategy, RevealTarget, Shell, TaskTemplate};
use util::ResultExt as _;

#[derive(Debug, Clone)]
pub struct TaskContextLocation {
    pub worktree_id: u64,
    /// Path relative to the worktree root.
    pub file_path: String,
    pub range: std::ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub struct TaskContextFile {
    pub worktree_id: u64,
    /// Path relative to the worktree root.
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct TaskVariable {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct TaskDefinition {
    pub label: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cwd: Option<String>,
    pub use_new_terminal: Option<bool>,
    pub allow_concurrent_runs: Option<bool>,
    pub reveal: Option<RevealStrategy>,
    pub reveal_target: Option<RevealTarget>,
    pub hide: Option<HideStrategy>,
    pub shell: Option<Shell>,
    pub show_summary: Option<bool>,
    pub show_command: Option<bool>,
    pub tags: Vec<String>,
}

impl From<TaskDefinition> for TaskTemplate {
    fn from(definition: TaskDefinition) -> Self {
        Self {
            label: definition.label,
            command: definition.command,
            args: definition.args,
            env: definition.env.into_iter().collect(),
            cwd: definition.cwd,
            use_new_terminal: definition.use_new_terminal.unwrap_or_default(),
            allow_concurrent_runs: definition.allow_concurrent_runs.unwrap_or_default(),
            reveal: definition.reveal.unwrap_or_default(),
            reveal_target: definition.reveal_target.unwrap_or_default(),
            hide: definition.hide.unwrap_or_default(),
            shell: definition.shell.unwrap_or_default(),
            show_summary: definition.show_summary.unwrap_or(true),
            show_command: definition.show_command.unwrap_or(true),
            tags: definition.tags,
        }
    }
}

pub struct ExtensionContextProvider {
    pub extension_id: Arc<str>,
    pub language_name: language::LanguageName,
    pub static_templates: Option<task::TaskTemplates>,
}

impl language::ContextProvider for ExtensionContextProvider {
    fn build_context(
        &self,
        _variables: &task::TaskVariables,
        location: language::ContextLocation<'_>,
        _project_env: Option<collections::HashMap<String, String>>,
        _toolchains: Arc<dyn language::LanguageToolchainStore>,
        cx: &mut gpui::App,
    ) -> gpui::Task<anyhow::Result<task::TaskVariables>> {
        let extension_id = self.extension_id.clone();
        let language_name = self.language_name.clone();
        let proxy = crate::ExtensionHostProxy::global(cx);
        let buffer = location.file_location.buffer.read(cx);
        let file = buffer.file();
        let worktree_id = file.map(|f| f.worktree_id(cx).to_proto()).unwrap_or(0);
        let file_path = file
            .map(|f| f.path().as_std_path().to_string_lossy().to_string())
            .unwrap_or_default();
        let snapshot = buffer.text_snapshot();
        let range = location.file_location.range.start.to_offset(&snapshot)
            ..location.file_location.range.end.to_offset(&snapshot);

        let location = TaskContextLocation {
            worktree_id,
            file_path,
            range,
        };

        let worktree_delegate = proxy.worktree_delegate(worktree_id, cx);

        cx.background_spawn(async move {
            let worktree_delegate =
                worktree_delegate.ok_or_else(|| anyhow!("worktree delegate not found"))?;

            let extension = proxy
                .extension_by_id(&extension_id)
                .ok_or_else(|| anyhow!("extension not found"))?;
            let variables = extension
                .build_context(language_name.to_string(), location, worktree_delegate)
                .await?;
            let mut result = task::TaskVariables::default();
            for variable in variables {
                result.insert(
                    task::VariableName::Custom(variable.name.into()),
                    variable.value,
                );
            }

            Ok(result)
        })
    }

    fn associated_tasks(
        &self,
        file: Option<Arc<dyn language::File>>,
        cx: &gpui::App,
    ) -> gpui::Task<Option<task::TaskTemplates>> {
        let extension_id = self.extension_id.clone();
        let language_name = self.language_name.clone();
        let static_templates = self.static_templates.clone();
        let proxy = crate::ExtensionHostProxy::global(cx);

        let worktree_id = file.as_ref().map(|file| file.worktree_id(cx).to_proto());
        let file = file.map(|file| TaskContextFile {
            worktree_id: worktree_id.unwrap_or(0),
            path: file.path().as_std_path().to_string_lossy().to_string(),
        });

        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            let mut templates = static_templates.unwrap_or_default();
            let worktree_delegate = if let Some(worktree_id) = worktree_id {
                cx.update(|cx| proxy.worktree_delegate(worktree_id, cx))
                    .ok()
                    .flatten()
            } else {
                None
            };

            if let (Some(extension), Some(worktree_delegate)) =
                (proxy.extension_by_id(&extension_id), worktree_delegate)
            {
                if let Some(definitions) = extension
                    .associated_tasks(language_name.to_string(), file, worktree_delegate)
                    .await
                    .log_err()
                {
                    templates
                        .0
                        .extend(definitions.into_iter().map(task::TaskTemplate::from));
                }
            }

            if templates.0.is_empty() {
                None
            } else {
                Some(templates)
            }
        })
    }
}
