use anyhow::Result;
use call::report_call_event_for_channel;
use channel::{Channel, ChannelBuffer, ChannelBufferEvent, ChannelStore};
use client::{
    proto::{self, PeerId},
    ChannelId, Collaborator, ParticipantIndex,
};
use collections::HashMap;
use editor::{
    display_map::ToDisplayPoint, scroll::Autoscroll, CollaborationHub, DisplayPoint, Editor,
    EditorEvent,
};
use gpui::{
    actions, AnyView, AppContext, AppContext, ClipboardItem, Entity as _, EventEmitter,
    FocusableView, Model, Pixels, Point, Render, Subscription, Task, View, VisualContext as _,
    WeakView,
};
use project::Project;
use rpc::proto::ChannelVisibility;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::prelude::*;
use util::ResultExt;
use workspace::item::TabContentParams;
use workspace::{item::Dedup, notifications::NotificationId};
use workspace::{
    item::{FollowableItem, Item, ItemEvent, ItemHandle},
    searchable::SearchableItemHandle,
    ItemNavHistory, Pane, SaveIntent, Toast, ViewId, Workspace, WorkspaceId,
};

actions!(collab, [CopyLink]);

pub fn init(cx: &mut AppContext) {
    workspace::FollowableViewRegistry::register::<ChannelView>(cx)
}

pub struct ChannelView {
    pub editor: Model<Editor>,
    workspace: WeakModel<Workspace>,
    project: Model<Project>,
    channel_store: Model<ChannelStore>,
    channel_buffer: Model<ChannelBuffer>,
    remote_id: Option<ViewId>,
    _editor_event_subscription: Subscription,
    _reparse_subscription: Option<Subscription>,
}

impl ChannelView {
    pub fn open(
        channel_id: ChannelId,
        link_position: Option<String>,
        workspace: Model<Workspace>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<Model<Self>>> {
        let pane = workspace.read(cx).active_pane().clone();
        let channel_view = Self::open_in_pane(
            channel_id,
            link_position,
            pane.clone(),
            workspace.clone(),
            model,
            cx,
        );
        cx.spawn(|mut cx| async move {
            let channel_view = channel_view.await?;
            pane.update(&mut cx, |pane, cx| {
                report_call_event_for_channel(
                    "open channel notes",
                    channel_id,
                    &workspace.read(cx).app_state().client,
                    cx,
                );
                pane.add_item(Box::new(channel_view.clone()), true, true, None, cx);
            })?;
            anyhow::Ok(channel_view)
        })
    }

    pub fn open_in_pane(
        channel_id: ChannelId,
        link_position: Option<String>,
        pane: Model<Pane>,
        workspace: Model<Workspace>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<Model<Self>>> {
        let channel_view = Self::load(channel_id, workspace, model, cx);
        cx.spawn(|mut cx| async move {
            let channel_view = channel_view.await?;

            pane.update(&mut cx, |pane, cx| {
                let buffer_id = channel_view.read(cx).channel_buffer.read(cx).remote_id(cx);

                let existing_view = pane
                    .items_of_type::<Self>()
                    .find(|view| view.read(cx).channel_buffer.read(cx).remote_id(cx) == buffer_id);

                // If this channel buffer is already open in this pane, just return it.
                if let Some(existing_view) = existing_view.clone() {
                    if existing_view.read(cx).channel_buffer == channel_view.read(cx).channel_buffer
                    {
                        if let Some(link_position) = link_position {
                            existing_view.update(cx, |channel_view, model, cx| {
                                channel_view.focus_position_from_link(link_position, true, cx)
                            });
                        }
                        return existing_view;
                    }
                }

                // If the pane contained a disconnected view for this channel buffer,
                // replace that.
                if let Some(existing_item) = existing_view {
                    if let Some(ix) = pane.index_for_item(&existing_item) {
                        pane.close_item_by_id(existing_item.entity_id(), SaveIntent::Skip, cx)
                            .detach();
                        pane.add_item(Box::new(channel_view.clone()), true, true, Some(ix), cx);
                    }
                }

                if let Some(link_position) = link_position {
                    channel_view.update(cx, |channel_view, model, cx| {
                        channel_view.focus_position_from_link(link_position, true, model, cx)
                    });
                }

                channel_view
            })
        })
    }

    pub fn load(
        channel_id: ChannelId,
        workspace: Model<Workspace>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<Model<Self>>> {
        let weak_workspace = workspace.downgrade();
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();
        let channel_store = ChannelStore::global(cx);
        let language_registry = workspace.app_state().languages.clone();
        let markdown = language_registry.language_for_name("Markdown");
        let channel_buffer = channel_store.update(cx, |store, model, cx| {
            store.open_channel_buffer(channel_id, model, cx)
        });

        cx.spawn(|mut cx| async move {
            let channel_buffer = channel_buffer.await?;
            let markdown = markdown.await.log_err();

            channel_buffer.update(&mut cx, |channel_buffer, cx| {
                channel_buffer.buffer().update(cx, |buffer, model, cx| {
                    buffer.set_language_registry(language_registry);
                    let Some(markdown) = markdown else {
                        return;
                    };
                    buffer.set_language(Some(markdown), cx);
                })
            })?;

            cx.new_model(|model, cx| {
                let mut this = Self::new(
                    project,
                    weak_workspace,
                    channel_store,
                    channel_buffer,
                    model,
                    cx,
                );
                this.acknowledge_buffer_version(model, cx);
                this
            })
        })
    }

    pub fn new(
        project: Model<Project>,
        workspace: WeakModel<Workspace>,
        channel_store: Model<ChannelStore>,
        channel_buffer: Model<ChannelBuffer>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        let this = model.downgrade();
        let editor = cx.new_model(|model, cx| {
            let mut editor = Editor::for_buffer(buffer, None, model, cx);
            editor.set_collaboration_hub(Box::new(ChannelBufferCollaborationHub(
                channel_buffer.clone(),
            )));
            editor.set_custom_context_menu(move |_, position, cx| {
                let this = this.clone();
                Some(ui::ContextMenu::build(
                    window,
                    cx,
                    move |menu, model, window, cx| {
                        menu.entry("Copy link to section", None, move |cx| {
                            this.update(cx, |this, model, cx| {
                                this.copy_link_for_position(position, cx)
                            })
                            .ok();
                        })
                    },
                ))
            });
            editor
        });
        let _editor_event_subscription = cx.subscribe(&editor, |_, _, e: &EditorEvent, cx| {
            model.emit(e.clone(), cx)
        });

        cx.subscribe(&channel_buffer, Self::handle_channel_buffer_event)
            .detach();

        Self {
            editor,
            workspace,
            project,
            channel_store,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
            _reparse_subscription: None,
        }
    }

    fn focus_position_from_link(
        &mut self,
        position: String,
        first_attempt: bool,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let position = Channel::slug(&position).to_lowercase();
        let snapshot = self
            .editor
            .update(cx, |editor, model, cx| editor.snapshot(model, cx));

        if let Some(outline) = snapshot.buffer_snapshot.outline(None) {
            if let Some(item) = outline
                .items
                .iter()
                .find(|item| &Channel::slug(&item.text).to_lowercase() == &position)
            {
                self.editor.update(cx, |editor, model, cx| {
                    editor.change_selections(Some(Autoscroll::focused()), model, cx, |s| {
                        s.replace_cursors_with(|map| vec![item.range.start.to_display_point(map)])
                    })
                });
                return;
            }
        }

        if !first_attempt {
            return;
        }
        self._reparse_subscription = Some(cx.subscribe(
            &self.editor,
            move |this, _, e: &EditorEvent, cx| {
                match e {
                    EditorEvent::Reparsed(_) => {
                        this.focus_position_from_link(position.clone(), false, cx);
                        this._reparse_subscription.take();
                    }
                    EditorEvent::Edited { .. } | EditorEvent::SelectionsChanged { local: true } => {
                        this._reparse_subscription.take();
                    }
                    _ => {}
                };
            },
        ));
    }

    fn copy_link(&mut self, _: &CopyLink, model: &Model<Self>, cx: &mut AppContext) {
        let position = self.editor.update(cx, |editor, model, cx| {
            editor.selections.newest_display(cx).start
        });
        self.copy_link_for_position(position, model, cx)
    }

    fn copy_link_for_position(
        &self,
        position: DisplayPoint,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        let snapshot = self
            .editor
            .update(cx, |editor, model, cx| editor.snapshot(model, cx));

        let mut closest_heading = None;

        if let Some(outline) = snapshot.buffer_snapshot.outline(None) {
            for item in outline.items {
                if item.range.start.to_display_point(&snapshot) > position {
                    break;
                }
                closest_heading = Some(item);
            }
        }

        let Some(channel) = self.channel(cx) else {
            return;
        };

        let link = channel.notes_link(closest_heading.map(|heading| heading.text), cx);
        cx.write_to_clipboard(ClipboardItem::new_string(link));
        self.workspace
            .update(cx, |workspace, model, cx| {
                struct CopyLinkForPositionToast;

                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<CopyLinkForPositionToast>(),
                        "Link copied to clipboard",
                    ),
                    cx,
                );
            })
            .ok();
    }

    pub fn channel(&self, cx: &AppContext) -> Option<Arc<Channel>> {
        self.channel_buffer.read(cx).channel(cx)
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: Model<ChannelBuffer>,
        event: &ChannelBufferEvent,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        match event {
            ChannelBufferEvent::Disconnected => self.editor.update(cx, |editor, model, cx| {
                editor.set_read_only(true);
                model.notify(cx);
            }),
            ChannelBufferEvent::ChannelChanged => {
                self.editor.update(cx, |_, model, cx| {
                    model.emit(editor::EditorEvent::TitleChanged, cx);
                    model.notify(cx)
                });
            }
            ChannelBufferEvent::BufferEdited => {
                if self.editor.read(cx).is_focused(window) {
                    self.acknowledge_buffer_version(model, cx);
                } else {
                    self.channel_store.update(cx, |store, model, cx| {
                        let channel_buffer = self.channel_buffer.read(cx);
                        store.update_latest_notes_version(
                            channel_buffer.channel_id,
                            channel_buffer.epoch(),
                            &channel_buffer.buffer().read(cx).version(),
                            model,
                            cx,
                        )
                    });
                }
            }
            ChannelBufferEvent::CollaboratorsChanged => {}
        }
    }

    fn acknowledge_buffer_version(&mut self, model: &Model<ChannelView>, cx: &mut AppContext) {
        self.channel_store.update(cx, |store, model, cx| {
            let channel_buffer = self.channel_buffer.read(cx);
            store.acknowledge_notes_version(
                channel_buffer.channel_id,
                channel_buffer.epoch(),
                &channel_buffer.buffer().read(cx).version(),
                model,
                cx,
            )
        });
        self.channel_buffer.update(cx, |buffer, model, cx| {
            buffer.acknowledge_buffer_version(model, cx);
        });
    }
}

impl EventEmitter<EditorEvent> for ChannelView {}

impl Render for ChannelView {
    fn render(
        &mut self,
        model: &Model<Self>,
        window: &mut gpui::Window,
        cx: &mut AppContext,
    ) -> impl IntoElement {
        div()
            .size_full()
            .on_action(cx.listener(Self::copy_link))
            .child(self.editor.clone())
    }
}

impl FocusableView for ChannelView {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Item for ChannelView {
    type Event = EditorEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Model<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.model())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.model())
        } else {
            None
        }
    }

    fn tab_icon(&self, window: &Window, cx: &AppContext) -> Option<Icon> {
        let channel = self.channel(cx)?;
        let icon = match channel.visibility {
            ChannelVisibility::Public => IconName::Public,
            ChannelVisibility::Members => IconName::Hash,
        };

        Some(Icon::new(icon))
    }

    fn tab_content(
        &self,
        params: TabContentParams,
        window: &Window,
        cx: &AppContext,
    ) -> gpui::AnyElement {
        let (channel_name, status) = if let Some(channel) = self.channel(cx) {
            let status = match (
                self.channel_buffer.read(cx).buffer().read(cx).read_only(),
                self.channel_buffer.read(cx).is_connected(),
            ) {
                (false, true) => None,
                (true, true) => Some("read-only"),
                (_, false) => Some("disconnected"),
            };

            (channel.name.clone(), status)
        } else {
            ("<unknown>".into(), Some("disconnected"))
        };

        h_flex()
            .gap_2()
            .child(
                Label::new(channel_name)
                    .color(params.text_color())
                    .italic(params.preview),
            )
            .when_some(status, |element, status| {
                element.child(
                    Label::new(status)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn clone_on_split(
        &self,
        _: Option<WorkspaceId>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> Option<Model<Self>> {
        Some(cx.new_model(|model, cx| {
            Self::new(
                self.project.clone(),
                self.workspace.clone(),
                self.channel_store.clone(),
                self.channel_buffer.clone(),
                model,
                cx,
            )
        }))
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        false
    }

    fn navigate(&mut self, data: Box<dyn Any>, model: &Model<Self>, cx: &mut AppContext) -> bool {
        self.editor
            .update(cx, |editor, model, cx| editor.navigate(data, model, cx))
    }

    fn deactivated(&mut self, model: &Model<Self>, cx: &mut AppContext) {
        self.editor.update(cx, Item::deactivated)
    }

    fn set_nav_history(
        &mut self,
        history: ItemNavHistory,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.editor.update(cx, |editor, model, cx| {
            Item::set_nav_history(editor, history, model, cx)
        })
    }

    fn as_searchable(&self, _: &Model<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Point<Pixels>> {
        self.editor.read(cx).pixel_position_of_cursor(cx)
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }
}

impl FollowableItem for ChannelView {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, window: &Window, cx: &AppContext) -> Option<proto::view::Variant> {
        let channel_buffer = self.channel_buffer.read(cx);
        if !channel_buffer.is_connected() {
            return None;
        }

        Some(proto::view::Variant::ChannelView(
            proto::view::ChannelView {
                channel_id: channel_buffer.channel_id.0,
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(model, cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        workspace: Model<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        window: &mut gpui::Window,
        cx: &mut gpui::AppContext,
    ) -> Option<gpui::Task<anyhow::Result<Model<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::ChannelView(state)) = state.take() else {
            unreachable!()
        };

        let open = ChannelView::load(ChannelId(state.channel_id), workspace, model, cx);

        Some(cx.spawn(|mut cx| async move {
            let this = open.await?;

            let task = this.update(&mut cx, |this, model, cx| {
                this.remote_id = Some(remote_id);

                if let Some(state) = state.editor {
                    Some(this.editor.update(cx, |editor, model, cx| {
                        editor.apply_update_proto(
                            &this.project,
                            proto::update_view::Variant::Editor(proto::update_view::Editor {
                                selections: state.selections,
                                pending_selection: state.pending_selection,
                                scroll_top_anchor: state.scroll_top_anchor,
                                scroll_x: state.scroll_x,
                                scroll_y: state.scroll_y,
                                ..Default::default()
                            }),
                            model,
                            cx,
                        )
                    }))
                } else {
                    None
                }
            })?;

            if let Some(task) = task {
                task.await?;
            }

            Ok(this)
        }))
    }

    fn add_event_to_update_proto(
        &self,
        event: &EditorEvent,
        update: &mut Option<proto::update_view::Variant>,
        window: &Window,
        cx: &AppContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, model, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.editor.update(cx, |editor, model, cx| {
            editor.apply_update_proto(project, message, model, cx)
        })
    }

    fn set_leader_peer_id(
        &mut self,
        leader_peer_id: Option<PeerId>,
        model: &Model<Self>,
        cx: &mut AppContext,
    ) {
        self.editor.update(cx, |editor, model, cx| {
            editor.set_leader_peer_id(leader_peer_id, model, cx)
        })
    }

    fn is_project_item(&self, _window: &Window, cx: &AppContext) -> bool {
        false
    }

    fn to_follow_event(event: &Self::Event) -> Option<workspace::item::FollowEvent> {
        Editor::to_follow_event(event)
    }

    fn dedup(&self, existing: &Self, window: &Window, cx: &AppContext) -> Option<Dedup> {
        let existing = existing.channel_buffer.read(cx);
        if self.channel_buffer.read(cx).channel_id == existing.channel_id {
            if existing.is_connected() {
                Some(Dedup::KeepExisting)
            } else {
                Some(Dedup::ReplaceExisting)
            }
        } else {
            None
        }
    }
}

struct ChannelBufferCollaborationHub(Model<ChannelBuffer>);

impl CollaborationHub for ChannelBufferCollaborationHub {
    fn collaborators<'a>(&self, cx: &'a AppContext) -> &'a HashMap<PeerId, Collaborator> {
        self.0.read(cx).collaborators()
    }

    fn user_participant_indices<'a>(
        &self,
        cx: &'a AppContext,
    ) -> &'a HashMap<u64, ParticipantIndex> {
        self.0.read(cx).user_store().read(cx).participant_indices()
    }

    fn user_names(&self, cx: &AppContext) -> HashMap<u64, SharedString> {
        let user_ids = self.collaborators(cx).values().map(|c| c.user_id);
        self.0
            .read(cx)
            .user_store()
            .read(cx)
            .participant_names(user_ids, cx)
    }
}
