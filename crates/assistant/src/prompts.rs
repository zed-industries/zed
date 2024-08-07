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

pub struct PromptBuilder {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

impl PromptBuilder {
    pub fn new(
        fs_and_cx: Option<(Arc<dyn Fs>, &gpui::AppContext)>,
    ) -> Result<Self, Box<TemplateError>> {
        let mut handlebars = Handlebars::new();
        Self::register_templates(&mut handlebars)?;

        let handlebars = Arc::new(Mutex::new(handlebars));

        if let Some((fs, cx)) = fs_and_cx {
            Self::watch_fs_for_template_overrides(fs, cx, handlebars.clone());
        }

        Ok(Self { handlebars })
    }

    fn watch_fs_for_template_overrides(
        fs: Arc<dyn Fs>,
        cx: &gpui::AppContext,
        handlebars: Arc<Mutex<Handlebars<'static>>>,
    ) {
        let templates_dir = paths::prompt_templates_dir();

        cx.background_executor()
            .spawn(async move {
                // Create the prompt templates directory if it doesn't exist
                if !fs.is_dir(templates_dir).await {
                    if let Err(e) = fs.create_dir(templates_dir).await {
                        log::error!("Failed to create prompt templates directory: {}", e);
                        return;
                    }
                }

                // Initial scan of the prompts directory
                if let Ok(mut entries) = fs.read_dir(templates_dir).await {
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
                let (mut changes, watcher) = fs.watch(templates_dir, Duration::from_secs(1)).await;
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

    pub fn generate_step_resolution_prompt(&self) -> Result<String, RenderError> {
        self.handlebars.lock().render("step_resolution", &())
    }
}
