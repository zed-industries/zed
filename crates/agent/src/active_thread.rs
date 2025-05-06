use crate::AssistantPanel;
use crate::context::{AgentContextHandle, RULES_ICON};
use crate::context_picker::{ContextPicker, MentionLink};
use crate::context_store::ContextStore;
use crate::context_strip::{ContextStrip, ContextStripEvent, SuggestContextKind};
use crate::thread::{
    LastRestoreCheckpoint, MessageId, MessageSegment, Thread, ThreadError, ThreadEvent,
    ThreadFeedback,
};
use crate::thread_store::{RulesLoadingError, TextThreadStore, ThreadStore};
use crate::tool_use::{PendingToolUseStatus, ToolUse};
use crate::ui::{
    AddedContext, AgentNotification, AgentNotificationEvent, AnimatedLabel, ContextPill,
};
use anyhow::Context as _;
use assistant_settings::{AssistantSettings, NotifyWhenAgentWaiting};
use assistant_tool::ToolUseStatus;
use collections::{HashMap, HashSet};
use editor::actions::{MoveUp, Paste};
use editor::scroll::Autoscroll;
use editor::{Editor, EditorElement, EditorEvent, EditorStyle, MultiBuffer};
use gpui::{
    AbsoluteLength, Animation, AnimationExt, AnyElement, App, ClickEvent, ClipboardEntry,
    ClipboardItem, DefiniteLength, EdgesRefinement, Empty, Entity, EventEmitter, Focusable, Hsla,
    ListAlignment, ListState, MouseButton, PlatformDisplay, ScrollHandle, Stateful,
    StyleRefinement, Subscription, Task, TextStyle, TextStyleRefinement, Transformation,
    UnderlineStyle, WeakEntity, WindowHandle, linear_color_stop, linear_gradient, list, percentage,
    pulsating_between,
};
use language::{Buffer, Language, LanguageRegistry};
use language_model::{
    LanguageModelRequestMessage, LanguageModelToolUseId, MessageContent, Role, StopReason,
};
use markdown::parser::{CodeBlockKind, CodeBlockMetadata};
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle, ParsedMarkdown};
use project::{ProjectEntryId, ProjectItem as _};
use rope::Point;
use settings::{Settings as _, SettingsStore, update_settings_file};
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use text::ToPoint;
use theme::ThemeSettings;
use ui::{
    Disclosure, IconButton, KeyBinding, PopoverMenuHandle, Scrollbar, ScrollbarState, TextSize,
    Tooltip, prelude::*,
};
use util::ResultExt as _;
use util::markdown::MarkdownCodeBlock;
use workspace::Workspace;
use zed_actions::assistant::OpenRulesLibrary;

pub struct ActiveThread {
    context_store: Entity<ContextStore>,
    language_registry: Arc<LanguageRegistry>,
    thread_store: Entity<ThreadStore>,
    text_thread_store: Entity<TextThreadStore>,
    thread: Entity<Thread>,
    workspace: WeakEntity<Workspace>,
    save_thread_task: Option<Task<()>>,
    messages: Vec<MessageId>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    show_scrollbar: bool,
    hide_scrollbar_task: Option<Task<()>>,
    rendered_messages_by_id: HashMap<MessageId, RenderedMessage>,
    rendered_tool_uses: HashMap<LanguageModelToolUseId, RenderedToolUse>,
    editing_message: Option<(MessageId, EditingMessageState)>,
    expanded_tool_uses: HashMap<LanguageModelToolUseId, bool>,
    expanded_thinking_segments: HashMap<(MessageId, usize), bool>,
    expanded_code_blocks: HashMap<(MessageId, usize), bool>,
    last_error: Option<ThreadError>,
    notifications: Vec<WindowHandle<AgentNotification>>,
    copied_code_block_ids: HashSet<(MessageId, usize)>,
    _subscriptions: Vec<Subscription>,
    notification_subscriptions: HashMap<WindowHandle<AgentNotification>, Vec<Subscription>>,
    open_feedback_editors: HashMap<MessageId, Entity<Editor>>,
    _load_edited_message_context_task: Option<Task<()>>,
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

pub(crate) fn default_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
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
        CodeBlockKind::FencedLang(raw_language_name) => Some(render_code_language(
            parsed_markdown.languages_by_name.get(raw_language_name),
            raw_language_name.clone(),
            cx,
        )),
        CodeBlockKind::FencedSrc(path_range) => path_range.path.file_name().map(|file_name| {
            // We tell the model to use /dev/null for the path instead of using ```language
            // because otherwise it consistently fails to use code citations.
            if path_range.path.starts_with("/dev/null") {
                let ext = path_range
                    .path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(|str| SharedString::new(str.to_string()))
                    .unwrap_or_default();

                render_code_language(
                    parsed_markdown
                        .languages_by_path
                        .get(&path_range.path)
                        .or_else(|| parsed_markdown.languages_by_name.get(&ext)),
                    ext,
                    cx,
                )
            } else {
                let content = if let Some(parent) = path_range.path.parent() {
                    h_flex()
                        .ml_1()
                        .gap_1()
                        .child(
                            Label::new(file_name.to_string_lossy().to_string())
                                .size(LabelSize::Small),
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
                                        let Some(project_path) = workspace
                                            .project()
                                            .read(cx)
                                            .find_project_path(&path_range.path, cx)
                                        else {
                                            return;
                                        };
                                        let Some(target) = path_range.range.as_ref().map(|range| {
                                            Point::new(
                                                // Line number is 1-based
                                                range.start.line.saturating_sub(1),
                                                range.start.col.unwrap_or(0),
                                            )
                                        }) else {
                                            return;
                                        };
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
                                                if let Some(active_editor) =
                                                    item.downcast::<Editor>()
                                                {
                                                    active_editor
                                                        .update_in(cx, |editor, window, cx| {
                                                            editor.go_to_singleton_buffer_point(
                                                                target, window, cx,
                                                            );
                                                        })
                                                        .ok();
                                                }
                                                anyhow::Ok(())
                                            })
                                            .detach_and_log_err(cx);
                                    }
                                })
                                .ok();
                        }
                    })
                    .into_any_element()
            }
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
        .unwrap_or(true);

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
                .visible_on_hover("codeblock_container")
                .gap_1()
                .child(
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

                                let code =
                                    parsed_markdown.source()[code_block_range.clone()].to_string();
                                cx.write_to_clipboard(ClipboardItem::new_string(code));

                                cx.spawn(async move |this, cx| {
                                    cx.background_executor().timer(Duration::from_secs(2)).await;

                                    cx.update(|cx| {
                                        this.update(cx, |this, cx| {
                                            this.copied_code_block_ids.remove(&(message_id, ix));
                                            cx.notify();
                                        })
                                    })
                                    .ok();
                                })
                                .detach();
                            });
                        }
                    }),
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
                                            .or_insert(true);
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

fn render_code_language(
    language: Option<&Arc<Language>>,
    name_fallback: SharedString,
    cx: &App,
) -> AnyElement {
    let icon_path = language.and_then(|language| {
        language
            .config()
            .matcher
            .path_suffixes
            .iter()
            .find_map(|extension| file_icons::FileIcons::get_icon(Path::new(extension), cx))
            .map(Icon::from_path)
    });

    let language_label = language
        .map(|language| language.name().into())
        .unwrap_or(name_fallback);

    h_flex()
        .gap_1()
        .children(icon_path.map(|icon| icon.color(Color::Muted).size(IconSize::Small)))
        .child(Label::new(language_label).size(LabelSize::Small))
        .into_any_element()
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
                        .open_thread_by_id(&thread_id, window, cx)
                        .detach_and_log_err(cx)
                });
            }
        }),
        Some(MentionLink::TextThread(path)) => workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel
                        .open_saved_prompt_editor(path, window, cx)
                        .detach_and_log_err(cx);
                });
            }
        }),
        Some(MentionLink::Fetch(url)) => cx.open_url(&url),
        Some(MentionLink::Rule(prompt_id)) => window.dispatch_action(
            Box::new(OpenRulesLibrary {
                prompt_to_select: Some(prompt_id.0),
            }),
            cx,
        ),
        None => cx.open_url(&text),
    }
}

struct EditingMessageState {
    editor: Entity<Editor>,
    context_strip: Entity<ContextStrip>,
    context_picker_menu_handle: PopoverMenuHandle<ContextPicker>,
    last_estimated_token_count: Option<usize>,
    _subscriptions: [Subscription; 2],
    _update_token_count_task: Option<Task<()>>,
}

impl ActiveThread {
    pub fn new(
        thread: Entity<Thread>,
        thread_store: Entity<ThreadStore>,
        text_thread_store: Entity<TextThreadStore>,
        context_store: Entity<ContextStore>,
        language_registry: Arc<LanguageRegistry>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscriptions = vec![
            cx.observe(&thread, |_, _, cx| cx.notify()),
            cx.subscribe_in(&thread, window, Self::handle_thread_event),
            cx.subscribe(&thread_store, Self::handle_rules_loading_error),
            cx.observe_global::<SettingsStore>(|_, cx| cx.notify()),
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
            text_thread_store,
            context_store,
            thread: thread.clone(),
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
            copied_code_block_ids: HashSet::default(),
            notifications: Vec::new(),
            _subscriptions: subscriptions,
            notification_subscriptions: HashMap::default(),
            open_feedback_editors: HashMap::default(),
            _load_edited_message_context_task: None,
        };

        for message in thread.read(cx).messages().cloned().collect::<Vec<_>>() {
            this.push_message(&message.id, &message.segments, window, cx);

            for tool_use in thread.read(cx).tool_uses_for_message(message.id, cx) {
                this.render_tool_use_markdown(
                    tool_use.id.clone(),
                    tool_use.ui_text.clone(),
                    &serde_json::to_string_pretty(&tool_use.input).unwrap_or_default(),
                    tool_use.status.text(),
                    cx,
                );
            }
        }

        this
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

    pub fn cancel_last_completion(&mut self, window: &mut Window, cx: &mut App) -> bool {
        self.last_error.take();
        self.thread.update(cx, |thread, cx| {
            thread.cancel_last_completion(Some(window.window_handle()), cx)
        })
    }

    pub fn last_error(&self) -> Option<ThreadError> {
        self.last_error.clone()
    }

    pub fn clear_last_error(&mut self) {
        self.last_error.take();
    }

    /// Returns the editing message id and the estimated token count in the content
    pub fn editing_message_id(&self) -> Option<(MessageId, usize)> {
        self.editing_message
            .as_ref()
            .map(|(id, state)| (*id, state.last_estimated_token_count.unwrap_or(0)))
    }

    pub fn context_store(&self) -> &Entity<ContextStore> {
        &self.context_store
    }

    pub fn thread_store(&self) -> &Entity<ThreadStore> {
        &self.thread_store
    }

    pub fn text_thread_store(&self) -> &Entity<TextThreadStore> {
        &self.text_thread_store
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
        tool_input: &str,
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
            this.replace(
                MarkdownCodeBlock {
                    tag: "json",
                    text: tool_input,
                }
                .to_string(),
                cx,
            );
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
            ThreadEvent::CancelEditing => {
                if self.editing_message.is_some() {
                    self.cancel_editing_message(&menu::Cancel, window, cx);
                }
            }
            ThreadEvent::ShowError(error) => {
                self.last_error = Some(error.clone());
            }
            ThreadEvent::NewRequest | ThreadEvent::CompletionCanceled => {
                cx.notify();
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
                        &serde_json::to_string_pretty(&tool_use.input).unwrap_or_default(),
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
                    &serde_json::to_string_pretty(&input).unwrap_or_default(),
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
                        &serde_json::to_string_pretty(&tool_use.input).unwrap_or_default(),
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
            ThreadEvent::InvalidToolInput {
                tool_use_id,
                ui_text,
                invalid_input_json,
            } => {
                self.render_tool_use_markdown(
                    tool_use_id.clone(),
                    ui_text,
                    invalid_input_json,
                    self.thread
                        .read(cx)
                        .output_for_tool(tool_use_id)
                        .map(|output| output.clone().into())
                        .unwrap_or("".into()),
                    cx,
                );
            }
            ThreadEvent::MissingToolUse {
                tool_use_id,
                ui_text,
            } => {
                self.render_tool_use_markdown(
                    tool_use_id.clone(),
                    ui_text,
                    "",
                    self.thread
                        .read(cx)
                        .output_for_tool(tool_use_id)
                        .map(|output| output.clone().into())
                        .unwrap_or("".into()),
                    cx,
                );
            }
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

        // Cancel any ongoing streaming when user starts editing a previous message
        self.cancel_last_completion(window, cx);

        let editor = crate::message_editor::create_editor(
            self.workspace.clone(),
            self.context_store.downgrade(),
            self.thread_store.downgrade(),
            self.text_thread_store.downgrade(),
            window,
            cx,
        );
        editor.update(cx, |editor, cx| {
            editor.set_text(message_text.clone(), window, cx);
            editor.focus_handle(cx).focus(window);
            editor.move_to_end(&editor::actions::MoveToEnd, window, cx);
        });
        let buffer_edited_subscription = cx.subscribe(&editor, |this, _, event, cx| match event {
            EditorEvent::BufferEdited => {
                this.update_editing_message_token_count(true, cx);
            }
            _ => {}
        });

        let context_picker_menu_handle = PopoverMenuHandle::default();
        let context_strip = cx.new(|cx| {
            ContextStrip::new(
                self.context_store.clone(),
                self.workspace.clone(),
                Some(self.thread_store.downgrade()),
                Some(self.text_thread_store.downgrade()),
                context_picker_menu_handle.clone(),
                SuggestContextKind::File,
                window,
                cx,
            )
        });

        let context_strip_subscription =
            cx.subscribe_in(&context_strip, window, Self::handle_context_strip_event);

        self.editing_message = Some((
            message_id,
            EditingMessageState {
                editor: editor.clone(),
                context_strip,
                context_picker_menu_handle,
                last_estimated_token_count: None,
                _subscriptions: [buffer_edited_subscription, context_strip_subscription],
                _update_token_count_task: None,
            },
        ));
        self.update_editing_message_token_count(false, cx);
        cx.notify();
    }

    fn handle_context_strip_event(
        &mut self,
        _context_strip: &Entity<ContextStrip>,
        event: &ContextStripEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, state)) = self.editing_message.as_ref() {
            match event {
                ContextStripEvent::PickerDismissed
                | ContextStripEvent::BlurredEmpty
                | ContextStripEvent::BlurredDown => {
                    let editor_focus_handle = state.editor.focus_handle(cx);
                    window.focus(&editor_focus_handle);
                }
                ContextStripEvent::BlurredUp => {}
            }
        }
    }

    fn update_editing_message_token_count(&mut self, debounce: bool, cx: &mut Context<Self>) {
        let Some((message_id, state)) = self.editing_message.as_mut() else {
            return;
        };

        cx.emit(ActiveThreadEvent::EditingMessageTokenCountChanged);
        state._update_token_count_task.take();

        let Some(configured_model) = self.thread.read(cx).configured_model() else {
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

            let token_count = if let Some(task) = cx
                .update(|cx| {
                    let Some(message) = thread.read(cx).message(message_id) else {
                        log::error!("Message that was being edited no longer exists");
                        return None;
                    };
                    let message_text = editor.read(cx).text(cx);

                    if message_text.is_empty() && message.loaded_context.is_empty() {
                        return None;
                    }

                    let mut request_message = LanguageModelRequestMessage {
                        role: language_model::Role::User,
                        content: Vec::new(),
                        cache: false,
                    };

                    message
                        .loaded_context
                        .add_to_request_message(&mut request_message);

                    if !message_text.is_empty() {
                        request_message
                            .content
                            .push(MessageContent::Text(message_text));
                    }

                    let request = language_model::LanguageModelRequest {
                        thread_id: None,
                        prompt_id: None,
                        mode: None,
                        messages: vec![request_message],
                        tools: vec![],
                        stop: vec![],
                        temperature: None,
                    };

                    Some(configured_model.model.count_tokens(request, cx))
                })
                .ok()
                .flatten()
            {
                task.await.log_err()
            } else {
                Some(0)
            };

            if let Some(token_count) = token_count {
                this.update(cx, |this, cx| {
                    let Some((_message_id, state)) = this.editing_message.as_mut() else {
                        return;
                    };

                    state.last_estimated_token_count = Some(token_count);
                    cx.emit(ActiveThreadEvent::EditingMessageTokenCountChanged);
                })
                .ok();
            };
        }));
    }

    fn toggle_context_picker(
        &mut self,
        _: &crate::ToggleContextPicker,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((_, state)) = self.editing_message.as_mut() {
            let handle = state.context_picker_menu_handle.clone();
            window.defer(cx, move |window, cx| {
                handle.toggle(window, cx);
            });
        }
    }

    fn remove_all_context(
        &mut self,
        _: &crate::RemoveAllContext,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_store.update(cx, |store, _cx| store.clear());
        cx.notify();
    }

    fn move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((_, state)) = self.editing_message.as_mut() {
            if state.context_picker_menu_handle.is_deployed() {
                cx.propagate();
            } else {
                state.context_strip.focus_handle(cx).focus(window);
            }
        }
    }

    fn paste(&mut self, _: &Paste, _window: &mut Window, cx: &mut Context<Self>) {
        let images = cx
            .read_from_clipboard()
            .map(|item| {
                item.into_entries()
                    .filter_map(|entry| {
                        if let ClipboardEntry::Image(image) = entry {
                            Some(image)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if images.is_empty() {
            return;
        }
        cx.stop_propagation();

        self.context_store.update(cx, |store, cx| {
            for image in images {
                store.add_image_instance(Arc::new(image), cx);
            }
        });
    }

    fn cancel_editing_message(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        self.editing_message.take();
        cx.notify();
    }

    fn confirm_editing_message(
        &mut self,
        _: &menu::Confirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((message_id, state)) = self.editing_message.take() else {
            return;
        };

        let Some(model) = self
            .thread
            .update(cx, |thread, cx| thread.get_or_init_configured_model(cx))
        else {
            return;
        };

        if model.provider.must_accept_terms(cx) {
            cx.notify();
            return;
        }

        let edited_text = state.editor.read(cx).text(cx);

        let new_context = self
            .context_store
            .read(cx)
            .new_context_for_thread(self.thread.read(cx), Some(message_id));

        let project = self.thread.read(cx).project().clone();
        let prompt_store = self.thread_store.read(cx).prompt_store().clone();

        let load_context_task =
            crate::context::load_context(new_context, &project, &prompt_store, cx);
        self._load_edited_message_context_task =
            Some(cx.spawn_in(window, async move |this, cx| {
                let context = load_context_task.await;
                let _ = this
                    .update_in(cx, |this, window, cx| {
                        this.thread.update(cx, |thread, cx| {
                            thread.edit_message(
                                message_id,
                                Role::User,
                                vec![MessageSegment::Text(edited_text)],
                                Some(context.loaded_context),
                                cx,
                            );
                            for message_id in this.messages_after(message_id) {
                                thread.delete_message(*message_id, cx);
                            }
                        });

                        this.thread.update(cx, |thread, cx| {
                            thread.advance_prompt_id();
                            thread.send_to_model(model.model, Some(window.window_handle()), cx);
                        });
                        this._load_edited_message_context_task = None;
                        cx.notify();
                    })
                    .log_err();
            }));
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

    fn render_edit_message_editor(
        &self,
        state: &EditingMessageState,
        _window: &mut Window,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let font_size = TextSize::Small
            .rems(cx)
            .to_pixels(settings.agent_font_size(cx));
        let line_height = font_size * 1.75;

        let colors = cx.theme().colors();

        let text_style = TextStyle {
            color: colors.text,
            font_family: settings.buffer_font.family.clone(),
            font_fallbacks: settings.buffer_font.fallbacks.clone(),
            font_features: settings.buffer_font.features.clone(),
            font_size: font_size.into(),
            line_height: line_height.into(),
            ..Default::default()
        };

        v_flex()
            .key_context("EditMessageEditor")
            .on_action(cx.listener(Self::toggle_context_picker))
            .on_action(cx.listener(Self::remove_all_context))
            .on_action(cx.listener(Self::move_up))
            .on_action(cx.listener(Self::cancel_editing_message))
            .on_action(cx.listener(Self::confirm_editing_message))
            .capture_action(cx.listener(Self::paste))
            .min_h_6()
            .flex_grow()
            .w_full()
            .gap_2()
            .child(EditorElement::new(
                &state.editor,
                EditorStyle {
                    background: colors.editor_background,
                    local_player: cx.theme().players().local(),
                    text: text_style,
                    syntax: cx.theme().syntax().clone(),
                    ..Default::default()
                },
            ))
            .child(state.context_strip.clone())
    }

    fn render_message(&self, ix: usize, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let message_id = self.messages[ix];
        let Some(message) = self.thread.read(cx).message(message_id) else {
            return Empty.into_any();
        };

        let Some(rendered_message) = self.rendered_messages_by_id.get(&message_id) else {
            return Empty.into_any();
        };

        let workspace = self.workspace.clone();
        let thread = self.thread.read(cx);

        // Get all the data we need from thread before we start using it in closures
        let checkpoint = thread.checkpoint_for_message(message_id);
        let added_context = thread
            .context_for_message(message_id)
            .map(|context| AddedContext::new_attached(context, cx))
            .collect::<Vec<_>>();

        let tool_uses = thread.tool_uses_for_message(message_id, cx);
        let has_tool_uses = !tool_uses.is_empty();
        let is_generating = thread.is_generating();
        let is_generating_stale = thread.is_generation_stale().unwrap_or(false);

        let is_first_message = ix == 0;
        let is_last_message = ix == self.messages.len() - 1;

        let loading_dots = (is_generating_stale && is_last_message)
            .then(|| AnimatedLabel::new("").size(LabelSize::Small));

        let editing_message_state = self
            .editing_message
            .as_ref()
            .filter(|(id, _)| *id == message_id)
            .map(|(_, state)| state);

        let colors = cx.theme().colors();
        let editor_bg_color = colors.editor_background;

        let open_as_markdown = IconButton::new(("open-as-markdown", ix), IconName::FileCode)
            .shape(ui::IconButtonShape::Square)
            .icon_size(IconSize::XSmall)
            .icon_color(Color::Ignored)
            .tooltip(Tooltip::text("Open Thread as Markdown"))
            .on_click({
                let thread = self.thread.clone();
                let workspace = self.workspace.clone();
                move |_, window, cx| {
                    if let Some(workspace) = workspace.upgrade() {
                        open_active_thread_as_markdown(thread.clone(), workspace, window, cx)
                            .detach_and_log_err(cx);
                    }
                }
            });

        // For all items that should be aligned with the LLM's response.
        const RESPONSE_PADDING_X: Pixels = px(19.);

        let show_feedback = thread.is_turn_end(ix);

        let feedback_container = h_flex()
            .group("feedback_container")
            .mt_1()
            .py_2()
            .px(RESPONSE_PADDING_X)
            .gap_1()
            .flex_wrap()
            .justify_end();
        let feedback_items = match self.thread.read(cx).message_feedback(message_id) {
            Some(feedback) => feedback_container
                .child(
                    div().mr_1().visible_on_hover("feedback_container").child(
                        Label::new(match feedback {
                            ThreadFeedback::Positive => "Thanks for your feedback!",
                            ThreadFeedback::Negative => {
                                "We appreciate your feedback and will use it to improve."
                            }
                        })
                    .color(Color::Muted)
                    .size(LabelSize::XSmall)
                    .truncate())
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
                    div().mr_1().visible_on_hover("feedback_container").child(
                        Label::new(
                            "Rating the thread sends all of your current conversation to the Zed team.",
                        )
                        .color(Color::Muted)
                    .size(LabelSize::XSmall)
                    .truncate())
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

        let message_content = has_content.then(|| {
            if let Some(state) = editing_message_state.as_ref() {
                self.render_edit_message_editor(state, window, cx)
                    .into_any_element()
            } else {
                v_flex()
                    .w_full()
                    .gap_1()
                    .when(!message_is_empty, |parent| {
                        parent.child(div().min_h_6().child(self.render_message_content(
                            message_id,
                            rendered_message,
                            has_tool_uses,
                            workspace.clone(),
                            window,
                            cx,
                        )))
                    })
                    .when(!added_context.is_empty(), |parent| {
                        parent.child(h_flex().flex_wrap().gap_1().children(
                            added_context.into_iter().map(|added_context| {
                                let context = added_context.handle.clone();
                                ContextPill::added(added_context, false, false, None).on_click(
                                    Rc::new(cx.listener({
                                        let workspace = workspace.clone();
                                        move |_, _, window, cx| {
                                            if let Some(workspace) = workspace.upgrade() {
                                                open_context(&context, workspace, window, cx);
                                                cx.notify();
                                            }
                                        }
                                    })),
                                )
                            }),
                        ))
                    })
                    .into_any_element()
            }
        });

        let styled_message = match message.role {
            Role::User => v_flex()
                .id(("message-container", ix))
                .pt_2()
                .pl_2()
                .pr_2p5()
                .pb_4()
                .child(
                    v_flex()
                        .id(("user-message", ix))
                        .bg(editor_bg_color)
                        .rounded_lg()
                        .shadow_md()
                        .border_1()
                        .border_color(colors.border)
                        .hover(|hover| hover.border_color(colors.text_accent.opacity(0.5)))
                        .cursor_pointer()
                        .child(
                            h_flex()
                                .p_2p5()
                                .gap_1()
                                .children(message_content)
                                .when_some(editing_message_state, |this, state| {
                                    let focus_handle = state.editor.focus_handle(cx).clone();
                                    this.w_full().justify_between().child(
                                        h_flex()
                                            .gap_0p5()
                                            .child(
                                                IconButton::new(
                                                    "cancel-edit-message",
                                                    IconName::Close,
                                                )
                                                .shape(ui::IconButtonShape::Square)
                                                .icon_color(Color::Error)
                                                .tooltip({
                                                    let focus_handle = focus_handle.clone();
                                                    move |window, cx| {
                                                        Tooltip::for_action_in(
                                                            "Cancel Edit",
                                                            &menu::Cancel,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                    }
                                                })
                                                .on_click(cx.listener(Self::handle_cancel_click)),
                                            )
                                            .child(
                                                IconButton::new(
                                                    "confirm-edit-message",
                                                    IconName::Check,
                                                )
                                                .disabled(state.editor.read(cx).is_empty(cx))
                                                .shape(ui::IconButtonShape::Square)
                                                .icon_color(Color::Success)
                                                .tooltip({
                                                    let focus_handle = focus_handle.clone();
                                                    move |window, cx| {
                                                        Tooltip::for_action_in(
                                                            "Regenerate",
                                                            &menu::Confirm,
                                                            &focus_handle,
                                                            window,
                                                            cx,
                                                        )
                                                    }
                                                })
                                                .on_click(
                                                    cx.listener(Self::handle_regenerate_click),
                                                ),
                                            ),
                                    )
                                }),
                        )
                        .when(editing_message_state.is_none(), |this| {
                            this.tooltip(Tooltip::text("Click To Edit"))
                        })
                        .on_click(cx.listener({
                            let message_segments = message.segments.clone();
                            move |this, _, window, cx| {
                                this.start_editing_message(
                                    message_id,
                                    &message_segments,
                                    window,
                                    cx,
                                );
                            }
                        })),
                ),
            Role::Assistant => v_flex()
                .id(("message-container", ix))
                .px(RESPONSE_PADDING_X)
                .gap_2()
                .children(message_content)
                .when(has_tool_uses, |parent| {
                    parent.children(tool_uses.into_iter().map(|tool_use| {
                        self.render_tool_use(tool_use, window, workspace.clone(), cx)
                    }))
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
            .map(|parent| {
                if let Some(checkpoint) = checkpoint.filter(|_| !is_generating) {
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
                } else {
                    parent
                }
            })
            .when(is_first_message, |parent| {
                parent.child(self.render_rules_item(cx))
            })
            .child(styled_message)
            .when(is_generating && is_last_message, |this| {
                this.child(
                    h_flex()
                        .h_8()
                        .mt_2()
                        .mb_4()
                        .ml_4()
                        .py_1p5()
                        .when_some(loading_dots, |this, loading_dots| this.child(loading_dots)),
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
                                            let editor_bg = cx.theme().colors().editor_background;

                                            move |el, range, metadata, _, cx| {
                                                let is_expanded = active_thread
                                                    .read(cx)
                                                    .expanded_code_blocks
                                                    .get(&(message_id, range.start))
                                                    .copied()
                                                    .unwrap_or(true);

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
                                                        .bg(linear_gradient(
                                                            0.,
                                                            linear_color_stop(editor_bg, 0.),
                                                            linear_color_stop(
                                                                editor_bg.opacity(0.),
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
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        if let Some(card) = self.thread.read(cx).card_for_tool(&tool_use.id) {
            return card.render(&tool_use.status, window, workspace, cx);
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
                                // TODO: Figure out a way to pass focus handle here so we can display the `OpenRulesLibrary`  keybinding
                                .tooltip(Tooltip::text("View User Rules"))
                                .on_click(move |_event, window, cx| {
                                    window.dispatch_action(
                                        Box::new(OpenRulesLibrary {
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
        window: &mut Window,
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
                    Some(window.window_handle()),
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let window_handle = window.window_handle();
        self.thread.update(cx, |thread, cx| {
            thread.deny_tool_use(tool_use_id, tool_name, Some(window_handle), cx);
        });
    }

    fn handle_open_rules(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let project_context = self.thread.read(cx).project_context();
        let project_context = project_context.borrow();
        let Some(project_context) = project_context.as_ref() else {
            return;
        };

        let project_entry_ids = project_context
            .worktrees
            .iter()
            .flat_map(|worktree| worktree.rules_file.as_ref())
            .map(|rules_file| ProjectEntryId::from_usize(rules_file.project_entry_id))
            .collect::<Vec<_>>();

        self.workspace
            .update(cx, move |workspace, cx| {
                // TODO: Open a multibuffer instead? In some cases this doesn't make the set of rules
                // files clear. For example, if rules file 1 is already open but rules file 2 is not,
                // this would open and focus rules file 2 in a tab that is not next to rules file 1.
                let project = workspace.project().read(cx);
                let project_paths = project_entry_ids
                    .into_iter()
                    .flat_map(|entry_id| project.path_for_entry(entry_id, cx))
                    .collect::<Vec<_>>();
                for project_path in project_paths {
                    workspace
                        .open_path(project_path, None, true, window, cx)
                        .detach_and_log_err(cx);
                }
            })
            .ok();
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

pub(crate) fn open_active_thread_as_markdown(
    thread: Entity<Thread>,
    workspace: Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let markdown_language_task = workspace
        .read(cx)
        .app_state()
        .languages
        .language_for_name("Markdown");

    window.spawn(cx, async move |cx| {
        let markdown_language = markdown_language_task.await?;

        workspace.update_in(cx, |workspace, window, cx| {
            let thread = thread.read(cx);
            let markdown = thread.to_markdown(cx)?;
            let thread_summary = thread
                .summary()
                .map(|summary| summary.to_string())
                .unwrap_or_else(|| "Thread".to_string());

            let project = workspace.project().clone();
            let buffer = project.update(cx, |project, cx| {
                project.create_local_buffer(&markdown, Some(markdown_language), cx)
            });
            let buffer =
                cx.new(|cx| MultiBuffer::singleton(buffer, cx).with_title(thread_summary.clone()));

            workspace.add_item_to_active_pane(
                Box::new(cx.new(|cx| {
                    let mut editor =
                        Editor::for_multibuffer(buffer, Some(project.clone()), window, cx);
                    editor.set_breadcrumb_header(thread_summary);
                    editor
                })),
                None,
                true,
                window,
                cx,
            );

            anyhow::Ok(())
        })??;
        anyhow::Ok(())
    })
}

pub(crate) fn open_context(
    context: &AgentContextHandle,
    workspace: Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    match context {
        AgentContextHandle::File(file_context) => {
            if let Some(project_path) = file_context.project_path(cx) {
                workspace.update(cx, |workspace, cx| {
                    workspace
                        .open_path(project_path, None, true, window, cx)
                        .detach_and_log_err(cx);
                });
            }
        }

        AgentContextHandle::Directory(directory_context) => {
            let entry_id = directory_context.entry_id;
            workspace.update(cx, |workspace, cx| {
                workspace.project().update(cx, |_project, cx| {
                    cx.emit(project::Event::RevealInProjectPanel(entry_id));
                })
            })
        }

        AgentContextHandle::Symbol(symbol_context) => {
            let buffer = symbol_context.buffer.read(cx);
            if let Some(project_path) = buffer.project_path(cx) {
                let snapshot = buffer.snapshot();
                let target_position = symbol_context.range.start.to_point(&snapshot);
                open_editor_at_position(project_path, target_position, &workspace, window, cx)
                    .detach();
            }
        }

        AgentContextHandle::Selection(selection_context) => {
            let buffer = selection_context.buffer.read(cx);
            if let Some(project_path) = buffer.project_path(cx) {
                let snapshot = buffer.snapshot();
                let target_position = selection_context.range.start.to_point(&snapshot);

                open_editor_at_position(project_path, target_position, &workspace, window, cx)
                    .detach();
            }
        }

        AgentContextHandle::FetchedUrl(fetched_url_context) => {
            cx.open_url(&fetched_url_context.url);
        }

        AgentContextHandle::Thread(thread_context) => workspace.update(cx, |workspace, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.open_thread(thread_context.thread.clone(), window, cx);
                });
            }
        }),

        AgentContextHandle::TextThread(text_thread_context) => {
            workspace.update(cx, |workspace, cx| {
                if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                    panel.update(cx, |panel, cx| {
                        panel.open_prompt_editor(text_thread_context.context.clone(), window, cx)
                    });
                }
            })
        }

        AgentContextHandle::Rules(rules_context) => window.dispatch_action(
            Box::new(OpenRulesLibrary {
                prompt_to_select: Some(rules_context.prompt_id.0),
            }),
            cx,
        ),

        AgentContextHandle::Image(_) => {}
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

#[cfg(test)]
mod tests {
    use assistant_tool::{ToolRegistry, ToolWorkingSet};
    use editor::EditorSettings;
    use fs::FakeFs;
    use gpui::{TestAppContext, VisualTestContext};
    use language_model::{LanguageModel, fake_provider::FakeLanguageModel};
    use project::Project;
    use prompt_store::PromptBuilder;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use crate::{ContextLoadResult, thread_store};

    use super::*;

    #[gpui::test]
    async fn test_current_completion_cancelled_when_message_edited(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let project = create_test_project(
            cx,
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let (cx, active_thread, thread, model) = setup_test_environment(cx, project.clone()).await;

        // Insert user message without any context (empty context vector)
        let message = thread.update(cx, |thread, cx| {
            let message_id = thread.insert_user_message(
                "What is the best way to learn Rust?",
                ContextLoadResult::default(),
                None,
                vec![],
                cx,
            );
            thread
                .message(message_id)
                .expect("message should exist")
                .clone()
        });

        // Stream response to user message
        thread.update(cx, |thread, cx| {
            let request = thread.to_completion_request(model.clone(), cx);
            thread.stream_completion(request, model, cx.active_window(), cx)
        });
        let generating = thread.update(cx, |thread, _cx| thread.is_generating());
        assert!(generating, "There should be one pending completion");

        // Edit the previous message
        active_thread.update_in(cx, |active_thread, window, cx| {
            active_thread.start_editing_message(message.id, &message.segments, window, cx);
        });

        // Check that the stream was cancelled
        let generating = thread.update(cx, |thread, _cx| thread.is_generating());
        assert!(!generating, "The completion should have been cancelled");
    }

    fn init_test_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            AssistantSettings::register(cx);
            prompt_store::init(cx);
            thread_store::init(cx);
            workspace::init_settings(cx);
            language_model::init_settings(cx);
            ThemeSettings::register(cx);
            EditorSettings::register(cx);
            ToolRegistry::default_global(cx);
        });
    }

    // Helper to create a test project with test files
    async fn create_test_project(
        cx: &mut TestAppContext,
        files: serde_json::Value,
    ) -> Entity<Project> {
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(path!("/test"), files).await;
        Project::test(fs, [path!("/test").as_ref()], cx).await
    }

    async fn setup_test_environment(
        cx: &mut TestAppContext,
        project: Entity<Project>,
    ) -> (
        &mut VisualTestContext,
        Entity<ActiveThread>,
        Entity<Thread>,
        Arc<dyn LanguageModel>,
    ) {
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let prompt_builder = Arc::new(PromptBuilder::new(None).unwrap());
        let thread_store = cx
            .update(|_, cx| {
                ThreadStore::load(
                    project.clone(),
                    cx.new(|_| ToolWorkingSet::default()),
                    None,
                    prompt_builder.clone(),
                    cx,
                )
            })
            .await
            .unwrap();
        let text_thread_store = cx
            .update(|_, cx| {
                TextThreadStore::new(project.clone(), prompt_builder, Default::default(), cx)
            })
            .await
            .unwrap();

        let thread = thread_store.update(cx, |store, cx| store.create_thread(cx));
        let context_store = cx.new(|_cx| ContextStore::new(project.downgrade(), None));

        let model = FakeLanguageModel::default();
        let model: Arc<dyn LanguageModel> = Arc::new(model);

        let language_registry = LanguageRegistry::new(cx.executor());
        let language_registry = Arc::new(language_registry);

        let active_thread = cx.update(|window, cx| {
            cx.new(|cx| {
                ActiveThread::new(
                    thread.clone(),
                    thread_store.clone(),
                    text_thread_store.clone(),
                    context_store.clone(),
                    language_registry.clone(),
                    workspace.downgrade(),
                    window,
                    cx,
                )
            })
        });

        (cx, active_thread, thread, model)
    }
}
