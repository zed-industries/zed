use collections::HashMap;
use editor::{Editor, EditorEvent};
use fs::Fs;
use gpui::{prelude::FluentBuilder, *};
use language::{language_settings, Buffer, LanguageRegistry};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, IconButtonShape, Indicator, ListItem, ListItemSpacing, Tooltip};
use util::{ResultExt, TryFutureExt};
use workspace::ModalView;

use crate::prompts::{PromptId, PromptLibrary, SortOrder, StaticPrompt, PROMPT_DEFAULT_TITLE};

actions!(prompt_manager, [NewPrompt, SavePrompt]);

pub struct PromptManager {
    focus_handle: FocusHandle,
    prompt_library: Arc<PromptLibrary>,
    language_registry: Arc<LanguageRegistry>,
    #[allow(dead_code)]
    fs: Arc<dyn Fs>,
    picker: View<Picker<PromptManagerDelegate>>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt_id: Option<PromptId>,
    last_new_prompt_id: Option<PromptId>,
    _subscriptions: Vec<Subscription>,
}

impl PromptManager {
    pub fn new(
        prompt_library: Arc<PromptLibrary>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let prompt_manager = cx.view().downgrade();
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(
                PromptManagerDelegate {
                    prompt_manager,
                    matching_prompts: vec![],
                    matching_prompt_ids: vec![],
                    prompt_library: prompt_library.clone(),
                    selected_index: 0,
                    _subscriptions: vec![],
                },
                cx,
            )
            .max_height(rems(35.75))
            .modal(false)
        });

        let focus_handle = picker.focus_handle(cx);

        let subscriptions = vec![
            // cx.on_focus_in(&focus_handle, Self::focus_in),
            // cx.on_focus_out(&focus_handle, Self::focus_out),
        ];

        let mut manager = Self {
            focus_handle,
            prompt_library,
            language_registry,
            fs,
            picker,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
            last_new_prompt_id: None,
            _subscriptions: subscriptions,
        };

        manager.active_prompt_id = manager.prompt_library.first_prompt_id();

        manager
    }

    fn dispatch_context(&self, cx: &ViewContext<Self>) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("PromptManager");

        let identifier = match self.active_editor() {
            Some(active_editor) if active_editor.focus_handle(cx).is_focused(cx) => "editing",
            _ => "not_editing",
        };

        dispatch_context.add(identifier);
        dispatch_context
    }

    pub fn new_prompt(&mut self, _: &NewPrompt, cx: &mut ViewContext<Self>) {
        // TODO: Why doesn't this prevent making a new prompt if you
        // move the picker selection/maybe unfocus the editor?

        // Prevent making a new prompt if the last new prompt is still empty
        //
        // Instead, we'll focus the last new prompt
        if let Some(last_new_prompt_id) = self.last_new_prompt_id() {
            if let Some(last_new_prompt) = self.prompt_library.prompt_by_id(last_new_prompt_id) {
                let normalized_body = last_new_prompt
                    .body()
                    .trim()
                    .replace(['\r', '\n'], "")
                    .to_string();

                if last_new_prompt.title() == PROMPT_DEFAULT_TITLE && normalized_body.is_empty() {
                    self.set_editor_for_prompt(last_new_prompt_id, cx);
                    self.focus_active_editor(cx);
                }
            }
        }

        let prompt = self.prompt_library.new_prompt();
        self.set_last_new_prompt_id(Some(prompt.id().to_owned()));

        self.prompt_library.add_prompt(prompt.clone());

        let id = *prompt.id();
        self.picker.update(cx, |picker, _cx| {
            let prompts = self
                .prompt_library
                .sorted_prompts(SortOrder::Alphabetical)
                .clone()
                .into_iter();

            picker.delegate.prompt_library = self.prompt_library.clone();
            picker.delegate.matching_prompts = prompts.clone().map(|(_, p)| Arc::new(p)).collect();
            picker.delegate.matching_prompt_ids = prompts.map(|(id, _)| id).collect();
            picker.delegate.selected_index = picker
                .delegate
                .matching_prompts
                .iter()
                .position(|p| p.id() == &id)
                .unwrap_or(0);
        });

        self.active_prompt_id = Some(id);

        cx.notify();
    }

    pub fn save_prompt(
        &mut self,
        fs: Arc<dyn Fs>,
        prompt_id: PromptId,
        new_content: String,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        let library = self.prompt_library.clone();
        if library.prompt_by_id(prompt_id).is_some() {
            cx.spawn(|_, _| async move {
                library
                    .save_prompt(prompt_id, Some(new_content), fs)
                    .log_err()
                    .await;
            })
            .detach();
            cx.notify();
        }

        Ok(())
    }

    pub fn set_active_prompt(&mut self, prompt_id: Option<PromptId>, cx: &mut ViewContext<Self>) {
        self.active_prompt_id = prompt_id;
        cx.notify();
    }

    pub fn last_new_prompt_id(&self) -> Option<PromptId> {
        self.last_new_prompt_id
    }

    pub fn set_last_new_prompt_id(&mut self, id: Option<PromptId>) {
        self.last_new_prompt_id = id;
    }

    pub fn focus_active_editor(&self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            if let Some(editor) = self.prompt_editors.get(&active_prompt_id) {
                let focus_handle = editor.focus_handle(cx);

                cx.focus(&focus_handle)
            }
        }
    }

    pub fn active_editor(&self) -> Option<&View<Editor>> {
        self.active_prompt_id
            .and_then(|active_prompt_id| self.prompt_editors.get(&active_prompt_id))
    }

    fn set_editor_for_prompt(
        &mut self,
        prompt_id: PromptId,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let prompt_library = self.prompt_library.clone();

        let editor_for_prompt = self.prompt_editors.entry(prompt_id).or_insert_with(|| {
            cx.new_view(|cx| {
                let text = if let Some(prompt) = prompt_library.prompt_by_id(prompt_id) {
                    prompt.content().to_owned()
                } else {
                    "".to_string()
                };

                let buffer = cx.new_model(|cx| {
                    let mut buffer = Buffer::local(text, cx);
                    let markdown = self.language_registry.language_for_name("Markdown");
                    cx.spawn(|buffer, mut cx| async move {
                        if let Some(markdown) = markdown.await.log_err() {
                            _ = buffer.update(&mut cx, |buffer, cx| {
                                buffer.set_language(Some(markdown), cx);
                            });
                        }
                    })
                    .detach();
                    buffer.set_language_registry(self.language_registry.clone());
                    buffer
                });
                let mut editor = Editor::for_buffer(buffer, None, cx);
                editor.set_soft_wrap_mode(language_settings::SoftWrap::EditorWidth, cx);
                editor.set_show_gutter(false, cx);
                editor
            })
        });

        editor_for_prompt.clone()
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_prompt_list(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let picker = self.picker.clone();

        v_flex()
            .id("prompt-list")
            .bg(cx.theme().colors().surface_background)
            .h_full()
            .w_1_3()
            .overflow_hidden()
            .child(
                h_flex()
                    .bg(cx.theme().colors().background)
                    .p(Spacing::Small.rems(cx))
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .h(rems(1.75))
                    .w_full()
                    .flex_none()
                    .justify_between()
                    .child(Label::new("Prompt Library").size(LabelSize::Small))
                    .child(
                        IconButton::new("new-prompt", IconName::Plus)
                            .shape(IconButtonShape::Square)
                            .tooltip(move |cx| Tooltip::text("New Prompt", cx))
                            .on_click(|_, cx| {
                                cx.dispatch_action(NewPrompt.boxed_clone());
                            }),
                    ),
            )
            .child(
                v_flex()
                    .h(rems(38.25))
                    .flex_grow()
                    .justify_start()
                    .child(picker),
            )
    }
}

impl Render for PromptManager {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_prompt_id = self.active_prompt_id;
        let active_prompt = if let Some(active_prompt_id) = active_prompt_id {
            self.prompt_library.clone().prompt_by_id(active_prompt_id)
        } else {
            None
        };
        let active_editor = self.active_editor().map(|editor| editor.clone());
        let updated_content = if let Some(editor) = active_editor {
            Some(editor.read(cx).text(cx))
        } else {
            None
        };
        let can_save = active_prompt_id.is_some() && updated_content.is_some();
        let fs = self.fs.clone();

        h_flex()
            .id("prompt-manager")
            .key_context(self.dispatch_context(cx))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::new_prompt))
            .elevation_3(cx)
            .size_full()
            .flex_none()
            .w(rems(64.))
            .h(rems(40.))
            .overflow_hidden()
            .child(self.render_prompt_list(cx))
            .child(
                div().w_2_3().h_full().child(
                    v_flex()
                        .id("prompt-editor")
                        .border_l_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .size_full()
                        .flex_none()
                        .min_w_64()
                        .h_full()
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .bg(cx.theme().colors().background)
                                .p(Spacing::Small.rems(cx))
                                .border_b_1()
                                .border_color(cx.theme().colors().border)
                                .h_7()
                                .w_full()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .gap(Spacing::XXLarge.rems(cx))
                                        .child(if can_save {
                                            IconButton::new("save", IconName::Save)
                                                .shape(IconButtonShape::Square)
                                                .tooltip(move |cx| Tooltip::text("Save Prompt", cx))
                                                .on_click(cx.listener(move |this, _event, cx| {
                                                    if let Some(prompt_id) = active_prompt_id {
                                                        this.save_prompt(
                                                            fs.clone(),
                                                            prompt_id,
                                                            updated_content.clone().unwrap_or(
                                                                "TODO: make unreachable"
                                                                    .to_string(),
                                                            ),
                                                            cx,
                                                        )
                                                        .log_err();
                                                    }
                                                }))
                                        } else {
                                            IconButton::new("save", IconName::Save)
                                                .shape(IconButtonShape::Square)
                                                .disabled(true)
                                        })
                                        .when_some(active_prompt, |this, active_prompt| {
                                            let path = active_prompt.path();

                                            this.child(
                                                IconButton::new("reveal", IconName::Reveal)
                                                    .shape(IconButtonShape::Square)
                                                    .disabled(path.is_none())
                                                    .tooltip(move |cx| {
                                                        Tooltip::text("Reveal in Finder", cx)
                                                    })
                                                    .on_click(cx.listener(move |_, _event, cx| {
                                                        if let Some(path) = path.clone() {
                                                            cx.reveal_path(&path);
                                                        }
                                                    })),
                                            )
                                        }),
                                )
                                .child(
                                    IconButton::new("dismiss", IconName::Close)
                                        .shape(IconButtonShape::Square)
                                        .tooltip(move |cx| Tooltip::text("Close", cx))
                                        .on_click(|_, cx| {
                                            cx.dispatch_action(menu::Cancel.boxed_clone());
                                        }),
                                ),
                        )
                        .when_some(active_prompt_id, |this, active_prompt_id| {
                            this.child(
                                h_flex()
                                    .flex_1()
                                    .w_full()
                                    .py(Spacing::Large.rems(cx))
                                    .px(Spacing::XLarge.rems(cx))
                                    .child(self.set_editor_for_prompt(active_prompt_id, cx)),
                            )
                        }),
                ),
            )
    }
}

impl EventEmitter<DismissEvent> for PromptManager {}
impl EventEmitter<EditorEvent> for PromptManager {}

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
    _subscriptions: Vec<Subscription>,
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
                let prompts = prompt_library.sorted_prompts(SortOrder::Alphabetical);
                let matching_prompts = prompts
                    .into_iter()
                    .filter(|(_, prompt)| {
                        prompt
                            .content()
                            .to_lowercase()
                            .contains(&query.to_lowercase())
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
        let prompt_manager = self.prompt_manager.upgrade().unwrap();
        prompt_manager.update(cx, move |manager, cx| manager.focus_active_editor(cx));
    }

    fn should_dismiss(&self) -> bool {
        false
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
        let prompt = self.matching_prompts.get(ix)?;

        let is_diry = self.prompt_library.is_dirty(prompt.id());

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(prompt.title()))
                .end_slot(div().when(is_diry, |this| this.child(Indicator::dot()))),
        )
    }
}
