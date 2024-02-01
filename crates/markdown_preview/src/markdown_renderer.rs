use std::{ops::Range, sync::Arc};

use gpui::{
    div, px, rems, AnyElement, DefiniteLength, Div, ElementId, Hsla, ParentElement, SharedString,
    Styled, StyledText, WindowContext,
};
use language::LanguageRegistry;
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use rich_text::render_rich_text;
use theme::{ActiveTheme, Theme};
use ui::{h_flex, v_flex};

enum TableState {
    Header,
    Body,
}

struct MarkdownTable {
    column_alignments: Vec<Alignment>,
    header: Vec<Div>,
    body: Vec<Vec<Div>>,
    current_row: Vec<Div>,
    state: TableState,
    border_color: Hsla,
}

impl MarkdownTable {
    fn new(border_color: Hsla, column_alignments: Vec<Alignment>) -> Self {
        Self {
            column_alignments,
            header: Vec::new(),
            body: Vec::new(),
            current_row: Vec::new(),
            state: TableState::Header,
            border_color,
        }
    }

    fn finish_row(&mut self) {
        match self.state {
            TableState::Header => {
                self.header.extend(self.current_row.drain(..));
                self.state = TableState::Body;
            }
            TableState::Body => {
                self.body.push(self.current_row.drain(..).collect());
            }
        }
    }

    fn add_cell(&mut self, contents: AnyElement) {
        let container = match self.alignment_for_next_cell() {
            Alignment::Left | Alignment::None => div(),
            Alignment::Center => v_flex().items_center(),
            Alignment::Right => v_flex().items_end(),
        };

        let cell = container
            .w_full()
            .child(contents)
            .px_2()
            .py_1()
            .border_color(self.border_color);

        let cell = match self.state {
            TableState::Header => cell.border_2(),
            TableState::Body => cell.border_1(),
        };

        self.current_row.push(cell);
    }

    fn finish(self) -> Div {
        let mut table = v_flex().w_full();
        let mut header = h_flex();

        for cell in self.header {
            header = header.child(cell);
        }
        table = table.child(header);
        for row in self.body {
            let mut row_div = h_flex();
            for cell in row {
                row_div = row_div.child(cell);
            }
            table = table.child(row_div);
        }
        table
    }

    fn alignment_for_next_cell(&self) -> Alignment {
        self.column_alignments
            .get(self.current_row.len())
            .copied()
            .unwrap_or(Alignment::None)
    }
}

struct Renderer<I> {
    source_contents: String,
    iter: I,
    theme: Arc<Theme>,
    finished: Vec<Div>,
    language_registry: Arc<LanguageRegistry>,
    table: Option<MarkdownTable>,
    list_depth: usize,
    block_quote_depth: usize,
}

impl<'a, I> Renderer<I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    fn new(
        iter: I,
        source_contents: String,
        language_registry: &Arc<LanguageRegistry>,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            iter,
            source_contents,
            theme,
            table: None,
            finished: vec![],
            language_registry: language_registry.clone(),
            list_depth: 0,
            block_quote_depth: 0,
        }
    }

    fn run(mut self, cx: &WindowContext) -> Self {
        while let Some((event, source_range)) = self.iter.next() {
            match event {
                Event::Start(tag) => {
                    self.start_tag(tag);
                }
                Event::End(tag) => {
                    self.end_tag(tag, source_range, cx);
                }
                Event::Rule => {
                    let rule = div().w_full().h(px(2.)).bg(self.theme.colors().border);
                    self.finished.push(div().mb_4().child(rule));
                }
                _ => {}
            }
        }
        self
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::List(_) => {
                self.list_depth += 1;
            }
            Tag::BlockQuote => {
                self.block_quote_depth += 1;
            }
            Tag::Table(column_alignments) => {
                self.table = Some(MarkdownTable::new(
                    self.theme.colors().border,
                    column_alignments,
                ));
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: Tag, source_range: Range<usize>, cx: &WindowContext) {
        match tag {
            Tag::Paragraph => {
                if self.list_depth > 0 || self.block_quote_depth > 0 {
                    return;
                }

                let element = self.render_md_from_range(source_range.clone(), cx);
                let paragraph = div().mb_3().child(element);

                self.finished.push(paragraph);
            }
            Tag::Heading(level, _, _) => {
                let mut headline = self.headline(level);
                if source_range.start > 0 {
                    headline = headline.mt_4();
                }

                let element = self.render_md_from_range(source_range.clone(), cx);
                let headline = headline.child(element);

                self.finished.push(headline);
            }
            Tag::List(_) => {
                if self.list_depth == 1 {
                    let element = self.render_md_from_range(source_range.clone(), cx);
                    let list = div().mb_3().child(element);

                    self.finished.push(list);
                }

                self.list_depth -= 1;
            }
            Tag::BlockQuote => {
                let element = self.render_md_from_range(source_range.clone(), cx);

                let block_quote = h_flex()
                    .mb_3()
                    .child(
                        div()
                            .w(px(4.))
                            .bg(self.theme.colors().border)
                            .h_full()
                            .mr_2()
                            .mt_1(),
                    )
                    .text_color(self.theme.colors().text_muted)
                    .child(element);

                self.finished.push(block_quote);

                self.block_quote_depth -= 1;
            }
            Tag::CodeBlock(kind) => {
                let contents = self.source_contents[source_range.clone()].trim();
                let contents = contents.trim_start_matches("```");
                let contents = contents.trim_end_matches("```");
                let contents = match kind {
                    CodeBlockKind::Fenced(language) => {
                        contents.trim_start_matches(&language.to_string())
                    }
                    CodeBlockKind::Indented => contents,
                };
                let contents: String = contents.into();
                let contents = SharedString::from(contents);

                let code_block = div()
                    .mb_3()
                    .px_4()
                    .py_0()
                    .bg(self.theme.colors().surface_background)
                    .child(StyledText::new(contents));

                self.finished.push(code_block);
            }
            Tag::Table(_alignment) => {
                if self.table.is_none() {
                    log::error!("Table end without table ({:?})", source_range);
                    return;
                }

                let table = self.table.take().unwrap();
                let table = table.finish().mb_4();
                self.finished.push(table);
            }
            Tag::TableHead => {
                if self.table.is_none() {
                    log::error!("Table head without table ({:?})", source_range);
                    return;
                }

                self.table.as_mut().unwrap().finish_row();
            }
            Tag::TableRow => {
                if self.table.is_none() {
                    log::error!("Table row without table ({:?})", source_range);
                    return;
                }

                self.table.as_mut().unwrap().finish_row();
            }
            Tag::TableCell => {
                if self.table.is_none() {
                    log::error!("Table cell without table ({:?})", source_range);
                    return;
                }

                let contents = self.render_md_from_range(source_range.clone(), cx);
                self.table.as_mut().unwrap().add_cell(contents);
            }
            _ => {}
        }
    }

    fn render_md_from_range(
        &self,
        source_range: Range<usize>,
        cx: &WindowContext,
    ) -> gpui::AnyElement {
        let mentions = &[];
        let language = None;
        let paragraph = &self.source_contents[source_range.clone()];
        let rich_text = render_rich_text(
            paragraph.into(),
            mentions,
            &self.language_registry,
            language,
        );
        let id: ElementId = source_range.start.into();
        rich_text.element(id, cx)
    }

    fn headline(&self, level: HeadingLevel) -> Div {
        let size = match level {
            HeadingLevel::H1 => rems(2.),
            HeadingLevel::H2 => rems(1.5),
            HeadingLevel::H3 => rems(1.25),
            HeadingLevel::H4 => rems(1.),
            HeadingLevel::H5 => rems(0.875),
            HeadingLevel::H6 => rems(0.85),
        };

        let color = match level {
            HeadingLevel::H6 => self.theme.colors().text_muted,
            _ => self.theme.colors().text,
        };

        let line_height = DefiniteLength::from(rems(1.25));

        let headline = h_flex()
            .w_full()
            .line_height(line_height)
            .text_size(size)
            .text_color(color)
            .mb_4()
            .pb(rems(0.15));

        headline
    }
}

pub fn render_markdown(
    markdown_input: &str,
    language_registry: &Arc<LanguageRegistry>,
    cx: &WindowContext,
) -> Vec<Div> {
    let theme = cx.theme().clone();
    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);
    let renderer = Renderer::new(
        parser.into_offset_iter(),
        markdown_input.to_owned(),
        language_registry,
        theme,
    );
    let renderer = renderer.run(cx);
    return renderer.finished;
}
