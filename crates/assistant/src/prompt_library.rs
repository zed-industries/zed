use fs::Fs;
use futures::StreamExt;
use gpui::{AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Render};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use ui::{prelude::*, Checkbox, ModalHeader};
use util::{paths::PROMPTS_DIR, ResultExt};
use workspace::ModalView;

pub struct PromptLibraryState {
    /// The default prompt all assistant contexts will start with
    _system_prompt: String,
    /// All [UserPrompt]s loaded into the library
    prompts: HashMap<String, UserPrompt>,
    /// Prompts included in the default prompt
    default_prompts: Vec<String>,
    /// Prompts that have a pending update that hasn't been applied yet
    _updateable_prompts: Vec<String>,
    /// Prompts that have been changed since they were loaded
    /// and can be reverted to their original state
    _revertable_prompts: Vec<String>,
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
                _system_prompt: String::new(),
                prompts: HashMap::new(),
                default_prompts: Vec::new(),
                _updateable_prompts: Vec::new(),
                _revertable_prompts: Vec::new(),
                version: 0,
            }),
        }
    }

    pub async fn init(fs: Arc<dyn Fs>) -> anyhow::Result<Self> {
        let prompt_library = PromptLibrary::new();
        prompt_library.load_prompts(fs)?;
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
        let mut state = self.state.write();
        state.prompts.extend(prompts_with_ids);
        state.version += 1;

        Ok(())
    }

    pub fn default_prompt(&self) -> Option<String> {
        let state = self.state.read();

        if state.default_prompts.is_empty() {
            None
        } else {
            Some(self.join_default_prompts())
        }
    }

    pub fn add_prompt_to_default(&self, prompt_id: String) -> anyhow::Result<()> {
        let mut state = self.state.write();

        if !state.default_prompts.contains(&prompt_id) && state.prompts.contains_key(&prompt_id) {
            state.default_prompts.push(prompt_id);
            state.version += 1;
        }

        Ok(())
    }

    pub fn remove_prompt_from_default(&self, prompt_id: String) -> anyhow::Result<()> {
        let mut state = self.state.write();

        state.default_prompts.retain(|id| id != &prompt_id);
        state.version += 1;
        Ok(())
    }

    fn join_default_prompts(&self) -> String {
        let state = self.state.read();
        let active_prompt_ids = state.default_prompts.to_vec();

        active_prompt_ids
            .iter()
            .filter_map(|id| state.prompts.get(id).map(|p| p.prompt.clone()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    #[allow(unused)]
    pub fn prompts(&self) -> Vec<UserPrompt> {
        let state = self.state.read();
        state.prompts.values().cloned().collect()
    }

    pub fn prompts_with_ids(&self) -> Vec<(String, UserPrompt)> {
        let state = self.state.read();
        state
            .prompts
            .iter()
            .map(|(id, prompt)| (id.clone(), prompt.clone()))
            .collect()
    }

    pub fn _default_prompts(&self) -> Vec<UserPrompt> {
        let state = self.state.read();
        state
            .default_prompts
            .iter()
            .filter_map(|id| state.prompts.get(id).cloned())
            .collect()
    }

    pub fn default_prompt_ids(&self) -> Vec<String> {
        let state = self.state.read();
        state.default_prompts.clone()
    }
}

/// A custom prompt that can be loaded into the prompt library
///
/// Example:
///
/// ```json
/// {
///   "title": "Foo",
///   "version": "1.0",
///   "author": "Jane Kim <jane@kim.com>",
///   "languages": ["*"], // or ["rust", "python", "javascript"] etc...
///   "prompt": "bar"
/// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserPrompt {
    version: String,
    title: String,
    author: String,
    languages: Vec<String>,
    prompt: String,
}

impl UserPrompt {
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

            if path.extension() == Some(std::ffi::OsStr::new("json")) {
                match fs.load(&path).await {
                    Ok(content) => {
                        let user_prompt: UserPrompt =
                            serde_json::from_str(&content).map_err(|e| {
                                anyhow::anyhow!("Failed to deserialize UserPrompt: {}", e)
                            })?;

                        prompts.push(user_prompt);
                    }
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
    active_prompt: Option<String>,
}

impl PromptManager {
    pub fn new(prompt_library: Arc<PromptLibrary>, cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            prompt_library,
            active_prompt: None,
        }
    }

    pub fn set_active_prompt(&mut self, prompt_id: Option<String>) {
        self.active_prompt = prompt_id;
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}

impl Render for PromptManager {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let prompt_library = self.prompt_library.clone();
        let prompts = prompt_library
            .clone()
            .prompts_with_ids()
            .clone()
            .into_iter()
            .collect::<Vec<_>>();

        let active_prompt = self.active_prompt.as_ref().and_then(|id| {
            prompt_library
                .prompts_with_ids()
                .iter()
                .find(|(prompt_id, _)| prompt_id == id)
                .map(|(_, prompt)| prompt.clone())
        });

        v_flex()
            .key_context("PromptManager")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .elevation_3(cx)
            .size_full()
            .flex_none()
            .w(rems(54.))
            .h(rems(40.))
            .overflow_hidden()
            .child(
                ModalHeader::new("prompt-manager-header")
                    .child(Headline::new("Prompt Library").size(HeadlineSize::Small))
                    .show_dismiss_button(true),
            )
            .child(
                h_flex()
                    .flex_grow()
                    .overflow_hidden()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .id("prompt-preview")
                            .overflow_y_scroll()
                            .h_full()
                            .min_w_64()
                            .max_w_1_2()
                            .child(
                                v_flex()
                                    .justify_start()
                                    .py(Spacing::Medium.rems(cx))
                                    .px(Spacing::Large.rems(cx))
                                    .bg(cx.theme().colors().surface_background)
                                    .when_else(
                                        !prompts.is_empty(),
                                        |with_items| {
                                            with_items.children(prompts.into_iter().map(
                                                |(id, prompt)| {
                                                    let prompt_library = prompt_library.clone();
                                                    let prompt = prompt.clone();
                                                    let prompt_id = id.clone();
                                                    let shared_string_id: SharedString =
                                                        id.clone().into();

                                                    let default_prompt_ids =
                                                        prompt_library.clone().default_prompt_ids();
                                                    let is_default =
                                                        default_prompt_ids.contains(&id);
                                                    // We'll use this for conditionally enabled prompts
                                                    // like those loaded only for certain languages
                                                    let is_conditional = false;
                                                    let selection =
                                                        match (is_default, is_conditional) {
                                                            (_, true) => Selection::Indeterminate,
                                                            (true, _) => Selection::Selected,
                                                            (false, _) => Selection::Unselected,
                                                        };

                                                    v_flex()
                                                    .id(ElementId::Name(
                                                        format!("prompt-{}", shared_string_id)
                                                            .into(),
                                                    ))
                                                    .p(Spacing::Small.rems(cx))

                                                    .on_click(cx.listener({
                                                        let prompt_id = prompt_id.clone();
                                                        move |this, _event, _cx| {
                                                            this.set_active_prompt(Some(
                                                                prompt_id.clone(),
                                                            ));
                                                        }
                                                    }))
                                                    .child(
                                                        h_flex()
                                                            .justify_between()
                                                            .child(
                                                                h_flex()
                                                                    .gap(Spacing::Large.rems(cx))
                                                                    .child(
                                                                        Checkbox::new(
                                                                            shared_string_id,
                                                                            selection,
                                                                        )
                                                                        .on_click(move |_, _cx| {
                                                                            if is_default {
                                                                                prompt_library
                                                                        .clone()
                                                                        .remove_prompt_from_default(
                                                                            prompt_id.clone(),
                                                                        )
                                                                        .log_err();
                                                                            } else {
                                                                                prompt_library
                                                                            .clone()
                                                                            .add_prompt_to_default(
                                                                                prompt_id.clone(),
                                                                            )
                                                                            .log_err();
                                                                            }
                                                                        }),
                                                                    )
                                                                    .child(Label::new(
                                                                        prompt.title,
                                                                    )),
                                                            )
                                                            .child(div()),
                                                    )
                                                },
                                            ))
                                        },
                                        |no_items| {
                                            no_items.child(
                                                Label::new("No prompts").color(Color::Placeholder),
                                            )
                                        },
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("prompt-preview")
                            .overflow_y_scroll()
                            .border_l_1()
                            .border_color(cx.theme().colors().border)
                            .size_full()
                            .flex_none()
                            .child(
                                v_flex()
                                    .justify_start()
                                    .py(Spacing::Medium.rems(cx))
                                    .px(Spacing::Large.rems(cx))
                                    .gap(Spacing::Large.rems(cx))
                                    .when_else(
                                        active_prompt.is_some(),
                                        |with_prompt| {
                                            let active_prompt = active_prompt.as_ref().unwrap();
                                            with_prompt
                                                .child(
                                                    v_flex()
                                                        .gap_0p5()
                                                        .child(
                                                            Headline::new(
                                                                active_prompt.title.clone(),
                                                            )
                                                            .size(HeadlineSize::XSmall),
                                                        )
                                                        .child(
                                                            h_flex()
                                                                .child(
                                                                    Label::new(
                                                                        active_prompt
                                                                            .author
                                                                            .clone(),
                                                                    )
                                                                    .size(LabelSize::XSmall)
                                                                    .color(Color::Muted),
                                                                )
                                                                .child(
                                                                    Label::new(
                                                                        if active_prompt
                                                                            .languages
                                                                            .is_empty()
                                                                            || active_prompt
                                                                                .languages[0]
                                                                                == "*"
                                                                        {
                                                                            " · Global".to_string()
                                                                        } else {
                                                                            format!(
                                                                                " · {}",
                                                                                active_prompt
                                                                                    .languages
                                                                                    .join(", ")
                                                                            )
                                                                        },
                                                                    )
                                                                    .size(LabelSize::XSmall)
                                                                    .color(Color::Muted),
                                                                ),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .max_w(rems(30.))
                                                        .text_ui(cx)
                                                        .child(active_prompt.prompt.clone()),
                                                )
                                        },
                                        |without_prompt| {
                                            without_prompt.justify_center().items_center().child(
                                                Label::new("Select a prompt to view details.")
                                                    .color(Color::Placeholder),
                                            )
                                        },
                                    ),
                            ),
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
