use crate::utils::naive_format_distance_from_now;
use crate::{
    h_stack, prelude::*, static_new_notification_items_2, v_stack, Avatar, ButtonOrIconButton,
    Icon, IconElement, Label, LineHeightStyle, ListHeaderMeta, ListSeparator, PublicPlayer,
    TextColor, UnreadIndicator,
};
use crate::{ClickHandler, ListHeader};

#[derive(Component)]
pub struct NotificationsPanel {
    id: ElementId,
}

impl NotificationsPanel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self { id: id.into() }
    }

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().surface_background)
            .child(
                ListHeader::new("Notifications").meta(Some(ListHeaderMeta::Tools(vec![
                    Icon::AtSign,
                    Icon::BellOff,
                    Icon::MailOpen,
                ]))),
            )
            .child(ListSeparator::new())
            .child(
                v_stack()
                    .id("notifications-panel-scroll-view")
                    .py_1()
                    .overflow_y_scroll()
                    .flex_1()
                    .child(
                        div()
                            .mx_2()
                            .p_1()
                            // TODO: Add cursor style
                            // .cursor(Cursor::IBeam)
                            .bg(cx.theme().colors().element_background)
                            .border()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Label::new("Search...")
                                    .color(TextColor::Placeholder)
                                    .line_height_style(LineHeightStyle::UILabel),
                            ),
                    )
                    .child(v_stack().px_1().children(static_new_notification_items_2())),
            )
    }
}

pub struct NotificationAction<V: 'static> {
    button: ButtonOrIconButton<V>,
    tooltip: SharedString,
    /// Shows after action is chosen
    ///
    /// For example, if the action is "Accept" the taken message could be:
    ///
    /// - `(None,"Accepted")` - "Accepted"
    ///
    /// - `(Some(Icon::Check),"Accepted")` - ✓ "Accepted"
    taken_message: (Option<Icon>, SharedString),
}

impl<V: 'static> NotificationAction<V> {
    pub fn new(
        button: impl Into<ButtonOrIconButton<V>>,
        tooltip: impl Into<SharedString>,
        (icon, taken_message): (Option<Icon>, impl Into<SharedString>),
    ) -> Self {
        Self {
            button: button.into(),
            tooltip: tooltip.into(),
            taken_message: (icon, taken_message.into()),
        }
    }
}

pub enum ActorOrIcon {
    Actor(PublicPlayer),
    Icon(Icon),
}

pub struct NotificationMeta<V: 'static> {
    items: Vec<(Option<Icon>, SharedString, Option<ClickHandler<V>>)>,
}

struct NotificationHandlers<V: 'static> {
    click: Option<ClickHandler<V>>,
}

impl<V: 'static> Default for NotificationHandlers<V> {
    fn default() -> Self {
        Self { click: None }
    }
}

#[derive(Component)]
pub struct Notification<V: 'static> {
    id: ElementId,
    slot: ActorOrIcon,
    message: SharedString,
    date_received: NaiveDateTime,
    meta: Option<NotificationMeta<V>>,
    actions: Option<[NotificationAction<V>; 2]>,
    unread: bool,
    new: bool,
    action_taken: Option<NotificationAction<V>>,
    handlers: NotificationHandlers<V>,
}

impl<V> Notification<V> {
    fn new(
        id: ElementId,
        message: SharedString,
        date_received: NaiveDateTime,
        slot: ActorOrIcon,
        click_action: Option<ClickHandler<V>>,
    ) -> Self {
        let handlers = if click_action.is_some() {
            NotificationHandlers {
                click: click_action,
            }
        } else {
            NotificationHandlers::default()
        };

        Self {
            id,
            date_received,
            message,
            meta: None,
            slot,
            actions: None,
            unread: true,
            new: false,
            action_taken: None,
            handlers,
        }
    }

    /// Creates a new notification with an actor slot.
    ///
    /// Requires a click action.
    pub fn new_actor_message(
        id: impl Into<ElementId>,
        message: impl Into<SharedString>,
        date_received: NaiveDateTime,
        actor: PublicPlayer,
        click_action: ClickHandler<V>,
    ) -> Self {
        Self::new(
            id.into(),
            message.into(),
            date_received,
            ActorOrIcon::Actor(actor),
            Some(click_action),
        )
    }

    /// Creates a new notification with an icon slot.
    ///
    /// Requires a click action.
    pub fn new_icon_message(
        id: impl Into<ElementId>,
        message: impl Into<SharedString>,
        date_received: NaiveDateTime,
        icon: Icon,
        click_action: ClickHandler<V>,
    ) -> Self {
        Self::new(
            id.into(),
            message.into(),
            date_received,
            ActorOrIcon::Icon(icon),
            Some(click_action),
        )
    }

    /// Creates a new notification with an actor slot
    /// and a Call To Action row.
    ///
    /// Cannot take a click action due to required actions.
    pub fn new_actor_with_actions(
        id: impl Into<ElementId>,
        message: impl Into<SharedString>,
        date_received: NaiveDateTime,
        actor: PublicPlayer,
        actions: [NotificationAction<V>; 2],
    ) -> Self {
        Self::new(
            id.into(),
            message.into(),
            date_received,
            ActorOrIcon::Actor(actor),
            None,
        )
        .actions(actions)
    }

    /// Creates a new notification with an icon slot
    /// and a Call To Action row.
    ///
    /// Cannot take a click action due to required actions.
    pub fn new_icon_with_actions(
        id: impl Into<ElementId>,
        message: impl Into<SharedString>,
        date_received: NaiveDateTime,
        icon: Icon,
        actions: [NotificationAction<V>; 2],
    ) -> Self {
        Self::new(
            id.into(),
            message.into(),
            date_received,
            ActorOrIcon::Icon(icon),
            None,
        )
        .actions(actions)
    }

    fn on_click(mut self, handler: ClickHandler<V>) -> Self {
        self.handlers.click = Some(handler);
        self
    }

    pub fn actions(mut self, actions: [NotificationAction<V>; 2]) -> Self {
        self.actions = Some(actions);
        self
    }

    pub fn meta(mut self, meta: NotificationMeta<V>) -> Self {
        self.meta = Some(meta);
        self
    }

    fn render_meta_items(&self, cx: &mut ViewContext<V>) -> impl Component<V> {
        if let Some(meta) = &self.meta {
            h_stack().children(
                meta.items
                    .iter()
                    .map(|(icon, text, _)| {
                        let mut meta_el = div();
                        if let Some(icon) = icon {
                            meta_el = meta_el.child(IconElement::new(icon.clone()));
                        }
                        meta_el.child(Label::new(text.clone()).color(TextColor::Muted))
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            div()
        }
    }

    fn render_slot(&self, cx: &mut ViewContext<V>) -> impl Component<V> {
        match &self.slot {
            ActorOrIcon::Actor(actor) => Avatar::new(actor.avatar.clone()).render(),
            ActorOrIcon::Icon(icon) => IconElement::new(icon.clone()).render(),
        }
    }

    fn render(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
        div()
            .relative()
            .id(self.id.clone())
            .p_1()
            .flex()
            .flex_col()
            .w_full()
            .children(
                Some(
                    div()
                        .absolute()
                        .left(px(3.0))
                        .top_3()
                        .z_index(2)
                        .child(UnreadIndicator::new()),
                )
                .filter(|_| self.unread),
            )
            .child(
                v_stack()
                    .z_index(1)
                    .gap_1()
                    .w_full()
                    .child(
                        h_stack()
                            .w_full()
                            .gap_2()
                            .child(self.render_slot(cx))
                            .child(div().flex_1().child(Label::new(self.message.clone()))),
                    )
                    .child(
                        h_stack()
                            .justify_between()
                            .child(
                                h_stack()
                                    .gap_1()
                                    .child(
                                        Label::new(naive_format_distance_from_now(
                                            self.date_received,
                                            true,
                                            true,
                                        ))
                                        .color(TextColor::Muted),
                                    )
                                    .child(self.render_meta_items(cx)),
                            )
                            .child(match (self.actions, self.action_taken) {
                                // Show nothing
                                (None, _) => div(),
                                // Show the taken_message
                                (Some(_), Some(action_taken)) => h_stack()
                                    .children(action_taken.taken_message.0.map(|icon| {
                                        IconElement::new(icon).color(crate::TextColor::Muted)
                                    }))
                                    .child(
                                        Label::new(action_taken.taken_message.1.clone())
                                            .color(TextColor::Muted),
                                    ),
                                // Show the actions
                                (Some(actions), None) => {
                                    h_stack().children(actions.map(|action| match action.button {
                                        ButtonOrIconButton::Button(button) => {
                                            Component::render(button)
                                        }
                                        ButtonOrIconButton::IconButton(icon_button) => {
                                            Component::render(icon_button)
                                        }
                                    }))
                                }
                            }),
                    ),
            )
    }
}

use chrono::NaiveDateTime;
use gpui::{px, Styled};
#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::{Panel, Story};
    use gpui::{Div, Render};

    pub struct NotificationsPanelStory;

    impl Render for NotificationsPanelStory {
        type Element = Div<Self>;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<_, NotificationsPanel>(cx))
                .child(Story::label(cx, "Default"))
                .child(
                    Panel::new("panel", cx).child(NotificationsPanel::new("notifications_panel")),
                )
        }
    }
}
