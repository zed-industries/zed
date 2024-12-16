mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::sync::Arc;

use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, SharedString, Task, View,
    WeakModel, WeakView,
};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem, ListItemSpacing, Tooltip};
use util::ResultExt;
use workspace::Workspace;

use crate::context_picker::fetch_context_picker::FetchContextPicker;
use crate::context_picker::file_context_picker::FileContextPicker;
use crate::context_picker::thread_context_picker::ThreadContextPicker;
use crate::context_strip::ContextStrip;
use crate::thread_store::ThreadStore;

#[derive(Debug, Clone)]
enum ContextPickerMode {
    Default,
    File(View<FileContextPicker>),
    Fetch(View<FetchContextPicker>),
    Thread(View<ThreadContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerMode,
    picker: View<Picker<ContextPickerDelegate>>,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakView<Workspace>,
        thread_store: WeakModel<ThreadStore>,
        context_strip: WeakView<ContextStrip>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let delegate = ContextPickerDelegate {
            context_picker: cx.view().downgrade(),
            workspace,
            thread_store,
            context_strip,
            entries: vec![
                ContextPickerEntry {
                    name: "directory".into(),
                    description: "Insert any directory".into(),
                    icon: IconName::Folder,
                },
                ContextPickerEntry {
                    name: "file".into(),
                    description: "Insert any file".into(),
                    icon: IconName::File,
                },
                ContextPickerEntry {
                    name: "fetch".into(),
                    description: "Fetch content from URL".into(),
                    icon: IconName::Globe,
                },
                ContextPickerEntry {
                    name: "thread".into(),
                    description: "Insert any thread".into(),
                    icon: IconName::MessageBubbles,
                },
            ],
            selected_ix: 0,
        };

        let picker = cx.new_view(|cx| {
            Picker::nonsearchable_uniform_list(delegate, cx).max_height(Some(rems(20.).into()))
        });

        ContextPicker {
            mode: ContextPickerMode::Default,
            picker,
        }
    }

    pub fn reset_mode(&mut self) {
        self.mode = ContextPickerMode::Default;
    }
}

impl EventEmitter<DismissEvent> for ContextPicker {}

impl FocusableView for ContextPicker {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        match &self.mode {
            ContextPickerMode::Default => self.picker.focus_handle(cx),
            ContextPickerMode::File(file_picker) => file_picker.focus_handle(cx),
            ContextPickerMode::Fetch(fetch_picker) => fetch_picker.focus_handle(cx),
            ContextPickerMode::Thread(thread_picker) => thread_picker.focus_handle(cx),
        }
    }
}

impl Render for ContextPicker {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w(px(400.))
            .min_w(px(400.))
            .map(|parent| match &self.mode {
                ContextPickerMode::Default => parent.child(self.picker.clone()),
                ContextPickerMode::File(file_picker) => parent.child(file_picker.clone()),
                ContextPickerMode::Fetch(fetch_picker) => parent.child(fetch_picker.clone()),
                ContextPickerMode::Thread(thread_picker) => parent.child(thread_picker.clone()),
            })
    }
}

#[derive(Clone)]
struct ContextPickerEntry {
    name: SharedString,
    description: SharedString,
    icon: IconName,
}

pub(crate) struct ContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    thread_store: WeakModel<ThreadStore>,
    context_strip: WeakView<ContextStrip>,
    entries: Vec<ContextPickerEntry>,
    selected_ix: usize,
}

impl PickerDelegate for ContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_ix = ix.min(self.entries.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a context source…".into()
    }

    fn update_matches(&mut self, _query: String, _cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(entry) = self.entries.get(self.selected_ix) {
            self.context_picker
                .update(cx, |this, cx| {
                    match entry.name.to_string().as_str() {
                        "file" => {
                            this.mode = ContextPickerMode::File(cx.new_view(|cx| {
                                FileContextPicker::new(
                                    self.context_picker.clone(),
                                    self.workspace.clone(),
                                    self.context_strip.clone(),
                                    cx,
                                )
                            }));
                        }
                        "fetch" => {
                            this.mode = ContextPickerMode::Fetch(cx.new_view(|cx| {
                                FetchContextPicker::new(
                                    self.context_picker.clone(),
                                    self.workspace.clone(),
                                    self.context_strip.clone(),
                                    cx,
                                )
                            }));
                        }
                        "thread" => {
                            this.mode = ContextPickerMode::Thread(cx.new_view(|cx| {
                                ThreadContextPicker::new(
                                    self.thread_store.clone(),
                                    self.context_picker.clone(),
                                    self.context_strip.clone(),
                                    cx,
                                )
                            }));
                        }
                        _ => {}
                    }

                    cx.focus_self();
                })
                .log_err();
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| match this.mode {
                ContextPickerMode::Default => cx.emit(DismissEvent),
                ContextPickerMode::File(_)
                | ContextPickerMode::Fetch(_)
                | ContextPickerMode::Thread(_) => {}
            })
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = &self.entries[ix];

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Dense)
                .toggle_state(selected)
                .tooltip({
                    let description = entry.description.clone();
                    move |cx| cx.new_view(|_cx| Tooltip::new(description.clone())).into()
                })
                .child(
                    v_flex()
                        .group(format!("context-entry-label-{ix}"))
                        .w_full()
                        .py_0p5()
                        .min_w(px(250.))
                        .max_w(px(400.))
                        .child(
                            h_flex()
                                .gap_1p5()
                                .child(Icon::new(entry.icon).size(IconSize::XSmall))
                                .child(
                                    Label::new(entry.name.clone())
                                        .single_line()
                                        .size(LabelSize::Small),
                                ),
                        )
                        .child(
                            div().overflow_hidden().text_ellipsis().child(
                                Label::new(entry.description.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                        ),
                ),
        )
    }
}
