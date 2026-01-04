pub mod parser;
mod path_range;

use base64::Engine as _;
use futures::FutureExt as _;
use gpui::HitboxBehavior;
use language::LanguageName;
use log::Level;
pub use path_range::{LineCol, PathWithRange};
use ui::Checkbox;
use ui::CopyButton;

use std::borrow::Cow;
use std::iter;
use std::mem;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use collections::{HashMap, HashSet};
use gpui::{
    AnyElement, App, BorderStyle, Bounds, ClipboardItem, CursorStyle, DispatchPhase, Edges, Entity,
    FocusHandle, Focusable, FontStyle, FontWeight, GlobalElementId, Hitbox, Hsla, Image,
    ImageFormat, KeyContext, Length, MouseButton, MouseDownEvent, MouseEvent, MouseMoveEvent,
    MouseUpEvent, Point, ScrollHandle, Stateful, StrikethroughStyle, StyleRefinement, StyledText,
    Task, TextLayout, TextRun, TextStyle, TextStyleRefinement, actions, img, point, quad,
};
use language::{Language, LanguageRegistry, Rope};
use parser::CodeBlockMetadata;
use parser::{MarkdownEvent, MarkdownTag, MarkdownTagEnd, parse_links_only, parse_markdown};
use pulldown_cmark::Alignment;
use sum_tree::TreeMap;
use theme::SyntaxTheme;
use ui::{ScrollAxes, Scrollbars, WithScrollbar, prelude::*};
use util::ResultExt;

use crate::parser::CodeBlockKind;

/// A callback function that can be used to customize the style of links based on the destination URL.
/// If the callback returns `None`, the default link style will be used.
type LinkStyleCallback = Rc<dyn Fn(&str, &App) -> Option<TextStyleRefinement>>;

/// Defines custom style refinements for each heading level (H1-H6)
#[derive(Clone, Default)]
pub struct HeadingLevelStyles {
    pub h1: Option<TextStyleRefinement>,
    pub h2: Option<TextStyleRefinement>,
    pub h3: Option<TextStyleRefinement>,
    pub h4: Option<TextStyleRefinement>,
    pub h5: Option<TextStyleRefinement>,
    pub h6: Option<TextStyleRefinement>,
}

#[derive(Clone)]
pub struct MarkdownStyle {
    pub base_text_style: TextStyle,
    pub container_style: StyleRefinement,
    pub code_block: StyleRefinement,
    pub code_block_overflow_x_scroll: bool,
    pub inline_code: TextStyleRefinement,
    pub block_quote: TextStyleRefinement,
    pub link: TextStyleRefinement,
    pub link_callback: Option<LinkStyleCallback>,
    pub rule_color: Hsla,
    pub block_quote_border_color: Hsla,
    pub syntax: Arc<SyntaxTheme>,
    pub selection_background_color: Hsla,
    pub heading: StyleRefinement,
    pub heading_level_styles: Option<HeadingLevelStyles>,
    pub height_is_multiple_of_line_height: bool,
    pub prevent_mouse_interaction: bool,
    pub table_columns_min_size: bool,
}

impl Default for MarkdownStyle {
    fn default() -> Self {
        Self {
            base_text_style: Default::default(),
            container_style: Default::default(),
            code_block: Default::default(),
            code_block_overflow_x_scroll: false,
            inline_code: Default::default(),
            block_quote: Default::default(),
            link: Default::default(),
            link_callback: None,
            rule_color: Default::default(),
            block_quote_border_color: Default::default(),
            syntax: Arc::new(SyntaxTheme::default()),
            selection_background_color: Default::default(),
            heading: Default::default(),
            heading_level_styles: None,
            height_is_multiple_of_line_height: false,
            prevent_mouse_interaction: false,
            table_columns_min_size: false,
        }
    }
}

pub struct Markdown {
    source: SharedString,
    selection: Selection,
    pressed_link: Option<RenderedLink>,
    autoscroll_request: Option<usize>,
    parsed_markdown: ParsedMarkdown,
    images_by_source_offset: HashMap<usize, Arc<Image>>,
    should_reparse: bool,
    pending_parse: Option<Task<()>>,
    focus_handle: FocusHandle,
    language_registry: Option<Arc<LanguageRegistry>>,
    fallback_code_block_language: Option<LanguageName>,
    options: Options,
    copied_code_blocks: HashSet<ElementId>,
    code_block_scroll_handles: HashMap<usize, ScrollHandle>,
    context_menu_selected_text: Option<String>,
}

struct Options {
    parse_links_only: bool,
}

pub enum CodeBlockRenderer {
    Default {
        copy_button: bool,
        copy_button_on_hover: bool,
        border: bool,
    },
    Custom {
        render: CodeBlockRenderFn,
        /// A function that can modify the parent container after the code block
        /// content has been appended as a child element.
        transform: Option<CodeBlockTransformFn>,
    },
}

pub type CodeBlockRenderFn = Arc<
    dyn Fn(
        &CodeBlockKind,
        &ParsedMarkdown,
        Range<usize>,
        CodeBlockMetadata,
        &mut Window,
        &App,
    ) -> Div,
>;

pub type CodeBlockTransformFn =
    Arc<dyn Fn(AnyDiv, Range<usize>, CodeBlockMetadata, &mut Window, &App) -> AnyDiv>;

actions!(
    markdown,
    [
        /// Copies the selected text to the clipboard.
        Copy,
        /// Copies the selected text as markdown to the clipboard.
        CopyAsMarkdown
    ]
);

impl Markdown {
    pub fn new(
        source: SharedString,
        language_registry: Option<Arc<LanguageRegistry>>,
        fallback_code_block_language: Option<LanguageName>,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let mut this = Self {
            source,
            selection: Selection::default(),
            pressed_link: None,
            autoscroll_request: None,
            should_reparse: false,
            images_by_source_offset: Default::default(),
            parsed_markdown: ParsedMarkdown::default(),
            pending_parse: None,
            focus_handle,
            language_registry,
            fallback_code_block_language,
            options: Options {
                parse_links_only: false,
            },
            copied_code_blocks: HashSet::default(),
            code_block_scroll_handles: HashMap::default(),
            context_menu_selected_text: None,
        };
        this.parse(cx);
        this
    }

    pub fn new_text(source: SharedString, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let mut this = Self {
            source,
            selection: Selection::default(),
            pressed_link: None,
            autoscroll_request: None,
            should_reparse: false,
            parsed_markdown: ParsedMarkdown::default(),
            images_by_source_offset: Default::default(),
            pending_parse: None,
            focus_handle,
            language_registry: None,
            fallback_code_block_language: None,
            options: Options {
                parse_links_only: true,
            },
            copied_code_blocks: HashSet::default(),
            code_block_scroll_handles: HashMap::default(),
            context_menu_selected_text: None,
        };
        this.parse(cx);
        this
    }

    fn code_block_scroll_handle(&mut self, id: usize) -> ScrollHandle {
        self.code_block_scroll_handles
            .entry(id)
            .or_insert_with(ScrollHandle::new)
            .clone()
    }

    fn retain_code_block_scroll_handles(&mut self, ids: &HashSet<usize>) {
        self.code_block_scroll_handles
            .retain(|id, _| ids.contains(id));
    }

    fn clear_code_block_scroll_handles(&mut self) {
        self.code_block_scroll_handles.clear();
    }

    pub fn is_parsing(&self) -> bool {
        self.pending_parse.is_some()
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn append(&mut self, text: &str, cx: &mut Context<Self>) {
        self.source = SharedString::new(self.source.to_string() + text);
        self.parse(cx);
    }

    pub fn replace(&mut self, source: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.source = source.into();
        self.parse(cx);
    }

    pub fn reset(&mut self, source: SharedString, cx: &mut Context<Self>) {
        if source == self.source() {
            return;
        }
        self.source = source;
        self.selection = Selection::default();
        self.autoscroll_request = None;
        self.pending_parse = None;
        self.should_reparse = false;
        // Don't clear parsed_markdown here - keep existing content visible until new parse completes
        self.parse(cx);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn parsed_markdown(&self) -> &ParsedMarkdown {
        &self.parsed_markdown
    }

    pub fn escape(s: &str) -> Cow<'_, str> {
        // Valid to use bytes since multi-byte UTF-8 doesn't use ASCII chars.
        let count = s
            .bytes()
            .filter(|c| *c == b'\n' || c.is_ascii_punctuation())
            .count();
        if count > 0 {
            let mut output = String::with_capacity(s.len() + count);
            let mut is_newline = false;
            for c in s.chars() {
                if is_newline && c == ' ' {
                    continue;
                }
                is_newline = c == '\n';
                if c == '\n' {
                    output.push('\n')
                } else if c.is_ascii_punctuation() {
                    output.push('\\')
                }
                output.push(c)
            }
            output.into()
        } else {
            s.into()
        }
    }

    pub fn selected_text(&self) -> Option<String> {
        if self.selection.end <= self.selection.start {
            None
        } else {
            Some(self.source[self.selection.start..self.selection.end].to_string())
        }
    }

    fn copy(&self, text: &RenderedText, _: &mut Window, cx: &mut Context<Self>) {
        if self.selection.end <= self.selection.start {
            return;
        }
        let text = text.text_for_range(self.selection.start..self.selection.end);
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn copy_as_markdown(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = self.context_menu_selected_text.take() {
            cx.write_to_clipboard(ClipboardItem::new_string(text));
            return;
        }
        if self.selection.end <= self.selection.start {
            return;
        }
        let text = self.source[self.selection.start..self.selection.end].to_string();
        cx.write_to_clipboard(ClipboardItem::new_string(text));
    }

    fn capture_selection_for_context_menu(&mut self) {
        self.context_menu_selected_text = self.selected_text();
    }

    fn parse(&mut self, cx: &mut Context<Self>) {
        if self.source.is_empty() {
            return;
        }

        if self.pending_parse.is_some() {
            self.should_reparse = true;
            return;
        }
        self.should_reparse = false;
        self.pending_parse = Some(self.start_background_parse(cx));
    }

    fn start_background_parse(&self, cx: &Context<Self>) -> Task<()> {
        let source = self.source.clone();
        let should_parse_links_only = self.options.parse_links_only;
        let language_registry = self.language_registry.clone();
        let fallback = self.fallback_code_block_language.clone();

        let parsed = cx.background_spawn(async move {
            if should_parse_links_only {
                return (
                    ParsedMarkdown {
                        events: Arc::from(parse_links_only(source.as_ref())),
                        source,
                        languages_by_name: TreeMap::default(),
                        languages_by_path: TreeMap::default(),
                    },
                    Default::default(),
                );
            }

            let (events, language_names, paths) = parse_markdown(&source);
            let mut images_by_source_offset = HashMap::default();
            let mut languages_by_name = TreeMap::default();
            let mut languages_by_path = TreeMap::default();
            if let Some(registry) = language_registry.as_ref() {
                for name in language_names {
                    let language = if !name.is_empty() {
                        registry.language_for_name_or_extension(&name).left_future()
                    } else if let Some(fallback) = &fallback {
                        registry.language_for_name(fallback.as_ref()).right_future()
                    } else {
                        continue;
                    };
                    if let Ok(language) = language.await {
                        languages_by_name.insert(name, language);
                    }
                }

                for path in paths {
                    if let Ok(language) = registry
                        .load_language_for_file_path(Path::new(path.as_ref()))
                        .await
                    {
                        languages_by_path.insert(path, language);
                    }
                }
            }

            for (range, event) in &events {
                if let MarkdownEvent::Start(MarkdownTag::Image { dest_url, .. }) = event
                    && let Some(data_url) = dest_url.strip_prefix("data:")
                {
                    let Some((mime_info, data)) = data_url.split_once(',') else {
                        continue;
                    };
                    let Some((mime_type, encoding)) = mime_info.split_once(';') else {
                        continue;
                    };
                    let Some(format) = ImageFormat::from_mime_type(mime_type) else {
                        continue;
                    };
                    let is_base64 = encoding == "base64";
                    if is_base64
                        && let Some(bytes) = base64::prelude::BASE64_STANDARD
                            .decode(data)
                            .log_with_level(Level::Debug)
                    {
                        let image = Arc::new(Image::from_bytes(format, bytes));
                        images_by_source_offset.insert(range.start, image);
                    }
                }
            }

            (
                ParsedMarkdown {
                    source,
                    events: Arc::from(events),
                    languages_by_name,
                    languages_by_path,
                },
                images_by_source_offset,
            )
        });

        cx.spawn(async move |this, cx| {
            let (parsed, images_by_source_offset) = parsed.await;

            this.update(cx, |this, cx| {
                this.parsed_markdown = parsed;
                this.images_by_source_offset = images_by_source_offset;
                this.pending_parse.take();
                if this.should_reparse {
                    this.parse(cx);
                }
                cx.refresh_windows();
            })
            .ok();
        })
    }
}

impl Focusable for Markdown {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Debug, Default, Clone)]
enum SelectMode {
    #[default]
    Character,
    Word(Range<usize>),
    Line(Range<usize>),
    All,
}

#[derive(Clone, Default)]
struct Selection {
    start: usize,
    end: usize,
    reversed: bool,
    pending: bool,
    mode: SelectMode,
}

impl Selection {
    fn set_head(&mut self, head: usize, rendered_text: &RenderedText) {
        match &self.mode {
            SelectMode::Character => {
                if head < self.tail() {
                    if !self.reversed {
                        self.end = self.start;
                        self.reversed = true;
                    }
                    self.start = head;
                } else {
                    if self.reversed {
                        self.start = self.end;
                        self.reversed = false;
                    }
                    self.end = head;
                }
            }
            SelectMode::Word(original_range) | SelectMode::Line(original_range) => {
                let head_range = if matches!(self.mode, SelectMode::Word(_)) {
                    rendered_text.surrounding_word_range(head)
                } else {
                    rendered_text.surrounding_line_range(head)
                };

                if head < original_range.start {
                    self.start = head_range.start;
                    self.end = original_range.end;
                    self.reversed = true;
                } else if head >= original_range.end {
                    self.start = original_range.start;
                    self.end = head_range.end;
                    self.reversed = false;
                } else {
                    self.start = original_range.start;
                    self.end = original_range.end;
                    self.reversed = false;
                }
            }
            SelectMode::All => {
                self.start = 0;
                self.end = rendered_text
                    .lines
                    .last()
                    .map(|line| line.source_end)
                    .unwrap_or(0);
                self.reversed = false;
            }
        }
    }

    fn tail(&self) -> usize {
        if self.reversed { self.end } else { self.start }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParsedMarkdown {
    pub source: SharedString,
    pub events: Arc<[(Range<usize>, MarkdownEvent)]>,
    pub languages_by_name: TreeMap<SharedString, Arc<Language>>,
    pub languages_by_path: TreeMap<Arc<str>, Arc<Language>>,
}

impl ParsedMarkdown {
    pub fn source(&self) -> &SharedString {
        &self.source
    }

    pub fn events(&self) -> &Arc<[(Range<usize>, MarkdownEvent)]> {
        &self.events
    }
}

pub struct MarkdownElement {
    markdown: Entity<Markdown>,
    style: MarkdownStyle,
    code_block_renderer: CodeBlockRenderer,
    on_url_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App)>>,
}

impl MarkdownElement {
    pub fn new(markdown: Entity<Markdown>, style: MarkdownStyle) -> Self {
        Self {
            markdown,
            style,
            code_block_renderer: CodeBlockRenderer::Default {
                copy_button: true,
                copy_button_on_hover: false,
                border: false,
            },
            on_url_click: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn rendered_text(
        markdown: Entity<Markdown>,
        cx: &mut gpui::VisualTestContext,
        style: impl FnOnce(&Window, &App) -> MarkdownStyle,
    ) -> String {
        use gpui::size;

        let (text, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |window, cx| Self::new(markdown, style(window, cx)),
        );
        text.text
            .lines
            .iter()
            .map(|line| line.layout.wrapped_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn code_block_renderer(mut self, variant: CodeBlockRenderer) -> Self {
        self.code_block_renderer = variant;
        self
    }

    pub fn on_url_click(
        mut self,
        handler: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_url_click = Some(Box::new(handler));
        self
    }

    fn paint_selection(
        &self,
        bounds: Bounds<Pixels>,
        rendered_text: &RenderedText,
        window: &mut Window,
        cx: &mut App,
    ) {
        let selection = self.markdown.read(cx).selection.clone();
        let selection_start = rendered_text.position_for_source_index(selection.start);
        let selection_end = rendered_text.position_for_source_index(selection.end);
        if let Some(((start_position, start_line_height), (end_position, end_line_height))) =
            selection_start.zip(selection_end)
        {
            if start_position.y == end_position.y {
                window.paint_quad(quad(
                    Bounds::from_corners(
                        start_position,
                        point(end_position.x, end_position.y + end_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                    BorderStyle::default(),
                ));
            } else {
                window.paint_quad(quad(
                    Bounds::from_corners(
                        start_position,
                        point(bounds.right(), start_position.y + start_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                    BorderStyle::default(),
                ));

                if end_position.y > start_position.y + start_line_height {
                    window.paint_quad(quad(
                        Bounds::from_corners(
                            point(bounds.left(), start_position.y + start_line_height),
                            point(bounds.right(), end_position.y),
                        ),
                        Pixels::ZERO,
                        self.style.selection_background_color,
                        Edges::default(),
                        Hsla::transparent_black(),
                        BorderStyle::default(),
                    ));
                }

                window.paint_quad(quad(
                    Bounds::from_corners(
                        point(bounds.left(), end_position.y),
                        point(end_position.x, end_position.y + end_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                    BorderStyle::default(),
                ));
            }
        }
    }

    fn paint_mouse_listeners(
        &mut self,
        hitbox: &Hitbox,
        rendered_text: &RenderedText,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.style.prevent_mouse_interaction {
            return;
        }

        let is_hovering_link = hitbox.is_hovered(window)
            && !self.markdown.read(cx).selection.pending
            && rendered_text
                .link_for_position(window.mouse_position())
                .is_some();

        if !self.style.prevent_mouse_interaction {
            if is_hovering_link {
                window.set_cursor_style(CursorStyle::PointingHand, hitbox);
            } else {
                window.set_cursor_style(CursorStyle::IBeam, hitbox);
            }
        }

        let on_open_url = self.on_url_click.take();

        self.on_mouse_event(window, cx, {
            let hitbox = hitbox.clone();
            move |markdown, event: &MouseDownEvent, phase, window, _| {
                if phase.capture()
                    && event.button == MouseButton::Right
                    && hitbox.is_hovered(window)
                {
                    // Capture selected text so it survives until menu item is clicked
                    markdown.capture_selection_for_context_menu();
                }
            }
        });

        self.on_mouse_event(window, cx, {
            let rendered_text = rendered_text.clone();
            let hitbox = hitbox.clone();
            move |markdown, event: &MouseDownEvent, phase, window, cx| {
                if hitbox.is_hovered(window) {
                    if phase.bubble() {
                        if let Some(link) = rendered_text.link_for_position(event.position) {
                            markdown.pressed_link = Some(link.clone());
                        } else {
                            let source_index =
                                match rendered_text.source_index_for_position(event.position) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                            let (range, mode) = match event.click_count {
                                1 => {
                                    let range = source_index..source_index;
                                    (range, SelectMode::Character)
                                }
                                2 => {
                                    let range = rendered_text.surrounding_word_range(source_index);
                                    (range.clone(), SelectMode::Word(range))
                                }
                                3 => {
                                    let range = rendered_text.surrounding_line_range(source_index);
                                    (range.clone(), SelectMode::Line(range))
                                }
                                _ => {
                                    let range = 0..rendered_text
                                        .lines
                                        .last()
                                        .map(|line| line.source_end)
                                        .unwrap_or(0);
                                    (range, SelectMode::All)
                                }
                            };
                            markdown.selection = Selection {
                                start: range.start,
                                end: range.end,
                                reversed: false,
                                pending: true,
                                mode,
                            };
                            window.focus(&markdown.focus_handle, cx);
                        }

                        window.prevent_default();
                        cx.notify();
                    }
                } else if phase.capture() && event.button == MouseButton::Left {
                    markdown.selection = Selection::default();
                    markdown.pressed_link = None;
                    cx.notify();
                }
            }
        });
        self.on_mouse_event(window, cx, {
            let rendered_text = rendered_text.clone();
            let hitbox = hitbox.clone();
            let was_hovering_link = is_hovering_link;
            move |markdown, event: &MouseMoveEvent, phase, window, cx| {
                if phase.capture() {
                    return;
                }

                if markdown.selection.pending {
                    let source_index = match rendered_text.source_index_for_position(event.position)
                    {
                        Ok(ix) | Err(ix) => ix,
                    };
                    markdown.selection.set_head(source_index, &rendered_text);
                    markdown.autoscroll_request = Some(source_index);
                    cx.notify();
                } else {
                    let is_hovering_link = hitbox.is_hovered(window)
                        && rendered_text.link_for_position(event.position).is_some();
                    if is_hovering_link != was_hovering_link {
                        cx.notify();
                    }
                }
            }
        });
        self.on_mouse_event(window, cx, {
            let rendered_text = rendered_text.clone();
            move |markdown, event: &MouseUpEvent, phase, window, cx| {
                if phase.bubble() {
                    if let Some(pressed_link) = markdown.pressed_link.take()
                        && Some(&pressed_link) == rendered_text.link_for_position(event.position)
                    {
                        if let Some(open_url) = on_open_url.as_ref() {
                            open_url(pressed_link.destination_url, window, cx);
                        } else {
                            cx.open_url(&pressed_link.destination_url);
                        }
                    }
                } else if markdown.selection.pending {
                    markdown.selection.pending = false;
                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                    {
                        let text = rendered_text
                            .text_for_range(markdown.selection.start..markdown.selection.end);
                        cx.write_to_primary(ClipboardItem::new_string(text))
                    }
                    cx.notify();
                }
            }
        });
    }

    fn autoscroll(
        &self,
        rendered_text: &RenderedText,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<()> {
        let autoscroll_index = self
            .markdown
            .update(cx, |markdown, _| markdown.autoscroll_request.take())?;
        let (position, line_height) = rendered_text.position_for_source_index(autoscroll_index)?;

        let text_style = self.style.base_text_style.clone();
        let font_id = window.text_system().resolve_font(&text_style.font());
        let font_size = text_style.font_size.to_pixels(window.rem_size());
        let em_width = window.text_system().em_width(font_id, font_size).unwrap();
        window.request_autoscroll(Bounds::from_corners(
            point(position.x - 3. * em_width, position.y - 3. * line_height),
            point(position.x + 3. * em_width, position.y + 3. * line_height),
        ));
        Some(())
    }

    fn on_mouse_event<T: MouseEvent>(
        &self,
        window: &mut Window,
        _cx: &mut App,
        mut f: impl 'static
        + FnMut(&mut Markdown, &T, DispatchPhase, &mut Window, &mut Context<Markdown>),
    ) {
        window.on_mouse_event({
            let markdown = self.markdown.downgrade();
            move |event, phase, window, cx| {
                markdown
                    .update(cx, |markdown, cx| f(markdown, event, phase, window, cx))
                    .log_err();
            }
        });
    }
}

impl Styled for MarkdownElement {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style.container_style
    }
}

impl Element for MarkdownElement {
    type RequestLayoutState = RenderedMarkdown;
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut builder = MarkdownElementBuilder::new(
            &self.style.container_style,
            self.style.base_text_style.clone(),
            self.style.syntax.clone(),
        );
        let (parsed_markdown, images) = {
            let markdown = self.markdown.read(cx);
            (
                markdown.parsed_markdown.clone(),
                markdown.images_by_source_offset.clone(),
            )
        };
        let markdown_end = if let Some(last) = parsed_markdown.events.last() {
            last.0.end
        } else {
            0
        };
        let mut code_block_ids = HashSet::default();

        let mut current_img_block_range: Option<Range<usize>> = None;
        for (index, (range, event)) in parsed_markdown.events.iter().enumerate() {
            // Skip alt text for images that rendered
            if let Some(current_img_block_range) = &current_img_block_range
                && current_img_block_range.end > range.end
            {
                continue;
            }

            match event {
                MarkdownEvent::Start(tag) => {
                    match tag {
                        MarkdownTag::Image { .. } => {
                            if let Some(image) = images.get(&range.start) {
                                current_img_block_range = Some(range.clone());
                                builder.modify_current_div(|el| {
                                    el.items_center()
                                        .flex()
                                        .flex_row()
                                        .child(img(image.clone()))
                                });
                            }
                        }
                        MarkdownTag::Paragraph => {
                            builder.push_div(
                                div().when(!self.style.height_is_multiple_of_line_height, |el| {
                                    el.mb_2().line_height(rems(1.3))
                                }),
                                range,
                                markdown_end,
                            );
                        }
                        MarkdownTag::Heading { level, .. } => {
                            let mut heading = div().mb_2();

                            heading = apply_heading_style(
                                heading,
                                *level,
                                self.style.heading_level_styles.as_ref(),
                            );

                            heading.style().refine(&self.style.heading);

                            let text_style = self.style.heading.text_style().clone();

                            builder.push_text_style(text_style);
                            builder.push_div(heading, range, markdown_end);
                        }
                        MarkdownTag::BlockQuote => {
                            builder.push_text_style(self.style.block_quote.clone());
                            builder.push_div(
                                div()
                                    .pl_4()
                                    .mb_2()
                                    .border_l_4()
                                    .border_color(self.style.block_quote_border_color),
                                range,
                                markdown_end,
                            );
                        }
                        MarkdownTag::CodeBlock { kind, .. } => {
                            let language = match kind {
                                CodeBlockKind::Fenced => None,
                                CodeBlockKind::FencedLang(language) => {
                                    parsed_markdown.languages_by_name.get(language).cloned()
                                }
                                CodeBlockKind::FencedSrc(path_range) => parsed_markdown
                                    .languages_by_path
                                    .get(&path_range.path)
                                    .cloned(),
                                _ => None,
                            };

                            let is_indented = matches!(kind, CodeBlockKind::Indented);
                            let scroll_handle = if self.style.code_block_overflow_x_scroll {
                                code_block_ids.insert(range.start);
                                Some(self.markdown.update(cx, |markdown, _| {
                                    markdown.code_block_scroll_handle(range.start)
                                }))
                            } else {
                                None
                            };

                            match (&self.code_block_renderer, is_indented) {
                                (CodeBlockRenderer::Default { .. }, _) | (_, true) => {
                                    // This is a parent container that we can position the copy button inside.
                                    let parent_container =
                                        div().group("code_block").relative().w_full();

                                    let mut parent_container: AnyDiv = if let Some(scroll_handle) =
                                        scroll_handle.as_ref()
                                    {
                                        let scrollbars = Scrollbars::new(ScrollAxes::Horizontal)
                                            .id(("markdown-code-block-scrollbar", range.start))
                                            .tracked_scroll_handle(scroll_handle)
                                            .with_track_along(
                                                ScrollAxes::Horizontal,
                                                cx.theme().colors().editor_background,
                                            )
                                            .notify_content();

                                        parent_container
                                            .rounded_lg()
                                            .custom_scrollbars(scrollbars, window, cx)
                                            .into()
                                    } else {
                                        parent_container.into()
                                    };

                                    if let CodeBlockRenderer::Default { border: true, .. } =
                                        &self.code_block_renderer
                                    {
                                        parent_container = parent_container
                                            .rounded_md()
                                            .border_1()
                                            .border_color(cx.theme().colors().border_variant);
                                    }

                                    parent_container.style().refine(&self.style.code_block);
                                    builder.push_div(parent_container, range, markdown_end);

                                    let code_block = div()
                                        .id(("code-block", range.start))
                                        .rounded_lg()
                                        .map(|mut code_block| {
                                            if let Some(scroll_handle) = scroll_handle.as_ref() {
                                                code_block.style().restrict_scroll_to_axis =
                                                    Some(true);
                                                code_block
                                                    .flex()
                                                    .overflow_x_scroll()
                                                    .track_scroll(scroll_handle)
                                            } else {
                                                code_block.w_full()
                                            }
                                        });

                                    builder.push_text_style(self.style.code_block.text.to_owned());
                                    builder.push_code_block(language);
                                    builder.push_div(code_block, range, markdown_end);
                                }
                                (CodeBlockRenderer::Custom { .. }, _) => {}
                            }
                        }
                        MarkdownTag::HtmlBlock => builder.push_div(div(), range, markdown_end),
                        MarkdownTag::List(bullet_index) => {
                            builder.push_list(*bullet_index);
                            builder.push_div(div().pl_2p5(), range, markdown_end);
                        }
                        MarkdownTag::Item => {
                            let bullet = if let Some((_, MarkdownEvent::TaskListMarker(checked))) =
                                parsed_markdown.events.get(index.saturating_add(1))
                            {
                                let source = &parsed_markdown.source()[range.clone()];

                                Checkbox::new(
                                    ElementId::Name(source.to_string().into()),
                                    if *checked {
                                        ToggleState::Selected
                                    } else {
                                        ToggleState::Unselected
                                    },
                                )
                                .fill()
                                .visualization_only(true)
                                .into_any_element()
                            } else if let Some(bullet_index) = builder.next_bullet_index() {
                                div().child(format!("{}.", bullet_index)).into_any_element()
                            } else {
                                div().child("•").into_any_element()
                            };
                            builder.push_div(
                                div()
                                    .when(!self.style.height_is_multiple_of_line_height, |el| {
                                        el.mb_1().gap_1().line_height(rems(1.3))
                                    })
                                    .h_flex()
                                    .items_start()
                                    .child(bullet),
                                range,
                                markdown_end,
                            );
                            // Without `w_0`, text doesn't wrap to the width of the container.
                            builder.push_div(div().flex_1().w_0(), range, markdown_end);
                        }
                        MarkdownTag::Emphasis => builder.push_text_style(TextStyleRefinement {
                            font_style: Some(FontStyle::Italic),
                            ..Default::default()
                        }),
                        MarkdownTag::Strong => builder.push_text_style(TextStyleRefinement {
                            font_weight: Some(FontWeight::BOLD),
                            ..Default::default()
                        }),
                        MarkdownTag::Strikethrough => {
                            builder.push_text_style(TextStyleRefinement {
                                strikethrough: Some(StrikethroughStyle {
                                    thickness: px(1.),
                                    color: None,
                                }),
                                ..Default::default()
                            })
                        }
                        MarkdownTag::Link { dest_url, .. } => {
                            if builder.code_block_stack.is_empty() {
                                builder.push_link(dest_url.clone(), range.clone());
                                let style = self
                                    .style
                                    .link_callback
                                    .as_ref()
                                    .and_then(|callback| callback(dest_url, cx))
                                    .unwrap_or_else(|| self.style.link.clone());
                                builder.push_text_style(style)
                            }
                        }
                        MarkdownTag::MetadataBlock(_) => {}
                        MarkdownTag::Table(alignments) => {
                            builder.table.start(alignments.clone());

                            let column_count = alignments.len();
                            builder.push_div(
                                div()
                                    .id(("table", range.start))
                                    .grid()
                                    .grid_cols(column_count as u16)
                                    .when(self.style.table_columns_min_size, |this| {
                                        this.grid_cols_min_content(column_count as u16)
                                    })
                                    .when(!self.style.table_columns_min_size, |this| {
                                        this.grid_cols(column_count as u16)
                                    })
                                    .size_full()
                                    .mb_2()
                                    .border(px(1.5))
                                    .border_color(cx.theme().colors().border)
                                    .rounded_sm()
                                    .overflow_hidden(),
                                range,
                                markdown_end,
                            );
                        }
                        MarkdownTag::TableHead => {
                            builder.table.start_head();
                            builder.push_text_style(TextStyleRefinement {
                                font_weight: Some(FontWeight::SEMIBOLD),
                                ..Default::default()
                            });
                        }
                        MarkdownTag::TableRow => {
                            builder.table.start_row();
                        }
                        MarkdownTag::TableCell => {
                            let is_header = builder.table.in_head;
                            let row_index = builder.table.row_index;
                            let col_index = builder.table.col_index;

                            builder.push_div(
                                div()
                                    .when(col_index > 0, |this| this.border_l_1())
                                    .when(row_index > 0, |this| this.border_t_1())
                                    .border_color(cx.theme().colors().border)
                                    .px_1()
                                    .py_0p5()
                                    .when(is_header, |this| {
                                        this.bg(cx.theme().colors().title_bar_background)
                                    })
                                    .when(!is_header && row_index % 2 == 1, |this| {
                                        this.bg(cx.theme().colors().panel_background)
                                    }),
                                range,
                                markdown_end,
                            );
                        }
                        _ => log::debug!("unsupported markdown tag {:?}", tag),
                    }
                }
                MarkdownEvent::End(tag) => match tag {
                    MarkdownTagEnd::Image => {
                        current_img_block_range.take();
                    }
                    MarkdownTagEnd::Paragraph => {
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Heading(_) => {
                        builder.pop_div();
                        builder.pop_text_style()
                    }
                    MarkdownTagEnd::BlockQuote(_kind) => {
                        builder.pop_text_style();
                        builder.pop_div()
                    }
                    MarkdownTagEnd::CodeBlock => {
                        builder.trim_trailing_newline();

                        builder.pop_div();
                        builder.pop_code_block();
                        builder.pop_text_style();

                        if let CodeBlockRenderer::Default {
                            copy_button: true, ..
                        } = &self.code_block_renderer
                        {
                            builder.modify_current_div(|el| {
                                let content_range = parser::extract_code_block_content_range(
                                    &parsed_markdown.source()[range.clone()],
                                );
                                let content_range = content_range.start + range.start
                                    ..content_range.end + range.start;

                                let code = parsed_markdown.source()[content_range].to_string();
                                let codeblock = render_copy_code_block_button(
                                    range.end,
                                    code,
                                    self.markdown.clone(),
                                );
                                el.child(
                                    h_flex()
                                        .w_4()
                                        .absolute()
                                        .top_1p5()
                                        .right_1p5()
                                        .justify_end()
                                        .child(codeblock),
                                )
                            });
                        }

                        if let CodeBlockRenderer::Default {
                            copy_button_on_hover: true,
                            ..
                        } = &self.code_block_renderer
                        {
                            builder.modify_current_div(|el| {
                                let content_range = parser::extract_code_block_content_range(
                                    &parsed_markdown.source()[range.clone()],
                                );
                                let content_range = content_range.start + range.start
                                    ..content_range.end + range.start;

                                let code = parsed_markdown.source()[content_range].to_string();
                                let codeblock = render_copy_code_block_button(
                                    range.end,
                                    code,
                                    self.markdown.clone(),
                                );
                                el.child(
                                    h_flex()
                                        .w_4()
                                        .absolute()
                                        .top_0()
                                        .right_0()
                                        .justify_end()
                                        .visible_on_hover("code_block")
                                        .child(codeblock),
                                )
                            });
                        }

                        // Pop the parent container.
                        builder.pop_div();
                    }
                    MarkdownTagEnd::HtmlBlock => builder.pop_div(),
                    MarkdownTagEnd::List(_) => {
                        builder.pop_list();
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Item => {
                        builder.pop_div();
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Emphasis => builder.pop_text_style(),
                    MarkdownTagEnd::Strong => builder.pop_text_style(),
                    MarkdownTagEnd::Strikethrough => builder.pop_text_style(),
                    MarkdownTagEnd::Link => {
                        if builder.code_block_stack.is_empty() {
                            builder.pop_text_style()
                        }
                    }
                    MarkdownTagEnd::Table => {
                        builder.pop_div();
                        builder.table.end();
                    }
                    MarkdownTagEnd::TableHead => {
                        builder.pop_text_style();
                        builder.table.end_head();
                    }
                    MarkdownTagEnd::TableRow => {
                        builder.table.end_row();
                    }
                    MarkdownTagEnd::TableCell => {
                        builder.pop_div();
                        builder.table.end_cell();
                    }
                    _ => log::debug!("unsupported markdown tag end: {:?}", tag),
                },
                MarkdownEvent::Text => {
                    builder.push_text(&parsed_markdown.source[range.clone()], range.clone());
                }
                MarkdownEvent::SubstitutedText(text) => {
                    builder.push_text(text, range.clone());
                }
                MarkdownEvent::Code => {
                    builder.push_text_style(self.style.inline_code.clone());
                    builder.push_text(&parsed_markdown.source[range.clone()], range.clone());
                    builder.pop_text_style();
                }
                MarkdownEvent::Html => {
                    let html = &parsed_markdown.source[range.clone()];
                    if html.starts_with("<!--") {
                        builder.html_comment = true;
                    }
                    if html.trim_end().ends_with("-->") {
                        builder.html_comment = false;
                        continue;
                    }
                    if builder.html_comment {
                        continue;
                    }
                    builder.push_text(html, range.clone());
                }
                MarkdownEvent::InlineHtml => {
                    let html = &parsed_markdown.source[range.clone()];
                    if html.starts_with("<code>") {
                        builder.push_text_style(self.style.inline_code.clone());
                        continue;
                    }
                    if html.trim_end().starts_with("</code>") {
                        builder.pop_text_style();
                        continue;
                    }
                    builder.push_text(&parsed_markdown.source[range.clone()], range.clone());
                }
                MarkdownEvent::Rule => {
                    builder.push_div(
                        div()
                            .border_b_1()
                            .my_2()
                            .border_color(self.style.rule_color),
                        range,
                        markdown_end,
                    );
                    builder.pop_div()
                }
                MarkdownEvent::SoftBreak => builder.push_text(" ", range.clone()),
                MarkdownEvent::HardBreak => builder.push_text("\n", range.clone()),
                MarkdownEvent::TaskListMarker(_) => {
                    // handled inside the `MarkdownTag::Item` case
                }
                _ => log::debug!("unsupported markdown event {:?}", event),
            }
        }
        if self.style.code_block_overflow_x_scroll {
            let code_block_ids = code_block_ids;
            self.markdown.update(cx, move |markdown, _| {
                markdown.retain_code_block_scroll_handles(&code_block_ids);
            });
        } else {
            self.markdown
                .update(cx, |markdown, _| markdown.clear_code_block_scroll_handles());
        }
        let mut rendered_markdown = builder.build();
        let child_layout_id = rendered_markdown.element.request_layout(window, cx);
        let layout_id = window.request_layout(gpui::Style::default(), [child_layout_id], cx);
        (layout_id, rendered_markdown)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let focus_handle = self.markdown.read(cx).focus_handle.clone();
        window.set_focus_handle(&focus_handle, cx);
        window.set_view_id(self.markdown.entity_id());

        let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);
        rendered_markdown.element.prepaint(window, cx);
        self.autoscroll(&rendered_markdown.text, window, cx);
        hitbox
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let mut context = KeyContext::default();
        context.add("Markdown");
        window.set_key_context(context);
        window.on_action(std::any::TypeId::of::<crate::Copy>(), {
            let entity = self.markdown.clone();
            let text = rendered_markdown.text.clone();
            move |_, phase, window, cx| {
                let text = text.clone();
                if phase == DispatchPhase::Bubble {
                    entity.update(cx, move |this, cx| this.copy(&text, window, cx))
                }
            }
        });
        window.on_action(std::any::TypeId::of::<crate::CopyAsMarkdown>(), {
            let entity = self.markdown.clone();
            move |_, phase, window, cx| {
                if phase == DispatchPhase::Bubble {
                    entity.update(cx, move |this, cx| this.copy_as_markdown(window, cx))
                }
            }
        });

        self.paint_mouse_listeners(hitbox, &rendered_markdown.text, window, cx);
        rendered_markdown.element.paint(window, cx);
        self.paint_selection(bounds, &rendered_markdown.text, window, cx);
    }
}

fn apply_heading_style(
    mut heading: Div,
    level: pulldown_cmark::HeadingLevel,
    custom_styles: Option<&HeadingLevelStyles>,
) -> Div {
    heading = match level {
        pulldown_cmark::HeadingLevel::H1 => heading.text_3xl(),
        pulldown_cmark::HeadingLevel::H2 => heading.text_2xl(),
        pulldown_cmark::HeadingLevel::H3 => heading.text_xl(),
        pulldown_cmark::HeadingLevel::H4 => heading.text_lg(),
        pulldown_cmark::HeadingLevel::H5 => heading.text_base(),
        pulldown_cmark::HeadingLevel::H6 => heading.text_sm(),
    };

    if let Some(styles) = custom_styles {
        let style_opt = match level {
            pulldown_cmark::HeadingLevel::H1 => &styles.h1,
            pulldown_cmark::HeadingLevel::H2 => &styles.h2,
            pulldown_cmark::HeadingLevel::H3 => &styles.h3,
            pulldown_cmark::HeadingLevel::H4 => &styles.h4,
            pulldown_cmark::HeadingLevel::H5 => &styles.h5,
            pulldown_cmark::HeadingLevel::H6 => &styles.h6,
        };

        if let Some(style) = style_opt {
            heading.style().text = style.clone();
        }
    }

    heading
}

fn render_copy_code_block_button(
    id: usize,
    code: String,
    markdown: Entity<Markdown>,
) -> impl IntoElement {
    let id = ElementId::named_usize("copy-markdown-code", id);

    CopyButton::new(code.clone()).custom_on_click({
        let markdown = markdown;
        move |_window, cx| {
            let id = id.clone();
            markdown.update(cx, |this, cx| {
                this.copied_code_blocks.insert(id.clone());

                cx.write_to_clipboard(ClipboardItem::new_string(code.clone()));

                cx.spawn(async move |this, cx| {
                    cx.background_executor().timer(Duration::from_secs(2)).await;

                    cx.update(|cx| {
                        this.update(cx, |this, cx| {
                            this.copied_code_blocks.remove(&id);
                            cx.notify();
                        })
                    })
                    .ok();
                })
                .detach();
            });
        }
    })
}

impl IntoElement for MarkdownElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

pub enum AnyDiv {
    Div(Div),
    Stateful(Stateful<Div>),
}

impl AnyDiv {
    fn into_any_element(self) -> AnyElement {
        match self {
            Self::Div(div) => div.into_any_element(),
            Self::Stateful(div) => div.into_any_element(),
        }
    }
}

impl From<Div> for AnyDiv {
    fn from(value: Div) -> Self {
        Self::Div(value)
    }
}

impl From<Stateful<Div>> for AnyDiv {
    fn from(value: Stateful<Div>) -> Self {
        Self::Stateful(value)
    }
}

impl Styled for AnyDiv {
    fn style(&mut self) -> &mut StyleRefinement {
        match self {
            Self::Div(div) => div.style(),
            Self::Stateful(div) => div.style(),
        }
    }
}

impl ParentElement for AnyDiv {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        match self {
            Self::Div(div) => div.extend(elements),
            Self::Stateful(div) => div.extend(elements),
        }
    }
}

#[derive(Default)]
struct TableState {
    alignments: Vec<Alignment>,
    in_head: bool,
    row_index: usize,
    col_index: usize,
}

impl TableState {
    fn start(&mut self, alignments: Vec<Alignment>) {
        self.alignments = alignments;
        self.in_head = false;
        self.row_index = 0;
        self.col_index = 0;
    }

    fn end(&mut self) {
        self.alignments.clear();
        self.in_head = false;
        self.row_index = 0;
        self.col_index = 0;
    }

    fn start_head(&mut self) {
        self.in_head = true;
    }

    fn end_head(&mut self) {
        self.in_head = false;
    }

    fn start_row(&mut self) {
        self.col_index = 0;
    }

    fn end_row(&mut self) {
        self.row_index += 1;
    }

    fn end_cell(&mut self) {
        self.col_index += 1;
    }
}

struct MarkdownElementBuilder {
    div_stack: Vec<AnyDiv>,
    rendered_lines: Vec<RenderedLine>,
    pending_line: PendingLine,
    rendered_links: Vec<RenderedLink>,
    current_source_index: usize,
    html_comment: bool,
    base_text_style: TextStyle,
    text_style_stack: Vec<TextStyleRefinement>,
    code_block_stack: Vec<Option<Arc<Language>>>,
    list_stack: Vec<ListStackEntry>,
    table: TableState,
    syntax_theme: Arc<SyntaxTheme>,
}

#[derive(Default)]
struct PendingLine {
    text: String,
    runs: Vec<TextRun>,
    source_mappings: Vec<SourceMapping>,
}

struct ListStackEntry {
    bullet_index: Option<u64>,
}

impl MarkdownElementBuilder {
    fn new(
        container_style: &StyleRefinement,
        base_text_style: TextStyle,
        syntax_theme: Arc<SyntaxTheme>,
    ) -> Self {
        Self {
            div_stack: vec![{
                let mut base_div = div();
                base_div.style().refine(container_style);
                base_div.debug_selector(|| "inner".into()).into()
            }],
            rendered_lines: Vec::new(),
            pending_line: PendingLine::default(),
            rendered_links: Vec::new(),
            current_source_index: 0,
            html_comment: false,
            base_text_style,
            text_style_stack: Vec::new(),
            code_block_stack: Vec::new(),
            list_stack: Vec::new(),
            table: TableState::default(),
            syntax_theme,
        }
    }

    fn push_text_style(&mut self, style: TextStyleRefinement) {
        self.text_style_stack.push(style);
    }

    fn text_style(&self) -> TextStyle {
        let mut style = self.base_text_style.clone();
        for refinement in &self.text_style_stack {
            style.refine(refinement);
        }
        style
    }

    fn pop_text_style(&mut self) {
        self.text_style_stack.pop();
    }

    fn push_div(&mut self, div: impl Into<AnyDiv>, range: &Range<usize>, markdown_end: usize) {
        let mut div = div.into();
        self.flush_text();

        if range.start == 0 {
            // Remove the top margin on the first element.
            div.style().refine(&StyleRefinement {
                margin: gpui::EdgesRefinement {
                    top: Some(Length::Definite(px(0.).into())),
                    left: None,
                    right: None,
                    bottom: None,
                },
                ..Default::default()
            });
        }

        if range.end == markdown_end {
            div.style().refine(&StyleRefinement {
                margin: gpui::EdgesRefinement {
                    top: None,
                    left: None,
                    right: None,
                    bottom: Some(Length::Definite(rems(0.).into())),
                },
                ..Default::default()
            });
        }

        self.div_stack.push(div);
    }

    fn modify_current_div(&mut self, f: impl FnOnce(AnyDiv) -> AnyDiv) {
        self.flush_text();
        if let Some(div) = self.div_stack.pop() {
            self.div_stack.push(f(div));
        }
    }

    fn pop_div(&mut self) {
        self.flush_text();
        let div = self.div_stack.pop().unwrap().into_any_element();
        self.div_stack.last_mut().unwrap().extend(iter::once(div));
    }

    fn push_list(&mut self, bullet_index: Option<u64>) {
        self.list_stack.push(ListStackEntry { bullet_index });
    }

    fn next_bullet_index(&mut self) -> Option<u64> {
        self.list_stack.last_mut().and_then(|entry| {
            let item_index = entry.bullet_index.as_mut()?;
            *item_index += 1;
            Some(*item_index - 1)
        })
    }

    fn pop_list(&mut self) {
        self.list_stack.pop();
    }

    fn push_code_block(&mut self, language: Option<Arc<Language>>) {
        self.code_block_stack.push(language);
    }

    fn pop_code_block(&mut self) {
        self.code_block_stack.pop();
    }

    fn push_link(&mut self, destination_url: SharedString, source_range: Range<usize>) {
        self.rendered_links.push(RenderedLink {
            source_range,
            destination_url,
        });
    }

    fn push_text(&mut self, text: &str, source_range: Range<usize>) {
        self.pending_line.source_mappings.push(SourceMapping {
            rendered_index: self.pending_line.text.len(),
            source_index: source_range.start,
        });
        self.pending_line.text.push_str(text);
        self.current_source_index = source_range.end;

        if let Some(Some(language)) = self.code_block_stack.last() {
            let mut offset = 0;
            for (range, highlight_id) in language.highlight_text(&Rope::from(text), 0..text.len()) {
                if range.start > offset {
                    self.pending_line
                        .runs
                        .push(self.text_style().to_run(range.start - offset));
                }

                let mut run_style = self.text_style();
                if let Some(highlight) = highlight_id.style(&self.syntax_theme) {
                    run_style = run_style.highlight(highlight);
                }
                self.pending_line.runs.push(run_style.to_run(range.len()));
                offset = range.end;
            }

            if offset < text.len() {
                self.pending_line
                    .runs
                    .push(self.text_style().to_run(text.len() - offset));
            }
        } else {
            self.pending_line
                .runs
                .push(self.text_style().to_run(text.len()));
        }
    }

    fn trim_trailing_newline(&mut self) {
        if self.pending_line.text.ends_with('\n') {
            self.pending_line
                .text
                .truncate(self.pending_line.text.len() - 1);
            self.pending_line.runs.last_mut().unwrap().len -= 1;
            self.current_source_index -= 1;
        }
    }

    fn flush_text(&mut self) {
        let line = mem::take(&mut self.pending_line);
        if line.text.is_empty() {
            return;
        }

        let text = StyledText::new(line.text).with_runs(line.runs);
        self.rendered_lines.push(RenderedLine {
            layout: text.layout().clone(),
            source_mappings: line.source_mappings,
            source_end: self.current_source_index,
        });
        self.div_stack.last_mut().unwrap().extend([text.into_any()]);
    }

    fn build(mut self) -> RenderedMarkdown {
        debug_assert_eq!(self.div_stack.len(), 1);
        self.flush_text();
        RenderedMarkdown {
            element: self.div_stack.pop().unwrap().into_any_element(),
            text: RenderedText {
                lines: self.rendered_lines.into(),
                links: self.rendered_links.into(),
            },
        }
    }
}

struct RenderedLine {
    layout: TextLayout,
    source_mappings: Vec<SourceMapping>,
    source_end: usize,
}

impl RenderedLine {
    fn rendered_index_for_source_index(&self, source_index: usize) -> usize {
        if source_index >= self.source_end {
            return self.layout.len();
        }

        let mapping = match self
            .source_mappings
            .binary_search_by_key(&source_index, |probe| probe.source_index)
        {
            Ok(ix) => &self.source_mappings[ix],
            Err(ix) => &self.source_mappings[ix - 1],
        };
        mapping.rendered_index + (source_index - mapping.source_index)
    }

    fn source_index_for_rendered_index(&self, rendered_index: usize) -> usize {
        if rendered_index >= self.layout.len() {
            return self.source_end;
        }

        let mapping = match self
            .source_mappings
            .binary_search_by_key(&rendered_index, |probe| probe.rendered_index)
        {
            Ok(ix) => &self.source_mappings[ix],
            Err(ix) => &self.source_mappings[ix - 1],
        };
        mapping.source_index + (rendered_index - mapping.rendered_index)
    }

    fn source_index_for_position(&self, position: Point<Pixels>) -> Result<usize, usize> {
        let line_rendered_index;
        let out_of_bounds;
        match self.layout.index_for_position(position) {
            Ok(ix) => {
                line_rendered_index = ix;
                out_of_bounds = false;
            }
            Err(ix) => {
                line_rendered_index = ix;
                out_of_bounds = true;
            }
        };
        let source_index = self.source_index_for_rendered_index(line_rendered_index);
        if out_of_bounds {
            Err(source_index)
        } else {
            Ok(source_index)
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct SourceMapping {
    rendered_index: usize,
    source_index: usize,
}

pub struct RenderedMarkdown {
    element: AnyElement,
    text: RenderedText,
}

#[derive(Clone)]
struct RenderedText {
    lines: Rc<[RenderedLine]>,
    links: Rc<[RenderedLink]>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RenderedLink {
    source_range: Range<usize>,
    destination_url: SharedString,
}

impl RenderedText {
    fn source_index_for_position(&self, position: Point<Pixels>) -> Result<usize, usize> {
        let mut lines = self.lines.iter().peekable();

        while let Some(line) = lines.next() {
            let line_bounds = line.layout.bounds();
            if position.y > line_bounds.bottom() {
                if let Some(next_line) = lines.peek()
                    && position.y < next_line.layout.bounds().top()
                {
                    return Err(line.source_end);
                }

                continue;
            }

            return line.source_index_for_position(position);
        }

        Err(self.lines.last().map_or(0, |line| line.source_end))
    }

    fn position_for_source_index(&self, source_index: usize) -> Option<(Point<Pixels>, Pixels)> {
        for line in self.lines.iter() {
            let line_source_start = line.source_mappings.first().unwrap().source_index;
            if source_index < line_source_start {
                break;
            } else if source_index > line.source_end {
                continue;
            } else {
                let line_height = line.layout.line_height();
                let rendered_index_within_line = line.rendered_index_for_source_index(source_index);
                let position = line.layout.position_for_index(rendered_index_within_line)?;
                return Some((position, line_height));
            }
        }
        None
    }

    fn surrounding_word_range(&self, source_index: usize) -> Range<usize> {
        for line in self.lines.iter() {
            if source_index > line.source_end {
                continue;
            }

            let line_rendered_start = line.source_mappings.first().unwrap().rendered_index;
            let rendered_index_in_line =
                line.rendered_index_for_source_index(source_index) - line_rendered_start;
            let text = line.layout.text();
            let previous_space = if let Some(idx) = text[0..rendered_index_in_line].rfind(' ') {
                idx + ' '.len_utf8()
            } else {
                0
            };
            let next_space = if let Some(idx) = text[rendered_index_in_line..].find(' ') {
                rendered_index_in_line + idx
            } else {
                text.len()
            };

            return line.source_index_for_rendered_index(line_rendered_start + previous_space)
                ..line.source_index_for_rendered_index(line_rendered_start + next_space);
        }

        source_index..source_index
    }

    fn surrounding_line_range(&self, source_index: usize) -> Range<usize> {
        for line in self.lines.iter() {
            if source_index > line.source_end {
                continue;
            }
            let line_source_start = line.source_mappings.first().unwrap().source_index;
            return line_source_start..line.source_end;
        }

        source_index..source_index
    }

    fn text_for_range(&self, range: Range<usize>) -> String {
        let mut accumulator = String::new();

        for line in self.lines.iter() {
            if range.start > line.source_end {
                continue;
            }
            let line_source_start = line.source_mappings.first().unwrap().source_index;
            if range.end < line_source_start {
                break;
            }

            let text = line.layout.text();

            let start = if range.start < line_source_start {
                0
            } else {
                line.rendered_index_for_source_index(range.start)
            };
            let end = if range.end > line.source_end {
                line.rendered_index_for_source_index(line.source_end)
            } else {
                line.rendered_index_for_source_index(range.end)
            }
            .min(text.len());

            accumulator.push_str(&text[start..end]);
            accumulator.push('\n');
        }
        // Remove trailing newline
        accumulator.pop();
        accumulator
    }

    fn link_for_position(&self, position: Point<Pixels>) -> Option<&RenderedLink> {
        let source_index = self.source_index_for_position(position).ok()?;
        self.links
            .iter()
            .find(|link| link.source_range.contains(&source_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, size};

    #[gpui::test]
    fn test_mappings(cx: &mut TestAppContext) {
        // Formatting.
        assert_mappings(
            &render_markdown("He*l*lo", cx),
            vec![vec![(0, 0), (1, 1), (2, 3), (3, 5), (4, 6), (5, 7)]],
        );

        // Multiple lines.
        assert_mappings(
            &render_markdown("Hello\n\nWorld", cx),
            vec![
                vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)],
                vec![(0, 7), (1, 8), (2, 9), (3, 10), (4, 11), (5, 12)],
            ],
        );

        // Multi-byte characters.
        assert_mappings(
            &render_markdown("αβγ\n\nδεζ", cx),
            vec![
                vec![(0, 0), (2, 2), (4, 4), (6, 6)],
                vec![(0, 8), (2, 10), (4, 12), (6, 14)],
            ],
        );

        // Smart quotes.
        assert_mappings(&render_markdown("\"", cx), vec![vec![(0, 0), (3, 1)]]);
        assert_mappings(
            &render_markdown("\"hey\"", cx),
            vec![vec![(0, 0), (3, 1), (4, 2), (5, 3), (6, 4), (9, 5)]],
        );

        // HTML Comments are ignored
        assert_mappings(
            &render_markdown(
                "<!--\nrdoc-file=string.c\n- str.intern   -> symbol\n- str.to_sym   -> symbol\n-->\nReturns",
                cx,
            ),
            vec![vec![
                (0, 78),
                (1, 79),
                (2, 80),
                (3, 81),
                (4, 82),
                (5, 83),
                (6, 84),
            ]],
        );
    }

    fn render_markdown(markdown: &str, cx: &mut TestAppContext) -> RenderedText {
        struct TestWindow;

        impl Render for TestWindow {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
            }
        }

        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| Markdown::new(markdown.to_string().into(), None, None, cx));
        cx.run_until_parked();
        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| MarkdownElement::new(markdown, MarkdownStyle::default()),
        );
        rendered.text
    }

    #[gpui::test]
    fn test_surrounding_word_range(cx: &mut TestAppContext) {
        let rendered = render_markdown("Hello world tesεζ", cx);

        // Test word selection for "Hello"
        let word_range = rendered.surrounding_word_range(2); // Simulate click on 'l' in "Hello"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "Hello");

        // Test word selection for "world"
        let word_range = rendered.surrounding_word_range(7); // Simulate click on 'o' in "world"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "world");

        // Test word selection for "tesεζ"
        let word_range = rendered.surrounding_word_range(14); // Simulate click on 's' in "tesεζ"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "tesεζ");

        // Test word selection at word boundary (space)
        let word_range = rendered.surrounding_word_range(5); // Simulate click on space between "Hello" and "world", expect highlighting word to the left
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "Hello");
    }

    #[gpui::test]
    fn test_surrounding_line_range(cx: &mut TestAppContext) {
        let rendered = render_markdown("First line\n\nSecond line\n\nThird lineεζ", cx);

        // Test getting line range for first line
        let line_range = rendered.surrounding_line_range(5); // Simulate click somewhere in first line
        let selected_text = rendered.text_for_range(line_range);
        assert_eq!(selected_text, "First line");

        // Test getting line range for second line
        let line_range = rendered.surrounding_line_range(13); // Simulate click at beginning in second line
        let selected_text = rendered.text_for_range(line_range);
        assert_eq!(selected_text, "Second line");

        // Test getting line range for third line
        let line_range = rendered.surrounding_line_range(37); // Simulate click at end of third line with multi-byte chars
        let selected_text = rendered.text_for_range(line_range);
        assert_eq!(selected_text, "Third lineεζ");
    }

    #[gpui::test]
    fn test_selection_head_movement(cx: &mut TestAppContext) {
        let rendered = render_markdown("Hello world test", cx);

        let mut selection = Selection {
            start: 5,
            end: 5,
            reversed: false,
            pending: false,
            mode: SelectMode::Character,
        };

        // Test forward selection
        selection.set_head(10, &rendered);
        assert_eq!(selection.start, 5);
        assert_eq!(selection.end, 10);
        assert!(!selection.reversed);
        assert_eq!(selection.tail(), 5);

        // Test backward selection
        selection.set_head(2, &rendered);
        assert_eq!(selection.start, 2);
        assert_eq!(selection.end, 5);
        assert!(selection.reversed);
        assert_eq!(selection.tail(), 5);

        // Test forward selection again from reversed state
        selection.set_head(15, &rendered);
        assert_eq!(selection.start, 5);
        assert_eq!(selection.end, 15);
        assert!(!selection.reversed);
        assert_eq!(selection.tail(), 5);
    }

    #[gpui::test]
    fn test_word_selection_drag(cx: &mut TestAppContext) {
        let rendered = render_markdown("Hello world test", cx);

        // Start with a simulated double-click on "world" (index 6-10)
        let word_range = rendered.surrounding_word_range(7); // Click on 'o' in "world"
        let mut selection = Selection {
            start: word_range.start,
            end: word_range.end,
            reversed: false,
            pending: true,
            mode: SelectMode::Word(word_range),
        };

        // Drag forward to "test" - should expand selection to include "test"
        selection.set_head(13, &rendered); // Index in "test"
        assert_eq!(selection.start, 6); // Start of "world"
        assert_eq!(selection.end, 16); // End of "test"
        assert!(!selection.reversed);
        let selected_text = rendered.text_for_range(selection.start..selection.end);
        assert_eq!(selected_text, "world test");

        // Drag backward to "Hello" - should expand selection to include "Hello"
        selection.set_head(2, &rendered); // Index in "Hello"
        assert_eq!(selection.start, 0); // Start of "Hello"
        assert_eq!(selection.end, 11); // End of "world" (original selection)
        assert!(selection.reversed);
        let selected_text = rendered.text_for_range(selection.start..selection.end);
        assert_eq!(selected_text, "Hello world");

        // Drag back within original word - should revert to original selection
        selection.set_head(8, &rendered); // Back within "world"
        assert_eq!(selection.start, 6); // Start of "world"
        assert_eq!(selection.end, 11); // End of "world"
        assert!(!selection.reversed);
        let selected_text = rendered.text_for_range(selection.start..selection.end);
        assert_eq!(selected_text, "world");
    }

    #[gpui::test]
    fn test_selection_with_markdown_formatting(cx: &mut TestAppContext) {
        let rendered = render_markdown(
            "This is **bold** text, this is *italic* text, use `code` here",
            cx,
        );
        let word_range = rendered.surrounding_word_range(10); // Inside "bold"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "bold");

        let word_range = rendered.surrounding_word_range(32); // Inside "italic"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "italic");

        let word_range = rendered.surrounding_word_range(51); // Inside "code"
        let selected_text = rendered.text_for_range(word_range);
        assert_eq!(selected_text, "code");
    }

    #[gpui::test]
    fn test_all_selection(cx: &mut TestAppContext) {
        let rendered = render_markdown("Hello world\n\nThis is a test\n\nwith multiple lines", cx);

        let total_length = rendered
            .lines
            .last()
            .map(|line| line.source_end)
            .unwrap_or(0);

        let mut selection = Selection {
            start: 0,
            end: total_length,
            reversed: false,
            pending: true,
            mode: SelectMode::All,
        };

        selection.set_head(5, &rendered); // Try to set head in middle
        assert_eq!(selection.start, 0);
        assert_eq!(selection.end, total_length);
        assert!(!selection.reversed);

        selection.set_head(25, &rendered); // Try to set head near end
        assert_eq!(selection.start, 0);
        assert_eq!(selection.end, total_length);
        assert!(!selection.reversed);

        let selected_text = rendered.text_for_range(selection.start..selection.end);
        assert_eq!(
            selected_text,
            "Hello world\nThis is a test\nwith multiple lines"
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(Markdown::escape("hello `world`"), "hello \\`world\\`");
        assert_eq!(
            Markdown::escape("hello\n    cool world"),
            "hello\n\ncool world"
        );
    }

    #[track_caller]
    fn assert_mappings(rendered: &RenderedText, expected: Vec<Vec<(usize, usize)>>) {
        assert_eq!(rendered.lines.len(), expected.len(), "line count mismatch");
        for (line_ix, line_mappings) in expected.into_iter().enumerate() {
            let line = &rendered.lines[line_ix];

            assert!(
                line.source_mappings.windows(2).all(|mappings| {
                    mappings[0].source_index < mappings[1].source_index
                        && mappings[0].rendered_index < mappings[1].rendered_index
                }),
                "line {} has duplicate mappings: {:?}",
                line_ix,
                line.source_mappings
            );

            for (rendered_ix, source_ix) in line_mappings {
                assert_eq!(
                    line.source_index_for_rendered_index(rendered_ix),
                    source_ix,
                    "line {}, rendered_ix {}",
                    line_ix,
                    rendered_ix
                );

                assert_eq!(
                    line.rendered_index_for_source_index(source_ix),
                    rendered_ix,
                    "line {}, source_ix {}",
                    line_ix,
                    source_ix
                );
            }
        }
    }
}
