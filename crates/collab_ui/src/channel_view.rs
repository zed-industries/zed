use anyhow::Result;
use call::report_call_event_for_channel;
use channel::{Channel, ChannelBuffer, ChannelBufferEvent, ChannelId, ChannelStore};
use client::{
    proto::{self, PeerId},
    Collaborator, ParticipantIndex,
};
use collections::HashMap;
use editor::{CollaborationHub, Editor, EditorEvent};
use gpui::{
    actions, AnyElement, AnyView, AppContext, Entity as _, EventEmitter, FocusableView,
    IntoElement as _, Model, Pixels, Point, Render, Subscription, Task, View, ViewContext,
    VisualContext as _, WindowContext,
};
use project::Project;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use ui::{prelude::*, Label};
use util::ResultExt;
use workspace::{
    item::{FollowableItem, Item, ItemEvent, ItemHandle},
    register_followable_item,
    searchable::SearchableItemHandle,
    ItemNavHistory, Pane, SaveIntent, ViewId, Workspace, WorkspaceId,
};

actions!(collab, [Deploy]);

pub fn init(cx: &mut AppContext) {
    register_followable_item::<ChannelView>(cx)
}

pub struct ChannelView {
    pub editor: View<Editor>,
    project: Model<Project>,
    channel_store: Model<ChannelStore>,
    channel_buffer: Model<ChannelBuffer>,
    remote_id: Option<ViewId>,
    _editor_event_subscription: Subscription,
}

impl ChannelView {
    pub fn open(
        channel_id: ChannelId,
        workspace: View<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<View<Self>>> {
        let pane = workspace.read(cx).active_pane().clone();
        let channel_view = Self::open_in_pane(channel_id, pane.clone(), workspace.clone(), cx);
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
        pane: View<Pane>,
        workspace: View<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<View<Self>>> {
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();
        let channel_store = ChannelStore::global(cx);
        let language_registry = workspace.app_state().languages.clone();
        let markdown = language_registry.language_for_name("Markdown");
        let channel_buffer =
            channel_store.update(cx, |store, cx| store.open_channel_buffer(channel_id, cx));

        cx.spawn(|mut cx| async move {
            let channel_buffer = channel_buffer.await?;
            let markdown = markdown.await.log_err();

            channel_buffer.update(&mut cx, |buffer, cx| {
                buffer.buffer().update(cx, |buffer, cx| {
                    buffer.set_language_registry(language_registry);
                    if let Some(markdown) = markdown {
                        buffer.set_language(Some(markdown), cx);
                    }
                })
            })?;

            pane.update(&mut cx, |pane, cx| {
                let buffer_id = channel_buffer.read(cx).remote_id(cx);

                let existing_view = pane
                    .items_of_type::<Self>()
                    .find(|view| view.read(cx).channel_buffer.read(cx).remote_id(cx) == buffer_id);

                // If this channel buffer is already open in this pane, just return it.
                if let Some(existing_view) = existing_view.clone() {
                    if existing_view.read(cx).channel_buffer == channel_buffer {
                        return existing_view;
                    }
                }

                let view = cx.new_view(|cx| {
                    let mut this = Self::new(project, channel_store, channel_buffer, cx);
                    this.acknowledge_buffer_version(cx);
                    this
                });

                // If the pane contained a disconnected view for this channel buffer,
                // replace that.
                if let Some(existing_item) = existing_view {
                    if let Some(ix) = pane.index_for_item(&existing_item) {
                        pane.close_item_by_id(existing_item.entity_id(), SaveIntent::Skip, cx)
                            .detach();
                        pane.add_item(Box::new(view.clone()), true, true, Some(ix), cx);
                    }
                }

                view
            })
        })
    }

    pub fn new(
        project: Model<Project>,
        channel_store: Model<ChannelStore>,
        channel_buffer: Model<ChannelBuffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, None, cx);
            editor.set_collaboration_hub(Box::new(ChannelBufferCollaborationHub(
                channel_buffer.clone(),
            )));
            editor.set_read_only(
                !channel_buffer
                    .read(cx)
                    .channel(cx)
                    .is_some_and(|c| c.can_edit_notes()),
            );
            editor
        });
        let _editor_event_subscription =
            cx.subscribe(&editor, |_, _, e: &EditorEvent, cx| cx.emit(e.clone()));

        cx.subscribe(&channel_buffer, Self::handle_channel_buffer_event)
            .detach();

        Self {
            editor,
            project,
            channel_store,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
        }
    }

    pub fn channel(&self, cx: &AppContext) -> Option<Arc<Channel>> {
        self.channel_buffer.read(cx).channel(cx)
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: Model<ChannelBuffer>,
        event: &ChannelBufferEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelBufferEvent::Disconnected => self.editor.update(cx, |editor, cx| {
                editor.set_read_only(true);
                cx.notify();
            }),
            ChannelBufferEvent::ChannelChanged => {
                self.editor.update(cx, |editor, cx| {
                    editor.set_read_only(!self.channel(cx).is_some_and(|c| c.can_edit_notes()));
                    cx.emit(editor::EditorEvent::TitleChanged);
                    cx.notify()
                });
            }
            ChannelBufferEvent::BufferEdited => {
                if self.editor.read(cx).is_focused(cx) {
                    self.acknowledge_buffer_version(cx);
                } else {
                    self.channel_store.update(cx, |store, cx| {
                        let channel_buffer = self.channel_buffer.read(cx);
                        store.notes_changed(
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

    fn acknowledge_buffer_version(&mut self, cx: &mut ViewContext<ChannelView>) {
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
}

impl EventEmitter<EditorEvent> for ChannelView {}

impl Render for ChannelView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.editor.clone()
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
        self_handle: &'a View<Self>,
        _: &'a AppContext,
    ) -> Option<AnyView> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.to_any())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.to_any())
        } else {
            None
        }
    }

    fn tab_content(&self, _: Option<usize>, selected: bool, cx: &WindowContext) -> AnyElement {
        let label = if let Some(channel) = self.channel(cx) {
            match (
                channel.can_edit_notes(),
                self.channel_buffer.read(cx).is_connected(),
            ) {
                (true, true) => format!("#{}", channel.name),
                (false, true) => format!("#{} (read-only)", channel.name),
                (_, false) => format!("#{} (disconnected)", channel.name),
            }
        } else {
            format!("channel notes (disconnected)")
        };
        Label::new(label)
            .color(if selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn clone_on_split(&self, _: WorkspaceId, cx: &mut ViewContext<Self>) -> Option<View<Self>> {
        Some(cx.new_view(|cx| {
            Self::new(
                self.project.clone(),
                self.channel_store.clone(),
                self.channel_buffer.clone(),
                cx,
            )
        }))
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        false
    }

    fn navigate(&mut self, data: Box<dyn Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }

    fn set_nav_history(&mut self, history: ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::set_nav_history(editor, history, cx))
    }

    fn as_searchable(&self, _: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
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

    fn to_state_proto(&self, cx: &WindowContext) -> Option<proto::view::Variant> {
        let channel_buffer = self.channel_buffer.read(cx);
        if !channel_buffer.is_connected() {
            return None;
        }

        Some(proto::view::Variant::ChannelView(
            proto::view::ChannelView {
                channel_id: channel_buffer.channel_id,
                editor: if let Some(proto::view::Variant::Editor(proto)) =
                    self.editor.read(cx).to_state_proto(cx)
                {
                    Some(proto)
                } else {
                    None
                },
            },
        ))
    }

    fn from_state_proto(
        pane: View<workspace::Pane>,
        workspace: View<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut WindowContext,
    ) -> Option<gpui::Task<anyhow::Result<View<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::ChannelView(state)) = state.take() else {
            unreachable!()
        };

        let open = ChannelView::open_in_pane(state.channel_id, pane, workspace, cx);

        Some(cx.spawn(|mut cx| async move {
            let this = open.await?;

            let task = this.update(&mut cx, |this, cx| {
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
        cx: &WindowContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &Model<Project>,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        self.editor.update(cx, |editor, cx| {
            editor.apply_update_proto(project, message, cx)
        })
    }

    fn set_leader_peer_id(&mut self, leader_peer_id: Option<PeerId>, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_peer_id(leader_peer_id, cx)
        })
    }

    fn is_project_item(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn to_follow_event(event: &Self::Event) -> Option<workspace::item::FollowEvent> {
        Editor::to_follow_event(event)
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
}
