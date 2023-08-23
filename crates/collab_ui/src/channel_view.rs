use channel::channel_buffer::ChannelBuffer;
use clock::ReplicaId;
use collections::HashMap;
use editor::Editor;
use gpui::{
    actions,
    elements::{ChildView, Label},
    AnyElement, AppContext, Element, Entity, ModelHandle, View, ViewContext, ViewHandle,
};
use language::Language;
use project::Project;
use std::sync::Arc;
use workspace::item::{Item, ItemHandle};

actions!(channel_view, [Deploy]);

pub(crate) fn init(cx: &mut AppContext) {
    // TODO
}

pub struct ChannelView {
    editor: ViewHandle<Editor>,
    project: ModelHandle<Project>,
    channel_buffer: ModelHandle<ChannelBuffer>,
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
        let this = Self {
            editor,
            project,
            channel_buffer,
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

    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self> {
        ChildView::new(self.editor.as_any(), cx).into_any()
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
