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

actions!(notebook, [OpenNotebook, RunAll, ClearOutputs, MoveCellUp, MoveCellDown, AddMarkdownBlock, AddCodeBlock]);

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

    fn notebook_control(
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
                            .child(Self::notebook_control("run-all-cells", IconName::Play, cx))
                            .child(Self::notebook_control(
                                "clear-all-outputs",
                                IconName::Close,
                                cx,
                            )),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(
                                Self::notebook_control("move-cell-up", IconName::ChevronUp, cx)
                                    .disabled(true),
                            )
                            .child(Self::notebook_control(
                                "move-cell-down",
                                IconName::ChevronDown,
                                cx,
                            )),
                    )
                    .child(
                        Self::button_group(cx)
                            .child(Self::notebook_control(
                                "new-markdown-cell",
                                IconName::Plus,
                                cx,
                            ))
                            .child(Self::notebook_control("new-code-cell", IconName::Code, cx)),
                    ),
            )
            .child(
                v_flex()
                    .gap(Spacing::Large.rems(cx))
                    .items_center()
                    .child(Self::notebook_control("more-menu", IconName::Ellipsis, cx))
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
                    .gap(large_gap)
                    .child(NotebookMarkdownCell::new())
                    .child(NotebookCodeCell::new())
                    .child(NotebookMarkdownCell::new())
                    .child(NotebookCodeCell::new()),
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
