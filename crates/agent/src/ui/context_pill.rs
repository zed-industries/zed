use std::sync::Arc;
use std::{rc::Rc, time::Duration};

use file_icons::FileIcons;
use futures::FutureExt;
use gpui::{Animation, AnimationExt as _, Image, MouseButton, pulsating_between};
use gpui::{ClickEvent, Task};
use language_model::LanguageModelImage;
use ui::{IconButtonShape, Tooltip, prelude::*, tooltip_container};

use crate::context::{AssistantContext, ContextId, ContextKind, ImageContext};

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
            } => {
                let status_is_error = matches!(context.status, ContextStatus::Error { .. });

                base_pill
                    .pr(if on_remove.is_some() { px(2.) } else { px(4.) })
                    .map(|pill| {
                        if status_is_error {
                            pill.bg(cx.theme().status().error_background)
                                .border_color(cx.theme().status().error_border)
                        } else if *focused {
                            pill.bg(color.element_background)
                                .border_color(color.border_focused)
                        } else {
                            pill.bg(color.element_background)
                                .border_color(color.border.opacity(0.5))
                        }
                    })
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
                            })
                            .map(|element| match &context.status {
                                ContextStatus::Ready => element
                                    .when_some(
                                        context.render_preview.as_ref(),
                                        |element, render_preview| {
                                            element.hoverable_tooltip({
                                                let render_preview = render_preview.clone();
                                                move |_, cx| {
                                                    cx.new(|_| ContextPillPreview {
                                                        render_preview: render_preview.clone(),
                                                    })
                                                    .into()
                                                }
                                            })
                                        },
                                    )
                                    .into_any(),
                                ContextStatus::Loading { message } => element
                                    .tooltip(ui::Tooltip::text(message.clone()))
                                    .with_animation(
                                        "pulsating-ctx-pill",
                                        Animation::new(Duration::from_secs(2))
                                            .repeat()
                                            .with_easing(pulsating_between(0.4, 0.8)),
                                        |label, delta| label.opacity(delta),
                                    )
                                    .into_any_element(),
                                ContextStatus::Error { message } => element
                                    .tooltip(ui::Tooltip::text(message.clone()))
                                    .into_any_element(),
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
                    })
                    .into_any_element()
            }
            ContextPill::Suggested {
                name,
                icon_path: _,
                kind: _,
                focused,
                on_click,
            } => base_pill
                .cursor_pointer()
                .pr_1()
                .border_dashed()
                .map(|pill| {
                    if *focused {
                        pill.border_color(color.border_focused)
                            .bg(color.element_background.opacity(0.5))
                    } else {
                        pill.border_color(color.border)
                    }
                })
                .hover(|style| style.bg(color.element_hover.opacity(0.5)))
                .child(
                    div().max_w_64().child(
                        Label::new(name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
                )
                .tooltip(|window, cx| {
                    Tooltip::with_meta("Suggested Context", None, "Click to add it", window, cx)
                })
                .when_some(on_click.as_ref(), |element, on_click| {
                    let on_click = on_click.clone();
                    element.on_click(move |event, window, cx| on_click(event, window, cx))
                })
                .into_any(),
        }
    }
}

pub enum ContextStatus {
    Ready,
    Loading { message: SharedString },
    Error { message: SharedString },
}

#[derive(RegisterComponent)]
pub struct AddedContext {
    pub id: ContextId,
    pub kind: ContextKind,
    pub name: SharedString,
    pub parent: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub icon_path: Option<SharedString>,
    pub status: ContextStatus,
    pub render_preview: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

impl AddedContext {
    pub fn new(context: &AssistantContext, cx: &App) -> AddedContext {
        match context {
            AssistantContext::File(file_context) => {
                let full_path = file_context.context_buffer.full_path(cx);
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
                    status: ContextStatus::Ready,
                    render_preview: None,
                }
            }

            AssistantContext::Directory(directory_context) => {
                let worktree = directory_context.worktree.read(cx);
                // If the directory no longer exists, use its last known path.
                let full_path = worktree
                    .entry_for_id(directory_context.entry_id)
                    .map_or_else(
                        || directory_context.last_path.clone(),
                        |entry| worktree.full_path(&entry.path).into(),
                    );
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
                    status: ContextStatus::Ready,
                    render_preview: None,
                }
            }

            AssistantContext::Symbol(symbol_context) => AddedContext {
                id: symbol_context.id,
                kind: ContextKind::Symbol,
                name: symbol_context.context_symbol.id.name.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: ContextStatus::Ready,
                render_preview: None,
            },

            AssistantContext::Excerpt(excerpt_context) => {
                let full_path = excerpt_context.context_buffer.full_path(cx);
                let mut full_path_string = full_path.to_string_lossy().into_owned();
                let mut name = full_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| full_path_string.clone());

                let line_range_text = format!(
                    " ({}-{})",
                    excerpt_context.line_range.start.row + 1,
                    excerpt_context.line_range.end.row + 1
                );

                full_path_string.push_str(&line_range_text);
                name.push_str(&line_range_text);

                let parent = full_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned().into());

                AddedContext {
                    id: excerpt_context.id,
                    kind: ContextKind::Excerpt,
                    name: name.into(),
                    parent,
                    tooltip: Some(full_path_string.into()),
                    icon_path: FileIcons::get_icon(&full_path, cx),
                    status: ContextStatus::Ready,
                    render_preview: None,
                }
            }

            AssistantContext::FetchedUrl(fetched_url_context) => AddedContext {
                id: fetched_url_context.id,
                kind: ContextKind::FetchedUrl,
                name: fetched_url_context.url.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: ContextStatus::Ready,
                render_preview: None,
            },

            AssistantContext::Thread(thread_context) => AddedContext {
                id: thread_context.id,
                kind: ContextKind::Thread,
                name: thread_context.summary(cx),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: if thread_context
                    .thread
                    .read(cx)
                    .is_generating_detailed_summary()
                {
                    ContextStatus::Loading {
                        message: "Summarizing…".into(),
                    }
                } else {
                    ContextStatus::Ready
                },
                render_preview: None,
            },

            AssistantContext::Rules(user_rules_context) => AddedContext {
                id: user_rules_context.id,
                kind: ContextKind::Rules,
                name: user_rules_context.title.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: ContextStatus::Ready,
                render_preview: None,
            },

            AssistantContext::Image(image_context) => AddedContext {
                id: image_context.id,
                kind: ContextKind::Image,
                name: "Image".into(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: if image_context.is_loading() {
                    ContextStatus::Loading {
                        message: "Loading…".into(),
                    }
                } else if image_context.is_error() {
                    ContextStatus::Error {
                        message: "Failed to load image".into(),
                    }
                } else {
                    ContextStatus::Ready
                },
                render_preview: Some(Rc::new({
                    let image = image_context.original_image.clone();
                    move |_, _| {
                        gpui::img(image.clone())
                            .max_w_96()
                            .max_h_96()
                            .into_any_element()
                    }
                })),
            },
        }
    }
}

struct ContextPillPreview {
    render_preview: Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>,
}

impl Render for ContextPillPreview {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, move |this, window, cx| {
            this.occlude()
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child((self.render_preview)(window, cx))
        })
    }
}

impl Component for AddedContext {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AddedContext"
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let image_ready = (
            "Ready",
            AddedContext::new(
                &AssistantContext::Image(ImageContext {
                    id: ContextId(0),
                    original_image: Arc::new(Image::empty()),
                    image_task: Task::ready(Some(LanguageModelImage::empty())).shared(),
                }),
                cx,
            ),
        );

        let image_loading = (
            "Loading",
            AddedContext::new(
                &AssistantContext::Image(ImageContext {
                    id: ContextId(1),
                    original_image: Arc::new(Image::empty()),
                    image_task: cx
                        .background_spawn(async move {
                            smol::Timer::after(Duration::from_secs(60 * 5)).await;
                            Some(LanguageModelImage::empty())
                        })
                        .shared(),
                }),
                cx,
            ),
        );

        let image_error = (
            "Error",
            AddedContext::new(
                &AssistantContext::Image(ImageContext {
                    id: ContextId(2),
                    original_image: Arc::new(Image::empty()),
                    image_task: Task::ready(None).shared(),
                }),
                cx,
            ),
        );

        Some(
            v_flex()
                .gap_6()
                .children(
                    vec![image_ready, image_loading, image_error]
                        .into_iter()
                        .map(|(text, context)| {
                            single_example(
                                text,
                                ContextPill::added(context, false, false, None).into_any_element(),
                            )
                        }),
                )
                .into_any(),
        )
    }
}
