use anyhow::Result;
use assets::Assets;
use fs::Fs;
use futures::StreamExt;
use gpui::AssetSource;
use handlebars::{Handlebars, RenderError};
use language::BufferSnapshot;
use parking_lot::Mutex;
use serde::Serialize;
use std::{ops::Range, path::PathBuf, sync::Arc, time::Duration};
use text::LineEnding;
use util::ResultExt;

#[derive(Serialize)]
pub struct ContentPromptContext {
    pub content_type: String,
    pub language_name: Option<String>,
    pub is_insert: bool,
    pub is_truncated: bool,
    pub document_content: String,
    pub user_prompt: String,
    pub rewrite_section: Option<String>,
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

/// Context required to generate a workflow step resolution prompt.
#[derive(Debug, Serialize)]
pub struct StepResolutionContext {
    /// The full context, including <step>...</step> tags
    pub workflow_context: String,
    /// The text of the specific step from the context to resolve
    pub step_to_resolve: String,
}

pub struct PromptLoadingParams<'a> {
    pub fs: Arc<dyn Fs>,
    pub repo_path: Option<PathBuf>,
    pub cx: &'a gpui::AppContext,
}

pub struct PromptBuilder {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

impl PromptBuilder {
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
        mut params: PromptLoadingParams,
        handlebars: Arc<Mutex<Handlebars<'static>>>,
    ) {
        params.repo_path = None;
        let templates_dir = paths::prompt_overrides_dir(params.repo_path.as_deref());
        params.cx.background_executor()
            .spawn(async move {
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
                        log::info!("{}.", log_message);
                    } else {
                        if !found_dir_once {
                            log::info!("No prompt template overrides directory found at {}. Using built-in prompts.", templates_dir.display());
                            if let Some(target) = symlink_status {
                                log::info!("Symlink found pointing to {}, but target is invalid.", target.display());
                            }
                        }

                        if params.fs.is_dir(parent_dir).await {
                            let (mut changes, _watcher) = params.fs.watch(parent_dir, Duration::from_secs(1)).await;
                            while let Some(changed_paths) = changes.next().await {
                                if changed_paths.iter().any(|p| p == &templates_dir) {
                                    let mut log_message = format!("Prompt template overrides directory detected at {}", templates_dir.display());
                                    if let Ok(target) = params.fs.read_link(&templates_dir).await {
                                        log_message.push_str(" -> ");
                                        log_message.push_str(&target.display().to_string());
                                    }
                                    log::info!("{}.", log_message);
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
                            if file_path.to_string_lossy().ends_with(".hbs") {
                                if let Ok(content) = params.fs.load(&file_path).await {
                                    let file_name = file_path.file_stem().unwrap().to_string_lossy();
                                    log::info!("Registering prompt template override: {}", file_name);
                                    handlebars.lock().register_template_string(&file_name, content).log_err();
                                }
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
                        if changed_paths.iter().any(|p| p == &templates_dir) {
                            if !params.fs.is_dir(&templates_dir).await {
                                log::info!("Prompt template overrides directory removed. Restoring built-in prompt templates.");
                                Self::register_built_in_templates(&mut handlebars.lock()).log_err();
                                break;
                            }
                        }
                        for changed_path in changed_paths {
                            if changed_path.starts_with(&templates_dir) && changed_path.extension().map_or(false, |ext| ext == "hbs") {
                                log::info!("Reloading prompt template override: {}", changed_path.display());
                                if let Some(content) = params.fs.load(&changed_path).await.log_err() {
                                    let file_name = changed_path.file_stem().unwrap().to_string_lossy();
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
            if let Some(id) = path.split('/').last().and_then(|s| s.strip_suffix(".hbs")) {
                if let Some(prompt) = Assets.load(path.as_ref()).log_err().flatten() {
                    log::info!("Registering built-in prompt template: {}", id);
                    let prompt = String::from_utf8_lossy(prompt.as_ref());
                    handlebars.register_template_string(id, LineEnding::normalize_cow(prompt))?
                }
            }
        }

        Ok(())
    }

    pub fn generate_content_prompt(
        &self,
        user_prompt: String,
        language_name: Option<&str>,
        buffer: BufferSnapshot,
        range: Range<usize>,
    ) -> Result<String, RenderError> {
        let content_type = match language_name {
            None | Some("Markdown" | "Plain Text") => "text",
            Some(_) => "code",
        };

        const MAX_CTX: usize = 50000;
        let is_insert = range.is_empty();
        let mut is_truncated = false;

        let before_range = 0..range.start;
        let truncated_before = if before_range.len() > MAX_CTX {
            is_truncated = true;
            range.start - MAX_CTX..range.start
        } else {
            before_range
        };

        let after_range = range.end..buffer.len();
        let truncated_after = if after_range.len() > MAX_CTX {
            is_truncated = true;
            range.end..range.end + MAX_CTX
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

        let context = ContentPromptContext {
            content_type: content_type.to_string(),
            language_name: language_name.map(|s| s.to_string()),
            is_insert,
            is_truncated,
            document_content,
            user_prompt,
            rewrite_section,
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

    pub fn generate_workflow_prompt(&self) -> Result<String, RenderError> {
        self.handlebars.lock().render("edit_workflow", &())
    }
}
