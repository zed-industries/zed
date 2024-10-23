#![allow(unused)]
use anyhow::Result;
use client::proto::ViewId;
use collections::HashMap;
use gpui::{
    actions, prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView, Model, Task, View,
    WeakView,
};
use project::Project;
use ui::prelude::*;
use util::ResultExt;
use workspace::{FollowableItem, Item, ItemHandle, Pane, Workspace};

use super::{
    deserialize_notebook,
    static_sample::{no_cells_example, simple_example},
    Cell, CellId, DeserializedMetadata, Notebook, DEFAULT_NOTEBOOK_FORMAT,
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

    pub fn load(workspace: View<Workspace>, cx: &mut WindowContext) -> Task<Result<View<Self>>> {
        let weak_workspace = workspace.downgrade();
        let workspace = workspace.read(cx);
        let project = workspace.project().to_owned();

        cx.spawn(|mut cx| async move {
            cx.new_view(|cx| Self::new(weak_workspace.clone(), project, cx))
        })
    }

    pub fn new(
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let this = cx.view().downgrade();
        let focus_handle = cx.focus_handle();
        // let notebook = Notebook::default();

        let mut metadata: DeserializedMetadata = Default::default();
        let mut nbformat = DEFAULT_NOTEBOOK_FORMAT;
        let mut nbformat_minor = DEFAULT_NOTEBOOK_FORMAT_MINOR;
        let mut cell_order = vec![];
        let mut cell_map = HashMap::default();

        // let deserialized_notebook = deserialize_notebook(no_cells_example());
        let deserialized_notebook = deserialize_notebook(simple_example());

        if let Ok(notebook) = deserialized_notebook {
            metadata = notebook.metadata;
            nbformat = notebook.nbformat;
            nbformat_minor = notebook.nbformat_minor;
            for cell in notebook.cells {
                let id = match &cell {
                    super::DeserializedCell::Markdown { id, .. } => id,
                    super::DeserializedCell::Code { id, .. } => id,
                    super::DeserializedCell::Raw { id, .. } => id,
                };
                let id = CellId::from(id.clone());
                cell_order.push(id.clone());
                cell_map.insert(id, Cell::load(cell, cx));
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

    fn cells(&self) -> Vec<Cell> {
        self.cell_map.values().cloned().collect()
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _cx: &mut ViewContext<Self>) {
        println!("Open notebook triggered");
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

    fn render_control(
        id: impl Into<SharedString>,
        icon: IconName,
        cx: &ViewContext<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE).into())
    }

    fn render_controls(cx: &ViewContext<Self>) -> impl IntoElement {
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
                            .child(Self::render_control("run-all-cells", IconName::Play, cx))
                            .child(Self::render_control(
                                "clear-all-outputs",
                                IconName::Close,
                                cx,
                            )),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(
                                Self::render_control("move-cell-up", IconName::ChevronUp, cx)
                                    .disabled(true),
                            )
                            .child(Self::render_control(
                                "move-cell-down",
                                IconName::ChevronDown,
                                cx,
                            )),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(Self::render_control(
                                "new-markdown-cell",
                                IconName::Plus,
                                cx,
                            ))
                            .child(Self::render_control("new-code-cell", IconName::Code, cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap(Spacing::Large.rems(cx))
                    .items_center()
                    .child(Self::render_control("more-menu", IconName::Ellipsis, cx))
                    .child(
                        Self::button_group(cx)
                            .child(IconButton::new("repl", IconName::ReplNeutral)),
                    ),
            )
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // cell bar
        // scrollbar
        // settings

        let large_gap = Spacing::XLarge.px(cx);
        let gap = Spacing::Large.px(cx);

        div()
            .key_context("notebook")
            .on_action(cx.listener(Self::open_notebook))
            .track_focus(&self.focus_handle)
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
            .child(Self::render_controls(cx))
            .child(
                // notebook cells
                v_flex()
                    .id("notebook-cells")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .gap_6()
                    .children(self.cells().iter().map(|cell| match cell {
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
                    .child("cell bar")
                    .child("scrollbar"),
            )

        // .child("settings")
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
