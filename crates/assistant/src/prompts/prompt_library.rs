use collections::HashMap;
use editor::Editor;
use fs::Fs;
use gpui::{prelude::FluentBuilder, *};
use language::language_settings;
use parking_lot::RwLock;
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, Checkbox, IconButtonShape, ListItem, ListItemSpacing};
use util::{ResultExt, TryFutureExt};
use uuid::Uuid;
use workspace::ModalView;

// actions!(prompt_manager, [NewPrompt, EditPrompt, SavePrompt]);

use super::custom_prompts::StaticPrompt;

pub struct PromptLibraryState {
    /// The default prompt all assistant contexts will start with
    _system_prompt: String,
    /// All [UserPrompt]s loaded into the library
    prompts: HashMap<PromptId, StaticPrompt>,
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
        let prompts = futures::executor::block_on(StaticPrompt::list(fs))?;
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
    pub fn prompts(&self) -> Vec<StaticPrompt> {
        let state = self.state.read();
        state.prompts.values().cloned().collect()
    }

    pub fn prompts_with_ids(&self) -> Vec<(PromptId, StaticPrompt)> {
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
            .and_then(|prompt| Some(prompt.to_str()))
    }

    pub fn _default_prompts(&self) -> Vec<StaticPrompt> {
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
    picker: View<Picker<PromptManagerDelegate>>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt_id: Option<PromptId>,
}

impl PromptManager {
    pub fn new(prompt_library: Arc<PromptLibrary>, cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let prompt_manager = cx.view().downgrade();
        let active_prompt_id = if prompt_library.prompts().is_empty() {
            None
        } else {
            Some(prompt_library.prompts_with_ids()[0].0)
        };
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                PromptManagerDelegate {
                    prompt_manager,
                    matching_prompts: vec![],
                    matching_prompt_ids: vec![],
                    prompt_library: prompt_library.clone(),
                    selected_index: 0,
                    // match_candidates: vec![],
                },
                cx,
            )
            .modal(false)
        });

        Self {
            focus_handle,
            prompt_library,
            picker,
            prompt_editors: HashMap::default(),
            active_prompt_id,
        }
    }

    pub fn set_active_prompt(&mut self, prompt_id: Option<PromptId>, cx: &mut ViewContext<Self>) {
        self.active_prompt_id = prompt_id;
        cx.notify();
    }

    pub fn selected_index(&self, cx: &ViewContext<Self>) -> usize {
        self.picker.read(cx).delegate.selected_index
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_no_prompts_state(&self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .child(Label::new("No prompts in library"))
            .child(
                Label::new("Add prompts to the prompts folder to get started.").color(Color::Muted),
            )
            // TODO: Add a button to open the prompts folder
            .child(Button::new("open-prompts-folder", "Open Prompts Folder"))
    }

    fn render_prompt_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("prompt-list")
            .bg(cx.theme().colors().surface_background)
            .h_full()
            .w_2_5()
            .child(
                h_flex()
                    .bg(cx.theme().colors().background)
                    .p(Spacing::Small.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .h_7()
                    .w_full()
                    .flex_none()
                    .justify_between()
                    .child(Label::new("Prompt Library").size(LabelSize::Small))
                    .child(IconButton::new("new-prompt", IconName::AtSign).disabled(true)),
            )
            .child(
                v_flex()
                    .h_full()
                    .flex_grow()
                    .justify_start()
                    .child(self.picker.clone()),
            )
    }

    fn render_prompt_item(
        &mut self,
        id: PromptId,
        prompt: StaticPrompt,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let prompt_library = self.prompt_library.clone();
        let prompt = prompt.clone();
        let prompt_id = id.clone();

        let default_prompt_ids = prompt_library.clone().default_prompt_ids();
        let is_default = default_prompt_ids.contains(&id);
        // We'll use this for conditionally enabled prompts
        // like those loaded only for certain languages
        let is_conditional = false;
        let selection = match (is_default, is_conditional) {
            (_, true) => Selection::Indeterminate,
            (true, _) => Selection::Selected,
            (false, _) => Selection::Unselected,
        };

        v_flex()
            .id(ElementId::Name(format!("prompt-{:?}", prompt_id,).into()))
            .p(Spacing::Small.rems(cx))
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap(Spacing::Large.rems(cx))
                            .child(
                                Checkbox::new(ElementId::from(prompt_id.0), selection).on_click(
                                    move |_, _cx| {
                                        if is_default {
                                            prompt_library
                                                .clone()
                                                .remove_prompt_from_default(prompt_id.clone())
                                                .log_err();
                                        } else {
                                            prompt_library
                                                .clone()
                                                .add_prompt_to_default(prompt_id.clone())
                                                .log_err();
                                        }
                                    },
                                ),
                            )
                            .child(Label::new(prompt.title)),
                    )
                    .child(div()),
            )
    }

    fn render_editor_for_prompt(
        &mut self,
        prompt_id: PromptId,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let prompt_library = self.prompt_library.clone();
        let editor_for_prompt = self.prompt_editors.entry(prompt_id).or_insert_with(|| {
            cx.new_view(|cx| {
                let mut editor = Editor::multi_line(cx);
                if let Some(prompt_text) = prompt_library.prompt_for_id(prompt_id) {
                    editor.set_text(prompt_text, cx);
                }
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_show_gutter(false, cx);
                editor
            })
        });
        editor_for_prompt.clone()
    }
}

impl Render for PromptManager {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .key_context("PromptManager")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .elevation_3(cx)
            .size_full()
            .flex_none()
            .w(rems(64.))
            .h(rems(40.))
            .overflow_hidden()
            .child(self.render_prompt_list(cx))
            .child(
                div().w_3_5().h_full().child(
                    v_flex()
                        .id("prompt-editor")
                        .border_l_1()
                        .border_color(cx.theme().colors().border)
                        .size_full()
                        .flex_none()
                        .min_w_64()
                        .h_full()
                        .child(
                            h_flex()
                                .bg(cx.theme().colors().background)
                                .p(Spacing::Small.rems(cx))
                                .border_b_1()
                                .border_color(cx.theme().colors().border)
                                .h_7()
                                .w_full()
                                .justify_between()
                                .child(div())
                                .child(
                                    IconButton::new("dismiss", IconName::Close)
                                        .shape(IconButtonShape::Square)
                                        .on_click(|_, cx| {
                                            cx.dispatch_action(menu::Cancel.boxed_clone());
                                        }),
                                ),
                        )
                        .when_some(self.active_prompt_id, |this, active_prompt_id| {
                            this.child(self.render_editor_for_prompt(active_prompt_id, cx))
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

pub struct PromptManagerDelegate {
    prompt_manager: WeakView<PromptManager>,
    matching_prompts: Vec<Arc<StaticPrompt>>,
    matching_prompt_ids: Vec<PromptId>,
    prompt_library: Arc<PromptLibrary>,
    selected_index: usize,
    // match_candidates: Vec<StringMatchCandidate>,
}

impl PickerDelegate for PromptManagerDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Find a prompt…".into()
    }

    fn match_count(&self) -> usize {
        self.matching_prompt_ids.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn selected_index_changed(
        &self,
        ix: usize,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut WindowContext) + 'static>> {
        let prompt_id = self.matching_prompt_ids.get(ix).copied()?;
        let prompt_manager = self.prompt_manager.upgrade()?;

        Some(Box::new(move |cx| {
            prompt_manager.update(cx, |manager, cx| {
                manager.set_active_prompt(Some(prompt_id), cx);
            })
        }))
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let prompt_library = self.prompt_library.clone();
        cx.spawn(|picker, mut cx| async move {
            async {
                let prompts = prompt_library.prompts_with_ids();
                let matching_prompts = prompts
                    .into_iter()
                    .filter(|(_, prompt)| {
                        prompt.title.to_lowercase().contains(&query.to_lowercase())
                    })
                    .collect::<Vec<_>>();
                picker.update(&mut cx, |picker, cx| {
                    picker.delegate.matching_prompt_ids =
                        matching_prompts.iter().map(|(id, _)| *id).collect();
                    picker.delegate.matching_prompts = matching_prompts
                        .into_iter()
                        .map(|(_, prompt)| Arc::new(prompt))
                        .collect();
                    cx.notify();
                })?;
                anyhow::Ok(())
            }
            .log_err()
            .await;
        })
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(prompt_id) = self.matching_prompt_ids.get(self.selected_index) {
            let prompt_manager = self.prompt_manager.upgrade().unwrap();
            prompt_manager.update(cx, move |manager, cx| {
                manager.set_active_prompt(Some(*prompt_id), cx);
            });
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.prompt_manager
            .update(cx, |_, cx| {
                cx.emit(DismissEvent);
            })
            .ok();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matching_prompt = self.matching_prompts.get(ix)?;
        let prompt = matching_prompt.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(prompt.title.clone())),
        )
    }
}

impl PromptManagerDelegate {
    fn prompt_for_index(
        &mut self,
        index: usize,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Arc<StaticPrompt>> {
        self.matching_prompts.get(index).cloned()
    }
}
