use assets::Assets;
use fs::Fs;
use futures::StreamExt;
use handlebars::{Handlebars, RenderError, TemplateError};
use language::BufferSnapshot;
use parking_lot::Mutex;
use serde::Serialize;
use std::{ops::Range, sync::Arc, time::Duration};
use util::ResultExt;

#[derive(Serialize)]
pub struct ContentPromptContext {
    pub content_type: String,
    pub language_name: Option<String>,
    pub is_truncated: bool,
    pub document_content: String,
    pub user_prompt: String,
    pub rewrite_section: String,
    pub rewrite_section_prefix: String,
    pub rewrite_section_suffix: String,
    pub rewrite_section_with_edits: String,
    pub has_insertion: bool,
    pub has_replacement: bool,
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

pub struct PromptBuilder {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

pub struct PromptOverrideContext<'a> {
    pub dev_mode: bool,
    pub fs: Arc<dyn Fs>,
    pub cx: &'a mut gpui::AppContext,
}

impl PromptBuilder {
    pub fn new(override_cx: Option<PromptOverrideContext>) -> Result<Self, Box<TemplateError>> {
        let mut handlebars = Handlebars::new();
        Self::register_templates(&mut handlebars)?;

        let handlebars = Arc::new(Mutex::new(handlebars));

        if let Some(override_cx) = override_cx {
            Self::watch_fs_for_template_overrides(override_cx, handlebars.clone());
        }

        Ok(Self { handlebars })
    }

    fn watch_fs_for_template_overrides(
        PromptOverrideContext { dev_mode, fs, cx }: PromptOverrideContext,
        handlebars: Arc<Mutex<Handlebars<'static>>>,
    ) {
        cx.background_executor()
            .spawn(async move {
                let templates_dir = if dev_mode {
                    std::env::current_dir()
                        .ok()
                        .and_then(|pwd| {
                            let pwd_assets_prompts = pwd.join("assets").join("prompts");
                            pwd_assets_prompts.exists().then_some(pwd_assets_prompts)
                        })
                        .unwrap_or_else(|| paths::prompt_overrides_dir().clone())
                } else {
                    paths::prompt_overrides_dir().clone()
                };

                // Create the prompt templates directory if it doesn't exist
                if !fs.is_dir(&templates_dir).await {
                    if let Err(e) = fs.create_dir(&templates_dir).await {
                        log::error!("Failed to create prompt templates directory: {}", e);
                        return;
                    }
                }

                // Initial scan of the prompts directory
                if let Ok(mut entries) = fs.read_dir(&templates_dir).await {
                    while let Some(Ok(file_path)) = entries.next().await {
                        if file_path.to_string_lossy().ends_with(".hbs") {
                            if let Ok(content) = fs.load(&file_path).await {
                                let file_name = file_path.file_stem().unwrap().to_string_lossy();

                                match handlebars.lock().register_template_string(&file_name, content) {
                                    Ok(_) => {
                                        log::info!(
                                            "Successfully registered template override: {} ({})",
                                            file_name,
                                            file_path.display()
                                        );
                                    },
                                    Err(e) => {
                                        log::error!(
                                            "Failed to register template during initial scan: {} ({})",
                                            e,
                                            file_path.display()
                                        );
                                    },
                                }
                            }
                        }
                    }
                }

                // Watch for changes
                let (mut changes, watcher) = fs.watch(&templates_dir, Duration::from_secs(1)).await;
                while let Some(changed_paths) = changes.next().await {
                    for changed_path in changed_paths {
                        if changed_path.extension().map_or(false, |ext| ext == "hbs") {
                            log::info!("Reloading template: {}", changed_path.display());
                            if let Some(content) = fs.load(&changed_path).await.log_err() {
                                let file_name = changed_path.file_stem().unwrap().to_string_lossy();
                                let file_path = changed_path.to_string_lossy();
                                match handlebars.lock().register_template_string(&file_name, content) {
                                    Ok(_) => log::info!(
                                        "Successfully reloaded template: {} ({})",
                                        file_name,
                                        file_path
                                    ),
                                    Err(e) => log::error!(
                                        "Failed to register template: {} ({})",
                                        e,
                                        file_path
                                    ),
                                }
                            }
                        }
                    }
                }
                drop(watcher);
            })
            .detach();
    }

    fn register_templates(handlebars: &mut Handlebars) -> Result<(), Box<TemplateError>> {
        let mut register_template = |id: &str| {
            let prompt = Assets::get(&format!("prompts/{}.hbs", id))
                .unwrap_or_else(|| panic!("{} prompt template not found", id))
                .data;
            handlebars
                .register_template_string(id, String::from_utf8_lossy(&prompt))
                .map_err(Box::new)
        };

        register_template("content_prompt")?;
        register_template("terminal_assistant_prompt")?;
        register_template("edit_workflow")?;
        register_template("step_resolution")?;

        Ok(())
    }

    pub fn generate_content_prompt(
        &self,
        user_prompt: String,
        language_name: Option<&str>,
        buffer: BufferSnapshot,
        transform_range: Range<usize>,
        selected_ranges: Vec<Range<usize>>,
        transform_context_range: Range<usize>,
    ) -> Result<String, RenderError> {
        let content_type = match language_name {
            None | Some("Markdown" | "Plain Text") => "text",
            Some(_) => "code",
        };

        const MAX_CTX: usize = 50000;
        let mut is_truncated = false;

        let before_range = 0..transform_range.start;
        let truncated_before = if before_range.len() > MAX_CTX {
            is_truncated = true;
            transform_range.start - MAX_CTX..transform_range.start
        } else {
            before_range
        };

        let after_range = transform_range.end..buffer.len();
        let truncated_after = if after_range.len() > MAX_CTX {
            is_truncated = true;
            transform_range.end..transform_range.end + MAX_CTX
        } else {
            after_range
        };

        let mut document_content = String::new();
        for chunk in buffer.text_for_range(truncated_before) {
            document_content.push_str(chunk);
        }

        document_content.push_str("<rewrite_this>\n");
        for chunk in buffer.text_for_range(transform_range.clone()) {
            document_content.push_str(chunk);
        }
        document_content.push_str("\n</rewrite_this>");

        for chunk in buffer.text_for_range(truncated_after) {
            document_content.push_str(chunk);
        }

        let mut rewrite_section = String::new();
        for chunk in buffer.text_for_range(transform_range.clone()) {
            rewrite_section.push_str(chunk);
        }

        let mut rewrite_section_prefix = String::new();
        for chunk in buffer.text_for_range(transform_context_range.start..transform_range.start) {
            rewrite_section_prefix.push_str(chunk);
        }

        let mut rewrite_section_suffix = String::new();
        for chunk in buffer.text_for_range(transform_range.end..transform_context_range.end) {
            rewrite_section_suffix.push_str(chunk);
        }

        let rewrite_section_with_edits = {
            let mut section_with_selections = String::new();
            let mut last_end = 0;
            for selected_range in &selected_ranges {
                if selected_range.start > last_end {
                    section_with_selections.push_str(
                        &rewrite_section[last_end..selected_range.start - transform_range.start],
                    );
                }
                if selected_range.start == selected_range.end {
                    section_with_selections.push_str("<insert_here></insert_here>");
                } else {
                    section_with_selections.push_str("<edit_here>");
                    section_with_selections.push_str(
                        &rewrite_section[selected_range.start - transform_range.start
                            ..selected_range.end - transform_range.start],
                    );
                    section_with_selections.push_str("</edit_here>");
                }
                last_end = selected_range.end - transform_range.start;
            }
            if last_end < rewrite_section.len() {
                section_with_selections.push_str(&rewrite_section[last_end..]);
            }
            section_with_selections
        };

        let has_insertion = selected_ranges.iter().any(|range| range.start == range.end);
        let has_replacement = selected_ranges.iter().any(|range| range.start != range.end);

        let context = ContentPromptContext {
            content_type: content_type.to_string(),
            language_name: language_name.map(|s| s.to_string()),
            is_truncated,
            document_content,
            user_prompt,
            rewrite_section,
            rewrite_section_prefix,
            rewrite_section_suffix,
            rewrite_section_with_edits,
            has_insertion,
            has_replacement,
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

    pub fn generate_step_resolution_prompt(
        &self,
        context: &StepResolutionContext,
    ) -> Result<String, RenderError> {
        self.handlebars.lock().render("step_resolution", context)
    }
}
