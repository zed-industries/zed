mod parser;

use crate::parser::CodeBlockKind;
use futures::FutureExt;
use gpui::{
    actions, point, quad, AnyElement, AppContext, Bounds, ClipboardItem, CursorStyle,
    DispatchPhase, Edges, FocusHandle, FocusableView, FontStyle, FontWeight, GlobalElementId,
    Hitbox, Hsla, KeyContext, MouseDownEvent, MouseEvent, MouseMoveEvent, MouseUpEvent, Point,
    Render, StrikethroughStyle, Style, StyledText, Task, TextLayout, TextRun, TextStyle,
    TextStyleRefinement, View,
};
use language::{Language, LanguageRegistry, Rope};
use parser::{parse_markdown, MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::{iter, mem, ops::Range, rc::Rc, sync::Arc};
use theme::SyntaxTheme;
use ui::prelude::*;
use util::{ResultExt, TryFutureExt};

#[derive(Clone)]
pub struct MarkdownStyle {
    pub code_block: TextStyleRefinement,
    pub inline_code: TextStyleRefinement,
    pub block_quote: TextStyleRefinement,
    pub link: TextStyleRefinement,
    pub rule_color: Hsla,
    pub block_quote_border_color: Hsla,
    pub syntax: Arc<SyntaxTheme>,
    pub selection_background_color: Hsla,
}

pub struct Markdown {
    source: String,
    selection: Selection,
    pressed_link: Option<RenderedLink>,
    autoscroll_request: Option<usize>,
    style: MarkdownStyle,
    parsed_markdown: ParsedMarkdown,
    should_reparse: bool,
    pending_parse: Option<Task<Option<()>>>,
    focus_handle: FocusHandle,
    language_registry: Option<Arc<LanguageRegistry>>,
}

actions!(markdown, [Copy]);

impl Markdown {
    pub fn new(
        source: String,
        style: MarkdownStyle,
        language_registry: Option<Arc<LanguageRegistry>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let mut this = Self {
            source,
            selection: Selection::default(),
            pressed_link: None,
            autoscroll_request: None,
            style,
            should_reparse: false,
            parsed_markdown: ParsedMarkdown::default(),
            pending_parse: None,
            focus_handle,
            language_registry,
        };
        this.parse(cx);
        this
    }

    pub fn append(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.source.push_str(text);
        self.parse(cx);
    }

    pub fn reset(&mut self, source: String, cx: &mut ViewContext<Self>) {
        if source == self.source() {
            return;
        }
        self.source = source;
        self.selection = Selection::default();
        self.autoscroll_request = None;
        self.pending_parse = None;
        self.should_reparse = false;
        self.parsed_markdown = ParsedMarkdown::default();
        self.parse(cx);
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    fn copy(&self, text: &RenderedText, cx: &mut ViewContext<Self>) {
        let text = text.text_for_range(self.selection.start..self.selection.end);
        cx.write_to_clipboard(ClipboardItem::new(text));
    }

    fn parse(&mut self, cx: &mut ViewContext<Self>) {
        if self.source.is_empty() {
            return;
        }

        if self.pending_parse.is_some() {
            self.should_reparse = true;
            return;
        }

        let text = self.source.clone();
        let parsed = cx.background_executor().spawn(async move {
            let text = SharedString::from(text);
            let events = Arc::from(parse_markdown(text.as_ref()));
            anyhow::Ok(ParsedMarkdown {
                source: text,
                events,
            })
        });

        self.should_reparse = false;
        self.pending_parse = Some(cx.spawn(|this, mut cx| {
            async move {
                let parsed = parsed.await?;
                this.update(&mut cx, |this, cx| {
                    this.parsed_markdown = parsed;
                    this.pending_parse.take();
                    if this.should_reparse {
                        this.parse(cx);
                    }
                    cx.notify();
                })
                .ok();
                anyhow::Ok(())
            }
            .log_err()
        }));
    }
}

impl Render for Markdown {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        MarkdownElement::new(
            cx.view().clone(),
            self.style.clone(),
            self.language_registry.clone(),
        )
    }
}

impl FocusableView for Markdown {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Selection {
    start: usize,
    end: usize,
    reversed: bool,
    pending: bool,
}

impl Selection {
    fn set_head(&mut self, head: usize) {
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

    fn tail(&self) -> usize {
        if self.reversed {
            self.end
        } else {
            self.start
        }
    }
}

#[derive(Clone)]
struct ParsedMarkdown {
    source: SharedString,
    events: Arc<[(Range<usize>, MarkdownEvent)]>,
}

impl Default for ParsedMarkdown {
    fn default() -> Self {
        Self {
            source: SharedString::default(),
            events: Arc::from([]),
        }
    }
}

pub struct MarkdownElement {
    markdown: View<Markdown>,
    style: MarkdownStyle,
    language_registry: Option<Arc<LanguageRegistry>>,
}

impl MarkdownElement {
    fn new(
        markdown: View<Markdown>,
        style: MarkdownStyle,
        language_registry: Option<Arc<LanguageRegistry>>,
    ) -> Self {
        Self {
            markdown,
            style,
            language_registry,
        }
    }

    fn load_language(&self, name: &str, cx: &mut WindowContext) -> Option<Arc<Language>> {
        let language = self
            .language_registry
            .as_ref()?
            .language_for_name(name)
            .map(|language| language.ok())
            .shared();

        match language.clone().now_or_never() {
            Some(language) => language,
            None => {
                let markdown = self.markdown.downgrade();
                cx.spawn(|mut cx| async move {
                    language.await;
                    markdown.update(&mut cx, |_, cx| cx.notify())
                })
                .detach_and_log_err(cx);
                None
            }
        }
    }

    fn paint_selection(
        &mut self,
        bounds: Bounds<Pixels>,
        rendered_text: &RenderedText,
        cx: &mut WindowContext,
    ) {
        let selection = self.markdown.read(cx).selection;
        let selection_start = rendered_text.position_for_source_index(selection.start);
        let selection_end = rendered_text.position_for_source_index(selection.end);

        if let Some(((start_position, start_line_height), (end_position, end_line_height))) =
            selection_start.zip(selection_end)
        {
            if start_position.y == end_position.y {
                cx.paint_quad(quad(
                    Bounds::from_corners(
                        start_position,
                        point(end_position.x, end_position.y + end_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                ));
            } else {
                cx.paint_quad(quad(
                    Bounds::from_corners(
                        start_position,
                        point(bounds.right(), start_position.y + start_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                ));

                if end_position.y > start_position.y + start_line_height {
                    cx.paint_quad(quad(
                        Bounds::from_corners(
                            point(bounds.left(), start_position.y + start_line_height),
                            point(bounds.right(), end_position.y),
                        ),
                        Pixels::ZERO,
                        self.style.selection_background_color,
                        Edges::default(),
                        Hsla::transparent_black(),
                    ));
                }

                cx.paint_quad(quad(
                    Bounds::from_corners(
                        point(bounds.left(), end_position.y),
                        point(end_position.x, end_position.y + end_line_height),
                    ),
                    Pixels::ZERO,
                    self.style.selection_background_color,
                    Edges::default(),
                    Hsla::transparent_black(),
                ));
            }
        }
    }

    fn paint_mouse_listeners(
        &mut self,
        hitbox: &Hitbox,
        rendered_text: &RenderedText,
        cx: &mut WindowContext,
    ) {
        let is_hovering_link = hitbox.is_hovered(cx)
            && !self.markdown.read(cx).selection.pending
            && rendered_text
                .link_for_position(cx.mouse_position())
                .is_some();

        if is_hovering_link {
            cx.set_cursor_style(CursorStyle::PointingHand, hitbox);
        } else {
            cx.set_cursor_style(CursorStyle::IBeam, hitbox);
        }

        self.on_mouse_event(cx, {
            let rendered_text = rendered_text.clone();
            let hitbox = hitbox.clone();
            move |markdown, event: &MouseDownEvent, phase, cx| {
                if hitbox.is_hovered(cx) {
                    if phase.bubble() {
                        if let Some(link) = rendered_text.link_for_position(event.position) {
                            markdown.pressed_link = Some(link.clone());
                        } else {
                            let source_index =
                                match rendered_text.source_index_for_position(event.position) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                            let range = if event.click_count == 2 {
                                rendered_text.surrounding_word_range(source_index)
                            } else if event.click_count == 3 {
                                rendered_text.surrounding_line_range(source_index)
                            } else {
                                source_index..source_index
                            };
                            markdown.selection = Selection {
                                start: range.start,
                                end: range.end,
                                reversed: false,
                                pending: true,
                            };
                            cx.focus(&markdown.focus_handle);
                            cx.prevent_default()
                        }

                        cx.notify();
                    }
                } else if phase.capture() {
                    markdown.selection = Selection::default();
                    markdown.pressed_link = None;
                    cx.notify();
                }
            }
        });
        self.on_mouse_event(cx, {
            let rendered_text = rendered_text.clone();
            let hitbox = hitbox.clone();
            let was_hovering_link = is_hovering_link;
            move |markdown, event: &MouseMoveEvent, phase, cx| {
                if phase.capture() {
                    return;
                }

                if markdown.selection.pending {
                    let source_index = match rendered_text.source_index_for_position(event.position)
                    {
                        Ok(ix) | Err(ix) => ix,
                    };
                    markdown.selection.set_head(source_index);
                    markdown.autoscroll_request = Some(source_index);
                    cx.notify();
                } else {
                    let is_hovering_link = hitbox.is_hovered(cx)
                        && rendered_text.link_for_position(event.position).is_some();
                    if is_hovering_link != was_hovering_link {
                        cx.notify();
                    }
                }
            }
        });
        self.on_mouse_event(cx, {
            let rendered_text = rendered_text.clone();
            move |markdown, event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    if let Some(pressed_link) = markdown.pressed_link.take() {
                        if Some(&pressed_link) == rendered_text.link_for_position(event.position) {
                            cx.open_url(&pressed_link.destination_url);
                        }
                    }
                } else {
                    if markdown.selection.pending {
                        markdown.selection.pending = false;
                        #[cfg(target_os = "linux")]
                        {
                            let text = rendered_text
                                .text_for_range(markdown.selection.start..markdown.selection.end);
                            cx.write_to_primary(ClipboardItem::new(text))
                        }
                        cx.notify();
                    }
                }
            }
        });
    }

    fn autoscroll(&mut self, rendered_text: &RenderedText, cx: &mut WindowContext) -> Option<()> {
        let autoscroll_index = self
            .markdown
            .update(cx, |markdown, _| markdown.autoscroll_request.take())?;
        let (position, line_height) = rendered_text.position_for_source_index(autoscroll_index)?;

        let text_style = cx.text_style();
        let font_id = cx.text_system().resolve_font(&text_style.font());
        let font_size = text_style.font_size.to_pixels(cx.rem_size());
        let em_width = cx
            .text_system()
            .typographic_bounds(font_id, font_size, 'm')
            .unwrap()
            .size
            .width;
        cx.request_autoscroll(Bounds::from_corners(
            point(position.x - 3. * em_width, position.y - 3. * line_height),
            point(position.x + 3. * em_width, position.y + 3. * line_height),
        ));
        Some(())
    }

    fn on_mouse_event<T: MouseEvent>(
        &self,
        cx: &mut WindowContext,
        mut f: impl 'static + FnMut(&mut Markdown, &T, DispatchPhase, &mut ViewContext<Markdown>),
    ) {
        cx.on_mouse_event({
            let markdown = self.markdown.downgrade();
            move |event, phase, cx| {
                markdown
                    .update(cx, |markdown, cx| f(markdown, event, phase, cx))
                    .log_err();
            }
        });
    }
}

impl Element for MarkdownElement {
    type RequestLayoutState = RenderedMarkdown;
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        let mut builder = MarkdownElementBuilder::new(cx.text_style(), self.style.syntax.clone());
        let parsed_markdown = self.markdown.read(cx).parsed_markdown.clone();
        for (range, event) in parsed_markdown.events.iter() {
            match event {
                MarkdownEvent::Start(tag) => {
                    match tag {
                        MarkdownTag::Paragraph => {
                            builder.push_div(div().mb_2().line_height(rems(1.3)));
                        }
                        MarkdownTag::Heading { level, .. } => {
                            let mut heading = div().mb_2();
                            heading = match level {
                                pulldown_cmark::HeadingLevel::H1 => heading.text_3xl(),
                                pulldown_cmark::HeadingLevel::H2 => heading.text_2xl(),
                                pulldown_cmark::HeadingLevel::H3 => heading.text_xl(),
                                pulldown_cmark::HeadingLevel::H4 => heading.text_lg(),
                                _ => heading,
                            };
                            builder.push_div(heading);
                        }
                        MarkdownTag::BlockQuote => {
                            builder.push_text_style(self.style.block_quote.clone());
                            builder.push_div(
                                div()
                                    .pl_4()
                                    .mb_2()
                                    .border_l_4()
                                    .border_color(self.style.block_quote_border_color),
                            );
                        }
                        MarkdownTag::CodeBlock(kind) => {
                            let language = if let CodeBlockKind::Fenced(language) = kind {
                                self.load_language(language.as_ref(), cx)
                            } else {
                                None
                            };

                            builder.push_code_block(language);
                            builder.push_text_style(self.style.code_block.clone());
                            builder.push_div(div().rounded_lg().p_4().mb_2().w_full().when_some(
                                self.style.code_block.background_color,
                                |div, color| div.bg(color),
                            ));
                        }
                        MarkdownTag::HtmlBlock => builder.push_div(div()),
                        MarkdownTag::List(bullet_index) => {
                            builder.push_list(*bullet_index);
                            builder.push_div(div().pl_4());
                        }
                        MarkdownTag::Item => {
                            let bullet = if let Some(bullet_index) = builder.next_bullet_index() {
                                format!("{}.", bullet_index)
                            } else {
                                "•".to_string()
                            };
                            builder.push_div(
                                div()
                                    .h_flex()
                                    .mb_2()
                                    .line_height(rems(1.3))
                                    .items_start()
                                    .gap_1()
                                    .child(bullet),
                            );
                            // Without `w_0`, text doesn't wrap to the width of the container.
                            builder.push_div(div().flex_1().w_0());
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
                                builder.push_text_style(self.style.link.clone())
                            }
                        }
                        _ => log::error!("unsupported markdown tag {:?}", tag),
                    }
                }
                MarkdownEvent::End(tag) => match tag {
                    MarkdownTagEnd::Paragraph => {
                        builder.pop_div();
                    }
                    MarkdownTagEnd::Heading(_) => builder.pop_div(),
                    MarkdownTagEnd::BlockQuote => {
                        builder.pop_text_style();
                        builder.pop_div()
                    }
                    MarkdownTagEnd::CodeBlock => {
                        builder.trim_trailing_newline();
                        builder.pop_div();
                        builder.pop_text_style();
                        builder.pop_code_block();
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
                    _ => log::error!("unsupported markdown tag end: {:?}", tag),
                },
                MarkdownEvent::Text => {
                    builder.push_text(&parsed_markdown.source[range.clone()], range.start);
                }
                MarkdownEvent::Code => {
                    builder.push_text_style(self.style.inline_code.clone());
                    builder.push_text(&parsed_markdown.source[range.clone()], range.start);
                    builder.pop_text_style();
                }
                MarkdownEvent::Html => {
                    builder.push_text(&parsed_markdown.source[range.clone()], range.start);
                }
                MarkdownEvent::InlineHtml => {
                    builder.push_text(&parsed_markdown.source[range.clone()], range.start);
                }
                MarkdownEvent::Rule => {
                    builder.push_div(
                        div()
                            .border_b_1()
                            .my_2()
                            .border_color(self.style.rule_color),
                    );
                    builder.pop_div()
                }
                MarkdownEvent::SoftBreak => builder.push_text("\n", range.start),
                MarkdownEvent::HardBreak => builder.push_text("\n", range.start),
                _ => log::error!("unsupported markdown event {:?}", event),
            }
        }

        let mut rendered_markdown = builder.build();
        let child_layout_id = rendered_markdown.element.request_layout(cx);
        let layout_id = cx.request_layout(Style::default(), [child_layout_id]);
        (layout_id, rendered_markdown)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        let hitbox = cx.insert_hitbox(bounds, false);
        rendered_markdown.element.prepaint(cx);
        self.autoscroll(&rendered_markdown.text, cx);
        hitbox
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        let focus_handle = self.markdown.read(cx).focus_handle.clone();
        cx.set_focus_handle(&focus_handle);

        let mut context = KeyContext::default();
        context.add("Markdown");
        cx.set_key_context(context);
        let view = self.markdown.clone();
        cx.on_action(std::any::TypeId::of::<crate::Copy>(), {
            let text = rendered_markdown.text.clone();
            move |_, phase, cx| {
                let text = text.clone();
                if phase == DispatchPhase::Bubble {
                    view.update(cx, move |this, cx| this.copy(&text, cx))
                }
            }
        });

        self.paint_mouse_listeners(hitbox, &rendered_markdown.text, cx);
        rendered_markdown.element.paint(cx);
        self.paint_selection(bounds, &rendered_markdown.text, cx);
    }
}

impl IntoElement for MarkdownElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct MarkdownElementBuilder {
    div_stack: Vec<Div>,
    rendered_lines: Vec<RenderedLine>,
    pending_line: PendingLine,
    rendered_links: Vec<RenderedLink>,
    current_source_index: usize,
    base_text_style: TextStyle,
    text_style_stack: Vec<TextStyleRefinement>,
    code_block_stack: Vec<Option<Arc<Language>>>,
    list_stack: Vec<ListStackEntry>,
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
    fn new(base_text_style: TextStyle, syntax_theme: Arc<SyntaxTheme>) -> Self {
        Self {
            div_stack: vec![div().debug_selector(|| "inner".into())],
            rendered_lines: Vec::new(),
            pending_line: PendingLine::default(),
            rendered_links: Vec::new(),
            current_source_index: 0,
            base_text_style,
            text_style_stack: Vec::new(),
            code_block_stack: Vec::new(),
            list_stack: Vec::new(),
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

    fn push_div(&mut self, div: Div) {
        self.flush_text();
        self.div_stack.push(div);
    }

    fn pop_div(&mut self) {
        self.flush_text();
        let div = self.div_stack.pop().unwrap().into_any();
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

    fn push_text(&mut self, text: &str, source_index: usize) {
        self.pending_line.source_mappings.push(SourceMapping {
            rendered_index: self.pending_line.text.len(),
            source_index,
        });
        self.pending_line.text.push_str(text);
        self.current_source_index = source_index + text.len();

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
            element: self.div_stack.pop().unwrap().into_any(),
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

#[derive(Clone, Eq, PartialEq)]
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
                if let Some(next_line) = lines.peek() {
                    if position.y < next_line.layout.bounds().top() {
                        return Err(line.source_end);
                    }
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
        let mut ret = vec![];

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

            ret.push(text[start..end].to_string());
        }
        ret.join("\n")
    }

    fn link_for_position(&self, position: Point<Pixels>) -> Option<&RenderedLink> {
        let source_index = self.source_index_for_position(position).ok()?;
        self.links
            .iter()
            .find(|link| link.source_range.contains(&source_index))
    }
}
