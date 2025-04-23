use crate::context::{AssistantContext, RULES_ICON};
use crate::context_picker::MentionLink;
use crate::thread::{
    LastRestoreCheckpoint, MessageId, MessageSegment, Thread, ThreadError, ThreadEvent,
    ThreadFeedback,
};
use crate::thread_store::{RulesLoadingError, ThreadStore};
use crate::tool_use::{PendingToolUseStatus, ToolUse};
use crate::ui::{
    AddedContext, AgentNotification, AgentNotificationEvent, AnimatedLabel, ContextPill,
};
use crate::{AssistantPanel, OpenActiveThreadAsMarkdown};
use anyhow::Context as _;
use assistant_settings::{AssistantSettings, NotifyWhenAgentWaiting};
use assistant_tool::ToolUseStatus;
use collections::{HashMap, HashSet};
use editor::scroll::Autoscroll;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle, MultiBuffer};
use gpui::{
    AbsoluteLength, Animation, AnimationExt, AnyElement, App, ClickEvent, ClipboardItem,
    DefiniteLength, EdgesRefinement, Empty, Entity, EventEmitter, Focusable, Hsla, ListAlignment,
    ListState, MouseButton, PlatformDisplay, ScrollHandle, Stateful, StyleRefinement, Subscription,
    Task, TextStyle, TextStyleRefinement, Transformation, UnderlineStyle, WeakEntity, WindowHandle,
    linear_color_stop, linear_gradient, list, percentage, pulsating_between,
};
use language::{Buffer, LanguageRegistry};
use language_model::{
    LanguageModelRegistry, LanguageModelRequestMessage, LanguageModelToolUseId, RequestUsage, Role,
    StopReason,
};
use markdown::parser::{CodeBlockKind, CodeBlockMetadata};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle, ParsedMarkdown};
use project::ProjectItem as _;
use rope::Point;
use settings::{Settings as _, update_settings_file};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use text::ToPoint;
use theme::ThemeSettings;
use ui::{
    Disclosure, IconButton, KeyBinding, Scrollbar, ScrollbarState, TextSize, Tooltip, prelude::*,
};
use util::ResultExt as _;
use workspace::{OpenOptions, Workspace};
use zed_actions::assistant::OpenPromptLibrary;

use crate::context_store::ContextStore;

pub struct ActiveThread {
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    thread: Entity<Thread>,
    context_store: Entity<ContextStore>,
    workspace: WeakEntity<Workspace>,
    save_thread_task: Option<Task<()>>,
    messages: Vec<MessageId>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    rendered_messages_by_id: HashMap<MessageId, RenderedMessage>,
    rendered_tool_uses: HashMap<LanguageModelToolUseId, RenderedToolUse>,
    editing_message: Option<(MessageId, EditMessageState)>,
    expanded_tool_uses: HashMap<LanguageModelToolUseId, bool>,
    expanded_thinking_segments: HashMap<(MessageId, usize), bool>,
    expanded_code_blocks: HashMap<(MessageId, usize), bool>,
    last_error: Option<ThreadError>,
    last_usage: Option<RequestUsage>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    copied_code_block_ids: HashSet<(MessageId, usize)>,
    _subscriptions: Vec<Subscription>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    open_feedback_editors: HashMap<MessageId, Entity<Editor>>,
}

struct RenderedMessage {
    language_registry: Arc<LanguageRegistry>,
    segments: Vec<RenderedMessageSegment>,
}

#[derive(Clone)]
struct RenderedToolUse {
    label: Entity<Markdown>,
    input: Entity<Markdown>,
    output: Entity<Markdown>,
}

impl RenderedMessage {
    fn from_segments(
        segments: &[MessageSegment],
        language_registry: Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let mut this = Self {
            language_registry,
            segments: Vec::with_capacity(segments.len()),
        };
        for segment in segments {
            this.push_segment(segment, cx);
        }
        this
    }

    fn append_thinking(&mut self, text: &String, cx: &mut App) {
        if let Some(RenderedMessageSegment::Thinking {
            content,
            scroll_handle,
        }) = self.segments.last_mut()
        {
            content.update(cx, |markdown, cx| {
                markdown.append(text, cx);
            });
            scroll_handle.scroll_to_bottom();
        } else {
            self.segments.push(RenderedMessageSegment::Thinking {
                content: parse_markdown(text.into(), self.language_registry.clone(), cx),
                scroll_handle: ScrollHandle::default(),
            });
        }
    }

    fn append_text(&mut self, text: &String, cx: &mut App) {
        if let Some(RenderedMessageSegment::Text(markdown)) = self.segments.last_mut() {
            markdown.update(cx, |markdown, cx| markdown.append(text, cx));
        } else {
            self.segments
                .push(RenderedMessageSegment::Text(parse_markdown(
                    SharedString::from(text),
                    self.language_registry.clone(),
                    cx,
                )));
        }
    }

    fn push_segment(&mut self, segment: &MessageSegment, cx: &mut App) {
        match segment {
            MessageSegment::Thinking { text, .. } => {
                self.segments.push(RenderedMessageSegment::Thinking {
                    content: parse_markdown(text.into(), self.language_registry.clone(), cx),
                    scroll_handle: ScrollHandle::default(),
                })
            }
            MessageSegment::Text(text) => {
                self.segments
                    .push(RenderedMessageSegment::Text(parse_markdown(
                        text.into(),
                        self.language_registry.clone(),
                        cx,
                    )))
            }
            MessageSegment::RedactedThinking(_) => {}
        };
    }
}

enum RenderedMessageSegment {
    Thinking {
        content: Entity<Markdown>,
        scroll_handle: ScrollHandle,
    },
    Text(Entity<Markdown>),
}

fn parse_markdown(
    text: SharedString,
    language_registry: Arc<LanguageRegistry>,
    cx: &mut App,
) -> Entity<Markdown> {
    cx.new(|cx| Markdown::new(text, Some(language_registry), None, cx))
}

fn default_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let ui_font_size = TextSize::Default.rems(cx);
    let buffer_font_size = TextSize::Small.rems(cx);
    let mut text_style = window.text_style();

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(ui_font_size.into()),
        color: Some(cx.theme().colors().text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style.clone(),
        syntax: cx.theme().syntax().clone(),
        selection_background_color: cx.theme().players().local().selection,
        code_block_overflow_x_scroll: true,
        table_overflow_x_scroll: true,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        code_block: StyleRefinement {
            padding: EdgesRefinement {
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(Pixels(8.)))),
            },
            background: Some(colors.editor_background.into()),
            text: Some(TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            font_features: Some(theme_settings.buffer_font.features.clone()),
            font_size: Some(buffer_font_size.into()),
            background_color: Some(colors.editor_foreground.opacity(0.08)),
            ..Default::default()
        },
        link: TextStyleRefinement {
            background_color: Some(colors.editor_foreground.opacity(0.025)),
            underline: Some(UnderlineStyle {
                color: Some(colors.text_accent.opacity(0.5)),
                thickness: px(1.),
                ..Default::default()
            }),
            ..Default::default()
        },
        link_callback: Some(Rc::new(move |url, cx| {
            if MentionLink::is_valid(url) {
                let colors = cx.theme().colors();
                Some(TextStyleRefinement {
                    background_color: Some(colors.element_background),
                    ..Default::default()
                })
            } else {
                None
            }
        })),
        ..Default::default()
    }
}

fn tool_use_markdown_style(window: &Window, cx: &mut App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();
    let ui_font_size = TextSize::Default.rems(cx);
    let buffer_font_size = TextSize::Small.rems(cx);
    let mut text_style = window.text_style();

    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(ui_font_size.into()),
        color: Some(cx.theme().colors().text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style,
        syntax: cx.theme().syntax().clone(),
        selection_background_color: cx.theme().players().local().selection,
        code_block_overflow_x_scroll: true,
        code_block: StyleRefinement {
            margin: EdgesRefinement::default(),
            padding: EdgesRefinement::default(),
            background: Some(colors.editor_background.into()),
            border_color: None,
            border_widths: EdgesRefinement::default(),
            text: Some(TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(buffer_font_size.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            font_features: Some(theme_settings.buffer_font.features.clone()),
            font_size: Some(TextSize::XSmall.rems(cx).into()),
            ..Default::default()
        },
        heading: StyleRefinement {
            text: Some(TextStyleRefinement {
                font_size: Some(ui_font_size.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

const MAX_UNCOLLAPSED_LINES_IN_CODE_BLOCK: usize = 10;

fn render_markdown_code_block(
    message_id: MessageId,
    ix: usize,
    kind: &CodeBlockKind,
    parsed_markdown: &ParsedMarkdown,
    metadata: CodeBlockMetadata,
    active_thread: Entity<ActiveThread>,
    workspace: WeakEntity<Workspace>,
    _window: &Window,
    cx: &App,
) -> Div {
    let label = match kind {
        CodeBlockKind::Indented => None,
        CodeBlockKind::Fenced => Some(
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::Code)
                        .color(Color::Muted)
                        .size(IconSize::XSmall),
                )
                .child(Label::new("untitled").size(LabelSize::Small))
                .into_any_element(),
        ),
        CodeBlockKind::FencedLang(raw_language_name) => Some(
            h_flex()
                .gap_1()
                .children(
                    parsed_markdown
                        .languages_by_name
                        .get(raw_language_name)
                        .and_then(|language| {
                            language
                                .config()
                                .matcher
                                .path_suffixes
                                .iter()
                                .find_map(|extension| {
                                    file_icons::FileIcons::get_icon(Path::new(extension), cx)
                                })
                                .map(Icon::from_path)
                                .map(|icon| icon.color(Color::Muted).size(IconSize::Small))
                        }),
                )
                .child(
                    Label::new(
                        parsed_markdown
                            .languages_by_name
                            .get(raw_language_name)
                            .map(|language| language.name().into())
                            .clone()
                            .unwrap_or_else(|| raw_language_name.clone()),
                    )
                    .size(LabelSize::Small),
                )
                .into_any_element(),
        ),
        CodeBlockKind::FencedSrc(path_range) => path_range.path.file_name().map(|file_name| {
            let content = if let Some(parent) = path_range.path.parent() {
                h_flex()
                    .ml_1()
                    .gap_1()
                    .child(
                        Label::new(file_name.to_string_lossy().to_string()).size(LabelSize::Small),
                    )
                    .child(
                        Label::new(parent.to_string_lossy().to_string())
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .into_any_element()
            } else {
                Label::new(path_range.path.to_string_lossy().to_string())
                    .size(LabelSize::Small)
                    .ml_1()
                    .into_any_element()
            };

            h_flex()
                .id(("code-block-header-label", ix))
                .w_full()
                .max_w_full()
                .px_1()
                .gap_0p5()
                .cursor_pointer()
                .rounded_sm()
                .hover(|item| item.bg(cx.theme().colors().element_hover.opacity(0.5)))
                .tooltip(Tooltip::text("Jump to File"))
                .child(
                    h_flex()
                        .gap_0p5()
                        .children(
                            file_icons::FileIcons::get_icon(&path_range.path, cx)
                                .map(Icon::from_path)
                                .map(|icon| icon.color(Color::Muted).size(IconSize::XSmall)),
                        )
                        .child(content)
                        .child(
                            Icon::new(IconName::ArrowUpRight)
                                .size(IconSize::XSmall)
                                .color(Color::Ignored),
                        ),
                )
                .on_click({
                    let path_range = path_range.clone();
                    move |_, window, cx| {
                        workspace
                            .update(cx, {
                                |workspace, cx| {
                                    if let Some(project_path) = workspace
                                        .project()
                                        .read(cx)
                                        .find_project_path(&path_range.path, cx)
                                    {
                                        let target = path_range.range.as_ref().map(|range| {
                                            Point::new(
                                                // Line number is 1-based
                                                range.start.line.saturating_sub(1),
                                                range.start.col.unwrap_or(0),
                                            )
                                        });
                                        let open_task = workspace.open_path(
                                            project_path,
                                            None,
                                            true,
                                            window,
                                            cx,
                                        );
                                        window
                                            .spawn(cx, async move |cx| {
                                                let item = open_task.await?;
                                                if let Some(target) = target {
                                                    if let Some(active_editor) =
                                                        item.downcast::<Editor>()
                                                    {
                                                        active_editor
                                                            .downgrade()
                                                            .update_in(cx, |editor, window, cx| {
                                                                editor
                                                                    .go_to_singleton_buffer_point(
                                                                        target, window, cx,
                                                                    );
                                                            })
                                                            .log_err();
                                                    }
                                                }
                                                anyhow::Ok(())
                                            })
                                            .detach_and_log_err(cx);
                                    }
                                }
                            })
                            .ok();
                    }
                })
                .into_any_element()
        }),
    };

    let codeblock_was_copied = active_thread
        .read(cx)
        .copied_code_block_ids
        .contains(&(message_id, ix));

    let is_expanded = active_thread
        .read(cx)
        .expanded_code_blocks
        .get(&(message_id, ix))
        .copied()
        .unwrap_or(false);

    let codeblock_header_bg = cx
        .theme()
        .colors()
        .element_background
        .blend(cx.theme().colors().editor_foreground.opacity(0.01));

    let codeblock_header = h_flex()
        .py_1()
        .pl_1p5()
        .pr_1()
        .gap_1()
        .justify_between()
        .border_b_1()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .bg(codeblock_header_bg)
        .rounded_t_md()
        .children(label)
        .child(
            h_flex()
                .gap_1()
                .child(
                    div().visible_on_hover("codeblock_container").child(
                        IconButton::new(
                            ("copy-markdown-code", ix),
                            if codeblock_was_copied {
                                IconName::Check
                            } else {
                                IconName::Copy
                            },
                        )
                        .icon_color(Color::Muted)
                        .shape(ui::IconButtonShape::Square)
                        .tooltip(Tooltip::text("Copy Code"))
                        .on_click({
                            let active_thread = active_thread.clone();
                            let parsed_markdown = parsed_markdown.clone();
                            let code_block_range = metadata.content_range.clone();
                            move |_event, _window, cx| {
                                active_thread.update(cx, |this, cx| {
                                    this.copied_code_block_ids.insert((message_id, ix));

                                    let code = parsed_markdown.source()[code_block_range.clone()]
                                        .to_string();
                                    cx.write_to_clipboard(ClipboardItem::new_string(code));

                                    cx.spawn(async move |this, cx| {
                                        cx.background_executor()
                                            .timer(Duration::from_secs(2))
                                            .await;

                                        cx.update(|cx| {
                                            this.update(cx, |this, cx| {
                                                this.copied_code_block_ids
                                                    .remove(&(message_id, ix));
                                                cx.notify();
                                            })
                                        })
                                        .ok();
                                    })
                                    .detach();
                                });
                            }
                        }),
                    ),
                )
                .when(
                    metadata.line_count > MAX_UNCOLLAPSED_LINES_IN_CODE_BLOCK,
                    |header| {
                        header.child(
                            IconButton::new(
                                ("expand-collapse-code", ix),
                                if is_expanded {
                                    IconName::ChevronUp
                                } else {
                                    IconName::ChevronDown
                                },
                            )
                            .icon_color(Color::Muted)
                            .shape(ui::IconButtonShape::Square)
                            .tooltip(Tooltip::text(if is_expanded {
                                "Collapse Code"
                            } else {
                                "Expand Code"
                            }))
                            .on_click({
                                let active_thread = active_thread.clone();
                                move |_event, _window, cx| {
                                    active_thread.update(cx, |this, cx| {
                                        let is_expanded = this
                                            .expanded_code_blocks
                                            .entry((message_id, ix))
                                            .or_insert(false);
                                        *is_expanded = !*is_expanded;
                                        cx.notify();
                                    });
                                }
                            }),
                        )
                    },
                ),
        );

    v_flex()
        .group("codeblock_container")
        .my_2()
        .overflow_hidden()
        .rounded_lg()
        .border_1()
        .border_color(cx.theme().colors().border.opacity(0.6))
        .bg(cx.theme().colors().editor_background)
        .child(codeblock_header)
        .when(
            metadata.line_count > MAX_UNCOLLAPSED_LINES_IN_CODE_BLOCK,
            |this| {
                if is_expanded {
                    this.h_full()
                } else {
                    this.max_h_80()
                }
            },
        )
}

fn open_markdown_link(
    text: SharedString,
    workspace: WeakEntity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(workspace) = workspace.upgrade() else {
        cx.open_url(&text);
        return;
    };

    match MentionLink::try_parse(&text, &workspace, cx) {
        Some(MentionLink::File(path, entry)) => workspace.update(cx, |workspace, cx| {
            if entry.is_dir() {
                workspace.project().update(cx, |_, cx| {
                    cx.emit(project::Event::RevealInProjectPanel(entry.id));
                })
            } else {
                workspace
                    .open_path(path, None, true, window, cx)
                    .detach_and_log_err(cx);
            }
        }),
        Some(MentionLink::Symbol(path, symbol_name)) => {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_path(path, None, true, window, cx)
            });
            window
                .spawn(cx, async move |cx| {
                    let active_editor = open_task
                        .await?
                        .downcast::<Editor>()
                        .context("Item is not an editor")?;
                    active_editor.update_in(cx, |editor, window, cx| {
                        let symbol_range = editor
                            .buffer()
                            .read(cx)
                            .snapshot(cx)
                            .outline(None)
                            .and_then(|outline| {
                                outline
                                    .find_most_similar(&symbol_name)
                                    .map(|(_, item)| item.range.clone())
                            })
                            .context("Could not find matching symbol")?;

                        editor.change_selections(Some(Autoscroll::center()), window, cx, |s| {
                            s.select_anchor_ranges([symbol_range.start..symbol_range.start])
                        });
                        anyhow::Ok(())
                    })
                })
                .detach_and_log_err(cx);
        }
        Some(MentionLink::Selection(path, line_range)) => {
            let open_task = workspace.update(cx, |workspace, cx| {
                workspace.open_path(path, None, true, window, cx)
            });
            window
                .spawn(cx, async move |cx| {
                    let active_editor = open_task
                        .await?
                        .downcast::<Editor>()
                        .context("Item is not an editor")?;
                    active_editor.update_in(cx, |editor, window, cx| {
                        editor.change_selections(Some(Autoscroll::center()), window, cx, |s| {
                            s.select_ranges([Point::new(line_range.start as u32, 0)
                                ..Point::new(line_range.start as u32, 0)])
                        });
                        anyhow::Ok(())
                    })
                })
                .detach_and_log_err(cx);
        }
        Some(MentionLink::Thread(thread_id)) => workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel
                        .open_thread(&thread_id, window, cx)
                        .detach_and_log_err(cx)
                });
            }
        }),
        Some(MentionLink::Fetch(url)) => cx.open_url(&url),
        Some(MentionLink::Rules(prompt_id)) => window.dispatch_action(
            Box::new(OpenPromptLibrary {
                prompt_to_select: Some(prompt_id.0),
            }),
            cx,
        ),
        None => cx.open_url(&text),
    }
}

struct EditMessageState {
    editor: Entity<Editor>,
    last_estimated_token_count: Option<usize>,
    _subscription: Subscription,
    _update_token_count_task: Option<Task<anyhow::Result<()>>>,
}

impl ActiveThread {
    pub fn new(
        thread: Entity<Thread>,
        thread_store: Entity<ThreadStore>,
        language_registry: Arc<LanguageRegistry>,
        context_store: Entity<ContextStore>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
            cx.subscribe(&thread_store, Self::handle_rules_loading_error),
        ];

        let list_state = ListState::new(0, ListAlignment::Bottom, px(2048.), {
            let this = cx.entity().downgrade();
            move |ix, window: &mut Window, cx: &mut App| {
                this.update(cx, |this, cx| this.render_message(ix, window, cx))
                    .unwrap()
            }
        });

        let mut this = Self {
            language_registry,
            thread_store,
            thread: thread.clone(),
            context_store,
            workspace,
            save_thread_task: None,
            messages: Vec::new(),
            rendered_messages_by_id: HashMap::default(),
            rendered_tool_uses: HashMap::default(),
            expanded_tool_uses: HashMap::default(),
            expanded_thinking_segments: HashMap::default(),
            expanded_code_blocks: HashMap::default(),
            list_state: list_state.clone(),
            scrollbar_state: ScrollbarState::new(list_state),
            show_scrollbar: false,
            hide_scrollbar_task: None,
            editing_message: None,
            last_error: None,
            last_usage: None,
            copied_code_block_ids: HashSet::default(),
            notifications: Vec::new(),
            _subscriptions: subscriptions,
            notification_subscriptions: HashMap::default(),
            open_feedback_editors: HashMap::default(),
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, &message.segments, window, cx);

            for tool_use in thread.read(cx).tool_uses_for_message(message.id, cx) {
                this.render_tool_use_markdown(
                    tool_use.id.clone(),
                    tool_use.ui_text.clone(),
                    &tool_use.input,
                    tool_use.status.text(),
                    cx,
                );
            }
        }

        this
    }

    pub fn context_store(&self) -> &Entity<ContextStore> {
        &self.context_store
    }

    pub fn thread(&self) -> &Entity<Thread> {
        &self.thread
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn summary(&self, cx: &App) -> Option<SharedString> {
        self.thread.read(cx).summary()
    }

    pub fn summary_or_default(&self, cx: &App) -> SharedString {
        self.thread.read(cx).summary_or_default()
    }

    pub fn cancel_last_completion(&mut self, cx: &mut App) -> bool {
        self.last_error.take();
        self.thread
            .update(cx, |thread, cx| thread.cancel_last_completion(cx))
    }

    pub fn last_error(&self) -> Option<ThreadError> {
        self.last_error.clone()
    }

    pub fn clear_last_error(&mut self) {
        self.last_error.take();
    }

    pub fn last_usage(&self) -> Option<RequestUsage> {
        self.last_usage
    }

    /// Returns the editing message id and the estimated token count in the content
    pub fn editing_message_id(&self) -> Option<(MessageId, usize)> {
        self.editing_message
            .as_ref()
            .map(|(id, state)| (*id, state.last_estimated_token_count.unwrap_or(0)))
    }

    fn push_message(
        &mut self,
        id: &MessageId,
        segments: &[MessageSegment],
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_len = self.messages.len();
        self.messages.push(*id);
        self.list_state.splice(old_len..old_len, 1);

        let rendered_message =
            RenderedMessage::from_segments(segments, self.language_registry.clone(), cx);
        self.rendered_messages_by_id.insert(*id, rendered_message);
    }

    fn edited_message(
        &mut self,
        id: &MessageId,
        segments: &[MessageSegment],
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.list_state.splice(index..index + 1, 1);
        let rendered_message =
            RenderedMessage::from_segments(segments, self.language_registry.clone(), cx);
        self.rendered_messages_by_id.insert(*id, rendered_message);
    }

    fn deleted_message(&mut self, id: &MessageId) {
        let Some(index) = self.messages.iter().position(|message_id| message_id == id) else {
            return;
        };
        self.messages.remove(index);
        self.list_state.splice(index..index + 1, 0);
        self.rendered_messages_by_id.remove(id);
    }

    fn render_tool_use_markdown(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_label: impl Into<SharedString>,
        tool_input: &serde_json::Value,
        tool_output: SharedString,
        cx: &mut Context<Self>,
    ) {
        let rendered = self
            .rendered_tool_uses
            .entry(tool_use_id.clone())
            .or_insert_with(|| RenderedToolUse {
                label: cx.new(|cx| {
                    Markdown::new("".into(), Some(self.language_registry.clone()), None, cx)
                }),
                input: cx.new(|cx| {
                    Markdown::new("".into(), Some(self.language_registry.clone()), None, cx)
                }),
                output: cx.new(|cx| {
                    Markdown::new("".into(), Some(self.language_registry.clone()), None, cx)
                }),
            });

        rendered.label.update(cx, |this, cx| {
            this.replace(tool_label, cx);
        });
        rendered.input.update(cx, |this, cx| {
            let input = format!(
                "```json\n{}\n```",
                serde_json::to_string_pretty(tool_input).unwrap_or_default()
            );
            this.replace(input, cx);
        });
        rendered.output.update(cx, |this, cx| {
            this.replace(tool_output, cx);
        });
    }

    fn handle_thread_event(
        &mut self,
        _thread: &Entity<Thread>,
        event: &ThreadEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            ThreadEvent::ShowError(error) => {
                self.last_error = Some(error.clone());
            }
            ThreadEvent::UsageUpdated(usage) => {
                self.last_usage = Some(*usage);
            }
            ThreadEvent::StreamedCompletion
            | ThreadEvent::SummaryGenerated
            | ThreadEvent::SummaryChanged => {
                self.save_thread(cx);
            }
            ThreadEvent::Stopped(reason) => match reason {
                Ok(StopReason::EndTurn | StopReason::MaxTokens) => {
                    let thread = self.thread.read(cx);
                    self.show_notification(
                        if thread.used_tools_since_last_user_message() {
                            "Finished running tools"
                        } else {
                            "New message"
                        },
                        IconName::ZedAssistant,
                        window,
                        cx,
                    );
                }
                _ => {}
            },
            ThreadEvent::ToolConfirmationNeeded => {
                self.show_notification("Waiting for tool confirmation", IconName::Info, window, cx);
            }
            ThreadEvent::StreamedAssistantText(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_text(text, cx);
                }
            }
            ThreadEvent::StreamedAssistantThinking(message_id, text) => {
                if let Some(rendered_message) = self.rendered_messages_by_id.get_mut(&message_id) {
                    rendered_message.append_thinking(text, cx);
                }
            }
            ThreadEvent::MessageAdded(message_id) => {
                if let Some(message_segments) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.segments.clone())
                {
                    self.push_message(message_id, &message_segments, window, cx);
                }

                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::MessageEdited(message_id) => {
                if let Some(message_segments) = self
                    .thread
                    .read(cx)
                    .message(*message_id)
                    .map(|message| message.segments.clone())
                {
                    self.edited_message(message_id, &message_segments, window, cx);
                }

                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::MessageDeleted(message_id) => {
                self.deleted_message(message_id);
                self.save_thread(cx);
                cx.notify();
            }
            ThreadEvent::UsePendingTools { tool_uses } => {
                for tool_use in tool_uses {
                    self.render_tool_use_markdown(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        &tool_use.input,
                        "".into(),
                        cx,
                    );
                }
            }
            ThreadEvent::StreamedToolUse {
                tool_use_id,
                ui_text,
                input,
            } => {
                self.render_tool_use_markdown(
                    tool_use_id.clone(),
                    ui_text.clone(),
                    input,
                    "".into(),
                    cx,
                );
            }
            ThreadEvent::ToolFinished {
                pending_tool_use, ..
            } => {
                if let Some(tool_use) = pending_tool_use {
                    self.render_tool_use_markdown(
                        tool_use.id.clone(),
                        tool_use.ui_text.clone(),
                        &tool_use.input,
                        self.thread
                            .read(cx)
                            .output_for_tool(&tool_use.id)
                            .map(|output| output.clone().into())
                            .unwrap_or("".into()),
                        cx,
                    );
                }
            }
            ThreadEvent::CheckpointChanged => cx.notify(),
            ThreadEvent::ReceivedTextChunk => {}
        }
    }

    fn handle_rules_loading_error(
        &mut self,
        _thread_store: Entity<ThreadStore>,
        error: &RulesLoadingError,
        cx: &mut Context<Self>,
    ) {
        self.last_error = Some(ThreadError::Message {
            header: "Error loading rules file".into(),
            message: error.message.clone(),
        });
        cx.notify();
    }

    fn show_notification(
        &mut self,
        caption: impl Into<SharedString>,
        icon: IconName,
        window: &mut Window,
        cx: &mut Context<ActiveThread>,
    ) {
        if window.is_window_active() || !self.notifications.is_empty() {
            return;
        }

        let title = self
            .thread
            .read(cx)
            .summary()
            .unwrap_or("Agent Panel".into());

        match AssistantSettings::get_global(cx).notify_when_agent_waiting {
            NotifyWhenAgentWaiting::PrimaryScreen => {
                if let Some(primary) = cx.primary_display() {
                    self.pop_up(icon, caption.into(), title.clone(), window, primary, cx);
                }
            }
            NotifyWhenAgentWaiting::AllScreens => {
                let caption = caption.into();
                for screen in cx.displays() {
                    self.pop_up(icon, caption.clone(), title.clone(), window, screen, cx);
                }
            }
            NotifyWhenAgentWaiting::Never => {
                // Don't show anything
            }
        }
    }

    fn pop_up(
        &mut self,
        icon: IconName,
        caption: SharedString,
        title: SharedString,
        window: &mut Window,
        screen: Rc<dyn PlatformDisplay>,
        cx: &mut Context<'_, ActiveThread>,
    ) {
        let options = AgentNotification::window_options(screen, cx);

        let project_name = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .visible_worktrees(cx)
                .next()
                .map(|worktree| worktree.read(cx).root_name().to_string())
        });

        if let Some(screen_window) = cx
            .open_window(options, |_, cx| {
                cx.new(|_| {
                    AgentNotification::new(title.clone(), caption.clone(), icon, project_name)
                })
            })
            .log_err()
        {
            if let Some(pop_up) = screen_window.entity(cx).log_err() {
                self.notification_subscriptions
                    .entry(screen_window)
                    .or_insert_with(Vec::new)
                    .push(cx.subscribe_in(&pop_up, window, {
                        |this, _, event, window, cx| match event {
                            AgentNotificationEvent::Accepted => {
                                let handle = window.window_handle();
                                cx.activate(true);

                                let workspace_handle = this.workspace.clone();

                                // If there are multiple Zed windows, activate the correct one.
                                cx.defer(move |cx| {
                                    handle
                                        .update(cx, |_view, window, _cx| {
                                            window.activate_window();

                                            if let Some(workspace) = workspace_handle.upgrade() {
                                                workspace.update(_cx, |workspace, cx| {
                                                    workspace
                                                        .focus_panel::<AssistantPanel>(window, cx);
                                                });
                                            }
                                        })
                                        .log_err();
                                });

                                this.dismiss_notifications(cx);
                            }
                            AgentNotificationEvent::Dismissed => {
                                this.dismiss_notifications(cx);
                            }
                        }
                    }));

                self.notifications.push(screen_window);

                // If the user manually refocuses the original window, dismiss the popup.
                self.notification_subscriptions
                    .entry(screen_window)
                    .or_insert_with(Vec::new)
                    .push({
                        let pop_up_weak = pop_up.downgrade();

                        cx.observe_window_activation(window, move |_, window, cx| {
                            if window.is_window_active() {
                                if let Some(pop_up) = pop_up_weak.upgrade() {
                                    pop_up.update(cx, |_, cx| {
                                        cx.emit(AgentNotificationEvent::Dismissed);
                                    });
                                }
                            }
                        })
                    });
            }
        }
    }

    /// Spawns a task to save the active thread.
    ///
    /// Only one task to save the thread will be in flight at a time.
    fn save_thread(&mut self, cx: &mut Context<Self>) {
        let thread = self.thread.clone();
        self.save_thread_task = Some(cx.spawn(async move |this, cx| {
            let task = this
                .update(cx, |this, cx| {
                    this.thread_store
                        .update(cx, |thread_store, cx| thread_store.save_thread(&thread, cx))
                })
                .ok();

            if let Some(task) = task {
                task.await.log_err();
            }
        }));
    }

    fn start_editing_message(
        &mut self,
        message_id: MessageId,
        message_segments: &[MessageSegment],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // User message should always consist of a single text segment,
        // therefore we can skip returning early if it's not a text segment.
        let Some(MessageSegment::Text(message_text)) = message_segments.first() else {
            return;
        };

        let buffer = cx.new(|cx| {
            MultiBuffer::singleton(cx.new(|cx| Buffer::local(message_text.clone(), cx)), cx)
        });
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight { max_lines: 8 },
                buffer,
                None,
                window,
                cx,
            );
            editor.focus_handle(cx).focus(window);
            editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
            editor
        });
        let subscription = cx.subscribe(&editor, |this, _, event, cx| match event {
            EditorEvent::BufferEdited => {
                this.update_editing_message_token_count(true, cx);
            }
            _ => {}
        });
        self.editing_message = Some((
            message_id,
            EditMessageState {
                editor: editor.clone(),
                last_estimated_token_count: None,
                _subscription: subscription,
                _update_token_count_task: None,
            },
        ));
        self.update_editing_message_token_count(false, cx);
        cx.notify();
    }

    fn update_editing_message_token_count(&mut self, debounce: bool, cx: &mut Context<Self>) {
        /* todo!
        let Some((message_id, state)) = self.editing_message.as_mut() else {
            return;
        };

        cx.emit(ActiveThreadEvent::EditingMessageTokenCountChanged);
        state._update_token_count_task.take();

        let Some(default_model) = LanguageModelRegistry::read_global(cx).default_model() else {
            state.last_estimated_token_count.take();
            return;
        };

        let editor = state.editor.clone();
        let thread = self.thread.clone();
        let message_id = *message_id;

        state._update_token_count_task = Some(cx.spawn(async move |this, cx| {
            if debounce {
                cx.background_executor()
                    .timer(Duration::from_millis(200))
                    .await;
            }

            let token_count = if let Some(task) = cx.update(|cx| {
                let context = thread.read(cx).context_for_message(message_id);
                let new_context = thread.read(cx).filter_new_context(context);
                let context_text =
                    format_context_as_string(new_context, cx).unwrap_or(String::new());
                let message_text = editor.read(cx).text(cx);

                let content = context_text + &message_text;

                if content.is_empty() {
                    return None;
                }

                let request = language_model::LanguageModelRequest {
                    thread_id: None,
                    prompt_id: None,
                    messages: vec![LanguageModelRequestMessage {
                        role: language_model::Role::User,
                        content: vec![content.into()],
                        cache: false,
                    }],
                    tools: vec![],
                    stop: vec![],
                    temperature: None,
                };

                Some(default_model.model.count_tokens(request, cx))
            })? {
                task.await?
            } else {
                0
            };

            this.update(cx, |this, cx| {
                let Some((_message_id, state)) = this.editing_message.as_mut() else {
                    return;
                };

                state.last_estimated_token_count = Some(token_count);
                cx.emit(ActiveThreadEvent::EditingMessageTokenCountChanged);
            })
        }));
        */
    }

    fn cancel_editing_message(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.editing_message.take();
        cx.notify();
    }

    fn confirm_editing_message(
        &mut self,
        _: &menu::Confirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((message_id, state)) = self.editing_message.take() else {
            return;
        };
        let edited_text = state.editor.read(cx).text(cx);
        self.thread.update(cx, |thread, cx| {
            thread.edit_message(
                message_id,
                Role::User,
                vec![MessageSegment::Text(edited_text)],
                cx,
            );
            for message_id in self.messages_after(message_id) {
                thread.delete_message(*message_id, cx);
            }
        });

        let Some(model) = LanguageModelRegistry::read_global(cx).default_model() else {
            return;
        };

        if model.provider.must_accept_terms(cx) {
            cx.notify();
            return;
        }

        self.thread.update(cx, |thread, cx| {
            thread.advance_prompt_id();
            thread.send_to_model(model.model, cx)
        });
        cx.notify();
    }

    fn messages_after(&self, message_id: MessageId) -> &[MessageId] {
        self.messages
            .iter()
            .position(|id| *id == message_id)
            .map(|index| &self.messages[index + 1..])
            .unwrap_or(&[])
    }

    fn handle_cancel_click(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.cancel_editing_message(&menu::Cancel, window, cx);
    }

    fn handle_regenerate_click(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.confirm_editing_message(&menu::Confirm, window, cx);
    }

    fn handle_feedback_click(
        &mut self,
        message_id: MessageId,
        feedback: ThreadFeedback,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let report = self.thread.update(cx, |thread, cx| {
            thread.report_message_feedback(message_id, feedback, cx)
        });

        cx.spawn(async move |this, cx| {
            report.await?;
            this.update(cx, |_this, cx| cx.notify())
        })
        .detach_and_log_err(cx);

        match feedback {
            ThreadFeedback::Positive => {
                self.open_feedback_editors.remove(&message_id);
            }
            ThreadFeedback::Negative => {
                self.handle_show_feedback_comments(message_id, window, cx);
            }
        }
    }

    fn handle_show_feedback_comments(
        &mut self,
        message_id: MessageId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = cx.new(|cx| {
            let empty_string = String::new();
            MultiBuffer::singleton(cx.new(|cx| Buffer::local(empty_string, cx)), cx)
        });

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                editor::EditorMode::AutoHeight { max_lines: 4 },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_placeholder_text(
                "What went wrong? Share your feedback so we can improve.",
                cx,
            );
            editor
        });

        editor.read(cx).focus_handle(cx).focus(window);
        self.open_feedback_editors.insert(message_id, editor);
        cx.notify();
    }

    fn submit_feedback_message(&mut self, message_id: MessageId, cx: &mut Context<Self>) {
        let Some(editor) = self.open_feedback_editors.get(&message_id) else {
            return;
        };

        let report_task = self.thread.update(cx, |thread, cx| {
            thread.report_message_feedback(message_id, ThreadFeedback::Negative, cx)
        });

        let comments = editor.read(cx).text(cx);
        if !comments.is_empty() {
            let thread_id = self.thread.read(cx).id().clone();
            let comments_value = String::from(comments.as_str());

            let message_content = self
                .thread
                .read(cx)
                .message(message_id)
                .map(|msg| msg.to_string())
                .unwrap_or_default();

            telemetry::event!(
                "Assistant Thread Feedback Comments",
                thread_id,
                message_id = message_id.0,
                message_content,
                comments = comments_value
            );

            self.open_feedback_editors.remove(&message_id);

            cx.spawn(async move |this, cx| {
                report_task.await?;
                this.update(cx, |_this, cx| cx.notify())
            })
            .detach_and_log_err(cx);
        }
    }

    fn render_message(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(rendered_message) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let context_store = self.context_store.clone();
        let workspace = self.workspace.clone();
        let thread = self.thread.read(cx);

        // Get all the data we need from thread before we start using it in closures
        let checkpoint = thread.checkpoint_for_message(message_id);
        let added_context = if let Some(workspace) = workspace.upgrade() {
            let project = workspace.read(cx).project().read(cx);
            thread
                .context_for_message(message_id)
                .flat_map(|context| AddedContext::new(context.clone(), project, cx))
                .collect::<Vec<_>>()
        } else {
            return Empty.into_any();
        };

        let tool_uses = thread.tool_uses_for_message(message_id, cx);
        let has_tool_uses = !tool_uses.is_empty();
        let is_generating = thread.is_generating();

        let is_first_message = ix == 0;
        let is_last_message = ix == self.messages.len() - 1;

        let show_feedback = (!is_generating && is_last_message && message.role != Role::User)
            || self.messages.get(ix + 1).map_or(false, |next_id| {
                self.thread
                    .read(cx)
                    .message(*next_id)
                    .map_or(false, |next_message| {
                        next_message.role == Role::User
                            && thread.tool_uses_for_message(*next_id, cx).is_empty()
                            && thread.tool_results_for_message(*next_id).is_empty()
                    })
            });

        let needs_confirmation = tool_uses.iter().any(|tool_use| tool_use.needs_confirmation);

        let generating_label = (is_generating && is_last_message)
            .then(|| AnimatedLabel::new("Generating").size(LabelSize::Small));

        // Don't render user messages that are just there for returning tool results.
        if message.role == Role::User && thread.message_has_tool_results(message_id) {
            if let Some(generating_label) = generating_label {
                return h_flex()
                    .w_full()
                    .h_10()
                    .py_1p5()
                    .pl_4()
                    .pb_3()
                    .child(generating_label)
                    .into_any_element();
            }

            return Empty.into_any();
        }

        let allow_editing_message = message.role == Role::User;

        let edit_message_editor = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state.editor.clone());

        let colors = cx.theme().colors();
        let editor_bg_color = colors.editor_background;

        let open_as_markdown = IconButton::new(("open-as-markdown", ix), IconName::FileCode)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Open Thread as Markdown"))
            .on_click(|_, window, cx| {
                window.dispatch_action(Box::new(OpenActiveThreadAsMarkdown), cx)
            });

        // For all items that should be aligned with the Assistant's response.
        const RESPONSE_PADDING_X: Pixels = px(18.);

        let feedback_container = h_flex()
            .py_2()
            .px(RESPONSE_PADDING_X)
            .gap_1()
            .justify_between();
        let feedback_items = match self.thread.read(cx).message_feedback(message_id) {
            Some(feedback) => feedback_container
                .child(
                    Label::new(match feedback {
                        ThreadFeedback::Positive => "Thanks for your feedback!",
                        ThreadFeedback::Negative => {
                            "We appreciate your feedback and will use it to improve."
                        }
                    })
                    .color(Color::Muted)
                    .size(LabelSize::XSmall),
                )
                .child(
                    h_flex()
                        .pr_1()
                        .gap_1()
                        .child(
                            IconButton::new(("feedback-thumbs-up", ix), IconName::ThumbsUp)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::XSmall)
                                .icon_color(match feedback {
                                    ThreadFeedback::Positive => Color::Accent,
                                    ThreadFeedback::Negative => Color::Ignored,
                                })
                                .tooltip(Tooltip::text("Helpful Response"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        message_id,
                                        ThreadFeedback::Positive,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            IconButton::new(("feedback-thumbs-down", ix), IconName::ThumbsDown)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::XSmall)
                                .icon_color(match feedback {
                                    ThreadFeedback::Positive => Color::Ignored,
                                    ThreadFeedback::Negative => Color::Accent,
                                })
                                .tooltip(Tooltip::text("Not Helpful"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        message_id,
                                        ThreadFeedback::Negative,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(open_as_markdown),
                )
                .into_any_element(),
            None => feedback_container
                .child(
                    Label::new(
                        "Rating the thread sends all of your current conversation to the Zed team.",
                    )
                    .color(Color::Muted)
                    .size(LabelSize::XSmall),
                )
                .child(
                    h_flex()
                        .pr_1()
                        .gap_1()
                        .child(
                            IconButton::new(("feedback-thumbs-up", ix), IconName::ThumbsUp)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Helpful Response"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        message_id,
                                        ThreadFeedback::Positive,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(
                            IconButton::new(("feedback-thumbs-down", ix), IconName::ThumbsDown)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Not Helpful"))
                                .on_click(cx.listener(move |this, _, window, cx| {
                                    this.handle_feedback_click(
                                        message_id,
                                        ThreadFeedback::Negative,
                                        window,
                                        cx,
                                    );
                                })),
                        )
                        .child(open_as_markdown),
                )
                .into_any_element(),
        };

        let message_is_empty = message.should_display_content();
        let has_content = !message_is_empty || !added_context.is_empty();

        let message_content =
            has_content.then(|| {
                v_flex()
                    .gap_1()
                    .when(!message_is_empty, |parent| {
                        parent.child(
                            if let Some(edit_message_editor) = edit_message_editor.clone() {
                                let settings = ThemeSettings::get_global(cx);
                                let font_size = TextSize::Small.rems(cx);
                                let line_height = font_size.to_pixels(window.rem_size()) * 1.5;

                                let text_style = TextStyle {
                                    color: cx.theme().colors().text,
                                    font_family: settings.buffer_font.family.clone(),
                                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                                    font_features: settings.buffer_font.features.clone(),
                                    font_size: font_size.into(),
                                    line_height: line_height.into(),
                                    ..Default::default()
                                };

                                div()
                                    .key_context("EditMessageEditor")
                                    .on_action(cx.listener(Self::cancel_editing_message))
                                    .on_action(cx.listener(Self::confirm_editing_message))
                                    .min_h_6()
                                    .child(EditorElement::new(
                                        &edit_message_editor,
                                        EditorStyle {
                                            background: colors.editor_background,
                                            local_player: cx.theme().players().local(),
                                            text: text_style,
                                            syntax: cx.theme().syntax().clone(),
                                            ..Default::default()
                                        },
                                    ))
                                    .into_any()
                            } else {
                                div()
                                    .min_h_6()
                                    .child(self.render_message_content(
                                        message_id,
                                        rendered_message,
                                        has_tool_uses,
                                        workspace.clone(),
                                        window,
                                        cx,
                                    ))
                                    .into_any()
                            },
                        )
                    })
                    .when(!added_context.is_empty(), |parent| {
                        parent.child(h_flex().flex_wrap().gap_1().children(
                            added_context.into_iter().map(|added_context| {
                                let context = added_context.context.clone();
                                ContextPill::added(added_context, false, false, None).on_click(Rc::new(
                                    cx.listener({
                                    let workspace = workspace.clone();
                                    let context_store = context_store.clone();
                                    move |_, _, window, cx| {
                                        if let Some(workspace) = workspace.upgrade() {
                                            open_context(
                                                &context,
                                                context_store.clone(),
                                                workspace,
                                                window,
                                                cx,
                                            );
                                            cx.notify();
                                        }
                                    }
                                }),
                            ))
                        }),
                    ))
                })
        });

        let styled_message = match message.role {
            Role::User => v_flex()
                .id(("message-container", ix))
                .map(|this| {
                    if is_first_message {
                        this.pt_2()
                    } else {
                        this.pt_4()
                    }
                })
                .pl_2()
                .pr_2p5()
                .pb_4()
                .child(
                    v_flex()
                        .bg(editor_bg_color)
                        .rounded_lg()
                        .border_1()
                        .border_color(colors.border)
                        .shadow_md()
                        .child(div().p_2().children(message_content))
                        .child(
                            h_flex()
                                .p_1()
                                .border_t_1()
                                .border_color(colors.border_variant)
                                .justify_end()
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .when_some(
                                            edit_message_editor.clone(),
                                            |this, edit_message_editor| {
                                                let focus_handle =
                                                    edit_message_editor.focus_handle(cx);
                                                this.child(
                                                    Button::new("cancel-edit-message", "Cancel")
                                                        .label_size(LabelSize::Small)
                                                        .key_binding(
                                                            KeyBinding::for_action_in(
                                                                &menu::Cancel,
                                                                &focus_handle,
                                                                window,
                                                                cx,
                                                            )
                                                            .map(|kb| kb.size(rems_from_px(12.))),
                                                        )
                                                        .on_click(
                                                            cx.listener(Self::handle_cancel_click),
                                                        ),
                                                )
                                                .child(
                                                    Button::new(
                                                        "confirm-edit-message",
                                                        "Regenerate",
                                                    )
                                                    .disabled(
                                                        edit_message_editor.read(cx).is_empty(cx),
                                                    )
                                                    .label_size(LabelSize::Small)
                                                    .key_binding(
                                                        KeyBinding::for_action_in(
                                                            &menu::Confirm,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                        .map(|kb| kb.size(rems_from_px(12.))),
                                                    )
                                                    .on_click(
                                                        cx.listener(Self::handle_regenerate_click),
                                                    ),
                                                )
                                            },
                                        )
                                        .when(
                                            edit_message_editor.is_none() && allow_editing_message,
                                            |this| {
                                                this.child(
                                                    Button::new("edit-message", "Edit Message")
                                                        .label_size(LabelSize::Small)
                                                        .icon(IconName::Pencil)
                                                        .icon_size(IconSize::XSmall)
                                                        .icon_color(Color::Muted)
                                                        .icon_position(IconPosition::Start)
                                                        .on_click(cx.listener({
                                                            let message_segments =
                                                                message.segments.clone();
                                                            move |this, _, window, cx| {
                                                                this.start_editing_message(
                                                                    message_id,
                                                                    &message_segments,
                                                                    window,
                                                                    cx,
                                                                );
                                                            }
                                                        })),
                                                )
                                            },
                                        ),
                                ),
                        ),
                ),
            Role::Assistant => v_flex()
                .id(("message-container", ix))
                .px(RESPONSE_PADDING_X)
                .gap_2()
                .children(message_content)
                .when(has_tool_uses, |parent| {
                    parent.children(
                        tool_uses
                            .into_iter()
                            .map(|tool_use| self.render_tool_use(tool_use, window, cx)),
                    )
                }),
            Role::System => div().id(("message-container", ix)).py_1().px_2().child(
                v_flex()
                    .bg(colors.editor_background)
                    .rounded_sm()
                    .child(div().p_4().children(message_content)),
            ),
        };

        let after_editing_message = self
            .editing_message
            .as_ref()
            .map_or(false, |(editing_message_id, _)| {
                message_id > *editing_message_id
            });

        let panel_background = cx.theme().colors().panel_background;

        v_flex()
            .w_full()
            .when_some(checkpoint, |parent, checkpoint| {
                let mut is_pending = false;
                let mut error = None;
                if let Some(last_restore_checkpoint) =
                    self.thread.read(cx).last_restore_checkpoint()
                {
                    if last_restore_checkpoint.message_id() == message_id {
                        match last_restore_checkpoint {
                            LastRestoreCheckpoint::Pending { .. } => is_pending = true,
                            LastRestoreCheckpoint::Error { error: err, .. } => {
                                error = Some(err.clone());
                            }
                        }
                    }
                }

                let restore_checkpoint_button =
                    Button::new(("restore-checkpoint", ix), "Restore Checkpoint")
                        .icon(if error.is_some() {
                            IconName::XCircle
                        } else {
                            IconName::Undo
                        })
                        .icon_size(IconSize::XSmall)
                        .icon_position(IconPosition::Start)
                        .icon_color(if error.is_some() {
                            Some(Color::Error)
                        } else {
                            None
                        })
                        .label_size(LabelSize::XSmall)
                        .disabled(is_pending)
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.thread.update(cx, |thread, cx| {
                                thread
                                    .restore_checkpoint(checkpoint.clone(), cx)
                                    .detach_and_log_err(cx);
                            });
                        }));

                let restore_checkpoint_button = if is_pending {
                    restore_checkpoint_button
                        .with_animation(
                            ("pulsating-restore-checkpoint-button", ix),
                            Animation::new(Duration::from_secs(2))
                                .repeat()
                                .with_easing(pulsating_between(0.6, 1.)),
                            |label, delta| label.alpha(delta),
                        )
                        .into_any_element()
                } else if let Some(error) = error {
                    restore_checkpoint_button
                        .tooltip(Tooltip::text(error.to_string()))
                        .into_any_element()
                } else {
                    restore_checkpoint_button.into_any_element()
                };

                parent.child(
                    h_flex()
                        .pt_2p5()
                        .px_2p5()
                        .w_full()
                        .gap_1()
                        .child(ui::Divider::horizontal())
                        .child(restore_checkpoint_button)
                        .child(ui::Divider::horizontal()),
                )
            })
            .when(is_first_message, |parent| {
                parent.child(self.render_rules_item(cx))
            })
            .child(styled_message)
            .when(!needs_confirmation && generating_label.is_some(), |this| {
                this.child(
                    h_flex()
                        .h_8()
                        .mt_2()
                        .mb_4()
                        .ml_4()
                        .py_1p5()
                        .child(generating_label.unwrap()),
                )
            })
            .when(show_feedback, move |parent| {
                parent.child(feedback_items).when_some(
                    self.open_feedback_editors.get(&message_id),
                    move |parent, feedback_editor| {
                        let focus_handle = feedback_editor.focus_handle(cx);
                        parent.child(
                            v_flex()
                                .key_context("AgentFeedbackMessageEditor")
                                .on_action(cx.listener(move |this, _: &menu::Cancel, _, cx| {
                                    this.open_feedback_editors.remove(&message_id);
                                    cx.notify();
                                }))
                                .on_action(cx.listener(move |this, _: &menu::Confirm, _, cx| {
                                    this.submit_feedback_message(message_id, cx);
                                    cx.notify();
                                }))
                                .on_action(cx.listener(Self::confirm_editing_message))
                                .mb_2()
                                .mx_4()
                                .p_2()
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .bg(cx.theme().colors().editor_background)
                                .child(feedback_editor.clone())
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .justify_end()
                                        .child(
                                            Button::new("dismiss-feedback-message", "Cancel")
                                                .label_size(LabelSize::Small)
                                                .key_binding(
                                                    KeyBinding::for_action_in(
                                                        &menu::Cancel,
                                                        &focus_handle,
                                                        window,
                                                        cx,
                                                    )
                                                    .map(|kb| kb.size(rems_from_px(10.))),
                                                )
                                                .on_click(cx.listener(
                                                    move |this, _, _window, cx| {
                                                        this.open_feedback_editors
                                                            .remove(&message_id);
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new(
                                                "submit-feedback-message",
                                                "Share Feedback",
                                            )
                                            .style(ButtonStyle::Tinted(ui::TintColor::Accent))
                                            .label_size(LabelSize::Small)
                                            .key_binding(
                                                KeyBinding::for_action_in(
                                                    &menu::Confirm,
                                                    &focus_handle,
                                                    window,
                                                    cx,
                                                )
                                                .map(|kb| kb.size(rems_from_px(10.))),
                                            )
                                            .on_click(
                                                cx.listener(move |this, _, _window, cx| {
                                                    this.submit_feedback_message(message_id, cx);
                                                    cx.notify()
                                                }),
                                            ),
                                        ),
                                ),
                        )
                    },
                )
            })
            .when(after_editing_message, |parent| {
                // Backdrop to dim out the whole thread below the editing user message
                parent.relative().child(
                    div()
                        .occlude()
                        .absolute()
                        .inset_0()
                        .size_full()
                        .bg(panel_background)
                        .opacity(0.8),
                )
            })
            .into_any()
    }

    fn render_message_content(
        &self,
        message_id: MessageId,
        rendered_message: &RenderedMessage,
        has_tool_uses: bool,
        workspace: WeakEntity<Workspace>,
        window: &Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_last_message = self.messages.last() == Some(&message_id);
        let is_generating = self.thread.read(cx).is_generating();
        let pending_thinking_segment_index = if is_generating && is_last_message && !has_tool_uses {
            rendered_message
                .segments
                .iter()
                .enumerate()
                .next_back()
                .filter(|(_, segment)| matches!(segment, RenderedMessageSegment::Thinking { .. }))
                .map(|(index, _)| index)
        } else {
            None
        };

        let message_role = self
            .thread
            .read(cx)
            .message(message_id)
            .map(|m| m.role)
            .unwrap_or(Role::User);

        let is_assistant_message = message_role == Role::Assistant;
        let is_user_message = message_role == Role::User;

        v_flex()
            .text_ui(cx)
            .gap_2()
            .when(is_user_message, |this| this.text_xs())
            .children(
                rendered_message.segments.iter().enumerate().map(
                    |(index, segment)| match segment {
                        RenderedMessageSegment::Thinking {
                            content,
                            scroll_handle,
                        } => self
                            .render_message_thinking_segment(
                                message_id,
                                index,
                                content.clone(),
                                &scroll_handle,
                                Some(index) == pending_thinking_segment_index,
                                window,
                                cx,
                            )
                            .into_any_element(),
                        RenderedMessageSegment::Text(markdown) => {
                            let markdown_element = MarkdownElement::new(
                                markdown.clone(),
                                if is_user_message {
                                    let mut style = default_markdown_style(window, cx);
                                    let mut text_style = window.text_style();
                                    let theme_settings = ThemeSettings::get_global(cx);

                                    let buffer_font = theme_settings.buffer_font.family.clone();
                                    let buffer_font_size = TextSize::Small.rems(cx);

                                    text_style.refine(&TextStyleRefinement {
                                        font_family: Some(buffer_font),
                                        font_size: Some(buffer_font_size.into()),
                                        ..Default::default()
                                    });

                                    style.base_text_style = text_style;
                                    style
                                } else {
                                    default_markdown_style(window, cx)
                                },
                            );

                            let markdown_element = if is_assistant_message {
                                markdown_element.code_block_renderer(
                                    markdown::CodeBlockRenderer::Custom {
                                        render: Arc::new({
                                            let workspace = workspace.clone();
                                            let active_thread = cx.entity();
                                            move |kind,
                                                  parsed_markdown,
                                                  range,
                                                  metadata,
                                                  window,
                                                  cx| {
                                                render_markdown_code_block(
                                                    message_id,
                                                    range.start,
                                                    kind,
                                                    parsed_markdown,
                                                    metadata,
                                                    active_thread.clone(),
                                                    workspace.clone(),
                                                    window,
                                                    cx,
                                                )
                                            }
                                        }),
                                        transform: Some(Arc::new({
                                            let active_thread = cx.entity();
                                            move |el, range, metadata, _, cx| {
                                                let is_expanded = active_thread
                                                    .read(cx)
                                                    .expanded_code_blocks
                                                    .get(&(message_id, range.start))
                                                    .copied()
                                                    .unwrap_or(false);

                                                if is_expanded
                                                    || metadata.line_count
                                                        <= MAX_UNCOLLAPSED_LINES_IN_CODE_BLOCK
                                                {
                                                    return el;
                                                }
                                                el.child(
                                                    div()
                                                        .absolute()
                                                        .bottom_0()
                                                        .left_0()
                                                        .w_full()
                                                        .h_1_4()
                                                        .rounded_b_lg()
                                                        .bg(gpui::linear_gradient(
                                                            0.,
                                                            gpui::linear_color_stop(
                                                                cx.theme()
                                                                    .colors()
                                                                    .editor_background,
                                                                0.,
                                                            ),
                                                            gpui::linear_color_stop(
                                                                cx.theme()
                                                                    .colors()
                                                                    .editor_background
                                                                    .opacity(0.),
                                                                1.,
                                                            ),
                                                        )),
                                                )
                                            }
                                        })),
                                    },
                                )
                            } else {
                                markdown_element.code_block_renderer(
                                    markdown::CodeBlockRenderer::Default {
                                        copy_button: false,
                                        border: true,
                                    },
                                )
                            };

                            div()
                                .child(markdown_element.on_url_click({
                                    let workspace = self.workspace.clone();
                                    move |text, window, cx| {
                                        open_markdown_link(text, workspace.clone(), window, cx);
                                    }
                                }))
                                .into_any_element()
                        }
                    },
                ),
            )
    }

    fn tool_card_border_color(&self, cx: &Context<Self>) -> Hsla {
        cx.theme().colors().border.opacity(0.5)
    }

    fn tool_card_header_bg(&self, cx: &Context<Self>) -> Hsla {
        cx.theme()
            .colors()
            .element_background
            .blend(cx.theme().colors().editor_foreground.opacity(0.025))
    }

    fn render_message_thinking_segment(
        &self,
        message_id: MessageId,
        ix: usize,
        markdown: Entity<Markdown>,
        scroll_handle: &ScrollHandle,
        pending: bool,
        window: &Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let is_open = self
            .expanded_thinking_segments
            .get(&(message_id, ix))
            .copied()
            .unwrap_or_default();

        let editor_bg = cx.theme().colors().panel_background;

        div().map(|this| {
            if pending {
                this.v_flex()
                    .mt_neg_2()
                    .mb_1p5()
                    .child(
                        h_flex()
                            .group("disclosure-header")
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_1p5()
                                    .child(
                                        Icon::new(IconName::LightBulb)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(AnimatedLabel::new("Thinking").size(LabelSize::Small)),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        div().visible_on_hover("disclosure-header").child(
                                            Disclosure::new("thinking-disclosure", is_open)
                                                .opened_icon(IconName::ChevronUp)
                                                .closed_icon(IconName::ChevronDown)
                                                .on_click(cx.listener({
                                                    move |this, _event, _window, _cx| {
                                                        let is_open = this
                                                            .expanded_thinking_segments
                                                            .entry((message_id, ix))
                                                            .or_insert(false);

                                                        *is_open = !*is_open;
                                                    }
                                                })),
                                        ),
                                    )
                                    .child({
                                        Icon::new(IconName::ArrowCircle)
                                            .color(Color::Accent)
                                            .size(IconSize::Small)
                                            .with_animation(
                                                "arrow-circle",
                                                Animation::new(Duration::from_secs(2)).repeat(),
                                                |icon, delta| {
                                                    icon.transform(Transformation::rotate(
                                                        percentage(delta),
                                                    ))
                                                },
                                            )
                                    }),
                            ),
                    )
                    .when(!is_open, |this| {
                        let gradient_overlay = div()
                            .rounded_b_lg()
                            .h_full()
                            .absolute()
                            .w_full()
                            .bottom_0()
                            .left_0()
                            .bg(linear_gradient(
                                180.,
                                linear_color_stop(editor_bg, 1.),
                                linear_color_stop(editor_bg.opacity(0.2), 0.),
                            ));

                        this.child(
                            div()
                                .relative()
                                .bg(editor_bg)
                                .rounded_b_lg()
                                .mt_2()
                                .pl_4()
                                .child(
                                    div()
                                        .id(("thinking-content", ix))
                                        .max_h_20()
                                        .track_scroll(scroll_handle)
                                        .text_ui_sm(cx)
                                        .overflow_hidden()
                                        .child(
                                            MarkdownElement::new(
                                                markdown.clone(),
                                                default_markdown_style(window, cx),
                                            )
                                            .on_url_click({
                                                let workspace = self.workspace.clone();
                                                move |text, window, cx| {
                                                    open_markdown_link(
                                                        text,
                                                        workspace.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            }),
                                        ),
                                )
                                .child(gradient_overlay),
                        )
                    })
                    .when(is_open, |this| {
                        this.child(
                            div()
                                .id(("thinking-content", ix))
                                .h_full()
                                .bg(editor_bg)
                                .text_ui_sm(cx)
                                .child(
                                    MarkdownElement::new(
                                        markdown.clone(),
                                        default_markdown_style(window, cx),
                                    )
                                    .on_url_click({
                                        let workspace = self.workspace.clone();
                                        move |text, window, cx| {
                                            open_markdown_link(text, workspace.clone(), window, cx);
                                        }
                                    }),
                                ),
                        )
                    })
            } else {
                this.v_flex()
                    .mt_neg_2()
                    .child(
                        h_flex()
                            .group("disclosure-header")
                            .pr_1()
                            .justify_between()
                            .opacity(0.8)
                            .hover(|style| style.opacity(1.))
                            .child(
                                h_flex()
                                    .gap_1p5()
                                    .child(
                                        Icon::new(IconName::LightBulb)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(Label::new("Thought Process").size(LabelSize::Small)),
                            )
                            .child(
                                div().visible_on_hover("disclosure-header").child(
                                    Disclosure::new("thinking-disclosure", is_open)
                                        .opened_icon(IconName::ChevronUp)
                                        .closed_icon(IconName::ChevronDown)
                                        .on_click(cx.listener({
                                            move |this, _event, _window, _cx| {
                                                let is_open = this
                                                    .expanded_thinking_segments
                                                    .entry((message_id, ix))
                                                    .or_insert(false);

                                                *is_open = !*is_open;
                                            }
                                        })),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .id(("thinking-content", ix))
                            .relative()
                            .mt_1p5()
                            .ml_1p5()
                            .pl_2p5()
                            .border_l_1()
                            .border_color(cx.theme().colors().border_variant)
                            .text_ui_sm(cx)
                            .when(is_open, |this| {
                                this.child(
                                    MarkdownElement::new(
                                        markdown.clone(),
                                        default_markdown_style(window, cx),
                                    )
                                    .on_url_click({
                                        let workspace = self.workspace.clone();
                                        move |text, window, cx| {
                                            open_markdown_link(text, workspace.clone(), window, cx);
                                        }
                                    }),
                                )
                            }),
                    )
            }
        })
    }

    fn render_tool_use(
        &self,
        tool_use: ToolUse,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        if let Some(card) = self.thread.read(cx).card_for_tool(&tool_use.id) {
            return card.render(&tool_use.status, window, cx);
        }

        let is_open = self
            .expanded_tool_uses
            .get(&tool_use.id)
            .copied()
            .unwrap_or_default();

        let is_status_finished = matches!(&tool_use.status, ToolUseStatus::Finished(_));

        let fs = self
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).app_state().fs.clone());
        let needs_confirmation = matches!(&tool_use.status, ToolUseStatus::NeedsConfirmation);
        let needs_confirmation_tools = tool_use.needs_confirmation;

        let status_icons = div().child(match &tool_use.status {
            ToolUseStatus::NeedsConfirmation => {
                let icon = Icon::new(IconName::Warning)
                    .color(Color::Warning)
                    .size(IconSize::Small);
                icon.into_any_element()
            }
            ToolUseStatus::Pending
            | ToolUseStatus::InputStillStreaming
            | ToolUseStatus::Running => {
                let icon = Icon::new(IconName::ArrowCircle)
                    .color(Color::Accent)
                    .size(IconSize::Small);
                icon.with_animation(
                    "arrow-circle",
                    Animation::new(Duration::from_secs(2)).repeat(),
                    |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
                )
                .into_any_element()
            }
            ToolUseStatus::Finished(_) => div().w_0().into_any_element(),
            ToolUseStatus::Error(_) => {
                let icon = Icon::new(IconName::Close)
                    .color(Color::Error)
                    .size(IconSize::Small);
                icon.into_any_element()
            }
        });

        let rendered_tool_use = self.rendered_tool_uses.get(&tool_use.id).cloned();
        let results_content_container = || v_flex().p_2().gap_0p5();

        let results_content = v_flex()
            .gap_1()
            .child(
                results_content_container()
                    .child(
                        Label::new("Input")
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                            .buffer_font(cx),
                    )
                    .child(
                        div()
                            .w_full()
                            .text_ui_sm(cx)
                            .children(rendered_tool_use.as_ref().map(|rendered| {
                                MarkdownElement::new(
                                    rendered.input.clone(),
                                    tool_use_markdown_style(window, cx),
                                )
                                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                    copy_button: false,
                                    border: false,
                                })
                                .on_url_click({
                                    let workspace = self.workspace.clone();
                                    move |text, window, cx| {
                                        open_markdown_link(text, workspace.clone(), window, cx);
                                    }
                                })
                            })),
                    ),
            )
            .map(|container| match tool_use.status {
                ToolUseStatus::Finished(_) => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Result")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        )
                        .child(div().w_full().text_ui_sm(cx).children(
                            rendered_tool_use.as_ref().map(|rendered| {
                                MarkdownElement::new(
                                    rendered.output.clone(),
                                    tool_use_markdown_style(window, cx),
                                )
                                .code_block_renderer(markdown::CodeBlockRenderer::Default {
                                    copy_button: false,
                                    border: false,
                                })
                                .on_url_click({
                                    let workspace = self.workspace.clone();
                                    move |text, window, cx| {
                                        open_markdown_link(text, workspace.clone(), window, cx);
                                    }
                                })
                                .into_any_element()
                            }),
                        )),
                ),
                ToolUseStatus::InputStillStreaming | ToolUseStatus::Running => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Icon::new(IconName::ArrowCircle)
                                        .size(IconSize::Small)
                                        .color(Color::Accent)
                                        .with_animation(
                                            "arrow-circle",
                                            Animation::new(Duration::from_secs(2)).repeat(),
                                            |icon, delta| {
                                                icon.transform(Transformation::rotate(percentage(
                                                    delta,
                                                )))
                                            },
                                        ),
                                )
                                .child(
                                    Label::new("Running…")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted)
                                        .buffer_font(cx),
                                ),
                        ),
                ),
                ToolUseStatus::Error(_) => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Error")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        )
                        .child(
                            div()
                                .text_ui_sm(cx)
                                .children(rendered_tool_use.as_ref().map(|rendered| {
                                    MarkdownElement::new(
                                        rendered.output.clone(),
                                        tool_use_markdown_style(window, cx),
                                    )
                                    .on_url_click({
                                        let workspace = self.workspace.clone();
                                        move |text, window, cx| {
                                            open_markdown_link(text, workspace.clone(), window, cx);
                                        }
                                    })
                                    .into_any_element()
                                })),
                        ),
                ),
                ToolUseStatus::Pending => container,
                ToolUseStatus::NeedsConfirmation => container.child(
                    results_content_container()
                        .border_t_1()
                        .border_color(self.tool_card_border_color(cx))
                        .child(
                            Label::new("Asking Permission")
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .buffer_font(cx),
                        ),
                ),
            });

        let gradient_overlay = |color: Hsla| {
            div()
                .h_full()
                .absolute()
                .w_12()
                .bottom_0()
                .map(|element| {
                    if is_status_finished {
                        element.right_6()
                    } else {
                        element.right(px(44.))
                    }
                })
                .bg(linear_gradient(
                    90.,
                    linear_color_stop(color, 1.),
                    linear_color_stop(color.opacity(0.2), 0.),
                ))
        };

        v_flex().gap_1().mb_2().map(|element| {
            if !needs_confirmation_tools {
                element.child(
                    v_flex()
                        .child(
                            h_flex()
                                .group("disclosure-header")
                                .relative()
                                .gap_1p5()
                                .justify_between()
                                .opacity(0.8)
                                .hover(|style| style.opacity(1.))
                                .when(!is_status_finished, |this| this.pr_2())
                                .child(
                                    h_flex()
                                        .id("tool-label-container")
                                        .gap_1p5()
                                        .max_w_full()
                                        .overflow_x_scroll()
                                        .child(
                                            Icon::new(tool_use.icon)
                                                .size(IconSize::XSmall)
                                                .color(Color::Muted),
                                        )
                                        .child(
                                            h_flex().pr_8().text_size(rems(0.8125)).children(
                                                rendered_tool_use.map(|rendered| MarkdownElement::new(rendered.label, tool_use_markdown_style(window, cx)).on_url_click({let workspace = self.workspace.clone(); move |text, window, cx| {
                                                    open_markdown_link(text, workspace.clone(), window, cx);
                                                }}))
                                            ),
                                        ),
                                )
                                .child(
                                    h_flex()
                                        .gap_1()
                                        .child(
                                            div().visible_on_hover("disclosure-header").child(
                                                Disclosure::new("tool-use-disclosure", is_open)
                                                    .opened_icon(IconName::ChevronUp)
                                                    .closed_icon(IconName::ChevronDown)
                                                    .on_click(cx.listener({
                                                        let tool_use_id = tool_use.id.clone();
                                                        move |this, _event, _window, _cx| {
                                                            let is_open = this
                                                                .expanded_tool_uses
                                                                .entry(tool_use_id.clone())
                                                                .or_insert(false);

                                                            *is_open = !*is_open;
                                                        }
                                                    })),
                                            ),
                                        )
                                        .child(status_icons),
                                )
                                .child(gradient_overlay(cx.theme().colors().panel_background)),
                        )
                        .map(|parent| {
                            if !is_open {
                                return parent;
                            }

                            parent.child(
                                v_flex()
                                    .mt_1()
                                    .border_1()
                                    .border_color(self.tool_card_border_color(cx))
                                    .bg(cx.theme().colors().editor_background)
                                    .rounded_lg()
                                    .child(results_content),
                            )
                        }),
                )
            } else {
                v_flex()
                    .mb_2()
                    .rounded_lg()
                    .border_1()
                    .border_color(self.tool_card_border_color(cx))
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .group("disclosure-header")
                            .relative()
                            .justify_between()
                            .py_1()
                            .map(|element| {
                                if is_status_finished {
                                    element.pl_2().pr_0p5()
                                } else {
                                    element.px_2()
                                }
                            })
                            .bg(self.tool_card_header_bg(cx))
                            .map(|element| {
                                if is_open {
                                    element.border_b_1().rounded_t_md()
                                } else if needs_confirmation {
                                    element.rounded_t_md()
                                } else {
                                    element.rounded_md()
                                }
                            })
                            .border_color(self.tool_card_border_color(cx))
                            .child(
                                h_flex()
                                    .id("tool-label-container")
                                    .gap_1p5()
                                    .max_w_full()
                                    .overflow_x_scroll()
                                    .child(
                                        Icon::new(tool_use.icon)
                                            .size(IconSize::XSmall)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        h_flex().pr_8().text_ui_sm(cx).children(
                                            rendered_tool_use.map(|rendered| MarkdownElement::new(rendered.label, tool_use_markdown_style(window, cx)).on_url_click({let workspace = self.workspace.clone(); move |text, window, cx| {
                                                open_markdown_link(text, workspace.clone(), window, cx);
                                            }}))
                                        ),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        div().visible_on_hover("disclosure-header").child(
                                            Disclosure::new("tool-use-disclosure", is_open)
                                                .opened_icon(IconName::ChevronUp)
                                                .closed_icon(IconName::ChevronDown)
                                                .on_click(cx.listener({
                                                    let tool_use_id = tool_use.id.clone();
                                                    move |this, _event, _window, _cx| {
                                                        let is_open = this
                                                            .expanded_tool_uses
                                                            .entry(tool_use_id.clone())
                                                            .or_insert(false);

                                                        *is_open = !*is_open;
                                                    }
                                                })),
                                        ),
                                    )
                                    .child(status_icons),
                            )
                            .child(gradient_overlay(self.tool_card_header_bg(cx))),
                    )
                    .map(|parent| {
                        if !is_open {
                            return parent;
                        }

                        parent.child(
                            v_flex()
                                .bg(cx.theme().colors().editor_background)
                                .map(|element| {
                                    if  needs_confirmation {
                                        element.rounded_none()
                                    } else {
                                        element.rounded_b_lg()
                                    }
                                })
                                .child(results_content),
                        )
                    })
                    .when(needs_confirmation, |this| {
                        this.child(
                            h_flex()
                                .py_1()
                                .pl_2()
                                .pr_1()
                                .gap_1()
                                .justify_between()
                                .bg(cx.theme().colors().editor_background)
                                .border_t_1()
                                .border_color(self.tool_card_border_color(cx))
                                .rounded_b_lg()
                                .child(
                                    AnimatedLabel::new("Waiting for Confirmation").size(LabelSize::Small)
                                )
                                .child(
                                    h_flex()
                                        .gap_0p5()
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            Button::new(
                                                "always-allow-tool-action",
                                                "Always Allow",
                                            )
                                            .label_size(LabelSize::Small)
                                            .icon(IconName::CheckDouble)
                                            .icon_position(IconPosition::Start)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Success)
                                            .tooltip(move |window, cx|  {
                                                Tooltip::with_meta(
                                                    "Never ask for permission",
                                                    None,
                                                    "Restore the original behavior in your Agent Panel settings",
                                                    window,
                                                    cx,
                                                )
                                            })
                                            .on_click(cx.listener(
                                                move |this, event, window, cx| {
                                                    if let Some(fs) = fs.clone() {
                                                        update_settings_file::<AssistantSettings>(
                                                            fs.clone(),
                                                            cx,
                                                            |settings, _| {
                                                                settings.set_always_allow_tool_actions(true);
                                                            },
                                                        );
                                                    }
                                                    this.handle_allow_tool(
                                                        tool_id.clone(),
                                                        event,
                                                        window,
                                                        cx,
                                                    )
                                                },
                                            ))
                                        })
                                        .child(ui::Divider::vertical())
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            Button::new("allow-tool-action", "Allow")
                                                .label_size(LabelSize::Small)
                                                .icon(IconName::Check)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Success)
                                                .on_click(cx.listener(
                                                    move |this, event, window, cx| {
                                                        this.handle_allow_tool(
                                                            tool_id.clone(),
                                                            event,
                                                            window,
                                                            cx,
                                                        )
                                                    },
                                                ))
                                        })
                                        .child({
                                            let tool_id = tool_use.id.clone();
                                            let tool_name: Arc<str> = tool_use.name.into();
                                            Button::new("deny-tool", "Deny")
                                                .label_size(LabelSize::Small)
                                                .icon(IconName::Close)
                                                .icon_position(IconPosition::Start)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Error)
                                                .on_click(cx.listener(
                                                    move |this, event, window, cx| {
                                                        this.handle_deny_tool(
                                                            tool_id.clone(),
                                                            tool_name.clone(),
                                                            event,
                                                            window,
                                                            cx,
                                                        )
                                                    },
                                                ))
                                        }),
                                ),
                        )
                    })
            }
        }).into_any_element()
    }

    fn render_rules_item(&self, cx: &Context<Self>) -> AnyElement {
        let project_context = self.thread.read(cx).project_context();
        let project_context = project_context.borrow();
        let Some(project_context) = project_context.as_ref() else {
            return div().into_any();
        };

        let user_rules_text = if project_context.user_rules.is_empty() {
            None
        } else if project_context.user_rules.len() == 1 {
            let user_rules = &project_context.user_rules[0];

            match user_rules.title.as_ref() {
                Some(title) => Some(format!("Using \"{title}\" user rule")),
                None => Some("Using user rule".into()),
            }
        } else {
            Some(format!(
                "Using {} user rules",
                project_context.user_rules.len()
            ))
        };

        let first_user_rules_id = project_context
            .user_rules
            .first()
            .map(|user_rules| user_rules.uuid.0);

        let rules_files = project_context
            .worktrees
            .iter()
            .filter_map(|worktree| worktree.rules_file.as_ref())
            .collect::<Vec<_>>();

        let rules_file_text = match rules_files.as_slice() {
            &[] => None,
            &[rules_file] => Some(format!(
                "Using project {:?} file",
                rules_file.path_in_worktree
            )),
            rules_files => Some(format!("Using {} project rules files", rules_files.len())),
        };

        if user_rules_text.is_none() && rules_file_text.is_none() {
            return div().into_any();
        }

        v_flex()
            .pt_2()
            .px_2p5()
            .gap_1()
            .when_some(user_rules_text, |parent, user_rules_text| {
                parent.child(
                    h_flex()
                        .w_full()
                        .child(
                            Icon::new(RULES_ICON)
                                .size(IconSize::XSmall)
                                .color(Color::Disabled),
                        )
                        .child(
                            Label::new(user_rules_text)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .truncate()
                                .buffer_font(cx)
                                .ml_1p5()
                                .mr_0p5(),
                        )
                        .child(
                            IconButton::new("open-prompt-library", IconName::ArrowUpRightAlt)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                // TODO: Figure out a way to pass focus handle here so we can display the `OpenPromptLibrary`  keybinding
                                .tooltip(Tooltip::text("View User Rules"))
                                .on_click(move |_event, window, cx| {
                                    window.dispatch_action(
                                        Box::new(OpenPromptLibrary {
                                            prompt_to_select: first_user_rules_id,
                                        }),
                                        cx,
                                    )
                                }),
                        ),
                )
            })
            .when_some(rules_file_text, |parent, rules_file_text| {
                parent.child(
                    h_flex()
                        .w_full()
                        .child(
                            Icon::new(IconName::File)
                                .size(IconSize::XSmall)
                                .color(Color::Disabled),
                        )
                        .child(
                            Label::new(rules_file_text)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .buffer_font(cx)
                                .ml_1p5()
                                .mr_0p5(),
                        )
                        .child(
                            IconButton::new("open-rule", IconName::ArrowUpRightAlt)
                                .shape(ui::IconButtonShape::Square)
                                .icon_size(IconSize::XSmall)
                                .icon_color(Color::Ignored)
                                .on_click(cx.listener(Self::handle_open_rules))
                                .tooltip(Tooltip::text("View Rules")),
                        ),
                )
            })
            .into_any()
    }

    fn handle_allow_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(PendingToolUseStatus::NeedsConfirmation(c)) = self
            .thread
            .read(cx)
            .pending_tool(&tool_use_id)
            .map(|tool_use| tool_use.status.clone())
        {
            self.thread.update(cx, |thread, cx| {
                thread.run_tool(
                    c.tool_use_id.clone(),
                    c.ui_text.clone(),
                    c.input.clone(),
                    &c.messages,
                    c.tool.clone(),
                    cx,
                );
            });
        }
    }

    fn handle_deny_tool(
        &mut self,
        tool_use_id: LanguageModelToolUseId,
        tool_name: Arc<str>,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.thread.update(cx, |thread, cx| {
            thread.deny_tool_use(tool_use_id, tool_name, cx);
        });
    }

    fn handle_open_rules(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let project_context = self.thread.read(cx).project_context();
        let project_context = project_context.borrow();
        let Some(project_context) = project_context.as_ref() else {
            return;
        };

        let abs_paths = project_context
            .worktrees
            .iter()
            .flat_map(|worktree| worktree.rules_file.as_ref())
            .map(|rules_file| rules_file.abs_path.to_path_buf())
            .collect::<Vec<_>>();

        if let Ok(task) = self.workspace.update(cx, move |workspace, cx| {
            // TODO: Open a multibuffer instead? In some cases this doesn't make the set of rules
            // files clear. For example, if rules file 1 is already open but rules file 2 is not,
            // this would open and focus rules file 2 in a tab that is not next to rules file 1.
            workspace.open_paths(abs_paths, OpenOptions::default(), None, window, cx)
        }) {
            task.detach();
        }
    }

    fn dismiss_notifications(&mut self, cx: &mut Context<ActiveThread>) {
        for window in self.notifications.drain(..) {
            window
                .update(cx, |_, window, _| {
                    window.remove_window();
                })
                .ok();

            self.notification_subscriptions.remove(&window);
        }
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !self.show_scrollbar && !self.scrollbar_state.is_dragging() {
            return None;
        }

        Some(
            div()
                .occlude()
                .id("active-thread-scrollbar")
                .on_mouse_move(cx.listener(|_, _, _, cx| {
                    cx.notify();
                    cx.stop_propagation()
                }))
                .on_hover(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_any_mouse_down(|_, _, cx| {
                    cx.stop_propagation();
                })
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|_, _, _, cx| {
                        cx.stop_propagation();
                    }),
                )
                .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                    cx.notify();
                }))
                .h_full()
                .absolute()
                .right_1()
                .top_1()
                .bottom_0()
                .w(px(12.))
                .cursor_default()
                .children(Scrollbar::vertical(self.scrollbar_state.clone())),
        )
    }

    fn hide_scrollbar_later(&mut self, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        self.hide_scrollbar_task = Some(cx.spawn(async move |thread, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            thread
                .update(cx, |thread, cx| {
                    if !thread.scrollbar_state.is_dragging() {
                        thread.show_scrollbar = false;
                        cx.notify();
                    }
                })
                .log_err();
        }))
    }
}

pub enum ActiveThreadEvent {
    EditingMessageTokenCountChanged,
}

impl EventEmitter<ActiveThreadEvent> for ActiveThread {}

impl Render for ActiveThread {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .relative()
            .on_mouse_move(cx.listener(|this, _, _, cx| {
                this.show_scrollbar = true;
                this.hide_scrollbar_later(cx);
                cx.notify();
            }))
            .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                this.show_scrollbar = true;
                this.hide_scrollbar_later(cx);
                cx.notify();
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.hide_scrollbar_later(cx);
                }),
            )
            .child(list(self.list_state.clone()).flex_grow())
            .when_some(self.render_vertical_scrollbar(cx), |this, scrollbar| {
                this.child(scrollbar)
            })
    }
}

pub(crate) fn open_context(
    context: &AssistantContext,
    context_store: Entity<ContextStore>,
    workspace: Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    match context {
        AssistantContext::File(file_context) => {
            let project = workspace.read(cx).project().read(cx);
            if let Some(project_path) = file_context.project_path(project, cx) {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(project_path, None, true, window, cx)
                        .detach_and_log_err(cx);
                });
            }
        }
        AssistantContext::Directory(directory_context) => {
            let entry_id = directory_context.entry_id;
            workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |_project, cx| {
                    cx.emit(project::Event::RevealInProjectPanel(entry_id));
                })
            })
        } /*
          AssistantContext::Symbol(symbol_context) => {
              if let Some(project_path) = symbol_context
                  .context_symbol
                  .buffer
                  .read(cx)
                  .project_path(cx)
              {
                  let snapshot = symbol_context.context_symbol.buffer.read(cx).snapshot();
                  let target_position = symbol_context
                      .context_symbol
                      .id
                      .range
                      .start
                      .to_point(&snapshot);

                open_editor_at_position(project_path, target_position, &workspace, window, cx)
                    .detach();
            }
        }
        AssistantContext::Selection(selection_context) => {
            if let Some(project_path) = selection_context
                .context_buffer
                .buffer
                .read(cx)
                .project_path(cx)
            {
                let snapshot = selection_context.context_buffer.buffer.read(cx).snapshot();
                let target_position = selection_context.range.start.to_point(&snapshot);

                open_editor_at_position(project_path, target_position, &workspace, window, cx)
                    .detach();
            }
        }
        AssistantContext::FetchedUrl(fetched_url_context) => {
            cx.open_url(&fetched_url_context.url);
        }
        AssistantContext::Thread(thread_context) => {
            let thread_id = thread_context.thread.read(cx).id().clone();
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel
                            .open_thread(&thread_id, window, cx)
                            .detach_and_log_err(cx)
                    });
                }
            })
        }
        AssistantContext::Rules(rules_context) => window.dispatch_action(
            Box::new(OpenPromptLibrary {
                prompt_to_select: Some(rules_context.prompt_id.0),
            }),
            cx,
        ),
        AssistantContext::Image(_) => {}
        */
    }
}

fn open_editor_at_position(
    project_path: project::ProjectPath,
    target_position: Point,
    workspace: &Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Task<()> {
    let open_task = workspace.update(cx, |workspace, cx| {
        workspace.open_path(project_path, None, true, window, cx)
    });
    window.spawn(cx, async move |cx| {
        if let Some(active_editor) = open_task
            .await
            .log_err()
            .and_then(|item| item.downcast::<Editor>())
        {
            active_editor
                .downgrade()
                .update_in(cx, |editor, window, cx| {
                    editor.go_to_singleton_buffer_point(target_position, window, cx);
                })
                .log_err();
        }
    })
}
