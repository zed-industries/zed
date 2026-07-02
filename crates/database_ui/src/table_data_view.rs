use std::ops::Range;
use std::sync::Arc;

use database_client::{
    DatabaseClient, Filter, FilterOp, RowsPage, SelectSpec, Sort, SortDirection, TableRef,
    TableStructure,
};
use gpui::{
    Anchor, AnyElement, App, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    Task, WeakEntity, Window, actions,
};
use settings::Settings as _;
use ui::{
    AbsoluteLength, ColumnWidthConfig, ContextMenu, PopoverMenu, ResizableColumnsState, Table,
    TableInteractionState, TableResizeBehavior, Tooltip, prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::{Workspace, item::Item};

use crate::DatabaseSettings;

actions!(
    database,
    [
        /// Loads the next page of rows in the table view.
        NextPage,
        /// Loads the previous page of rows in the table view.
        PrevPage,
        /// Switches between the data and structure views of a table.
        ToggleStructure,
        /// Reloads the current table data (and structure if loaded).
        RefreshData,
    ]
);

/// The default column width for the resizable data grid.
const COLUMN_WIDTH: f32 = 180.;

/// The short symbol shown in the UI for each filter operator.
fn filter_op_label(op: FilterOp) -> &'static str {
    match op {
        FilterOp::Eq => "=",
        FilterOp::NotEq => "≠",
        FilterOp::Gt => ">",
        FilterOp::Lt => "<",
        FilterOp::Contains => "contains",
        FilterOp::IsNull => "is null",
    }
}

/// Every filter operator, in the order they appear in the operator dropdown.
fn all_filter_ops() -> [FilterOp; 6] {
    [
        FilterOp::Eq,
        FilterOp::NotEq,
        FilterOp::Gt,
        FilterOp::Lt,
        FilterOp::Contains,
        FilterOp::IsNull,
    ]
}

/// Which of the two tabs of a table view is currently shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewMode {
    Data,
    Structure,
}

/// Tracks the in-flight state of the current data load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadState {
    Idle,
    Loading,
    Error(String),
}

/// A workspace tab showing the rows and structure of a single database table.
///
/// The data grid supports server-side sorting and offset pagination through the
/// [`SelectSpec`] handed to [`DatabaseClient::fetch_rows`]; the structure tab is
/// fetched lazily on first display and cached until an explicit refresh.
pub struct TableDataView {
    focus_handle: FocusHandle,
    client: Arc<dyn DatabaseClient>,
    table: TableRef,
    mode: ViewMode,
    spec: SelectSpec,
    page: Option<RowsPage>,
    structure: Option<TableStructure>,
    load_state: LoadState,
    interaction: Entity<TableInteractionState>,
    /// Recreated whenever the rendered column set changes so the grid keeps the
    /// right number of resize handles.
    column_widths: Option<Entity<ResizableColumnsState>>,
    /// Whether the inline filter-builder row is expanded under the header.
    filter_builder_open: bool,
    /// The column selected in the filter builder, if any.
    draft_column: Option<String>,
    /// The operator selected in the filter builder.
    draft_op: FilterOp,
    /// The value input for the filter builder (ignored for `IsNull`).
    draft_value: Entity<InputField>,
    _load_task: Option<Task<()>>,
}

impl TableDataView {
    pub fn new(
        client: Arc<dyn DatabaseClient>,
        table: TableRef,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let limit = DatabaseSettings::get_global(cx).page_size.max(1) as usize;
        cx.new(|cx| {
            let interaction = cx.new(|cx| TableInteractionState::new(cx));
            let draft_value = cx.new(|cx| InputField::new(window, cx, "Value"));
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                client,
                table,
                mode: ViewMode::Data,
                spec: SelectSpec {
                    limit,
                    ..Default::default()
                },
                page: None,
                structure: None,
                load_state: LoadState::Idle,
                interaction,
                column_widths: None,
                filter_builder_open: false,
                draft_column: None,
                draft_op: FilterOp::Eq,
                draft_value,
                _load_task: None,
            };
            view.reload_data(cx);
            view
        })
    }

    pub fn table(&self) -> &TableRef {
        &self.table
    }

    pub fn spec(&self) -> &SelectSpec {
        &self.spec
    }

    pub fn page(&self) -> Option<&RowsPage> {
        self.page.as_ref()
    }

    pub fn structure(&self) -> Option<&TableStructure> {
        self.structure.as_ref()
    }

    pub fn load_state(&self) -> &LoadState {
        &self.load_state
    }

    pub fn mode(&self) -> ViewMode {
        self.mode
    }

    /// Cycles the sort on `column` (None -> Asc -> Desc -> None), resets the
    /// page offset, and reloads the current page.
    pub fn toggle_sort(&mut self, column: &str, cx: &mut Context<Self>) {
        let next = match &self.spec.sort {
            Some(sort) if sort.column == column => match sort.direction {
                SortDirection::Asc => Some(Sort {
                    column: column.to_string(),
                    direction: SortDirection::Desc,
                }),
                SortDirection::Desc => None,
            },
            _ => Some(Sort {
                column: column.to_string(),
                direction: SortDirection::Asc,
            }),
        };
        self.spec.sort = next;
        self.spec.offset = 0;
        self.reload_data(cx);
    }

    /// Appends `filter` to the active filter set, resets the page offset, and
    /// reloads the current page.
    pub fn add_filter(&mut self, filter: Filter, cx: &mut Context<Self>) {
        self.spec.filters.push(filter);
        self.spec.offset = 0;
        self.reload_data(cx);
    }

    /// Removes the filter at `index`, resets the page offset, and reloads. An
    /// out-of-bounds index is a no-op.
    pub fn remove_filter(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.spec.filters.len() {
            log::debug!(
                "remove_filter: index {index} out of bounds ({} filters)",
                self.spec.filters.len()
            );
            return;
        }
        self.spec.filters.remove(index);
        self.spec.offset = 0;
        self.reload_data(cx);
    }

    /// Advances to the next page when the current page reports more rows.
    pub fn next_page(&mut self, cx: &mut Context<Self>) {
        let has_more = self.page.as_ref().is_some_and(|page| page.has_more);
        if !has_more {
            return;
        }
        self.spec.offset += self.spec.limit;
        self.reload_data(cx);
    }

    /// Moves back one page, clamping the offset at zero. No-op at the first page.
    pub fn prev_page(&mut self, cx: &mut Context<Self>) {
        if self.spec.offset == 0 {
            return;
        }
        self.spec.offset = self.spec.offset.saturating_sub(self.spec.limit);
        self.reload_data(cx);
    }

    /// Switches between the data and structure tabs, fetching the structure the
    /// first time it is shown.
    pub fn toggle_structure(&mut self, cx: &mut Context<Self>) {
        self.mode = match self.mode {
            ViewMode::Data => ViewMode::Structure,
            ViewMode::Structure => ViewMode::Data,
        };
        if self.mode == ViewMode::Structure && self.structure.is_none() {
            self.reload_structure(cx);
        }
        cx.notify();
    }

    /// Re-fetches the current page and, if it was already loaded, the structure.
    fn refresh(&mut self, cx: &mut Context<Self>) {
        self.reload_data(cx);
        if self.structure.is_some() {
            self.reload_structure(cx);
        }
    }

    fn reload_data(&mut self, cx: &mut Context<Self>) {
        self.load_state = LoadState::Loading;
        cx.notify();

        let client = self.client.clone();
        let table = self.table.clone();
        let spec = self.spec.clone();
        let task =
            gpui_tokio::Tokio::spawn_result(
                cx,
                async move { client.fetch_rows(&table, &spec).await },
            );

        self._load_task = Some(cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(page) => {
                        this.set_column_widths(page.columns.len(), cx);
                        this.page = Some(page);
                        this.load_state = LoadState::Idle;
                    }
                    Err(error) => {
                        this.load_state = LoadState::Error(error.to_string());
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    fn reload_structure(&mut self, cx: &mut Context<Self>) {
        let client = self.client.clone();
        let table = self.table.clone();
        let task =
            gpui_tokio::Tokio::spawn_result(
                cx,
                async move { client.table_structure(&table).await },
            );

        self._load_task = Some(cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(structure) => {
                        this.structure = Some(structure);
                        this.load_state = LoadState::Idle;
                    }
                    Err(error) => {
                        this.load_state = LoadState::Error(error.to_string());
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Recreates the resizable-columns state when the number of data columns
    /// changes, so the grid renders the correct number of resize handles.
    fn set_column_widths(&mut self, cols: usize, cx: &mut Context<Self>) {
        if cols == 0 {
            self.column_widths = None;
            return;
        }
        let matches = self
            .column_widths
            .as_ref()
            .is_some_and(|widths| widths.read(cx).cols() == cols);
        if matches {
            return;
        }
        self.column_widths = Some(cx.new(|_cx| {
            ResizableColumnsState::new(
                cols,
                vec![AbsoluteLength::Pixels(px(COLUMN_WIDTH)); cols],
                vec![TableResizeBehavior::Resizable; cols],
            )
        }));
    }

    fn render_data(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let Some(page) = self.page.clone() else {
            return v_flex().into_any_element();
        };
        let Some(widths) = self.column_widths.clone() else {
            return v_flex().into_any_element();
        };

        let headers: Vec<AnyElement> = page
            .columns
            .iter()
            .enumerate()
            .map(|(index, column)| self.render_header(index, column, cx))
            .collect();

        let column_count = page.columns.len();
        let rows = Arc::new(page.rows);

        Table::new(column_count)
            .interactable(&self.interaction)
            .striped()
            .width_config(ColumnWidthConfig::Resizable(widths))
            .header(headers)
            .uniform_list(
                "db-rows",
                rows.len(),
                cx.processor(move |_this, range: Range<usize>, _window, _cx| {
                    range
                        .filter_map(|row_index| {
                            let row = rows.get(row_index)?;
                            let cells: Vec<AnyElement> = (0..column_count)
                                .map(|col| render_cell(row.get(col).and_then(|cell| cell.clone())))
                                .collect();
                            Some(cells)
                        })
                        .collect()
                }),
            )
            .into_any_element()
    }

    fn render_header(&self, index: usize, column: &str, cx: &Context<Self>) -> AnyElement {
        let sorted = self
            .spec
            .sort
            .as_ref()
            .filter(|sort| sort.column == column)
            .map(|sort| sort.direction);
        let indicator = match sorted {
            Some(SortDirection::Asc) => "↑",
            Some(SortDirection::Desc) => "↓",
            None => "↕",
        };
        let tooltip = match sorted {
            Some(SortDirection::Asc) => "Sorted ascending. Click to sort descending",
            Some(SortDirection::Desc) => "Sorted descending. Click to clear sorting",
            None => "Not sorted. Click to sort ascending",
        };
        let column = column.to_string();

        h_flex()
            .justify_between()
            .items_center()
            .w_full()
            .child(Label::new(column.clone()))
            .child(
                Button::new(
                    ElementId::NamedInteger("db-sort".into(), index as u64),
                    indicator,
                )
                .size(ButtonSize::Compact)
                .style(if sorted.is_some() {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .tooltip(Tooltip::text(tooltip))
                .on_click(cx.listener(move |this, _event, _window, cx| {
                    this.toggle_sort(&column, cx);
                })),
            )
            .into_any_element()
    }

    fn render_structure(&self) -> AnyElement {
        let Some(structure) = self.structure.as_ref() else {
            return v_flex()
                .p_4()
                .child(Label::new("Loading structure…").color(Color::Muted))
                .into_any_element();
        };

        let mut table = Table::new(6).striped().header(vec![
            "Name".into_any_element(),
            "Type".into_any_element(),
            "Nullable".into_any_element(),
            "Default".into_any_element(),
            "PK".into_any_element(),
            "FK".into_any_element(),
        ]);

        for column in &structure.columns {
            let foreign_key = structure
                .foreign_keys
                .iter()
                .find(|fk| fk.column == column.name)
                .map(|fk| {
                    format!(
                        "→ {}.{}.{}",
                        fk.references_schema, fk.references_table, fk.references_column
                    )
                })
                .unwrap_or_default();
            table = table.row(vec![
                Label::new(column.name.clone()).into_any_element(),
                Label::new(column.data_type.clone()).into_any_element(),
                Label::new(if column.is_nullable { "YES" } else { "NO" }).into_any_element(),
                Label::new(column.default.clone().unwrap_or_default()).into_any_element(),
                Label::new(if column.is_primary_key { "PK" } else { "" }).into_any_element(),
                Label::new(foreign_key).into_any_element(),
            ]);
        }

        let indexes =
            if structure.indexes.is_empty() {
                None
            } else {
                Some(
                    v_flex()
                        .pt_2()
                        .gap_1()
                        .child(Label::new("Indexes").color(Color::Muted))
                        .children(structure.indexes.iter().enumerate().map(
                            |(index_pos, index)| {
                                let definition: SharedString = index.definition.clone().into();
                                div()
                                    .id(ElementId::NamedInteger(
                                        "db-index".into(),
                                        index_pos as u64,
                                    ))
                                    .w_full()
                                    .whitespace_nowrap()
                                    .text_ellipsis()
                                    .child(Label::new(definition.clone()).size(LabelSize::Small))
                                    .tooltip(move |_, cx| Tooltip::simple(definition.clone(), cx))
                            },
                        )),
                )
            };

        v_flex()
            .p_2()
            .gap_2()
            .child(table.into_any_element())
            .children(indexes)
            .into_any_element()
    }

    fn render_toggle(&self, cx: &Context<Self>) -> AnyElement {
        h_flex()
            .gap_1()
            .child(
                Button::new("db-mode-data", "Data")
                    .toggle_state(self.mode == ViewMode::Data)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if this.mode != ViewMode::Data {
                            this.toggle_structure(cx);
                        }
                    })),
            )
            .child(
                Button::new("db-mode-structure", "Structure")
                    .toggle_state(self.mode == ViewMode::Structure)
                    .on_click(cx.listener(|this, _, _, cx| {
                        if this.mode != ViewMode::Structure {
                            this.toggle_structure(cx);
                        }
                    })),
            )
            .into_any_element()
    }

    /// The column names offered in the filter builder, taken from the current
    /// page's header row (empty until the first page loads).
    fn available_columns(&self) -> Vec<String> {
        self.page
            .as_ref()
            .map(|page| page.columns.clone())
            .unwrap_or_default()
    }

    /// Whether the current draft is complete enough to apply: a column must be
    /// chosen, and non-`IsNull` operators additionally require a value.
    fn draft_apply_enabled(&self, cx: &App) -> bool {
        let has_column = self.draft_column.is_some();
        let needs_value = self.draft_op != FilterOp::IsNull;
        let has_value = !self.draft_value.read(cx).text(cx).trim().is_empty();
        has_column && (!needs_value || has_value)
    }

    /// Commits the current draft as a new filter, closing and clearing the
    /// builder. No-op if the draft is incomplete.
    fn apply_draft_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.draft_apply_enabled(cx) {
            return;
        }
        let Some(column) = self.draft_column.clone() else {
            return;
        };
        let value = if self.draft_op == FilterOp::IsNull {
            String::new()
        } else {
            self.draft_value.read(cx).text(cx)
        };
        self.add_filter(
            Filter {
                column,
                op: self.draft_op,
                value,
            },
            cx,
        );
        self.filter_builder_open = false;
        self.draft_column = None;
        self.draft_op = FilterOp::Eq;
        self.draft_value
            .update(cx, |field, cx| field.clear(window, cx));
    }

    fn render_filter_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let chips = self
            .spec
            .filters
            .iter()
            .enumerate()
            .map(|(index, filter)| self.render_filter_chip(index, filter, cx));

        let mut bar = h_flex()
            .w_full()
            .px_2()
            .py_1()
            .gap_1()
            .flex_wrap()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .children(chips)
            .child(
                Button::new("db-add-filter", "+ Filter")
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(self.filter_builder_open)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.filter_builder_open = !this.filter_builder_open;
                        cx.notify();
                    })),
            );

        if self.filter_builder_open {
            bar = bar.child(self.render_filter_builder(window, cx));
        }

        bar.into_any_element()
    }

    fn render_filter_chip(&self, index: usize, filter: &Filter, cx: &Context<Self>) -> AnyElement {
        let text = if filter.op == FilterOp::IsNull {
            format!("{} {}", filter.column, filter_op_label(filter.op))
        } else {
            format!(
                "{} {} '{}'",
                filter.column,
                filter_op_label(filter.op),
                filter.value
            )
        };

        h_flex()
            .gap_1()
            .px_1p5()
            .py_0p5()
            .rounded_sm()
            .bg(cx.theme().colors().element_background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(Label::new(text).size(LabelSize::Small))
            .child(
                IconButton::new(
                    ElementId::NamedInteger("db-filter-remove".into(), index as u64),
                    IconName::Close,
                )
                .icon_size(IconSize::XSmall)
                .tooltip(Tooltip::text("Remove filter"))
                .on_click(cx.listener(move |this, _, _, cx| this.remove_filter(index, cx))),
            )
            .into_any_element()
    }

    fn render_filter_builder(&self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let columns = self.available_columns();
        let column_label = self
            .draft_column
            .clone()
            .unwrap_or_else(|| "Column".to_string());

        let column_dropdown = PopoverMenu::new("db-filter-column")
            .trigger(
                Button::new("db-filter-column-trigger", column_label).size(ButtonSize::Compact),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let this = cx.weak_entity();
                move |window, cx| {
                    let this = this.clone();
                    let columns = columns.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for column in &columns {
                            let column = column.clone();
                            let this = this.clone();
                            menu = menu.entry(column.clone(), None, move |_, cx| {
                                this.update(cx, |this, cx| {
                                    this.draft_column = Some(column.clone());
                                    cx.notify();
                                })
                                .log_err();
                            });
                        }
                        menu
                    }))
                }
            });

        let op_dropdown = PopoverMenu::new("db-filter-op")
            .trigger(
                Button::new(
                    "db-filter-op-trigger",
                    filter_op_label(self.draft_op).to_string(),
                )
                .size(ButtonSize::Compact),
            )
            .anchor(Anchor::TopLeft)
            .menu({
                let this = cx.weak_entity();
                move |window, cx| {
                    let this = this.clone();
                    Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                        for op in all_filter_ops() {
                            let this = this.clone();
                            menu = menu.entry(filter_op_label(op), None, move |_, cx| {
                                this.update(cx, |this, cx| {
                                    this.draft_op = op;
                                    cx.notify();
                                })
                                .log_err();
                            });
                        }
                        menu
                    }))
                }
            });

        let apply_enabled = self.draft_apply_enabled(cx);
        let show_value = self.draft_op != FilterOp::IsNull;

        h_flex()
            .gap_1()
            .items_center()
            .child(column_dropdown)
            .child(op_dropdown)
            .when(show_value, |this| {
                this.child(div().w_40().child(self.draft_value.clone()))
            })
            .child(
                Button::new("db-filter-apply", "Apply")
                    .size(ButtonSize::Compact)
                    .style(ButtonStyle::Filled)
                    .disabled(!apply_enabled)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.apply_draft_filter(window, cx);
                        cx.notify();
                    })),
            )
            .into_any_element()
    }

    fn render_footer(&self, cx: &Context<Self>) -> AnyElement {
        let (summary, has_more) = match &self.page {
            Some(page) if page.rows.is_empty() => ("No rows".to_string(), false),
            Some(page) => {
                let start = self.spec.offset + 1;
                let end = self.spec.offset + page.rows.len();
                let suffix = if page.has_more { "+" } else { "" };
                (format!("rows {start}–{end}{suffix}"), page.has_more)
            }
            None => (String::new(), false),
        };
        let at_start = self.spec.offset == 0;

        h_flex()
            .w_full()
            .px_2()
            .py_1()
            .justify_between()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(summary)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("db-prev-page", IconName::ChevronLeft)
                            .icon_size(IconSize::Small)
                            .disabled(at_start)
                            .tooltip(Tooltip::text("Previous page"))
                            .on_click(cx.listener(|this, _, _, cx| this.prev_page(cx))),
                    )
                    .child(
                        IconButton::new("db-refresh", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh"))
                            .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
                    )
                    .child(
                        IconButton::new("db-next-page", IconName::ChevronRight)
                            .icon_size(IconSize::Small)
                            .disabled(!has_more)
                            .tooltip(Tooltip::text("Next page"))
                            .on_click(cx.listener(|this, _, _, cx| this.next_page(cx))),
                    ),
            )
            .into_any_element()
    }

    fn render_error(&self, message: &str, cx: &Context<Self>) -> AnyElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_2()
            .child(Label::new(message.to_string()).color(Color::Error))
            .child(
                Button::new("db-retry", "Retry")
                    .on_click(cx.listener(|this, _, _, cx| this.refresh(cx))),
            )
            .into_any_element()
    }
}

impl Render for TableDataView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let body = match (&self.load_state, self.mode) {
            (LoadState::Error(message), _) => self.render_error(&message.clone(), cx),
            (_, ViewMode::Structure) => self.render_structure(),
            (_, ViewMode::Data) => self.render_data(cx),
        };
        let in_data =
            self.mode == ViewMode::Data && !matches!(self.load_state, LoadState::Error(_));
        let filter_bar = in_data.then(|| self.render_filter_bar(window, cx));

        v_flex()
            .key_context("TableDataView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &NextPage, _, cx| this.next_page(cx)))
            .on_action(cx.listener(|this, _: &PrevPage, _, cx| this.prev_page(cx)))
            .on_action(cx.listener(|this, _: &ToggleStructure, _, cx| this.toggle_structure(cx)))
            .on_action(cx.listener(|this, _: &RefreshData, _, cx| this.refresh(cx)))
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .w_full()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new(format!(
                        "{}.{}",
                        self.table.schema, self.table.name
                    )))
                    .child(self.render_toggle(cx)),
            )
            .children(filter_bar)
            .child(v_flex().flex_1().size_full().overflow_hidden().child(body))
            .when(in_data, |this| this.child(self.render_footer(cx)))
    }
}

impl Focusable for TableDataView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for TableDataView {}

impl Item for TableDataView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileTree))
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        format!("{}.{}", self.table.schema, self.table.name).into()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<SharedString> {
        Some(
            format!(
                "{}.{}.{}",
                self.table.database, self.table.schema, self.table.name
            )
            .into(),
        )
    }
}

/// Renders a single data cell, showing a muted italic `NULL` for absent values.
fn render_cell(value: Option<String>) -> AnyElement {
    match value {
        Some(value) => div()
            .w_full()
            .whitespace_nowrap()
            .text_ellipsis()
            .child(value)
            .into_any_element(),
        None => div()
            .w_full()
            .child(Label::new("NULL").color(Color::Muted).italic())
            .into_any_element(),
    }
}

/// Opens (or activates an existing) table data tab in the workspace's active
/// pane, de-duplicating by [`TableRef`].
pub fn open_table_tab(
    workspace: &WeakEntity<Workspace>,
    client: Arc<dyn DatabaseClient>,
    table: TableRef,
    window: &mut Window,
    cx: &mut App,
) {
    workspace
        .update(cx, |workspace, cx| {
            let existing = workspace
                .active_pane()
                .read(cx)
                .items_of_type::<TableDataView>()
                .find(|view| view.read(cx).table() == &table);
            let view = existing.unwrap_or_else(|| TableDataView::new(client, table, window, cx));
            workspace.active_pane().update(cx, |pane, cx| {
                if let Some(index) = pane.index_for_item(&view) {
                    pane.activate_item(index, true, true, window, cx);
                } else {
                    pane.add_item(Box::new(view), true, true, None, window, cx);
                }
            });
        })
        .log_err();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use database_client::fake::FakeDatabaseClient;
    use database_client::{DatabaseClient, Filter, FilterOp, SortDirection, TableRef};
    use gpui::{TestAppContext, VisualTestContext};

    use super::{LoadState, TableDataView, ViewMode, all_filter_ops, filter_op_label};

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = settings::SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            gpui_tokio::init(cx);
            crate::init(cx);
        });
    }

    fn table_ref() -> TableRef {
        TableRef {
            database: "app".into(),
            schema: "public".into(),
            name: "users".into(),
        }
    }

    /// Drives the deterministic scheduler while giving the real tokio runtime a
    /// chance to complete cross-thread work, until `condition` holds or a bound
    /// is reached. Requires `cx.executor().allow_parking()`.
    ///
    /// Operates on a [`VisualTestContext`], which derefs into the underlying
    /// [`TestAppContext`] for scheduler and timer control.
    async fn wait_until(
        cx: &mut VisualTestContext,
        condition: impl Fn(&mut VisualTestContext) -> bool,
    ) {
        for _ in 0..200 {
            cx.run_until_parked();
            if condition(cx) {
                return;
            }
            cx.background_executor
                .timer(std::time::Duration::from_millis(5))
                .await;
        }
        cx.run_until_parked();
        assert!(
            condition(cx),
            "condition did not become true within the time bound"
        );
    }

    #[gpui::test]
    async fn table_view_loads_first_page(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));

        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.read_with(cx, |view, _| {
            assert!(view.page().is_some(), "first page should be loaded");
            assert_eq!(view.spec().limit, 100, "limit comes from page_size setting");
            assert_eq!(view.load_state(), &LoadState::Idle);
        });
        assert!(
            fake.calls()
                .iter()
                .any(|call| call.starts_with("fetch_rows users")),
            "fetch_rows should have been called: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn sort_click_resets_offset_and_reloads(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Advance to a non-zero offset first so the reset is observable, and
        // let that load settle so its fetch is recorded before we sort.
        view.update(cx, |view, cx| view.next_page(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update(cx, |view, cx| view.toggle_sort("name", cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().sort.is_some() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            let sort = view.spec().sort.as_ref().expect("sort should be set");
            assert_eq!(sort.column, "name");
            assert_eq!(sort.direction, SortDirection::Asc);
            assert_eq!(view.spec().offset, 0, "sorting resets offset to 0");
        });

        let fetch_calls = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("fetch_rows"))
            .count();
        assert!(
            fetch_calls >= 3,
            "expected initial + next_page + sort fetches, got {fetch_calls}"
        );
    }

    #[gpui::test]
    async fn next_prev_page_updates_offset(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // has_more == true in the fake, so next_page advances by the limit.
        // Wait for each load to settle so its fetch is recorded (the abort-on-
        // supersede behaviour would otherwise drop an in-flight fetch).
        view.update(cx, |view, cx| view.next_page(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.read_with(cx, |view, _| assert_eq!(view.spec().offset, 100));

        view.update(cx, |view, cx| view.prev_page(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().offset == 0 && view.load_state() == &LoadState::Idle
            })
        })
        .await;
        view.read_with(cx, |view, _| assert_eq!(view.spec().offset, 0));

        // prev_page at offset 0 is a no-op.
        let before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("fetch_rows"))
            .count();
        view.update(cx, |view, cx| view.prev_page(cx));
        cx.run_until_parked();
        let after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("fetch_rows"))
            .count();
        assert_eq!(before, after, "prev_page at offset 0 should not refetch");
        view.read_with(cx, |view, _| assert_eq!(view.spec().offset, 0));
    }

    #[gpui::test]
    async fn structure_mode_fetches_structure_once(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update(cx, |view, cx| view.toggle_structure(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| view.structure().is_some())
        })
        .await;
        view.read_with(cx, |view, _| {
            assert!(view.structure().is_some());
            assert_eq!(view.mode(), ViewMode::Structure);
        });

        let structure_calls_first = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();
        assert_eq!(structure_calls_first, 1);

        // Toggle back to Data and again to Structure: no second structure fetch.
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();
        view.update(cx, |view, cx| view.toggle_structure(cx));
        cx.run_until_parked();

        let structure_calls_second = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("table_structure"))
            .count();
        assert_eq!(
            structure_calls_second, 1,
            "structure should be cached after first fetch"
        );
    }

    #[gpui::test]
    async fn load_error_is_surfaced(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::with_error("connection refused"));
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                matches!(view.load_state(), LoadState::Error(_))
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            let LoadState::Error(message) = view.load_state() else {
                panic!("expected error load state, got {:?}", view.load_state());
            };
            assert!(
                message.contains("connection refused"),
                "unexpected error message: {message}"
            );
        });
    }

    #[gpui::test]
    async fn add_filter_resets_offset_and_reloads(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        // Advance to a non-zero offset so the reset is observable, letting the
        // load settle before we add a filter.
        view.update(cx, |view, cx| view.next_page(cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().offset == 100 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.update(cx, |view, cx| {
            view.add_filter(
                Filter {
                    column: "name".into(),
                    op: FilterOp::Contains,
                    value: "ali".into(),
                },
                cx,
            )
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().filters.len() == 1 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert_eq!(view.spec().filters.len(), 1, "filter should be stored");
            assert_eq!(view.spec().offset, 0, "adding a filter resets the offset");
        });

        assert!(
            fake.calls().iter().any(|call| call.contains("filters=1")),
            "adding a filter should trigger a fetch with filters=1: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    async fn remove_filter_reloads_without_filters(cx: &mut TestAppContext) {
        init_test(cx);
        cx.executor().allow_parking();
        let fake = Arc::new(FakeDatabaseClient::new());
        let client: Arc<dyn DatabaseClient> = fake.clone();

        let cx = cx.add_empty_window();
        let view = cx.update(|window, cx| TableDataView::new(client, table_ref(), window, cx));
        wait_until(cx, |cx| view.read_with(cx, |view, _| view.page().is_some())).await;

        view.update(cx, |view, cx| {
            view.add_filter(
                Filter {
                    column: "name".into(),
                    op: FilterOp::Eq,
                    value: "Alice".into(),
                },
                cx,
            )
        });
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().filters.len() == 1 && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        // Removing an out-of-bounds index is a no-op and does not refetch.
        let before = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("fetch_rows"))
            .count();
        view.update(cx, |view, cx| view.remove_filter(5, cx));
        cx.run_until_parked();
        let after = fake
            .calls()
            .into_iter()
            .filter(|call| call.starts_with("fetch_rows"))
            .count();
        assert_eq!(
            before, after,
            "out-of-bounds remove_filter should not refetch"
        );
        view.read_with(cx, |view, _| assert_eq!(view.spec().filters.len(), 1));

        view.update(cx, |view, cx| view.remove_filter(0, cx));
        wait_until(cx, |cx| {
            view.read_with(cx, |view, _| {
                view.spec().filters.is_empty() && view.load_state() == &LoadState::Idle
            })
        })
        .await;

        view.read_with(cx, |view, _| {
            assert!(view.spec().filters.is_empty(), "filter should be removed");
        });
        assert!(
            fake.calls().iter().any(|call| call.contains("filters=0")),
            "removing the filter should trigger a fetch with filters=0: {:?}",
            fake.calls()
        );
    }

    #[gpui::test]
    fn filter_op_labels_cover_all_ops(_cx: &mut TestAppContext) {
        assert_eq!(filter_op_label(FilterOp::Eq), "=");
        assert_eq!(filter_op_label(FilterOp::NotEq), "≠");
        assert_eq!(filter_op_label(FilterOp::Gt), ">");
        assert_eq!(filter_op_label(FilterOp::Lt), "<");
        assert_eq!(filter_op_label(FilterOp::Contains), "contains");
        assert_eq!(filter_op_label(FilterOp::IsNull), "is null");

        let ops = all_filter_ops();
        assert_eq!(ops.len(), 6, "there should be six filter operators");
        for op in ops {
            assert!(
                !filter_op_label(op).is_empty(),
                "every operator needs a label"
            );
        }
    }
}
