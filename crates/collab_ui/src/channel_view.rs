use anyhow::{anyhow, Result};
use call::report_call_event_for_channel;
use channel::{Channel, ChannelBuffer, ChannelBufferEvent, ChannelId};
use client::{
    proto::{self, PeerId},
    Collaborator, ParticipantIndex,
};
use collections::HashMap;
use editor::{CollaborationHub, Editor};
use gpui::{
    actions,
    elements::{ChildView, Label},
    geometry::vector::Vector2F,
    AnyElement, AnyViewHandle, AppContext, Element, Entity, ModelHandle, Subscription, Task, View,
    ViewContext, ViewHandle,
};
use project::Project;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};
use util::ResultExt;
use workspace::{
    item::{FollowableItem, Item, ItemHandle},
    register_followable_item,
    searchable::SearchableItemHandle,
    ItemNavHistory, Pane, ViewId, Workspace, WorkspaceId,
};

actions!(channel_view, [Deploy]);

pub fn init(cx: &mut AppContext) {
    register_followable_item::<ChannelView>(cx)
}

pub struct ChannelView {
    pub editor: ViewHandle<Editor>,
    project: ModelHandle<Project>,
    channel_buffer: ModelHandle<ChannelBuffer>,
    remote_id: Option<ViewId>,
    _editor_event_subscription: Subscription,
}

impl ChannelView {
    pub fn open(
        channel_id: ChannelId,
        workspace: ViewHandle<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
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
            });
            anyhow::Ok(channel_view)
        })
    }

    pub fn open_in_pane(
        channel_id: ChannelId,
        pane: ViewHandle<Pane>,
        workspace: ViewHandle<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();
        let channel_store = workspace.app_state().channel_store.clone();
        let markdown = workspace
            .app_state()
            .languages
            .language_for_name("Markdown");
        let channel_buffer =
            channel_store.update(cx, |store, cx| store.open_channel_buffer(channel_id, cx));

        cx.spawn(|mut cx| async move {
            let channel_buffer = channel_buffer.await?;

            if let Some(markdown) = markdown.await.log_err() {
                channel_buffer.update(&mut cx, |buffer, cx| {
                    buffer.buffer().update(cx, |buffer, cx| {
                        buffer.set_language(Some(markdown), cx);
                    })
                });
            }

            pane.update(&mut cx, |pane, cx| {
                pane.items_of_type::<Self>()
                    .find(|channel_view| channel_view.read(cx).channel_buffer == channel_buffer)
                    .unwrap_or_else(|| cx.add_view(|cx| Self::new(project, channel_buffer, cx)))
            })
            .ok_or_else(|| anyhow!("pane was dropped"))
        })
    }

    pub fn new(
        project: ModelHandle<Project>,
        channel_buffer: ModelHandle<ChannelBuffer>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, None, cx);
            editor.set_collaboration_hub(Box::new(ChannelBufferCollaborationHub(
                channel_buffer.clone(),
            )));
            editor
        });
        let _editor_event_subscription = cx.subscribe(&editor, |_, _, e, cx| cx.emit(e.clone()));

        cx.subscribe(&channel_buffer, Self::handle_channel_buffer_event)
            .detach();

        Self {
            editor,
            project,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
        }
    }

    pub fn channel(&self, cx: &AppContext) -> Arc<Channel> {
        self.channel_buffer.read(cx).channel()
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: ModelHandle<ChannelBuffer>,
        event: &ChannelBufferEvent,
        cx: &mut ViewContext<Self>,
    ) {
        if let ChannelBufferEvent::Disconnected = event {
            self.editor.update(cx, |editor, cx| {
                editor.set_read_only(true);
                cx.notify();
            })
        }
    }
}

impl Entity for ChannelView {
    type Event = editor::Event;
}

impl View for ChannelView {
    fn ui_name() -> &'static str {
        "ChannelView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        ChildView::new(self.editor.as_any(), cx).into_any()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(self.editor.as_any())
        }
    }
}

impl Item for ChannelView {
    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a ViewHandle<Self>,
        _: &'a AppContext,
    ) -> Option<&'a AnyViewHandle> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle)
        } else if type_id == TypeId::of::<Editor>() {
            Some(&self.editor)
        } else {
            None
        }
    }

    fn tab_content<V: 'static>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> AnyElement<V> {
        let channel_name = &self.channel_buffer.read(cx).channel().name;
        let label = if self.channel_buffer.read(cx).is_connected() {
            format!("#{}", channel_name)
        } else {
            format!("#{} (disconnected)", channel_name)
        };
        Label::new(label, style.label.to_owned()).into_any()
    }

    fn clone_on_split(&self, _: WorkspaceId, cx: &mut ViewContext<Self>) -> Option<Self> {
        Some(Self::new(
            self.project.clone(),
            self.channel_buffer.clone(),
            cx,
        ))
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

    fn as_searchable(&self, _: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    fn pixel_position_of_cursor(&self, cx: &AppContext) -> Option<Vector2F> {
        self.editor.read(cx).pixel_position_of_cursor(cx)
    }
}

impl FollowableItem for ChannelView {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        let channel = self.channel_buffer.read(cx).channel();
        Some(proto::view::Variant::ChannelView(
            proto::view::ChannelView {
                channel_id: channel.id,
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
        pane: ViewHandle<workspace::Pane>,
        workspace: ViewHandle<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state: &mut Option<proto::view::Variant>,
        cx: &mut AppContext,
    ) -> Option<gpui::Task<anyhow::Result<ViewHandle<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state else {
            return None;
        };
        let Some(proto::view::Variant::ChannelView(state)) = state.take() else {
            unreachable!()
        };

        let open = ChannelView::open_in_pane(state.channel_id, pane, workspace, cx);

        Some(cx.spawn(|mut cx| async move {
            let this = open.await?;

            let task = this
                .update(&mut cx, |this, cx| {
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
                })
                .ok_or_else(|| anyhow!("window was closed"))?;

            if let Some(task) = task {
                task.await?;
            }

            Ok(this)
        }))
    }

    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool {
        self.editor
            .read(cx)
            .add_event_to_update_proto(event, update, cx)
    }

    fn apply_update_proto(
        &mut self,
        project: &ModelHandle<Project>,
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

    fn should_unfollow_on_event(event: &Self::Event, cx: &AppContext) -> bool {
        Editor::should_unfollow_on_event(event, cx)
    }

    fn is_project_item(&self, _cx: &AppContext) -> bool {
        false
    }
}

struct ChannelBufferCollaborationHub(ModelHandle<ChannelBuffer>);

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
