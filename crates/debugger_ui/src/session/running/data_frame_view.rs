use anyhow::{Context as _, Result, anyhow};
use dap::EvaluateArgumentsContext;
use gpui::{Context, Entity, FocusHandle, Focusable, Window};
use project::debugger::session::Session;
use serde::Deserialize;
use ui::{
    Color, ContextMenu, DropdownMenu, IconButton, IconName, IconSize, Label, LabelSize,
    PopoverMenuHandle, Render, Table, Tooltip, UncheckedTableRow, prelude::*,
};

#[derive(Clone, Debug)]
struct DataFrameTable {
    columns: Vec<SharedString>,
    rows: Vec<Vec<SharedString>>,
}

#[derive(Deserialize)]
struct SplitOrientation {
    columns: Vec<String>,
    data: Vec<Vec<serde_json::Value>>,
}

pub(crate) struct DataFrameView {
    focus_handle: FocusHandle,
    session: Entity<Session>,
    row_limit: usize,
    row_limit_picker_handle: PopoverMenuHandle<ContextMenu>,

    expression: Option<SharedString>,
    frame_id: Option<u64>,
    loading: bool,
    error: Option<SharedString>,
    table: Option<DataFrameTable>,
}

impl Focusable for DataFrameView {
    fn focus_handle(&self, _: &ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DataFrameView {
    pub(crate) fn new(
        session: Entity<Session>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            session,
            row_limit: 100,
            row_limit_picker_handle: Default::default(),
            expression: None,
            frame_id: None,
            loading: false,
            error: None,
            table: None,
        }
    }

    pub(crate) fn show_expression(
        &mut self,
        expression: SharedString,
        frame_id: Option<u64>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.expression = Some(expression);
        self.frame_id = frame_id;
        self.refresh(window, cx);
    }

    pub(crate) fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(expression) = self.expression.clone() else {
            return;
        };
        let frame_id = self.frame_id;
        let row_limit = self.row_limit;
        let session = self.session.clone();
        let weak = cx.weak_entity();

        self.loading = true;
        self.error = None;
        cx.notify();

        cx.spawn_in(window, async move |_, cx| {
            let python = format!(
                "({expr}).head({row_limit}).to_json(orient=\"split\", date_format=\"iso\")",
                expr = expression
            );

            let response = session
                .update(cx, |session, cx| {
                    session.evaluate_silent(
                        python,
                        Some(EvaluateArgumentsContext::Watch),
                        frame_id,
                        None,
                        cx,
                    )
                })
                .await;

            let result = response.and_then(|resp| parse_dataframe_json(&resp.result));

            _ = weak.update(cx, |this, cx| {
                this.loading = false;
                match result {
                    Ok(table) => {
                        this.table = Some(table);
                        this.error = None;
                    }
                    Err(error) => {
                        this.table = None;
                        this.error = Some(SharedString::from(error.to_string()));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn render_row_limit_picker(&self, window: &mut Window, cx: &mut Context<Self>) -> DropdownMenu {
        const OPTIONS: [usize; 4] = [25, 50, 100, 200];
        let weak = cx.weak_entity();
        let selected = self.row_limit;

        DropdownMenu::new(
            "dataframe-view-row-limit-picker",
            SharedString::from(format!("Rows: {selected}")),
            ContextMenu::build(window, cx, move |mut this, window, cx| {
                for option in OPTIONS {
                    let weak = weak.clone();
                    this = this.entry(SharedString::from(option.to_string()), None, move |_, cx| {
                        _ = weak.update(cx, |this, cx| {
                            this.row_limit = option;
                            cx.notify();
                        });
                    });
                }

                if let Some(ix) = OPTIONS.iter().position(|opt| *opt == selected) {
                    for _ in 0..=ix {
                        this.select_next(&Default::default(), window, cx);
                    }
                }
                this
            }),
        )
        .handle(self.row_limit_picker_handle.clone())
    }
}

impl Render for DataFrameView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = h_flex()
            .gap_2()
            .items_center()
            .justify_between()
            .px_2()
            .py_1()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new("Data").size(LabelSize::Small))
                    .when_some(self.expression.clone(), |this, expr| {
                        this.child(
                            Label::new(expr)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate(),
                        )
                    }),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(self.render_row_limit_picker(window, cx))
                    .child(
                        IconButton::new("dataframe-view-refresh", IconName::RotateCcw)
                            .icon_size(IconSize::Small)
                            .disabled(self.expression.is_none() || self.loading)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.refresh(window, cx);
                            }))
                            .tooltip(|window, cx| Tooltip::text("Refresh")(window, cx)),
                    ),
            );

        let body: AnyElement = if self.loading {
            v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .child(Label::new("Loading…").size(LabelSize::Small))
                .into_any_element()
        } else if let Some(error) = self.error.clone() {
            v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .child(
                    Label::new(error)
                        .size(LabelSize::Small)
                        .color(Color::Error),
                )
                .into_any_element()
        } else if let Some(table) = self.table.clone() {
            let cols = table.columns.len().max(1);
            let headers: UncheckedTableRow<_> = table
                .columns
                .iter()
                .cloned()
                .map(|c| Label::new(c).size(LabelSize::Small).into_any_element())
                .collect::<Vec<_>>()
                .into();

            let rows = table.rows.clone();
            Table::new(cols)
                .striped()
                .header(headers)
                .uniform_list("dataframe-view-table", rows.len(), move |range, _, _| {
                    range
                        .into_iter()
                        .map(|row_ix| {
                            let row = rows
                                .get(row_ix)
                                .cloned()
                                .unwrap_or_else(|| vec![SharedString::new_static("")]);
                            let cells: UncheckedTableRow<_> = row
                                .into_iter()
                                .map(|cell| {
                                    Label::new(cell)
                                        .size(LabelSize::Small)
                                        .into_any_element()
                                })
                                .collect();
                            cells
                        })
                        .collect()
                })
                .into_any_element()
        } else {
            v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .child(
                    Label::new("Right-click a variable → View as DataFrame")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        };

        v_flex().size_full().child(header).child(body)
    }
}

fn parse_dataframe_json(result: &str) -> Result<DataFrameTable> {
    if let Ok(table) = parse_split_json(result) {
        return Ok(table);
    }

    if let Ok(unquoted) = serde_json::from_str::<String>(result) {
        if let Ok(table) = parse_split_json(&unquoted) {
            return Ok(table);
        }
    }

    if let Some(stripped) = result.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        let unescaped = unescape_python_repr(stripped);
        return parse_split_json(&unescaped);
    }

    Err(anyhow!(
        "Could not parse DataFrame JSON (expected pandas .to_json(orient=\"split\"))."
    ))
}

fn parse_split_json(json_str: &str) -> Result<DataFrameTable> {
    let split: SplitOrientation =
        serde_json::from_str(json_str).with_context(|| "parsing JSON")?;

    let columns = split.columns.into_iter().map(SharedString::from).collect();
    let mut rows = Vec::with_capacity(split.data.len());
    for row in split.data {
        rows.push(row.into_iter().map(cell_to_string).collect());
    }
    Ok(DataFrameTable { columns, rows })
}

fn cell_to_string(value: serde_json::Value) -> SharedString {
    match value {
        serde_json::Value::Null => SharedString::new_static("None"),
        serde_json::Value::Bool(v) => SharedString::from(v.to_string()),
        serde_json::Value::Number(v) => SharedString::from(v.to_string()),
        serde_json::Value::String(v) => SharedString::from(v),
        other => SharedString::from(other.to_string()),
    }
}

fn unescape_python_repr(input: &str) -> String {
    let mut unescaped = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            unescaped.push(c);
        } else {
            match chars.next() {
                Some('\\') => unescaped.push('\\'),
                Some('n') => unescaped.push('\n'),
                Some('t') => unescaped.push('\t'),
                Some('r') => unescaped.push('\r'),
                Some('\'') => unescaped.push('\''),
                Some('"') => unescaped.push('"'),
                Some(c) => {
                    unescaped.push('\\');
                    unescaped.push(c);
                }
                None => {}
            }
        }
    }
    unescaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_split_orientation_json() {
        let json = r#"{"columns":["a","b"],"data":[[1,"x"],[2,"y"]]}"#;
        let table = parse_dataframe_json(json).unwrap();
        assert_eq!(
            table.columns,
            vec![SharedString::from("a"), SharedString::from("b")]
        );
        assert_eq!(table.rows.len(), 2);
        assert_eq!(
            table.rows[0],
            vec![SharedString::from("1"), SharedString::from("x")]
        );
    }

    #[test]
    fn parses_quoted_json_string() {
        let inner = r#"{"columns":["a"],"data":[[1],[2]]}"#;
        let wrapped = serde_json::to_string(inner).unwrap();
        let table = parse_dataframe_json(&wrapped).unwrap();
        assert_eq!(table.columns, vec![SharedString::from("a")]);
        assert_eq!(table.rows[1], vec![SharedString::from("2")]);
    }

    #[test]
    fn parses_single_quoted_python_repr_string() {
        let wrapped = "'{\"columns\":[\"a\"],\"data\":[[\"x\\\\n\"]]}'";
        let table = parse_dataframe_json(&wrapped).unwrap();
        assert_eq!(table.columns, vec![SharedString::from("a")]);
        assert_eq!(table.rows[0], vec![SharedString::from("x\n")]);
    }
}
