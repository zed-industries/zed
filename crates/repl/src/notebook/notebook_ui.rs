#![allow(unused, dead_code)]
use std::future::Future;
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use client::proto::ViewId;
use collections::HashMap;
use feature_flags::{FeatureFlagAppExt as _, NotebookFeatureFlag};
use futures::FutureExt;
use futures::future::Shared;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, ListScrollEvent, ListState,
    Point, Task, actions, list, prelude::*,
};
use language::{Language, LanguageRegistry};
use project::{Project, ProjectEntryId, ProjectPath};
use ui::{Tooltip, prelude::*};
use workspace::item::{ItemEvent, SaveOptions, TabContentParams};
use workspace::searchable::SearchableItemHandle;
use workspace::{Item, ItemHandle, Pane, ProjectItem, ToolbarItemLocation};
use workspace::{ToolbarItemEvent, ToolbarItemView};

use super::{Cell, CellPosition, RenderableCell};

use nbformat::v4::CellId;
use nbformat::v4::Metadata as NotebookMetadata;

actions!(
    notebook,
    [
        /// Opens a Jupyter notebook file.
        OpenNotebook,
        /// Runs all cells in the notebook.
        RunAll,
        /// Clears all cell outputs.
        ClearOutputs,
        /// Moves the current cell up.
        MoveCellUp,
        /// Moves the current cell down.
        MoveCellDown,
        /// Adds a new markdown cell.
        AddMarkdownBlock,
        /// Adds a new code cell.
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

pub fn init(cx: &mut App) {
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
    project: Entity<Project>,

    focus_handle: FocusHandle,
    notebook_item: Entity<NotebookItem>,

    remote_id: Option<ViewId>,
    cell_list: ListState,

    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
}

impl NotebookEditor {
    pub fn new(
        project: Entity<Project>,
        notebook_item: Entity<NotebookItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let languages = project.read(cx).languages().clone();
        let language_name = notebook_item.read(cx).language_name();

        let notebook_language = notebook_item.read(cx).notebook_language();
        let notebook_language = cx
            .spawn_in(window, async move |_, _| notebook_language.await)
            .shared();

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
                Cell::load(cell, &languages, notebook_language.clone(), window, cx),
            );
        }

        let notebook_handle = cx.entity().downgrade();
        let cell_count = cell_order.len();

        let this = cx.entity();
        let cell_list = ListState::new(
            cell_count,
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, window, cx| {
                notebook_handle
                    .upgrade()
                    .and_then(|notebook_handle| {
                        notebook_handle.update(cx, |notebook, cx| {
                            notebook
                                .cell_order
                                .get(ix)
                                .and_then(|cell_id| notebook.cell_map.get(cell_id))
                                .map(|cell| {
                                    notebook
                                        .render_cell(ix, cell, window, cx)
                                        .into_any_element()
                                })
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

    fn has_outputs(&self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for cell in self.cell_map.values() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, _cx| {
                    cell.clear_outputs();
                });
            }
        }
    }

    fn run_cells(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Cells would all run here, if that was implemented!");
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("Open notebook triggered");
    }

    fn move_cell_up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell up triggered");
    }

    fn move_cell_down(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell down triggered");
    }

    fn add_markdown_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Add markdown block triggered");
    }

    fn add_code_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
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
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // let previous_index = self.selected_cell_index;
        self.selected_cell_index = index;
        let current_index = self.selected_cell_index;

        // in the future we may have some `on_cell_change` event that we want to fire here

        if jump_to_index {
            self.jump_to_cell(current_index, window, cx);
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == count - 1 {
                count - 1
            } else {
                index + 1
            };
            self.set_selected_index(ix, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(0, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_last(
        &mut self,
        _: &menu::SelectLast,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, window, cx);
            cx.notify();
        }
    }

    fn jump_to_cell(&mut self, index: usize, _window: &mut Window, _cx: &mut Context<Self>) {
        self.cell_list.scroll_to_reveal_item(index);
    }

    fn button_group(window: &mut Window, cx: &mut Context<Self>) -> Div {
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
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE).into())
    }

    fn render_notebook_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_outputs = self.has_outputs(window, cx);

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
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "run-all-cells",
                                    IconName::PlayOutlined,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Execute all cells", &RunAll, window, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(RunAll), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "clear-all-outputs",
                                    IconName::ListX,
                                    window,
                                    cx,
                                )
                                .disabled(!has_outputs)
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action(
                                        "Clear all outputs",
                                        &ClearOutputs,
                                        window,
                                        cx,
                                    )
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(ClearOutputs), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-up",
                                    IconName::ArrowUp,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Move cell up", &MoveCellUp, window, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellUp), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-down",
                                    IconName::ArrowDown,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Move cell down", &MoveCellDown, window, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellDown), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "new-markdown-cell",
                                    IconName::Plus,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action(
                                        "Add markdown block",
                                        &AddMarkdownBlock,
                                        window,
                                        cx,
                                    )
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddMarkdownBlock), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "new-code-cell",
                                    IconName::Code,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Add code block", &AddCodeBlock, window, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddCodeBlock), cx);
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
                        window,
                        cx,
                    ))
                    .child(
                        Self::button_group(window, cx)
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
        window: &mut Window,
        cx: &mut Context<Self>,
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("notebook")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, &OpenNotebook, window, cx| {
                this.open_notebook(&OpenNotebook, window, cx)
            }))
            .on_action(
                cx.listener(|this, &ClearOutputs, window, cx| this.clear_outputs(window, cx)),
            )
            .on_action(cx.listener(|this, &RunAll, window, cx| this.run_cells(window, cx)))
            .on_action(cx.listener(|this, &MoveCellUp, window, cx| this.move_cell_up(window, cx)))
            .on_action(
                cx.listener(|this, &MoveCellDown, window, cx| this.move_cell_down(window, cx)),
            )
            .on_action(cx.listener(|this, &AddMarkdownBlock, window, cx| {
                this.add_markdown_block(window, cx)
            }))
            .on_action(
                cx.listener(|this, &AddCodeBlock, window, cx| this.add_code_block(window, cx)),
            )
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
            .child(self.render_notebook_controls(window, cx))
    }
}

impl Focusable for NotebookEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
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

impl project::ProjectItem for NotebookItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        let path = path.clone();
        let project = project.clone();
        let fs = project.read(cx).fs().clone();
        let languages = project.read(cx).languages().clone();

        if path.path.extension().unwrap_or_default() == "ipynb" {
            Some(cx.spawn(async move |cx| {
                let abs_path = project
                    .read_with(cx, |project, cx| project.absolute_path(&path, cx))?
                    .with_context(|| format!("finding the absolute path of {path:?}"))?;

                // todo: watch for changes to the file
                let file_content = fs.load(&abs_path.as_path()).await?;
                let notebook = nbformat::parse_notebook(&file_content);

                let notebook = match notebook {
                    Ok(nbformat::Notebook::V4(notebook)) => notebook,
                    // 4.1 - 4.4 are converted to 4.5
                    Ok(nbformat::Notebook::Legacy(legacy_notebook)) => {
                        // TODO: Decide if we want to mutate the notebook by including Cell IDs
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
                    .update(cx, |project, cx| project.entry_for_path(&path, cx))?
                    .context("Entry not found")?
                    .id;

                cx.new(|_| NotebookItem {
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

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        Some(self.id)
    }

    fn project_path(&self, _: &App) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }

    fn is_dirty(&self) -> bool {
        false
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

    pub fn notebook_language(&self) -> impl Future<Output = Option<Arc<Language>>> + use<> {
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
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         div().child("notebook controls")
//     }
// }

// impl ToolbarItemView for NotebookControls {
//     fn set_active_pane_item(
//         &mut self,
//         active_pane_item: Option<&dyn workspace::ItemHandle>,
//         window: &mut Window, cx: &mut Context<Self>,
//     ) -> workspace::ToolbarItemLocation {
//         cx.notify();
//         self.active_item = None;

//         let Some(item) = active_pane_item else {
//             return ToolbarItemLocation::Hidden;
//         };

//         ToolbarItemLocation::PrimaryLeft
//     }

//     fn pane_focus_update(&mut self, pane_focused: bool, _window: &mut Window, _cx: &mut Context<Self>) {
//         self.pane_focused = pane_focused;
//     }
// }

impl Item for NotebookEditor {
    type Event = ();

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<Self>>
    where
        Self: Sized,
    {
        Some(cx.new(|cx| Self::new(self.project.clone(), self.notebook_item.clone(), window, cx)))
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.notebook_item.entity_id(), self.notebook_item.read(cx))
    }

    fn is_singleton(&self, _cx: &App) -> bool {
        true
    }

    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or(0), cx))
            .single_line()
            .color(params.text_color())
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let path = &self.notebook_item.read(cx).path;
        let title = path
            .file_name()
            .unwrap_or_else(|| path.as_os_str())
            .to_string_lossy()
            .to_string();
        title.into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    fn show_toolbar(&self) -> bool {
        false
    }

    // TODO
    fn pixel_position_of_cursor(&self, _: &App) -> Option<Point<Pixels>> {
        None
    }

    // TODO
    fn as_searchable(&self, _: &Entity<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        None
    }

    fn set_nav_history(
        &mut self,
        _: workspace::ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        // TODO
    }

    // TODO
    fn can_save(&self, _cx: &App) -> bool {
        false
    }
    // TODO
    fn save(
        &mut self,
        _options: SaveOptions,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save() must be implemented if can_save() returns true")
    }

    // TODO
    fn save_as(
        &mut self,
        _project: Entity<Project>,
        _path: ProjectPath,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("save_as() must be implemented if can_save() returns true")
    }
    // TODO
    fn reload(
        &mut self,
        _project: Entity<Project>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        unimplemented!("reload() must be implemented if can_save() returns true")
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).is_dirty(cx)
            } else {
                false
            }
        })
    }
}

// TODO: Implement this to allow us to persist to the database, etc:
// impl SerializableItem for NotebookEditor {}

impl ProjectItem for NotebookEditor {
    type Item = NotebookItem;

    fn for_project_item(
        project: Entity<Project>,
        _: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self::new(project, item, window, cx)
    }
}
