use std::rc::Rc;

use gpui::ClickEvent;
use ui::{prelude::*, IconButtonShape, Tooltip};

use crate::context::{ContextKind, ContextSnapshot};

#[derive(IntoElement)]
<<<<<<< HEAD
pub struct ContextPill {
    context: Context,
    on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut AppContext)>>,
=======
pub enum ContextPill {
    Added {
        context: ContextSnapshot,
        dupe_name: bool,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext)>>,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext)>>,
    },
    Suggested {
        name: SharedString,
        icon_path: Option<SharedString>,
        kind: ContextKind,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext)>>,
    },
>>>>>>> main
}

impl ContextPill {
    pub fn new_added(
        context: ContextSnapshot,
        dupe_name: bool,
        focused: bool,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext)>>,
    ) -> Self {
        Self::Added {
            context,
            dupe_name,
            on_remove,
            focused,
            on_click: None,
        }
    }

<<<<<<< HEAD
    pub fn on_remove(
        mut self,
        on_remove: Rc<dyn Fn(&ClickEvent, &mut Window, &mut AppContext)>,
    ) -> Self {
        self.on_remove = Some(on_remove);
=======
    pub fn new_suggested(
        name: SharedString,
        icon_path: Option<SharedString>,
        kind: ContextKind,
        focused: bool,
    ) -> Self {
        Self::Suggested {
            name,
            icon_path,
            kind,
            focused,
            on_click: None,
        }
    }

    pub fn on_click(mut self, listener: Rc<dyn Fn(&ClickEvent, &mut WindowContext)>) -> Self {
        match &mut self {
            ContextPill::Added { on_click, .. } => {
                *on_click = Some(listener);
            }
            ContextPill::Suggested { on_click, .. } => {
                *on_click = Some(listener);
            }
        }
>>>>>>> main
        self
    }

    pub fn id(&self) -> ElementId {
        match self {
            Self::Added { context, .. } => {
                ElementId::NamedInteger("context-pill".into(), context.id.0)
            }
            Self::Suggested { .. } => "suggested-context-pill".into(),
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::Added { context, .. } => match &context.icon_path {
                Some(icon_path) => Icon::from_path(icon_path),
                None => Icon::new(context.kind.icon()),
            },
            Self::Suggested {
                icon_path: Some(icon_path),
                ..
            } => Icon::from_path(icon_path),
            Self::Suggested {
                kind,
                icon_path: None,
                ..
            } => Icon::new(kind.icon()),
        }
    }
}

impl RenderOnce for ContextPill {
<<<<<<< HEAD
    fn render(self, _window: &mut Window, cx: &mut AppContext) -> impl IntoElement {
        let padding_right = if self.on_remove.is_some() {
            px(2.)
        } else {
            px(4.)
        };
        let icon = match self.context.kind {
            ContextKind::File => IconName::File,
            ContextKind::Directory => IconName::Folder,
            ContextKind::FetchedUrl => IconName::Globe,
            ContextKind::Thread => IconName::MessageCircle,
        };
=======
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let color = cx.theme().colors();
>>>>>>> main

        let base_pill = h_flex()
            .id(self.id())
            .pl_1()
            .pb(px(1.))
            .border_1()
            .rounded_md()
<<<<<<< HEAD
            .child(Icon::new(icon).size(IconSize::XSmall).color(Color::Muted))
            .child(Label::new(self.context.name.clone()).size(LabelSize::Small))
            .when_some(self.on_remove, |parent, on_remove| {
                parent.child(
                    IconButton::new(("remove", self.context.id.0), IconName::Close)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .on_click({
                            let on_remove = on_remove.clone();
                            move |event, window, cx| on_remove(event, window, cx)
=======
            .gap_1()
            .child(self.icon().size(IconSize::XSmall).color(Color::Muted));

        match &self {
            ContextPill::Added {
                context,
                dupe_name,
                on_remove,
                focused,
                on_click,
            } => base_pill
                .bg(color.element_background)
                .border_color(if *focused {
                    color.border_focused
                } else {
                    color.border.opacity(0.5)
                })
                .pr(if on_remove.is_some() { px(2.) } else { px(4.) })
                .child(
                    h_flex()
                        .id("context-data")
                        .gap_1()
                        .child(Label::new(context.name.clone()).size(LabelSize::Small))
                        .when_some(context.parent.as_ref(), |element, parent_name| {
                            if *dupe_name {
                                element.child(
                                    Label::new(parent_name.clone())
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                )
                            } else {
                                element
                            }
                        })
                        .when_some(context.tooltip.clone(), |element, tooltip| {
                            element.tooltip(move |cx| Tooltip::text(tooltip.clone(), cx))
>>>>>>> main
                        }),
                )
                .when_some(on_remove.as_ref(), |element, on_remove| {
                    element.child(
                        IconButton::new(("remove", context.id.0), IconName::Close)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::XSmall)
                            .tooltip(|cx| Tooltip::text("Remove Context", cx))
                            .on_click({
                                let on_remove = on_remove.clone();
                                move |event, cx| on_remove(event, cx)
                            }),
                    )
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, cx| on_click(event, cx))
                }),
            ContextPill::Suggested {
                name,
                icon_path: _,
                kind,
                focused,
                on_click,
            } => base_pill
                .cursor_pointer()
                .pr_1()
                .border_color(if *focused {
                    color.border_focused
                } else {
                    color.border_variant.opacity(0.5)
                })
                .hover(|style| style.bg(color.element_hover.opacity(0.5)))
                .child(
                    Label::new(name.clone())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .child(
                    div().px_0p5().child(
                        Label::new(match kind {
                            ContextKind::File => "Active Tab",
                            ContextKind::Thread
                            | ContextKind::Directory
                            | ContextKind::FetchedUrl => "Active",
                        })
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                    ),
                )
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::XSmall)
                        .into_any_element(),
                )
                .tooltip(|cx| Tooltip::with_meta("Suggested Context", None, "Click to add it", cx))
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, cx| on_click(event, cx))
                }),
        }
    }
}
