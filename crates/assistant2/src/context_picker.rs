mod directory_context_picker;
mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::sync::Arc;

use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, SharedString, Task, View,
    WeakModel, WeakView,
};
use picker::{Picker, PickerDelegate};
use ui::{prelude::*, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::Workspace;

use crate::context::ContextKind;
use crate::context_picker::directory_context_picker::DirectoryContextPicker;
use crate::context_picker::fetch_context_picker::FetchContextPicker;
use crate::context_picker::file_context_picker::FileContextPicker;
use crate::context_picker::thread_context_picker::ThreadContextPicker;
use crate::context_store::ContextStore;
use crate::thread_store::ThreadStore;

#[derive(Debug, Clone, Copy)]
pub enum ConfirmBehavior {
    KeepOpen,
    Close,
}

#[derive(Debug, Clone)]
enum ContextPickerMode {
    Default,
    File(View<FileContextPicker>),
    Directory(View<DirectoryContextPicker>),
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
        thread_store: Option<WeakModel<ThreadStore>>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut recent = Vec::new();
        // TODO az
        recent.push(RecentContextPickerEntry {
            name: "Recent File 1".into(),
            kind: ContextKind::File,
        });
        recent.push(RecentContextPickerEntry {
            name: "Recent File 2".into(),
            kind: ContextKind::File,
        });
        recent.push(RecentContextPickerEntry {
            name: "Recent Thread 1".into(),
            kind: ContextKind::Thread,
        });
        recent.push(RecentContextPickerEntry {
            name: "Recent Thread 2".into(),
            kind: ContextKind::Thread,
        });

        let mut kinds = Vec::new();
        kinds.push(ContextPickerEntry {
            name: "File".into(),
            kind: ContextKind::File,
        });
        kinds.push(ContextPickerEntry {
            name: "Folder".into(),
            kind: ContextKind::Directory,
        });
        kinds.push(ContextPickerEntry {
            name: "Fetch".into(),
            kind: ContextKind::FetchedUrl,
        });

        if thread_store.is_some() {
            kinds.push(ContextPickerEntry {
                name: "Thread".into(),
                kind: ContextKind::Thread,
            });
        }

        let delegate = ContextPickerDelegate {
            context_picker: cx.view().downgrade(),
            workspace,
            thread_store,
            context_store,
            confirm_behavior,
            recent,
            kinds,
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
            ContextPickerMode::Directory(directory_picker) => directory_picker.focus_handle(cx),
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
                ContextPickerMode::Directory(directory_picker) => {
                    parent.child(directory_picker.clone())
                }
                ContextPickerMode::Fetch(fetch_picker) => parent.child(fetch_picker.clone()),
                ContextPickerMode::Thread(thread_picker) => parent.child(thread_picker.clone()),
            })
    }
}

#[derive(Clone)]
struct ContextPickerEntry {
    name: SharedString,
    kind: ContextKind,
}

struct RecentContextPickerEntry {
    name: SharedString,
    // TODO az considering refining
    kind: ContextKind,
}

pub(crate) struct ContextPickerDelegate {
    context_picker: WeakView<ContextPicker>,
    workspace: WeakView<Workspace>,
    thread_store: Option<WeakModel<ThreadStore>>,
    context_store: WeakModel<ContextStore>,
    confirm_behavior: ConfirmBehavior,
    recent: Vec<RecentContextPickerEntry>,
    kinds: Vec<ContextPickerEntry>,
    selected_ix: usize,
}

impl PickerDelegate for ContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.recent.len() + self.kinds.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_ix
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        if self.recent.is_empty() {
            vec![]
        } else {
            vec![self.recent.len() - 1]
        }
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_ix = ix.min(self.match_count().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select a context source…".into()
    }

    fn update_matches(&mut self, _query: String, _cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if self.selected_ix < self.recent.len() {
            todo!()
        } else {
            let selected_ix = self.selected_ix - self.recent.len();

            if let Some(entry) = self.kinds.get(selected_ix) {
                self.context_picker
                    .update(cx, |this, cx| {
                        match entry.kind {
                            ContextKind::File => {
                                this.mode = ContextPickerMode::File(cx.new_view(|cx| {
                                    FileContextPicker::new(
                                        self.context_picker.clone(),
                                        self.workspace.clone(),
                                        self.context_store.clone(),
                                        self.confirm_behavior,
                                        cx,
                                    )
                                }));
                            }
                            ContextKind::Directory => {
                                this.mode = ContextPickerMode::Directory(cx.new_view(|cx| {
                                    DirectoryContextPicker::new(
                                        self.context_picker.clone(),
                                        self.workspace.clone(),
                                        self.context_store.clone(),
                                        self.confirm_behavior,
                                        cx,
                                    )
                                }));
                            }
                            ContextKind::FetchedUrl => {
                                this.mode = ContextPickerMode::Fetch(cx.new_view(|cx| {
                                    FetchContextPicker::new(
                                        self.context_picker.clone(),
                                        self.workspace.clone(),
                                        self.context_store.clone(),
                                        self.confirm_behavior,
                                        cx,
                                    )
                                }));
                            }
                            ContextKind::Thread => {
                                if let Some(thread_store) = self.thread_store.as_ref() {
                                    this.mode = ContextPickerMode::Thread(cx.new_view(|cx| {
                                        ThreadContextPicker::new(
                                            thread_store.clone(),
                                            self.context_picker.clone(),
                                            self.context_store.clone(),
                                            self.confirm_behavior,
                                            cx,
                                        )
                                    }));
                                }
                            }
                        }

                        cx.focus_self();
                    })
                    .log_err();
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.context_picker
            .update(cx, |this, cx| match this.mode {
                ContextPickerMode::Default => cx.emit(DismissEvent),
                ContextPickerMode::File(_)
                | ContextPickerMode::Directory(_)
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
        if ix < self.recent.len() {
            let entry = &self.recent[ix];
            Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .toggle_state(selected)
                    .child(
                        h_flex()
                            .min_w(px(250.))
                            .max_w(px(400.))
                            .gap_2()
                            .child(Icon::new(entry.kind.icon()).size(IconSize::Small))
                            .child(Label::new(entry.name.clone()).single_line()),
                    ),
            )
        } else {
            let entry = &self.kinds[ix - self.recent.len()];
            Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Dense)
                    .toggle_state(selected)
                    .child(
                        h_flex()
                            .min_w(px(250.))
                            .max_w(px(400.))
                            .gap_2()
                            .child(Icon::new(entry.kind.icon()).size(IconSize::Small))
                            .child(Label::new(entry.name.clone()).single_line()),
                    ),
            )
        }
    }

    fn render_header(&self, _: &mut ViewContext<Picker<Self>>) -> Option<gpui::AnyElement> {
        if self.recent.is_empty() {
            None
        } else {
            Some(
                Label::new("Recent")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .mt_1()
                    .mb_0p5()
                    .ml_3()
                    .into_any_element(),
            )
        }
    }
}
