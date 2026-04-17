use crate::{
    ActiveDebugLine, Anchor, Autoscroll, BufferSerialization, Capability, Editor, EditorEvent,
    EditorSettings, ExcerptRange, FormatTarget, MultiBuffer, MultiBufferSnapshot, NavigationData,
    ReportEditorEvent, SelectionEffects, ToPoint as _,
    display_map::HighlightKey,
    editor_settings::SeedQuerySetting,
    persistence::{EditorDb, SerializedEditor},
    scroll::{ScrollAnchor, ScrollOffset},
};
use anyhow::{Context as _, Result, anyhow};
use collections::{HashMap, HashSet};
use file_icons::FileIcons;
use fs::MTime;
use futures::future::try_join_all;
use git::status::GitSummary;
use gpui::{
    AnyElement, App, AsyncWindowContext, Context, Entity, EntityId, EventEmitter, Font,
    IntoElement, ParentElement, Pixels, SharedString, Styled, Task, WeakEntity, Window, point,
};
use language::{
    Bias, Buffer, BufferRow, CharKind, CharScopeContext, HighlightedText, LocalFile, Point,
    SelectionGoal, proto::serialize_anchor as serialize_text_anchor,
};
use lsp::DiagnosticSeverity;
use multi_buffer::{MultiBufferOffset, PathKey};
use project::{
    File, Project, ProjectItem as _, ProjectPath, lsp_store::FormatTrigger,
    project_settings::ProjectSettings, search::SearchQuery,
};
use rpc::proto::{self, update_view};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cmp::{self, Ordering},
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use text::{BufferId, BufferSnapshot, Selection};
use ui::{IconDecorationKind, prelude::*};
use util::{ResultExt, TryFutureExt, paths::PathExt, rel_path::RelPath};
use workspace::item::{Dedup, ItemSettings, SerializableItem, TabContentParams};
use workspace::{
    CollaboratorId, ItemId, ItemNavHistory, ToolbarItemLocation, ViewId, Workspace, WorkspaceId,
    invalid_item_view::InvalidItemView,
    item::{FollowableItem, Item, ItemBufferKind, ItemEvent, ProjectItem, SaveOptions},
    searchable::{
        Direction, FilteredSearchRange, SearchEvent, SearchToken, SearchableItem,
        SearchableItemHandle,
    },
};
use workspace::{
    Pane, WorkspaceSettings,
    item::{FollowEvent, ProjectItemKind},
    searchable::SearchOptions,
};
use zed_actions::preview::{
    markdown::OpenPreview as OpenMarkdownPreview, svg::OpenPreview as OpenSvgPreview,
};

pub const MAX_TAB_TITLE_LEN: usize = 24;

impl FollowableItem for Editor {
    fn remote_id(&self) -> Option<ViewId> {
        self.remote_id
    }

    fn from_state_proto(
        workspace: Entity<Workspace>,
        remote_id: ViewId,
        state: &mut Option<proto::view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Task<Result<Entity<Self>>>> {
        let project = workspace.read(cx).project().to_owned();
        let Some(proto::view::Variant::Editor(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::Editor(state)) = state.take() else {
            unreachable!()
        };

        let buffer_ids = state
            .path_excerpts
            .iter()
            .map(|excerpt| excerpt.buffer_id)
            .collect::<HashSet<_>>();

        let buffers = project.update(cx, |project, cx| {
            buffer_ids
                .iter()
                .map(|id| BufferId::new(*id).map(|id| project.open_buffer_by_id(id, cx)))
                .collect::<Result<Vec<_>>>()
        });

        Some(window.spawn(cx, async move |cx| {
            let mut buffers = futures::future::try_join_all(buffers?)
                .await
                .debug_assert_ok("leaders don't share views for unshared buffers")?;

            let editor = cx.update(|window, cx| {
                let multibuffer = cx.new(|cx| {
                    let mut multibuffer;
                    if state.singleton && buffers.len() == 1 {
                        multibuffer = MultiBuffer::singleton(buffers.pop().unwrap(), cx)
                    } else {
                        multibuffer = MultiBuffer::new(project.read(cx).capability());
                        for path_with_ranges in state.path_excerpts {
                            let Some(path_key) =
                                path_with_ranges.path_key.and_then(deserialize_path_key)
                            else {
                                continue;
                            };
                            let Some(buffer_id) = BufferId::new(path_with_ranges.buffer_id).ok()
                            else {
                                continue;
                            };
                            let Some(buffer) =
                                buffers.iter().find(|b| b.read(cx).remote_id() == buffer_id)
                            else {
                                continue;
                            };
                            let buffer_snapshot = buffer.read(cx).snapshot();
                            let ranges = path_with_ranges
                                .ranges
                                .into_iter()
                                .filter_map(deserialize_excerpt_range)
                                .collect::<Vec<_>>();
                            multibuffer.update_path_excerpts(
                                path_key,
                                buffer.clone(),
                                &buffer_snapshot,
                                &ranges,
                                cx,
                            );
                        }
                    };

                    if let Some(title) = &state.title {
                        multibuffer = multibuffer.with_title(title.clone())
                    }

                    multibuffer
                });

                cx.new(|cx| {
                    let mut editor =
                        Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx);
                    editor.remote_id = Some(remote_id);
                    editor
                })
            })?;

            editor.update(cx, |editor, cx| editor.text(cx));
            update_editor_from_message(
                editor.downgrade(),
                project,
                proto::update_view::Editor {
                    selections: state.selections,
                    pending_selection: state.pending_selection,
                    scroll_top_anchor: state.scroll_top_anchor,
                    scroll_x: state.scroll_x,
                    scroll_y: state.scroll_y,
                    ..Default::default()
                },
                cx,
            )
            .await?;

            Ok(editor)
        }))
    }

    fn set_leader_id(
        &mut self,
        leader_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.leader_id = leader_id;
        if self.leader_id.is_some() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.remove_active_selections(cx);
            });
        } else if self.focus_handle.is_focused(window) {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(
                    &self.selections.disjoint_anchors_arc(),
                    self.selections.line_mode(),
                    self.cursor_shape,
                    cx,
                );
            });
        }
        cx.notify();
    }

    fn to_state_proto(&self, _: &mut Window, cx: &mut App) -> Option<proto::view::Variant> {
        let is_private = self
            .buffer
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| file.is_private());
        if is_private {
            return None;
        }

        let display_snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let scroll_anchor = self.scroll_manager.native_anchor(&display_snapshot, cx);
        let buffer = self.buffer.read(cx);
        let snapshot = buffer.snapshot(cx);
        let mut path_excerpts: Vec<proto::PathExcerpts> = Vec::new();
        for excerpt in snapshot.excerpts() {
            if let Some(prev_entry) = path_excerpts.last_mut()
                && prev_entry.buffer_id == excerpt.context.start.buffer_id.to_proto()
            {
                prev_entry.ranges.push(serialize_excerpt_range(excerpt));
            } else if let Some(path_key) = snapshot.path_for_buffer(excerpt.context.start.buffer_id)
            {
                path_excerpts.push(proto::PathExcerpts {
                    path_key: Some(serialize_path_key(path_key)),
                    buffer_id: excerpt.context.start.buffer_id.to_proto(),
                    ranges: vec![serialize_excerpt_range(excerpt)],
                });
            }
        }

        Some(proto::view::Variant::Editor(proto::view::Editor {
            singleton: buffer.is_singleton(),
            title: buffer.explicit_title().map(ToOwned::to_owned),
            excerpts: Vec::new(),
            scroll_top_anchor: Some(serialize_anchor(&scroll_anchor.anchor)),
            scroll_x: scroll_anchor.offset.x,
            scroll_y: scroll_anchor.offset.y,
            selections: self
                .selections
                .disjoint_anchors_arc()
                .iter()
                .map(serialize_selection)
                .collect(),
            pending_selection: self
                .selections
                .pending_anchor()
                .as_ref()
                .copied()
                .map(serialize_selection),
            path_excerpts,
        }))
    }

    fn to_follow_event(event: &EditorEvent) -> Option<workspace::item::FollowEvent> {
        match event {
            EditorEvent::Edited { .. } => Some(FollowEvent::Unfollow),
            EditorEvent::SelectionsChanged { local }
            | EditorEvent::ScrollPositionChanged { local, .. } => {
                if *local {
                    Some(FollowEvent::Unfollow)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn add_event_to_update_proto(
        &self,
        event: &EditorEvent,
        update: &mut Option<proto::update_view::Variant>,
        _: &mut Window,
        cx: &mut App,
    ) -> bool {
        let update =
            update.get_or_insert_with(|| proto::update_view::Variant::Editor(Default::default()));

        match update {
            proto::update_view::Variant::Editor(update) => match event {
                EditorEvent::BufferRangesUpdated {
                    buffer,
                    path_key,
                    ranges,
                } => {
                    let buffer_id = buffer.read(cx).remote_id().to_proto();
                    let path_key = serialize_path_key(path_key);
                    let ranges = ranges
                        .iter()
                        .cloned()
                        .map(serialize_excerpt_range)
                        .collect::<Vec<_>>();
                    update.updated_paths.push(proto::PathExcerpts {
                        path_key: Some(path_key),
                        buffer_id,
                        ranges,
                    });
                    true
                }
                EditorEvent::BuffersRemoved { removed_buffer_ids } => {
                    update
                        .deleted_buffers
                        .extend(removed_buffer_ids.iter().copied().map(BufferId::to_proto));
                    true
                }
                EditorEvent::ScrollPositionChanged { autoscroll, .. } if !autoscroll => {
                    let display_snapshot = self.display_map.update(cx, |map, cx| map.snapshot(cx));
                    let scroll_anchor = self.scroll_manager.native_anchor(&display_snapshot, cx);
                    update.scroll_top_anchor = Some(serialize_anchor(&scroll_anchor.anchor));
                    update.scroll_x = scroll_anchor.offset.x;
                    update.scroll_y = scroll_anchor.offset.y;
                    true
                }
                EditorEvent::SelectionsChanged { .. } => {
                    update.selections = self
                        .selections
                        .disjoint_anchors_arc()
                        .iter()
                        .map(serialize_selection)
                        .collect();
                    update.pending_selection = self
                        .selections
                        .pending_anchor()
                        .as_ref()
                        .copied()
                        .map(serialize_selection);
                    true
                }
                _ => false,
            },
        }
    }

    fn apply_update_proto(
        &mut self,
        project: &Entity<Project>,
        message: update_view::Variant,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let update_view::Variant::Editor(message) = message;
        let project = project.clone();
        cx.spawn_in(window, async move |this, cx| {
            update_editor_from_message(this, project, message, cx).await
        })
    }

    fn is_project_item(&self, _window: &Window, _cx: &App) -> bool {
        true
    }

    fn dedup(&self, existing: &Self, _: &Window, cx: &App) -> Option<Dedup> {
        let self_singleton = self.buffer.read(cx).as_singleton()?;
        let other_singleton = existing.buffer.read(cx).as_singleton()?;
        if self_singleton == other_singleton {
            Some(Dedup::KeepExisting)
        } else {
            None
        }
    }

    fn update_agent_location(
        &mut self,
        location: language::Anchor,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = self.buffer.read(cx);
        let buffer = buffer.read(cx);
        let Some(position) = buffer.anchor_in_excerpt(location) else {
            return;
        };
        let selection = Selection {
            id: 0,
            reversed: false,
            start: position,
            end: position,
            goal: SelectionGoal::None,
        };
        drop(buffer);
        self.set_selections_from_remote(vec![selection], None, window, cx);
        self.request_autoscroll_remotely(Autoscroll::fit(), cx);
    }
}

async fn update_editor_from_message(
    this: WeakEntity<Editor>,
    project: Entity<Project>,
    message: proto::update_view::Editor,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    // Open all of the buffers of which excerpts were added to the editor.
    let inserted_excerpt_buffer_ids = message
        .updated_paths
        .iter()
        .map(|insertion| insertion.buffer_id)
        .collect::<HashSet<_>>();
    let inserted_excerpt_buffers = project.update(cx, |project, cx| {
        inserted_excerpt_buffer_ids
            .into_iter()
            .map(|id| BufferId::new(id).map(|id| project.open_buffer_by_id(id, cx)))
            .collect::<Result<Vec<_>>>()
    })?;
    let _inserted_excerpt_buffers = try_join_all(inserted_excerpt_buffers).await?;

    // Update the editor's excerpts.
    let buffer_snapshot = this.update(cx, |editor, cx| {
        editor.buffer.update(cx, |multibuffer, cx| {
            for path_with_excerpts in message.updated_paths {
                let Some(path_key) = path_with_excerpts.path_key.and_then(deserialize_path_key)
                else {
                    continue;
                };
                let ranges = path_with_excerpts
                    .ranges
                    .into_iter()
                    .filter_map(deserialize_excerpt_range)
                    .collect::<Vec<_>>();
                let Some(buffer) = BufferId::new(path_with_excerpts.buffer_id)
                    .ok()
                    .and_then(|buffer_id| project.read(cx).buffer_for_id(buffer_id, cx))
                else {
                    continue;
                };

                let buffer_snapshot = buffer.read(cx).snapshot();
                multibuffer.update_path_excerpts(path_key, buffer, &buffer_snapshot, &ranges, cx);
            }

            for buffer_id in message
                .deleted_buffers
                .into_iter()
                .filter_map(|buffer_id| BufferId::new(buffer_id).ok())
            {
                multibuffer.remove_excerpts_for_buffer(buffer_id, cx);
            }

            multibuffer.snapshot(cx)
        })
    })?;

    // Deserialize the editor state.
    let selections = message
        .selections
        .into_iter()
        .filter_map(|selection| deserialize_selection(selection, &buffer_snapshot))
        .collect::<Vec<_>>();
    let pending_selection = message
        .pending_selection
        .and_then(|selection| deserialize_selection(selection, &buffer_snapshot));
    let scroll_top_anchor = message
        .scroll_top_anchor
        .and_then(|selection| deserialize_anchor(selection, &buffer_snapshot));

    // Wait until the buffer has received all of the operations referenced by
    // the editor's new state.
    this.update(cx, |editor, cx| {
        editor.buffer.update(cx, |buffer, cx| {
            buffer.wait_for_anchors(
                selections
                    .iter()
                    .chain(pending_selection.as_ref())
                    .flat_map(|selection| [selection.start, selection.end])
                    .chain(scroll_top_anchor),
                cx,
            )
        })
    })?
    .await?;

    // Update the editor's state.
    this.update_in(cx, |editor, window, cx| {
        if !selections.is_empty() || pending_selection.is_some() {
            editor.set_selections_from_remote(selections, pending_selection, window, cx);
            editor.request_autoscroll_remotely(Autoscroll::newest(), cx);
        } else if let Some(scroll_top_anchor) = scroll_top_anchor {
            editor.set_scroll_anchor_remote(
                ScrollAnchor {
                    anchor: scroll_top_anchor,
                    offset: point(message.scroll_x, message.scroll_y),
                },
                window,
                cx,
            );
        }
    })?;
    Ok(())
}

fn serialize_selection(selection: &Selection<Anchor>) -> proto::Selection {
    proto::Selection {
        id: selection.id as u64,
        start: Some(serialize_anchor(&selection.start)),
        end: Some(serialize_anchor(&selection.end)),
        reversed: selection.reversed,
    }
}

fn serialize_anchor(anchor: &Anchor) -> proto::EditorAnchor {
    match anchor {
        Anchor::Min => proto::EditorAnchor {
            excerpt_id: None,
            anchor: Some(proto::Anchor {
                replica_id: 0,
                timestamp: 0,
                offset: 0,
                bias: proto::Bias::Left as i32,
                buffer_id: None,
            }),
        },
        Anchor::Excerpt(_) => proto::EditorAnchor {
            excerpt_id: None,
            anchor: anchor.raw_text_anchor().map(|a| serialize_text_anchor(&a)),
        },
        Anchor::Max => proto::EditorAnchor {
            excerpt_id: None,
            anchor: Some(proto::Anchor {
                replica_id: u32::MAX,
                timestamp: u32::MAX,
                offset: u64::MAX,
                bias: proto::Bias::Right as i32,
                buffer_id: None,
            }),
        },
    }
}

fn serialize_excerpt_range(range: ExcerptRange<language::Anchor>) -> proto::ExcerptRange {
    let context_start = language::proto::serialize_anchor(&range.context.start);
    let context_end = language::proto::serialize_anchor(&range.context.end);
    let primary_start = language::proto::serialize_anchor(&range.primary.start);
    let primary_end = language::proto::serialize_anchor(&range.primary.end);
    proto::ExcerptRange {
        context_start: Some(context_start),
        context_end: Some(context_end),
        primary_start: Some(primary_start),
        primary_end: Some(primary_end),
    }
}

fn deserialize_excerpt_range(
    excerpt_range: proto::ExcerptRange,
) -> Option<ExcerptRange<language::Anchor>> {
    let context = {
        let start = language::proto::deserialize_anchor(excerpt_range.context_start?)?;
        let end = language::proto::deserialize_anchor(excerpt_range.context_end?)?;
        start..end
    };
    let primary = excerpt_range
        .primary_start
        .zip(excerpt_range.primary_end)
        .and_then(|(start, end)| {
            let start = language::proto::deserialize_anchor(start)?;
            let end = language::proto::deserialize_anchor(end)?;
            Some(start..end)
        })
        .unwrap_or_else(|| context.clone());
    Some(ExcerptRange { context, primary })
}

fn deserialize_selection(
    selection: proto::Selection,
    buffer: &MultiBufferSnapshot,
) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: deserialize_anchor(selection.start?, buffer)?,
        end: deserialize_anchor(selection.end?, buffer)?,
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

fn deserialize_anchor(anchor: proto::EditorAnchor, buffer: &MultiBufferSnapshot) -> Option<Anchor> {
    let anchor = anchor.anchor?;
    if let Some(buffer_id) = anchor.buffer_id
        && BufferId::new(buffer_id).is_ok()
    {
        let text_anchor = language::proto::deserialize_anchor(anchor)?;
        buffer.anchor_in_buffer(text_anchor)
    } else {
        match proto::Bias::from_i32(anchor.bias)? {
            proto::Bias::Left => Some(Anchor::Min),
            proto::Bias::Right => Some(Anchor::Max),
        }
    }
}

impl Item for Editor {
    type Event = EditorEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle.clone().into())
        } else if TypeId::of::<MultiBuffer>() == type_id {
            Some(self_handle.read(cx).buffer.clone().into())
        } else {
            None
        }
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if let Some(data) = data.downcast_ref::<NavigationData>() {
            let newest_selection = self.selections.newest::<Point>(&self.display_snapshot(cx));
            let buffer = self.buffer.read(cx).read(cx);
            let offset = if buffer.can_resolve(&data.cursor_anchor) {
                data.cursor_anchor.to_point(&buffer)
            } else {
                buffer.clip_point(data.cursor_position, Bias::Left)
            };

            let mut scroll_anchor = data.scroll_anchor;
            if !buffer.can_resolve(&scroll_anchor.anchor) {
                scroll_anchor.anchor = buffer.anchor_before(
                    buffer.clip_point(Point::new(data.scroll_top_row, 0), Bias::Left),
                );
            }

            drop(buffer);

            if newest_selection.head() == offset {
                false
            } else {
                self.set_scroll_anchor(scroll_anchor, window, cx);
                self.change_selections(
                    SelectionEffects::default().nav_history(false),
                    window,
                    cx,
                    |s| s.select_ranges([offset..offset]),
                );
                true
            }
        } else {
            false
        }
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        self.buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .and_then(|file| File::from_dyn(Some(file)))
            .map(|file| {
                file.worktree
                    .read(cx)
                    .absolutize(&file.path)
                    .compact()
                    .to_string_lossy()
                    .into_owned()
                    .into()
            })
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn tab_content_text(&self, detail: usize, cx: &App) -> SharedString {
        if let Some(path) = path_for_buffer(&self.buffer, detail, true, cx) {
            path.to_string().into()
        } else {
            // Use the same logic as the displayed title for consistency
            self.buffer.read(cx).title(cx).to_string().into()
        }
    }

    fn suggested_filename(&self, cx: &App) -> SharedString {
        self.buffer.read(cx).title(cx).to_string().into()
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        ItemSettings::get_global(cx)
            .file_icons
            .then(|| {
                path_for_buffer(&self.buffer, 0, true, cx)
                    .and_then(|path| FileIcons::get_icon(Path::new(&*path), cx))
            })
            .flatten()
            .map(Icon::from_path)
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> AnyElement {
        let label_color = if ItemSettings::get_global(cx).git_status {
            self.buffer()
                .read(cx)
                .as_singleton()
                .and_then(|buffer| {
                    let buffer = buffer.read(cx);
                    let path = buffer.project_path(cx)?;
                    let buffer_id = buffer.remote_id();
                    let project = self.project()?.read(cx);
                    let entry = project.entry_for_path(&path, cx)?;
                    let (repo, repo_path) = project
                        .git_store()
                        .read(cx)
                        .repository_and_path_for_buffer_id(buffer_id, cx)?;
                    let status = repo.read(cx).status_for_path(&repo_path)?.status;

                    Some(entry_git_aware_label_color(
                        status.summary(),
                        entry.is_ignored,
                        params.selected,
                    ))
                })
                .unwrap_or_else(|| entry_label_color(params.selected))
        } else {
            entry_label_color(params.selected)
        };

        let description = params.detail.and_then(|detail| {
            let path = path_for_buffer(&self.buffer, detail, false, cx)?;
            let description = path.trim();

            if description.is_empty() {
                return None;
            }

            Some(util::truncate_and_trailoff(description, MAX_TAB_TITLE_LEN))
        });

        // Whether the file was saved in the past but is now deleted.
        let was_deleted: bool = self
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| file.disk_state().is_deleted());

        h_flex()
            .gap_2()
            .child(
                Label::new(util::truncate_and_trailoff(
                    &self.title(cx),
                    MAX_TAB_TITLE_LEN,
                ))
                .color(label_color)
                .when(params.preview, |this| this.italic())
                .when(was_deleted, |this| this.strikethrough()),
            )
            .when_some(description, |this, description| {
                this.child(
                    Label::new(description)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
            })
            .into_any_element()
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
        self.buffer
            .read(cx)
            .for_each_buffer(&mut |buffer| f(buffer.entity_id(), buffer.read(cx)));
    }

    fn buffer_kind(&self, cx: &App) -> ItemBufferKind {
        match self.buffer.read(cx).is_singleton() {
            true => ItemBufferKind::Singleton,
            false => ItemBufferKind::Multibuffer,
        }
    }

    fn can_save_as(&self, cx: &App) -> bool {
        self.buffer.read(cx).is_singleton()
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Editor>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| self.clone(window, cx))))
    }

    fn set_nav_history(
        &mut self,
        history: ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        self.nav_history = Some(history);
    }

    fn on_removed(&self, cx: &mut Context<Self>) {
        self.report_editor_event(ReportEditorEvent::Closed, None, cx);
    }

    fn deactivated(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let selection = self.selections.newest_anchor();
        self.push_to_nav_history(selection.head(), None, true, false, cx);
    }

    fn workspace_deactivated(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.hide_hovered_link(cx);
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.buffer().read(cx).read(cx).is_dirty()
    }

    fn capability(&self, cx: &App) -> Capability {
        self.capability(cx)
    }

    // Note: this mirrors the logic in `Editor::toggle_read_only`, but is reachable
    // without relying on focus-based action dispatch.
    fn toggle_read_only(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(buffer) = self.buffer.read(cx).as_singleton() {
            buffer.update(cx, |buffer, cx| {
                buffer.set_capability(
                    match buffer.capability() {
                        Capability::ReadWrite => Capability::Read,
                        Capability::Read => Capability::ReadWrite,
                        Capability::ReadOnly => Capability::ReadOnly,
                    },
                    cx,
                );
            });
        }
        cx.notify();
        window.refresh();
    }

    fn has_deleted_file(&self, cx: &App) -> bool {
        self.buffer().read(cx).read(cx).has_deleted_file()
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.buffer().read(cx).read(cx).has_conflict()
    }

    fn can_save(&self, cx: &App) -> bool {
        let buffer = &self.buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton() {
            buffer.read(cx).project_path(cx).is_some()
        } else {
            true
        }
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        // Add meta data tracking # of auto saves
        if options.autosave {
            self.report_editor_event(ReportEditorEvent::Saved { auto_saved: true }, None, cx);
        } else {
            self.report_editor_event(ReportEditorEvent::Saved { auto_saved: false }, None, cx);
        }

        let buffers = self.buffer().clone().read(cx).all_buffers();
        let buffers = buffers
            .into_iter()
            .map(|handle| handle.read(cx).base_buffer().unwrap_or(handle.clone()))
            .collect::<HashSet<_>>();

        let buffers_to_save = if self.buffer.read(cx).is_singleton() && !options.autosave {
            buffers
        } else {
            buffers
                .into_iter()
                .filter(|buffer| buffer.read(cx).is_dirty())
                .collect()
        };

        cx.spawn_in(window, async move |this, cx| {
            if options.format {
                this.update_in(cx, |editor, window, cx| {
                    editor.perform_format(
                        project.clone(),
                        FormatTrigger::Save,
                        FormatTarget::Buffers(buffers_to_save.clone()),
                        window,
                        cx,
                    )
                })?
                .await?;
            }

            if !buffers_to_save.is_empty() {
                project
                    .update(cx, |project, cx| {
                        project.save_buffers(buffers_to_save.clone(), cx)
                    })
                    .await?;
            }

            Ok(())
        })
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: ProjectPath,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer = self
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("cannot call save_as on an excerpt list");

        let file_extension = path.path.extension().map(|a| a.to_string());
        self.report_editor_event(
            ReportEditorEvent::Saved { auto_saved: false },
            file_extension,
            cx,
        );

        project.update(cx, |project, cx| project.save_buffer_as(buffer, path, cx))
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = self.buffer.read(cx).all_buffers();
        let reload_buffers =
            project.update(cx, |project, cx| project.reload_buffers(buffers, true, cx));
        cx.spawn_in(window, async move |this, cx| {
            let transaction = reload_buffers.log_err().await;
            this.update(cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::fit(), cx)
            })?;
            buffer.update(cx, |buffer, cx| {
                if let Some(transaction) = transaction
                    && !buffer.is_singleton()
                {
                    buffer.push_transaction(&transaction.0, cx);
                }
            });
            Ok(())
        })
    }

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn pixel_position_of_cursor(&self, _: &App) -> Option<gpui::Point<Pixels>> {
        self.pixel_position_of_newest_cursor
    }

    fn breadcrumb_location(&self, cx: &App) -> ToolbarItemLocation {
        if self.show_breadcrumbs && self.buffer().read(cx).is_singleton() {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    // In a non-singleton case, the breadcrumbs are actually shown on sticky file headers of the multibuffer.
    fn breadcrumbs(&self, cx: &App) -> Option<(Vec<HighlightedText>, Option<Font>)> {
        if self.buffer.read(cx).is_singleton() {
            let font = theme_settings::ThemeSettings::get_global(cx)
                .buffer_font
                .clone();
            Some((self.breadcrumbs_inner(cx)?, Some(font)))
        } else {
            None
        }
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace = Some((workspace.weak_handle(), workspace.database_id()));
        if let Some(workspace_entity) = &workspace.weak_handle().upgrade() {
            cx.subscribe(
                workspace_entity,
                |editor, _, event: &workspace::Event, _cx| {
                    if let workspace::Event::ModalOpened = event {
                        editor.mouse_context_menu.take();
                        editor.inline_blame_popover.take();
                    }
                },
            )
            .detach();
        }

        // Load persisted folds if this editor doesn't already have folds.
        // This handles manually-opened files (not workspace restoration).
        let display_snapshot = self
            .display_map
            .update(cx, |display_map, cx| display_map.snapshot(cx));
        let has_folds = display_snapshot
            .folds_in_range(MultiBufferOffset(0)..display_snapshot.buffer_snapshot().len())
            .next()
            .is_some();

        if !has_folds {
            if let Some(workspace_id) = workspace.database_id()
                && let Some(file_path) = self.buffer().read(cx).as_singleton().and_then(|buffer| {
                    project::File::from_dyn(buffer.read(cx).file()).map(|file| file.abs_path(cx))
                })
            {
                self.load_folds_from_db(workspace_id, file_path, window, cx);
            }
        }
    }

    fn pane_changed(&mut self, new_pane_id: EntityId, cx: &mut Context<Self>) {
        if self
            .highlighted_rows
            .get(&TypeId::of::<ActiveDebugLine>())
            .is_some_and(|lines| !lines.is_empty())
            && let Some(breakpoint_store) = self.breakpoint_store.as_ref()
        {
            breakpoint_store.update(cx, |store, _cx| {
                store.set_active_debug_pane_id(new_pane_id);
            });
        }
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        match event {
            EditorEvent::Saved | EditorEvent::TitleChanged => {
                f(ItemEvent::UpdateTab);
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::Reparsed(_) => {
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::SelectionsChanged { local } if *local => {
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::BreadcrumbsChanged => {
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::DirtyChanged => {
                f(ItemEvent::UpdateTab);
            }

            EditorEvent::BufferEdited => {
                f(ItemEvent::Edit);
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::BufferRangesUpdated { .. } | EditorEvent::BuffersRemoved { .. } => {
                f(ItemEvent::Edit);
            }

            _ => {}
        }
    }

    fn tab_extra_context_menu_actions(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<(SharedString, Box<dyn gpui::Action>)> {
        let mut actions = Vec::new();

        let is_markdown = self
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).language())
            .is_some_and(|language| language.name().as_ref() == "Markdown");

        let is_svg = self
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .is_some_and(|file| {
                std::path::Path::new(file.file_name(cx))
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"))
            });

        if is_markdown {
            actions.push((
                "Open Markdown Preview".into(),
                Box::new(OpenMarkdownPreview) as Box<dyn gpui::Action>,
            ));
        }

        if is_svg {
            actions.push((
                "Open SVG Preview".into(),
                Box::new(OpenSvgPreview) as Box<dyn gpui::Action>,
            ));
        }

        actions
    }

    fn preserve_preview(&self, cx: &App) -> bool {
        self.buffer.read(cx).preserve_preview(cx)
    }
}

impl SerializableItem for Editor {
    fn serialized_item_kind() -> &'static str {
        "Editor"
    }

    fn cleanup(
        workspace_id: WorkspaceId,
        alive_items: Vec<ItemId>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<()>> {
        workspace::delete_unloaded_items(
            alive_items,
            workspace_id,
            "editors",
            &EditorDb::global(cx),
            cx,
        )
    }

    fn deserialize(
        project: Entity<Project>,
        _workspace: WeakEntity<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: ItemId,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let serialized_editor = match EditorDb::global(cx)
            .get_serialized_editor(item_id, workspace_id)
            .context("Failed to query editor state")
        {
            Ok(Some(serialized_editor)) => {
                if ProjectSettings::get_global(cx)
                    .session
                    .restore_unsaved_buffers
                {
                    serialized_editor
                } else {
                    SerializedEditor {
                        abs_path: serialized_editor.abs_path,
                        contents: None,
                        language: None,
                        mtime: None,
                    }
                }
            }
            Ok(None) => {
                return Task::ready(Err(anyhow!(
                    "Unable to deserialize editor: No entry in database for item_id: {item_id} and workspace_id {workspace_id:?}"
                )));
            }
            Err(error) => {
                return Task::ready(Err(error));
            }
        };
        log::debug!(
            "Deserialized editor {item_id:?} in workspace {workspace_id:?}, {serialized_editor:?}"
        );

        match serialized_editor {
            SerializedEditor {
                abs_path: None,
                contents: Some(contents),
                language,
                ..
            } => window.spawn(cx, {
                let project = project.clone();
                async move |cx| {
                    let language_registry =
                        project.read_with(cx, |project, _| project.languages().clone());

                    let language = if let Some(language_name) = language {
                        // We don't fail here, because we'd rather not set the language if the name changed
                        // than fail to restore the buffer.
                        language_registry
                            .language_for_name(&language_name)
                            .await
                            .ok()
                    } else {
                        None
                    };

                    // First create the empty buffer
                    let buffer = project
                        .update(cx, |project, cx| project.create_buffer(language, true, cx))
                        .await
                        .context("Failed to create buffer while deserializing editor")?;

                    // Then set the text so that the dirty bit is set correctly
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_language_registry(language_registry);
                        buffer.set_text(contents, cx);
                        if let Some(entry) = buffer.peek_undo_stack() {
                            buffer.forget_transaction(entry.transaction_id());
                        }
                    });

                    cx.update(|window, cx| {
                        cx.new(|cx| {
                            let mut editor = Editor::for_buffer(buffer, Some(project), window, cx);

                            editor.read_metadata_from_db(item_id, workspace_id, window, cx);
                            editor
                        })
                    })
                }
            }),
            SerializedEditor {
                abs_path: Some(abs_path),
                contents,
                mtime,
                ..
            } => {
                let opened_buffer = project.update(cx, |project, cx| {
                    let (worktree, path) = project.find_worktree(&abs_path, cx)?;
                    let project_path = ProjectPath {
                        worktree_id: worktree.read(cx).id(),
                        path: path,
                    };
                    Some(project.open_path(project_path, cx))
                });

                match opened_buffer {
                    Some(opened_buffer) => window.spawn(cx, async move |cx| {
                        let (_, buffer) = opened_buffer
                            .await
                            .context("Failed to open path in project")?;

                        if let Some(contents) = contents {
                            buffer.update(cx, |buffer, cx| {
                                restore_serialized_buffer_contents(buffer, contents, mtime, cx);
                            });
                        }

                        cx.update(|window, cx| {
                            cx.new(|cx| {
                                let mut editor =
                                    Editor::for_buffer(buffer, Some(project), window, cx);

                                editor.read_metadata_from_db(item_id, workspace_id, window, cx);
                                editor
                            })
                        })
                    }),
                    None => {
                        // File is not in any worktree (e.g., opened as a standalone file).
                        // Open the buffer directly via the project rather than through
                        // workspace.open_abs_path(), which has the side effect of adding
                        // the item to a pane. The caller (deserialize_to) will add the
                        // returned item to the correct pane.
                        window.spawn(cx, async move |cx| {
                            let buffer = project
                                .update(cx, |project, cx| project.open_local_buffer(&abs_path, cx))
                                .await
                                .with_context(|| {
                                    format!("Failed to open buffer for {abs_path:?}")
                                })?;

                            if let Some(contents) = contents {
                                buffer.update(cx, |buffer, cx| {
                                    restore_serialized_buffer_contents(buffer, contents, mtime, cx);
                                });
                            }

                            cx.update(|window, cx| {
                                cx.new(|cx| {
                                    let mut editor =
                                        Editor::for_buffer(buffer, Some(project), window, cx);
                                    editor.read_metadata_from_db(item_id, workspace_id, window, cx);
                                    editor
                                })
                            })
                        })
                    }
                }
            }
            SerializedEditor {
                abs_path: None,
                contents: None,
                ..
            } => window.spawn(cx, async move |cx| {
                let buffer = project
                    .update(cx, |project, cx| project.create_buffer(None, true, cx))
                    .await
                    .context("Failed to create buffer")?;

                cx.update(|window, cx| {
                    cx.new(|cx| {
                        let mut editor = Editor::for_buffer(buffer, Some(project), window, cx);

                        editor.read_metadata_from_db(item_id, workspace_id, window, cx);
                        editor
                    })
                })
            }),
        }
    }

    fn serialize(
        &mut self,
        workspace: &mut Workspace,
        item_id: ItemId,
        closing: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        let buffer_serialization = self.buffer_serialization?;
        let project = self.project.clone()?;

        let serialize_dirty_buffers = match buffer_serialization {
            // Always serialize dirty buffers, including for worktree-less windows.
            // This enables hot-exit functionality for empty windows and single files.
            BufferSerialization::All => true,
            BufferSerialization::NonDirtyBuffers => false,
        };

        if closing && !serialize_dirty_buffers {
            return None;
        }

        let workspace_id = workspace.database_id()?;

        let buffer = self.buffer().read(cx).as_singleton()?;

        let abs_path = buffer.read(cx).file().and_then(|file| {
            let worktree_id = file.worktree_id(cx);
            project
                .read(cx)
                .worktree_for_id(worktree_id, cx)
                .map(|worktree| worktree.read(cx).absolutize(file.path()))
                .or_else(|| {
                    let full_path = file.full_path(cx);
                    let project_path = project.read(cx).find_project_path(&full_path, cx)?;
                    project.read(cx).absolute_path(&project_path, cx)
                })
        });

        let is_dirty = buffer.read(cx).is_dirty();
        let mtime = buffer.read(cx).saved_mtime();

        let snapshot = buffer.read(cx).snapshot();

        let db = EditorDb::global(cx);
        Some(cx.spawn_in(window, async move |_this, cx| {
            cx.background_spawn(async move {
                let (contents, language) = if serialize_dirty_buffers && is_dirty {
                    let contents = snapshot.text();
                    let language = snapshot.language().map(|lang| lang.name().to_string());
                    (Some(contents), language)
                } else {
                    (None, None)
                };

                let editor = SerializedEditor {
                    abs_path,
                    contents,
                    language,
                    mtime,
                };
                log::debug!("Serializing editor {item_id:?} in workspace {workspace_id:?}");
                db.save_serialized_editor(item_id, workspace_id, editor)
                    .await
                    .context("failed to save serialized editor")
            })
            .await
            .context("failed to save contents of buffer")?;

            Ok(())
        }))
    }

    fn should_serialize(&self, event: &Self::Event) -> bool {
        self.should_serialize_buffer()
            && matches!(
                event,
                EditorEvent::Saved | EditorEvent::DirtyChanged | EditorEvent::BufferEdited
            )
    }
}

#[derive(Debug, Default)]
struct EditorRestorationData {
    entries: HashMap<PathBuf, RestorationData>,
}

#[derive(Default, Debug)]
pub struct RestorationData {
    pub scroll_position: (BufferRow, gpui::Point<ScrollOffset>),
    pub folds: Vec<Range<Point>>,
    pub selections: Vec<Range<Point>>,
}

impl ProjectItem for Editor {
    type Item = Buffer;

    fn project_item_kind() -> Option<ProjectItemKind> {
        Some(ProjectItemKind("Editor"))
    }

    fn for_project_item(
        project: Entity<Project>,
        pane: Option<&Pane>,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut editor = Self::for_buffer(buffer.clone(), Some(project), window, cx);
        let multibuffer_snapshot = editor.buffer().read(cx).snapshot(cx);

        if let Some(buffer_snapshot) = editor.buffer().read(cx).snapshot(cx).as_singleton()
            && WorkspaceSettings::get(None, cx).restore_on_file_reopen
            && let Some(restoration_data) = Self::project_item_kind()
                .and_then(|kind| pane.as_ref()?.project_item_restoration_data.get(&kind))
                .and_then(|data| data.downcast_ref::<EditorRestorationData>())
                .and_then(|data| {
                    let file = project::File::from_dyn(buffer.read(cx).file())?;
                    data.entries.get(&file.abs_path(cx))
                })
        {
            if !restoration_data.folds.is_empty() {
                editor.fold_ranges(
                    clip_ranges(&restoration_data.folds, buffer_snapshot),
                    false,
                    window,
                    cx,
                );
            }
            if !restoration_data.selections.is_empty() {
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges(clip_ranges(&restoration_data.selections, buffer_snapshot));
                });
            }
            let (top_row, offset) = restoration_data.scroll_position;
            let anchor = multibuffer_snapshot.anchor_before(Point::new(top_row, 0));
            editor.set_scroll_anchor(ScrollAnchor { anchor, offset }, window, cx);
        }

        editor
    }

    fn for_broken_project_item(
        abs_path: &Path,
        is_local: bool,
        e: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<InvalidItemView> {
        Some(InvalidItemView::new(abs_path, is_local, e, window, cx))
    }
}

fn clip_ranges<'a>(
    original: impl IntoIterator<Item = &'a Range<Point>> + 'a,
    snapshot: &'a BufferSnapshot,
) -> Vec<Range<Point>> {
    original
        .into_iter()
        .map(|range| {
            snapshot.clip_point(range.start, Bias::Left)
                ..snapshot.clip_point(range.end, Bias::Right)
        })
        .collect()
}

impl EventEmitter<SearchEvent> for Editor {}

impl Editor {
    pub fn update_restoration_data(
        &self,
        cx: &mut Context<Self>,
        write: impl for<'a> FnOnce(&'a mut RestorationData) + 'static,
    ) {
        if self.mode.is_minimap() || !WorkspaceSettings::get(None, cx).restore_on_file_reopen {
            return;
        }

        let editor = cx.entity();
        cx.defer(move |cx| {
            editor.update(cx, |editor, cx| {
                let kind = Editor::project_item_kind()?;
                let pane = editor.workspace()?.read(cx).pane_for(&cx.entity())?;
                let buffer = editor.buffer().read(cx).as_singleton()?;
                let file_abs_path = project::File::from_dyn(buffer.read(cx).file())?.abs_path(cx);
                pane.update(cx, |pane, _| {
                    let data = pane
                        .project_item_restoration_data
                        .entry(kind)
                        .or_insert_with(|| Box::new(EditorRestorationData::default()) as Box<_>);
                    let data = match data.downcast_mut::<EditorRestorationData>() {
                        Some(data) => data,
                        None => {
                            *data = Box::new(EditorRestorationData::default());
                            data.downcast_mut::<EditorRestorationData>()
                                .expect("just written the type downcasted to")
                        }
                    };

                    let data = data.entries.entry(file_abs_path).or_default();
                    write(data);
                    Some(())
                })
            });
        });
    }
}

impl SearchableItem for Editor {
    type Match = Range<Anchor>;

    fn get_matches(&self, _window: &mut Window, _: &mut App) -> (Vec<Range<Anchor>>, SearchToken) {
        (
            self.background_highlights
                .get(&HighlightKey::BufferSearchHighlights)
                .map_or(Vec::new(), |(_color, ranges)| {
                    ranges.iter().cloned().collect()
                }),
            SearchToken::default(),
        )
    }

    fn clear_matches(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self
            .clear_background_highlights(HighlightKey::BufferSearchHighlights, cx)
            .is_some()
        {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
    }

    fn update_matches(
        &mut self,
        matches: &[Range<Anchor>],
        active_match_index: Option<usize>,
        _token: SearchToken,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let existing_range = self
            .background_highlights
            .get(&HighlightKey::BufferSearchHighlights)
            .map(|(_, range)| range.as_ref());
        let updated = existing_range != Some(matches);
        self.highlight_background(
            HighlightKey::BufferSearchHighlights,
            matches,
            move |index, theme| {
                if active_match_index == Some(*index) {
                    theme.colors().search_active_match_background
                } else {
                    theme.colors().search_match_background
                }
            },
            cx,
        );
        if updated {
            cx.emit(SearchEvent::MatchesInvalidated);
        }
    }

    fn has_filtered_search_ranges(&mut self) -> bool {
        self.has_background_highlights(HighlightKey::SearchWithinRange)
    }

    fn toggle_filtered_search_ranges(
        &mut self,
        enabled: Option<FilteredSearchRange>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.has_filtered_search_ranges() {
            self.previous_search_ranges = self
                .clear_background_highlights(HighlightKey::SearchWithinRange, cx)
                .map(|(_, ranges)| ranges)
        }

        if let Some(range) = enabled {
            let ranges = self.selections.disjoint_anchor_ranges().collect::<Vec<_>>();

            if ranges.iter().any(|s| s.start != s.end) {
                self.set_search_within_ranges(&ranges, cx);
            } else if let Some(previous_search_ranges) = self.previous_search_ranges.take()
                && range != FilteredSearchRange::Selection
            {
                self.set_search_within_ranges(&previous_search_ranges, cx);
            }
        }
    }

    fn supported_options(&self) -> SearchOptions {
        if self.in_project_search {
            SearchOptions {
                case: true,
                word: true,
                regex: true,
                replacement: false,
                selection: false,
                select_all: true,
                find_in_results: true,
            }
        } else {
            SearchOptions {
                case: true,
                word: true,
                regex: true,
                replacement: true,
                selection: true,
                select_all: true,
                find_in_results: false,
            }
        }
    }

    fn query_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) -> String {
        let setting = EditorSettings::get_global(cx).seed_search_query_from_cursor;
        let snapshot = self.snapshot(window, cx);
        let selection = self.selections.newest_adjusted(&snapshot.display_snapshot);
        let buffer_snapshot = snapshot.buffer_snapshot();

        match setting {
            SeedQuerySetting::Never => String::new(),
            SeedQuerySetting::Selection | SeedQuerySetting::Always if !selection.is_empty() => {
                buffer_snapshot
                    .text_for_range(selection.start..selection.end)
                    .collect()
            }
            SeedQuerySetting::Selection => String::new(),
            SeedQuerySetting::Always => {
                let (range, kind) = buffer_snapshot
                    .surrounding_word(selection.start, Some(CharScopeContext::Completion));
                if kind == Some(CharKind::Word) {
                    let text: String = buffer_snapshot.text_for_range(range).collect();
                    if !text.trim().is_empty() {
                        return text;
                    }
                }
                String::new()
            }
        }
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Range<Anchor>],
        _token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.unfold_ranges(&[matches[index].clone()], false, true, cx);
        let range = self.range_for_match(&matches[index]);
        let autoscroll = if EditorSettings::get_global(cx).search.center_on_match {
            Autoscroll::center()
        } else {
            Autoscroll::fit()
        };
        self.change_selections(SelectionEffects::scroll(autoscroll), window, cx, |s| {
            s.select_ranges([range]);
        })
    }

    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        _token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.unfold_ranges(matches, false, false, cx);
        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges(matches.iter().cloned())
        });
    }
    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &SearchQuery,
        _token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text = self.buffer.read(cx);
        let text = text.snapshot(cx);
        let text = text.text_for_range(identifier.clone()).collect::<Vec<_>>();
        let text: Cow<_> = if text.len() == 1 {
            text.first().cloned().unwrap().into()
        } else {
            let joined_chunks = text.join("");
            joined_chunks.into()
        };

        if let Some(replacement) = query.replacement_for(&text) {
            self.transact(window, cx, |this, _, cx| {
                this.edit([(identifier.clone(), Arc::from(&*replacement))], cx);
            });
        }
    }
    fn replace_all(
        &mut self,
        matches: &mut dyn Iterator<Item = &Self::Match>,
        query: &SearchQuery,
        _token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let text = self.buffer.read(cx);
        let text = text.snapshot(cx);
        let mut edits = vec![];

        // A regex might have replacement variables so we cannot apply
        // the same replacement to all matches
        if query.is_regex() {
            edits = matches
                .filter_map(|m| {
                    let text = text.text_for_range(m.clone()).collect::<Vec<_>>();

                    let text: Cow<_> = if text.len() == 1 {
                        text.first().cloned().unwrap().into()
                    } else {
                        let joined_chunks = text.join("");
                        joined_chunks.into()
                    };

                    query
                        .replacement_for(&text)
                        .map(|replacement| (m.clone(), Arc::from(&*replacement)))
                })
                .collect();
        } else if let Some(replacement) = query.replacement().map(Arc::<str>::from) {
            edits = matches.map(|m| (m.clone(), replacement.clone())).collect();
        }

        if !edits.is_empty() {
            self.transact(window, cx, |this, _, cx| {
                this.edit(edits, cx);
            });
        }
    }
    fn match_index_for_direction(
        &mut self,
        matches: &[Range<Anchor>],
        current_index: usize,
        direction: Direction,
        count: usize,
        _token: SearchToken,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> usize {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let current_index_position = if self.selections.disjoint_anchors_arc().len() == 1 {
            self.selections.newest_anchor().head()
        } else {
            matches[current_index].start
        };

        let mut count = count % matches.len();
        if count == 0 {
            return current_index;
        }
        match direction {
            Direction::Next => {
                if matches[current_index]
                    .start
                    .cmp(&current_index_position, &buffer)
                    .is_gt()
                {
                    count -= 1
                }

                (current_index + count) % matches.len()
            }
            Direction::Prev => {
                if matches[current_index]
                    .end
                    .cmp(&current_index_position, &buffer)
                    .is_lt()
                {
                    count -= 1;
                }

                if current_index >= count {
                    current_index - count
                } else {
                    matches.len() - (count - current_index)
                }
            }
        }
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<Range<Anchor>>> {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let search_within_ranges = self
            .background_highlights
            .get(&HighlightKey::SearchWithinRange)
            .map_or(vec![], |(_color, ranges)| {
                ranges.iter().cloned().collect::<Vec<_>>()
            });

        cx.background_spawn(async move {
            let mut ranges = Vec::new();

            let search_within_ranges = if search_within_ranges.is_empty() {
                vec![buffer.anchor_before(MultiBufferOffset(0))..buffer.anchor_after(buffer.len())]
            } else {
                search_within_ranges
            };

            for range in search_within_ranges {
                for (search_buffer, search_range, deleted_hunk_anchor) in
                    buffer.range_to_buffer_ranges_with_deleted_hunks(range)
                {
                    ranges.extend(
                        query
                            .search(
                                search_buffer,
                                Some(search_range.start.0..search_range.end.0),
                            )
                            .await
                            .into_iter()
                            .filter_map(|match_range| {
                                if let Some(deleted_hunk_anchor) = deleted_hunk_anchor {
                                    let start = search_buffer
                                        .anchor_after(search_range.start + match_range.start);
                                    let end = search_buffer
                                        .anchor_before(search_range.start + match_range.end);
                                    Some(
                                        deleted_hunk_anchor.with_diff_base_anchor(start)
                                            ..deleted_hunk_anchor.with_diff_base_anchor(end),
                                    )
                                } else {
                                    let start = search_buffer
                                        .anchor_after(search_range.start + match_range.start);
                                    let end = search_buffer
                                        .anchor_before(search_range.start + match_range.end);
                                    buffer.buffer_anchor_range_to_anchor_range(start..end)
                                }
                            }),
                    );
                }
            }

            ranges
        })
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Range<Anchor>],
        _token: SearchToken,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        active_match_index(
            direction,
            matches,
            &self.selections.newest_anchor().head(),
            &self.buffer().read(cx).snapshot(cx),
        )
    }

    fn search_bar_visibility_changed(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) {
        self.expect_bounds_change = self.last_bounds;
    }

    fn set_search_is_case_sensitive(
        &mut self,
        case_sensitive: Option<bool>,
        _cx: &mut Context<Self>,
    ) {
        self.select_next_is_case_sensitive = case_sensitive;
    }
}

pub fn active_match_index(
    direction: Direction,
    ranges: &[Range<Anchor>],
    cursor: &Anchor,
    buffer: &MultiBufferSnapshot,
) -> Option<usize> {
    if ranges.is_empty() {
        None
    } else {
        let r = ranges.binary_search_by(|probe| {
            if probe.end.cmp(cursor, buffer).is_lt() {
                Ordering::Less
            } else if probe.start.cmp(cursor, buffer).is_gt() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        match direction {
            Direction::Prev => match r {
                Ok(i) => Some(i),
                Err(i) => Some(i.saturating_sub(1)),
            },
            Direction::Next => match r {
                Ok(i) | Err(i) => Some(cmp::min(i, ranges.len() - 1)),
            },
        }
    }
}

pub fn entry_label_color(selected: bool) -> Color {
    if selected {
        Color::Default
    } else {
        Color::Muted
    }
}

pub fn entry_diagnostic_aware_icon_name_and_color(
    diagnostic_severity: Option<DiagnosticSeverity>,
) -> Option<(IconName, Color)> {
    match diagnostic_severity {
        Some(DiagnosticSeverity::ERROR) => Some((IconName::Close, Color::Error)),
        Some(DiagnosticSeverity::WARNING) => Some((IconName::Triangle, Color::Warning)),
        _ => None,
    }
}

pub fn entry_diagnostic_aware_icon_decoration_and_color(
    diagnostic_severity: Option<DiagnosticSeverity>,
) -> Option<(IconDecorationKind, Color)> {
    match diagnostic_severity {
        Some(DiagnosticSeverity::ERROR) => Some((IconDecorationKind::X, Color::Error)),
        Some(DiagnosticSeverity::WARNING) => Some((IconDecorationKind::Triangle, Color::Warning)),
        _ => None,
    }
}

pub fn entry_git_aware_label_color(git_status: GitSummary, ignored: bool, selected: bool) -> Color {
    let tracked = git_status.index + git_status.worktree;
    if git_status.conflict > 0 {
        Color::Conflict
    } else if tracked.deleted > 0 {
        Color::Deleted
    } else if tracked.modified > 0 {
        Color::Modified
    } else if tracked.added > 0 || git_status.untracked > 0 {
        Color::Created
    } else if ignored {
        Color::Ignored
    } else {
        entry_label_color(selected)
    }
}

fn path_for_buffer<'a>(
    buffer: &Entity<MultiBuffer>,
    height: usize,
    include_filename: bool,
    cx: &'a App,
) -> Option<Cow<'a, str>> {
    let file = buffer.read(cx).as_singleton()?.read(cx).file()?;
    path_for_file(file, height, include_filename, cx)
}

fn path_for_file<'a>(
    file: &'a Arc<dyn language::File>,
    mut height: usize,
    include_filename: bool,
    cx: &'a App,
) -> Option<Cow<'a, str>> {
    if project::File::from_dyn(Some(file)).is_none() {
        return None;
    }

    let file = file.as_ref();
    // Ensure we always render at least the filename.
    height += 1;

    let mut prefix = file.path().as_ref();
    while height > 0 {
        if let Some(parent) = prefix.parent() {
            prefix = parent;
            height -= 1;
        } else {
            break;
        }
    }

    // The full_path method allocates, so avoid calling it if height is zero.
    if height > 0 {
        let mut full_path = file.full_path(cx);
        if !include_filename {
            if !full_path.pop() {
                return None;
            }
        }
        Some(full_path.to_string_lossy().into_owned().into())
    } else {
        let mut path = file.path().strip_prefix(prefix).ok()?;
        if !include_filename {
            path = path.parent()?;
        }
        Some(path.display(file.path_style(cx)))
    }
}

/// Restores serialized buffer contents by overwriting the buffer with saved text.
/// This is somewhat wasteful since we load the whole buffer from disk then overwrite it,
/// but keeps implementation simple as we don't need to persist all metadata from loading
/// (git diff base, etc.).
fn restore_serialized_buffer_contents(
    buffer: &mut Buffer,
    contents: String,
    mtime: Option<MTime>,
    cx: &mut Context<Buffer>,
) {
    // If we did restore an mtime, store it on the buffer so that
    // the next edit will mark the buffer as dirty/conflicted.
    if mtime.is_some() {
        buffer.did_reload(buffer.version(), buffer.line_ending(), mtime, cx);
    }
    buffer.set_text(contents, cx);
    if let Some(entry) = buffer.peek_undo_stack() {
        buffer.forget_transaction(entry.transaction_id());
    }
}

fn serialize_path_key(path_key: &PathKey) -> proto::PathKey {
    proto::PathKey {
        sort_prefix: path_key.sort_prefix,
        path: path_key.path.to_proto(),
    }
}

fn deserialize_path_key(path_key: proto::PathKey) -> Option<PathKey> {
    Some(PathKey {
        sort_prefix: path_key.sort_prefix,
        path: RelPath::from_proto(&path_key.path).ok()?,
    })
}

#[cfg(test)]
mod tests {
    use crate::editor_tests::init_test;
    use fs::Fs;
    use workspace::MultiWorkspace;

    use super::*;
    use fs::MTime;
    use gpui::{App, VisualTestContext};
    use language::TestFile;
    use project::FakeFs;
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use util::{path, rel_path::RelPath};

    #[gpui::test]
    fn test_path_for_file(cx: &mut App) {
        let file: Arc<dyn language::File> = Arc::new(TestFile {
            path: RelPath::empty().into(),
            root_name: String::new(),
            local_root: None,
        });
        assert_eq!(path_for_file(&file, 0, false, cx), None);
    }

    async fn deserialize_editor(
        item_id: ItemId,
        workspace_id: WorkspaceId,
        workspace: Entity<Workspace>,
        project: Entity<Project>,
        cx: &mut VisualTestContext,
    ) -> Entity<Editor> {
        workspace
            .update_in(cx, |workspace, window, cx| {
                let pane = workspace.active_pane();
                pane.update(cx, |_, cx| {
                    Editor::deserialize(
                        project.clone(),
                        workspace.weak_handle(),
                        workspace_id,
                        item_id,
                        window,
                        cx,
                    )
                })
            })
            .await
            .unwrap()
    }

    #[gpui::test]
    async fn test_deserialize(cx: &mut gpui::TestAppContext) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_file(path!("/file.rs"), Default::default()).await;

        // Test case 1: Deserialize with path and contents
        {
            let project = Project::test(fs.clone(), [path!("/file.rs").as_ref()], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let workspace_id = db.next_id().await.unwrap();
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));
            let item_id = 1234 as ItemId;
            let mtime = fs
                .metadata(Path::new(path!("/file.rs")))
                .await
                .unwrap()
                .unwrap()
                .mtime;

            let serialized_editor = SerializedEditor {
                abs_path: Some(PathBuf::from(path!("/file.rs"))),
                contents: Some("fn main() {}".to_string()),
                language: Some("Rust".to_string()),
                mtime: Some(mtime),
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor.clone())
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "fn main() {}");
                assert!(editor.is_dirty(cx));
                assert!(!editor.has_conflict(cx));
                let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                assert!(buffer.file().is_some());
            });
        }

        // Test case 2: Deserialize with only path
        {
            let project = Project::test(fs.clone(), [path!("/file.rs").as_ref()], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));

            let workspace_id = db.next_id().await.unwrap();

            let item_id = 5678 as ItemId;
            let serialized_editor = SerializedEditor {
                abs_path: Some(PathBuf::from(path!("/file.rs"))),
                contents: None,
                language: None,
                mtime: None,
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor)
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), ""); // The file should be empty as per our initial setup
                assert!(!editor.is_dirty(cx));
                assert!(!editor.has_conflict(cx));

                let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                assert!(buffer.file().is_some());
            });
        }

        // Test case 3: Deserialize with no path (untitled buffer, with content and language)
        {
            let project = Project::test(fs.clone(), [path!("/file.rs").as_ref()], cx).await;
            // Add Rust to the language, so that we can restore the language of the buffer
            project.read_with(cx, |project, _| {
                project.languages().add(languages::rust_lang())
            });

            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));

            let workspace_id = db.next_id().await.unwrap();

            let item_id = 9012 as ItemId;
            let serialized_editor = SerializedEditor {
                abs_path: None,
                contents: Some("hello".to_string()),
                language: Some("Rust".to_string()),
                mtime: None,
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor)
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "hello");
                assert!(editor.is_dirty(cx)); // The editor should be dirty for an untitled buffer

                let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                assert_eq!(
                    buffer.language().map(|lang| lang.name()),
                    Some("Rust".into())
                ); // Language should be set to Rust
                assert!(buffer.file().is_none()); // The buffer should not have an associated file
            });
        }

        // Test case 4: Deserialize with path, content, and old mtime
        {
            let project = Project::test(fs.clone(), [path!("/file.rs").as_ref()], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));

            let workspace_id = db.next_id().await.unwrap();

            let item_id = 9345 as ItemId;
            let old_mtime = MTime::from_seconds_and_nanos(0, 50);
            let serialized_editor = SerializedEditor {
                abs_path: Some(PathBuf::from(path!("/file.rs"))),
                contents: Some("fn main() {}".to_string()),
                language: Some("Rust".to_string()),
                mtime: Some(old_mtime),
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor)
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "fn main() {}");
                assert!(editor.has_conflict(cx)); // The editor should have a conflict
            });
        }

        // Test case 5: Deserialize with no path, no content, no language, and no old mtime (new, empty, unsaved buffer)
        {
            let project = Project::test(fs.clone(), [path!("/file.rs").as_ref()], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));

            let workspace_id = db.next_id().await.unwrap();

            let item_id = 10000 as ItemId;
            let serialized_editor = SerializedEditor {
                abs_path: None,
                contents: None,
                language: None,
                mtime: None,
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor)
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "");
                assert!(!editor.is_dirty(cx));
                assert!(!editor.has_conflict(cx));

                let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                assert!(buffer.file().is_none());
            });
        }

        // Test case 6: Deserialize with path and contents in an empty workspace (no worktree)
        // This tests the hot-exit scenario where a file is opened in an empty workspace
        // and has unsaved changes that should be restored.
        {
            let fs = FakeFs::new(cx.executor());
            fs.insert_file(path!("/standalone.rs"), "original content".into())
                .await;

            // Create an empty project with no worktrees
            let project = Project::test(fs.clone(), [], cx).await;
            let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
                MultiWorkspace::test_new(project.clone(), window, cx)
            });
            let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
            let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
            let editor_db = cx.update(|_, cx| EditorDb::global(cx));

            let workspace_id = db.next_id().await.unwrap();
            let item_id = 11000 as ItemId;

            let mtime = fs
                .metadata(Path::new(path!("/standalone.rs")))
                .await
                .unwrap()
                .unwrap()
                .mtime;

            // Simulate serialized state: file with unsaved changes
            let serialized_editor = SerializedEditor {
                abs_path: Some(PathBuf::from(path!("/standalone.rs"))),
                contents: Some("modified content".to_string()),
                language: Some("Rust".to_string()),
                mtime: Some(mtime),
            };

            editor_db
                .save_serialized_editor(item_id, workspace_id, serialized_editor)
                .await
                .unwrap();

            let deserialized =
                deserialize_editor(item_id, workspace_id, workspace, project, cx).await;

            deserialized.update(cx, |editor, cx| {
                // The editor should have the serialized contents, not the disk contents
                assert_eq!(editor.text(cx), "modified content");
                assert!(editor.is_dirty(cx));
                assert!(!editor.has_conflict(cx));

                let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                assert!(buffer.file().is_some());
            });
        }
    }

    // Regression test for https://github.com/zed-industries/zed/issues/35947
    // Verifies that deserializing a non-worktree editor does not add the item
    // to any pane as a side effect.
    #[gpui::test]
    async fn test_deserialize_non_worktree_file_does_not_add_to_pane(
        cx: &mut gpui::TestAppContext,
    ) {
        init_test(cx, |_| {});

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/outside"), json!({ "settings.json": "{}" }))
            .await;

        // Project with a different root — settings.json is NOT in any worktree
        let project = Project::test(fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        let db = cx.update(|_, cx| workspace::WorkspaceDb::global(cx));
        let editor_db = cx.update(|_, cx| EditorDb::global(cx));

        let workspace_id = db.next_id().await.unwrap();
        let item_id = 99999 as ItemId;

        let serialized_editor = SerializedEditor {
            abs_path: Some(PathBuf::from(path!("/outside/settings.json"))),
            contents: None,
            language: None,
            mtime: None,
        };

        editor_db
            .save_serialized_editor(item_id, workspace_id, serialized_editor)
            .await
            .unwrap();

        // Count items in all panes before deserialization
        let pane_items_before = workspace.read_with(cx, |workspace, cx| {
            workspace
                .panes()
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .sum::<usize>()
        });

        let deserialized =
            deserialize_editor(item_id, workspace_id, workspace.clone(), project, cx).await;

        cx.run_until_parked();

        // The editor should exist and have the file
        deserialized.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
            assert!(buffer.file().is_some());
        });

        // No items should have been added to any pane as a side effect
        let pane_items_after = workspace.read_with(cx, |workspace, cx| {
            workspace
                .panes()
                .iter()
                .map(|pane| pane.read(cx).items_len())
                .sum::<usize>()
        });

        assert_eq!(
            pane_items_before, pane_items_after,
            "Editor::deserialize should not add items to panes as a side effect"
        );
    }
}
