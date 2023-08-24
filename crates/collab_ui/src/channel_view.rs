use channel::channel_buffer::ChannelBuffer;
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
    editor: ViewHandle<Editor>,
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

        let this = Self {
            editor,
            project,
            channel_buffer,
            remote_id: None,
            _editor_event_subscription,
        };
        let mapping = this.project_replica_ids_by_channel_buffer_replica_id(cx);
        this.editor
            .update(cx, |editor, cx| editor.set_replica_id_mapping(mapping, cx));

        this
    }

    /// Channel Buffer Replica ID -> Project Replica ID
    pub fn project_replica_ids_by_channel_buffer_replica_id(
        &self,
        cx: &AppContext,
    ) -> HashMap<ReplicaId, ReplicaId> {
        let project = self.project.read(cx);
        let mut result = HashMap::default();
        result.insert(
            self.channel_buffer.read(cx).replica_id(cx),
            project.replica_id(),
        );
        for collaborator in self.channel_buffer.read(cx).collaborators() {
            let project_replica_id =
                project
                    .collaborators()
                    .values()
                    .find_map(|project_collaborator| {
                        (project_collaborator.user_id == collaborator.user_id)
                            .then_some(project_collaborator.replica_id)
                    });
            if let Some(project_replica_id) = project_replica_id {
                result.insert(collaborator.replica_id as ReplicaId, project_replica_id);
            }
        }
        result
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
