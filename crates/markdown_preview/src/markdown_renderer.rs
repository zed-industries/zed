use std::{ops::Range, sync::Arc};

use gpui::{
    div, rems, AbsoluteLength, AnyElement, DefiniteLength, Div, ElementId, Hsla, ParentElement,
    SharedString, Styled, StyledText, TextStyle, WindowContext,
};
use language::LanguageRegistry;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use rich_text::render_rich_text;
use theme::ActiveTheme;
use ui::{h_flex, v_flex, HeadlineSize};

enum TableState {
    Header,
    Body,
}

struct MarkdownTable {
    header: Vec<Div>,
    body: Vec<Vec<Div>>,
    current_row: Vec<Div>,
    state: TableState,
    border_color: Hsla,
}

impl MarkdownTable {
    fn new(border_color: Hsla) -> Self {
        Self {
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
        let cell = div()
            .child(contents)
            .w_full()
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
}

struct Renderer<I> {
    iter: I,

    finished: Vec<Div>,
    source_contents: String,

    language_registry: Arc<LanguageRegistry>,

    // TODO: consider removing current_block as it doesn't serve it's original purpose
    current_block: Div,

    table: Option<MarkdownTable>,
    list_depth: usize,

    ui_text_color: Hsla,
    ui_code_background: Hsla,
    ui_border_color: Hsla,
    ui_text_style: TextStyle,
}

impl<'a, I> Renderer<I>
where
    I: Iterator<Item = (Event<'a>, Range<usize>)>,
{
    fn new(
        iter: I,
        source_contents: String,
        language_registry: &Arc<LanguageRegistry>,
        ui_text_color: Hsla,
        ui_code_background: Hsla,
        ui_border_color: Hsla,
        text_style: TextStyle,
    ) -> Self {
        Self {
            iter,
            source_contents,
            current_block: div(),
            table: None,
            finished: vec![],
            language_registry: language_registry.clone(),
            list_depth: 0,
            ui_border_color,
            ui_text_color,
            ui_code_background,
            ui_text_style: text_style,
        }
    }

    fn run(mut self) -> Self {
        while let Some((event, source_range)) = self.iter.next() {
            match event {
                Event::Start(tag) => {
                    self.start_tag(tag);
                }
                Event::End(tag) => {
                    self.end_tag(tag, source_range);
                }
                Event::Rule => {
                    self.finished
                        .push(div().w_full().h(rems(1. / 4.)).bg(self.ui_border_color));
                }
                _ => {
                    // TODO: SoftBreak, HardBreak, FootnoteReference
                }
            }
        }
        self
    }

    fn render_md_from_range(&self, source_range: Range<usize>) -> gpui::AnyElement {
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
        rich_text.element_no_cx(id, self.ui_text_style.clone(), self.ui_code_background)
    }

    fn start_tag(&mut self, tag: Tag<'a>) {
        match tag {
            Tag::List(_) => {
                self.list_depth += 1;
            }
            Tag::Paragraph => {
                self.current_block = h_flex();
            }
            Tag::Heading(level, _, _) => {
                let size: HeadlineSize = match level {
                    HeadingLevel::H1 => HeadlineSize::XLarge,
                    HeadingLevel::H2 => HeadlineSize::Large,
                    HeadingLevel::H3 => HeadlineSize::Medium,
                    HeadingLevel::H4 => HeadlineSize::Small,
                    HeadingLevel::H5 => HeadlineSize::XSmall,
                    HeadingLevel::H6 => HeadlineSize::XSmall,
                };

                let line_height =
                    DefiniteLength::Absolute(AbsoluteLength::Rems(size.line_height()));

                self.current_block = h_flex()
                    .line_height(size.line_height())
                    .text_size(size.size())
                    .text_color(self.ui_text_color)
                    .mb(line_height);
            }
            Tag::Table(_text_alignments) => {
                self.table = Some(MarkdownTable::new(self.ui_border_color));
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: Tag, source_range: Range<usize>) {
        match tag {
            Tag::Paragraph => {
                let element = self.render_md_from_range(source_range.clone());
                let current_block = std::mem::replace(&mut self.current_block, div());
                let paragraph = current_block.child(element);

                self.finished.push(paragraph);
            }
            Tag::Heading(_level, _, _) => {
                let element = self.render_md_from_range(source_range.clone());
                let current_block = std::mem::replace(&mut self.current_block, div());
                let headline = current_block.child(element);

                self.finished.push(headline);
            }
            Tag::List(_) => {
                if self.list_depth == 1 {
                    let element = self.render_md_from_range(source_range.clone());
                    let current_block = std::mem::replace(&mut self.current_block, div());
                    let list = current_block.child(element);

                    self.finished.push(list);
                }

                self.list_depth -= 1;
            }
            Tag::BlockQuote => {
                // TODO: will render twice because there's <paragraph> in a block quote
                let element = self.render_md_from_range(source_range.clone());
                let current_block = std::mem::replace(&mut self.current_block, div());

                let block_quote = current_block
                    .pl_2()
                    .bg(self.ui_code_background)
                    .child(element);

                self.finished.push(block_quote);
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
                    .px_4()
                    .bg(self.ui_code_background)
                    .child(StyledText::new(contents));

                self.finished.push(code_block);
            }
            Tag::Table(_alignment) => {
                if self.table.is_none() {
                    log::error!("Table end without table ({:?})", source_range);
                    return;
                }

                let table = self.table.take().unwrap();
                let table = table.finish();
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

                let contents = self.render_md_from_range(source_range.clone());
                self.table.as_mut().unwrap().add_cell(contents);
            }
            _ => {}
        }
    }
}

pub fn render_markdown(
    markdown_input: &str,
    cx: &WindowContext,
    language_registry: &Arc<LanguageRegistry>,
) -> Vec<Div> {
    let theme = cx.theme();
    let ui_code_background = theme.colors().surface_background;
    let ui_text_color = theme.colors().text;
    let ui_border_color = theme.colors().border;
    let text_style = cx.text_style();

    let options = Options::all();
    let parser = Parser::new_ext(markdown_input, options);
    let renderer = Renderer::new(
        parser.into_offset_iter(),
        markdown_input.to_owned(),
        language_registry,
        ui_text_color,
        ui_code_background,
        ui_border_color,
        text_style,
    );
    let renderer = renderer.run();
    return renderer.finished;
}
