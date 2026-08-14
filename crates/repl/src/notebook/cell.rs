use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use editor::{CompletionProvider, Editor, EditorMode, MultiBuffer, SizingBehavior};
use futures::channel::oneshot;
use futures::future::Shared;
use gpui::{
    App, Context, Entity, EventEmitter, Focusable, Hsla, InteractiveElement, RetainAllImageCache,
    StatefulInteractiveElement, Task, WeakEntity, Window, prelude::*,
};
use language::{Buffer, CodeLabel, Language, LanguageRegistry, ToOffset};
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use nbformat::v4::{CellId, CellMetadata, CellType};
use project::lsp_store::CompletionDocumentation;
use project::{
    Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource,
};
use runtimelib::{
    CompleteReply, CompleteRequest, InspectReply, InspectRequest, JupyterMessage,
    JupyterMessageContent, MediaType,
};
use settings::Settings as _;
use ui::{CommonAnimationExt, IconButtonShape, prelude::*};
use util::ResultExt;
use zed_actions::notebook::InterruptKernel;

use crate::{
    notebook::{CODE_BLOCK_INSET, GUTTER_WIDTH},
    outputs::{Output, plain, plain::TerminalOutput, user_error::ErrorView},
    repl_settings::ReplSettings,
};

use super::notebook_ui::NotebookEditor;

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

    pub fn cell_type(&self, cx: &App) -> CellType {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).cell_type(),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).cell_type(),
            Cell::Raw(raw_cell) => raw_cell.read(cx).cell_type(),
        }
    }

    pub fn metadata(&self, cx: &App) -> CellMetadata {
        match self {
            Cell::Code(code_cell) => code_cell.read(cx).metadata().clone(),
            Cell::Markdown(markdown_cell) => markdown_cell.read(cx).metadata().clone(),
            Cell::Raw(raw_cell) => raw_cell.read(cx).metadata().clone(),
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
        notebook_editor: WeakEntity<NotebookEditor>,
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
                        CellSource::Existing {
                            execution_count: *execution_count,
                            outputs,
                        },
                        id.clone(),
                        metadata.clone(),
                        text,
                        notebook_language,
                        notebook_editor.clone(),
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
    fn controls(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Vec<CellControl> {
        Vec::new()
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
        let controls = self.controls(window, cx);

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
            .when(!controls.is_empty(), |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .w(px(GUTTER_WIDTH))
                        .bg(cx.theme().colors().tab_bar_background)
                        .children(controls.into_iter().map(|control| control.button)),
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

    fn controls(&self, _window: &mut Window, _: &mut Context<Self>) -> Vec<CellControl> {
        Vec::new()
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

        let style = MarkdownStyle::themed(MarkdownFont::Preview, window, cx);

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
    _language_task: Task<()>,
    execution_start_time: Option<Instant>,
    execution_duration: Option<Duration>,
    is_executing: bool,
    outputs_collapsed: bool,
}

/// Provides completions for a code cell's editor by querying the notebook's
/// running kernel over the Jupyter `complete_request`/`complete_reply` protocol.
pub struct KernelCompletionProvider {
    notebook_editor: WeakEntity<NotebookEditor>,
}

impl KernelCompletionProvider {
    pub fn new(notebook_editor: WeakEntity<NotebookEditor>) -> Self {
        Self { notebook_editor }
    }
}

impl CompletionProvider for KernelCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        buffer_position: language::Anchor,
        _trigger: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<Vec<CompletionResponse>>> {
        let Some(notebook_editor) = self.notebook_editor.upgrade() else {
            return Task::ready(Ok(Vec::new()));
        };

        let snapshot = buffer.read(cx).snapshot();
        let code = snapshot.text();
        let cursor_byte_offset = buffer_position.to_offset(&snapshot);

        // The editor only re-queries the provider when the current query does not
        // extend the previous one, so this runs at most once per word (the same way
        // an LSP server would be queried). Skip the request only when there is no
        // identifier or trigger character before the cursor.
        let char_before_cursor = code[..cursor_byte_offset].chars().next_back();
        if !char_before_cursor.is_some_and(|char| {
            char.is_alphanumeric() || char == '_' || KERNEL_COMPLETION_TRIGGERS.contains(&char)
        }) {
            return Task::ready(Ok(Vec::new()));
        }

        let cursor_pos = byte_offset_to_char_offset(&code, cursor_byte_offset);

        let request = CompleteRequest {
            code: code.clone(),
            cursor_pos,
        };
        let message: JupyterMessage = request.into();

        let (tx, rx) = oneshot::channel::<CompleteReply>();

        let send_result = notebook_editor.update(cx, |editor, cx| {
            editor.request_completions(message, tx, cx)
        });
        if send_result.is_err() {
            return Task::ready(Ok(Vec::new()));
        }

        let snapshot = snapshot;
        cx.background_spawn(async move {
            let reply = match rx.await {
                Ok(reply) => reply,
                Err(_) => return Ok(Vec::new()),
            };
            let completions = reply
                .matches
                .into_iter()
                .map(|text| {
                    let start_byte = char_offset_to_byte_offset(&code, reply.cursor_start);
                    let end_byte = char_offset_to_byte_offset(&code, reply.cursor_end);
                    let replace_range =
                        snapshot.anchor_before(start_byte)..snapshot.anchor_after(end_byte);
                    Completion {
                        replace_range: replace_range.clone(),
                        new_text: text.clone(),
                        label: CodeLabel::plain(text, None),
                        documentation: None,
                        source: CompletionSource::Custom,
                        icon_path: None,
                        icon_color: None,
                        match_start: Some(replace_range.start),
                        snippet_deduplication_key: None,
                        insert_text_mode: None,
                        confirm: None,
                        group: None,
                    }
                })
                .collect();
            Ok(vec![CompletionResponse {
                completions,
                display_options: CompletionDisplayOptions::default(),
                is_incomplete: false,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<Buffer>,
        position: language::Anchor,
        text: &str,
        trigger_in_words: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        let mut chars = text.chars();
        let Some(char) = chars.next() else {
            return false;
        };
        if chars.next().is_some() {
            return false;
        }

        let buffer = buffer.read(cx);
        let snapshot = buffer.snapshot();
        let classifier = snapshot
            .char_classifier_at(position)
            .scope_context(Some(language::CharScopeContext::Completion));
        if trigger_in_words && classifier.is_word(char) {
            return true;
        }

        // The cell buffer has no language server, so `completion_triggers()` is
        // empty; treat common completion trigger characters as triggers directly.
        KERNEL_COMPLETION_TRIGGERS.contains(&char)
            || buffer.completion_triggers().contains(text)
    }

    fn resolve_completions(
        &self,
        buffer: Entity<Buffer>,
        completion_indices: Vec<usize>,
        completions: Rc<RefCell<Box<[Completion]>>>,
        cx: &mut Context<Editor>,
    ) -> Task<anyhow::Result<bool>> {
        let Some(notebook_editor) = self.notebook_editor.upgrade() else {
            return Task::ready(Ok(false));
        };
        let Some(&completion_index) = completion_indices.first() else {
            return Task::ready(Ok(false));
        };

        let buffer_snapshot = buffer.read(cx).snapshot();
        let (code, cursor_pos) = {
            let completion = completions.borrow()[completion_index].clone();
            // The text up to the end of the replacement range (e.g. `pd.`) plus the
            // completion's own text yields the full expression to inspect (e.g.
            // `pd.read_csv`).
            let prefix_offset = completion.replace_range.end.to_offset(&buffer_snapshot);
            let prefix = buffer_snapshot
                .text_for_range(0..prefix_offset)
                .collect::<String>();
            let code = format!("{prefix}{}", completion.new_text);
            let cursor_pos = code.chars().count();
            (code, cursor_pos)
        };

        let request = InspectRequest {
            code,
            cursor_pos,
            detail_level: Some(0),
        };
        let message: JupyterMessage = request.into();
        let (tx, rx) = oneshot::channel::<InspectReply>();

        let send_result = notebook_editor.update(cx, |editor, cx| {
            editor.request_inspect(message, tx, cx)
        });
        if send_result.is_err() {
            return Task::ready(Ok(false));
        }

        let inspect_task = cx.background_spawn(async move {
            let reply = match rx.await {
                Ok(reply) => reply,
                Err(_) => return None,
            };
            if !reply.found {
                return None;
            }
            let documentation = reply
                .data
                .content
                .iter()
                .find_map(|media| match media {
                    MediaType::Plain(text) => Some(text.clone()),
                    _ => None,
                })
                .unwrap_or_default()
                .trim()
                .to_string();
            (!documentation.is_empty()).then_some(documentation)
        });

        cx.spawn(async move |_editor, _cx| {
            let Some(documentation) = inspect_task.await else {
                return Ok(false);
            };
            completions.borrow_mut()[completion_index].documentation = Some(
                CompletionDocumentation::MultiLinePlainText(documentation.into()),
            );
            Ok(true)
        })
    }
}

const KERNEL_COMPLETION_TRIGGERS: [char; 7] = ['.', ':', '(', '[', ',', '=', ' '];

fn byte_offset_to_char_offset(code: &str, byte_offset: usize) -> usize {
    code[..byte_offset].chars().count()
}

fn char_offset_to_byte_offset(code: &str, char_offset: usize) -> usize {
    code.char_indices()
        .nth(char_offset)
        .map(|(i, _)| i)
        .unwrap_or(code.len())
}

impl EventEmitter<CellEvent> for CodeCell {}

pub(super) enum CellSource {
    /// Crate a new empty cell
    None,
    /// Backed by an existing notebook cell
    Existing {
        execution_count: Option<i32>,
        outputs: Vec<Output>,
    },
}

impl CellSource {
    fn into_outputs(self) -> (Option<i32>, Vec<Output>) {
        match self {
            CellSource::Existing {
                execution_count,
                outputs,
            } => (execution_count, outputs),
            CellSource::None => Default::default(),
        }
    }
}

impl CodeCell {
    pub(super) fn new(
        cell_source: CellSource,
        id: CellId,
        metadata: CellMetadata,
        source: String,
        notebook_language: Shared<Task<Option<Arc<Language>>>>,
        notebook_editor: WeakEntity<NotebookEditor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = cx.new(|cx| Buffer::local(source.clone(), cx));
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
            editor.set_text(source.clone(), window, cx);
            editor.set_show_gutter(false, cx);
            editor.set_use_modal_editing(true);
            editor.set_completion_provider(Some(Rc::new(KernelCompletionProvider::new(
                notebook_editor.clone(),
            ))));
            editor
        });

        let language_task = cx.spawn_in(window, async move |_this, cx| {
            let language = notebook_language.await;
            buffer.update(cx, |buffer, cx| {
                buffer.set_language(language.clone(), cx);
            });
        });

        let (execution_count, outputs) = cell_source.into_outputs();
        let outputs_collapsed = metadata
            .jupyter
            .as_ref()
            .and_then(|jupyter| jupyter.outputs_hidden)
            .unwrap_or(false);

        Self {
            id,
            metadata,
            execution_count,
            source,
            editor,
            outputs,
            selected: false,
            cell_position: None,
            execution_start_time: None,
            execution_duration: None,
            is_executing: false,
            outputs_collapsed,
            _language_task: language_task,
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

    pub fn to_nbformat_cell(&self, cx: &App) -> nbformat::v4::Cell {
        let source = self.current_source(cx);
        let source_lines: Vec<String> = source.lines().map(|l| format!("{}\n", l)).collect();

        let outputs = self.outputs_to_nbformat(cx);

        let mut metadata = self.metadata.clone();
        self.write_outputs_collapsed_to_metadata(&mut metadata);

        nbformat::v4::Cell::Code {
            id: self.id.clone(),
            metadata,
            execution_count: self.execution_count,
            source: source_lines,
            outputs,
        }
    }

    /// Persists the in-memory collapsed state of the cell's outputs to the cell's
    /// notebook metadata (`jupyter.outputs_hidden`), preserving any other fields
    /// such as `jupyter.source_hidden`.
    fn write_outputs_collapsed_to_metadata(&self, metadata: &mut CellMetadata) {
        match metadata.jupyter.as_mut() {
            Some(jupyter) => {
                jupyter.outputs_hidden = self.outputs_collapsed.then_some(true);
            }
            None if self.outputs_collapsed => {
                metadata.jupyter = Some(nbformat::v4::JupyterCellMetadata {
                    source_hidden: None,
                    outputs_hidden: Some(true),
                    additional: Default::default(),
                });
            }
            None => {}
        }
    }

    pub fn toggle_outputs_collapsed(&mut self, cx: &mut Context<Self>) {
        self.outputs_collapsed = !self.outputs_collapsed;
        cx.notify();
    }

    pub fn outputs_collapsed(&self) -> bool {
        self.outputs_collapsed
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
        &self.source
    }

    fn controls(&self, _window: &mut Window, cx: &mut Context<Self>) -> Vec<CellControl> {
        let run_control_type = if self.is_executing {
            CellControlType::StopCell
        } else if self.has_outputs() {
            CellControlType::RerunCell
        } else {
            CellControlType::RunCell
        };

        let mut controls = vec![
            CellControl::new(run_control_type.id(), run_control_type).on_click(cx.listener(
                move |this, _, window, cx| {
                    if this.is_executing {
                        window.dispatch_action(Box::new(InterruptKernel), cx);
                    } else {
                        this.run(window, cx);
                    }
                },
            )),
        ];

        if self.has_outputs() {
            let collapse_control_type = if self.outputs_collapsed {
                CellControlType::ExpandCell
            } else {
                CellControlType::CollapseCell
            };
            controls.push(
                CellControl::new(collapse_control_type.id(), collapse_control_type).on_click(
                    cx.listener(|this, _, _window, cx| this.toggle_outputs_collapsed(cx)),
                ),
            );
        }

        controls
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
        let controls = self.controls(window, cx);

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
            .when(!controls.is_empty(), |this| {
                this.child(
                    div()
                        .absolute()
                        .top(px(CODE_BLOCK_INSET - 2.0))
                        .left_0()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .w(px(GUTTER_WIDTH))
                        .bg(cx.theme().colors().tab_bar_background)
                        .children(controls.into_iter().map(|control| control.button))
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

impl Render for CodeCell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let output_max_height = ReplSettings::get_global(cx).output_max_height_lines;
        let output_max_height = if output_max_height > 0 {
            Some(window.line_height() * output_max_height as f32)
        } else {
            None
        };
        let output_max_width =
            plain::max_width_for_columns(ReplSettings::get_global(cx).max_columns, window, cx);
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
                                        .child(
                                            div()
                                                .id((
                                                    ElementId::from(self.id.to_string()),
                                                    "output-scroll",
                                                ))
                                                .w_full()
                                                .when_some(output_max_width, |div, max_width| {
                                                    div.max_w(max_width).overflow_x_scroll()
                                                })
                                                .when_some(output_max_height, |div, max_height| {
                                                    div.max_h(max_height).overflow_y_scroll()
                                                })
                                                .when(
                                                    self.outputs_collapsed && self.has_outputs(),
                                                    |this| {
                                                        this.child(
                                                            div()
                                                                .text_xs()
                                                                .text_color(
                                                                    cx.theme().colors().text_muted,
                                                                )
                                                                .child(
                                                                    "Output hidden — expand to view",
                                                                ),
                                                        )
                                                    },
                                                )
                                                .when(
                                                    !self.outputs_collapsed,
                                                    |this| {
                                                        this.children(self.outputs.iter().map(
                                                            |output| {
                                                                div().children(
                                                                    output.content(window, cx),
                                                                )
                                                            },
                                                        ))
                                                    },
                                                ),
                                        ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;

    const RICH_NOTEBOOK: &str = r##"{
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3",
                "custom_kernelspec_key": { "nested": true }
            },
            "language_info": { "name": "python", "custom_language_key": 42 },
            "custom_metadata_key": "preserved"
        },
        "nbformat": 4,
        "nbformat_minor": 5,
        "cells": [
            {
                "cell_type": "code",
                "id": "cell-one",
                "execution_count": 3,
                "metadata": {
                    "collapsed": false,
                    "custom_cell_key": { "x": 1 },
                    "jupyter": {
                        "outputs_hidden": true,
                        "source_hidden": false,
                        "custom_jupyter_key": "kept"
                    }
                },
                "outputs": [
                    { "output_type": "stream", "name": "stdout", "text": "hello\n" }
                ],
                "source": ["print('hi')"]
            },
            {
                "cell_type": "markdown",
                "id": "cell-two",
                "metadata": { "custom_md": true },
                "source": ["# Title"]
            },
            {
                "cell_type": "raw",
                "id": "cell-three",
                "metadata": {},
                "source": ["raw text"]
            }
        ]
}"##;

    /// Parsing a notebook with rich metadata and unknown ("additional") fields
    /// must not drop those fields, so that saving the notebook later does not
    /// corrupt or discard data produced by other Jupyter clients.
    #[test]
    fn test_rich_notebook_round_trip_preserves_metadata_and_unknown_fields() {
        let nbformat::Notebook::V4(notebook) = nbformat::parse_notebook(RICH_NOTEBOOK).unwrap()
        else {
            panic!("expected a V4 notebook");
        };

        assert_eq!(notebook.nbformat, 4);
        assert_eq!(notebook.cells.len(), 3);

        // Notebook-level custom metadata is preserved.
        assert_eq!(
            notebook.metadata.additional["custom_metadata_key"],
            serde_json::json!("preserved")
        );
        assert_eq!(
            notebook.metadata.kernelspec.as_ref().unwrap().additional["custom_kernelspec_key"],
            serde_json::json!({ "nested": true })
        );
        assert_eq!(
            notebook.metadata.language_info.as_ref().unwrap().additional["custom_language_key"],
            serde_json::json!(42)
        );

        // Round-tripping through serialization keeps those fields.
        let value = serde_json::to_value(&notebook).unwrap();
        assert_eq!(value["metadata"]["custom_metadata_key"], "preserved");
        assert_eq!(
            value["metadata"]["kernelspec"]["custom_kernelspec_key"],
            serde_json::json!({ "nested": true })
        );

        // Cell metadata and Jupyter-specific metadata survive.
        let nbformat::v4::Cell::Code {
            metadata,
            outputs,
            execution_count,
            ..
        } = &notebook.cells[0]
        else {
            panic!("expected a code cell");
        };
        assert_eq!(execution_count, &Some(3));
        assert_eq!(
            metadata.additional["custom_cell_key"],
            serde_json::json!({"x": 1})
        );
        let jupyter = metadata.jupyter.as_ref().unwrap();
        assert_eq!(jupyter.outputs_hidden, Some(true));
        assert_eq!(jupyter.source_hidden, Some(false));
        assert_eq!(jupyter.additional["custom_jupyter_key"], "kept");

        let cell_value = serde_json::to_value(&notebook.cells[0]).unwrap();
        assert_eq!(
            cell_value["metadata"]["custom_cell_key"],
            serde_json::json!({ "x": 1 })
        );
        assert_eq!(cell_value["metadata"]["jupyter"]["outputs_hidden"], true);

        // Stream outputs parse to stream outputs with the correct text.
        match &outputs[0] {
            nbformat::v4::Output::Stream { name, text, .. } => {
                assert_eq!(name, "stdout");
                assert_eq!(text.0, "hello\n");
            }
            other => panic!("expected a stream output, got: {other:?}"),
        }
    }

    /// Malformed notebook JSON should produce a parse error rather than a panic.
    #[test]
    fn test_malformed_notebook_errors() {
        let malformed = r#"{ "nbformat": 4, "cells": [ { "cell_type": "bogus" } ] }"#;
        assert!(nbformat::parse_notebook(malformed).is_err());

        let not_json = "this is not json";
        assert!(nbformat::parse_notebook(not_json).is_err());
    }

    fn code_cell_metadata(outputs_hidden: Option<bool>) -> CellMetadata {
        serde_json::from_str(&format!(
            r#"{{ "jupyter": {{ "source_hidden": false, "outputs_hidden": {} }} }}"#,
            outputs_hidden
                .map(|hidden| hidden.to_string())
                .unwrap_or_else(|| "null".to_string())
        ))
        .expect("metadata should parse")
    }

    /// A code cell with `jupyter.outputs_hidden` set in its metadata starts out
    /// collapsed, and toggling the collapsed state round-trips back into the
    /// cell metadata (preserving unrelated fields) when serialized.
    #[gpui::test]
    async fn test_outputs_collapsed_persists_to_metadata(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let cell = cx.add_window(|window, cx| {
            CodeCell::new(
                CellSource::Existing {
                    execution_count: Some(1),
                    outputs: vec![],
                },
                CellId::new("cell-one").unwrap(),
                code_cell_metadata(Some(true)),
                "print('hi')".to_string(),
                Task::ready(None).shared(),
                WeakEntity::new_invalid(),
                window,
                cx,
            )
        });

        // Loaded from `outputs_hidden: true` in metadata.
        cell.read_with(cx, |cell, _| {
            assert!(cell.outputs_collapsed(), "expected to load collapsed");
        })
        .unwrap();

        // Toggling to expanded clears `outputs_hidden` but keeps `source_hidden`.
        cell.update(cx, |cell, _window, cx| cell.toggle_outputs_collapsed(cx))
            .unwrap();
        cell.read_with(cx, |cell, cx| {
            assert!(!cell.outputs_collapsed());
            let nbformat::v4::Cell::Code { metadata, .. } = cell.to_nbformat_cell(cx) else {
                panic!("expected a code cell");
            };
            let jupyter = metadata.jupyter.as_ref().unwrap();
            assert_eq!(jupyter.outputs_hidden, None);
            assert_eq!(jupyter.source_hidden, Some(false));
        })
        .unwrap();

        // Toggling back to collapsed persists `outputs_hidden: true`.
        cell.update(cx, |cell, _window, cx| cell.toggle_outputs_collapsed(cx))
            .unwrap();
        cell.read_with(cx, |cell, cx| {
            let nbformat::v4::Cell::Code { metadata, .. } = cell.to_nbformat_cell(cx) else {
                panic!("expected a code cell");
            };
            let jupyter = metadata.jupyter.as_ref().unwrap();
            assert_eq!(jupyter.outputs_hidden, Some(true));
            assert_eq!(jupyter.source_hidden, Some(false));
        })
        .unwrap();
    }
}
