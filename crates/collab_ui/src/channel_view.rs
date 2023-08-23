use channel::channel_buffer::{self, ChannelBuffer};
use client::proto;
use clock::ReplicaId;
use collections::HashMap;
use editor::Editor;
use gpui::{
    actions,
    elements::{ChildView, Label},
    AnyElement, AnyViewHandle, AppContext, Element, Entity, ModelHandle, Subscription, View,
    ViewContext, ViewHandle,
};
use language::Language;
use project::Project;
use std::sync::Arc;
use workspace::{
    item::{FollowableItem, Item, ItemHandle},
    register_followable_item, ViewId,
};

actions!(channel_view, [Deploy]);

pub(crate) fn init(cx: &mut AppContext) {
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
    pub fn new(
        project: ModelHandle<Project>,
        channel_buffer: ModelHandle<ChannelBuffer>,
        language: Option<Arc<Language>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        buffer.update(cx, |buffer, cx| buffer.set_language(language, cx));
        let editor = cx.add_view(|cx| Editor::for_buffer(buffer, None, cx));
        let _editor_event_subscription = cx.subscribe(&editor, |_, _, e, cx| cx.emit(e.clone()));

        cx.subscribe(&project, Self::handle_project_event).detach();
        cx.subscribe(&channel_buffer, Self::handle_channel_buffer_event)
            .detach();

        let this = Self {
            editor,
            project,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
        };
        this.refresh_replica_id_map(cx);
        this
    }

    fn handle_project_event(
        &mut self,
        _: ModelHandle<Project>,
        event: &project::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            project::Event::RemoteIdChanged(_) => {}
            project::Event::DisconnectedFromHost => {}
            project::Event::Closed => {}
            project::Event::CollaboratorUpdated { .. } => {}
            project::Event::CollaboratorLeft(_) => {}
            project::Event::CollaboratorJoined(_) => {}
            _ => return,
        }
        self.refresh_replica_id_map(cx);
    }

    fn handle_channel_buffer_event(
        &mut self,
        _: ModelHandle<ChannelBuffer>,
        _: &channel_buffer::Event,
        cx: &mut ViewContext<Self>,
    ) {
        self.refresh_replica_id_map(cx);
    }

    /// Build a mapping of channel buffer replica ids to the corresponding
    /// replica ids in the current project.
    ///
    /// Using this mapping, a given user can be displayed with the same color
    /// in the channel buffer as in other files in the project. Users who are
    /// in the channel buffer but not the project will not have a color.
    fn refresh_replica_id_map(&self, cx: &mut ViewContext<Self>) {
        let mut project_replica_ids_by_channel_buffer_replica_id = HashMap::default();
        let project = self.project.read(cx);
        let channel_buffer = self.channel_buffer.read(cx);
        project_replica_ids_by_channel_buffer_replica_id
            .insert(channel_buffer.replica_id(cx), project.replica_id());
        project_replica_ids_by_channel_buffer_replica_id.extend(
            channel_buffer
                .collaborators()
                .iter()
                .filter_map(|channel_buffer_collaborator| {
                    project
                        .collaborators()
                        .values()
                        .find_map(|project_collaborator| {
                            (project_collaborator.user_id == channel_buffer_collaborator.user_id)
                                .then_some((
                                    channel_buffer_collaborator.replica_id as ReplicaId,
                                    project_collaborator.replica_id,
                                ))
                        })
                }),
        );

        self.editor.update(cx, |editor, cx| {
            editor.set_replica_id_map(Some(project_replica_ids_by_channel_buffer_replica_id), cx)
        });
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
    fn tab_content<V: 'static>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> AnyElement<V> {
        let channel_name = self
            .channel_buffer
            .read(cx)
            .channel(cx)
            .map_or("[Deleted channel]".to_string(), |channel| {
                format!("#{}", channel.name)
            });
        Label::new(channel_name, style.label.to_owned()).into_any()
    }
}

impl FollowableItem for ChannelView {
    fn remote_id(&self) -> Option<workspace::ViewId> {
        self.remote_id
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        self.channel_buffer.read(cx).channel(cx).map(|channel| {
            proto::view::Variant::ChannelView(proto::view::ChannelView {
                channel_id: channel.id,
            })
        })
    }

    fn from_state_proto(
        _: ViewHandle<workspace::Pane>,
        workspace: ViewHandle<workspace::Workspace>,
        remote_id: workspace::ViewId,
        state_proto: &mut Option<proto::view::Variant>,
        cx: &mut AppContext,
    ) -> Option<gpui::Task<anyhow::Result<ViewHandle<Self>>>> {
        let Some(proto::view::Variant::ChannelView(_)) = state_proto else { return None };
        let Some(proto::view::Variant::ChannelView(state)) = state_proto.take() else { unreachable!() };

        let channel_store = &workspace.read(cx).app_state().channel_store.clone();
        let open_channel_buffer = channel_store.update(cx, |store, cx| {
            store.open_channel_buffer(state.channel_id, cx)
        });
        let project = workspace.read(cx).project().to_owned();
        let language = workspace.read(cx).app_state().languages.clone();
        let get_markdown = language.language_for_name("Markdown");

        Some(cx.spawn(|mut cx| async move {
            let channel_buffer = open_channel_buffer.await?;
            let markdown = get_markdown.await?;

            let this = workspace
                .update(&mut cx, move |_, cx| {
                    cx.add_view(|cx| {
                        let mut this = Self::new(project, channel_buffer, Some(markdown), cx);
                        this.remote_id = Some(remote_id);
                        this
                    })
                })
                .ok_or_else(|| anyhow::anyhow!("workspace droppped"))?;

            Ok(this)
        }))
    }

    fn add_event_to_update_proto(
        &self,
        _: &Self::Event,
        _: &mut Option<proto::update_view::Variant>,
        _: &AppContext,
    ) -> bool {
        false
    }

    fn apply_update_proto(
        &mut self,
        _: &ModelHandle<Project>,
        _: proto::update_view::Variant,
        _: &mut ViewContext<Self>,
    ) -> gpui::Task<anyhow::Result<()>> {
        gpui::Task::ready(Ok(()))
    }

    fn set_leader_replica_id(
        &mut self,
        leader_replica_id: Option<u16>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.set_leader_replica_id(leader_replica_id, cx)
        })
    }

    fn should_unfollow_on_event(event: &Self::Event, cx: &AppContext) -> bool {
        Editor::should_unfollow_on_event(event, cx)
    }
}
