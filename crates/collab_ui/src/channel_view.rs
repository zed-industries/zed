use channel::channel_buffer::ChannelBuffer;
use editor::Editor;
use gpui::{
    actions,
    elements::{ChildView, Label},
    AnyElement, AppContext, Element, Entity, ModelHandle, View, ViewContext, ViewHandle,
};
use language::Language;
use std::sync::Arc;
use workspace::item::{Item, ItemHandle};

actions!(channel_view, [Deploy]);

pub(crate) fn init(cx: &mut AppContext) {
    // TODO
}

pub struct ChannelView {
    editor: ViewHandle<Editor>,
    channel_buffer: ModelHandle<ChannelBuffer>,
}

impl ChannelView {
    pub fn new(
        channel_buffer: ModelHandle<ChannelBuffer>,
        language: Arc<Language>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let buffer = channel_buffer.read(cx).buffer();
        buffer.update(cx, |buffer, cx| buffer.set_language(Some(language), cx));
        let editor = cx.add_view(|cx| Editor::for_buffer(buffer, None, cx));
        Self {
            editor,
            channel_buffer,
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
