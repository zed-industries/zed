use collections::HashMap;
use editor::Editor;
use fs::Fs;
use gpui::{prelude::FluentBuilder, *};
use language::{language_settings, Buffer, LanguageRegistry};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, IconButtonShape, ListItem, ListItemSpacing};
use util::{ResultExt, TryFutureExt};
use workspace::ModalView;

use super::prompt_library::{PromptId, PromptLibrary};
use crate::prompts::prompt::StaticPrompt;

pub struct PromptManager {
    focus_handle: FocusHandle,
    prompt_library: Arc<PromptLibrary>,
    language_registry: Arc<LanguageRegistry>,
    #[allow(dead_code)]
    fs: Arc<dyn Fs>,
    picker: View<Picker<PromptManagerDelegate>>,
    prompt_editors: HashMap<PromptId, View<Editor>>,
    active_prompt_id: Option<PromptId>,
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
                },
                cx,
            )
            .max_height(rems(35.75))
            .modal(false)
        });

        let focus_handle = picker.focus_handle(cx);

        let mut manager = Self {
            focus_handle,
            prompt_library,
            language_registry,
            fs,
            picker,
            prompt_editors: HashMap::default(),
            active_prompt_id: None,
        };

        manager.active_prompt_id = manager.prompt_library.first_prompt_id();

        manager
    }

    pub fn set_active_prompt(&mut self, prompt_id: Option<PromptId>, cx: &mut ViewContext<Self>) {
        self.active_prompt_id = prompt_id;
        cx.notify();
    }

    pub fn focus_active_editor(&self, cx: &mut ViewContext<Self>) {
        if let Some(active_prompt_id) = self.active_prompt_id {
            if let Some(editor) = self.prompt_editors.get(&active_prompt_id) {
                let focus_handle = editor.focus_handle(cx);

                cx.focus(&focus_handle)
            }
        }
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
            .w_2_5()
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
                    .child(IconButton::new("new-prompt", IconName::Plus).disabled(true)),
            )
            .child(
                v_flex()
                    .h(rems(38.25))
                    .flex_grow()
                    .justify_start()
                    .child(picker),
            )
    }

    fn set_editor_for_prompt(
        &mut self,
        prompt_id: PromptId,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let prompt_library = self.prompt_library.clone();

        let editor_for_prompt = self.prompt_editors.entry(prompt_id).or_insert_with(|| {
            cx.new_view(|cx| {
                let text = if let Some(prompt) = prompt_library.prompt(prompt_id) {
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
}

impl Render for PromptManager {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .key_context("PromptManager")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::dismiss))
            // .on_action(cx.listener(Self::save_active_prompt))
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
                        .bg(cx.theme().colors().editor_background)
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
                let prompts = prompt_library.prompts();
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
        let matching_prompt = self.matching_prompts.get(ix)?;
        let prompt = matching_prompt.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(Label::new(prompt.title().unwrap_or_default().clone())),
        )
    }
}
