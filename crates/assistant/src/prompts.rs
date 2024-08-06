use assets::Assets;
use handlebars::Handlebars;
use language::BufferSnapshot;
use serde::Serialize;
use std::ops::Range;

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
    handlebars: Handlebars<'static>,
}

impl PromptBuilder {
    pub fn new() -> Result<Self, handlebars::TemplateError> {
        let mut handlebars = Handlebars::new();
        Self::register_templates(&mut handlebars)?;
        Ok(Self { handlebars })
    }

    fn register_templates(handlebars: &mut Handlebars) -> Result<(), handlebars::TemplateError> {
        let content_prompt = Assets::get("prompts/content_prompt.hbs")
            .expect("Content prompt template not found")
            .data;
        let terminal_assistant_prompt = Assets::get("prompts/terminal_assistant_prompt.hbs")
            .expect("Terminal assistant prompt template not found")
            .data;

        handlebars
            .register_template_string("content_prompt", String::from_utf8_lossy(&content_prompt))?;
        handlebars.register_template_string(
            "terminal_assistant_prompt",
            String::from_utf8_lossy(&terminal_assistant_prompt),
        )?;
        Ok(())
    }

    pub fn generate_content_prompt(
        &self,
        user_prompt: String,
        language_name: Option<&str>,
        buffer: BufferSnapshot,
        range: Range<usize>,
    ) -> Result<String, handlebars::RenderError> {
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

        self.handlebars.render("content_prompt", &context)
    }

    pub fn generate_terminal_assistant_prompt(
        &self,
        user_prompt: &str,
        shell: Option<&str>,
        working_directory: Option<&str>,
        latest_output: &[String],
    ) -> Result<String, handlebars::RenderError> {
        let context = TerminalAssistantPromptContext {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            shell: shell.map(|s| s.to_string()),
            working_directory: working_directory.map(|s| s.to_string()),
            latest_output: latest_output.to_vec(),
            user_prompt: user_prompt.to_string(),
        };

        self.handlebars
            .render("terminal_assistant_prompt", &context)
    }
}
