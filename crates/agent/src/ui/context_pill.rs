use std::{rc::Rc, time::Duration};

use file_icons::FileIcons;
use gpui::{Animation, AnimationExt as _, ClickEvent, Entity, MouseButton, pulsating_between};
use project::Project;
use prompt_store::PromptStore;
use text::OffsetRangeExt;
use ui::{IconButtonShape, Tooltip, prelude::*, tooltip_container};

use crate::context::{AgentContext, ContextKind, ImageStatus};

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
            Self::Added { context, .. } => context.context.element_id("context-pill".into()),
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
                            IconButton::new(
                                context.context.element_id("remove".into()),
                                IconName::Close,
                            )
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

// TODO: Component commented out due to new dependency on `Project`.
//
// #[derive(RegisterComponent)]
pub struct AddedContext {
    pub context: AgentContext,
    pub kind: ContextKind,
    pub name: SharedString,
    pub parent: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub icon_path: Option<SharedString>,
    pub status: ContextStatus,
    pub render_preview: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

impl AddedContext {
    /// Creates an `AddedContext` by retrieving relevant details of `AgentContext`. This returns a
    /// `None` if `DirectoryContext` or `RulesContext` no longer exist.
    ///
    /// TODO: `None` cases are unremovable from `ContextStore` and so are a very minor memory leak.
    pub fn new(
        context: AgentContext,
        prompt_store: Option<&Entity<PromptStore>>,
        project: &Project,
        cx: &App,
    ) -> Option<AddedContext> {
        match context {
            AgentContext::File(ref file_context) => {
                let full_path = file_context.buffer.read(cx).file()?.full_path(cx);
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
                Some(AddedContext {
                    kind: ContextKind::File,
                    name,
                    parent,
                    tooltip: Some(full_path_string),
                    icon_path: FileIcons::get_icon(&full_path, cx),
                    status: ContextStatus::Ready,
                    render_preview: None,
                    context,
                })
            }

            AgentContext::Directory(ref directory_context) => {
                let worktree = project
                    .worktree_for_entry(directory_context.entry_id, cx)?
                    .read(cx);
                let entry = worktree.entry_for_id(directory_context.entry_id)?;
                let full_path = worktree.full_path(&entry.path);
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
                Some(AddedContext {
                    kind: ContextKind::Directory,
                    name,
                    parent,
                    tooltip: Some(full_path_string),
                    icon_path: None,
                    status: ContextStatus::Ready,
                    render_preview: None,
                    context,
                })
            }

            AgentContext::Symbol(ref symbol_context) => Some(AddedContext {
                kind: ContextKind::Symbol,
                name: symbol_context.symbol.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: ContextStatus::Ready,
                render_preview: None,
                context,
            }),

            AgentContext::Selection(ref selection_context) => {
                let buffer = selection_context.buffer.read(cx);
                let full_path = buffer.file()?.full_path(cx);
                let mut full_path_string = full_path.to_string_lossy().into_owned();
                let mut name = full_path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| full_path_string.clone());

                let line_range = selection_context.range.to_point(&buffer.snapshot());

                let line_range_text =
                    format!(" ({}-{})", line_range.start.row + 1, line_range.end.row + 1);

                full_path_string.push_str(&line_range_text);
                name.push_str(&line_range_text);

                let parent = full_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().into_owned().into());

                Some(AddedContext {
                    kind: ContextKind::Selection,
                    name: name.into(),
                    parent,
                    tooltip: None,
                    icon_path: FileIcons::get_icon(&full_path, cx),
                    status: ContextStatus::Ready,
                    render_preview: None,
                    /*
                    render_preview: Some(Rc::new({
                        let content = selection_context.text.clone();
                        move |_, cx| {
                            div()
                                .id("context-pill-selection-preview")
                                .overflow_scroll()
                                .max_w_128()
                                .max_h_96()
                                .child(Label::new(content.clone()).buffer_font(cx))
                                .into_any_element()
                        }
                    })),
                    */
                    context,
                })
            }

            AgentContext::FetchedUrl(ref fetched_url_context) => Some(AddedContext {
                kind: ContextKind::FetchedUrl,
                name: fetched_url_context.url.clone(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: ContextStatus::Ready,
                render_preview: None,
                context,
            }),

            AgentContext::Thread(ref thread_context) => Some(AddedContext {
                kind: ContextKind::Thread,
                name: thread_context.name(cx),
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
                context,
            }),

            AgentContext::Rules(ref user_rules_context) => {
                let name = prompt_store
                    .as_ref()?
                    .read(cx)
                    .metadata(user_rules_context.prompt_id.into())?
                    .title?;
                Some(AddedContext {
                    kind: ContextKind::Rules,
                    name: name.clone(),
                    parent: None,
                    tooltip: None,
                    icon_path: None,
                    status: ContextStatus::Ready,
                    render_preview: None,
                    context,
                })
            }

            AgentContext::Image(ref image_context) => Some(AddedContext {
                kind: ContextKind::Image,
                name: "Image".into(),
                parent: None,
                tooltip: None,
                icon_path: None,
                status: match image_context.status() {
                    ImageStatus::Loading => ContextStatus::Loading {
                        message: "Loading…".into(),
                    },
                    ImageStatus::Error => ContextStatus::Error {
                        message: "Failed to load image".into(),
                    },
                    ImageStatus::Ready => ContextStatus::Ready,
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
                context,
            }),
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

// TODO: Component commented out due to new dependency on `Project`.
/*
impl Component for AddedContext {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn sort_name() -> &'static str {
        "AddedContext"
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        let next_context_id = ContextId::zero();
        let image_ready = (
            "Ready",
            AddedContext::new(
                AgentContext::Image(ImageContext {
                    context_id: next_context_id.post_inc(),
                    original_image: Arc::new(Image::empty()),
                    image_task: Task::ready(Some(LanguageModelImage::empty())).shared(),
                }),
                cx,
            ),
        );

        let image_loading = (
            "Loading",
            AddedContext::new(
                AgentContext::Image(ImageContext {
                    context_id: next_context_id.post_inc(),
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
                AgentContext::Image(ImageContext {
                    context_id: next_context_id.post_inc(),
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

        None
    }
}
*/
