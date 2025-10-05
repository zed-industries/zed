use anyhow::Result;
use call::ActiveCall;
use channel::{Channel, ChannelBuffer, ChannelBufferEvent, ChannelStore};
use client::{
    ChannelId, Collaborator, ParticipantIndex,
    proto::{self, PeerId},
};
use collections::HashMap;
use editor::{
    CollaborationHub, DisplayPoint, Editor, EditorEvent, SelectionEffects,
    display_map::ToDisplayPoint, scroll::Autoscroll,
};
use gpui::{
    AnyView, App, ClipboardItem, Context, Entity, EventEmitter, Focusable, Pixels, Point, Render,
    Subscription, Task, VisualContext as _, WeakEntity, Window, actions,
};
use project::Project;
use rpc::proto::ChannelVisibility;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::prelude::*;
use util::ResultExt;
use workspace::{CollaboratorId, item::TabContentParams};
use workspace::{
    ItemNavHistory, Pane, SaveIntent, Toast, ViewId, Workspace, WorkspaceId,
    item::{FollowableItem, Item, ItemEvent, ItemHandle},
    searchable::SearchableItemHandle,
};
use workspace::{item::Dedup, notifications::NotificationId};

actions!(
    collab,
    [
        /// Copies a link to the current position in the channel buffer.
        CopyLink
    ]
);

pub fn init(cx: &mut App) {
    workspace::FollowableViewRegistry::register::<ChannelView>(cx)
}

pub struct ChannelView {
    pub editor: Entity<Editor>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    channel_store: Entity<ChannelStore>,
    channel_buffer: Entity<ChannelBuffer>,
    remote_id: Option<ViewId>,
    _editor_event_subscription: Subscription,
    _reparse_subscription: Option<Subscription>,
}

impl ChannelView {
    pub fn open(
        channel_id: ChannelId,
        link_position: Option<String>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let pane = workspace.read(cx).active_pane().clone();
        let channel_view = Self::open_in_pane(
            channel_id,
            link_position,
            pane.clone(),
            workspace,
            window,
            cx,
        );
        window.spawn(cx, async move |cx| {
            let channel_view = channel_view.await?;
            pane.update_in(cx, |pane, window, cx| {
                telemetry::event!(
                    "Channel Notes Opened",
                    channel_id,
                    room_id = ActiveCall::global(cx)
                        .read(cx)
                        .room()
                        .map(|r| r.read(cx).id())
                );
                pane.add_item(Box::new(channel_view.clone()), true, true, None, window, cx);
            })?;
            anyhow::Ok(channel_view)
        })
    }

    pub fn open_in_pane(
        channel_id: ChannelId,
        link_position: Option<String>,
        pane: Entity<Pane>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let channel_view = Self::load(channel_id, workspace, window, cx);
        window.spawn(cx, async move |cx| {
            let channel_view = channel_view.await?;

            pane.update_in(cx, |pane, window, cx| {
                let buffer_id = channel_view.read(cx).channel_buffer.read(cx).remote_id(cx);

                let existing_view = pane
                    .items_of_type::<Self>()
                    .find(|view| view.read(cx).channel_buffer.read(cx).remote_id(cx) == buffer_id);

                // If this channel buffer is already open in this pane, just return it.
                if let Some(existing_view) = existing_view.clone()
                    && existing_view.read(cx).channel_buffer == channel_view.read(cx).channel_buffer
                {
                    if let Some(link_position) = link_position {
                        existing_view.update(cx, |channel_view, cx| {
                            channel_view.focus_position_from_link(link_position, true, window, cx)
                        });
                    }
                    return existing_view;
                }

                // If the pane contained a disconnected view for this channel buffer,
                // replace that.
                if let Some(existing_item) = existing_view
                    && let Some(ix) = pane.index_for_item(&existing_item)
                {
                    pane.close_item_by_id(existing_item.entity_id(), SaveIntent::Skip, window, cx)
                        .detach();
                    pane.add_item(
                        Box::new(channel_view.clone()),
                        true,
                        true,
                        Some(ix),
                        window,
                        cx,
                    );
                }

                if let Some(link_position) = link_position {
                    channel_view.update(cx, |channel_view, cx| {
                        channel_view.focus_position_from_link(link_position, true, window, cx)
                    });
                }

                channel_view
            })
        })
    }

    pub fn load(
        channel_id: ChannelId,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let weak_workspace = workspace.downgrade();
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();
        let channel_store = ChannelStore::global(cx);
        let language_registry = workspace.app_state().languages.clone();
        let markdown = language_registry.language_for_name("Markdown");
        let channel_buffer =
            channel_store.update(cx, |store, cx| store.open_channel_buffer(channel_id, cx));

        window.spawn(cx, async move |cx| {
            let channel_buffer = channel_buffer.await?;
            let markdown = markdown.await.log_err();

            channel_buffer.update(cx, |channel_buffer, cx| {
                channel_buffer.buffer().update(cx, |buffer, cx| {
                    buffer.set_language_registry(language_registry);
                    let Some(markdown) = markdown else {
                        return;
                    };
                    buffer.set_language(Some(markdown), cx);
                })
            })?;

            cx.new_window_entity(|window, cx| {
                let mut this = Self::new(
                    project,
                    weak_workspace,
                    channel_store,
                    channel_buffer,
                    window,
                    cx,
                );
                this.acknowledge_buffer_version(cx);
                this
            })
        })
    }

    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        channel_store: Entity<ChannelStore>,
        channel_buffer: Entity<ChannelBuffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        let this = cx.entity().downgrade();
        let editor = cx.new(|cx| {
            let mut editor = Editor::for_buffer(buffer, None, window, cx);
            editor.set_collaboration_hub(Box::new(ChannelBufferCollaborationHub(
                channel_buffer.clone(),
            )));
            editor.set_custom_context_menu(move |_, position, window, cx| {
                let this = this.clone();
                Some(ui::ContextMenu::build(window, cx, move |menu, _, _| {
                    menu.entry("Copy link to section", None, move |window, cx| {
                        this.update(cx, |this, cx| {
                            this.copy_link_for_position(position, window, cx)
                        })
                        .ok();
                    })
                }))
            });
            editor
        });
        let _editor_event_subscription =
            cx.subscribe(&editor, |_, _, e: &EditorEvent, cx| cx.emit(e.clone()));

        cx.subscribe_in(&channel_buffer, window, Self::handle_channel_buffer_event)
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let position = Channel::slug(&position).to_lowercase();
        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));

        if let Some(outline) = snapshot.buffer_snapshot().outline(None)
            && let Some(item) = outline
                .items
                .iter()
                .find(|item| &Channel::slug(&item.text).to_lowercase() == &position)
        {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    SelectionEffects::scroll(Autoscroll::focused()),
                    window,
                    cx,
                    |s| s.replace_cursors_with(|map| vec![item.range.start.to_display_point(map)]),
                )
            });
            return;
        }

        if !first_attempt {
            return;
        }
        self._reparse_subscription = Some(cx.subscribe_in(
            &self.editor,
            window,
            move |this, _, e: &EditorEvent, window, cx| {
                match e {
                    EditorEvent::Reparsed(_) => {
                        this.focus_position_from_link(position.clone(), false, window, cx);
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

    fn copy_link(&mut self, _: &CopyLink, window: &mut Window, cx: &mut Context<Self>) {
        let position = self
            .editor
            .update(cx, |editor, cx| editor.selections.newest_display(cx).start);
        self.copy_link_for_position(position, window, cx)
    }

    fn copy_link_for_position(
        &self,
        position: DisplayPoint,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let snapshot = self
            .editor
            .update(cx, |editor, cx| editor.snapshot(window, cx));

        let mut closest_heading = None;

        if let Some(outline) = snapshot.buffer_snapshot().outline(None) {
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
            .update(cx, |workspace, cx| {
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

    pub fn channel(&self, cx: &App) -> Option<Arc<Channel>> {
        self.channel_buffer.read(cx).channel(cx)
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: &Entity<ChannelBuffer>,
        event: &ChannelBufferEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ChannelBufferEvent::Disconnected => self.editor.update(cx, |editor, cx| {
                editor.set_read_only(true);
                cx.notify();
            }),
            ChannelBufferEvent::Connected => self.editor.update(cx, |editor, cx| {
                editor.set_read_only(false);
                cx.notify();
            }),
            ChannelBufferEvent::ChannelChanged => {
                self.editor.update(cx, |_, cx| {
                    cx.emit(editor::EditorEvent::TitleChanged);
                    cx.notify()
                });
            }
            ChannelBufferEvent::BufferEdited => {
                if self.editor.read(cx).is_focused(window) {
                    self.acknowledge_buffer_version(cx);
                } else {
                    self.channel_store.update(cx, |store, cx| {
                        let channel_buffer = self.channel_buffer.read(cx);
                        store.update_latest_notes_version(
                            channel_buffer.channel_id,
                            channel_buffer.epoch(),
                            &channel_buffer.buffer().read(cx).version(),
                            cx,
                        )
                    });
                }
            }
            ChannelBufferEvent::CollaboratorsChanged => {}
        }
    }

    fn acknowledge_buffer_version(&mut self, cx: &mut Context<ChannelView>) {
        self.channel_store.update(cx, |store, cx| {
            let channel_buffer = self.channel_buffer.read(cx);
            store.acknowledge_notes_version(
                channel_buffer.channel_id,
                channel_buffer.epoch(),
                &channel_buffer.buffer().read(cx).version(),
                cx,
            )
        });
        self.channel_buffer.update(cx, |buffer, cx| {
            buffer.acknowledge_buffer_version(cx);
        });
    }

    fn get_channel(&self, cx: &App) -> (SharedString, Option<SharedString>) {
        if let Some(channel) = self.channel(cx) {
            let status = match (
                self.channel_buffer.read(cx).buffer().read(cx).read_only(),
                self.channel_buffer.read(cx).is_connected(),
            ) {
                (false, true) => None,
                (true, true) => Some("read-only"),
                (_, false) => Some("disconnected"),
            };

            (channel.name.clone(), status.map(Into::into))
        } else {
            ("<unknown>".into(), Some("disconnected".into()))
        }
    }
}

impl EventEmitter<EditorEvent> for ChannelView {}

impl Render for ChannelView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .on_action(cx.listener(Self::copy_link))
            .child(self.editor.clone())
    }
}

impl Focusable for ChannelView {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.editor.read(cx).focus_handle(cx)
    }
}

impl Item for ChannelView {
    type Event = EditorEvent;

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn tab_icon(&self, _: &Window, cx: &App) -> Option<Icon> {
        let channel = self.channel(cx)?;
        let icon = match channel.visibility {
            ChannelVisibility::Public => IconName::Public,
            ChannelVisibility::Members => IconName::Hash,
        };

        Some(Icon::new(icon))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let (name, status) = self.get_channel(cx);
        if let Some(status) = status {
            format!("{name} - {status}").into()
        } else {
            name
        }
    }

    fn tab_content(&self, params: TabContentParams, _: &Window, cx: &App) -> gpui::AnyElement {
        let (name, status) = self.get_channel(cx);
        h_flex()
            .gap_2()
            .child(
                Label::new(name)
                    .color(params.text_color())
                    .when(params.preview, |this| this.italic()),
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>> {
        Some(cx.new(|cx| {
            Self::new(
                self.project.clone(),
                self.workspace.clone(),
                self.channel_store.clone(),
                self.channel_buffer.clone(),
                window,
                cx,
            )
        }))
    }

    fn navigate(
        &mut self,
        data: Box<dyn Any>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |item, cx| item.deactivated(window, cx))
    }

    fn set_nav_history(
        &mut self,
        history: ItemNavHistory,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, history, window, cx)
        })
    }

    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, cx: &App) -> Option<Point<Pixels>> {
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

    fn to_state_proto(&self, window: &Window, cx: &App) -> Option<proto::view::Variant> {
        let channel_buffer = self.channel_buffer.read(cx);
        if !channel_buffer.is_connected() {
            return None;
        }

        Some(proto::view::Variant::ChannelView(
            proto::view::ChannelView {
                channel_id: channel_buffer.channel_id.0,
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(window, cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        workspace: Entity<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<gpui::Task<anyhow::Result<Entity<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::ChannelView(state)) = state.take() else {
            unreachable!()
        };

        let open = ChannelView::load(ChannelId(state.channel_id), workspace, window, cx);

        Some(window.spawn(cx, async move |cx| {
            let this = open.await?;

            let task = this.update_in(cx, |this, window, cx| {
                this.remote_id = Some(remote_id);

                if let Some(state) = state.editor {
                    Some(this.editor.update(cx, |editor, cx| {
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
                            window,
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
        cx: &App,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, window, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Entity<Project>,
        message: proto::update_view::Variant,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, window, cx)
        })
    }

    fn set_leader_id(
        &mut self,
        leader_id: Option<CollaboratorId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.set_leader_id(leader_id, window, cx))
    }

    fn is_project_item(&self, _window: &Window, _cx: &App) -> bool {
        false
    }

    fn to_follow_event(event: &Self::Event) -> Option<workspace::item::FollowEvent> {
        Editor::to_follow_event(event)
    }

    fn dedup(&self, existing: &Self, _: &Window, cx: &App) -> Option<Dedup> {
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

struct ChannelBufferCollaborationHub(Entity<ChannelBuffer>);

impl CollaborationHub for ChannelBufferCollaborationHub {
    fn collaborators<'a>(&self, cx: &'a App) -> &'a HashMap<PeerId, Collaborator> {
        self.0.read(cx).collaborators()
    }

    fn user_participant_indices<'a>(&self, cx: &'a App) -> &'a HashMap<u64, ParticipantIndex> {
        self.0.read(cx).user_store().read(cx).participant_indices()
    }

    fn user_names(&self, cx: &App) -> HashMap<u64, SharedString> {
        let user_ids = self.collaborators(cx).values().map(|c| c.user_id);
        self.0
            .read(cx)
            .user_store()
            .read(cx)
            .participant_names(user_ids, cx)
    }
}
