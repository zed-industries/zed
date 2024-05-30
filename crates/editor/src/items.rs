use crate::{
    editor_settings::SeedQuerySetting, persistence::DB, scroll::ScrollAnchor, Anchor, Autoscroll,
    Editor, EditorEvent, EditorSettings, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot,
    NavigationData, SearchWithinRange, ToPoint as _,
};
use anyhow::{anyhow, Context as _, Result};
use collections::HashSet;
use futures::future::try_join_all;
use git::repository::GitFileStatus;
use gpui::{
    point, AnyElement, AppContext, AsyncWindowContext, Context, Entity, EntityId, EventEmitter,
    IntoElement, Model, ParentElement, Pixels, SharedString, Styled, Task, View, ViewContext,
    VisualContext, WeakView, WindowContext,
};
use language::{
    proto::serialize_anchor as serialize_text_anchor, Bias, Buffer, CharKind, OffsetRangeExt,
    Point, SelectionGoal,
};
use multi_buffer::AnchorRangeExt;
use project::{search::SearchQuery, FormatTrigger, Item as _, Project, ProjectPath};
use rpc::proto::{self, update_view, PeerId};
use settings::Settings;
use workspace::item::{ItemSettings, TabContentParams};

use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{self, Ordering},
    iter,
    ops::Range,
    path::Path,
    sync::Arc,
};
use text::{BufferId, Selection};
use theme::{Theme, ThemeSettings};
use ui::{h_flex, prelude::*, Label};
use util::{paths::PathExt, ResultExt, TryFutureExt};
use workspace::item::{BreadcrumbText, FollowEvent, FollowableItemHandle};
use workspace::{
    item::{FollowableItem, Item, ItemEvent, ItemHandle, ProjectItem},
    searchable::{Direction, SearchEvent, SearchableItem, SearchableItemHandle},
    ItemId, ItemNavHistory, Pane, ToolbarItemLocation, ViewId, Workspace, WorkspaceId,
};

pub const MAX_TAB_TITLE_LEN: usize = 24;

impl FollowableItem for Editor {
    fn remote_id(&self) -> Option<ViewId> {
        self.remote_id
    }

    fn from_state_proto(
        pane: View<workspace::Pane>,
        workspace: View<Workspace>,
        remote_id: ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<Task<Result<View<Self>>>> {
        let project = workspace.read(cx).project().to_owned();
        let Some(proto::view::Variant::Editor(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::Editor(state)) = state.take() else {
            unreachable!()
        };

        let client = project.read(cx).client();
        let replica_id = project.read(cx).replica_id();
        let buffer_ids = state
            .excerpts
            .iter()
            .map(|excerpt| excerpt.buffer_id)
            .collect::<HashSet<_>>();
        let buffers = project.update(cx, |project, cx| {
            buffer_ids
                .iter()
                .map(|id| BufferId::new(*id).map(|id| project.open_buffer_by_id(id, cx)))
                .collect::<Result<Vec<_>>>()
        });

        let pane = pane.downgrade();
        Some(cx.spawn(|mut cx| async move {
            let mut buffers = futures::future::try_join_all(buffers?)
                .await
                .debug_assert_ok("leaders don't share views for unshared buffers")?;

            let editor = pane.update(&mut cx, |pane, cx| {
                let mut editors = pane.items_of_type::<Self>();
                editors.find(|editor| {
                    let ids_match = editor.remote_id(&client, cx) == Some(remote_id);
                    let singleton_buffer_matches = state.singleton
                        && buffers.first()
                            == editor.read(cx).buffer.read(cx).as_singleton().as_ref();
                    ids_match || singleton_buffer_matches
                })
            })?;

            let editor = if let Some(editor) = editor {
                editor
            } else {
                pane.update(&mut cx, |_, cx| {
                    let multibuffer = cx.new_model(|cx| {
                        let mut multibuffer;
                        if state.singleton && buffers.len() == 1 {
                            multibuffer = MultiBuffer::singleton(buffers.pop().unwrap(), cx)
                        } else {
                            multibuffer =
                                MultiBuffer::new(replica_id, project.read(cx).capability());
                            let mut excerpts = state.excerpts.into_iter().peekable();
                            while let Some(excerpt) = excerpts.peek() {
                                let Ok(buffer_id) = BufferId::new(excerpt.buffer_id) else {
                                    continue;
                                };
                                let buffer_excerpts = iter::from_fn(|| {
                                    let excerpt = excerpts.peek()?;
                                    (excerpt.buffer_id == u64::from(buffer_id))
                                        .then(|| excerpts.next().unwrap())
                                });
                                let buffer =
                                    buffers.iter().find(|b| b.read(cx).remote_id() == buffer_id);
                                if let Some(buffer) = buffer {
                                    multibuffer.push_excerpts(
                                        buffer.clone(),
                                        buffer_excerpts.filter_map(deserialize_excerpt_range),
                                        cx,
                                    );
                                }
                            }
                        };

                        if let Some(title) = &state.title {
                            multibuffer = multibuffer.with_title(title.clone())
                        }

                        multibuffer
                    });

                    cx.new_view(|cx| {
                        let mut editor =
                            Editor::for_multibuffer(multibuffer, Some(project.clone()), true, cx);
                        editor.remote_id = Some(remote_id);
                        editor
                    })
                })?
            };

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
                &mut cx,
            )
            .await?;

            Ok(editor)
        }))
    }

    fn set_leader_peer_id(&mut self, leader_peer_id: Option<PeerId>, cx: &mut ViewContext<Self>) {
        self.leader_peer_id = leader_peer_id;
        if self.leader_peer_id.is_some() {
            self.buffer.update(cx, |buffer, cx| {
                buffer.remove_active_selections(cx);
            });
        } else if self.focus_handle.is_focused(cx) {
            self.buffer.update(cx, |buffer, cx| {
                buffer.set_active_selections(
                    &self.selections.disjoint_anchors(),
                    self.selections.line_mode,
                    self.cursor_shape,
                    cx,
                );
            });
        }
        cx.notify();
    }

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        let buffer = self.buffer.read(cx);
        if buffer
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).file())
            .map_or(false, |file| file.is_private())
        {
            return None;
        }

        let scroll_anchor = self.scroll_manager.anchor();
        let excerpts = buffer
            .read(cx)
            .excerpts()
            .map(|(id, buffer, range)| proto::Excerpt {
                id: id.to_proto(),
                buffer_id: buffer.remote_id().into(),
                context_start: Some(serialize_text_anchor(&range.context.start)),
                context_end: Some(serialize_text_anchor(&range.context.end)),
                primary_start: range
                    .primary
                    .as_ref()
                    .map(|range| serialize_text_anchor(&range.start)),
                primary_end: range
                    .primary
                    .as_ref()
                    .map(|range| serialize_text_anchor(&range.end)),
            })
            .collect();

        Some(proto::view::Variant::Editor(proto::view::Editor {
            singleton: buffer.is_singleton(),
            title: (!buffer.is_singleton()).then(|| buffer.title(cx).into()),
            excerpts,
            scroll_top_anchor: Some(serialize_anchor(&scroll_anchor.anchor)),
            scroll_x: scroll_anchor.offset.x,
            scroll_y: scroll_anchor.offset.y,
            selections: self
                .selections
                .disjoint_anchors()
                .iter()
                .map(serialize_selection)
                .collect(),
            pending_selection: self
                .selections
                .pending_anchor()
                .as_ref()
                .map(serialize_selection),
        }))
    }

    fn to_follow_event(event: &EditorEvent) -> Option<workspace::item::FollowEvent> {
        match event {
            EditorEvent::Edited => Some(FollowEvent::Unfollow),
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
        cx: &WindowContext,
    ) -> bool {
        let update =
            update.get_or_insert_with(|| proto::update_view::Variant::Editor(Default::default()));

        match update {
            proto::update_view::Variant::Editor(update) => match event {
                EditorEvent::ExcerptsAdded {
                    buffer,
                    predecessor,
                    excerpts,
                } => {
                    let buffer_id = buffer.read(cx).remote_id();
                    let mut excerpts = excerpts.iter();
                    if let Some((id, range)) = excerpts.next() {
                        update.inserted_excerpts.push(proto::ExcerptInsertion {
                            previous_excerpt_id: Some(predecessor.to_proto()),
                            excerpt: serialize_excerpt(buffer_id, id, range),
                        });
                        update.inserted_excerpts.extend(excerpts.map(|(id, range)| {
                            proto::ExcerptInsertion {
                                previous_excerpt_id: None,
                                excerpt: serialize_excerpt(buffer_id, id, range),
                            }
                        }))
                    }
                    true
                }
                EditorEvent::ExcerptsRemoved { ids } => {
                    update
                        .deleted_excerpts
                        .extend(ids.iter().map(ExcerptId::to_proto));
                    true
                }
                EditorEvent::ScrollPositionChanged { autoscroll, .. } if !autoscroll => {
                    let scroll_anchor = self.scroll_manager.anchor();
                    update.scroll_top_anchor = Some(serialize_anchor(&scroll_anchor.anchor));
                    update.scroll_x = scroll_anchor.offset.x;
                    update.scroll_y = scroll_anchor.offset.y;
                    true
                }
                EditorEvent::SelectionsChanged { .. } => {
                    update.selections = self
                        .selections
                        .disjoint_anchors()
                        .iter()
                        .map(serialize_selection)
                        .collect();
                    update.pending_selection = self
                        .selections
                        .pending_anchor()
                        .as_ref()
                        .map(serialize_selection);
                    true
                }
                _ => false,
            },
        }
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let update_view::Variant::Editor(message) = message;
        let project = project.clone();
        cx.spawn(|this, mut cx| async move {
            update_editor_from_message(this, project, message, &mut cx).await
        })
    }

    fn is_project_item(&self, _cx: &WindowContext) -> bool {
        true
    }
}

async fn update_editor_from_message(
    this: WeakView<Editor>,
    project: Model<Project>,
    message: proto::update_view::Editor,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    // Open all of the buffers of which excerpts were added to the editor.
    let inserted_excerpt_buffer_ids = message
        .inserted_excerpts
        .iter()
        .filter_map(|insertion| Some(insertion.excerpt.as_ref()?.buffer_id))
        .collect::<HashSet<_>>();
    let inserted_excerpt_buffers = project.update(cx, |project, cx| {
        inserted_excerpt_buffer_ids
            .into_iter()
            .map(|id| BufferId::new(id).map(|id| project.open_buffer_by_id(id, cx)))
            .collect::<Result<Vec<_>>>()
    })??;
    let _inserted_excerpt_buffers = try_join_all(inserted_excerpt_buffers).await?;

    // Update the editor's excerpts.
    this.update(cx, |editor, cx| {
        editor.buffer.update(cx, |multibuffer, cx| {
            let mut removed_excerpt_ids = message
                .deleted_excerpts
                .into_iter()
                .map(ExcerptId::from_proto)
                .collect::<Vec<_>>();
            removed_excerpt_ids.sort_by({
                let multibuffer = multibuffer.read(cx);
                move |a, b| a.cmp(&b, &multibuffer)
            });

            let mut insertions = message.inserted_excerpts.into_iter().peekable();
            while let Some(insertion) = insertions.next() {
                let Some(excerpt) = insertion.excerpt else {
                    continue;
                };
                let Some(previous_excerpt_id) = insertion.previous_excerpt_id else {
                    continue;
                };
                let buffer_id = BufferId::new(excerpt.buffer_id)?;
                let Some(buffer) = project.read(cx).buffer_for_id(buffer_id) else {
                    continue;
                };

                let adjacent_excerpts = iter::from_fn(|| {
                    let insertion = insertions.peek()?;
                    if insertion.previous_excerpt_id.is_none()
                        && insertion.excerpt.as_ref()?.buffer_id == u64::from(buffer_id)
                    {
                        insertions.next()?.excerpt
                    } else {
                        None
                    }
                });

                multibuffer.insert_excerpts_with_ids_after(
                    ExcerptId::from_proto(previous_excerpt_id),
                    buffer,
                    [excerpt]
                        .into_iter()
                        .chain(adjacent_excerpts)
                        .filter_map(|excerpt| {
                            Some((
                                ExcerptId::from_proto(excerpt.id),
                                deserialize_excerpt_range(excerpt)?,
                            ))
                        }),
                    cx,
                );
            }

            multibuffer.remove_excerpts(removed_excerpt_ids, cx);
            Result::<(), anyhow::Error>::Ok(())
        })
    })??;

    // Deserialize the editor state.
    let (selections, pending_selection, scroll_top_anchor) = this.update(cx, |editor, cx| {
        let buffer = editor.buffer.read(cx).read(cx);
        let selections = message
            .selections
            .into_iter()
            .filter_map(|selection| deserialize_selection(&buffer, selection))
            .collect::<Vec<_>>();
        let pending_selection = message
            .pending_selection
            .and_then(|selection| deserialize_selection(&buffer, selection));
        let scroll_top_anchor = message
            .scroll_top_anchor
            .and_then(|anchor| deserialize_anchor(&buffer, anchor));
        anyhow::Ok((selections, pending_selection, scroll_top_anchor))
    })??;

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
    this.update(cx, |editor, cx| {
        if !selections.is_empty() || pending_selection.is_some() {
            editor.set_selections_from_remote(selections, pending_selection, cx);
            editor.request_autoscroll_remotely(Autoscroll::newest(), cx);
        } else if let Some(scroll_top_anchor) = scroll_top_anchor {
            editor.set_scroll_anchor_remote(
                ScrollAnchor {
                    anchor: scroll_top_anchor,
                    offset: point(message.scroll_x, message.scroll_y),
                },
                cx,
            );
        }
    })?;
    Ok(())
}

fn serialize_excerpt(
    buffer_id: BufferId,
    id: &ExcerptId,
    range: &ExcerptRange<language::Anchor>,
) -> Option<proto::Excerpt> {
    Some(proto::Excerpt {
        id: id.to_proto(),
        buffer_id: buffer_id.into(),
        context_start: Some(serialize_text_anchor(&range.context.start)),
        context_end: Some(serialize_text_anchor(&range.context.end)),
        primary_start: range
            .primary
            .as_ref()
            .map(|r| serialize_text_anchor(&r.start)),
        primary_end: range
            .primary
            .as_ref()
            .map(|r| serialize_text_anchor(&r.end)),
    })
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
    proto::EditorAnchor {
        excerpt_id: anchor.excerpt_id.to_proto(),
        anchor: Some(serialize_text_anchor(&anchor.text_anchor)),
    }
}

fn deserialize_excerpt_range(excerpt: proto::Excerpt) -> Option<ExcerptRange<language::Anchor>> {
    let context = {
        let start = language::proto::deserialize_anchor(excerpt.context_start?)?;
        let end = language::proto::deserialize_anchor(excerpt.context_end?)?;
        start..end
    };
    let primary = excerpt
        .primary_start
        .zip(excerpt.primary_end)
        .and_then(|(start, end)| {
            let start = language::proto::deserialize_anchor(start)?;
            let end = language::proto::deserialize_anchor(end)?;
            Some(start..end)
        });
    Some(ExcerptRange { context, primary })
}

fn deserialize_selection(
    buffer: &MultiBufferSnapshot,
    selection: proto::Selection,
) -> Option<Selection<Anchor>> {
    Some(Selection {
        id: selection.id as usize,
        start: deserialize_anchor(buffer, selection.start?)?,
        end: deserialize_anchor(buffer, selection.end?)?,
        reversed: selection.reversed,
        goal: SelectionGoal::None,
    })
}

fn deserialize_anchor(buffer: &MultiBufferSnapshot, anchor: proto::EditorAnchor) -> Option<Anchor> {
    let excerpt_id = ExcerptId::from_proto(anchor.excerpt_id);
    Some(Anchor {
        excerpt_id,
        text_anchor: language::proto::deserialize_anchor(anchor.anchor?)?,
        buffer_id: buffer.buffer_id_for_excerpt(excerpt_id),
    })
}

impl Item for Editor {
    type Event = EditorEvent;

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        if let Ok(data) = data.downcast::<NavigationData>() {
            let newest_selection = self.selections.newest::<Point>(cx);
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
                let nav_history = self.nav_history.take();
                self.set_scroll_anchor(scroll_anchor, cx);
                self.change_selections(Some(Autoscroll::fit()), cx, |s| {
                    s.select_ranges([offset..offset])
                });
                self.nav_history = nav_history;
                true
            }
        } else {
            false
        }
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        let file_path = self
            .buffer()
            .read(cx)
            .as_singleton()?
            .read(cx)
            .file()
            .and_then(|f| f.as_local())?
            .abs_path(cx);

        let file_path = file_path.compact().to_string_lossy().to_string();

        Some(file_path.into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn tab_description(&self, detail: usize, cx: &AppContext) -> Option<SharedString> {
        let path = path_for_buffer(&self.buffer, detail, true, cx)?;
        Some(path.to_string_lossy().to_string().into())
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let label_color = if ItemSettings::get_global(cx).git_status {
            self.buffer()
                .read(cx)
                .as_singleton()
                .and_then(|buffer| buffer.read(cx).project_path(cx))
                .and_then(|path| self.project.as_ref()?.read(cx).entry_for_path(&path, cx))
                .map(|entry| {
                    entry_git_aware_label_color(entry.git_status, entry.is_ignored, params.selected)
                })
                .unwrap_or_else(|| entry_label_color(params.selected))
        } else {
            entry_label_color(params.selected)
        };

        let description = params.detail.and_then(|detail| {
            let path = path_for_buffer(&self.buffer, detail, false, cx)?;
            let description = path.to_string_lossy();
            let description = description.trim();

            if description.is_empty() {
                return None;
            }

            Some(util::truncate_and_trailoff(&description, MAX_TAB_TITLE_LEN))
        });

        h_flex()
            .gap_2()
            .child(
                Label::new(self.title(cx).to_string())
                    .color(label_color)
                    .italic(params.preview),
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
        cx: &AppContext,
        f: &mut dyn FnMut(EntityId, &dyn project::Item),
    ) {
        self.buffer
            .read(cx)
            .for_each_buffer(|buffer| f(buffer.entity_id(), buffer.read(cx)));
    }

    fn is_singleton(&self, cx: &AppContext) -> bool {
        self.buffer.read(cx).is_singleton()
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Editor>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| self.clone(cx)))
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        let selection = self.selections.newest_anchor();
        self.push_to_nav_history(selection.head(), None, cx);
    }

    fn workspace_deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.hide_hovered_link(cx);
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).is_dirty()
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.buffer().read(cx).read(cx).has_conflict()
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        let buffer = &self.buffer().read(cx);
        if let Some(buffer) = buffer.as_singleton() {
            buffer.read(cx).project_path(cx).is_some()
        } else {
            true
        }
    }

    fn save(
        &mut self,
        format: bool,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.report_editor_event("save", None, cx);
        let buffers = self.buffer().clone().read(cx).all_buffers();
        cx.spawn(|this, mut cx| async move {
            if format {
                this.update(&mut cx, |editor, cx| {
                    editor.perform_format(project.clone(), FormatTrigger::Save, cx)
                })?
                .await?;
            }

            if buffers.len() == 1 {
                // Apply full save routine for singleton buffers, to allow to `touch` the file via the editor.
                project
                    .update(&mut cx, |project, cx| project.save_buffers(buffers, cx))?
                    .await?;
            } else {
                // For multi-buffers, only format and save the buffers with changes.
                // For clean buffers, we simulate saving by calling `Buffer::did_save`,
                // so that language servers or other downstream listeners of save events get notified.
                let (dirty_buffers, clean_buffers) = buffers.into_iter().partition(|buffer| {
                    buffer
                        .update(&mut cx, |buffer, _| {
                            buffer.is_dirty() || buffer.has_conflict()
                        })
                        .unwrap_or(false)
                });

                project
                    .update(&mut cx, |project, cx| {
                        project.save_buffers(dirty_buffers, cx)
                    })?
                    .await?;
                for buffer in clean_buffers {
                    buffer
                        .update(&mut cx, |buffer, cx| {
                            let version = buffer.saved_version().clone();
                            let mtime = buffer.saved_mtime();
                            buffer.did_save(version, mtime, cx);
                        })
                        .ok();
                }
            }

            Ok(())
        })
    }

    fn save_as(
        &mut self,
        project: Model<Project>,
        path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("cannot call save_as on an excerpt list");

        let file_extension = path
            .path
            .extension()
            .map(|a| a.to_string_lossy().to_string());
        self.report_editor_event("save", file_extension, cx);

        project.update(cx, |project, cx| project.save_buffer_as(buffer, path, cx))
    }

    fn reload(&mut self, project: Model<Project>, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = self.buffer.read(cx).all_buffers();
        let reload_buffers =
            project.update(cx, |project, cx| project.reload_buffers(buffers, true, cx));
        cx.spawn(|this, mut cx| async move {
            let transaction = reload_buffers.log_err().await;
            this.update(&mut cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::fit(), cx)
            })?;
            buffer
                .update(&mut cx, |buffer, cx| {
                    if let Some(transaction) = transaction {
                        if !buffer.is_singleton() {
                            buffer.push_transaction(&transaction.0, cx);
                        }
                    }
                })
                .ok();
            Ok(())
        })
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn pixel_position_of_cursor(&self, _: &AppContext) -> Option<gpui::Point<Pixels>> {
        self.pixel_position_of_newest_cursor
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        if self.show_breadcrumbs {
            ToolbarItemLocation::PrimaryLeft
        } else {
            ToolbarItemLocation::Hidden
        }
    }

    fn breadcrumbs(&self, variant: &Theme, cx: &AppContext) -> Option<Vec<BreadcrumbText>> {
        let cursor = self.selections.newest_anchor().head();
        let multibuffer = &self.buffer().read(cx);
        let (buffer_id, symbols) =
            multibuffer.symbols_containing(cursor, Some(&variant.syntax()), cx)?;
        let buffer = multibuffer.buffer(buffer_id)?;

        let buffer = buffer.read(cx);
        let filename = buffer
            .snapshot()
            .resolve_file_path(
                cx,
                self.project
                    .as_ref()
                    .map(|project| project.read(cx).visible_worktrees(cx).count() > 1)
                    .unwrap_or_default(),
            )
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".to_string());

        let settings = ThemeSettings::get_global(cx);

        let mut breadcrumbs = vec![BreadcrumbText {
            text: filename,
            highlights: None,
            font: Some(settings.buffer_font.clone()),
        }];

        breadcrumbs.extend(symbols.into_iter().map(|symbol| BreadcrumbText {
            text: symbol.text,
            highlights: Some(symbol.highlight_ranges),
            font: Some(settings.buffer_font.clone()),
        }));
        Some(breadcrumbs)
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        self.workspace = Some((workspace.weak_handle(), workspace.database_id()));
        let Some(workspace_id) = workspace.database_id() else {
            return;
        };

        let item_id = cx.view().item_id().as_u64() as ItemId;

        fn serialize(
            buffer: Model<Buffer>,
            workspace_id: WorkspaceId,
            item_id: ItemId,
            cx: &mut AppContext,
        ) {
            if let Some(file) = buffer.read(cx).file().and_then(|file| file.as_local()) {
                let path = file.abs_path(cx);

                cx.background_executor()
                    .spawn(async move {
                        DB.save_path(item_id, workspace_id, path.clone())
                            .await
                            .log_err()
                    })
                    .detach();
            }
        }

        if let Some(buffer) = self.buffer().read(cx).as_singleton() {
            serialize(buffer.clone(), workspace_id, item_id, cx);

            cx.subscribe(&buffer, |this, buffer, event, cx| {
                if let Some((_, Some(workspace_id))) = this.workspace.as_ref() {
                    if let language::Event::FileHandleChanged = event {
                        serialize(
                            buffer,
                            *workspace_id,
                            cx.view().item_id().as_u64() as ItemId,
                            cx,
                        );
                    }
                }
            })
            .detach();
        }
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("Editor")
    }

    fn to_item_events(event: &EditorEvent, mut f: impl FnMut(ItemEvent)) {
        match event {
            EditorEvent::Closed => f(ItemEvent::CloseItem),

            EditorEvent::Saved | EditorEvent::TitleChanged => {
                f(ItemEvent::UpdateTab);
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::Reparsed => {
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::SelectionsChanged { local } if *local => {
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::DirtyChanged => {
                f(ItemEvent::UpdateTab);
            }

            EditorEvent::BufferEdited => {
                f(ItemEvent::Edit);
                f(ItemEvent::UpdateBreadcrumbs);
            }

            EditorEvent::ExcerptsAdded { .. } | EditorEvent::ExcerptsRemoved { .. } => {
                f(ItemEvent::Edit);
            }

            _ => {}
        }
    }

    fn deserialize(
        project: Model<Project>,
        _workspace: WeakView<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<Result<View<Self>>> {
        let project_item: Result<_> = project.update(cx, |project, cx| {
            // Look up the path with this key associated, create a self with that path
            let path = DB
                .get_path(item_id, workspace_id)?
                .context("No path stored for this editor")?;

            let (worktree, path) = project
                .find_local_worktree(&path, cx)
                .with_context(|| format!("No worktree for path: {path:?}"))?;
            let project_path = ProjectPath {
                worktree_id: worktree.read(cx).id(),
                path: path.into(),
            };

            Ok(project.open_path(project_path, cx))
        });

        project_item
            .map(|project_item| {
                cx.spawn(|pane, mut cx| async move {
                    let (_, project_item) = project_item.await?;
                    let buffer = project_item
                        .downcast::<Buffer>()
                        .map_err(|_| anyhow!("Project item at stored path was not a buffer"))?;
                    pane.update(&mut cx, |_, cx| {
                        cx.new_view(|cx| {
                            let mut editor = Editor::for_buffer(buffer, Some(project), cx);

                            editor.read_scroll_position_from_db(item_id, workspace_id, cx);
                            editor
                        })
                    })
                })
            })
            .unwrap_or_else(|error| Task::ready(Err(error)))
    }
}

impl ProjectItem for Editor {
    type Item = Buffer;

    fn for_project_item(
        project: Model<Project>,
        buffer: Model<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        Self::for_buffer(buffer, Some(project), cx)
    }
}

impl EventEmitter<SearchEvent> for Editor {}

pub(crate) enum BufferSearchHighlights {}
impl SearchableItem for Editor {
    type Match = Range<Anchor>;

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_background_highlights::<BufferSearchHighlights>(cx);
    }

    fn update_matches(&mut self, matches: &[Range<Anchor>], cx: &mut ViewContext<Self>) {
        self.highlight_background::<BufferSearchHighlights>(
            matches,
            |theme| theme.search_match_background,
            cx,
        );
    }

    fn has_filtered_search_ranges(&mut self) -> bool {
        self.has_background_highlights::<SearchWithinRange>()
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        let setting = EditorSettings::get_global(cx).seed_search_query_from_cursor;
        let snapshot = &self.snapshot(cx).buffer_snapshot;
        let selection = self.selections.newest::<usize>(cx);

        match setting {
            SeedQuerySetting::Never => String::new(),
            SeedQuerySetting::Selection | SeedQuerySetting::Always if !selection.is_empty() => {
                snapshot
                    .text_for_range(selection.start..selection.end)
                    .collect()
            }
            SeedQuerySetting::Selection => String::new(),
            SeedQuerySetting::Always => {
                let (range, kind) = snapshot.surrounding_word(selection.start);
                if kind == Some(CharKind::Word) {
                    let text: String = snapshot.text_for_range(range).collect();
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
        cx: &mut ViewContext<Self>,
    ) {
        self.unfold_ranges([matches[index].clone()], false, true, cx);
        let range = self.range_for_match(&matches[index]);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges([range]);
        })
    }

    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.unfold_ranges(matches.to_vec(), false, false, cx);
        let mut ranges = Vec::new();
        for m in matches {
            ranges.push(self.range_for_match(&m))
        }
        self.change_selections(None, cx, |s| s.select_ranges(ranges));
    }
    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &SearchQuery,
        cx: &mut ViewContext<Self>,
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
            self.transact(cx, |this, cx| {
                this.edit([(identifier.clone(), Arc::from(&*replacement))], cx);
            });
        }
    }
    fn match_index_for_direction(
        &mut self,
        matches: &[Range<Anchor>],
        current_index: usize,
        direction: Direction,
        count: usize,
        cx: &mut ViewContext<Self>,
    ) -> usize {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let current_index_position = if self.selections.disjoint_anchors().len() == 1 {
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
                    count = count - 1
                }

                (current_index + count) % matches.len()
            }
            Direction::Prev => {
                if matches[current_index]
                    .end
                    .cmp(&current_index_position, &buffer)
                    .is_lt()
                {
                    count = count - 1;
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
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Range<Anchor>>> {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let search_within_ranges = self
            .background_highlights
            .get(&TypeId::of::<SearchWithinRange>())
            .map(|(_color, ranges)| {
                ranges
                    .iter()
                    .map(|range| range.to_offset(&buffer))
                    .collect::<Vec<_>>()
            });
        cx.background_executor().spawn(async move {
            let mut ranges = Vec::new();
            if let Some((_, _, excerpt_buffer)) = buffer.as_singleton() {
                if let Some(search_within_ranges) = search_within_ranges {
                    for range in search_within_ranges {
                        let offset = range.start;
                        ranges.extend(
                            query
                                .search(excerpt_buffer, Some(range))
                                .await
                                .into_iter()
                                .map(|range| {
                                    buffer.anchor_after(range.start + offset)
                                        ..buffer.anchor_before(range.end + offset)
                                }),
                        );
                    }
                } else {
                    ranges.extend(query.search(excerpt_buffer, None).await.into_iter().map(
                        |range| buffer.anchor_after(range.start)..buffer.anchor_before(range.end),
                    ));
                }
            } else {
                for excerpt in buffer.excerpt_boundaries_in_range(0..buffer.len()) {
                    if let Some(next_excerpt) = excerpt.next {
                        let excerpt_range =
                            next_excerpt.range.context.to_offset(&next_excerpt.buffer);
                        ranges.extend(
                            query
                                .search(&next_excerpt.buffer, Some(excerpt_range.clone()))
                                .await
                                .into_iter()
                                .map(|range| {
                                    let start = next_excerpt
                                        .buffer
                                        .anchor_after(excerpt_range.start + range.start);
                                    let end = next_excerpt
                                        .buffer
                                        .anchor_before(excerpt_range.start + range.end);
                                    buffer.anchor_in_excerpt(next_excerpt.id, start).unwrap()
                                        ..buffer.anchor_in_excerpt(next_excerpt.id, end).unwrap()
                                }),
                        );
                    }
                }
            }
            ranges
        })
    }

    fn active_match_index(
        &mut self,
        matches: &[Range<Anchor>],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        active_match_index(
            matches,
            &self.selections.newest_anchor().head(),
            &self.buffer().read(cx).snapshot(cx),
        )
    }

    fn search_bar_visibility_changed(&mut self, _visible: bool, _cx: &mut ViewContext<Self>) {
        self.expect_bounds_change = self.last_bounds;
    }
}

pub fn active_match_index(
    ranges: &[Range<Anchor>],
    cursor: &Anchor,
    buffer: &MultiBufferSnapshot,
) -> Option<usize> {
    if ranges.is_empty() {
        None
    } else {
        match ranges.binary_search_by(|probe| {
            if probe.end.cmp(cursor, buffer).is_lt() {
                Ordering::Less
            } else if probe.start.cmp(cursor, buffer).is_gt() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) | Err(i) => Some(cmp::min(i, ranges.len() - 1)),
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

pub fn entry_git_aware_label_color(
    git_status: Option<GitFileStatus>,
    ignored: bool,
    selected: bool,
) -> Color {
    if ignored {
        Color::Ignored
    } else {
        match git_status {
            Some(GitFileStatus::Added) => Color::Created,
            Some(GitFileStatus::Modified) => Color::Modified,
            Some(GitFileStatus::Conflict) => Color::Conflict,
            None => entry_label_color(selected),
        }
    }
}

fn path_for_buffer<'a>(
    buffer: &Model<MultiBuffer>,
    height: usize,
    include_filename: bool,
    cx: &'a AppContext,
) -> Option<Cow<'a, Path>> {
    let file = buffer.read(cx).as_singleton()?.read(cx).file()?;
    path_for_file(file.as_ref(), height, include_filename, cx)
}

fn path_for_file<'a>(
    file: &'a dyn language::File,
    mut height: usize,
    include_filename: bool,
    cx: &'a AppContext,
) -> Option<Cow<'a, Path>> {
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

    // Here we could have just always used `full_path`, but that is very
    // allocation-heavy and so we try to use a `Cow<Path>` if we haven't
    // traversed all the way up to the worktree's root.
    if height > 0 {
        let full_path = file.full_path(cx);
        if include_filename {
            Some(full_path.into())
        } else {
            Some(full_path.parent()?.to_path_buf().into())
        }
    } else {
        let mut path = file.path().strip_prefix(prefix).ok()?;
        if !include_filename {
            path = path.parent()?;
        }
        Some(path.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::AppContext;
    use language::TestFile;
    use std::path::Path;

    #[gpui::test]
    fn test_path_for_file(cx: &mut AppContext) {
        let file = TestFile {
            path: Path::new("").into(),
            root_name: String::new(),
        };
        assert_eq!(path_for_file(&file, 0, false, cx), None);
    }
}
