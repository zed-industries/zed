use gpui::{Action, FocusHandle, prelude::*};
use ui::{ElevationIndex, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*};

#[derive(IntoElement)]
pub struct ModelSelectorHeader {
    title: SharedString,
    has_border: bool,
}

impl ModelSelectorHeader {
    pub fn new(title: impl Into<SharedString>, has_border: bool) -> Self {
        Self {
            title: title.into(),
            has_border,
        }
    }
}

impl RenderOnce for ModelSelectorHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .px_2()
            .pb_1()
            .when(self.has_border, |this| {
                this.mt_1()
                    .pt_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
            })
            .child(
                Label::new(self.title)
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
    }
}

#[derive(Copy, Clone)]
pub enum ModelSelectorFavoriteAction {
    Favorite,
    Unfavorite,
}

impl ModelSelectorFavoriteAction {
    pub fn from_is_favorite(is_favorite: bool) -> Self {
        if is_favorite {
            Self::Unfavorite
        } else {
            Self::Favorite
        }
    }

    pub fn icon_name(&self) -> IconName {
        match self {
            Self::Favorite => IconName::Star,
            Self::Unfavorite => IconName::StarFilled,
        }
    }

    pub fn icon_color(&self) -> Color {
        match self {
            Self::Favorite => Color::Default,
            Self::Unfavorite => Color::Accent,
        }
    }

    pub fn tooltip(&self) -> SharedString {
        match self {
            Self::Favorite => "Favorite Model".into(),
            Self::Unfavorite => "Unfavorite Model".into(),
        }
    }
}

#[derive(IntoElement)]
pub struct ModelSelectorListItem {
    index: usize,
    title: SharedString,
    icon: Option<IconName>,
    is_selected: bool,
    is_focused: bool,
    favorite_action: Option<ModelSelectorFavoriteAction>,
    on_favorite_action_click: Option<Box<dyn Fn(&App) + 'static>>,
}

impl ModelSelectorListItem {
    pub fn new(index: usize, title: impl Into<SharedString>) -> Self {
        Self {
            index,
            title: title.into(),
            icon: None,
            is_selected: false,
            is_focused: false,
            favorite_action: None,
            on_favorite_action_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn is_selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;
        self
    }

    pub fn is_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }

    pub fn favorite_action(mut self, action: ModelSelectorFavoriteAction) -> Self {
        self.favorite_action = Some(action);
        self
    }

    pub fn on_favorite_action_click(mut self, handler: impl Fn(&App) + 'static) -> Self {
        self.on_favorite_action_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ModelSelectorListItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let model_icon_color = if self.is_selected {
            Color::Accent
        } else {
            Color::Muted
        };

        let favorite_action = self.favorite_action;
        let on_favorite_action_click = self.on_favorite_action_click;

        ListItem::new(self.index)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(self.is_focused)
            .child(
                h_flex()
                    .w_full()
                    .gap_1p5()
                    .when_some(self.icon, |this, icon| {
                        this.child(
                            Icon::new(icon)
                                .color(model_icon_color)
                                .size(IconSize::Small),
                        )
                    })
                    .child(Label::new(self.title).truncate()),
            )
            .end_slot(div().pr_2().when(self.is_selected, |this| {
                this.child(
                    Icon::new(IconName::Check)
                        .color(Color::Accent)
                        .size(IconSize::Small),
                )
            }))
            .end_hover_slot(div().pr_2().when_some(
                favorite_action.zip(on_favorite_action_click),
                |this, (action, handle_action_click)| {
                    this.child(
                        IconButton::new(("toggle-favorite", self.index), action.icon_name())
                            .layer(ElevationIndex::ElevatedSurface)
                            .icon_color(action.icon_color())
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text(action.tooltip()))
                            .on_click(move |_, _, cx| (handle_action_click)(cx)),
                    )
                },
            ))
    }
}

#[derive(IntoElement)]
pub struct ModelSelectorFooter {
    action: Box<dyn Action>,
    focus_handle: FocusHandle,
}

impl ModelSelectorFooter {
    pub fn new(action: Box<dyn Action>, focus_handle: FocusHandle) -> Self {
        Self {
            action,
            focus_handle,
        }
    }
}

impl RenderOnce for ModelSelectorFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let action = self.action;
        let focus_handle = self.focus_handle;

        h_flex()
            .w_full()
            .p_1p5()
            .border_t_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Button::new("configure", "Configure")
                    .full_width()
                    .style(ButtonStyle::Outlined)
                    .key_binding(
                        KeyBinding::for_action_in(action.as_ref(), &focus_handle, cx)
                            .map(|kb| kb.size(rems_from_px(12.))),
                    )
                    .on_click(move |_, window, cx| {
                        window.dispatch_action(action.boxed_clone(), cx);
                    }),
            )
    }
}
