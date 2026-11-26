use std::sync::Arc;
use std::time::{Duration, Instant};

use editor::{
    Editor, EditorMode, EditorSettings, EditorSettingsScrollbarProxy, MultiBuffer, SizingBehavior,
};
use futures::{StreamExt as _, future::Shared};
use gpui::{
    App, Entity, EventEmitter, Focusable, Hsla, InteractiveElement, StatefulInteractiveElement,
    Task, prelude::*,
};
use jupyter_protocol::{JupyterMessage, JupyterMessageContent};
use language::{Buffer, Language, LanguageRegistry};
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use nbformat::v4::{CellId, CellMetadata, CellType};
use settings::Settings as _;
use ui::{CommonAnimationExt, IconButtonShape, ScrollAxes, Scrollbars, WithScrollbar, prelude::*};
use util::ResultExt;
use zed_actions::notebook::InterruptKernel;

use crate::{
    notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH},
    outputs::{Output, plain, plain::TerminalOutput, user_error::ErrorView},
    repl_settings::ReplSettings,
};

fn preserve_middle_click(div: gpui::Div, cx: &App) -> gpui::Div {
    div.when(
        cx.platform_info().os == gpui::OperatingSystem::Windows,
        |div| div.on_mouse_down(gpui::MouseButton::Middle, |_, _, cx| cx.stop_propagation()),
    )
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum CellPosition {
    First,
    Middle,
    Last,
}

pub enum CellControlType {
    RunCell,
    RerunCell,
    StopCell,
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
            CellControlType::StopCell => IconName::Stop,
            CellControlType::ClearCell => IconName::ListX,
            CellControlType::CellOptions => IconName::Ellipsis,
            CellControlType::CollapseCell => IconName::ChevronDown,
            CellControlType::ExpandCell => IconName::ChevronRight,
        }
    }
    fn id(&self) -> &'static str {
        match self {
            CellControlType::RunCell => "CellControlType::RunCell",
            CellControlType::RerunCell => "CellControlType::RerunCell",
            CellControlType::StopCell => "CellControlType::StopCell",
            CellControlType::ClearCell => "CellControlType::ClearCell",
            CellControlType::CellOptions => "CellControlType::CellOptions",
            CellControlType::CollapseCell => "CellControlType::CollapseCelln",
            CellControlType::ExpandCell => "CellControlType::ExpandCell",
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

pub(crate) enum MovementDirection {
    Start,
    End,
}

fn convert_outputs(
    outputs: &[nbformat::v4::Output],
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
                Output::for_notebook(&display_data.data, window, cx)
            }
            nbformat::v4::Output::ExecuteResult(execute_result) => {
                Output::for_notebook(&execute_result.data, window, cx)
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
                let source = source.concat();

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
                let text = source.concat();
                let outputs = convert_outputs(outputs, window, cx);

                Cell::Code(cx.new(|cx| {
                    CodeCell::new(
                        *execution_count,
                        outputs,
                        id.clone(),
                        metadata.clone(),
                        text,
                        languages.clone(),
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
                source: source.concat(),
                selected: false,
                cell_position: None,
            })),
        }
    }

    pub(crate) fn move_to(&self, direction: MovementDirection, window: &mut Window, cx: &mut App) {
        fn move_in_editor(
            editor: &Entity<Editor>,
            direction: MovementDirection,
            window: &mut Window,
            cx: &mut App,
        ) {
            editor.update(cx, |editor, cx| {
                match direction {
                    MovementDirection::Start => {
                        editor.move_to_beginning(&Default::default(), window, cx);
                    }
                    MovementDirection::End => {
                        editor.move_to_end(&Default::default(), window, cx);
                    }
                }
                editor.focus_handle(cx).focus(window, cx);
            })
        }

        match self {
            Cell::Code(cell) => {
                cell.update(cx, |cell, cx| {
                    move_in_editor(&cell.editor, direction, window, cx)
                });
            }
            Cell::Markdown(cell) => {
                cell.update(cx, |cell, cx| {
                    cell.set_editing(true);
                    move_in_editor(&cell.editor, direction, window, cx);

                    cx.notify();
                });
            }
            _ => {}
        }
    }

    pub(crate) fn editor<'a>(&'a self, cx: &'a App) -> Option<&'a Entity<Editor>> {
        match self {
            Cell::Code(cell) => Some(cell.read(cx).editor()),
            Cell::Markdown(cell) => Some(cell.read(cx).editor()),
            _ => None,
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
    fn selected_bg_color(&self, _window: &mut Window, cx: &mut Context<Self>) -> Hsla {
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
        _window: &mut Window,
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

/// Creates a cell's buffer along with a task that keeps its injections resolved.
/// This task is subscribed to `LanguageRegistry`'s changes, so that the cell is re-parsed
/// whenever loaded languages change. This is a simpler version of what
/// `LspStore::maintain_buffer_languages` does for other project buffers, but this way does not
/// require registering each cell as a project buffer.
fn new_cell_buffer(
    source: &str,
    languages: Arc<LanguageRegistry>,
    cx: &mut App,
) -> (Entity<Buffer>, Task<()>) {
    let buffer = cx.new(|cx| {
        let line_ending = match source.split_once('\n') {
            Some((first_line, _)) if first_line.ends_with('\r') => language::LineEnding::Windows,
            _ => language::LineEnding::Unix,
        };
        let mut source = source.to_owned();
        language::LineEnding::normalize(&mut source);
        let buffer = Buffer::local_normalized(source.as_str().into(), line_ending, cx);
        buffer.set_language_registry(languages.clone());
        buffer
    });

    let weak_buffer = buffer.downgrade();
    let mut registry_changes = languages.subscribe();
    let reparse_task = cx.spawn(async move |cx| {
        while registry_changes.next().await.is_some() {
            let updated = weak_buffer.update(cx, |buffer, cx| {
                if buffer.contains_unknown_injections() {
                    buffer.reparse(cx, false);
                }
            });
            if updated.is_err() {
                break;
            }
        }
    });

    (buffer, reparse_task)
}

pub struct MarkdownCell {
    id: CellId,
    metadata: CellMetadata,
    source: String,
    editor: Entity<Editor>,
    markdown: Entity<Markdown>,
    editing: bool,
    selected: bool,
    cell_position: Option<CellPosition>,
    _editor_subscription: gpui::Subscription,
    _reparse_task: Task<()>,
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
        let (buffer, reparse_task) = new_cell_buffer(&source, languages.clone(), cx);
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
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sizing_behavior: SizingBehavior::SizeByContent,
                },
                multi_buffer,
                None,
                window,
                cx,
            );

            editor.set_show_gutter(false, cx);
            editor.set_use_modal_editing(true);
            editor.disable_mouse_wheel_zoom();
            editor.disable_scrollbars_and_minimap(window, cx);
            editor
        });

        let markdown = cx.new(|cx| Markdown::new(source.clone().into(), None, None, cx));

        let editor_subscription =
            cx.subscribe(&editor, move |this, _editor, event, cx| match event {
                editor::EditorEvent::Focused if !this.editing => {
                    this.editing = true;
                    cx.notify();
                }
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
            source,
            editor,
            markdown,
            editing: start_editing,
            selected: false,
            cell_position: None,
            _editor_subscription: editor_subscription,
            _reparse_task: reparse_task,
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
                            preserve_middle_click(div(), cx)
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
            links_as_text: true,
            prevent_mouse_interaction: true,
            ..MarkdownStyle::themed(MarkdownFont::Preview, window, cx)
        };

        v_flex()
            .size_full()
            // A dismissed prompt or palette can restore this handle while the editor is hidden.
            .child(div().size_0().track_focus(&self.editor.focus_handle(cx)))
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
    pub(super) output_source: String,
    editor: Entity<editor::Editor>,
    outputs: Vec<Output>,
    output_scroll: gpui::ScrollHandle,
    selected: bool,
    cell_position: Option<CellPosition>,
    _editor_subscription: gpui::Subscription,
    _language_task: Task<()>,
    _reparse_task: Task<()>,
    execution_start_time: Option<Instant>,
    execution_duration: Option<Duration>,
    is_executing: bool,
}

impl EventEmitter<CellEvent> for CodeCell {}

pub(super) struct PreviousExecution {
    source: String,
    outputs: Vec<Output>,
    count: Option<i32>,
    duration: Option<Duration>,
}

impl CodeCell {
    pub(super) fn new(
        execution_count: Option<i32>,
        outputs: Vec<Output>,
        id: CellId,
        metadata: CellMetadata,
        mut source: String,
        languages: Arc<LanguageRegistry>,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (buffer, reparse_task) = new_cell_buffer(&source, languages, cx);
        language::LineEnding::normalize(&mut source);
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer.clone(), cx));

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(
                EditorMode::Full {
                    scale_ui_elements_with_buffer_font_size: false,
                    show_active_line_background: false,
                    sizing_behavior: SizingBehavior::SizeByContent,
                },
                multi_buffer,
                None,
                window,
                cx,
            );

            editor.disable_mouse_wheel_zoom();
            editor.disable_scrollbars_and_minimap(window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_use_modal_editing(true);
            editor
        });

        let language_task = cx.spawn_in(window, async move |_this, cx| {
            let language = notebook_language.await;
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(language.clone(), cx);
            });
        });

        let editor_subscription = cx.subscribe(&editor, |_, _, event, cx| {
            if matches!(event, editor::EditorEvent::BufferEdited) {
                cx.notify();
            }
        });

        Self {
            id,
            metadata,
            execution_count,
            output_source: source,
            editor,
            outputs,
            output_scroll: Default::default(),
            selected: false,
            cell_position: None,
            execution_start_time: None,
            execution_duration: None,
            is_executing: false,
            _language_task: language_task,
            _reparse_task: reparse_task,
            _editor_subscription: editor_subscription,
        }
    }

    pub fn set_language(&mut self, language: Option<Arc<Language>>, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                if let Some(buffer) = buffer.as_singleton() {
                    buffer.update(cx, |buffer, cx| {
                        buffer.set_language(language, cx);
                    });
                }
            });
        });
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

    #[cfg(test)]
    pub(super) fn output_views(&self) -> &[Output] {
        &self.outputs
    }

    pub fn has_outputs(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub(super) fn has_results(&self) -> bool {
        self.has_outputs() || self.execution_count.is_some()
    }

    pub(super) fn has_stale_outputs(&self, cx: &App) -> bool {
        self.has_results() && self.current_source(cx) != self.output_source
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
        self.execution_duration = None;
    }

    pub(super) fn clear_results(&mut self) {
        self.clear_outputs();
        self.execution_count = None;
    }

    pub(super) fn queue_execution(&mut self) {
        self.is_executing = true;
    }

    pub(super) fn start_execution(&mut self, source: String) -> PreviousExecution {
        self.execution_start_time = Some(Instant::now());
        self.is_executing = true;
        PreviousExecution {
            source: std::mem::replace(&mut self.output_source, source),
            outputs: std::mem::take(&mut self.outputs),
            count: self.execution_count.take(),
            duration: self.execution_duration.take(),
        }
    }

    pub(super) fn cancel_execution(&mut self, previous: Option<PreviousExecution>) {
        if let Some(previous) = previous {
            self.output_source = previous.source;
            self.outputs = previous.outputs;
            self.execution_count = previous.count;
            self.execution_duration = previous.duration;
        }
        self.execution_start_time = None;
        self.is_executing = false;
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

    pub(super) fn update_display(
        &mut self,
        indices: &[usize],
        data: &jupyter_protocol::Media,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for index in indices {
            if let Some(output) = self.outputs.get_mut(*index) {
                *output = Output::for_notebook(data, window, cx);
            }
        }
        cx.notify();
    }

    /// Displays a kernel-level failure (e.g. the kernel failed to launch because
    /// Python is not installed) as an error output on this cell, so the user gets
    /// feedback instead of a spinner that never resolves.
    pub fn show_kernel_error(
        &mut self,
        error_message: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.outputs.push(Output::ErrorOutput(ErrorView {
            ename: "Kernel Error".to_string(),
            evalue: "cell could not be executed".to_string(),
            traceback: cx.new(|cx| TerminalOutput::from(error_message, window, cx)),
        }));
        self.execution_start_time = None;
        self.is_executing = false;
        cx.notify();
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
                    .push(Output::for_notebook(&display_data.data, window, cx));
            }
            JupyterMessageContent::ExecuteResult(execute_result) => {
                self.outputs
                    .push(Output::for_notebook(&execute_result.data, window, cx));
            }
            JupyterMessageContent::ExecuteInput(input) => {
                self.execution_count = i32::try_from(input.execution_count.value()).ok();
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

    pub fn gutter_output(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
        &self.output_source
    }

    fn control(&self, _window: &mut Window, cx: &mut Context<Self>) -> Option<CellControl> {
        let control_type = if self.is_executing {
            CellControlType::StopCell
        } else if self.has_outputs() {
            CellControlType::RerunCell
        } else {
            CellControlType::RunCell
        };

        Some(
            CellControl::new(control_type.id(), control_type).on_click(cx.listener(
                move |this, _, window, cx| {
                    if this.is_executing {
                        window.dispatch_action(Box::new(InterruptKernel), cx);
                    } else {
                        this.run(window, cx);
                    }
                },
            )),
        )
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
    fn run(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        cx.emit(CellEvent::Run(self.id.clone()));
    }

    fn execution_count(&self) -> Option<i32> {
        self.execution_count.filter(|&count| count > 0)
    }

    fn set_execution_count(&mut self, count: i32) -> &mut Self {
        self.execution_count = Some(count);
        self
    }
}

impl CodeCell {
    fn render_output_viewport(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let output_max_height = ReplSettings::get_global(cx).output_max_height_lines;
        let output_max_height = if output_max_height > 0 {
            Some(window.line_height() * output_max_height as f32)
        } else {
            None
        };
        let output_max_width =
            plain::max_width_for_columns(ReplSettings::get_global(cx).max_columns, window, cx);
        let content = v_flex().flex_none().min_w_full().children(
            self.outputs
                .iter()
                .filter_map(|output| output.content(window, cx)),
        );
        let horizontal = div()
            .id("output-scroll")
            .flex()
            .items_start()
            .w_full()
            .overflow_x_scroll()
            .restrict_scroll_to_axis()
            .track_scroll(&self.output_scroll)
            .child(content);
        div()
            .id("output-viewport")
            .relative()
            .w_full()
            .when_some(output_max_width, |div, width| div.max_w(width))
            .child(
                div()
                    .id("output-height")
                    .when_some(output_max_height, |div, height| {
                        div.max_h(height)
                            .overflow_y_scroll()
                            .restrict_scroll_to_axis()
                    })
                    .child(horizontal),
            )
            .when(
                EditorSettings::get_global(cx).scrollbar.axes.horizontal,
                |div| {
                    div.custom_scrollbars(
                        Scrollbars::for_settings::<EditorSettingsScrollbarProxy>()
                            .tracked_scroll_handle(&self.output_scroll)
                            .with_track_along(
                                ScrollAxes::Horizontal,
                                cx.theme().colors().editor_background,
                            )
                            .notify_content(),
                        window,
                        cx,
                    )
                },
            )
    }
}

impl Render for CodeCell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let stale = self.has_stale_outputs(cx);
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
                                .flex()
                                .items_start()
                                .gap_2()
                                .size_full()
                                .flex_1()
                                .py_3()
                                .px_5()
                                .rounded_lg()
                                .border_1()
                                .border_color(cx.theme().colors().border)
                                .bg(cx.theme().colors().editor_background)
                                .child(
                                    preserve_middle_click(div(), cx)
                                        .flex_1()
                                        .min_w_0()
                                        .child(self.editor.clone()),
                                )
                                // lang badge in top-right corner
                                .when_some(language_name, |this, name| {
                                    this.child(
                                        div()
                                            .flex_none()
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
            .when(
                self.has_outputs()
                    || self.execution_duration.is_some()
                    || self.is_executing
                    || stale,
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
                                                                .child(
                                                                    if self
                                                                        .execution_start_time
                                                                        .is_some()
                                                                    {
                                                                        "Running..."
                                                                    } else {
                                                                        "Queued..."
                                                                    },
                                                                ),
                                                        )
                                                        .into_any_element()
                                                } else if let Some(duration_text) =
                                                    execution_time_label.clone()
                                                {
                                                    let (icon, color) =
                                                        if self.outputs.iter().any(|output| {
                                                            matches!(output, Output::ErrorOutput(_))
                                                        }) {
                                                            (IconName::XCircle, Color::Error)
                                                        } else {
                                                            (IconName::Check, Color::Success)
                                                        };
                                                    h_flex()
                                                        .gap_1()
                                                        .items_center()
                                                        .child(
                                                            Icon::new(icon)
                                                                .size(IconSize::XSmall)
                                                                .color(color),
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
                                        .when(stale, |this| {
                                            this.child(
                                                Label::new("Cell changed since last execution")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Warning),
                                            )
                                        })
                                        .child(self.render_output_viewport(window, cx)),
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

#[cfg(test)]
mod tests {
    use super::*;

    use collections::HashSet;
    use futures::FutureExt as _;
    use gpui::TestAppContext;
    use indoc::indoc;
    use language::LanguageConfig;
    use settings::SettingsStore;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    #[gpui::test]
    async fn drag_wide_notebook_output(cx: &mut TestAppContext) {
        use crate::notebook::{
            NotebookPreview,
            preview::preview_tests::{notebook_workspace, open_file},
        };
        use gpui::{Modifiers, MouseButton, point, size};
        use serde_json::json;
        let file = json!({"nbformat":4, "nbformat_minor":5, "metadata":{}, "cells":[{
            "cell_type":"code", "id":"wide", "metadata":{}, "source":"x", "execution_count":1,
            "outputs":[{"output_type":"stream", "name":"stdout", "text":"x".repeat(90)}]
        }]})
        .to_string();
        let (_, workspace, cx) = notebook_workspace(json!({"wide.ipynb": file}), cx).await;
        cx.update(|_, cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.scrollbar.get_or_insert_default().show =
                        Some(settings::ShowScrollbar::Always);
                });
            })
        });
        cx.simulate_resize(size(px(500.), px(640.)));
        open_file("wide.ipynb", &workspace, cx).await;
        let scroll = workspace.read_with(cx, |workspace, cx| {
            let tab = workspace.active_item_as::<NotebookPreview>(cx).unwrap();
            let notebook = tab.read(cx).notebook.as_ref().unwrap().read(cx);
            let Some(Cell::Code(cell)) = notebook.cell(0) else {
                panic!("expected code cell")
            };
            cell.read(cx).output_scroll.clone()
        });
        assert!(scroll.max_offset().x > px(0.));
        let viewport = scroll.bounds();
        let thumb = point(viewport.left() + px(10.), viewport.bottom() + px(7.));
        let right = point(viewport.right(), thumb.y);
        let modifiers = Modifiers::none();
        cx.simulate_mouse_down(thumb, MouseButton::Left, modifiers);
        cx.simulate_mouse_move(right, MouseButton::Left, modifiers);
        cx.simulate_mouse_up(right, MouseButton::Left, modifiers);
        assert!(
            (scroll.offset().x + scroll.max_offset().x).abs() <= px(1.),
            "dragging the thumb exposes the last output column"
        );
        cx.simulate_resize(size(px(1800.), px(640.)));
        cx.run_until_parked();
        assert_eq!(
            scroll.max_offset().x,
            px(0.),
            "fitting output has no scroll range"
        );
    }

    #[gpui::test]
    fn test_notebook_static_outputs(cx: &mut TestAppContext) {
        init_test(cx);
        let languages = Arc::new(LanguageRegistry::test(cx.executor()));
        let cell = serde_json::from_value(serde_json::json!({
            "cell_type": "code", "id": "static", "metadata": {}, "source": "x",
            "execution_count": 1,
            "outputs": [
                {"output_type":"display_data", "metadata":{}, "data":{
                    "image/png":["iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAAAAAA6fptV\n",
                                 "AAAACklEQVR4nGP4DwABAQEAsTj2FAAAAABJRU5ErkJggg=="],
                    "text/plain":"figure", "text/html":"<div>rich output</div>"}},
                {"output_type":"execute_result", "execution_count":1, "metadata":{}, "data":{
                    "text/plain":"42", "text/markdown":"![remote](https://example.com/image)"}},
                {"output_type":"display_data", "metadata":{}, "data":{
                    "text/html":"<img src='https://example.com/image'>",
                    "application/javascript":"ignored", "image/svg+xml":"<svg/>"}},
                {"output_type":"display_data", "metadata":{}, "data":{
                    "image/png":"R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw=="}},
                {"output_type":"stream", "name":"stderr", "text":["one\n", "two\n"]},
                {"output_type":"error", "ename":"ValueError", "evalue":"bad value",
                 "traceback":["traceback"]}
            ]
        }))
        .expect("valid code cell");
        let cx = cx.add_empty_window();
        let Cell::Code(cell) = cx.update(|window, cx| {
            Cell::load(&cell, &languages, Task::ready(None).shared(), window, cx)
        }) else {
            panic!("expected code cell")
        };
        cell.read_with(cx, |cell, cx| {
            assert!(!cell.has_stale_outputs(cx));
            let [
                Output::Image { .. },
                Output::Markdown { .. },
                Output::Message(_),
                Output::Message(image_error),
                Output::Stream { content: stream },
                Output::ErrorOutput(error),
            ] = cell.outputs.as_slice()
            else {
                panic!("expected PNG, Markdown, unsupported notice, invalid PNG, stream, and error")
            };
            assert!(image_error.contains("Failed to load PNG"));
            assert_eq!(stream.read(cx).full_text(cx), "one\ntwo\n");
            assert_eq!(error.evalue, "bad value");
            assert_eq!(error.traceback.read(cx).full_text(cx), "traceback\n");
        });

        let data = serde_json::from_value(serde_json::json!({
            "text/plain": "live result", "text/html": "<div>rich output</div>"
        }))
        .expect("valid MIME bundle");
        cell.update_in(cx, |cell, window, cx| {
            cell.editor
                .update(cx, |editor, cx| editor.set_text("y", window, cx));
            assert!(cell.has_stale_outputs(cx));
            assert_eq!(cell.output_views().len(), 6);
            cell.start_execution("y".into());
            assert!(!cell.has_outputs());
            assert_eq!(cell.execution_count(), None);
            let message = jupyter_protocol::DisplayData::new(data).into();
            cell.handle_message(&message, window, cx);
            let Some(Output::Plain { content, .. }) = cell.outputs.last() else {
                panic!("live output should use the same passive renderer");
            };
            assert_eq!(content.read(cx).full_text(cx), "live result\n");
            assert!(!cell.has_stale_outputs(cx));
        });
    }

    #[gpui::test]
    fn test_typing_in_notebook_cells(cx: &mut TestAppContext) {
        init_test(cx);
        let languages = Arc::new(LanguageRegistry::test(cx.executor()));
        languages.add(languages::markdown_lang());
        cx.update(|cx| {
            cx.set_http_client(http_client::FakeHttpClient::create(|request| async move {
                panic!("opening a notebook must not fetch {}", request.uri())
            }));
        });
        let source = "![remote](https://example.com/image)";
        let (markdown, markdown_cx) = cx.add_window_view(|window, cx| {
            MarkdownCell::new(
                uuid::Uuid::new_v4().into(),
                CellMetadata::default(),
                source.into(),
                languages.clone(),
                window,
                cx,
            )
        });
        markdown_cx.update(|window, cx| {
            Cell::Markdown(markdown.clone()).move_to(MovementDirection::End, window, cx);
        });
        markdown_cx.run_until_parked();
        markdown_cx.simulate_input(" caption");
        markdown.read_with(markdown_cx, |cell, cx| {
            assert_eq!(cell.current_source(cx), format!("{source} caption"));
        });

        let (code, code_cx) = cx.add_window_view(|window, cx| {
            CodeCell::new(
                None,
                Vec::new(),
                uuid::Uuid::new_v4().into(),
                CellMetadata::default(),
                "x = 1".into(),
                languages,
                Task::ready(None).shared(),
                window,
                cx,
            )
        });
        code_cx.update(|window, cx| {
            Cell::Code(code.clone()).move_to(MovementDirection::End, window, cx);
        });
        code_cx.run_until_parked();
        code_cx.simulate_input("0");
        code.read_with(code_cx, |cell, cx| {
            assert_eq!(cell.current_source(cx), "x = 10")
        });
    }

    /// Returns markdown-inline language for immediate registration into a buffer's language repo.
    fn markdown_inline_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Markdown-Inline".into(),
                hidden: true,
                ..LanguageConfig::default()
            },
            Some(tree_sitter_md::INLINE_LANGUAGE.into()),
        )
        .with_highlights_query("(emphasis) @emphasis")
        .unwrap()
    }

    /// Registers Markdown-Inline the way the app does: available by name, but
    /// only loaded once a buffer's injection first asks for it.
    fn register_markdown_inline_lazily(languages: &LanguageRegistry) {
        languages.register_native_grammars([("markdown-inline", tree_sitter_md::INLINE_LANGUAGE)]);
        languages.register_test_language(LanguageConfig {
            name: "Markdown-Inline".into(),
            grammar: Some("markdown-inline".into()),
            hidden: true,
            ..LanguageConfig::default()
        });
    }

    fn assert_layer_languages(editor: &Entity<Editor>, expected_langs: &[&str], cx: &App) {
        let buffer = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
        let snapshot = buffer.read(cx).snapshot();
        let actual = snapshot
            .syntax_layers_languages()
            .map(|language| language.name().to_string())
            .collect::<HashSet<String>>();

        let expected = expected_langs
            .iter()
            .map(|name| name.to_string())
            .collect::<HashSet<String>>();
        assert_eq!(
            expected, actual,
            "cell languages mismatch: expected {expected:?}, got {actual:?}"
        )
    }

    fn run_cell_language_test(cx: &mut TestAppContext, cell_source: &str, expected_langs: &[&str]) {
        init_test(cx);

        let languages = Arc::new(LanguageRegistry::test(cx.executor()));

        languages.add(languages::json_lang());
        languages.add(languages::rust_lang());
        languages.add(languages::markdown_lang());
        languages.add(Arc::new(markdown_inline_lang()));

        let metadata: CellMetadata = serde_json::from_str("{}").unwrap();

        let cx = cx.add_empty_window();
        let cell = cx.update(|window, cx| {
            cx.new(|cx| {
                MarkdownCell::new(
                    uuid::Uuid::new_v4().into(),
                    metadata,
                    cell_source.to_string(),
                    languages,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();

        cell.read_with(cx, |cell, cx| {
            assert_layer_languages(&cell.editor, expected_langs, cx)
        });
    }

    #[gpui::test]
    fn test_markdown_cell_parses_injections(cx: &mut TestAppContext) {
        run_cell_language_test(cx, "Hello, World!", &["Markdown", "Markdown-Inline"]);
        run_cell_language_test(
            cx,
            indoc! {r#"
                Some *emphasized* text
                ```json
                { "foo": "bar" }
                ```
            "#},
            &["Markdown", "Markdown-Inline", "JSON"],
        );
        run_cell_language_test(
            cx,
            indoc! {r#"
                ```json
                [ "a", "b", { "foo": "bar" }]
                ```
                ```rs
                fn main() {
                    println!("Hello, World!");
                }
                ```
            "#},
            &["Markdown", "JSON", "Rust"],
        );
    }

    #[gpui::test]
    fn test_markdown_cell_parses_lazily_loaded_injections(cx: &mut TestAppContext) {
        init_test(cx);

        let languages = Arc::new(LanguageRegistry::test(cx.executor()));
        languages.add(languages::markdown_lang());
        register_markdown_inline_lazily(&languages);

        let metadata: CellMetadata = serde_json::from_str("{}").unwrap();

        let cx = cx.add_empty_window();
        let cell = cx.update(|window, cx| {
            cx.new(|cx| {
                MarkdownCell::new(
                    uuid::Uuid::new_v4().into(),
                    metadata,
                    "Some *emphasized* text".to_string(),
                    languages,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();

        cell.read_with(cx, |cell, cx| {
            assert_layer_languages(&cell.editor, &["Markdown", "Markdown-Inline"], cx)
        });
    }

    #[gpui::test]
    fn test_code_cell_parses_lazily_loaded_injections(cx: &mut TestAppContext) {
        init_test(cx);

        let languages = Arc::new(LanguageRegistry::test(cx.executor()));
        languages.add(languages::markdown_lang());
        register_markdown_inline_lazily(&languages);

        // Any kernel language can inject others; Markdown is the injecting
        // language available to these tests.
        let notebook_language = Task::ready(Some(languages::markdown_lang())).shared();
        let metadata: CellMetadata = serde_json::from_str("{}").unwrap();

        let cx = cx.add_empty_window();
        let cell = cx.update(|window, cx| {
            cx.new(|cx| {
                CodeCell::new(
                    None,
                    Vec::new(),
                    uuid::Uuid::new_v4().into(),
                    metadata,
                    "Some *emphasized* text".to_string(),
                    languages,
                    notebook_language,
                    window,
                    cx,
                )
            })
        });
        cx.run_until_parked();

        cell.read_with(cx, |cell, cx| {
            assert_layer_languages(&cell.editor, &["Markdown", "Markdown-Inline"], cx)
        });
    }
}
