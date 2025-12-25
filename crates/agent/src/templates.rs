use anyhow::Result;
use fs::Fs;
use futures::StreamExt;
use gpui::{App, SharedString};
use handlebars::Handlebars;
use parking_lot::Mutex;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use util::ResultExt;

#[derive(RustEmbed)]
#[folder = "src/templates"]
#[include = "*.hbs"]
struct Assets;

pub struct Templates {
    handlebars: Arc<Mutex<Handlebars<'static>>>,
}

impl Templates {
    /// Creates a new Templates instance with built-in templates.
    /// This is a simplified version for tests or cases where overrides aren't needed.
    pub fn new() -> Arc<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_helper("contains", Box::new(contains));
        handlebars.register_embed_templates::<Assets>().unwrap();
        Arc::new(Self {
            handlebars: Arc::new(Mutex::new(handlebars)),
        })
    }

    /// Creates a new Templates instance that watches for filesystem overrides.
    pub fn with_overrides(repo_path: Option<&Path>, fs: Arc<dyn Fs>, cx: &mut App) -> Arc<Self> {
        let this = Self::new();
        let templates_dir = paths::prompt_overrides_dir(repo_path);

        cx.background_executor()
            .spawn({
                let handlebars = this.handlebars.clone();
                let fs = fs.clone();
                async move {
                    // Initial load
                    if fs.is_dir(&templates_dir).await {
                        Self::reload_overrides(&templates_dir, &fs, &handlebars).await;
                    }

                    // Setup watcher
                    if let Some(parent_dir) = templates_dir.parent() {
                        let (mut changes, _watcher) =
                            fs.watch(parent_dir, Duration::from_secs(1)).await;
                        while let Some(changed_paths) = changes.next().await {
                            if changed_paths
                                .iter()
                                .any(|p| p.path.starts_with(&templates_dir))
                            {
                                Self::reload_overrides(&templates_dir, &fs, &handlebars).await;
                            }
                        }
                    }
                }
            })
            .detach();

        this
    }

    async fn reload_overrides(
        templates_dir: &Path,
        fs: &Arc<dyn Fs>,
        handlebars: &Arc<Mutex<Handlebars<'static>>>,
    ) {
        if let Ok(mut entries) = fs.read_dir(templates_dir).await {
            while let Some(Ok(file_path)) = entries.next().await {
                if file_path.extension().is_some_and(|ext| ext == "hbs") {
                    if let Ok(content) = fs.load(&file_path).await {
                        // We use the full file name (e.g. "system_prompt.hbs") as the template name
                        // to match what TEMPLATE_NAME constants expect.
                        if let Some(file_name) = file_path.file_name().map(|n| n.to_string_lossy())
                        {
                            log::info!("Registering prompt template override: {}", file_name);
                            handlebars
                                .lock()
                                .register_template_string(&file_name, content)
                                .log_err();
                        }
                    }
                }
            }
        }
    }
}

pub trait Template: Sized {
    const TEMPLATE_NAME: &'static str;

    fn render(&self, templates: &Templates) -> Result<String>
    where
        Self: Serialize + Sized,
    {
        Ok(templates
            .handlebars
            .lock()
            .render(Self::TEMPLATE_NAME, self)?)
    }
}

#[derive(Serialize)]
pub struct SystemPromptTemplate<'a> {
    #[serde(flatten)]
    pub project: &'a prompt_store::ProjectContext,
    pub available_tools: Vec<SharedString>,
    pub model_name: Option<String>,
}

impl Template for SystemPromptTemplate<'_> {
    const TEMPLATE_NAME: &'static str = "system_prompt.hbs";
}

/// Handlebars helper for checking if an item is in a list
fn contains(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let list = h
        .param(0)
        .and_then(|v| v.value().as_array())
        .ok_or_else(|| {
            handlebars::RenderError::new("contains: missing or invalid list parameter")
        })?;
    let query = h.param(1).map(|v| v.value()).ok_or_else(|| {
        handlebars::RenderError::new("contains: missing or invalid query parameter")
    })?;

    if list.contains(query) {
        out.write("true")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_template() {
        let project = prompt_store::ProjectContext::default();
        let template = SystemPromptTemplate {
            project: &project,
            available_tools: vec!["echo".into()],
            model_name: Some("test-model".to_string()),
        };
        let templates = Templates::new();
        let rendered = template.render(&templates).unwrap();
        assert!(rendered.contains("## Fixing Diagnostics"));
        assert!(rendered.contains("test-model"));
    }
}
