mod events;

use gpui::{Empty, HighlightStyle, Render, Task};
use pulldown_cmark::{
    Alignment, HeadingLevel, LinkType, MetadataBlockKind, Options, Parser, TagEnd,
};
use std::ops::Range;
use ui::prelude::*;
use util::TryFutureExt;

pub struct Markdown {
    text: String,
    parsed_markdown: Option<ParsedMarkdown>,
    pending_parse: Option<Task<Option<()>>>,
}

impl Markdown {
    pub fn new(text: String, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            text: text.clone(),
            parsed_markdown: None,
            pending_parse: None,
        };

        if !text.is_empty() {
            this.parse(cx)
        }
        this
    }

    pub fn append(&mut self, text: &str, cx: &mut ViewContext<Self>) {
        self.text.push_str(text);
        self.parse(cx);
    }

    fn parse(&mut self, cx: &mut ViewContext<Self>) {
        let text = SharedString::from(self.text.clone());
        let parsed = cx.background_executor().spawn(async move {
            let parser = Parser::new_ext(text.as_ref(), Options::all());

            for (event, range) in parser.into_offset_iter() {
                match event {
                    pulldown_cmark::Event::Start(_) => todo!(),
                    pulldown_cmark::Event::End(_) => todo!(),
                    pulldown_cmark::Event::Text(_) => todo!(),
                    pulldown_cmark::Event::Code(_) => todo!(),
                    pulldown_cmark::Event::Html(_) => todo!(),
                    pulldown_cmark::Event::InlineHtml(_) => todo!(),
                    pulldown_cmark::Event::FootnoteReference(_) => todo!(),
                    pulldown_cmark::Event::SoftBreak => todo!(),
                    pulldown_cmark::Event::HardBreak => todo!(),
                    pulldown_cmark::Event::Rule => todo!(),
                    pulldown_cmark::Event::TaskListMarker(_) => todo!(),
                }
            }

            anyhow::Ok(ParsedMarkdown {
                text,
                blocks: Vec::new(),
                highlights: Vec::new(),
            })
        });

        self.pending_parse = Some(cx.spawn(|this, mut cx| {
            async move {
                let parsed = parsed.await?;
                this.update(&mut cx, |this, cx| {
                    this.parsed_markdown = Some(parsed);
                    this.pending_parse.take();
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
        self.parsed_markdown
            .clone()
            .map(|parsed| parsed.into_any())
            .unwrap_or_else(|| Empty.into_any())
    }
}

#[derive(Clone)]
struct ParsedMarkdown {
    text: SharedString,
    blocks: Vec<MarkdownBlock>,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
}

#[derive(Clone)]
enum MarkdownBlock {
    Heading { level: u8, range: Range<usize> },
    Text(Range<usize>),
    Code(Range<usize>),
}

impl Element for ParsedMarkdown {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ui::prelude::ElementId> {
        todo!()
    }

    fn request_layout(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        cx: &mut ui::prelude::WindowContext,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        todo!()
    }

    fn prepaint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<ui::prelude::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut ui::prelude::WindowContext,
    ) -> Self::PrepaintState {
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&gpui::GlobalElementId>,
        bounds: gpui::Bounds<ui::prelude::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut ui::prelude::WindowContext,
    ) {
        todo!()
    }
}

impl IntoElement for ParsedMarkdown {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
