#![allow(unused, dead_code)]
use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, Render};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use ui::{prelude::*, ModalHeader};
use util::paths::PROMPTS_DIR;
use workspace::ModalView;

pub struct PromptLibraryState {
    /// The default prompt all assistant contexts will start with
    system_prompt: String,
    /// All [UserPrompt]s loaded into the library
    prompts: HashMap<String, UserPrompt>,
    /// Prompts included in the default prompt
    default_prompts: Vec<String>,
    /// Prompts that are currently enabled. This is different from
    /// the default prompt in that it may change during a conversation
    enabled_prompts: Vec<String>,
    /// Prompts that have a pending update that hasn't been applied yet
    updateable_prompts: Vec<String>,
    /// Prompts that have been changed since they were loaded
    /// and can be reverted to their original state
    revertable_prompts: Vec<String>,
    version: usize,
}

pub struct PromptLibrary {
    state: RwLock<PromptLibraryState>,
}

impl Default for PromptLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl PromptLibrary {
    fn new() -> Self {
        Self {
            state: RwLock::new(PromptLibraryState {
                system_prompt: String::new(),
                prompts: HashMap::new(),
                default_prompts: Vec::new(),
                enabled_prompts: Vec::new(),
                updateable_prompts: Vec::new(),
                revertable_prompts: Vec::new(),
                version: 0,
            }),
        }
    }

    pub async fn init(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
        // -- debug --
        println!("Initializing prompt library");
        // -- /debug --
        let prompt_library = PromptLibrary::new();
        prompt_library.load_prompts(fs)?;
        // -- debug --
        println!(
            "Loaded {:?} prompts",
            prompt_library.state.read().prompts.len()
        );
        let prompts = prompt_library.state.read().prompts.clone();
        prompt_library.state.write().default_prompts = prompts.keys().cloned().collect();
        // -- /debug --
        Ok(prompt_library)
    }

    fn load_prompts(&self, fs: Arc<dyn Fs>) -> anyhow::Result<()> {
        let prompts = futures::executor::block_on(UserPrompt::list(fs))?;
        let prompts_with_ids = prompts
            .clone()
            .into_iter()
            .map(|prompt| {
                let id = uuid::Uuid::new_v4().to_string();
                (id, prompt)
            })
            .collect::<Vec<_>>();
        // -- debug --
        for (id, prompt) in &prompts_with_ids {
            log::info!("Loaded prompt: {} - {}", id, prompt.content);
        }
        // -- debug --
        let mut state = self.state.write();
        state.prompts.extend(prompts_with_ids);
        state.version += 1;

        Ok(())
    }

    pub fn default_prompt(&self) -> Option<String> {
        let mut state = self.state.read();

        if state.default_prompts.is_empty() {
            // -- debug --
            println!("No default prompts set");
            // -- debug --
            None
        } else {
            // -- debug --
            println!("Default prompts: {:?}", state.default_prompts);
            // -- debug --
            Some(self.join_default_prompts())
        }
    }

    pub fn add_to_default_prompt(&self, prompt_ids: Vec<String>) -> anyhow::Result<()> {
        let mut state = self.state.write();

        let ids_to_add: Vec<String> = prompt_ids
            .into_iter()
            .filter(|id| !state.default_prompts.contains(id) && state.prompts.contains_key(id))
            .collect();

        state.default_prompts.extend(ids_to_add);
        state.version += 1;

        Ok(())
    }

    pub fn remove_from_default_prompt(&self, prompt_id: String) -> anyhow::Result<()> {
        let mut state = self.state.write();

        state.default_prompts.retain(|id| id != &prompt_id);
        state.version += 1;
        Ok(())
    }

    fn join_default_prompts(&self) -> String {
        let state = self.state.read();
        let active_prompt_ids = state.default_prompts.iter().cloned().collect::<Vec<_>>();

        active_prompt_ids
            .iter()
            .filter_map(|id| state.prompts.get(id).map(|p| p.content.clone()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    pub fn prompts(&self) -> Vec<UserPrompt> {
        let state = self.state.read();
        state.prompts.values().cloned().collect()
    }

    pub fn default_prompts(&self) -> Vec<UserPrompt> {
        let state = self.state.read();
        state
            .default_prompts
            .iter()
            .filter_map(|id| state.prompts.get(id).map(|p| p.clone()))
            .collect()
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
    prompt_library: Arc<PromptLibrary>,
}

impl PromptManager {
    pub fn new(prompt_library: Arc<PromptLibrary>, cx: &mut WindowContext) -> Self {
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
        let prompts_state = self.prompt_library.state.read();

        let prompts = prompts_state
            .prompts
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        let default_prompts = prompts_state.default_prompts.clone();

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

                                v_flex().p(Spacing::Small.rems(cx)).child(
                                    h_flex()
                                        .justify_between()
                                        .child(Label::new(prompt.metadata.title))
                                        .child(
                                            Button::new("add-prompt", "Add")
                                                .selected(is_default)
                                                .on_click(move |_, cx| {
                                                    println!("Clicked {}!", prompt_id);
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
