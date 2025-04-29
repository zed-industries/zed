use std::{ops::Range, path::Path, rc::Rc, sync::Arc, time::Duration};

use file_icons::FileIcons;
use futures::FutureExt as _;
use gpui::{
    Animation, AnimationExt as _, AnyView, ClickEvent, Entity, Image, MouseButton, Task,
    pulsating_between,
};
use language_model::LanguageModelImage;
use project::Project;
use prompt_store::PromptStore;
use rope::Point;
use ui::{IconButtonShape, Tooltip, prelude::*, tooltip_container};

use crate::context::{
    AgentContext, AgentContextHandle, ContextId, ContextKind, DirectoryContext,
    DirectoryContextHandle, FetchedUrlContext, FileContext, FileContextHandle, ImageContext,
    ImageStatus, RulesContext, RulesContextHandle, SelectionContext, SelectionContextHandle,
    SymbolContext, SymbolContextHandle, ThreadContext, ThreadContextHandle,
};

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
            Self::Added { context, .. } => context.handle.element_id("context-pill".into()),
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
                                        context.render_hover.as_ref(),
                                        |element, render_hover| {
                                            let render_hover = render_hover.clone();
                                            element.hoverable_tooltip(move |window, cx| {
                                                render_hover(window, cx)
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
                                context.handle.element_id("remove".into()),
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

#[derive(RegisterComponent)]
pub struct AddedContext {
    pub handle: AgentContextHandle,
    pub kind: ContextKind,
    pub name: SharedString,
    pub parent: Option<SharedString>,
    pub tooltip: Option<SharedString>,
    pub icon_path: Option<SharedString>,
    pub status: ContextStatus,
    pub render_hover: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
}

impl AddedContext {
    /// Creates an `AddedContext` by retrieving relevant details of `AgentContext`. This returns a
    /// `None` if `DirectoryContext` or `RulesContext` no longer exist.
    ///
    /// TODO: `None` cases are unremovable from `ContextStore` and so are a very minor memory leak.
    pub fn new_pending(
        handle: AgentContextHandle,
        prompt_store: Option<&Entity<PromptStore>>,
        project: &Project,
        cx: &App,
    ) -> Option<AddedContext> {
        match handle {
            AgentContextHandle::File(handle) => Self::pending_file(handle, cx),
            AgentContextHandle::Directory(handle) => Self::pending_directory(handle, project, cx),
            AgentContextHandle::Symbol(handle) => Self::pending_symbol(handle, cx),
            AgentContextHandle::Selection(handle) => Self::pending_selection(handle, cx),
            AgentContextHandle::FetchedUrl(handle) => Some(Self::fetched_url(handle)),
            AgentContextHandle::Thread(handle) => Some(Self::pending_thread(handle, cx)),
            AgentContextHandle::Rules(handle) => Self::pending_rules(handle, prompt_store, cx),
            AgentContextHandle::Image(handle) => Some(Self::image(handle)),
        }
    }

    pub fn new_attached(context: &AgentContext, cx: &App) -> AddedContext {
        match context {
            AgentContext::File(context) => Self::attached_file(context, cx),
            AgentContext::Directory(context) => Self::attached_directory(context),
            AgentContext::Symbol(context) => Self::attached_symbol(context, cx),
            AgentContext::Selection(context) => Self::attached_selection(context, cx),
            AgentContext::FetchedUrl(context) => Self::fetched_url(context.clone()),
            AgentContext::Thread(context) => Self::attached_thread(context),
            AgentContext::Rules(context) => Self::attached_rules(context),
            AgentContext::Image(context) => Self::image(context.clone()),
        }
    }

    fn pending_file(handle: FileContextHandle, cx: &App) -> Option<AddedContext> {
        let full_path = handle.buffer.read(cx).file()?.full_path(cx);
        Some(Self::file(handle, &full_path, cx))
    }

    fn attached_file(context: &FileContext, cx: &App) -> AddedContext {
        Self::file(context.handle.clone(), &context.full_path, cx)
    }

    fn file(handle: FileContextHandle, full_path: &Path, cx: &App) -> AddedContext {
        let full_path_string: SharedString = full_path.to_string_lossy().into_owned().into();
        let name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned().into())
            .unwrap_or_else(|| full_path_string.clone());
        let parent = full_path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned().into());
        AddedContext {
            kind: ContextKind::File,
            name,
            parent,
            tooltip: Some(full_path_string),
            icon_path: FileIcons::get_icon(&full_path, cx),
            status: ContextStatus::Ready,
            render_hover: None,
            handle: AgentContextHandle::File(handle),
        }
    }

    fn pending_directory(
        handle: DirectoryContextHandle,
        project: &Project,
        cx: &App,
    ) -> Option<AddedContext> {
        let worktree = project.worktree_for_entry(handle.entry_id, cx)?.read(cx);
        let entry = worktree.entry_for_id(handle.entry_id)?;
        let full_path = worktree.full_path(&entry.path);
        Some(Self::directory(handle, &full_path))
    }

    fn attached_directory(context: &DirectoryContext) -> AddedContext {
        Self::directory(context.handle.clone(), &context.full_path)
    }

    fn directory(handle: DirectoryContextHandle, full_path: &Path) -> AddedContext {
        let full_path_string: SharedString = full_path.to_string_lossy().into_owned().into();
        let name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned().into())
            .unwrap_or_else(|| full_path_string.clone());
        let parent = full_path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned().into());
        AddedContext {
            kind: ContextKind::Directory,
            name,
            parent,
            tooltip: Some(full_path_string),
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: None,
            handle: AgentContextHandle::Directory(handle),
        }
    }

    fn pending_symbol(handle: SymbolContextHandle, cx: &App) -> Option<AddedContext> {
        let excerpt =
            ContextFileExcerpt::new(&handle.full_path(cx)?, handle.enclosing_line_range(cx), cx);
        Some(AddedContext {
            kind: ContextKind::Symbol,
            name: handle.symbol.clone(),
            parent: Some(excerpt.file_name_and_range.clone()),
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: {
                let handle = handle.clone();
                Some(Rc::new(move |_, cx| {
                    excerpt.hover_view(handle.text(cx), cx).into()
                }))
            },
            handle: AgentContextHandle::Symbol(handle),
        })
    }

    fn attached_symbol(context: &SymbolContext, cx: &App) -> AddedContext {
        let excerpt = ContextFileExcerpt::new(&context.full_path, context.line_range.clone(), cx);
        AddedContext {
            kind: ContextKind::Symbol,
            name: context.handle.symbol.clone(),
            parent: Some(excerpt.file_name_and_range.clone()),
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: {
                let text = context.text.clone();
                Some(Rc::new(move |_, cx| {
                    excerpt.hover_view(text.clone(), cx).into()
                }))
            },
            handle: AgentContextHandle::Symbol(context.handle.clone()),
        }
    }

    fn pending_selection(handle: SelectionContextHandle, cx: &App) -> Option<AddedContext> {
        let excerpt = ContextFileExcerpt::new(&handle.full_path(cx)?, handle.line_range(cx), cx);
        Some(AddedContext {
            kind: ContextKind::Selection,
            name: excerpt.file_name_and_range.clone(),
            parent: excerpt.parent_name.clone(),
            tooltip: None,
            icon_path: excerpt.icon_path.clone(),
            status: ContextStatus::Ready,
            render_hover: {
                let handle = handle.clone();
                Some(Rc::new(move |_, cx| {
                    excerpt.hover_view(handle.text(cx), cx).into()
                }))
            },
            handle: AgentContextHandle::Selection(handle),
        })
    }

    fn attached_selection(context: &SelectionContext, cx: &App) -> AddedContext {
        let excerpt = ContextFileExcerpt::new(&context.full_path, context.line_range.clone(), cx);
        AddedContext {
            kind: ContextKind::Selection,
            name: excerpt.file_name_and_range.clone(),
            parent: excerpt.parent_name.clone(),
            tooltip: None,
            icon_path: excerpt.icon_path.clone(),
            status: ContextStatus::Ready,
            render_hover: {
                let text = context.text.clone();
                Some(Rc::new(move |_, cx| {
                    excerpt.hover_view(text.clone(), cx).into()
                }))
            },
            handle: AgentContextHandle::Selection(context.handle.clone()),
        }
    }

    fn fetched_url(context: FetchedUrlContext) -> AddedContext {
        AddedContext {
            kind: ContextKind::FetchedUrl,
            name: context.url.clone(),
            parent: None,
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: None,
            handle: AgentContextHandle::FetchedUrl(context),
        }
    }

    fn pending_thread(handle: ThreadContextHandle, cx: &App) -> AddedContext {
        AddedContext {
            kind: ContextKind::Thread,
            name: handle.title(cx),
            parent: None,
            tooltip: None,
            icon_path: None,
            status: if handle.thread.read(cx).is_generating_detailed_summary() {
                ContextStatus::Loading {
                    message: "Summarizing…".into(),
                }
            } else {
                ContextStatus::Ready
            },
            render_hover: {
                let thread = handle.thread.clone();
                Some(Rc::new(move |_, cx| {
                    let text = thread.read(cx).latest_detailed_summary_or_text();
                    text_hover_view(text.clone(), cx).into()
                }))
            },
            handle: AgentContextHandle::Thread(handle),
        }
    }

    fn attached_thread(context: &ThreadContext) -> AddedContext {
        AddedContext {
            kind: ContextKind::Thread,
            name: context.title.clone(),
            parent: None,
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: {
                let text = context.text.clone();
                Some(Rc::new(move |_, cx| {
                    text_hover_view(text.clone(), cx).into()
                }))
            },
            handle: AgentContextHandle::Thread(context.handle.clone()),
        }
    }

    fn pending_rules(
        handle: RulesContextHandle,
        prompt_store: Option<&Entity<PromptStore>>,
        cx: &App,
    ) -> Option<AddedContext> {
        let title = prompt_store
            .as_ref()?
            .read(cx)
            .metadata(handle.prompt_id.into())?
            .title
            .unwrap_or_else(|| "Unnamed Rule".into());
        Some(AddedContext {
            kind: ContextKind::Rules,
            name: title.clone(),
            parent: None,
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: None,
            handle: AgentContextHandle::Rules(handle),
        })
    }

    fn attached_rules(context: &RulesContext) -> AddedContext {
        let title = context
            .title
            .clone()
            .unwrap_or_else(|| "Unnamed Rule".into());
        AddedContext {
            kind: ContextKind::Rules,
            name: title,
            parent: None,
            tooltip: None,
            icon_path: None,
            status: ContextStatus::Ready,
            render_hover: {
                let text = context.text.clone();
                Some(Rc::new(move |_, cx| {
                    text_hover_view(text.clone(), cx).into()
                }))
            },
            handle: AgentContextHandle::Rules(context.handle.clone()),
        }
    }

    fn image(context: ImageContext) -> AddedContext {
        AddedContext {
            kind: ContextKind::Image,
            name: "Image".into(),
            parent: None,
            tooltip: None,
            icon_path: None,
            status: match context.status() {
                ImageStatus::Loading => ContextStatus::Loading {
                    message: "Loading…".into(),
                },
                ImageStatus::Error => ContextStatus::Error {
                    message: "Failed to load image".into(),
                },
                ImageStatus::Ready => ContextStatus::Ready,
            },
            render_hover: Some(Rc::new({
                let image = context.original_image.clone();
                move |_, cx| {
                    let image = image.clone();
                    ContextPillHover::new(cx, move |_, _| {
                        gpui::img(image.clone())
                            .max_w_96()
                            .max_h_96()
                            .into_any_element()
                    })
                    .into()
                }
            })),
            handle: AgentContextHandle::Image(context),
        }
    }
}

#[derive(Debug, Clone)]
struct ContextFileExcerpt {
    pub file_name_and_range: SharedString,
    pub full_path_and_range: SharedString,
    pub parent_name: Option<SharedString>,
    pub icon_path: Option<SharedString>,
}

impl ContextFileExcerpt {
    pub fn new(full_path: &Path, line_range: Range<Point>, cx: &App) -> Self {
        let full_path_string = full_path.to_string_lossy().into_owned();
        let file_name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| full_path_string.clone());

        let line_range_text = format!(" ({}-{})", line_range.start.row + 1, line_range.end.row + 1);
        let mut full_path_and_range = full_path_string;
        full_path_and_range.push_str(&line_range_text);
        let mut file_name_and_range = file_name;
        file_name_and_range.push_str(&line_range_text);

        let parent_name = full_path
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned().into());

        let icon_path = FileIcons::get_icon(&full_path, cx);

        ContextFileExcerpt {
            file_name_and_range: file_name_and_range.into(),
            full_path_and_range: full_path_and_range.into(),
            parent_name,
            icon_path,
        }
    }

    fn hover_view(&self, text: SharedString, cx: &mut App) -> Entity<ContextPillHover> {
        let icon_path = self.icon_path.clone();
        let full_path_and_range = self.full_path_and_range.clone();
        ContextPillHover::new(cx, move |_, cx| {
            v_flex()
                .child(
                    h_flex()
                        .gap_0p5()
                        .w_full()
                        .max_w_full()
                        .border_b_1()
                        .border_color(cx.theme().colors().border.opacity(0.6))
                        .children(
                            icon_path
                                .clone()
                                .map(Icon::from_path)
                                .map(|icon| icon.color(Color::Muted).size(IconSize::XSmall)),
                        )
                        .child(
                            // TODO: make this truncate on the left.
                            Label::new(full_path_and_range.clone())
                                .size(LabelSize::Small)
                                .ml_1(),
                        ),
                )
                .child(
                    div()
                        .id("context-pill-hover-contents")
                        .overflow_scroll()
                        .max_w_128()
                        .max_h_96()
                        .child(Label::new(text.clone()).buffer_font(cx)),
                )
                .into_any_element()
        })
    }
}

fn text_hover_view(content: SharedString, cx: &mut App) -> Entity<ContextPillHover> {
    ContextPillHover::new(cx, move |_, _| {
        div()
            .id("context-pill-hover-contents")
            .overflow_scroll()
            .max_w_128()
            .max_h_96()
            .child(content.clone())
            .into_any_element()
    })
}

struct ContextPillHover {
    render_hover: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
}

impl ContextPillHover {
    fn new(
        cx: &mut App,
        render_hover: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Entity<Self> {
        cx.new(|_| Self {
            render_hover: Box::new(render_hover),
        })
    }
}

impl Render for ContextPillHover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        tooltip_container(window, cx, move |this, window, cx| {
            this.occlude()
                .on_mouse_move(|_, _, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child((self.render_hover)(window, cx))
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
        let mut next_context_id = ContextId::zero();
        let image_ready = (
            "Ready",
            AddedContext::image(ImageContext {
                context_id: next_context_id.post_inc(),
                project_path: None,
                original_image: Arc::new(Image::empty()),
                image_task: Task::ready(Some(LanguageModelImage::empty())).shared(),
            }),
        );

        let image_loading = (
            "Loading",
            AddedContext::image(ImageContext {
                context_id: next_context_id.post_inc(),
                project_path: None,
                original_image: Arc::new(Image::empty()),
                image_task: cx
                    .background_spawn(async move {
                        smol::Timer::after(Duration::from_secs(60 * 5)).await;
                        Some(LanguageModelImage::empty())
                    })
                    .shared(),
            }),
        );

        let image_error = (
            "Error",
            AddedContext::image(ImageContext {
                context_id: next_context_id.post_inc(),
                project_path: None,
                original_image: Arc::new(Image::empty()),
                image_task: Task::ready(None).shared(),
            }),
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
