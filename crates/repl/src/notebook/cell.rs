#![allow(unused, dead_code)]
use std::sync::Arc;
use std::time::{Duration, Instant};

use editor::{Editor, EditorMode, MultiBuffer};
use futures::future::Shared;
use gpui::{
    App, Entity, EventEmitter, Focusable, Hsla, InteractiveElement, RetainAllImageCache,
    StatefulInteractiveElement, Task, TextStyleRefinement, image_cache, prelude::*,
};
use language::{Buffer, Language, LanguageRegistry};
use markdown::{Markdown, MarkdownElement, MarkdownStyle};
use nbformat::v4::{CellId, CellMetadata, CellType};
use runtimelib::{JupyterMessage, JupyterMessageContent};
use settings::Settings as _;
use theme::ThemeSettings;
use ui::{CommonAnimationExt, IconButtonShape, prelude::*};
use util::ResultExt;

use crate::{
    notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH},
    outputs::{Output, plain::TerminalOutput, user_error::ErrorView},
};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum CellPosition {
    First,
    Middle,
    Last,
}

pub enum CellControlType {
    RunCell,
    RerunCell,
    ClearCell,
    CellOptions,
    CollapseCell,
    ExpandCell,
}

pub enum CellEvent {
    Run(CellId),
    FocusedIn(CellId),
}

pub enum MarkdownCellEvent {
    FinishedEditing,
    Run(CellId),
}

impl CellControlType {
    fn icon_name(&self) -> IconName {
        match self {
            CellControlType::RunCell => IconName::PlayFilled,
            CellControlType::RerunCell => IconName::ArrowCircle,
            CellControlType::ClearCell => IconName::ListX,
            CellControlType::CellOptions => IconName::Ellipsis,
            CellControlType::CollapseCell => IconName::ChevronDown,
            CellControlType::ExpandCell => IconName::ChevronRight,
        }
    }
}

pub struct CellControl {
    button: IconButton,
}

impl CellControl {
    fn new(id: impl Into<SharedString>, control_type: CellControlType) -> Self {
        let icon_name = control_type.icon_name();
        let id = id.into();
        let button = IconButton::new(id, icon_name)
            .icon_size(IconSize::Small)
            .shape(IconButtonShape::Square);
        Self { button }
    }
}

impl Clickable for CellControl {
    fn on_click(
        self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        let button = self.button.on_click(handler);
        Self { button }
    }

    fn cursor_style(self, _cursor_style: gpui::CursorStyle) -> Self {
        self
    }
}

/// A notebook cell
#[derive(Clone)]
pub enum Cell {
    Code(Entity<CodeCell>),
    Markdown(Entity<MarkdownCell>),
    Raw(Entity<RawCell>),
}

fn convert_outputs(
    outputs: &Vec<nbformat::v4::Output>,
    window: &mut Window,
    cx: &mut App,
) -> Vec<Output> {
    outputs
        .iter()
        .map(|output| match output {
            nbformat::v4::Output::Stream { text, .. } => Output::Stream {
                content: cx.new(|cx| TerminalOutput::from(&text.0, window, cx)),
            },
            nbformat::v4::Output::DisplayData(display_data) => {
                Output::new(&display_data.data, None, window, cx)
            }
            nbformat::v4::Output::ExecuteResult(execute_result) => {
                Output::new(&execute_result.data, None, window, cx)
            }
            nbformat::v4::Output::Error(error) => Output::ErrorOutput(ErrorView {
                ename: error.ename.clone(),
                evalue: error.evalue.clone(),
                traceback: cx
                    .new(|cx| TerminalOutput::from(&error.traceback.join("\n"), window, cx)),
            }),
        })
        .collect()
}

impl Cell {
    pub fn id(&self, cx: &App) -> CellId {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).id().clone(),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).id().clone(),
            Cell::Raw(raw_cell) => raw_cell.read(cx).id().clone(),
        }
    }

    pub fn current_source(&self, cx: &App) -> String {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).current_source(cx),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).current_source(cx),
            Cell::Raw(raw_cell) => raw_cell.read(cx).source.clone(),
        }
    }

    pub fn to_nbformat_cell(&self, cx: &App) -> nbformat::v4::Cell {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).to_nbformat_cell(cx),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).to_nbformat_cell(cx),
            Cell::Raw(raw_cell) => raw_cell.read(cx).to_nbformat_cell(),
        }
    }

    pub fn is_dirty(&self, cx: &App) -> bool {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).is_dirty(cx),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).is_dirty(cx),
            Cell::Raw(_) => false,
        }
    }

    pub fn load(
        cell: &nbformat::v4::Cell,
        languages: &Arc<LanguageRegistry>,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match cell {
            nbformat::v4::Cell::Markdown {
                id,
                metadata,
                source,
                ..
            } => {
                let source = source.join("");

                let entity = cx.new(|cx| {
                    MarkdownCell::new(
                        id.clone(),
                        metadata.clone(),
                        source,
                        languages.clone(),
                        window,
                        cx,
                    )
                });

                Cell::Markdown(entity)
            }
            nbformat::v4::Cell::Code {
                id,
                metadata,
                execution_count,
                source,
                outputs,
            } => {
                let text = source.join("");
                let outputs = convert_outputs(outputs, window, cx);

                Cell::Code(cx.new(|cx| {
                    CodeCell::load(
                        id.clone(),
                        metadata.clone(),
                        *execution_count,
                        text,
                        outputs,
                        notebook_language,
                        window,
                        cx,
                    )
                }))
            }
            nbformat::v4::Cell::Raw {
                id,
                metadata,
                source,
            } => Cell::Raw(cx.new(|_| RawCell {
                id: id.clone(),
                metadata: metadata.clone(),
                source: source.join(""),
                selected: false,
                cell_position: None,
            })),
        }
    }
}

pub trait RenderableCell: Render {
    const CELL_TYPE: CellType;

    fn id(&self) -> &CellId;
    fn cell_type(&self) -> CellType;
    fn metadata(&self) -> &CellMetadata;
    fn source(&self) -> &String;
    fn selected(&self) -> bool;
    fn set_selected(&mut self, selected: bool) -> &mut Self;
    fn selected_bg_color(&self, window: &mut Window, cx: &mut Context<Self>) -> Hsla {
        if self.selected() {
            let mut color = cx.theme().colors().element_hover;
            color.fade_out(0.5);
            color
        } else {
            // Not sure if this is correct, previous was TODO: this is wrong
            gpui::transparent_black()
        }
    }
    fn control(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Option<CellControl> {
        None
    }

    fn cell_position_spacer(
        &self,
        is_first: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        let cell_position = self.cell_position();

        if (cell_position == Some(&CellPosition::First) && is_first)
            || (cell_position == Some(&CellPosition::Last) && !is_first)
        {
            Some(div().flex().w_full().h(DynamicSpacing::Base12.px(cx)))
        } else {
            None
        }
    }

    fn gutter(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .when_some(self.control(window, cx), |this, control| {
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
                        .child(control.button),
                )
            })
    }

    fn cell_position(&self) -> Option<&CellPosition>;
    fn set_cell_position(&mut self, position: CellPosition) -> &mut Self;
}

pub trait RunnableCell: RenderableCell {
    fn execution_count(&self) -> Option<i32>;
    fn set_execution_count(&mut self, count: i32) -> &mut Self;
    fn run(&mut self, window: &mut Window, cx: &mut Context<Self>) -> ();
}

pub struct MarkdownCell {
    id: CellId,
    metadata: CellMetadata,
    image_cache: Entity<RetainAllImageCache>,
    source: String,
    editor: Entity<Editor>,
    markdown: Entity<Markdown>,
    editing: bool,
    selected: bool,
    cell_position: Option<CellPosition>,
    languages: Arc<LanguageRegistry>,
    _editor_subscription: gpui::Subscription,
}

impl EventEmitter<MarkdownCellEvent> for MarkdownCell {}

impl MarkdownCell {
    pub fn new(
        id: CellId,
        metadata: CellMetadata,
        source: String,
        languages: Arc<LanguageRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = cx.new(|cx| Buffer::local(source.clone(), cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let markdown_language = languages.language_for_name("Markdown");
        cx.spawn_in(window, async move |_this, cx| {
            if let Some(markdown) = markdown_language.await.log_err() {
                buffer.update(cx, |buffer, cx| {
                    buffer.set_language(Some(markdown), cx);
                });
            }
        })
        .detach();

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(1024),
                },
                multi_buffer,
                None,
                window,
                cx,
            );

            let theme = ThemeSettings::get_global(cx);
            let refinement = TextStyleRefinement {
                font_family: Some(theme.buffer_font.family.clone()),
                font_size: Some(theme.buffer_font_size(cx).into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(gpui::transparent_black()),
                ..Default::default()
            };

            editor.set_show_gutter(false, cx);
            editor.set_text_style_refinement(refinement);
            editor
        });

        let markdown = cx.new(|cx| Markdown::new(source.clone().into(), None, None, cx));

        let cell_id = id.clone();
        let editor_subscription =
            cx.subscribe(&editor, move |this, _editor, event, cx| match event {
                editor::EditorEvent::Blurred => {
                    if this.editing {
                        this.editing = false;
                        cx.emit(MarkdownCellEvent::FinishedEditing);
                        cx.notify();
                    }
                }
                _ => {}
            });

        let start_editing = source.is_empty();
        Self {
            id,
            metadata,
            image_cache: RetainAllImageCache::new(cx),
            source,
            editor,
            markdown,
            editing: start_editing,
            selected: false,
            cell_position: None,
            languages,
            _editor_subscription: editor_subscription,
        }
    }

    pub fn editor(&self) -> &Entity<Editor> {
        &self.editor
    }

    pub fn current_source(&self, cx: &App) -> String {
        let editor = self.editor.read(cx);
        let buffer = editor.buffer().read(cx);
        buffer
            .as_singleton()
            .map(|b| b.read(cx).text())
            .unwrap_or_default()
    }

    pub fn is_dirty(&self, cx: &App) -> bool {
        self.editor.read(cx).buffer().read(cx).is_dirty(cx)
    }

    pub fn to_nbformat_cell(&self, cx: &App) -> nbformat::v4::Cell {
        let source = self.current_source(cx);
        let source_lines: Vec<String> = source.lines().map(|l| format!("{}\n", l)).collect();

        nbformat::v4::Cell::Markdown {
            id: self.id.clone(),
            metadata: self.metadata.clone(),
            source: source_lines,
            attachments: None,
        }
    }

    pub fn is_editing(&self) -> bool {
        self.editing
    }

    pub fn set_editing(&mut self, editing: bool) {
        self.editing = editing;
    }

    pub fn reparse_markdown(&mut self, cx: &mut Context<Self>) {
        let editor = self.editor.read(cx);
        let buffer = editor.buffer().read(cx);
        let source = buffer
            .as_singleton()
            .map(|b| b.read(cx).text())
            .unwrap_or_default();

        self.source = source.clone();
        let languages = self.languages.clone();

        self.markdown.update(cx, |markdown, cx| {
            markdown.reset(source.into(), cx);
        });
    }

    /// Called when user presses Shift+Enter or Ctrl+Enter while editing.
    /// Finishes editing and signals to move to the next cell.
    pub fn run(&mut self, cx: &mut Context<Self>) {
        if self.editing {
            self.editing = false;
            cx.emit(MarkdownCellEvent::FinishedEditing);
            cx.emit(MarkdownCellEvent::Run(self.id.clone()));
            cx.notify();
        }
    }
}

impl RenderableCell for MarkdownCell {
    const CELL_TYPE: CellType = CellType::Markdown;

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

    fn control(&self, _window: &mut Window, _: &mut Context<Self>) -> Option<CellControl> {
        None
    }

    fn cell_position(&self) -> Option<&CellPosition> {
        self.cell_position.as_ref()
    }

    fn set_cell_position(&mut self, cell_position: CellPosition) -> &mut Self {
        self.cell_position = Some(cell_position);
        self
    }
}

impl Render for MarkdownCell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // If editing, show the editor
        if self.editing {
            return v_flex()
                .size_full()
                .children(self.cell_position_spacer(true, window, cx))
                .child(
                    h_flex()
                        .w_full()
                        .pr_6()
                        .rounded_xs()
                        .items_start()
                        .gap(DynamicSpacing::Base08.rems(cx))
                        .bg(self.selected_bg_color(window, cx))
                        .child(self.gutter(window, cx))
                        .child(
                            div()
                                .flex_1()
                                .p_3()
                                .bg(cx.theme().colors().editor_background)
                                .rounded_sm()
                                .child(self.editor.clone())
                                .on_mouse_down(
                                    gpui::MouseButton::Left,
                                    cx.listener(|_this, _event, _window, _cx| {
                                        // Prevent the click from propagating
                                    }),
                                ),
                        ),
                )
                .children(self.cell_position_spacer(false, window, cx));
        }

        // Preview mode - show rendered markdown

        let style = MarkdownStyle {
            base_text_style: window.text_style(),
            ..Default::default()
        };

        v_flex()
            .size_full()
            .children(self.cell_position_spacer(true, window, cx))
            .child(
                h_flex()
                    .w_full()
                    .pr_6()
                    .rounded_xs()
                    .items_start()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .bg(self.selected_bg_color(window, cx))
                    .child(self.gutter(window, cx))
                    .child(
                        v_flex()
                            .image_cache(self.image_cache.clone())
                            .id("markdown-content")
                            .size_full()
                            .flex_1()
                            .p_3()
                            .font_ui(cx)
                            .text_size(TextSize::Default.rems(cx))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _event, window, cx| {
                                this.editing = true;
                                window.focus(&this.editor.focus_handle(cx), cx);
                                cx.notify();
                            }))
                            .child(MarkdownElement::new(self.markdown.clone(), style)),
                    ),
            )
            .children(self.cell_position_spacer(false, window, cx))
    }
}

pub struct CodeCell {
    id: CellId,
    metadata: CellMetadata,
    execution_count: Option<i32>,
    source: String,
    editor: Entity<editor::Editor>,
    outputs: Vec<Output>,
    selected: bool,
    cell_position: Option<CellPosition>,
    language_task: Task<()>,
    execution_start_time: Option<Instant>,
    execution_duration: Option<Duration>,
    is_executing: bool,
}

impl EventEmitter<CellEvent> for CodeCell {}

impl CodeCell {
    pub fn new(
        id: CellId,
        metadata: CellMetadata,
        source: String,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = cx.new(|cx| Buffer::local(source.clone(), cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let editor_view = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(1024),
                },
                multi_buffer,
                None,
                window,
                cx,
            );

            let theme = ThemeSettings::get_global(cx);
            let refinement = TextStyleRefinement {
                font_family: Some(theme.buffer_font.family.clone()),
                font_size: Some(theme.buffer_font_size(cx).into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(gpui::transparent_black()),
                ..Default::default()
            };

            editor.set_show_gutter(false, cx);
            editor.set_text_style_refinement(refinement);
            editor
        });

        let language_task = cx.spawn_in(window, async move |_this, cx| {
            let language = notebook_language.await;
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(language.clone(), cx);
            });
        });

        Self {
            id,
            metadata,
            execution_count: None,
            source,
            editor: editor_view,
            outputs: Vec::new(),
            selected: false,
            cell_position: None,
            language_task,
            execution_start_time: None,
            execution_duration: None,
            is_executing: false,
        }
    }

    /// Load a code cell from notebook file data, including existing outputs and execution count
    pub fn load(
        id: CellId,
        metadata: CellMetadata,
        execution_count: Option<i32>,
        source: String,
        outputs: Vec<Output>,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = cx.new(|cx| Buffer::local(source.clone(), cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let editor_view = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(1024),
                },
                multi_buffer,
                None,
                window,
                cx,
            );

            let theme = ThemeSettings::get_global(cx);
            let refinement = TextStyleRefinement {
                font_family: Some(theme.buffer_font.family.clone()),
                font_size: Some(theme.buffer_font_size(cx).into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(gpui::transparent_black()),
                ..Default::default()
            };

            editor.set_text(source.clone(), window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_text_style_refinement(refinement);
            editor
        });

        let language_task = cx.spawn_in(window, async move |_this, cx| {
            let language = notebook_language.await;
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(language.clone(), cx);
            });
        });

        Self {
            id,
            metadata,
            execution_count,
            source,
            editor: editor_view,
            outputs,
            selected: false,
            cell_position: None,
            language_task,
            execution_start_time: None,
            execution_duration: None,
            is_executing: false,
        }
    }

    pub fn editor(&self) -> &Entity<editor::Editor> {
        &self.editor
    }

    pub fn current_source(&self, cx: &App) -> String {
        let editor = self.editor.read(cx);
        let buffer = editor.buffer().read(cx);
        buffer
            .as_singleton()
            .map(|b| b.read(cx).text())
            .unwrap_or_default()
    }

    pub fn is_dirty(&self, cx: &App) -> bool {
        self.editor.read(cx).buffer().read(cx).is_dirty(cx)
    }

    pub fn to_nbformat_cell(&self, cx: &App) -> nbformat::v4::Cell {
        let source = self.current_source(cx);
        let source_lines: Vec<String> = source.lines().map(|l| format!("{}\n", l)).collect();

        let outputs = self.outputs_to_nbformat(cx);

        nbformat::v4::Cell::Code {
            id: self.id.clone(),
            metadata: self.metadata.clone(),
            execution_count: self.execution_count,
            source: source_lines,
            outputs,
        }
    }

    fn outputs_to_nbformat(&self, cx: &App) -> Vec<nbformat::v4::Output> {
        self.outputs
            .iter()
            .filter_map(|output| output.to_nbformat(cx))
            .collect()
    }

    pub fn has_outputs(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
        self.execution_duration = None;
    }

    pub fn start_execution(&mut self) {
        self.execution_start_time = Some(Instant::now());
        self.execution_duration = None;
        self.is_executing = true;
    }

    pub fn finish_execution(&mut self) {
        if let Some(start_time) = self.execution_start_time.take() {
            self.execution_duration = Some(start_time.elapsed());
        }
        self.is_executing = false;
    }

    pub fn is_executing(&self) -> bool {
        self.is_executing
    }

    pub fn execution_duration(&self) -> Option<Duration> {
        self.execution_duration
    }

    fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs_f64();
        if total_secs < 1.0 {
            format!("{:.0}ms", duration.as_millis())
        } else if total_secs < 60.0 {
            format!("{:.1}s", total_secs)
        } else {
            let minutes = (total_secs / 60.0).floor() as u64;
            let secs = total_secs % 60.0;
            format!("{}m {:.1}s", minutes, secs)
        }
    }

    pub fn handle_message(
        &mut self,
        message: &JupyterMessage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match &message.content {
            JupyterMessageContent::StreamContent(stream) => {
                self.outputs.push(Output::Stream {
                    content: cx.new(|cx| TerminalOutput::from(&stream.text, window, cx)),
                });
            }
            JupyterMessageContent::DisplayData(display_data) => {
                self.outputs
                    .push(Output::new(&display_data.data, None, window, cx));
            }
            JupyterMessageContent::ExecuteResult(execute_result) => {
                self.outputs
                    .push(Output::new(&execute_result.data, None, window, cx));
            }
            JupyterMessageContent::ExecuteInput(input) => {
                self.execution_count = serde_json::to_value(&input.execution_count)
                    .ok()
                    .and_then(|v| v.as_i64())
                    .map(|v| v as i32);
            }
            JupyterMessageContent::ExecuteReply(_) => {
                self.finish_execution();
            }
            JupyterMessageContent::ErrorOutput(error) => {
                self.outputs.push(Output::ErrorOutput(ErrorView {
                    ename: error.ename.clone(),
                    evalue: error.evalue.clone(),
                    traceback: cx
                        .new(|cx| TerminalOutput::from(&error.traceback.join("\n"), window, cx)),
                }));
            }
            _ => {}
        }
        cx.notify();
    }

    fn output_control(&self) -> Option<CellControlType> {
        if self.has_outputs() {
            Some(CellControlType::ClearCell)
        } else {
            None
        }
    }

    pub fn gutter_output(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        .child(IconButton::new("control", IconName::Ellipsis)),
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

    fn control(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<CellControl> {
        let control_type = if self.has_outputs() {
            CellControlType::RerunCell
        } else {
            CellControlType::RunCell
        };

        let cell_control = CellControl::new(
            if self.has_outputs() {
                "rerun-cell"
            } else {
                "run-cell"
            },
            control_type,
        )
        .on_click(cx.listener(move |this, _, window, cx| this.run(window, cx)));

        Some(cell_control)
    }

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    fn cell_position(&self) -> Option<&CellPosition> {
        self.cell_position.as_ref()
    }

    fn set_cell_position(&mut self, cell_position: CellPosition) -> &mut Self {
        self.cell_position = Some(cell_position);
        self
    }

    fn gutter(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.selected();
        let execution_count = self.execution_count;

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
            .when_some(self.control(window, cx), |this, control| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_col()
                        .w(px(GUTTER_WIDTH))
                        .items_center()
                        .justify_center()
                        .bg(cx.theme().colors().tab_bar_background)
                        .child(control.button)
                        .when_some(execution_count, |this, count| {
                            this.child(
                                div()
                                    .mt_1()
                                    .text_xs()
                                    .text_color(cx.theme().colors().text_muted)
                                    .child(format!("{}", count)),
                            )
                        }),
                )
            })
    }
}

impl RunnableCell for CodeCell {
    fn run(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Running code cell: {}", self.id);
        cx.emit(CellEvent::Run(self.id.clone()));
    }

    fn execution_count(&self) -> Option<i32> {
        self.execution_count
            .and_then(|count| if count > 0 { Some(count) } else { None })
    }

    fn set_execution_count(&mut self, count: i32) -> &mut Self {
        self.execution_count = Some(count);
        self
    }
}

impl Render for CodeCell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // get the language from the editor's buffer
        let language_name = self
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| buffer.read(cx).language())
            .map(|lang| lang.name().to_string());

        v_flex()
            .size_full()
            // TODO: Move base cell render into trait impl so we don't have to repeat this
            .children(self.cell_position_spacer(true, window, cx))
            // Editor portion
            .child(
                h_flex()
                    .w_full()
                    .pr_6()
                    .rounded_xs()
                    .items_start()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .bg(self.selected_bg_color(window, cx))
                    .child(self.gutter(window, cx))
                    .child(
                        div().py_1p5().w_full().child(
                            div()
                                .relative()
                                .flex()
                                .size_full()
                                .flex_1()
                                .py_3()
                                .px_5()
                                .rounded_lg()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .bg(cx.theme().colors().editor_background)
                                .child(div().w_full().child(self.editor.clone()))
                                // lang badge in top-right corner
                                .when_some(language_name, |this, name| {
                                    this.child(
                                        div()
                                            .absolute()
                                            .top_1()
                                            .right_2()
                                            .px_2()
                                            .py_0p5()
                                            .rounded_md()
                                            .bg(cx.theme().colors().element_background.opacity(0.7))
                                            .text_xs()
                                            .text_color(cx.theme().colors().text_muted)
                                            .child(name),
                                    )
                                }),
                        ),
                    ),
            )
            // Output portion
            .when(
                self.has_outputs() || self.execution_duration.is_some() || self.is_executing,
                |this| {
                    let execution_time_label = self.execution_duration.map(Self::format_duration);
                    let is_executing = self.is_executing;
                    this.child(
                        h_flex()
                            .w_full()
                            .pr_6()
                            .rounded_xs()
                            .items_start()
                            .gap(DynamicSpacing::Base08.rems(cx))
                            .bg(self.selected_bg_color(window, cx))
                            .child(self.gutter_output(window, cx))
                            .child(
                                div().py_1p5().w_full().child(
                                    v_flex()
                                        .size_full()
                                        .flex_1()
                                        .py_3()
                                        .px_5()
                                        .rounded_lg()
                                        .border_1()
                                        // execution status/time at the TOP
                                        .when(
                                            is_executing || execution_time_label.is_some(),
                                            |this| {
                                                let time_element = if is_executing {
                                                    h_flex()
                                                        .gap_1()
                                                        .items_center()
                                                        .child(
                                                            Icon::new(IconName::ArrowCircle)
                                                                .size(IconSize::XSmall)
                                                                .color(Color::Warning)
                                                                .with_rotate_animation(2)
                                                                .into_any_element(),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(
                                                                    cx.theme().colors().text_muted,
                                                                )
                                                                .child("Running..."),
                                                        )
                                                        .into_any_element()
                                                } else if let Some(duration_text) =
                                                    execution_time_label.clone()
                                                {
                                                    h_flex()
                                                        .gap_1()
                                                        .items_center()
                                                        .child(
                                                            Icon::new(IconName::Check)
                                                                .size(IconSize::XSmall)
                                                                .color(Color::Success),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(
                                                                    cx.theme().colors().text_muted,
                                                                )
                                                                .child(duration_text),
                                                        )
                                                        .into_any_element()
                                                } else {
                                                    div().into_any_element()
                                                };
                                                this.child(div().mb_2().child(time_element))
                                            },
                                        )
                                        // output at bottom
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
                                                        div()
                                                            .child(message.clone())
                                                            .into_any_element(),
                                                    ),
                                                    Output::Table { content, .. } => {
                                                        Some(content.clone().into_any_element())
                                                    }
                                                    Output::Json { content, .. } => {
                                                        Some(content.clone().into_any_element())
                                                    }
                                                    Output::ErrorOutput(error_view) => {
                                                        error_view.render(window, cx)
                                                    }
                                                    Output::ClearOutputWaitMarker => None,
                                                };

                                                div().children(content)
                                            },
                                        ))),
                                ),
                            ),
                    )
                },
            )
            // TODO: Move base cell render into trait impl so we don't have to repeat this
            .children(self.cell_position_spacer(false, window, cx))
    }
}

pub struct RawCell {
    id: CellId,
    metadata: CellMetadata,
    source: String,
    selected: bool,
    cell_position: Option<CellPosition>,
}

impl RawCell {
    pub fn to_nbformat_cell(&self) -> nbformat::v4::Cell {
        let source_lines: Vec<String> = self.source.lines().map(|l| format!("{}\n", l)).collect();

        nbformat::v4::Cell::Raw {
            id: self.id.clone(),
            metadata: self.metadata.clone(),
            source: source_lines,
        }
    }
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

    fn selected(&self) -> bool {
        self.selected
    }

    fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    fn cell_position(&self) -> Option<&CellPosition> {
        self.cell_position.as_ref()
    }

    fn set_cell_position(&mut self, cell_position: CellPosition) -> &mut Self {
        self.cell_position = Some(cell_position);
        self
    }
}

impl Render for RawCell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            // TODO: Move base cell render into trait impl so we don't have to repeat this
            .children(self.cell_position_spacer(true, window, cx))
            .child(
                h_flex()
                    .w_full()
                    .pr_2()
                    .rounded_xs()
                    .items_start()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .bg(self.selected_bg_color(window, cx))
                    .child(self.gutter(window, cx))
                    .child(
                        div()
                            .flex()
                            .size_full()
                            .flex_1()
                            .p_3()
                            .font_ui(cx)
                            .text_size(TextSize::Default.rems(cx))
                            .child(self.source.clone()),
                    ),
            )
            // TODO: Move base cell render into trait impl so we don't have to repeat this
            .children(self.cell_position_spacer(false, window, cx))
    }
}
