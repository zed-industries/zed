mod parser;

use crate::parser::CodeBlockKind;
use futures::FutureExt;
use gpui::{
    point, quad, AnyElement, Bounds, Edges, FontStyle, FontWeight, GlobalElementId, Hsla,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Point, Render, StrikethroughStyle,
    Style, StyledText, Task, TextLayout, TextRun, TextStyle, TextStyleRefinement, View,
};
use language::{Language, LanguageRegistry, Rope};
use parser::{parse_markdown, MarkdownEvent, MarkdownTag, MarkdownTagEnd};
use std::{iter, mem, ops::Range, sync::Arc};
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
}

pub struct Markdown {
    source: String,
    selection: Selection,
    is_selecting: bool,
    style: MarkdownStyle,
    parsed_markdown: ParsedMarkdown,
    should_reparse: bool,
    pending_parse: Option<Task<Option<()>>>,
    language_registry: Arc<LanguageRegistry>,
}

impl Markdown {
    pub fn new(
        source: String,
        style: MarkdownStyle,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            source: source.into(),
            selection: Selection::default(),
            is_selecting: false,
            style,
            should_reparse: false,
            parsed_markdown: ParsedMarkdown::default(),
            pending_parse: None,
            language_registry,
        };
        this.parse(cx);
        this
    }

    pub fn append(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.source.push_str(text);
        self.parse(cx);
    }

    pub fn source(&self) -> &str {
        &self.source
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

#[derive(Copy, Clone, Default, Debug)]
struct Selection {
    start: usize,
    end: usize,
    reversed: bool,
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
    language_registry: Arc<LanguageRegistry>,
}

impl MarkdownElement {
    fn new(
        markdown: View<Markdown>,
        style: MarkdownStyle,
        language_registry: Arc<LanguageRegistry>,
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
            .language_for_name(name)
            .log_err()
            .shared();

        match language.clone().now_or_never() {
            Some(language) => language,
            None => {
                let view_id = cx.parent_view_id();
                cx.spawn(|mut cx| async move {
                    language.await;
                    cx.update(|cx| {
                        if let Some(view_id) = view_id {
                            cx.notify(view_id);
                        } else {
                            cx.refresh();
                        }
                    })
                })
                .detach_and_log_err(cx);
                None
            }
        }
    }
}

impl Element for MarkdownElement {
    type RequestLayoutState = RenderedMarkdown;
    type PrepaintState = ();

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
                MarkdownEvent::Start(tag) => match tag {
                    MarkdownTag::Paragraph => {
                        builder.push_div(div().mb_4().line_height(rems(1.3)));
                    }
                    MarkdownTag::Heading { level, .. } => {
                        let mut heading = div().mt_2();
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
                                .my_2()
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
                        builder.push_div(
                            div()
                                .p_4()
                                .my_2()
                                .w_full()
                                .when_some(self.style.code_block.background_color, |div, color| {
                                    div.bg(color)
                                }),
                        );
                    }
                    MarkdownTag::HtmlBlock => builder.push_div(div()),
                    MarkdownTag::List(number) => {
                        builder.push_list(*number);
                        builder.push_div(div().pl_4());
                    }
                    MarkdownTag::Item => {
                        let item_index = if let Some(item_index) = builder.next_list_item_index() {
                            format!("{}.", item_index)
                        } else {
                            "•".to_string()
                        };
                        builder.push_div(
                            div()
                                .h_flex()
                                .line_height(rems(1.3))
                                .items_start()
                                .gap_1()
                                .child(item_index),
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
                    MarkdownTag::Strikethrough => builder.push_text_style(TextStyleRefinement {
                        strikethrough: Some(StrikethroughStyle {
                            thickness: px(1.),
                            color: None,
                        }),
                        ..Default::default()
                    }),
                    MarkdownTag::Link { .. } => builder.push_text_style(self.style.link.clone()),
                    MarkdownTag::Image { .. } => todo!(),
                    _ => log::info!("unsupported markdown tag {:?}", tag),
                },
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
                    MarkdownTagEnd::Link => builder.pop_text_style(),
                    MarkdownTagEnd::Image => todo!(),
                    _ => log::info!("unsupported markdown tag end: {:?}", tag),
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
                MarkdownEvent::FootnoteReference => todo!(),
                MarkdownEvent::SoftBreak => todo!(),
                MarkdownEvent::HardBreak => todo!(),
                MarkdownEvent::TaskListMarker(_) => todo!(),
            }
        }

        let mut rendered_markdown = builder.build();
        let child_layout_id = rendered_markdown.element.request_layout(cx);
        let layout_id = cx.request_layout(&Style::default(), [child_layout_id]);
        (layout_id, rendered_markdown)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        rendered_markdown.element.prepaint(cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        rendered_markdown: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.on_mouse_event({
            let rendered_text = rendered_markdown.text.clone();
            let markdown = self.markdown.downgrade();
            move |event: &MouseDownEvent, phase, cx| {
                if phase.capture() {
                    return;
                }

                markdown
                    .update(cx, |markdown, cx| {
                        if let Ok(index) = rendered_text.source_index_for_position(event.position) {
                            markdown.selection = Selection {
                                start: index,
                                end: index,
                                reversed: false,
                            };
                            markdown.is_selecting = true;
                            cx.notify();
                        } else {
                            markdown.selection = Selection::default();
                            markdown.is_selecting = false;
                            cx.notify();
                        }
                    })
                    .log_err();
            }
        });
        cx.on_mouse_event({
            let rendered_text = rendered_markdown.text.clone();
            let markdown = self.markdown.downgrade();
            move |event: &MouseMoveEvent, phase, cx| {
                if phase.capture() || event.pressed_button != Some(MouseButton::Left) {
                    return;
                }

                markdown
                    .update(cx, |markdown, cx| {
                        if markdown.is_selecting {
                            let index =
                                match rendered_text.source_index_for_position(event.position) {
                                    Ok(ix) | Err(ix) => ix,
                                };
                            markdown.selection.set_head(index);
                            cx.notify();
                        }
                    })
                    .log_err();
            }
        });
        cx.on_mouse_event({
            let markdown = self.markdown.downgrade();
            move |_event: &MouseUpEvent, phase, cx| {
                if phase.bubble() {
                    return;
                }

                markdown
                    .update(cx, |markdown, cx| {
                        if markdown.is_selecting {
                            markdown.is_selecting = false;
                            cx.notify();
                        }
                    })
                    .log_err();
            }
        });

        rendered_markdown.element.paint(cx);

        let selection = self.markdown.read(cx).selection;

        let mut selection_start: Option<(Point<Pixels>, Pixels)> = None;
        let mut selection_end: Option<(Point<Pixels>, Pixels)> = None;

        for line in rendered_markdown.text.lines.iter() {
            let line_height = line.layout.line_height();
            let line_source_start = line.source_mappings.first().unwrap().source_index;
            let line_source_range = line_source_start..=line.source_end;

            if line_source_range.contains(&selection.start) {
                let selection_start_position = line
                    .layout
                    .position_for_index(line.rendered_index_for_source_index(selection.start))
                    .unwrap();
                selection_start = Some((selection_start_position, line_height));
            }

            if line_source_range.contains(&selection.end) {
                let selection_end_position = line
                    .layout
                    .position_for_index(line.rendered_index_for_source_index(selection.end))
                    .unwrap();
                selection_end = Some((selection_end_position, line_height));
            }
        }

        if let Some(((start_position, start_line_height), (end_position, end_line_height))) =
            selection_start.zip(selection_end)
        {
            let selection_background = {
                // todo!("use the right color")
                let mut red = gpui::red();
                red.fade_out(0.9);
                red
            };

            if start_position.y == end_position.y {
                cx.paint_quad(quad(
                    Bounds::from_corners(
                        start_position,
                        point(end_position.x, end_position.y + end_line_height),
                    ),
                    Pixels::ZERO,
                    selection_background,
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
                    selection_background,
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
                        selection_background,
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
                    selection_background,
                    Edges::default(),
                    Hsla::transparent_black(),
                ));
            }
        }
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
    item_index: Option<u64>,
}

impl MarkdownElementBuilder {
    fn new(base_text_style: TextStyle, syntax_theme: Arc<SyntaxTheme>) -> Self {
        Self {
            div_stack: vec![div().debug_selector(|| "inner".into())],
            rendered_lines: Vec::new(),
            pending_line: PendingLine::default(),
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

    fn push_list(&mut self, item_index: Option<u64>) {
        self.list_stack.push(ListStackEntry { item_index });
    }

    fn next_list_item_index(&mut self) -> Option<u64> {
        self.list_stack.last_mut().and_then(|entry| {
            let item_index = entry.item_index.as_mut()?;
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

    fn source_index_for_position(&self, position: Point<Pixels>) -> Result<usize, usize> {
        let index_within_line;
        let out_of_bounds;
        match self.layout.index_for_position(position) {
            Ok(ix) => {
                index_within_line = ix;
                out_of_bounds = false;
            }
            Err(ix) => {
                index_within_line = ix;
                out_of_bounds = true;
            }
        };
        let mapping = match self
            .source_mappings
            .binary_search_by_key(&index_within_line, |probe| probe.rendered_index)
        {
            Ok(ix) => &self.source_mappings[ix],
            Err(ix) => &self.source_mappings[ix - 1],
        };
        let source_index = mapping.source_index + (index_within_line - mapping.rendered_index);
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
    lines: Arc<[RenderedLine]>,
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
}
