#![allow(unused)]
use anyhow::Result;
use client::proto::ViewId;
use gpui::{
    actions, prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView, Model, Task, View,
    WeakView,
};
use project::Project;
use ui::prelude::*;
use util::ResultExt;
use workspace::{FollowableItem, Item, ItemHandle, Pane, Workspace};

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

const MAX_TEXT_BLOCK_WIDTH: f32 = 9999.0;
const SMALL_SPACING_SIZE: f32 = 8.0;
const MEDIUM_SPACING_SIZE: f32 = 12.0;
const LARGE_SPACING_SIZE: f32 = 16.0;
const GUTTER_WIDTH: f32 = 19.0;
const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;
const CONTROL_SIZE: f32 = 20.0;

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|_, _: &OpenNotebook, cx| {
            let workspace = cx.view().clone();
            cx.window_context()
                .defer(move |cx| Notebook::open(workspace, cx).detach_and_log_err(cx));
        });
    })
    .detach();
}

pub struct Notebook {
    focus_handle: FocusHandle,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    remote_id: Option<ViewId>,
    selected_cell: usize,
}

impl Notebook {
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
        Self {
            focus_handle,
            workspace,
            project,
            remote_id: None,
            selected_cell: 0,
        }
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

impl Render for Notebook {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // cell bar
        // scrollbar
        // settings

        let large_gap = Spacing::XLarge.px(cx);
        let gap = Spacing::Large.px(cx);

        div()
            // .debug_below()
            .key_context("notebook")
            .on_action(cx.listener(Self::open_notebook))
            .track_focus(&self.focus_handle)
            .flex()
            .items_start()
            .size_full()
            .overflow_y_hidden()
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
                    .children(
                        sample_cells()
                            .into_iter()
                            .enumerate()
                            .map(|(ix, cell)| cell.selected(self.selected_cell == ix)),
                    ),
            )
            .child(div().flex_none().child("cell bar").child("scrollbar"))

        // .child("settings")
    }
}

impl FocusableView for Notebook {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for Notebook {}

impl Item for Notebook {
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

#[derive(Debug, Default, Clone, PartialEq)]
enum NotebookCellKind {
    Code,
    #[default]
    Markdown,
}

#[derive(IntoElement)]
struct Cell {
    cell_type: NotebookCellKind,
    control: Option<IconButton>,
    source: Vec<String>,
    selected: bool,
}

impl RenderOnce for Cell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let source = self.source.clone();
        let cell_type = self.cell_type.clone();
        let is_selected = self.selected.clone();
        let mut selected_bg = cx.theme().colors().icon_accent;
        selected_bg.fade_out(0.9);

        h_flex()
            .w_full()
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .child(
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
                                    .when(is_selected, |this| {
                                        this.bg(cx.theme().colors().icon_accent)
                                    })
                                    .when(!is_selected, |this| this.bg(cx.theme().colors().border)),
                            ),
                    )
                    .children(self.control.map(|action| {
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
                            .when(is_selected, |this| this.bg(selected_bg))
                            .when(!is_selected, |this| {
                                this.bg(cx.theme().colors().tab_bar_background)
                            })
                            .child(action)
                    })),
            )
            .when(cell_type == NotebookCellKind::Markdown, |this| {
                this.child(
                    v_flex()
                        .w_full()
                        .max_w(px(MAX_TEXT_BLOCK_WIDTH))
                        .px(px(CODE_BLOCK_INSET))
                        .children(source.clone()),
                )
            })
            .when(cell_type == NotebookCellKind::Code, |this| {
                this.child(
                    v_flex()
                        .size_full()
                        .flex_1()
                        .p_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().editor_background)
                        .font_buffer(cx)
                        .text_size(TextSize::Editor.rems(cx))
                        .children(source),
                )
            })
    }
}

impl Cell {
    pub fn markdown(source: Vec<String>) -> Self {
        Self {
            control: None,
            cell_type: NotebookCellKind::Markdown,
            source,
            selected: false,
        }
    }

    pub fn code(source: Vec<String>) -> Self {
        Self {
            control: None,
            cell_type: NotebookCellKind::Code,
            source,
            selected: false,
        }
    }

    pub fn kind(mut self, kind: NotebookCellKind) -> Self {
        self.cell_type = kind;
        self
    }

    pub fn control(mut self, control: IconButton) -> Self {
        self.control = Some(control);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

// impl FollowableItem for Notebook {}

enum NotebookCell {
    Code(NotebookCodeCell),
    Markdown(NotebookMarkdownCell),
}

#[derive(IntoElement)]
struct NotebookCodeCell {}

impl NotebookCodeCell {
    fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for NotebookCodeCell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .w_full()
            .h(px(280.))
            .items_start()
            .gap(Spacing::Large.rems(cx))
            .child(
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
                                    .bg(cx.theme().colors().border),
                            ),
                    )
                    .child(
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
                            .child(IconButton::new("run", IconName::Play)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .size_full()
                    .flex_1()
                    .p_3()
                    .rounded_lg()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .font_buffer(cx)
                    .text_size(TextSize::Editor.rems(cx))
                    .child("Code cell"),
            )
    }
}

#[derive(IntoElement)]
struct NotebookMarkdownCell {}

impl NotebookMarkdownCell {
    fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for NotebookMarkdownCell {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .w_full()
            .items_start()
            .gap(Spacing::Large.rems(cx))
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
                            .bg(cx.theme().colors().border),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .max_w(px(MAX_TEXT_BLOCK_WIDTH))
                    .px(px(CODE_BLOCK_INSET))
                    .child(Headline::new("Population Data from CSV").size(HeadlineSize::Large))
                    .child("This notebook reads sample population data from `data/atlantis.csv` and plots it using matplotlib. Edit `data/atlantis.csv` and re-run this cell to see how the plots change!"),
            )
    }
}

fn sample_cells() -> Vec<Cell> {
    vec![
        Cell::markdown(vec![
            "## Table of Contents".to_string(),
            "1.\tIntroduction".to_string(),
            "2.\tOverview of Python Data Visualization Tools".to_string(),
            "3.\tIntroduction to Matplotlib".to_string(),
            "4.\tImport Matplotlib".to_string(),
            "5.\tDisplaying Plots in Matplotlib".to_string(),
            "6.\tMatplotlib Object Hierarchy".to_string(),
            "7.\tMatplotlib interfaces".to_string(),
        ]),
        Cell::markdown(vec![
            "## 1. Introduction".to_string(),
            "When we want to convey some information to others, there are several ways to do so. The process of conveying the information with the help of plots and graphics is called **Data Visualization**. The plots and graphics take numerical data as input and display output in the form of charts, figures and tables. It helps to analyze and visualize the data clearly and make concrete decisions. It makes complex data more accessible and understandable. The goal of data visualization is to communicate information in a clear and efficient manner.".to_string(),
            "In this project, I shed some light on **Matplotlib**, which is the basic data visualization tool of Python programming language. Python has different data visualization tools available which are suitable for different purposes. First of all, I will list these data visualization tools and then I will discuss Matplotlib.".to_string()
        ])
    ]
}
