use crate::{Anchor, Autoscroll, Editor, Event, ExcerptId, NavigationData, ToOffset, ToPoint as _};
use anyhow::{anyhow, Result};
use gpui::{
    elements::*, geometry::vector::vec2f, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, Subscription, Task, View, ViewContext, ViewHandle,
};
use language::{Bias, Buffer, Diagnostic, File as _, SelectionGoal};
use project::{File, Project, ProjectEntryId, ProjectPath};
use rpc::proto::{self, update_view};
use std::{fmt::Write, path::PathBuf};
use text::{Point, Selection};
use util::ResultExt;
use workspace::{
    FollowableItem, Item, ItemHandle, ItemNavHistory, ProjectItem, Settings, StatusItemView,
};

impl FollowableItem for Editor {
    fn from_state_proto(
        pane: ViewHandle<workspace::Pane>,
        project: ModelHandle<Project>,
        state: &mut Option<proto::view::Variant>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>> {
        let state = if matches!(state, Some(proto::view::Variant::Editor(_))) {
            if let Some(proto::view::Variant::Editor(state)) = state.take() {
                state
            } else {
                unreachable!()
            }
        } else {
            return None;
        };

        let buffer = project.update(cx, |project, cx| {
            project.open_buffer_by_id(state.buffer_id, cx)
        });
        Some(cx.spawn(|mut cx| async move {
            let buffer = buffer.await?;
            let editor = pane
                .read_with(&cx, |pane, cx| {
                    pane.items_of_type::<Self>().find(|editor| {
                        editor.read(cx).buffer.read(cx).as_singleton().as_ref() == Some(&buffer)
                    })
                })
                .unwrap_or_else(|| {
                    cx.add_view(pane.window_id(), |cx| {
                        Editor::for_buffer(buffer, Some(project), cx)
                    })
                });
            editor.update(&mut cx, |editor, cx| {
                let excerpt_id;
                let buffer_id;
                {
                    let buffer = editor.buffer.read(cx).read(cx);
                    let singleton = buffer.as_singleton().unwrap();
                    excerpt_id = singleton.0.clone();
                    buffer_id = singleton.1;
                }
                let selections = state
                    .selections
                    .into_iter()
                    .map(|selection| {
                        deserialize_selection(&excerpt_id, buffer_id, selection)
                            .ok_or_else(|| anyhow!("invalid selection"))
                    })
                    .collect::<Result<Vec<_>>>()?;
                if !selections.is_empty() {
                    editor.set_selections_from_remote(selections.into(), cx);
                }

                if let Some(anchor) = state.scroll_top_anchor {
                    editor.set_scroll_top_anchor(
                        Anchor {
                            buffer_id: Some(state.buffer_id as usize),
                            excerpt_id: excerpt_id.clone(),
                            text_anchor: language::proto::deserialize_anchor(anchor)
                                .ok_or_else(|| anyhow!("invalid scroll top"))?,
                        },
                        vec2f(state.scroll_x, state.scroll_y),
                        cx,
                    );
                }

                Ok::<_, anyhow::Error>(())
            })?;
            Ok(editor)
        }))
    }

    fn set_leader_replica_id(
        &mut self,
        leader_replica_id: Option<u16>,
        cx: &mut ViewContext<Self>,
    ) {
        self.leader_replica_id = leader_replica_id;
        if self.leader_replica_id.is_some() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.remove_active_selections(cx);
            });
        } else {
            self.buffer.update(cx, |buffer, cx| {
                if self.focused {
                    buffer.set_active_selections(&self.selections, cx);
                }
            });
        }
        cx.notify();
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        let buffer_id = self.buffer.read(cx).as_singleton()?.read(cx).remote_id();
        Some(proto::view::Variant::Editor(proto::view::Editor {
            buffer_id,
            scroll_top_anchor: Some(language::proto::serialize_anchor(
                &self.scroll_top_anchor.text_anchor,
            )),
            scroll_x: self.scroll_position.x(),
            scroll_y: self.scroll_position.y(),
            selections: self.selections.iter().map(serialize_selection).collect(),
        }))
    }

    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        _: &AppContext,
    ) -> bool {
        let update =
            update.get_or_insert_with(|| proto::update_view::Variant::Editor(Default::default()));

        match update {
            proto::update_view::Variant::Editor(update) => match event {
                Event::ScrollPositionChanged { .. } => {
                    update.scroll_top_anchor = Some(language::proto::serialize_anchor(
                        &self.scroll_top_anchor.text_anchor,
                    ));
                    update.scroll_x = self.scroll_position.x();
                    update.scroll_y = self.scroll_position.y();
                    true
                }
                Event::SelectionsChanged { .. } => {
                    update.selections = self
                        .selections
                        .iter()
                        .chain(self.pending_selection.as_ref().map(|p| &p.selection))
                        .map(serialize_selection)
                        .collect();
                    true
                }
                _ => false,
            },
        }
    }

    fn apply_update_proto(
        &mut self,
        message: update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Result<()> {
        match message {
            update_view::Variant::Editor(message) => {
                let buffer = self.buffer.read(cx);
                let buffer = buffer.read(cx);
                let (excerpt_id, buffer_id, _) = buffer.as_singleton().unwrap();
                let excerpt_id = excerpt_id.clone();
                drop(buffer);

                let selections = message
                    .selections
                    .into_iter()
                    .filter_map(|selection| {
                        deserialize_selection(&excerpt_id, buffer_id, selection)
                    })
                    .collect::<Vec<_>>();

                if !selections.is_empty() {
                    self.set_selections_from_remote(selections, cx);
                    self.request_autoscroll_remotely(Autoscroll::Newest, cx);
                } else {
                    if let Some(anchor) = message.scroll_top_anchor {
                        self.set_scroll_top_anchor(
                            Anchor {
                                buffer_id: Some(buffer_id),
                                excerpt_id: excerpt_id.clone(),
                                text_anchor: language::proto::deserialize_anchor(anchor)
                                    .ok_or_else(|| anyhow!("invalid scroll top"))?,
                            },
                            vec2f(message.scroll_x, message.scroll_y),
                            cx,
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn should_unfollow_on_event(event: &Self::Event, _: &AppContext) -> bool {
        match event {
            // Event::BufferEdited { local } => *local,
            Event::SelectionsChanged { local } => *local,
            Event::ScrollPositionChanged { local } => *local,
            _ => false,
        }
    }
}

fn serialize_selection(selection: &Selection<Anchor>) -> proto::Selection {
    proto::Selection {
        id: selection.id as u64,
        start: Some(language::proto::serialize_anchor(
            &selection.start.text_anchor,
        )),
        end: Some(language::proto::serialize_anchor(
            &selection.end.text_anchor,
        )),
        reversed: selection.reversed,
    }
}

fn deserialize_selection(
    excerpt_id: &ExcerptId,
    buffer_id: usize,
    selection: proto::Selection,
) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: Anchor {
            buffer_id: Some(buffer_id),
            excerpt_id: excerpt_id.clone(),
            text_anchor: language::proto::deserialize_anchor(selection.start?)?,
        },
        end: Anchor {
            buffer_id: Some(buffer_id),
            excerpt_id: excerpt_id.clone(),
            text_anchor: language::proto::deserialize_anchor(selection.end?)?,
        },
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

impl Item for Editor {
    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        if let Some(data) = data.downcast_ref::<NavigationData>() {
            let buffer = self.buffer.read(cx).read(cx);
            let offset = if buffer.can_resolve(&data.anchor) {
                data.anchor.to_offset(&buffer)
            } else {
                buffer.clip_offset(data.offset, Bias::Left)
            };
            let newest_selection = self.newest_selection_with_snapshot::<usize>(&buffer);
            drop(buffer);

            if newest_selection.head() == offset {
                false
            } else {
                let nav_history = self.nav_history.take();
                self.select_ranges([offset..offset], Some(Autoscroll::Fit), cx);
                self.nav_history = nav_history;
                true
            }
        } else {
            false
        }
    }

    fn tab_content(&self, style: &theme::Tab, cx: &AppContext) -> ElementBox {
        let title = self.title(cx);
        Label::new(title, style.label.clone()).boxed()
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        File::from_dyn(self.buffer().read(cx).file(cx)).map(|file| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }

    fn project_entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId> {
        File::from_dyn(self.buffer().read(cx).file(cx)).and_then(|file| file.project_entry_id(cx))
    }

    fn clone_on_split(&self, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone(cx))
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        let selection = self.newest_anchor_selection();
        self.push_to_nav_history(selection.head(), None, cx);
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).has_conflict()
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        !self.buffer().read(cx).is_singleton() || self.project_path(cx).is_some()
    }

    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = buffer.read(cx).all_buffers();
        let transaction = project.update(cx, |project, cx| project.format(buffers, true, cx));
        cx.spawn(|this, mut cx| async move {
            let transaction = transaction.await.log_err();
            this.update(&mut cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::Fit, cx)
            });
            buffer
                .update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !buffer.is_singleton() {
                            buffer.push_transaction(&transaction.0);
                        }
                    }

                    buffer.save(cx)
                })
                .await?;
            Ok(())
        })
    }

    fn can_save_as(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).is_singleton()
    }

    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("cannot call save_as on an excerpt list")
            .clone();

        project.update(cx, |project, cx| {
            project.save_buffer_as(buffer, abs_path, cx)
        })
    }

    fn should_activate_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Activate)
    }

    fn should_close_item_on_event(event: &Event) -> bool {
        matches!(event, Event::Closed)
    }

    fn should_update_tab_on_event(event: &Event) -> bool {
        matches!(event, Event::Saved | Event::Dirtied | Event::TitleChanged)
    }
}

impl ProjectItem for Editor {
    type Item = Buffer;

    fn for_project_item(
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::for_buffer(buffer, Some(project), cx)
    }
}

pub struct CursorPosition {
    position: Option<Point>,
    selected_count: usize,
    _observe_active_editor: Option<Subscription>,
}

impl CursorPosition {
    pub fn new() -> Self {
        Self {
            position: None,
            selected_count: 0,
            _observe_active_editor: None,
        }
    }

    fn update_position(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx).snapshot(cx);

        self.selected_count = 0;
        let mut last_selection: Option<Selection<usize>> = None;
        for selection in editor.local_selections::<usize>(cx) {
            self.selected_count += selection.end - selection.start;
            if last_selection
                .as_ref()
                .map_or(true, |last_selection| selection.id > last_selection.id)
            {
                last_selection = Some(selection);
            }
        }
        self.position = last_selection.map(|s| s.head().to_point(&buffer));

        cx.notify();
    }
}

impl Entity for CursorPosition {
    type Event = ();
}

impl View for CursorPosition {
    fn ui_name() -> &'static str {
        "CursorPosition"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if let Some(position) = self.position {
            let theme = &cx.global::<Settings>().theme.workspace.status_bar;
            let mut text = format!("{},{}", position.row + 1, position.column + 1);
            if self.selected_count > 0 {
                write!(text, " ({} selected)", self.selected_count).unwrap();
            }
            Label::new(text, theme.cursor_position.clone()).boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for CursorPosition {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_position));
            self.update_position(editor, cx);
        } else {
            self.position = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

pub struct DiagnosticMessage {
    diagnostic: Option<Diagnostic>,
    _observe_active_editor: Option<Subscription>,
}

impl DiagnosticMessage {
    pub fn new() -> Self {
        Self {
            diagnostic: None,
            _observe_active_editor: None,
        }
    }

    fn update(&mut self, editor: ViewHandle<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        let buffer = editor.buffer().read(cx);
        let cursor_position = editor
            .newest_selection_with_snapshot::<usize>(&buffer.read(cx))
            .head();
        let new_diagnostic = buffer
            .read(cx)
            .diagnostics_in_range::<_, usize>(cursor_position..cursor_position, false)
            .filter(|entry| !entry.range.is_empty())
            .min_by_key(|entry| (entry.diagnostic.severity, entry.range.len()))
            .map(|entry| entry.diagnostic);
        if new_diagnostic != self.diagnostic {
            self.diagnostic = new_diagnostic;
            cx.notify();
        }
    }
}

impl Entity for DiagnosticMessage {
    type Event = ();
}

impl View for DiagnosticMessage {
    fn ui_name() -> &'static str {
        "DiagnosticMessage"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        if let Some(diagnostic) = &self.diagnostic {
            let theme = &cx.global::<Settings>().theme.workspace.status_bar;
            Label::new(
                diagnostic.message.split('\n').next().unwrap().to_string(),
                theme.diagnostic_message.clone(),
            )
            .boxed()
        } else {
            Empty::new().boxed()
        }
    }
}

impl StatusItemView for DiagnosticMessage {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update));
            self.update(editor, cx);
        } else {
            self.diagnostic = Default::default();
            self._observe_active_editor = None;
        }
        cx.notify();
    }
}
