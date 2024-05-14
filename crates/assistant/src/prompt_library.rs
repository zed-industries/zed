#![allow(unused, dead_code)]
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, Render};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use ui::{prelude::*, ModalHeader};
use util::paths::PROMPTS_DIR;
use workspace::ModalView;

pub struct PromptLibrary {
    prompts: HashMap<String, UserPrompt>,
    default_prompts: Vec<String>,
}

impl PromptLibrary {
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
            default_prompts: Vec::new(),
        }
    }

    pub fn load_prompts(&mut self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        let prompts = futures::executor::block_on(UserPrompt::list(fs))?;
        for prompt in prompts {
            let id = uuid::Uuid::new_v4().to_string();
            self.prompts.insert(id.clone(), prompt);
            // temp for testing, activate all prompts as they are loaded
            self.default_prompts.push(id);
        }
        Ok(())
    }

    pub fn default_prompt(&self) -> Option<String> {
        if self.default_prompts.is_empty() {
            None
        } else {
            Some(self.join_default_prompts())
        }
    }

    pub fn add_to_default_prompt(&mut self, prompt_ids: Vec<String>) -> anyhow::Result<()> {
        let ids_to_add: Vec<String> = prompt_ids
            .into_iter()
            .filter(|id| !self.default_prompts.contains(id) && self.prompts.contains_key(id))
            .collect();

        for id in ids_to_add {
            self.default_prompts.push(id);
        }

        Ok(())
    }

    pub fn remove_from_default_prompt(&mut self, prompt_id: String) -> anyhow::Result<()> {
        self.default_prompts.retain(|id| id != &prompt_id);
        Ok(())
    }

    pub fn join_default_prompts(&self) -> String {
        let active_prompt_ids = &self.default_prompts;

        active_prompt_ids
            .iter()
            .filter_map(|id| self.prompts.get(id).map(|p| p.content.clone()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PromptMetadata {
    title: String,
    author: String,
    #[serde(default)]
    languages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPrompt {
    metadata: PromptMetadata,
    content: String,
}

impl UserPrompt {
    fn parse_metadata(content: &str) -> anyhow::Result<(PromptMetadata, String)> {
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() >= 3 {
            let frontmatter_str = parts[1].trim();
            let metadata: PromptMetadata = serde_yml::from_str(frontmatter_str)
                .map_err(|e| anyhow::anyhow!("Failed to parse front matter: {}", e))?;

            let content_body = parts.get(2).map_or("", |s| *s).trim();

            Ok((metadata, content_body.to_string()))
        } else {
            Err(anyhow::anyhow!("Invalid or missing front matter"))
        }
    }

    async fn list(fs: Arc<dyn Fs>) -> anyhow::Result<Vec<Self>> {
        fs.create_dir(&PROMPTS_DIR).await?;

        let mut paths = fs.read_dir(&PROMPTS_DIR).await?;
        let mut prompts = Vec::new();

        while let Some(path_result) = paths.next().await {
            let path = match path_result {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error reading path: {:?}", e);
                    continue;
                }
            };

            if path.extension() == Some(std::ffi::OsStr::new("md")) {
                match fs.load(&path).await {
                    Ok(content) => match Self::parse_metadata(&content) {
                        Ok((metadata, content_body)) => prompts.push(UserPrompt {
                            metadata,
                            content: content_body,
                        }),
                        Err(e) => eprintln!("{}", e),
                    },
                    Err(e) => eprintln!("Failed to load file {}: {}", path.display(), e),
                }
            }
        }

        Ok(prompts)
    }
}

pub struct PromptManager {
    focus_handle: FocusHandle,
    prompt_library: Model<PromptLibrary>,
}

impl PromptManager {
    pub fn new(prompt_library: Model<PromptLibrary>, cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            prompt_library,
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Render for PromptManager {
    fn render(&mut self, cx: &mut ui::prelude::ViewContext<Self>) -> impl IntoElement {
        let prompts_map = self.prompt_library.read(cx).prompts.clone();
        let default_prompts = self.prompt_library.read(cx).default_prompts.clone();
        let prompts = prompts_map.into_iter().collect::<Vec<_>>();

        v_flex()
            .elevation_3(cx)
            .size_full()
            .flex_none()
            .w(rems(32.))
            .min_h(rems(1.))
            .child(ModalHeader::new("prompt-manager-header").child(Headline::new("Prompt Manager")))
            .child(
                v_flex()
                    .py(Spacing::Medium.rems(cx))
                    .px(Spacing::Large.rems(cx))
                    .when_else(
                        prompts.len() > 0,
                        |no_items| {
                            no_items.child(Label::new("No prompts").color(Color::Placeholder))
                        },
                        |with_items| {
                            with_items.children(prompts.into_iter().map(|(id, prompt)| {
                                let prompt = prompt.clone();
                                let prompt_id = id.clone();
                                let is_default = default_prompts.contains(&id);
                                let prompt_library = self.prompt_library.clone();

                                v_flex().p(Spacing::Small.rems(cx)).child(
                                    h_flex()
                                        .justify_between()
                                        .child(Label::new(prompt.metadata.title))
                                        .child(
                                            Button::new("add-prompt", "Add")
                                                .selected(is_default)
                                                .on_click(move |_, cx| {
                                                    // prompt_library
                                                    //     .read(cx)
                                                    //     .add_to_default_prompt(vec![prompt_id.clone()])
                                                    //     .unwrap();
                                                }),
                                        ),
                                )
                            }))
                        },
                    ),
            )
    }
}

impl EventEmitter<DismissEvent> for PromptManager {}
impl ModalView for PromptManager {}

impl FocusableView for PromptManager {
    fn focus_handle(&self, _cx: &AppContext) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
