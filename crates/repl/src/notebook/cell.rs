#![allow(unused, dead_code)]
use core::fmt;
use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use editor::{Editor, EditorMode, MultiBuffer};
use futures::future::Shared;
use gpui::{prelude::*, Hsla, Task, TextStyleRefinement, View, WeakView};
use language::{Buffer, Language, LanguageRegistry};
use markdown_preview::{markdown_parser::parse_markdown, markdown_renderer::render_markdown_block};
use nbformat::v4::{CellId, CellMetadata, CellType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::Settings as _;
use theme::ThemeSettings;
use ui::prelude::*;
use uuid::Uuid;

use crate::{
    notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH},
    outputs::{plain::TerminalOutput, user_error::ErrorView, Output},
};

#[derive(IntoElement)]
pub enum CellControl {
    RunCell,
}

impl CellControl {
    fn icon_name(&self) -> IconName {
        match self {
            CellControl::RunCell => IconName::Play,
        }
    }
}

impl RenderOnce for CellControl {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
    }
}

/// A notebook cell
#[derive(Clone)]
pub enum Cell {
    Code(View<CodeCell>),
    Markdown(View<MarkdownCell>),
    Raw(View<RawCell>),
}

fn convert_outputs(outputs: &Vec<nbformat::v4::Output>, cx: &mut WindowContext) -> Vec<Output> {
    outputs
        .into_iter()
        .map(|output| match output {
            nbformat::v4::Output::Stream { name, text } => Output::Stream {
                content: cx.new_view(|cx| TerminalOutput::from(&text.0, cx)),
            },
            nbformat::v4::Output::DisplayData(display_data) => {
                Output::new(&display_data.data, None, cx)
            }
            nbformat::v4::Output::ExecuteResult(execute_result) => {
                Output::new(&execute_result.data, None, cx)
            }
            nbformat::v4::Output::Error(error) => Output::ErrorOutput(ErrorView {
                ename: error.ename.clone(),
                evalue: error.evalue.clone(),
                traceback: cx.new_view(|cx| TerminalOutput::from(&error.traceback.join("\n"), cx)),
            }),
        })
        .collect()
}

impl Cell {
    pub fn load(
        cell: &nbformat::v4::Cell,
        languages: &Arc<LanguageRegistry>,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        cx: &mut WindowContext,
    ) -> Self {
        match cell {
            nbformat::v4::Cell::Markdown {
                id,
                metadata,
                source,
                attachments,
            } => {
                let source = source.join("");

                let view = cx.new_view(|cx| {
                    let markdown_parsing_task = {
                        let languages = languages.clone();
                        let source = source.clone();

                        cx.spawn(|this, mut cx| async move {
                            let parsed_markdown = cx
                                .background_executor()
                                .spawn(async move {
                                    parse_markdown(&source, None, Some(languages)).await
                                })
                                .await;

                            this.update(&mut cx, |cell: &mut MarkdownCell, _| {
                                cell.parsed_markdown = Some(parsed_markdown);
                            });
                        })
                    };

                    MarkdownCell {
                        markdown_parsing_task,
                        languages: languages.clone(),
                        id: id.clone(),
                        metadata: metadata.clone(),
                        source: source.clone(),
                        parsed_markdown: None,
                        selected: false,
                    }
                });

                Cell::Markdown(view)
            }
            nbformat::v4::Cell::Code {
                id,
                metadata,
                execution_count,
                source,
                outputs,
            } => Cell::Code(cx.new_view(|cx| {
                let text = source.join("");

                let buffer = cx.new_model(|cx| Buffer::local(text.clone(), cx));
                let multi_buffer = cx.new_model(|cx| MultiBuffer::singleton(buffer.clone(), cx));

                let editor_view = cx.new_view(|cx| {
                    let mut editor = Editor::new(
                        EditorMode::AutoHeight { max_lines: 1024 },
                        multi_buffer,
                        None,
                        false,
                        cx,
                    );

                    let theme = ThemeSettings::get_global(cx);

                    let refinement = TextStyleRefinement {
                        font_family: Some(theme.buffer_font.family.clone()),
                        font_size: Some(theme.buffer_font_size.into()),
                        color: Some(cx.theme().colors().editor_foreground),
                        background_color: Some(gpui::transparent_black()),
                        ..Default::default()
                    };

                    editor.set_text(text, cx);
                    editor.set_show_gutter(false, cx);
                    editor.set_text_style_refinement(refinement);

                    // editor.set_read_only(true);
                    editor
                });

                let buffer = buffer.clone();
                let language_task = cx.spawn(|this, mut cx| async move {
                    let language = notebook_language.await;

                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_language(language.clone(), cx);
                    });
                });

                CodeCell {
                    id: id.clone(),
                    metadata: metadata.clone(),
                    execution_count: *execution_count,
                    source: source.join(""),
                    editor: editor_view,
                    outputs: convert_outputs(outputs, cx),
                    selected: false,
                    language_task,
                }
            })),
            nbformat::v4::Cell::Raw {
                id,
                metadata,
                source,
            } => Cell::Raw(cx.new_view(|_| RawCell {
                id: id.clone(),
                metadata: metadata.clone(),
                source: source.join(""),
                selected: false,
            })),
        }
    }
}

pub trait RenderableCell: Render {
    const CELL_TYPE: CellType;

    // fn new(cx: &mut WindowContext) -> View<Self>;
    fn id(&self) -> &CellId;
    fn cell_type(&self) -> CellType;
    fn metadata(&self) -> &CellMetadata;
    fn source(&self) -> &String;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool) -> &mut Self;
    fn selected_bg_color(&self, cx: &ViewContext<Self>) -> Hsla {
        if self.selected() {
            let mut color = cx.theme().colors().icon_accent;
            color.fade_out(0.9);
            color
        } else {
            // TODO: this is wrong
            cx.theme().colors().tab_bar_background
        }
    }
    fn control(&self) -> Option<CellControl> {
        None
    }
    // fn language_registry(&self, language_registry: &Arc<LanguageRegistry>) -> &LanguageRegistry;
    fn gutter(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let is_selected = self.selected();

        div()
            .relative()
            .h_full()
            .w(px(GUTTER_WIDTH))
            .child(
                div()
                    .w(px(GUTTER_WIDTH))
                    .flex()
                    .flex_none()
                    .justify_center()
                    .h_full()
                    .child(
                        div()
                            .flex_none()
                            .w(px(1.))
                            .h_full()
                            .when(is_selected, |this| this.bg(cx.theme().colors().icon_accent))
                            .when(!is_selected, |this| this.bg(cx.theme().colors().border)),
                    ),
            )
            .when_some(self.control(), |this, control| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_none()
                        .w(px(GUTTER_WIDTH))
                        .h(px(GUTTER_WIDTH + 12.0))
                        .items_center()
                        .justify_center()
                        .bg(cx.theme().colors().tab_bar_background)
                        .child(IconButton::new("control", control.icon_name())),
                )
            })
    }

    // fn cell_placeholder(&self, cx: &ViewContext<Self>) -> impl IntoElement {
    //     // TODO: render placeholder
    //     div().into_element()
    // }
}

pub struct MarkdownCell {
    id: CellId,
    metadata: CellMetadata,
    source: String,
    parsed_markdown: Option<markdown_preview::markdown_elements::ParsedMarkdown>,
    markdown_parsing_task: Task<()>,
    selected: bool,
    languages: Arc<LanguageRegistry>,
}

impl RenderableCell for MarkdownCell {
    const CELL_TYPE: CellType = CellType::Markdown;

    // fn new(cx: &mut WindowContext) -> View<Self> {
    //     cx.new_view(|cx| MarkdownCell::default())
    // }

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> CellType {
        CellType::Markdown
    }

    fn metadata(&self) -> &CellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    fn control(&self) -> Option<CellControl> {
        None
    }
}

impl Render for MarkdownCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(parsed) = self.parsed_markdown.as_ref() else {
            return div();
        };

        let mut markdown_render_context =
            markdown_preview::markdown_renderer::RenderContext::new(None, cx);

        h_flex()
            .w_full()
            .pr_8()
            .rounded_sm()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .bg(self.selected_bg_color(cx))
            .child(self.gutter(cx))
            .child(
                v_flex()
                    .size_full()
                    .flex_1()
                    .p_3()
                    .font_ui(cx)
                    .text_size(TextSize::Default.rems(cx))
                    //
                    .children(parsed.children.iter().map(|child| {
                        div().relative().child(
                            div()
                                .relative()
                                .child(render_markdown_block(child, &mut markdown_render_context)),
                        )
                    })),
            )
    }
}

pub struct CodeCell {
    id: CellId,
    metadata: CellMetadata,
    execution_count: Option<i32>,
    source: String,
    editor: View<editor::Editor>,
    outputs: Vec<Output>,
    selected: bool,
    language_task: Task<()>,
}

impl CodeCell {
    pub fn has_outputs(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    pub fn gutter_output(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let is_selected = self.selected();

        div()
            .relative()
            .h_full()
            .w(px(GUTTER_WIDTH))
            .child(
                div()
                    .w(px(GUTTER_WIDTH))
                    .flex()
                    .flex_none()
                    .justify_center()
                    .h_full()
                    .child(
                        div()
                            .flex_none()
                            .w(px(1.))
                            .h_full()
                            .when(is_selected, |this| this.bg(cx.theme().colors().icon_accent))
                            .when(!is_selected, |this| this.bg(cx.theme().colors().border)),
                    ),
            )
            .when(self.has_outputs(), |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_none()
                        .w(px(GUTTER_WIDTH))
                        .h(px(GUTTER_WIDTH + 12.0))
                        .items_center()
                        .justify_center()
                        .bg(cx.theme().colors().tab_bar_background)
                        .child(IconButton::new("control", IconName::MailOpen)),
                )
            })
    }
}

impl RenderableCell for CodeCell {
    const CELL_TYPE: CellType = CellType::Code;

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> CellType {
        CellType::Code
    }

    fn metadata(&self) -> &CellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn control(&self) -> Option<CellControl> {
        Some(CellControl::RunCell)
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

impl Render for CodeCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let lines = self.source.lines().count();
        let height = lines as f32 * cx.line_height();

        v_flex()
            // Editor portion
            .child(
                h_flex()
                    .w_full()
                    .pr_2()
                    .rounded_sm()
                    .items_start()
                    .gap(Spacing::Large.rems(cx))
                    .bg(self.selected_bg_color(cx))
                    .child(self.gutter(cx))
                    .child(
                        div().py_1p5().w_full().child(
                            div()
                                .flex()
                                .size_full()
                                .flex_1()
                                .py_3()
                                .px_5()
                                .rounded_lg()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .bg(cx.theme().colors().editor_background)
                                .child(div().h(height).w_full().child(self.editor.clone())),
                        ),
                    ),
            )
            // Output portion
            .child(
                h_flex()
                    .w_full()
                    .pr_2()
                    .rounded_sm()
                    .items_start()
                    .gap(Spacing::Large.rems(cx))
                    .bg(self.selected_bg_color(cx))
                    .child(self.gutter_output(cx))
                    .child(
                        div().py_1p5().w_full().child(
                            div()
                                .flex()
                                .size_full()
                                .flex_1()
                                .py_3()
                                .px_5()
                                .rounded_lg()
                                .border_1()
                                // .border_color(cx.theme().colors().border)
                                // .bg(cx.theme().colors().editor_background)
                                .child(div().w_full().children(self.outputs.iter().map(
                                    |output| {
                                        let content = match output {
                                            Output::Plain { content, .. } => {
                                                Some(content.clone().into_any_element())
                                            }
                                            Output::Markdown { content, .. } => {
                                                Some(content.clone().into_any_element())
                                            }
                                            Output::Stream { content, .. } => {
                                                Some(content.clone().into_any_element())
                                            }
                                            Output::Image { content, .. } => {
                                                Some(content.clone().into_any_element())
                                            }
                                            Output::Message(message) => Some(
                                                div().child(message.clone()).into_any_element(),
                                            ),
                                            Output::Table { content, .. } => {
                                                Some(content.clone().into_any_element())
                                            }
                                            Output::ErrorOutput(error_view) => {
                                                error_view.render(cx)
                                            }
                                            Output::ClearOutputWaitMarker => None,
                                        };

                                        div()
                                            // .w_full()
                                            // .mt_3()
                                            // .p_3()
                                            // .rounded_md()
                                            // .bg(cx.theme().colors().editor_background)
                                            // .border(px(1.))
                                            // .border_color(cx.theme().colors().border)
                                            // .shadow_sm()
                                            .children(content)
                                    },
                                ))),
                        ),
                    ),
            )
    }
}

pub struct RawCell {
    id: CellId,
    metadata: CellMetadata,
    source: String,
    selected: bool,
}

impl RenderableCell for RawCell {
    const CELL_TYPE: CellType = CellType::Raw;

    fn id(&self) -> &CellId {
        &self.id
    }

    fn cell_type(&self) -> CellType {
        CellType::Raw
    }

    fn metadata(&self) -> &CellMetadata {
        &self.metadata
    }

    fn source(&self) -> &String {
        &self.source
    }

    fn control(&self) -> Option<CellControl> {
        None
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }
}

impl Render for RawCell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .w_full()
            .pr_8()
            .rounded_sm()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .bg(self.selected_bg_color(cx))
            .child(self.gutter(cx))
            .child(
                div()
                    .flex()
                    .size_full()
                    .flex_1()
                    .p_3()
                    .font_ui(cx)
                    .text_size(TextSize::Default.rems(cx))
                    .child(self.source.clone()),
            )
    }
}
