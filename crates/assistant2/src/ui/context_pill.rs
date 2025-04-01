use std::rc::Rc;

use file_icons::FileIcons;
use gpui::ClickEvent;
use ui::{IconButtonShape, Tooltip, prelude::*};

use crate::context::{AssistantContext, ContextId, ContextKind};

#[derive(IntoElement)]
pub enum ContextPill {
    Added {
        context: AddedContext,
        dupe_name: bool,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    },
    Suggested {
        name: SharedString,
        icon_path: Option<SharedString>,
        kind: ContextKind,
        focused: bool,
        on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    },
}

impl ContextPill {
    pub fn added(
        context: AddedContext,
        dupe_name: bool,
        focused: bool,
        on_remove: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    ) -> Self {
        Self::Added {
            context,
            dupe_name,
            on_remove,
            focused,
            on_click: None,
        }
    }

    pub fn suggested(
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

    pub fn on_click(mut self, listener: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>) -> Self {
        match &mut self {
            ContextPill::Added { on_click, .. } => {
                *on_click = Some(listener);
            }
            ContextPill::Suggested { on_click, .. } => {
                *on_click = Some(listener);
            }
        }
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
            Self::Suggested {
                icon_path: Some(icon_path),
                ..
            }
            | Self::Added {
                context:
                    AddedContext {
                        icon_path: Some(icon_path),
                        ..
                    },
                ..
            } => Icon::from_path(icon_path),
            Self::Suggested { kind, .. }
            | Self::Added {
                context: AddedContext { kind, .. },
                ..
            } => Icon::new(kind.icon()),
        }
    }
}

impl RenderOnce for ContextPill {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.theme().colors();

        let base_pill = h_flex()
            .id(self.id())
            .pl_1()
            .pb(px(1.))
            .border_1()
            .rounded_sm()
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
                        .child(
                            div().max_w_64().child(
                                Label::new(context.name.clone())
                                    .size(LabelSize::Small)
                                    .truncate(),
                            ),
                        )
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
                        .when_some(context.tooltip.as_ref(), |element, tooltip| {
                            element.tooltip(Tooltip::text(tooltip.clone()))
                        }),
                )
                .when_some(on_remove.as_ref(), |element, on_remove| {
                    element.child(
                        IconButton::new(("remove", context.id.0), IconName::Close)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::XSmall)
                            .tooltip(Tooltip::text("Remove Context"))
                            .on_click({
                                let on_remove = on_remove.clone();
                                move |event, window, cx| on_remove(event, window, cx)
                            }),
                    )
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element
                        .cursor_pointer()
                        .on_click(move |event, window, cx| on_click(event, window, cx))
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
                .when(*focused, |this| {
                    this.bg(color.element_background.opacity(0.5))
                })
                .border_dashed()
                .border_color(if *focused {
                    color.border_focused
                } else {
                    color.border
                })
                .hover(|style| style.bg(color.element_hover.opacity(0.5)))
                .child(
                    div().px_0p5().max_w_64().child(
                        Label::new(name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
                )
                .child(
                    Label::new(match kind {
                        ContextKind::File => "Active Tab",
                        ContextKind::Thread
                        | ContextKind::Directory
                        | ContextKind::FetchedUrl
                        | ContextKind::Symbol => "Active",
                    })
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
                )
                .child(
                    Icon::new(IconName::Plus)
                        .size(IconSize::XSmall)
                        .into_any_element(),
                )
                .tooltip(|window, cx| {
                    Tooltip::with_meta("Suggested Context", None, "Click to add it", window, cx)
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, window, cx| on_click(event, window, cx))
                }),
        }
    }
}

pub struct AddedContext {
    pub id: ContextId,
    pub kind: ContextKind,
    pub name: SharedString,
    pub parent: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub icon_path: Option<SharedString>,
}

impl AddedContext {
    pub fn new(context: &AssistantContext, cx: &App) -> AddedContext {
        match context {
            AssistantContext::File(file_context) => {
                let full_path = file_context.context_buffer.file.full_path(cx);
                let full_path_string: SharedString =
                    full_path.to_string_lossy().into_owned().into();
                let name = full_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned().into())
                    .unwrap_or_else(|| full_path_string.clone());
                let parent = full_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned().into());
                AddedContext {
                    id: file_context.id,
                    kind: ContextKind::File,
                    name,
                    parent,
                    tooltip: Some(full_path_string),
                    icon_path: FileIcons::get_icon(&full_path, cx),
                }
            }

            AssistantContext::Directory(directory_context) => {
                // TODO: handle worktree disambiguation. Maybe by storing an `Arc<dyn File>` to also
                // handle renames?
                let full_path = &directory_context.project_path.path;
                let full_path_string: SharedString =
                    full_path.to_string_lossy().into_owned().into();
                let name = full_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned().into())
                    .unwrap_or_else(|| full_path_string.clone());
                let parent = full_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned().into());
                AddedContext {
                    id: directory_context.id,
                    kind: ContextKind::Directory,
                    name,
                    parent,
                    tooltip: Some(full_path_string),
                    icon_path: None,
                }
            }

            AssistantContext::Symbol(symbol_context) => AddedContext {
                id: symbol_context.id,
                kind: ContextKind::Symbol,
                name: symbol_context.context_symbol.id.name.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
            },

            AssistantContext::FetchedUrl(fetched_url_context) => AddedContext {
                id: fetched_url_context.id,
                kind: ContextKind::FetchedUrl,
                name: fetched_url_context.url.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
            },

            AssistantContext::Thread(thread_context) => AddedContext {
                id: thread_context.id,
                kind: ContextKind::Thread,
                name: thread_context.summary(cx),
                parent: None,
                tooltip: None,
                icon_path: None,
            },
        }
    }
}
