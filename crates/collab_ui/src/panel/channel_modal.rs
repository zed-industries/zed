use client::{ChannelId, ChannelStore};
use editor::Editor;
use gpui::{
    elements::*, AnyViewHandle, AppContext, Entity, ModelHandle, View, ViewContext, ViewHandle,
};
use menu::Cancel;
use workspace::{item::ItemHandle, Modal};

pub fn init(cx: &mut AppContext) {
    cx.add_action(ChannelModal::cancel)
}

pub struct ChannelModal {
    has_focus: bool,
    filter_editor: ViewHandle<Editor>,
    selection: usize,
    list_state: ListState<Self>,
    channel_store: ModelHandle<ChannelStore>,
    channel_id: ChannelId,
}

pub enum Event {
    Dismiss,
}

impl Entity for ChannelModal {
    type Event = Event;
}

impl ChannelModal {
    pub fn new(
        channel_id: ChannelId,
        channel_store: ModelHandle<ChannelStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let input_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(None, cx);
            editor.set_placeholder_text("Add a member", cx);
            editor
        });

        let list_state = ListState::<Self>::new(0, Orientation::Top, 1000., move |this, ix, cx| {
            Empty::new().into_any()
        });

        ChannelModal {
            has_focus: false,
            filter_editor: input_editor,
            selection: 0,
            list_state,
            channel_id,
            channel_store,
        }
    }

    pub fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        self.dismiss(cx);
    }

    fn dismiss(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(Event::Dismiss)
    }
}

impl View for ChannelModal {
    fn ui_name() -> &'static str {
        "Channel Modal"
    }

    fn render(&mut self, cx: &mut gpui::ViewContext<'_, '_, Self>) -> gpui::AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let style = &theme.collab_panel.modal;
        let modal_container = theme::current(cx).picker.container.clone();

        enum ChannelModal {}
        MouseEventHandler::<ChannelModal, _>::new(0, cx, |_, cx| {
            Flex::column()
                .with_child(ChildView::new(self.filter_editor.as_any(), cx))
                .with_child(
                    List::new(self.list_state.clone())
                        .constrained()
                        .with_width(style.width)
                        .flex(1., true)
                        .into_any(),
                )
                .contained()
                .with_style(modal_container)
                .constrained()
                .with_max_width(540.)
                .with_max_height(420.)
        })
        .on_click(gpui::platform::MouseButton::Left, |_, _, _| {}) // Capture click and down events
        .on_down_out(gpui::platform::MouseButton::Left, |_, v, cx| v.dismiss(cx))
        .into_any_named("channel modal")
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = true;
        if cx.is_self_focused() {
            cx.focus(&self.filter_editor);
        }
    }

    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Modal for ChannelModal {
    fn has_focus(&self) -> bool {
        self.has_focus
    }

    fn dismiss_on_event(event: &Self::Event) -> bool {
        match event {
            Event::Dismiss => true,
        }
    }
}
