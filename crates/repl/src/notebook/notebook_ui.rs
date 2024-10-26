#![allow(unused)]
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use client::proto::ViewId;
use collections::HashMap;
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::{future::Shared, FutureExt};
use gpui::{
    actions, prelude::*, AppContext, EventEmitter, FocusHandle, FocusableView, Model, Task, View,
    WeakView,
};
use language::{Language, LanguageRegistry};
use project::{Project, ProjectEntryId, ProjectPath};
use ui::{prelude::*, Tooltip};
use util::ResultExt;
use uuid::Uuid;
use workspace::{FollowableItem, Item, ItemHandle, Pane, ProjectItem, SerializableItem, Workspace};

use super::{
    deserialize_notebook,
    static_sample::{complex_example, no_cells_example, simple_example},
    Cell, CellId, DeserializedCell, DeserializedMetadata, DeserializedNotebook, RenderableCell,
    DEFAULT_NOTEBOOK_FORMAT, DEFAULT_NOTEBOOK_FORMAT_MINOR,
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
        AddCodeBlock,
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
    if cx.has_flag::<NotebookFeatureFlag>() {
        eprintln!("Registering NotebookEditor...");
        workspace::register_project_item::<NotebookEditor>(cx);
    }

    cx.observe_flag::<NotebookFeatureFlag, _>({
        move |is_enabled, cx| {
            if is_enabled {
                workspace::register_project_item::<NotebookEditor>(cx);
            } else {
                // todo: there is no way to unregister a project item, so if the feature flag
                // gets turned off they need to restart Zed.
                //
            }
        }
    })
    .detach();
}

pub struct NotebookEditor {
    languages: Arc<LanguageRegistry>,

    focus_handle: FocusHandle,
    project: Model<Project>,
    remote_id: Option<ViewId>,

    metadata: DeserializedMetadata,
    nbformat: i32,
    nbformat_minor: i32,
    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
}

impl NotebookEditor {
    pub fn new(
        project: Model<Project>,
        notebook: DeserializedNotebook,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let languages = project.read(cx).languages().clone();

        let metadata = notebook.metadata;
        let nbformat = notebook.nbformat;
        let nbformat_minor = notebook.nbformat_minor;

        let language_name = metadata
            .language_info
            .as_ref()
            .map(|l| l.name.clone())
            .or(metadata
                .kernelspec
                .as_ref()
                .and_then(|spec| spec.language.clone()));

        let notebook_language = if let Some(language_name) = language_name {
            cx.spawn(|_, _| {
                let languages = languages.clone();
                async move { languages.language_for_name(&language_name).await.ok() }
            })
            .shared()
        } else {
            Task::ready(None).shared()
        };

        let languages = project.read(cx).languages().clone();
        let notebook_language = cx
            .spawn(|_, _| {
                // todo: pull from notebook metadata
                const TODO: &'static str = "Python";
                let languages = languages.clone();
                async move { languages.language_for_name(TODO).await.ok() }
            })
            .shared();

        let mut cell_order = vec![];
        let mut cell_map = HashMap::default();

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
            cell_map.insert(
                cell_id,
                Cell::load(cell, &languages, notebook_language.clone(), cx),
            );
        }

        Self {
            languages: languages.clone(),
            focus_handle,
            project,
            remote_id: None,
            selected_cell_index: 0,
            metadata,
            nbformat,
            nbformat_minor,
            cell_order,
            cell_map,
        }
    }

    // pub fn load(workspace: View<Workspace>, cx: &mut WindowContext) -> Task<Result<View<Self>>> {
    //     let weak_workspace = workspace.downgrade();
    //     let workspace = workspace.read(cx);
    //     let project = workspace.project().to_owned();

    //     cx.spawn(|mut cx| async move {
    //         let notebook = deserialize_notebook(simple_example())?;

    //         cx.new_view(|cx| Self::new(weak_workspace.clone(), project, notebook, cx))
    //     })
    // }

    // pub fn open(
    //     workspace_view: View<Workspace>,
    //     cx: &mut WindowContext,
    // ) -> Task<Result<View<Self>>> {
    //     let weak_workspace = workspace_view.downgrade();
    //     let workspace = workspace_view.read(cx);
    //     let project = workspace.project().to_owned();
    //     let pane = workspace.active_pane().clone();
    //     let notebook = Self::load(workspace_view, cx);

    //     cx.spawn(|mut cx| async move {
    //         let notebook = notebook.await?;
    //         pane.update(&mut cx, |pane, cx| {
    //             pane.add_item(Box::new(notebook.clone()), true, true, None, cx);
    //         })?;

    //         anyhow::Ok(notebook)
    //     })
    // }

    fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cell_order
            .iter()
            .filter_map(|id| self.cell_map.get(id))
    }

    fn has_outputs(&self, cx: &ViewContext<Self>) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, cx: &mut ViewContext<Self>) {
        for cell in self.cell_map.values() {
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

    fn cell_count(&self) -> usize {
        self.cell_map.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_cell_index
    }

    pub fn set_selected_index(
        &mut self,
        index: usize,
        jump_to_index: bool,
        cx: &mut ViewContext<Self>,
    ) {
        let previous_index = self.selected_cell_index;
        self.selected_cell_index = index;
        let current_index = self.selected_cell_index;

        // in the future we may have some `on_cell_change` event that we want to fire here

        if jump_to_index {
            self.jump_to_cell(current_index, cx);
        }
    }

    pub fn select_next(&mut self, _: &menu::SelectNext, cx: &mut ViewContext<Self>) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == count - 1 {
                count - 1
            } else {
                index + 1
            };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    pub fn select_previous(&mut self, _: &menu::SelectPrev, cx: &mut ViewContext<Self>) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, cx);
            cx.notify();
        }
    }

    pub fn select_first(&mut self, _: &menu::SelectFirst, cx: &mut ViewContext<Self>) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(0, true, cx);
            cx.notify();
        }
    }

    pub fn select_last(&mut self, _: &menu::SelectLast, cx: &mut ViewContext<Self>) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, cx);
            cx.notify();
        }
    }

    fn jump_to_cell(&mut self, index: usize, cx: &mut ViewContext<Self>) {
        // Logic to jump the view to make the selected cell visible
        println!("Scrolling to cell at index {}", index);
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

    fn render_cells(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .id("notebook-cells")
            .flex_1()
            .size_full()
            .overflow_y_scroll()
            .children(self.cells().enumerate().map(|(index, cell)| {
                let is_selected = index == self.selected_cell_index;
                match cell {
                    Cell::Code(cell) => {
                        cell.update(cx, |cell, cx| {
                            cell.set_selected(is_selected);
                        });
                        cell.clone().into_any_element()
                    }
                    Cell::Markdown(cell) => {
                        cell.update(cx, |cell, cx| {
                            cell.set_selected(is_selected);
                        });
                        cell.clone().into_any_element()
                    }
                    Cell::Raw(cell) => {
                        cell.update(cx, |cell, cx| {
                            cell.set_selected(is_selected);
                        });
                        cell.clone().into_any_element()
                    }
                }
            }))
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
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .flex()
            .items_start()
            // .size_full()
            // todo: figure out the flex height issue
            .h(px(800.))
            .w(px(1000.))
            .overflow_hidden()
            .p(large_gap)
            .gap(large_gap)
            .bg(cx.theme().colors().tab_bar_background)
            .child(self.render_notebook_controls(cx))
            .child(self.render_cells(cx))
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

pub struct NotebookItem {
    path: PathBuf,
    project_path: ProjectPath,
}

impl project::Item for NotebookItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        eprintln!("Trying to open notebook: {:?}", path);

        if path.path.extension().unwrap_or_default() == "ipynb" {
            Some(cx.spawn(|mut cx| async move {
                let abs_path = project
                    .read_with(&cx, |project, cx| project.absolute_path(&path, cx))?
                    .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                cx.new_model(|_| NotebookItem {
                    path: abs_path,
                    project_path: path,
                })
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

fn deserialize_notebook_from_path(path: &Path) -> Result<DeserializedNotebook> {
    let file_content = std::fs::read_to_string(path)?;
    deserialize_notebook(&file_content)
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

// TODO: Implement this to allow us to persist to the database, etc:
// impl SerializableItem for NotebookEditor {}

impl ProjectItem for NotebookEditor {
    type Item = NotebookItem;

    fn for_project_item(
        project: Model<Project>,
        item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    // todo... no workspace (?)
    where
        Self: Sized,
    {
        let path = item.read(cx).path.clone();
        let notebook =
            deserialize_notebook_from_path(&path).expect("Failed to deserialize notebook");
        // todo... no workspace (?)
        Self::new(project, notebook, cx)
    }
}
