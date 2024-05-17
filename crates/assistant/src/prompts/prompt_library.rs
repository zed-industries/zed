use collections::HashMap;
use editor::Editor;
use fs::Fs;
use gpui::{AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Render, View};
use parking_lot::RwLock;
use std::sync::Arc;
use ui::{prelude::*, Checkbox, ModalHeader};
use util::ResultExt;
use uuid::Uuid;
use workspace::ModalView;

use super::custom_prompts::CustomPrompt;

pub struct PromptLibraryState {
    /// The default prompt all assistant contexts will start with
    _system_prompt: String,
    /// All [UserPrompt]s loaded into the library
    prompts: HashMap<PromptId, CustomPrompt>,
    /// Prompts included in the default prompt
    default_prompts: Vec<PromptId>,
    /// Prompts that have a pending update that hasn't been applied yet
    _updateable_prompts: Vec<PromptId>,
    /// Prompts that have been changed since they were loaded
    /// and can be reverted to their original state
    _revertable_prompts: Vec<PromptId>,
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
                prompts: HashMap::default(),
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
        let prompts = futures::executor::block_on(CustomPrompt::list(fs))?;
        let prompts_with_ids = prompts
            .clone()
            .into_iter()
            .map(|prompt| {
                let id = PromptId(uuid::Uuid::new_v4());
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

    pub fn add_prompt_to_default(&self, prompt_id: PromptId) -> anyhow::Result<()> {
        let mut state = self.state.write();

        if !state.default_prompts.contains(&prompt_id) && state.prompts.contains_key(&prompt_id) {
            state.default_prompts.push(prompt_id);
            state.version += 1;
        }

        Ok(())
    }

    pub fn remove_prompt_from_default(&self, prompt_id: PromptId) -> anyhow::Result<()> {
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
            .filter_map(|id| state.prompts.get(id).map(|p| p.body.clone()))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    #[allow(unused)]
    pub fn prompts(&self) -> Vec<CustomPrompt> {
        let state = self.state.read();
        state.prompts.values().cloned().collect()
    }

    pub fn prompts_with_ids(&self) -> Vec<(PromptId, CustomPrompt)> {
        let state = self.state.read();
        state
            .prompts
            .iter()
            .map(|(id, prompt)| (id.clone(), prompt.clone()))
            .collect()
    }

    pub fn prompt_for_id(&self, prompt_id: PromptId) -> Option<String> {
        self.state
            .read()
            .prompts
            .get(&prompt_id)
            .and_then(|prompt| serde_json::to_string_pretty(prompt).log_err())
    }

    pub fn _default_prompts(&self) -> Vec<CustomPrompt> {
        let state = self.state.read();
        state
            .default_prompts
            .iter()
            .filter_map(|id| state.prompts.get(id).cloned())
            .collect()
    }

    pub fn default_prompt_ids(&self) -> Vec<PromptId> {
        let state = self.state.read();
        state.default_prompts.clone()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct PromptId(Uuid);

pub struct PromptManager {
    focus_handle: FocusHandle,
    prompt_library: Arc<PromptLibrary>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt_id: Option<PromptId>,
}

impl PromptManager {
    pub fn new(prompt_library: Arc<PromptLibrary>, cx: &mut WindowContext) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            focus_handle,
            prompt_library,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
        }
    }

    pub fn set_active_prompt(&mut self, prompt_id: Option<PromptId>, cx: &mut ViewContext<Self>) {
        dbg!();

        self.active_prompt_id = prompt_id;
        cx.notify();
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
                                                        format!("prompt-{:?}", prompt_id,).into(),
                                                    ))
                                                    .p(Spacing::Small.rems(cx))
                                                    .on_click(cx.listener({
                                                        let prompt_id = prompt_id.clone();
                                                        move |this, _event, cx| {
                                                            this.set_active_prompt(
                                                                Some(prompt_id),
                                                                cx,
                                                            );
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
                                                                            ElementId::from(
                                                                                prompt_id.0,
                                                                            ),
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
                            .min_w_64()
                            .h_full()
                            .debug_bg_green()
                            .when_some(self.active_prompt_id, |this, active_prompt_id| {
                                dbg!();

                                let editor_for_prompt = self
                                    .prompt_editors
                                    .entry(active_prompt_id)
                                    .or_insert_with(|| {
                                        cx.new_view(|cx| {
                                            let mut editor = Editor::multi_line(cx);
                                            if let Some(prompt_text) =
                                                prompt_library.prompt_for_id(active_prompt_id)
                                            {
                                                editor.set_text(prompt_text, cx);
                                            }
                                            editor
                                        })
                                    });
                                this.child(editor_for_prompt.clone())
                            }),
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
