use anyhow::Result;
use editor::Editor;
use encodings::Encoding;
use encodings::EncodingOptions;
use futures::channel::oneshot;
use gpui::ParentElement;
use gpui::Task;
use language::Buffer;
use picker::Picker;
use picker::PickerDelegate;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use ui::Label;
use ui::ListItemSpacing;
use ui::rems;
use util::ResultExt;

use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{DismissEvent, Entity, WeakEntity};

use ui::{Context, HighlightedLabel, ListItem, Window};
use workspace::Workspace;

pub fn save_or_reopen(
    buffer: Entity<Buffer>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let weak_workspace = cx.weak_entity();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = EncodingSaveOrReopenDelegate::new(buffer, weak_workspace);
        Picker::nonsearchable_uniform_list(delegate, window, cx)
            .modal(true)
            .width(rems(34.0))
    })
}

pub fn open_with_encoding(
    path: Arc<Path>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) -> Task<Result<()>> {
    let (tx, rx) = oneshot::channel();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = EncodingSelectorDelegate::new(None, tx);
        Picker::uniform_list(delegate, window, cx)
    });
    let project = workspace.project().clone();
    cx.spawn_in(window, async move |workspace, cx| {
        let encoding = rx.await.unwrap();

        let (worktree, rel_path) = project
            .update(cx, |project, cx| {
                project.find_or_create_worktree(path, false, cx)
            })?
            .await?;

        let project_path = (worktree.update(cx, |worktree, _| worktree.id())?, rel_path).into();

        let buffer = project
            .update(cx, |project, cx| {
                project.buffer_store().update(cx, |buffer_store, cx| {
                    buffer_store.open_buffer(
                        project_path,
                        &EncodingOptions {
                            expected: encoding,
                            auto_detect: true,
                        },
                        cx,
                    )
                })
            })?
            .await?;
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.open_project_item::<Editor>(
                workspace.active_pane().clone(),
                buffer,
                true,
                true,
                window,
                cx,
            )
        })?;

        Ok(())
    })
}

pub fn reopen_with_encoding(
    buffer: Entity<Buffer>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let encoding = buffer.read(cx).encoding();
    let (tx, rx) = oneshot::channel();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = EncodingSelectorDelegate::new(Some(encoding), tx);
        Picker::uniform_list(delegate, window, cx)
    });
    cx.spawn(async move |_, cx| {
        let encoding = rx.await.unwrap();

        let (task, prev) = buffer.update(cx, |buffer, cx| {
            let prev = buffer.encoding();
            buffer.set_encoding(encoding, cx);
            (buffer.reload(cx), prev)
        })?;

        if task.await.is_err() {
            buffer.update(cx, |buffer, cx| {
                buffer.set_encoding(prev, cx);
            })?;
        }

        anyhow::Ok(())
    })
    .detach();
}

pub fn save_with_encoding(
    buffer: Entity<Buffer>,
    workspace: &mut Workspace,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let encoding = buffer.read(cx).encoding();
    let (tx, rx) = oneshot::channel();
    workspace.toggle_modal(window, cx, |window, cx| {
        let delegate = EncodingSelectorDelegate::new(Some(encoding), tx);
        Picker::uniform_list(delegate, window, cx)
    });
    cx.spawn(async move |workspace, cx| {
        let encoding = rx.await.unwrap();
        workspace
            .update(cx, |workspace, cx| {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_encoding(encoding, cx);
                });
                workspace
                    .project()
                    .update(cx, |project, cx| project.save_buffer(buffer, cx))
            })
            .ok();
    })
    .detach();
}

pub enum SaveOrReopen {
    Save,
    Reopen,
}

pub struct EncodingSaveOrReopenDelegate {
    current_selection: usize,
    actions: Vec<SaveOrReopen>,
    workspace: WeakEntity<Workspace>,
    buffer: Entity<Buffer>,
}

impl EncodingSaveOrReopenDelegate {
    pub fn new(buffer: Entity<Buffer>, workspace: WeakEntity<Workspace>) -> Self {
        Self {
            current_selection: 0,
            actions: vec![SaveOrReopen::Save, SaveOrReopen::Reopen],
            workspace,
            buffer,
        }
    }
}

impl PickerDelegate for EncodingSaveOrReopenDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.actions.len()
    }

    fn selected_index(&self) -> usize {
        self.current_selection
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.current_selection = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut ui::App) -> std::sync::Arc<str> {
        "Select an action...".into()
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        return Task::ready(());
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.dismissed(window, cx);
        cx.defer_in(window, |this, window, cx| {
            let this = &this.delegate;
            this.workspace
                .update(cx, |workspace, cx| {
                    match this.actions[this.current_selection] {
                        SaveOrReopen::Reopen => {
                            reopen_with_encoding(this.buffer.clone(), workspace, window, cx);
                        }
                        SaveOrReopen::Save => {
                            save_with_encoding(this.buffer.clone(), workspace, window, cx);
                        }
                    }
                })
                .ok();
        })
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent)
    }

    fn render_match(
        &self,
        ix: usize,
        _: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            ListItem::new(ix)
                .child(match self.actions[ix] {
                    SaveOrReopen::Save => Label::new("Save with encoding"),
                    SaveOrReopen::Reopen => Label::new("Reopen with encoding"),
                })
                .spacing(ui::ListItemSpacing::Sparse),
        )
    }
}

pub struct EncodingSelectorDelegate {
    current_selection: usize,
    encodings: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    tx: Option<oneshot::Sender<Encoding>>,
}

impl EncodingSelectorDelegate {
    pub fn new(
        encoding: Option<Encoding>,
        tx: oneshot::Sender<Encoding>,
    ) -> EncodingSelectorDelegate {
        let encodings = vec![
            StringMatchCandidate::new(0, "UTF-8"),
            StringMatchCandidate::new(1, "UTF-16 LE"),
            StringMatchCandidate::new(2, "UTF-16 BE"),
            StringMatchCandidate::new(3, "Windows-1252"),
            StringMatchCandidate::new(4, "Windows-1251"),
            StringMatchCandidate::new(5, "Windows-1250"),
            StringMatchCandidate::new(6, "ISO 8859-2"),
            StringMatchCandidate::new(7, "ISO 8859-3"),
            StringMatchCandidate::new(8, "ISO 8859-4"),
            StringMatchCandidate::new(9, "ISO 8859-5"),
            StringMatchCandidate::new(10, "ISO 8859-6"),
            StringMatchCandidate::new(11, "ISO 8859-7"),
            StringMatchCandidate::new(12, "ISO 8859-8"),
            StringMatchCandidate::new(13, "ISO 8859-13"),
            StringMatchCandidate::new(14, "ISO 8859-15"),
            StringMatchCandidate::new(15, "KOI8-R"),
            StringMatchCandidate::new(16, "KOI8-U"),
            StringMatchCandidate::new(17, "MacRoman"),
            StringMatchCandidate::new(18, "Mac Cyrillic"),
            StringMatchCandidate::new(19, "Windows-874"),
            StringMatchCandidate::new(20, "Windows-1253"),
            StringMatchCandidate::new(21, "Windows-1254"),
            StringMatchCandidate::new(22, "Windows-1255"),
            StringMatchCandidate::new(23, "Windows-1256"),
            StringMatchCandidate::new(24, "Windows-1257"),
            StringMatchCandidate::new(25, "Windows-1258"),
            StringMatchCandidate::new(26, "Windows-949"),
            StringMatchCandidate::new(27, "EUC-JP"),
            StringMatchCandidate::new(28, "ISO 2022-JP"),
            StringMatchCandidate::new(29, "GBK"),
            StringMatchCandidate::new(30, "GB18030"),
            StringMatchCandidate::new(31, "Big5"),
        ];
        let current_selection = if let Some(encoding) = encoding {
            encodings
                .iter()
                .position(|e| encoding.name() == e.string)
                .unwrap_or_default()
        } else {
            0
        };

        EncodingSelectorDelegate {
            current_selection,
            encodings,
            matches: Vec::new(),
            tx: Some(tx),
        }
    }
}

impl PickerDelegate for EncodingSelectorDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.current_selection
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.current_selection = ix;
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut ui::App) -> std::sync::Arc<str> {
        "Select an encoding...".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let executor = cx.background_executor().clone();
        let encodings = self.encodings.clone();

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<StringMatch>;

            if query.is_empty() {
                matches = encodings
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| StringMatch {
                        candidate_id: index,
                        score: 0.0,
                        positions: Vec::new(),
                        string: value.string,
                    })
                    .collect();
            } else {
                matches = fuzzy::match_strings(
                    &encodings,
                    &query,
                    true,
                    false,
                    30,
                    &AtomicBool::new(false),
                    executor,
                )
                .await
            }
            picker
                .update(cx, |picker, cx| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    delegate.current_selection = delegate
                        .current_selection
                        .min(delegate.matches.len().saturating_sub(1));
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let current_selection = self.matches[self.current_selection].string.clone();
        let encoding = Encoding::from_name(&current_selection);
        if let Some(tx) = self.tx.take() {
            tx.send(encoding).log_err();
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        _: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            ListItem::new(ix)
                .child(HighlightedLabel::new(
                    &self.matches[ix].string,
                    self.matches[ix].positions.clone(),
                ))
                .spacing(ListItemSpacing::Sparse),
        )
    }
}
