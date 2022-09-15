use editor::Editor;
use gpui::{elements::*, Entity, RenderContext, View, ViewContext, ViewHandle};
use settings::Settings;

pub enum Event {
    Deactivated,
}

pub struct ContactsPopover {
    filter_editor: ViewHandle<Editor>,
}

impl Entity for ContactsPopover {
    type Event = Event;
}

impl View for ContactsPopover {
    fn ui_name() -> &'static str {
        "ContactsPopover"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &cx.global::<Settings>().theme.contacts_popover;

        Flex::row()
            .with_child(
                ChildView::new(self.filter_editor.clone())
                    .contained()
                    .with_style(
                        cx.global::<Settings>()
                            .theme
                            .contacts_panel
                            .user_query_editor
                            .container,
                    )
                    .flex(1., true)
                    .boxed(),
            )
            // .with_child(
            //     MouseEventHandler::<AddContact>::new(0, cx, |_, _| {
            //         Svg::new("icons/user_plus_16.svg")
            //             .with_color(theme.add_contact_button.color)
            //             .constrained()
            //             .with_height(16.)
            //             .contained()
            //             .with_style(theme.add_contact_button.container)
            //             .aligned()
            //             .boxed()
            //     })
            //     .with_cursor_style(CursorStyle::PointingHand)
            //     .on_click(MouseButton::Left, |_, cx| {
            //         cx.dispatch_action(contact_finder::Toggle)
            //     })
            //     .boxed(),
            // )
            .constrained()
            .with_height(
                cx.global::<Settings>()
                    .theme
                    .contacts_panel
                    .user_query_editor_height,
            )
            .aligned()
            .top()
            .contained()
            .with_background_color(theme.background)
            .with_uniform_padding(4.)
            .boxed()
    }
}

impl ContactsPopover {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        cx.observe_window_activation(Self::window_activation_changed)
            .detach();

        let filter_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(|theme| theme.contacts_panel.user_query_editor.clone()),
                cx,
            );
            editor.set_placeholder_text("Filter contacts", cx);
            editor
        });

        Self { filter_editor }
    }

    fn window_activation_changed(&mut self, is_active: bool, cx: &mut ViewContext<Self>) {
        if !is_active {
            cx.emit(Event::Deactivated);
        }
    }
}
