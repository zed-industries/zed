#![allow(unused)]
use anyhow::Result;
use client::proto::ViewId;
use collections::HashMap;
use gpui::{
    actions, prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView, Model, Task, View,
    WeakView,
};
use project::Project;
use ui::{prelude::*, Tooltip};
use util::ResultExt;
use uuid::Uuid;
use workspace::{FollowableItem, Item, ItemHandle, Pane, Workspace};

use super::{
    deserialize_notebook,
    static_sample::{no_cells_example, simple_example},
    Cell, CellId, DeserializedCell, DeserializedMetadata, DEFAULT_NOTEBOOK_FORMAT,
    DEFAULT_NOTEBOOK_FORMAT_MINOR,
};

actions!(
    notebook,
    [
        OpenNotebook,
        RunAll,
        ClearOutputs,
        MoveCellUp,
        MoveCellDown,
        AddMarkdownBlock,
        AddCodeBlock
    ]
);

pub(crate) const MAX_TEXT_BLOCK_WIDTH: f32 = 9999.0;
pub(crate) const SMALL_SPACING_SIZE: f32 = 8.0;
pub(crate) const MEDIUM_SPACING_SIZE: f32 = 12.0;
pub(crate) const LARGE_SPACING_SIZE: f32 = 16.0;
pub(crate) const GUTTER_WIDTH: f32 = 19.0;
pub(crate) const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;
pub(crate) const CONTROL_SIZE: f32 = 20.0;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|_, _: &OpenNotebook, cx| {
            let workspace = cx.view().clone();
            cx.window_context()
                .defer(move |cx| NotebookEditor::open(workspace, cx).detach_and_log_err(cx));
        });
    })
    .detach();
}

pub struct NotebookEditor {
    focus_handle: FocusHandle,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    remote_id: Option<ViewId>,

    metadata: DeserializedMetadata,
    nbformat: i32,
    nbformat_minor: i32,
    selected_cell: usize,
    cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
}

impl NotebookEditor {
    pub fn new(
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let mut metadata: DeserializedMetadata = Default::default();
        let mut nbformat = DEFAULT_NOTEBOOK_FORMAT;
        let mut nbformat_minor = DEFAULT_NOTEBOOK_FORMAT_MINOR;
        let mut cell_order = vec![];
        let mut cell_map = HashMap::default();

        let deserialized_notebook = deserialize_notebook(simple_example());

        if let Ok(notebook) = deserialized_notebook {
            metadata = notebook.metadata;
            nbformat = notebook.nbformat;
            nbformat_minor = notebook.nbformat_minor;
            for (index, cell) in notebook.cells.into_iter().enumerate() {
                let id = match &cell {
                    DeserializedCell::Markdown { id, .. }
                    | DeserializedCell::Code { id, .. }
                    | DeserializedCell::Raw { id, .. } => {
                        id.clone().unwrap_or_else(|| Uuid::new_v4().to_string())
                    }
                };
                let cell_id = CellId::from(id.clone());
                cell_order.push(cell_id.clone());
                cell_map.insert(cell_id, Cell::load(cell, cx));
            }
        } else {
            println!(
                "Error deserializing notebook: {:?}",
                deserialized_notebook.err()
            );
        }

        Self {
            focus_handle,
            workspace,
            project,
            remote_id: None,
            selected_cell: 0,
            metadata,
            nbformat,
            nbformat_minor,
            cell_order,
            cell_map,
        }
    }

    pub fn load(workspace: View<Workspace>, cx: &mut WindowContext) -> Task<Result<View<Self>>> {
        let weak_workspace = workspace.downgrade();
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();

        cx.spawn(|mut cx| async move {
            cx.new_view(|cx| Self::new(weak_workspace.clone(), project, cx))
        })
    }

    pub fn open(
        workspace_view: View<Workspace>,
        cx: &mut WindowContext,
    ) -> Task<Result<View<Self>>> {
        let weak_workspace = workspace_view.downgrade();
        let workspace = workspace_view.read(cx);
        let project = workspace.project().to_owned();
        let pane = workspace.active_pane().clone();
        let notebook = Self::load(workspace_view, cx);

        cx.spawn(|mut cx| async move {
            let notebook = notebook.await?;
            pane.update(&mut cx, |pane, cx| {
                pane.add_item(Box::new(notebook.clone()), true, true, None, cx);
            })?;

            anyhow::Ok(notebook)
        })
    }

    fn cells(&self) -> Vec<Cell> {
        self.cell_order
            .iter()
            .filter_map(|id| self.cell_map.get(id).cloned())
            .collect()
    }

    fn has_outputs(&self, cx: &ViewContext<Self>) -> bool {
        self.cells().iter().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, cx: &mut ViewContext<Self>) {
        for cell in self.cells() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, cx| {
                    cell.clear_outputs();
                });
            }
        }
    }

    fn run_cells(&mut self, cx: &mut ViewContext<Self>) {
        println!("Cells would all run here, if that was implemented!");
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _cx: &mut ViewContext<Self>) {
        println!("Open notebook triggered");
    }

    fn move_cell_up(&mut self, cx: &mut ViewContext<Self>) {
        println!("Move cell up triggered");
    }

    fn move_cell_down(&mut self, cx: &mut ViewContext<Self>) {
        println!("Move cell down triggered");
    }

    fn add_markdown_block(&mut self, cx: &mut ViewContext<Self>) {
        println!("Add markdown block triggered");
    }

    fn add_code_block(&mut self, cx: &mut ViewContext<Self>) {
        println!("Add code block triggered");
    }

    fn button_group(cx: &ViewContext<Self>) -> Div {
        v_flex()
            .gap(Spacing::Small.rems(cx))
            .items_center()
            .w(px(CONTROL_SIZE + 4.0))
            .overflow_hidden()
            .rounded(px(5.))
            .bg(cx.theme().colors().title_bar_background)
            .p_px()
            .border_1()
            .border_color(cx.theme().colors().border)
    }

    fn render_notebook_control(
        id: impl Into<SharedString>,
        icon: IconName,
        cx: &ViewContext<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE).into())
    }

    fn render_notebook_controls(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let has_outputs = self.has_outputs(cx);

        v_flex()
            .max_w(px(CONTROL_SIZE + 4.0))
            .items_center()
            .gap(Spacing::XXLarge.rems(cx))
            .justify_between()
            .flex_none()
            .h_full()
            .child(
                v_flex()
                    .gap(Spacing::Large.rems(cx))
                    .child(
                        Self::button_group(cx)
                            .child(
                                Self::render_notebook_control("run-all-cells", IconName::Play, cx)
                                    .tooltip(move |cx| {
                                        Tooltip::for_action("Execute all cells", &RunAll, cx)
                                    })
                                    .on_click(|_, cx| {
                                        cx.dispatch_action(Box::new(RunAll));
                                    }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "clear-all-outputs",
                                    IconName::ListX,
                                    cx,
                                )
                                .disabled(!has_outputs)
                                .tooltip(move |cx| {
                                    Tooltip::for_action("Clear all outputs", &ClearOutputs, cx)
                                })
                                .on_click(|_, cx| {
                                    cx.dispatch_action(Box::new(ClearOutputs));
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-up",
                                    IconName::ArrowUp,
                                    cx,
                                )
                                .tooltip(move |cx| {
                                    Tooltip::for_action("Move cell up", &MoveCellUp, cx)
                                })
                                .on_click(|_, cx| {
                                    cx.dispatch_action(Box::new(MoveCellUp));
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-down",
                                    IconName::ArrowDown,
                                    cx,
                                )
                                .tooltip(move |cx| {
                                    Tooltip::for_action("Move cell down", &MoveCellDown, cx)
                                })
                                .on_click(|_, cx| {
                                    cx.dispatch_action(Box::new(MoveCellDown));
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(
                                Self::render_notebook_control(
                                    "new-markdown-cell",
                                    IconName::Plus,
                                    cx,
                                )
                                .tooltip(move |cx| {
                                    Tooltip::for_action("Add markdown block", &AddMarkdownBlock, cx)
                                })
                                .on_click(|_, cx| {
                                    cx.dispatch_action(Box::new(AddMarkdownBlock));
                                }),
                            )
                            .child(
                                Self::render_notebook_control("new-code-cell", IconName::Code, cx)
                                    .tooltip(move |cx| {
                                        Tooltip::for_action("Add code block", &AddCodeBlock, cx)
                                    })
                                    .on_click(|_, cx| {
                                        cx.dispatch_action(Box::new(AddCodeBlock));
                                    }),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .gap(Spacing::Large.rems(cx))
                    .items_center()
                    .child(Self::render_notebook_control(
                        "more-menu",
                        IconName::Ellipsis,
                        cx,
                    ))
                    .child(
                        Self::button_group(cx)
                            .child(IconButton::new("repl", IconName::ReplNeutral)),
                    ),
            )
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let large_gap = Spacing::XLarge.px(cx);
        let gap = Spacing::Large.px(cx);

        div()
            .key_context("notebook")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, &OpenNotebook, cx| this.open_notebook(&OpenNotebook, cx)))
            .on_action(cx.listener(|this, &ClearOutputs, cx| this.clear_outputs(cx)))
            .on_action(cx.listener(|this, &RunAll, cx| this.run_cells(cx)))
            .on_action(cx.listener(|this, &MoveCellUp, cx| this.move_cell_up(cx)))
            .on_action(cx.listener(|this, &MoveCellDown, cx| this.move_cell_down(cx)))
            .on_action(cx.listener(|this, &AddMarkdownBlock, cx| this.add_markdown_block(cx)))
            .on_action(cx.listener(|this, &AddCodeBlock, cx| this.add_code_block(cx)))
            .flex()
            .items_start()
            // .size_full()
            // i'm lazy to figure out this flex height issue right now
            .h(px(800.))
            .w(px(1000.))
            .overflow_hidden()
            .p(large_gap)
            .gap(large_gap)
            .bg(cx.theme().colors().tab_bar_background)
            .child(self.render_notebook_controls(cx))
            .child(
                v_flex()
                    .id("notebook-cells")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .gap_6()
                    .children(self.cells().into_iter().map(|cell| match cell {
                        Cell::Code(view) => view.clone().into_any_element(),
                        Cell::Markdown(view) => view.clone().into_any_element(),
                        Cell::Raw(view) => view.clone().into_any_element(),
                    })),
            )
            .child(
                div()
                    .w(px(GUTTER_WIDTH))
                    .h_full()
                    .flex_none()
                    .overflow_hidden()
                    .child("scrollbar"),
            )
    }
}

impl FocusableView for NotebookEditor {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for NotebookEditor {}

impl Item for NotebookEditor {
    type Event = ();

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        // TODO: We want file name
        Some("Notebook".into())
    }

    fn tab_icon(&self, _cx: &ui::WindowContext) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }
}
