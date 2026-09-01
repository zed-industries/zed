use crate::ExtensionWorktreeProxy;
use anyhow::anyhow;
use gpui::AppContext;
use std::sync::Arc;

use language::ToOffset;
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

pub struct ExtensionContextProvider {
    pub extension_id: Arc<str>,
    pub language_name: language::LanguageName,
    pub static_templates: Option<task::TaskTemplates>,
}

impl language::ContextProvider for ExtensionContextProvider {
    fn build_context(
        &self,
        variables: &task::TaskVariables,
        location: language::ContextLocation<'_>,
        project_env: Option<collections::HashMap<String, String>>,
        _toolchains: Arc<dyn language::LanguageToolchainStore>,
        cx: &mut gpui::App,
    ) -> gpui::Task<anyhow::Result<task::TaskVariables>> {
        let extension_id = self.extension_id.clone();
        let language_name = self.language_name.clone();
        let variables = variables.clone();
        let proxy = crate::ExtensionHostProxy::global(cx);
        let buffer = location.file_location.buffer.read(cx);
        let file = buffer.file();
        let snapshot = buffer.text_snapshot();
        let range = location.file_location.range.start.to_offset(&snapshot)
            ..location.file_location.range.end.to_offset(&snapshot);

        let location = file.map(|file| TaskContextLocation {
            worktree_id: file.worktree_id(cx).to_proto(),
            file_path: file.path().as_std_path().to_string_lossy().to_string(),
            range,
        });
        let worktree_delegate = location
            .as_ref()
            .and_then(|location| proxy.worktree_delegate(location.worktree_id, cx));

        cx.background_spawn(async move {
            let Some(location) = location else {
                anyhow::bail!("buffer has no file; cannot build task context");
            };
            let worktree_delegate = worktree_delegate.ok_or_else(|| {
                anyhow!(
                    "no worktree found for id {}; cannot build task context",
                    location.worktree_id
                )
            })?;

            let extension = proxy
                .extension_by_id(&extension_id)
                .ok_or_else(|| anyhow!("extension not found"))?;
            extension
                .build_context(
                    language_name.to_string(),
                    variables,
                    project_env,
                    location,
                    worktree_delegate,
                )
                .await
        })
    }

    fn associated_tasks(
        &self,
        buffer: Option<gpui::Entity<language::Buffer>>,
        cx: &gpui::App,
    ) -> gpui::Task<Option<task::TaskTemplates>> {
        let extension_id = self.extension_id.clone();
        let language_name = self.language_name.clone();
        let static_templates = self.static_templates.clone();
        let proxy = crate::ExtensionHostProxy::global(cx);

        let file = buffer.as_ref().and_then(|buffer| {
            buffer.read(cx).file().map(|file| TaskContextFile {
                worktree_id: file.worktree_id(cx).to_proto(),
                path: file.path().as_std_path().to_string_lossy().to_string(),
            })
        });
        let worktree_id = file.as_ref().map(|file| file.worktree_id);

        cx.spawn(async move |cx: &mut gpui::AsyncApp| {
            let mut templates = static_templates.unwrap_or_default();
            let worktree_delegate = if let Some(worktree_id) = worktree_id {
                cx.update(|cx| proxy.worktree_delegate(worktree_id, cx))
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
                    templates.0.extend(definitions);
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
