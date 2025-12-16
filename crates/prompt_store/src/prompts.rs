use anyhow::Result;
use assets::Assets;
use fs::Fs;
use futures::StreamExt;
use gpui::{App, AppContext as _, AssetSource};
use handlebars::{Handlebars, RenderError};
use language::{BufferSnapshot, LanguageName, Point};
use parking_lot::Mutex;
use serde::Serialize;
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use text::LineEnding;
use util::{
    ResultExt, get_default_system_shell_preferring_bash, rel_path::RelPath, shell::ShellKind,
};

use crate::UserPromptId;

pub const RULES_FILE_NAMES: &[&str] = &[
    ".rules",
    ".cursorrules",
    ".windsurfrules",
    ".clinerules",
    ".github/copilot-instructions.md",
    "CLAUDE.md",
    "AGENT.md",
    "AGENTS.md",
    "GEMINI.md",
];

#[derive(Default, Debug, Clone, Serialize)]
pub struct ProjectContext {
    pub worktrees: Vec<WorktreeContext>,
    /// Whether any worktree has a rules_file. Provided as a field because handlebars can't do this.
    pub has_rules: bool,
    pub user_rules: Vec<UserRulesContext>,
    /// `!user_rules.is_empty()` - provided as a field because handlebars can't do this.
    pub has_user_rules: bool,
    pub os: String,
    pub arch: String,
    pub shell: String,
}

impl ProjectContext {
    pub fn new(worktrees: Vec<WorktreeContext>, default_user_rules: Vec<UserRulesContext>) -> Self {
        let has_rules = worktrees
            .iter()
            .any(|worktree| worktree.rules_file.is_some());
        Self {
            worktrees,
            has_rules,
            has_user_rules: !default_user_rules.is_empty(),
            user_rules: default_user_rules,
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            shell: ShellKind::new(&get_default_system_shell_preferring_bash(), cfg!(windows))
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserRulesContext {
    pub uuid: UserPromptId,
    pub title: Option<String>,
    pub contents: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct WorktreeContext {
    pub root_name: String,
    pub abs_path: Arc<Path>,
    pub rules_file: Option<RulesFileContext>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct RulesFileContext {
    pub path_in_worktree: Arc<RelPath>,
    pub text: String,
    // This used for opening rules files. TODO: Since it isn't related to prompt templating, this
    // should be moved elsewhere.
    #[serde(skip)]
    pub project_entry_id: usize,
}

#[derive(Serialize)]
pub struct ContentPromptDiagnosticContext {
    pub line_number: usize,
    pub error_message: String,
    pub code_content: String,
}

#[derive(Serialize)]
pub struct ContentPromptContext {
    pub content_type: String,
    pub language_name: Option<String>,
    pub is_insert: bool,
    pub is_truncated: bool,
    pub document_content: String,
    pub user_prompt: String,
    pub rewrite_section: Option<String>,
    pub diagnostic_errors: Vec<ContentPromptDiagnosticContext>,
}

#[derive(Serialize)]
pub struct ContentPromptContextV2 {
    pub content_type: String,
    pub language_name: Option<String>,
    pub is_truncated: bool,
    pub document_content: String,
    pub rewrite_section: Option<String>,
    pub diagnostic_errors: Vec<ContentPromptDiagnosticContext>,
}

#[derive(Serialize)]
pub struct TerminalAssistantPromptContext {
    pub os: String,
    pub arch: String,
    pub shell: Option<String>,
    pub working_directory: Option<String>,
    pub latest_output: Vec<String>,
    pub user_prompt: String,
}

pub struct PromptLoadingParams<'a> {
    pub fs: Arc<dyn Fs>,
    pub repo_path: Option<PathBuf>,
    pub cx: &'a gpui::App,
}

pub struct PromptBuilder {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

impl PromptBuilder {
    pub fn load(fs: Arc<dyn Fs>, stdout_is_a_pty: bool, cx: &mut App) -> Arc<Self> {
        Self::new(Some(PromptLoadingParams {
            fs: fs.clone(),
            repo_path: stdout_is_a_pty
                .then(|| std::env::current_dir().log_err())
                .flatten(),
            cx,
        }))
        .log_err()
        .map(Arc::new)
        .unwrap_or_else(|| Arc::new(Self::new(None).unwrap()))
    }

    pub fn new(loading_params: Option<PromptLoadingParams>) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        Self::register_built_in_templates(&mut handlebars)?;

        let handlebars = Arc::new(Mutex::new(handlebars));

        if let Some(params) = loading_params {
            Self::watch_fs_for_template_overrides(params, handlebars.clone());
        }

        Ok(Self { handlebars })
    }

    /// Watches the filesystem for changes to prompt template overrides.
    ///
    /// This function sets up a file watcher on the prompt templates directory. It performs
    /// an initial scan of the directory and registers any existing template overrides.
    /// Then it continuously monitors for changes, reloading templates as they are
    /// modified or added.
    ///
    /// If the templates directory doesn't exist initially, it waits for it to be created.
    /// If the directory is removed, it restores the built-in templates and waits for the
    /// directory to be recreated.
    ///
    /// # Arguments
    ///
    /// * `params` - A `PromptLoadingParams` struct containing the filesystem, repository path,
    ///   and application context.
    /// * `handlebars` - An `Arc<Mutex<Handlebars>>` for registering and updating templates.
    fn watch_fs_for_template_overrides(
        params: PromptLoadingParams,
        handlebars: Arc<Mutex<Handlebars<'static>>>,
    ) {
        let templates_dir = paths::prompt_overrides_dir(params.repo_path.as_deref());
        params.cx.background_spawn(async move {
            let Some(parent_dir) = templates_dir.parent() else {
                return;
            };

            let mut found_dir_once = false;
            loop {
                // Check if the templates directory exists and handle its status
                // If it exists, log its presence and check if it's a symlink
                // If it doesn't exist:
                //   - Log that we're using built-in prompts
                //   - Check if it's a broken symlink and log if so
                //   - Set up a watcher to detect when it's created
                // After the first check, set the `found_dir_once` flag
                // This allows us to avoid logging when looping back around after deleting the prompt overrides directory.
                let dir_status = params.fs.is_dir(&templates_dir).await;
                let symlink_status = params.fs.read_link(&templates_dir).await.ok();
                if dir_status {
                    let mut log_message = format!("Prompt template overrides directory found at {}", templates_dir.display());
                    if let Some(target) = symlink_status {
                        log_message.push_str(" -> ");
                        log_message.push_str(&target.display().to_string());
                    }
                    log::trace!("{}.", log_message);
                } else {
                    if !found_dir_once {
                        log::trace!("No prompt template overrides directory found at {}. Using built-in prompts.", templates_dir.display());
                        if let Some(target) = symlink_status {
                            log::trace!("Symlink found pointing to {}, but target is invalid.", target.display());
                        }
                    }

                    if params.fs.is_dir(parent_dir).await {
                        let (mut changes, _watcher) = params.fs.watch(parent_dir, Duration::from_secs(1)).await;
                        while let Some(changed_paths) = changes.next().await {
                            if changed_paths.iter().any(|p| &p.path == &templates_dir) {
                                let mut log_message = format!("Prompt template overrides directory detected at {}", templates_dir.display());
                                if let Ok(target) = params.fs.read_link(&templates_dir).await {
                                    log_message.push_str(" -> ");
                                    log_message.push_str(&target.display().to_string());
                                }
                                log::trace!("{}.", log_message);
                                break;
                            }
                        }
                    } else {
                        return;
                    }
                }

                found_dir_once = true;

                // Initial scan of the prompt overrides directory
                if let Ok(mut entries) = params.fs.read_dir(&templates_dir).await {
                    while let Some(Ok(file_path)) = entries.next().await {
                        if file_path.to_string_lossy().ends_with(".hbs")
                            && let Ok(content) = params.fs.load(&file_path).await {
                                let file_name = file_path.file_stem().unwrap().to_string_lossy();
                                log::debug!("Registering prompt template override: {}", file_name);
                                handlebars.lock().register_template_string(&file_name, content).log_err();
                            }
                    }
                }

                // Watch both the parent directory and the template overrides directory:
                // - Monitor the parent directory to detect if the template overrides directory is deleted.
                // - Monitor the template overrides directory to re-register templates when they change.
                // Combine both watch streams into a single stream.
                let (parent_changes, parent_watcher) = params.fs.watch(parent_dir, Duration::from_secs(1)).await;
                let (changes, watcher) = params.fs.watch(&templates_dir, Duration::from_secs(1)).await;
                let mut combined_changes = futures::stream::select(changes, parent_changes);

                while let Some(changed_paths) = combined_changes.next().await {
                    if changed_paths.iter().any(|p| &p.path == &templates_dir)
                        && !params.fs.is_dir(&templates_dir).await {
                            log::info!("Prompt template overrides directory removed. Restoring built-in prompt templates.");
                            Self::register_built_in_templates(&mut handlebars.lock()).log_err();
                            break;
                        }
                    for event in changed_paths {
                        if event.path.starts_with(&templates_dir) && event.path.extension().is_some_and(|ext| ext == "hbs") {
                            log::info!("Reloading prompt template override: {}", event.path.display());
                            if let Some(content) = params.fs.load(&event.path).await.log_err() {
                                let file_name = event.path.file_stem().unwrap().to_string_lossy();
                                handlebars.lock().register_template_string(&file_name, content).log_err();
                            }
                        }
                    }
                }

                drop(watcher);
                drop(parent_watcher);
            }
        })
            .detach();
    }

    fn register_built_in_templates(handlebars: &mut Handlebars) -> Result<()> {
        for path in Assets.list("prompts")? {
            if let Some(id) = path
                .split('/')
                .next_back()
                .and_then(|s| s.strip_suffix(".hbs"))
                && let Some(prompt) = Assets.load(path.as_ref()).log_err().flatten()
            {
                log::debug!("Registering built-in prompt template: {}", id);
                let prompt = String::from_utf8_lossy(prompt.as_ref());
                handlebars.register_template_string(id, LineEnding::normalize_cow(prompt))?
            }
        }

        Ok(())
    }

    pub fn generate_inline_transformation_prompt_tools(
        &self,
        language_name: Option<&LanguageName>,
        buffer: BufferSnapshot,
        range: Range<usize>,
    ) -> Result<String, RenderError> {
        let content_type = match language_name.as_ref().map(|l| l.as_ref()) {
            None | Some("Markdown" | "Plain Text") => "text",
            Some(_) => "code",
        };

        const MAX_CTX: usize = 50000;
        let is_insert = range.is_empty();
        let mut is_truncated = false;

        let before_range = 0..range.start;
        let truncated_before = if before_range.len() > MAX_CTX {
            is_truncated = true;
            let start = buffer.clip_offset(range.start - MAX_CTX, text::Bias::Right);
            start..range.start
        } else {
            before_range
        };

        let after_range = range.end..buffer.len();
        let truncated_after = if after_range.len() > MAX_CTX {
            is_truncated = true;
            let end = buffer.clip_offset(range.end + MAX_CTX, text::Bias::Left);
            range.end..end
        } else {
            after_range
        };

        let mut document_content = String::new();
        for chunk in buffer.text_for_range(truncated_before) {
            document_content.push_str(chunk);
        }
        if is_insert {
            document_content.push_str("<insert_here></insert_here>");
        } else {
            document_content.push_str("<rewrite_this>\n");
            for chunk in buffer.text_for_range(range.clone()) {
                document_content.push_str(chunk);
            }
            document_content.push_str("\n</rewrite_this>");
        }
        for chunk in buffer.text_for_range(truncated_after) {
            document_content.push_str(chunk);
        }

        let rewrite_section = if !is_insert {
            let mut section = String::new();
            for chunk in buffer.text_for_range(range.clone()) {
                section.push_str(chunk);
            }
            Some(section)
        } else {
            None
        };
        let diagnostics = buffer.diagnostics_in_range::<_, Point>(range, false);
        let diagnostic_errors: Vec<ContentPromptDiagnosticContext> = diagnostics
            .map(|entry| {
                let start = entry.range.start;
                ContentPromptDiagnosticContext {
                    line_number: (start.row + 1) as usize,
                    error_message: entry.diagnostic.message.clone(),
                    code_content: buffer.text_for_range(entry.range).collect(),
                }
            })
            .collect();

        let context = ContentPromptContextV2 {
            content_type: content_type.to_string(),
            language_name: language_name.map(|s| s.to_string()),
            is_truncated,
            document_content,
            rewrite_section,
            diagnostic_errors,
        };
        self.handlebars.lock().render("content_prompt_v2", &context)
    }

    pub fn generate_inline_transformation_prompt(
        &self,
        user_prompt: String,
        language_name: Option<&LanguageName>,
        buffer: BufferSnapshot,
        range: Range<usize>,
    ) -> Result<String, RenderError> {
        let content_type = match language_name.as_ref().map(|l| l.as_ref()) {
            None | Some("Markdown" | "Plain Text") => "text",
            Some(_) => "code",
        };

        const MAX_CTX: usize = 50000;
        let is_insert = range.is_empty();
        let mut is_truncated = false;

        let before_range = 0..range.start;
        let truncated_before = if before_range.len() > MAX_CTX {
            is_truncated = true;
            let start = buffer.clip_offset(range.start - MAX_CTX, text::Bias::Right);
            start..range.start
        } else {
            before_range
        };

        let after_range = range.end..buffer.len();
        let truncated_after = if after_range.len() > MAX_CTX {
            is_truncated = true;
            let end = buffer.clip_offset(range.end + MAX_CTX, text::Bias::Left);
            range.end..end
        } else {
            after_range
        };

        let mut document_content = String::new();
        for chunk in buffer.text_for_range(truncated_before) {
            document_content.push_str(chunk);
        }
        if is_insert {
            document_content.push_str("<insert_here></insert_here>");
        } else {
            document_content.push_str("<rewrite_this>\n");
            for chunk in buffer.text_for_range(range.clone()) {
                document_content.push_str(chunk);
            }
            document_content.push_str("\n</rewrite_this>");
        }
        for chunk in buffer.text_for_range(truncated_after) {
            document_content.push_str(chunk);
        }

        let rewrite_section = if !is_insert {
            let mut section = String::new();
            for chunk in buffer.text_for_range(range.clone()) {
                section.push_str(chunk);
            }
            Some(section)
        } else {
            None
        };
        let diagnostics = buffer.diagnostics_in_range::<_, Point>(range, false);
        let diagnostic_errors: Vec<ContentPromptDiagnosticContext> = diagnostics
            .map(|entry| {
                let start = entry.range.start;
                ContentPromptDiagnosticContext {
                    line_number: (start.row + 1) as usize,
                    error_message: entry.diagnostic.message.clone(),
                    code_content: buffer.text_for_range(entry.range).collect(),
                }
            })
            .collect();

        let context = ContentPromptContext {
            content_type: content_type.to_string(),
            language_name: language_name.map(|s| s.to_string()),
            is_insert,
            is_truncated,
            document_content,
            user_prompt,
            rewrite_section,
            diagnostic_errors,
        };
        self.handlebars.lock().render("content_prompt", &context)
    }

    pub fn generate_terminal_assistant_prompt(
        &self,
        user_prompt: &str,
        shell: Option<&str>,
        working_directory: Option<&str>,
        latest_output: &[String],
    ) -> Result<String, RenderError> {
        let context = TerminalAssistantPromptContext {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            shell: shell.map(|s| s.to_string()),
            working_directory: working_directory.map(|s| s.to_string()),
            latest_output: latest_output.to_vec(),
            user_prompt: user_prompt.to_string(),
        };

        self.handlebars
            .lock()
            .render("terminal_assistant_prompt", &context)
    }
}
