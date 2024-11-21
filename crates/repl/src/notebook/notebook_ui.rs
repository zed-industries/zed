#![allow(unused, dead_code)]
use std::future::Future;
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use client::proto::ViewId;
use collections::HashMap;
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::future::Shared;
use futures::FutureExt;
use gpui::{
    actions, list, prelude::*, AnyElement, AppContext, EventEmitter, FocusHandle, FocusableView,
    ListScrollEvent, ListState, Model, Point, Task, View,
};
use language::{Language, LanguageRegistry};
use project::{Project, ProjectEntryId, ProjectPath};
use ui::{prelude::*, Tooltip};
use workspace::item::{ItemEvent, TabContentParams};
use workspace::searchable::SearchableItemHandle;
use workspace::{Item, ItemHandle, ProjectItem, ToolbarItemLocation};
use workspace::{ToolbarItemEvent, ToolbarItemView};

use super::{Cell, CellPosition, RenderableCell};

use nbformat::v4::CellId;
use nbformat::v4::Metadata as NotebookMetadata;

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
    if cx.has_flag::<NotebookFeatureFlag>() || std::env::var("LOCAL_NOTEBOOK_DEV").is_ok() {
        workspace::register_project_item::<NotebookEditor>(cx);
    }

    cx.observe_flag::<NotebookFeatureFlag, _>({
        move |is_enabled, cx| {
            if is_enabled {
                workspace::register_project_item::<NotebookEditor>(cx);
            } else {
                // todo: there is no way to unregister a project item, so if the feature flag
                // gets turned off they need to restart Zed.
            }
        }
    })
    .detach();
}

pub struct NotebookEditor {
    languages: Arc<LanguageRegistry>,
    project: Model<Project>,

    focus_handle: FocusHandle,
    notebook_item: Model<NotebookItem>,

    remote_id: Option<ViewId>,
    cell_list: ListState,

    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
}

impl NotebookEditor {
    pub fn new(
        project: Model<Project>,
        notebook_item: Model<NotebookItem>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let languages = project.read(cx).languages().clone();
        let language_name = notebook_item.read(cx).language_name();

        let notebook_language = notebook_item.read(cx).notebook_language();
        let notebook_language = cx.spawn(|_, _| notebook_language).shared();

        let mut cell_order = vec![]; // Vec<CellId>
        let mut cell_map = HashMap::default(); // HashMap<CellId, Cell>

        for (index, cell) in notebook_item
            .read(cx)
            .notebook
            .clone()
            .cells
            .iter()
            .enumerate()
        {
            let cell_id = cell.id();
            cell_order.push(cell_id.clone());
            cell_map.insert(
                cell_id.clone(),
                Cell::load(cell, &languages, notebook_language.clone(), cx),
            );
        }

        let view = cx.view().downgrade();
        let cell_count = cell_order.len();

        let this = cx.view();
        let cell_list = ListState::new(
            cell_count,
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, cx| {
                view.upgrade()
                    .and_then(|notebook_handle| {
                        notebook_handle.update(cx, |notebook, cx| {
                            notebook
                                .cell_order
                                .get(ix)
                                .and_then(|cell_id| notebook.cell_map.get(cell_id))
                                .map(|cell| notebook.render_cell(ix, cell, cx).into_any_element())
                        })
                    })
                    .unwrap_or_else(|| div().into_any())
            },
        );

        Self {
            project,
            languages: languages.clone(),
            focus_handle,
            notebook_item,
            remote_id: None,
            cell_list,
            selected_cell_index: 0,
            cell_order: cell_order.clone(),
            cell_map: cell_map.clone(),
        }
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

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).is_dirty(cx)
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, cx: &mut ViewContext<Self>) {
        for cell in self.cell_map.values() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, _cx| {
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
        // let previous_index = self.selected_cell_index;
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

    fn jump_to_cell(&mut self, index: usize, _cx: &mut ViewContext<Self>) {
        self.cell_list.scroll_to_reveal_item(index);
    }

    fn button_group(cx: &ViewContext<Self>) -> Div {
        v_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
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
        _cx: &ViewContext<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE).into())
    }

    fn render_notebook_controls(&self, cx: &ViewContext<Self>) -> impl IntoElement {
        let has_outputs = self.has_outputs(cx);

        v_flex()
            .max_w(px(CONTROL_SIZE + 4.0))
            .items_center()
            .gap(DynamicSpacing::Base16.rems(cx))
            .justify_between()
            .flex_none()
            .h_full()
            .py(DynamicSpacing::Base12.px(cx))
            .child(
                v_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
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
                    .gap(DynamicSpacing::Base08.rems(cx))
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

    fn cell_position(&self, index: usize) -> CellPosition {
        match index {
            0 => CellPosition::First,
            index if index == self.cell_count() - 1 => CellPosition::Last,
            _ => CellPosition::Middle,
        }
    }

    fn render_cell(
        &self,
        index: usize,
        cell: &Cell,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let cell_position = self.cell_position(index);

        let is_selected = index == self.selected_cell_index;

        match cell {
            Cell::Code(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Markdown(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Raw(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
        }
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
            .size_full()
            .overflow_hidden()
            .px(DynamicSpacing::Base12.px(cx))
            .gap(DynamicSpacing::Base12.px(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .child(
                v_flex()
                    .id("notebook-cells")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .child(list(self.cell_list.clone()).size_full()),
            )
            .child(self.render_notebook_controls(cx))
    }
}

impl FocusableView for NotebookEditor {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Intended to be a NotebookBuffer
pub struct NotebookItem {
    path: PathBuf,
    project_path: ProjectPath,
    languages: Arc<LanguageRegistry>,
    // Raw notebook data
    notebook: nbformat::v4::Notebook,
    // Store our version of the notebook in memory (cell_order, cell_map)
    id: ProjectEntryId,
}

impl project::Item for NotebookItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();
        let fs = project.read(cx).fs().clone();
        let languages = project.read(cx).languages().clone();

        if path.path.extension().unwrap_or_default() == "ipynb" {
            Some(cx.spawn(|mut cx| async move {
                let abs_path = project
                    .read_with(&cx, |project, cx| project.absolute_path(&path, cx))?
                    .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                // todo: watch for changes to the file
                let file_content = fs.load(&abs_path.as_path()).await?;
                let notebook = nbformat::parse_notebook(&file_content);

                let notebook = match notebook {
                    Ok(nbformat::Notebook::V4(notebook)) => notebook,
                    // 4.1 - 4.4 are converted to 4.5
                    Ok(nbformat::Notebook::Legacy(legacy_notebook)) => {
                        // todo!(): Decide if we want to mutate the notebook by including Cell IDs
                        // and any other conversions
                        let notebook = nbformat::upgrade_legacy_notebook(legacy_notebook)?;
                        notebook
                    }
                    // Bad notebooks and notebooks v4.0 and below are not supported
                    Err(e) => {
                        anyhow::bail!("Failed to parse notebook: {:?}", e);
                    }
                };

                let id = project
                    .update(&mut cx, |project, cx| project.entry_for_path(&path, cx))?
                    .context("Entry not found")?
                    .id;

                cx.new_model(|_| NotebookItem {
                    path: abs_path,
                    project_path: path,
                    languages,
                    notebook,
                    id,
                })
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        Some(self.id)
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

impl NotebookItem {
    pub fn language_name(&self) -> Option<String> {
        self.notebook
            .metadata
            .language_info
            .as_ref()
            .map(|l| l.name.clone())
            .or(self
                .notebook
                .metadata
                .kernelspec
                .as_ref()
                .and_then(|spec| spec.language.clone()))
    }

    pub fn notebook_language(&self) -> impl Future<Output = Option<Arc<Language>>> {
        let language_name = self.language_name();
        let languages = self.languages.clone();

        async move {
            if let Some(language_name) = language_name {
                languages.language_for_name(&language_name).await.ok()
            } else {
                None
            }
        }
    }
}

impl EventEmitter<()> for NotebookEditor {}

// pub struct NotebookControls {
//     pane_focused: bool,
//     active_item: Option<Box<dyn ItemHandle>>,
//     // subscription: Option<Subscription>,
// }

// impl NotebookControls {
//     pub fn new() -> Self {
//         Self {
//             pane_focused: false,
//             active_item: Default::default(),
//             // subscription: Default::default(),
//         }
//     }
// }

// impl EventEmitter<ToolbarItemEvent> for NotebookControls {}

// impl Render for NotebookControls {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
//         div().child("notebook controls")
//     }
// }

// impl ToolbarItemView for NotebookControls {
//     fn set_active_pane_item(
//         &mut self,
//         active_pane_item: Option<&dyn workspace::ItemHandle>,
//         cx: &mut ViewContext<Self>,
//     ) -> workspace::ToolbarItemLocation {
//         cx.notify();
//         self.active_item = None;

//         let Some(item) = active_pane_item else {
//             return ToolbarItemLocation::Hidden;
//         };

//         ToolbarItemLocation::PrimaryLeft
//     }

//     fn pane_focus_update(&mut self, pane_focused: bool, _: &mut ViewContext<Self>) {
//         self.pane_focused = pane_focused;
//     }
// }

impl Item for NotebookEditor {
    type Event = ();

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<gpui::View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| Self::new(self.project.clone(), self.notebook_item.clone(), cx)))
    }

    fn for_each_project_item(
        &self,
        cx: &AppContext,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::Item),
    ) {
        f(self.notebook_item.entity_id(), self.notebook_item.read(cx))
    }

    fn is_singleton(&self, _cx: &AppContext) -> bool {
        true
    }

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let path = &self.notebook_item.read(cx).path;
        let title = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();
        Label::new(title)
            .single_line()
            .color(params.text_color())
            .italic(params.preview)
            .into_any_element()
    }

    fn tab_icon(&self, _cx: &ui::WindowContext) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    // TODO
    fn pixel_position_of_cursor(&self, _: &AppContext) -> Option<Point<Pixels>> {
        None
    }

    // TODO
    fn as_searchable(&self, _: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {
        // TODO
    }

    // TODO
    fn can_save(&self, _cx: &AppContext) -> bool {
        false
    }
    // TODO
    fn save(
        &mut self,
        _format: bool,
        _project: Model<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save() must be implemented if can_save() returns true")
    }

    // TODO
    fn save_as(
        &mut self,
        _project: Model<Project>,
        _path: ProjectPath,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save_as() must be implemented if can_save() returns true")
    }
    // TODO
    fn reload(
        &mut self,
        _project: Model<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("reload() must be implemented if can_save() returns true")
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        // self.is_dirty(cx) TODO
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
    where
        Self: Sized,
    {
        Self::new(project, item, cx)
    }
}
