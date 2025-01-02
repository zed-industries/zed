mod directory_context_picker;
mod fetch_context_picker;
mod file_context_picker;
mod thread_context_picker;

use std::sync::Arc;

use gpui::{
    AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model, SharedString, Task,
    WeakModel, WeakModel,
};
use picker::{Picker, PickerDelegate};
use release_channel::ReleaseChannel;
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
    File(Model<FileContextPicker>),
    Directory(Model<DirectoryContextPicker>),
    Fetch(Model<FetchContextPicker>),
    Thread(Model<ThreadContextPicker>),
}

pub(super) struct ContextPicker {
    mode: ContextPickerMode,
    picker: Model<Picker<ContextPickerDelegate>>,
}

impl ContextPicker {
    pub fn new(
        workspace: WeakModel<Workspace>,
        thread_store: Option<WeakModel<ThreadStore>>,
        context_store: WeakModel<ContextStore>,
        confirm_behavior: ConfirmBehavior,
        window: &mut Window,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let mut entries = Vec::new();
        entries.push(ContextPickerEntry {
            name: "File".into(),
            kind: ContextKind::File,
            icon: IconName::File,
        });
        let release_channel = ReleaseChannel::global(cx);
        // The directory context picker isn't fully implemented yet, so limit it
        // to development builds.
        if release_channel == ReleaseChannel::Dev {
            entries.push(ContextPickerEntry {
                name: "Folder".into(),
                kind: ContextKind::Directory,
                icon: IconName::Folder,
            });
        }
        entries.push(ContextPickerEntry {
            name: "Fetch".into(),
            kind: ContextKind::FetchedUrl,
            icon: IconName::Globe,
        });

        if thread_store.is_some() {
            entries.push(ContextPickerEntry {
                name: "Thread".into(),
                kind: ContextKind::Thread,
                icon: IconName::MessageCircle,
            });
        }

        let delegate = ContextPickerDelegate {
            context_picker: cx.view().downgrade(),
            workspace,
            thread_store,
            context_store,
            confirm_behavior,
            entries,
            selected_ix: 0,
        };

        let picker = window.new_view(cx, |cx| {
            Picker::nonsearchable_uniform_list(delegate, window, cx)
                .max_height(Some(rems(20.).into()))
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
    fn render(&mut self, _window: &mut Window, _cx: &mut ModelContext<Self>) -> impl IntoElement {
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
    icon: IconName,
}

pub(crate) struct ContextPickerDelegate {
    context_picker: WeakModel<ContextPicker>,
    workspace: WeakModel<Workspace>,
    thread_store: Option<WeakModel<ThreadStore>>,
    context_store: WeakModel<ContextStore>,
    confirm_behavior: ConfirmBehavior,
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

    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut ModelContext<Picker<Self>>,
    ) {
        self.selected_ix = ix.min(self.entries.len().saturating_sub(1));
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut AppContext) -> Arc<str> {
        "Select a context source…".into()
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut ModelContext<Picker<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut ModelContext<Picker<Self>>,
    ) {
        if let Some(entry) = self.entries.get(self.selected_ix) {
            self.context_picker
                .update(cx, |this, cx| {
                    match entry.kind {
                        ContextKind::File => {
                            this.mode = ContextPickerMode::File(window.new_view(cx, |cx| {
                                FileContextPicker::new(
                                    self.context_picker.clone(),
                                    self.workspace.clone(),
                                    self.context_store.clone(),
                                    self.confirm_behavior,
                                    window,
                                    cx,
                                )
                            }));
                        }
                        ContextKind::Directory => {
                            this.mode = ContextPickerMode::Directory(window.new_view(cx, |cx| {
                                DirectoryContextPicker::new(
                                    self.context_picker.clone(),
                                    self.workspace.clone(),
                                    self.context_store.clone(),
                                    self.confirm_behavior,
                                    window,
                                    cx,
                                )
                            }));
                        }
                        ContextKind::FetchedUrl => {
                            this.mode = ContextPickerMode::Fetch(window.new_view(cx, |cx| {
                                FetchContextPicker::new(
                                    self.context_picker.clone(),
                                    self.workspace.clone(),
                                    self.context_store.clone(),
                                    self.confirm_behavior,
                                    window,
                                    cx,
                                )
                            }));
                        }
                        ContextKind::Thread => {
                            if let Some(thread_store) = self.thread_store.as_ref() {
                                this.mode = ContextPickerMode::Thread(window.new_view(cx, |cx| {
                                    ThreadContextPicker::new(
                                        thread_store.clone(),
                                        self.context_picker.clone(),
                                        self.context_store.clone(),
                                        self.confirm_behavior,
                                        window,
                                        cx,
                                    )
                                }));
                            }
                        }
                    }

                    cx.focus_self(window);
                })
                .log_err();
        }
    }

    fn dismissed(&mut self, window: &mut Window, cx: &mut ModelContext<Picker<Self>>) {
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
        _window: &mut Window,
        _cx: &mut ModelContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry = &self.entries[ix];

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
                        .child(Icon::new(entry.icon).size(IconSize::Small))
                        .child(Label::new(entry.name.clone()).single_line()),
                ),
        )
    }
}
