use crate::{
    display_map::ToDisplayPoint, link_go_to_definition::hide_link_definition,
    movement::surrounding_word, persistence::DB, scroll::ScrollAnchor, Anchor, Autoscroll, Editor,
    Event, ExcerptId, ExcerptRange, MultiBuffer, MultiBufferSnapshot, NavigationData, ToPoint as _,
};
use anyhow::{anyhow, Context, Result};
use collections::HashSet;
use futures::future::try_join_all;
use gpui::{
    elements::*, geometry::vector::vec2f, AppContext, Entity, ModelHandle, MutableAppContext,
    RenderContext, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::{
    proto::serialize_anchor as serialize_text_anchor, Bias, Buffer, OffsetRangeExt, Point,
    SelectionGoal,
};
use project::{FormatTrigger, Item as _, Project, ProjectPath};
use rpc::proto::{self, update_view};
use settings::Settings;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    cmp::{self, Ordering},
    fmt::Write,
    iter,
    ops::Range,
    path::{Path, PathBuf},
};
use text::Selection;
use util::{ResultExt, TryFutureExt};
use workspace::item::FollowableItemHandle;
use workspace::{
    item::{FollowableItem, Item, ItemEvent, ItemHandle, ProjectItem},
    searchable::{Direction, SearchEvent, SearchableItem, SearchableItemHandle},
    ItemId, ItemNavHistory, Pane, StatusItemView, ToolbarItemLocation, ViewId, Workspace,
    WorkspaceId,
};

pub const MAX_TAB_TITLE_LEN: usize = 24;

impl FollowableItem for Editor {
    fn remote_id(&self) -> Option<ViewId> {
        self.remote_id
    }

    fn from_state_proto(
        pane: ViewHandle<workspace::Pane>,
        project: ModelHandle<Project>,
        remote_id: ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>> {
        let Some(proto::view::Variant::Editor(_)) = state else { return None };
        let Some(proto::view::Variant::Editor(state)) = state.take() else { unreachable!() };

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
                .map(|id| project.open_buffer_by_id(*id, cx))
                .collect::<Vec<_>>()
        });

        Some(cx.spawn(|mut cx| async move {
            let mut buffers = futures::future::try_join_all(buffers).await?;
            let editor = pane.read_with(&cx, |pane, cx| {
                let mut editors = pane.items_of_type::<Self>();
                editors.find(|editor| {
                    editor.remote_id(&client, cx) == Some(remote_id)
                        || state.singleton
                            && buffers.len() == 1
                            && editor.read(cx).buffer.read(cx).as_singleton().as_ref()
                                == Some(&buffers[0])
                })
            });

            let editor = editor.unwrap_or_else(|| {
                pane.update(&mut cx, |_, cx| {
                    let multibuffer = cx.add_model(|cx| {
                        let mut multibuffer;
                        if state.singleton && buffers.len() == 1 {
                            multibuffer = MultiBuffer::singleton(buffers.pop().unwrap(), cx)
                        } else {
                            multibuffer = MultiBuffer::new(replica_id);
                            let mut excerpts = state.excerpts.into_iter().peekable();
                            while let Some(excerpt) = excerpts.peek() {
                                let buffer_id = excerpt.buffer_id;
                                let buffer_excerpts = iter::from_fn(|| {
                                    let excerpt = excerpts.peek()?;
                                    (excerpt.buffer_id == buffer_id)
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

                    cx.add_view(|cx| Editor::for_multibuffer(multibuffer, Some(project), cx))
                })
            });

            editor.update(&mut cx, |editor, cx| {
                editor.remote_id = Some(remote_id);
                let buffer = editor.buffer.read(cx).read(cx);
                let selections = state
                    .selections
                    .into_iter()
                    .map(|selection| {
                        deserialize_selection(&buffer, selection)
                            .ok_or_else(|| anyhow!("invalid selection"))
                    })
                    .collect::<Result<Vec<_>>>()?;
                let pending_selection = state
                    .pending_selection
                    .map(|selection| deserialize_selection(&buffer, selection))
                    .flatten();
                let scroll_top_anchor = state
                    .scroll_top_anchor
                    .and_then(|anchor| deserialize_anchor(&buffer, anchor));
                drop(buffer);

                if !selections.is_empty() || pending_selection.is_some() {
                    editor.set_selections_from_remote(selections, pending_selection, cx);
                }

                if let Some(scroll_top_anchor) = scroll_top_anchor {
                    editor.set_scroll_anchor_remote(
                        ScrollAnchor {
                            top_anchor: scroll_top_anchor,
                            offset: vec2f(state.scroll_x, state.scroll_y),
                        },
                        cx,
                    );
                }

                anyhow::Ok(())
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
                    buffer.set_active_selections(
                        &self.selections.disjoint_anchors(),
                        self.selections.line_mode,
                        self.cursor_shape,
                        cx,
                    );
                }
            });
        }
        cx.notify();
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        let buffer = self.buffer.read(cx);
        let scroll_anchor = self.scroll_manager.anchor();
        let excerpts = buffer
            .read(cx)
            .excerpts()
            .map(|(id, buffer, range)| proto::Excerpt {
                id: id.to_proto(),
                buffer_id: buffer.remote_id(),
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
            scroll_top_anchor: Some(serialize_anchor(&scroll_anchor.top_anchor)),
            scroll_x: scroll_anchor.offset.x(),
            scroll_y: scroll_anchor.offset.y(),
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

    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool {
        let update =
            update.get_or_insert_with(|| proto::update_view::Variant::Editor(Default::default()));

        match update {
            proto::update_view::Variant::Editor(update) => match event {
                Event::ExcerptsAdded {
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
                Event::ExcerptsRemoved { ids } => {
                    update
                        .deleted_excerpts
                        .extend(ids.iter().map(ExcerptId::to_proto));
                    true
                }
                Event::ScrollPositionChanged { .. } => {
                    let scroll_anchor = self.scroll_manager.anchor();
                    update.scroll_top_anchor = Some(serialize_anchor(&scroll_anchor.top_anchor));
                    update.scroll_x = scroll_anchor.offset.x();
                    update.scroll_y = scroll_anchor.offset.y();
                    true
                }
                Event::SelectionsChanged { .. } => {
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
        project: &ModelHandle<Project>,
        message: update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let update_view::Variant::Editor(message) = message;
        let multibuffer = self.buffer.read(cx);
        let multibuffer = multibuffer.read(cx);

        let buffer_ids = message
            .inserted_excerpts
            .iter()
            .filter_map(|insertion| Some(insertion.excerpt.as_ref()?.buffer_id))
            .collect::<HashSet<_>>();

        let mut removals = message
            .deleted_excerpts
            .into_iter()
            .map(ExcerptId::from_proto)
            .collect::<Vec<_>>();
        removals.sort_by(|a, b| a.cmp(&b, &multibuffer));

        let selections = message
            .selections
            .into_iter()
            .filter_map(|selection| deserialize_selection(&multibuffer, selection))
            .collect::<Vec<_>>();
        let pending_selection = message
            .pending_selection
            .and_then(|selection| deserialize_selection(&multibuffer, selection));

        let scroll_top_anchor = message
            .scroll_top_anchor
            .and_then(|anchor| deserialize_anchor(&multibuffer, anchor));
        drop(multibuffer);

        let buffers = project.update(cx, |project, cx| {
            buffer_ids
                .into_iter()
                .map(|id| project.open_buffer_by_id(id, cx))
                .collect::<Vec<_>>()
        });

        let project = project.clone();
        cx.spawn(|this, mut cx| async move {
            let _buffers = try_join_all(buffers).await?;
            this.update(&mut cx, |this, cx| {
                this.buffer.update(cx, |multibuffer, cx| {
                    let mut insertions = message.inserted_excerpts.into_iter().peekable();
                    while let Some(insertion) = insertions.next() {
                        let Some(excerpt) = insertion.excerpt else { continue };
                        let Some(previous_excerpt_id) = insertion.previous_excerpt_id else { continue };
                        let buffer_id = excerpt.buffer_id;
                        let Some(buffer) = project.read(cx).buffer_for_id(buffer_id, cx) else { continue };

                        let adjacent_excerpts = iter::from_fn(|| {
                            let insertion = insertions.peek()?;
                            if insertion.previous_excerpt_id.is_none()
                                && insertion.excerpt.as_ref()?.buffer_id == buffer_id
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

                    multibuffer.remove_excerpts(removals, cx);
                });

                if !selections.is_empty() || pending_selection.is_some() {
                    this.set_selections_from_remote(selections, pending_selection, cx);
                    this.request_autoscroll_remotely(Autoscroll::newest(), cx);
                } else if let Some(anchor) = scroll_top_anchor {
                    this.set_scroll_anchor_remote(ScrollAnchor {
                        top_anchor: anchor,
                        offset: vec2f(message.scroll_x, message.scroll_y)
                    }, cx);
                }
            });
            Ok(())
        })
    }

    fn should_unfollow_on_event(event: &Self::Event, _: &AppContext) -> bool {
        match event {
            Event::Edited => true,
            Event::SelectionsChanged { local } => *local,
            Event::ScrollPositionChanged { local } => *local,
            _ => false,
        }
    }
}

fn serialize_excerpt(
    buffer_id: u64,
    id: &ExcerptId,
    range: &ExcerptRange<language::Anchor>,
) -> Option<proto::Excerpt> {
    Some(proto::Excerpt {
        id: id.to_proto(),
        buffer_id,
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
            if !buffer.can_resolve(&scroll_anchor.top_anchor) {
                scroll_anchor.top_anchor = buffer.anchor_before(
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

    fn tab_description<'a>(&'a self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>> {
        match path_for_buffer(&self.buffer, detail, true, cx)? {
            Cow::Borrowed(path) => Some(path.to_string_lossy()),
            Cow::Owned(path) => Some(path.to_string_lossy().to_string().into()),
        }
    }

    fn tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> ElementBox {
        Flex::row()
            .with_child(
                Label::new(self.title(cx).to_string(), style.label.clone())
                    .aligned()
                    .boxed(),
            )
            .with_children(detail.and_then(|detail| {
                let path = path_for_buffer(&self.buffer, detail, false, cx)?;
                let description = path.to_string_lossy();
                Some(
                    Label::new(
                        if description.len() > MAX_TAB_TITLE_LEN {
                            description[..MAX_TAB_TITLE_LEN].to_string() + "…"
                        } else {
                            description.into()
                        },
                        style.description.text.clone(),
                    )
                    .contained()
                    .with_style(style.description.container)
                    .aligned()
                    .boxed(),
                )
            }))
            .boxed()
    }

    fn for_each_project_item(&self, cx: &AppContext, f: &mut dyn FnMut(usize, &dyn project::Item)) {
        self.buffer
            .read(cx)
            .for_each_buffer(|buffer| f(buffer.id(), buffer.read(cx)));
    }

    fn is_singleton(&self, cx: &AppContext) -> bool {
        self.buffer.read(cx).is_singleton()
    }

    fn clone_on_split(&self, _workspace_id: WorkspaceId, cx: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone(cx))
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, _: &mut ViewContext<Self>) {
        self.nav_history = Some(history);
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        let selection = self.selections.newest_anchor();
        self.push_to_nav_history(selection.head(), None, cx);
    }

    fn workspace_deactivated(&mut self, cx: &mut ViewContext<Self>) {
        hide_link_definition(self, cx);
        self.link_go_to_definition_state.last_mouse_location = None;
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
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.report_event("save editor", cx);
        let format = self.perform_format(project.clone(), FormatTrigger::Save, cx);
        let buffers = self.buffer().clone().read(cx).all_buffers();
        cx.as_mut().spawn(|mut cx| async move {
            format.await?;
            project
                .update(&mut cx, |project, cx| project.save_buffers(buffers, cx))
                .await?;
            Ok(())
        })
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
            .expect("cannot call save_as on an excerpt list");

        project.update(cx, |project, cx| {
            project.save_buffer_as(buffer, abs_path, cx)
        })
    }

    fn reload(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        let buffer = self.buffer().clone();
        let buffers = self.buffer.read(cx).all_buffers();
        let reload_buffers =
            project.update(cx, |project, cx| project.reload_buffers(buffers, true, cx));
        cx.spawn(|this, mut cx| async move {
            let transaction = reload_buffers.log_err().await;
            this.update(&mut cx, |editor, cx| {
                editor.request_autoscroll(Autoscroll::fit(), cx)
            });
            buffer.update(&mut cx, |buffer, _| {
                if let Some(transaction) = transaction {
                    if !buffer.is_singleton() {
                        buffer.push_transaction(&transaction.0);
                    }
                }
            });
            Ok(())
        })
    }

    fn git_diff_recalc(
        &mut self,
        _project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        self.buffer().update(cx, |multibuffer, cx| {
            multibuffer.git_diff_recalc(cx);
        });
        Task::ready(Ok(()))
    }

    fn to_item_events(event: &Self::Event) -> SmallVec<[ItemEvent; 2]> {
        let mut result = SmallVec::new();
        match event {
            Event::Closed => result.push(ItemEvent::CloseItem),
            Event::Saved | Event::TitleChanged => {
                result.push(ItemEvent::UpdateTab);
                result.push(ItemEvent::UpdateBreadcrumbs);
            }
            Event::Reparsed => {
                result.push(ItemEvent::UpdateBreadcrumbs);
            }
            Event::SelectionsChanged { local } if *local => {
                result.push(ItemEvent::UpdateBreadcrumbs);
            }
            Event::DirtyChanged => {
                result.push(ItemEvent::UpdateTab);
            }
            Event::BufferEdited => {
                result.push(ItemEvent::Edit);
                result.push(ItemEvent::UpdateBreadcrumbs);
            }
            _ => {}
        }
        result
    }

    fn as_searchable(&self, handle: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft { flex: None }
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &AppContext) -> Option<Vec<ElementBox>> {
        let cursor = self.selections.newest_anchor().head();
        let multibuffer = &self.buffer().read(cx);
        let (buffer_id, symbols) =
            multibuffer.symbols_containing(cursor, Some(&theme.editor.syntax), cx)?;
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

        let mut breadcrumbs = vec![Label::new(filename, theme.breadcrumbs.text.clone()).boxed()];
        breadcrumbs.extend(symbols.into_iter().map(|symbol| {
            Text::new(symbol.text, theme.breadcrumbs.text.clone())
                .with_highlights(symbol.highlight_ranges)
                .boxed()
        }));
        Some(breadcrumbs)
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        let workspace_id = workspace.database_id();
        let item_id = cx.view_id();
        self.workspace_id = Some(workspace_id);

        fn serialize(
            buffer: ModelHandle<Buffer>,
            workspace_id: WorkspaceId,
            item_id: ItemId,
            cx: &mut MutableAppContext,
        ) {
            if let Some(file) = buffer.read(cx).file().and_then(|file| file.as_local()) {
                let path = file.abs_path(cx);

                cx.background()
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
                if let Some(workspace_id) = this.workspace_id {
                    if let language::Event::FileHandleChanged = event {
                        serialize(buffer, workspace_id, cx.view_id(), cx);
                    }
                }
            })
            .detach();
        }
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some("Editor")
    }

    fn deserialize(
        project: ModelHandle<Project>,
        _workspace: WeakViewHandle<Workspace>,
        workspace_id: workspace::WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<Result<ViewHandle<Self>>> {
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
                        .context("Project item at stored path was not a buffer")?;

                    Ok(cx.update(|cx| {
                        cx.add_view(pane, |cx| {
                            let mut editor = Editor::for_buffer(buffer, Some(project), cx);
                            editor.read_scroll_position_from_db(item_id, workspace_id, cx);
                            editor
                        })
                    }))
                })
            })
            .unwrap_or_else(|error| Task::ready(Err(error)))
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

enum BufferSearchHighlights {}
impl SearchableItem for Editor {
    type Match = Range<Anchor>;

    fn to_search_event(event: &Self::Event) -> Option<SearchEvent> {
        match event {
            Event::BufferEdited => Some(SearchEvent::MatchesInvalidated),
            Event::SelectionsChanged { .. } => Some(SearchEvent::ActiveMatchChanged),
            _ => None,
        }
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.clear_background_highlights::<BufferSearchHighlights>(cx);
    }

    fn update_matches(&mut self, matches: Vec<Range<Anchor>>, cx: &mut ViewContext<Self>) {
        self.highlight_background::<BufferSearchHighlights>(
            matches,
            |theme| theme.search.match_background,
            cx,
        );
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        let display_map = self.snapshot(cx).display_snapshot;
        let selection = self.selections.newest::<usize>(cx);
        if selection.start == selection.end {
            let point = selection.start.to_display_point(&display_map);
            let range = surrounding_word(&display_map, point);
            let range = range.start.to_offset(&display_map, Bias::Left)
                ..range.end.to_offset(&display_map, Bias::Right);
            let text: String = display_map.buffer_snapshot.text_for_range(range).collect();
            if text.trim().is_empty() {
                String::new()
            } else {
                text
            }
        } else {
            display_map
                .buffer_snapshot
                .text_for_range(selection.start..selection.end)
                .collect()
        }
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: Vec<Range<Anchor>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.unfold_ranges([matches[index].clone()], false, cx);
        self.change_selections(Some(Autoscroll::fit()), cx, |s| {
            s.select_ranges([matches[index].clone()])
        });
    }

    fn match_index_for_direction(
        &mut self,
        matches: &Vec<Range<Anchor>>,
        mut current_index: usize,
        direction: Direction,
        cx: &mut ViewContext<Self>,
    ) -> usize {
        let buffer = self.buffer().read(cx).snapshot(cx);
        let cursor = self.selections.newest_anchor().head();
        if matches[current_index].start.cmp(&cursor, &buffer).is_gt() {
            if direction == Direction::Prev {
                if current_index == 0 {
                    current_index = matches.len() - 1;
                } else {
                    current_index -= 1;
                }
            }
        } else if matches[current_index].end.cmp(&cursor, &buffer).is_lt() {
            if direction == Direction::Next {
                current_index = 0;
            }
        } else if direction == Direction::Prev {
            if current_index == 0 {
                current_index = matches.len() - 1;
            } else {
                current_index -= 1;
            }
        } else if direction == Direction::Next {
            if current_index == matches.len() - 1 {
                current_index = 0
            } else {
                current_index += 1;
            }
        };
        current_index
    }

    fn find_matches(
        &mut self,
        query: project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Range<Anchor>>> {
        let buffer = self.buffer().read(cx).snapshot(cx);
        cx.background().spawn(async move {
            let mut ranges = Vec::new();
            if let Some((_, _, excerpt_buffer)) = buffer.as_singleton() {
                ranges.extend(
                    query
                        .search(excerpt_buffer.as_rope())
                        .await
                        .into_iter()
                        .map(|range| {
                            buffer.anchor_after(range.start)..buffer.anchor_before(range.end)
                        }),
                );
            } else {
                for excerpt in buffer.excerpt_boundaries_in_range(0..buffer.len()) {
                    let excerpt_range = excerpt.range.context.to_offset(&excerpt.buffer);
                    let rope = excerpt.buffer.as_rope().slice(excerpt_range.clone());
                    ranges.extend(query.search(&rope).await.into_iter().map(|range| {
                        let start = excerpt
                            .buffer
                            .anchor_after(excerpt_range.start + range.start);
                        let end = excerpt
                            .buffer
                            .anchor_before(excerpt_range.start + range.end);
                        buffer.anchor_in_excerpt(excerpt.id.clone(), start)
                            ..buffer.anchor_in_excerpt(excerpt.id.clone(), end)
                    }));
                }
            }
            ranges
        })
    }

    fn active_match_index(
        &mut self,
        matches: Vec<Range<Anchor>>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        active_match_index(
            &matches,
            &self.selections.newest_anchor().head(),
            &self.buffer().read(cx).snapshot(cx),
        )
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
            if probe.end.cmp(cursor, &*buffer).is_lt() {
                Ordering::Less
            } else if probe.start.cmp(cursor, &*buffer).is_gt() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            Ok(i) | Err(i) => Some(cmp::min(i, ranges.len() - 1)),
        }
    }
}

pub struct CursorPosition {
    position: Option<Point>,
    selected_count: usize,
    _observe_active_editor: Option<Subscription>,
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::new()
    }
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
        for selection in editor.selections.all::<usize>(cx) {
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
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_position));
            self.update_position(editor, cx);
        } else {
            self.position = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}

fn path_for_buffer<'a>(
    buffer: &ModelHandle<MultiBuffer>,
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
    use gpui::MutableAppContext;
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
        time::SystemTime,
    };

    #[gpui::test]
    fn test_path_for_file(cx: &mut MutableAppContext) {
        let file = TestFile {
            path: Path::new("").into(),
            full_path: PathBuf::from(""),
        };
        assert_eq!(path_for_file(&file, 0, false, cx), None);
    }

    struct TestFile {
        path: Arc<Path>,
        full_path: PathBuf,
    }

    impl language::File for TestFile {
        fn path(&self) -> &Arc<Path> {
            &self.path
        }

        fn full_path(&self, _: &gpui::AppContext) -> PathBuf {
            self.full_path.clone()
        }

        fn as_local(&self) -> Option<&dyn language::LocalFile> {
            todo!()
        }

        fn mtime(&self) -> SystemTime {
            todo!()
        }

        fn file_name<'a>(&'a self, _: &'a gpui::AppContext) -> &'a std::ffi::OsStr {
            todo!()
        }

        fn is_deleted(&self) -> bool {
            todo!()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            todo!()
        }

        fn to_proto(&self) -> rpc::proto::File {
            todo!()
        }
    }
}
