use std::sync::Arc;

use collections::HashSet;
use gpui::{div, prelude::*, px, App, ClickEvent, IntoElement, MouseButton, MouseDownEvent, Pixels, Point, SharedString, UniformListScrollHandle};
use serde::{Deserialize, Serialize};
use ui::{prelude::*, Icon, IconName, Label};

use database_core::{DatabaseSchema, TableInfo};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemaNodeId {
    TablesHeader,
    Table(String),
    ColumnsHeader(String),
    Column(String, String),
    IndexesHeader(String),
    Index(String, String),
    ForeignKeysHeader(String),
    ForeignKey(String, String),
}

impl SchemaNodeId {
    pub fn table_name(&self) -> Option<&str> {
        match self {
            SchemaNodeId::Table(name) => Some(name),
            _ => None,
        }
    }
}

pub struct FlattenedNode {
    pub id: SchemaNodeId,
    pub depth: usize,
    pub label: SharedString,
    pub icon: IconName,
    pub expandable: bool,
    pub expanded: bool,
    pub detail: Option<SharedString>,
}

pub fn flatten_schema(
    schema: &DatabaseSchema,
    expanded: &HashSet<SchemaNodeId>,
    filter: &str,
) -> Vec<FlattenedNode> {
    let mut nodes = Vec::new();
    let filter_lower = filter.trim().to_lowercase();
    let has_filter = !filter_lower.is_empty();

    let filtered_tables: Vec<&TableInfo> = if has_filter {
        schema
            .tables
            .iter()
            .filter(|table| {
                table.name.to_lowercase().contains(&filter_lower)
                    || table
                        .columns
                        .iter()
                        .any(|col| col.name.to_lowercase().contains(&filter_lower))
            })
            .collect()
    } else {
        schema.tables.iter().collect()
    };

    let tables_header_expanded = has_filter || expanded.contains(&SchemaNodeId::TablesHeader);
    nodes.push(FlattenedNode {
        id: SchemaNodeId::TablesHeader,
        depth: 0,
        label: SharedString::from(format!("Tables ({})", filtered_tables.len())),
        icon: IconName::FolderOpen,
        expandable: true,
        expanded: tables_header_expanded,
        detail: None,
    });

    if tables_header_expanded {
        for table in &filtered_tables {
            flatten_table(&mut nodes, table, expanded, has_filter);
        }
    }

    nodes
}

fn flatten_table(
    nodes: &mut Vec<FlattenedNode>,
    table: &TableInfo,
    expanded: &HashSet<SchemaNodeId>,
    auto_expand: bool,
) {
    let table_id = SchemaNodeId::Table(table.name.clone());
    let table_expanded = auto_expand || expanded.contains(&table_id);
    let row_count_text = table
        .row_count
        .map(|count| SharedString::from(format!("{} rows", count)));

    nodes.push(FlattenedNode {
        id: table_id.clone(),
        depth: 1,
        label: SharedString::from(table.name.clone()),
        icon: IconName::DatabaseZap,
        expandable: true,
        expanded: table_expanded,
        detail: row_count_text,
    });

    if expanded.contains(&table_id) {
        if !table.columns.is_empty() {
            let columns_header_id = SchemaNodeId::ColumnsHeader(table.name.clone());
            let columns_expanded = expanded.contains(&columns_header_id);
            nodes.push(FlattenedNode {
                id: columns_header_id.clone(),
                depth: 2,
                label: SharedString::from(format!("Columns ({})", table.columns.len())),
                icon: IconName::ListTree,
                expandable: true,
                expanded: columns_expanded,
                detail: None,
            });

            if expanded.contains(&columns_header_id) {
                for col in &table.columns {
                    let type_info = if col.primary_key {
                        format!("{}, PK", col.data_type)
                    } else if !col.nullable {
                        format!("{}, NOT NULL", col.data_type)
                    } else {
                        col.data_type.clone()
                    };

                    let icon = if col.primary_key {
                        IconName::BoltFilled
                    } else {
                        IconName::Hash
                    };

                    nodes.push(FlattenedNode {
                        id: SchemaNodeId::Column(table.name.clone(), col.name.clone()),
                        depth: 3,
                        label: SharedString::from(col.name.clone()),
                        icon,
                        expandable: false,
                        expanded: false,
                        detail: Some(SharedString::from(type_info)),
                    });
                }
            }
        }

        if !table.indexes.is_empty() {
            let indexes_header_id = SchemaNodeId::IndexesHeader(table.name.clone());
            let indexes_expanded = expanded.contains(&indexes_header_id);
            nodes.push(FlattenedNode {
                id: indexes_header_id.clone(),
                depth: 2,
                label: SharedString::from(format!("Indexes ({})", table.indexes.len())),
                icon: IconName::ListTree,
                expandable: true,
                expanded: indexes_expanded,
                detail: None,
            });

            if expanded.contains(&indexes_header_id) {
                for idx in &table.indexes {
                    let detail = format!(
                        "{}{}",
                        idx.columns.join(", "),
                        if idx.unique { " (UNIQUE)" } else { "" }
                    );

                    nodes.push(FlattenedNode {
                        id: SchemaNodeId::Index(table.name.clone(), idx.name.clone()),
                        depth: 3,
                        label: SharedString::from(idx.name.clone()),
                        icon: IconName::ToolSearch,
                        expandable: false,
                        expanded: false,
                        detail: Some(SharedString::from(detail)),
                    });
                }
            }
        }

        if !table.foreign_keys.is_empty() {
            let fk_header_id = SchemaNodeId::ForeignKeysHeader(table.name.clone());
            let fk_expanded = expanded.contains(&fk_header_id);
            nodes.push(FlattenedNode {
                id: fk_header_id.clone(),
                depth: 2,
                label: SharedString::from(format!("Foreign Keys ({})", table.foreign_keys.len())),
                icon: IconName::ListTree,
                expandable: true,
                expanded: fk_expanded,
                detail: None,
            });

            if expanded.contains(&fk_header_id) {
                for fk in &table.foreign_keys {
                    let fk_label = format!("{} -> {}.{}", fk.from_column, fk.to_table, fk.to_column);
                    nodes.push(FlattenedNode {
                        id: SchemaNodeId::ForeignKey(table.name.clone(), fk.from_column.clone()),
                        depth: 3,
                        label: SharedString::from(fk_label),
                        icon: IconName::ArrowUpRight,
                        expandable: false,
                        expanded: false,
                        detail: None,
                    });
                }
            }
        }
    }
}

/// Render data needed per node in the uniform_list closure.
/// Only the fields needed for rendering are cloned, not the full FlattenedNode.
struct RenderNode {
    depth: usize,
    label: SharedString,
    icon: IconName,
    expandable: bool,
    expanded: bool,
    detail: Option<SharedString>,
}

/// Callback receives (index, click_count, window, cx).
/// click_count == 2 means double-click.
pub fn render_schema_tree(
    flattened_nodes: &[FlattenedNode],
    selected_index: Option<usize>,
    scroll_handle: &UniformListScrollHandle,
    on_click: impl Fn(usize, usize, &mut gpui::Window, &mut App) + Send + Sync + 'static,
    on_secondary_click: impl Fn(usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync + 'static,
) -> impl IntoElement {
    let on_click: Arc<dyn Fn(usize, usize, &mut gpui::Window, &mut App) + Send + Sync> =
        Arc::new(on_click);
    let on_secondary_click: Arc<dyn Fn(usize, Point<Pixels>, &mut gpui::Window, &mut App) + Send + Sync> =
        Arc::new(on_secondary_click);
    let node_count = flattened_nodes.len();
    let nodes: Vec<RenderNode> = flattened_nodes
        .iter()
        .map(|node| RenderNode {
            depth: node.depth,
            label: node.label.clone(),
            icon: node.icon,
            expandable: node.expandable,
            expanded: node.expanded,
            detail: node.detail.clone(),
        })
        .collect();

    gpui::uniform_list("schema-tree", node_count, {
        move |range, _window, cx| {
            range
                .map(|index| {
                    let node = &nodes[index];
                    let is_selected = selected_index == Some(index);
                    let indent = px(node.depth as f32 * 16.0 + 4.0);
                    let chevron_icon = if node.expanded {
                        IconName::ChevronDown
                    } else {
                        IconName::ChevronRight
                    };

                    div()
                        .id(gpui::ElementId::named_usize("schema-node", index))
                        .h(px(26.0))
                        .w_full()
                        .flex()
                        .items_center()
                        .pl(indent)
                        .gap_1()
                        .rounded_md()
                        .when(is_selected, |this: gpui::Stateful<gpui::Div>| {
                            this.bg(cx.theme().colors().ghost_element_selected)
                        })
                        .hover(|style| style.bg(cx.theme().colors().ghost_element_hover))
                        .on_click({
                            let on_click = on_click.clone();
                            move |event: &ClickEvent, window, cx| {
                                on_click(index, event.click_count(), window, cx)
                            }
                        })
                        .on_mouse_down(MouseButton::Right, {
                            let on_secondary_click = on_secondary_click.clone();
                            move |event: &MouseDownEvent, window, cx| {
                                on_secondary_click(index, event.position, window, cx)
                            }
                        })
                        .child(
                            h_flex()
                                .gap_1()
                                .items_center()
                                .when(node.expandable, |this| {
                                    this.child(
                                        Icon::new(chevron_icon)
                                            .size(IconSize::XSmall)
                                            .color(ui::Color::Muted),
                                    )
                                })
                                .when(!node.expandable, |this| this.child(div().w(px(12.0))))
                                .child(
                                    Icon::new(node.icon)
                                        .size(IconSize::Small)
                                        .color(ui::Color::Muted),
                                )
                                .child(
                                    Label::new(node.label.clone())
                                        .size(LabelSize::Small)
                                        .color(ui::Color::Default),
                                )
                                .when_some(node.detail.clone(), |this, d| {
                                    this.child(
                                        Label::new(d)
                                            .size(LabelSize::XSmall)
                                            .color(ui::Color::Muted),
                                    )
                                }),
                        )
                        .into_any_element()
                })
                .collect()
        }
    })
    .track_scroll(scroll_handle)
    .flex_grow()
}

#[cfg(test)]
mod tests {
    use super::*;
    use database_core::{ColumnInfo, ForeignKeyInfo, IndexInfo};

    fn test_schema() -> DatabaseSchema {
        DatabaseSchema {
            tables: vec![
                TableInfo {
                    name: "users".to_string(),
                    columns: vec![
                        ColumnInfo {
                            name: "id".to_string(),
                            data_type: "INTEGER".to_string(),
                            nullable: false,
                            primary_key: true,
                            default_value: None,
                        },
                        ColumnInfo {
                            name: "name".to_string(),
                            data_type: "TEXT".to_string(),
                            nullable: false,
                            primary_key: false,
                            default_value: None,
                        },
                    ],
                    indexes: vec![IndexInfo {
                        name: "idx_name".to_string(),
                        columns: vec!["name".to_string()],
                        unique: false,
                    }],
                    foreign_keys: vec![],
                    row_count: Some(10),
                    is_virtual: false,
                    ddl: None,
                },
                TableInfo {
                    name: "orders".to_string(),
                    columns: vec![ColumnInfo {
                        name: "id".to_string(),
                        data_type: "INTEGER".to_string(),
                        nullable: false,
                        primary_key: true,
                        default_value: None,
                    }],
                    indexes: vec![],
                    foreign_keys: vec![ForeignKeyInfo {
                        from_column: "user_id".to_string(),
                        to_table: "users".to_string(),
                        to_column: "id".to_string(),
                    }],
                    row_count: Some(5),
                    is_virtual: false,
                    ddl: None,
                },
            ],
        }
    }

    #[test]
    fn test_flatten_schema_collapsed() {
        let schema = test_schema();
        let expanded = HashSet::default();
        let nodes = flatten_schema(&schema, &expanded, "");

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, SchemaNodeId::TablesHeader);
        assert!(!nodes[0].expanded);
    }

    #[test]
    fn test_flatten_schema_tables_expanded() {
        let schema = test_schema();
        let mut expanded = HashSet::default();
        expanded.insert(SchemaNodeId::TablesHeader);
        let nodes = flatten_schema(&schema, &expanded, "");

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].id, SchemaNodeId::TablesHeader);
        assert_eq!(nodes[1].id, SchemaNodeId::Table("users".to_string()));
        assert_eq!(nodes[2].id, SchemaNodeId::Table("orders".to_string()));
    }

    #[test]
    fn test_flatten_schema_fully_expanded() {
        let schema = test_schema();
        let mut expanded = HashSet::default();
        expanded.insert(SchemaNodeId::TablesHeader);
        expanded.insert(SchemaNodeId::Table("users".to_string()));
        expanded.insert(SchemaNodeId::ColumnsHeader("users".to_string()));
        expanded.insert(SchemaNodeId::IndexesHeader("users".to_string()));
        let nodes = flatten_schema(&schema, &expanded, "");

        // TablesHeader + users + ColumnsHeader + 2 columns + IndexesHeader + 1 index + orders = 8
        assert_eq!(nodes.len(), 8);

        assert_eq!(nodes[3].id, SchemaNodeId::Column("users".to_string(), "id".to_string()));
        assert_eq!(nodes[3].depth, 3);
        assert_eq!(nodes[4].id, SchemaNodeId::Column("users".to_string(), "name".to_string()));

        assert_eq!(nodes[5].id, SchemaNodeId::IndexesHeader("users".to_string()));
        assert_eq!(nodes[6].id, SchemaNodeId::Index("users".to_string(), "idx_name".to_string()));
    }

    #[test]
    fn test_flatten_schema_filtered_by_table_name() {
        let schema = test_schema();
        let expanded = HashSet::default();
        let nodes = flatten_schema(&schema, &expanded, "users");

        // TablesHeader (auto-expanded) + users (auto-expanded with children)
        assert_eq!(nodes[0].id, SchemaNodeId::TablesHeader);
        assert!(nodes[0].expanded);
        assert_eq!(nodes[1].id, SchemaNodeId::Table("users".to_string()));
        assert!(nodes[1].expanded);
        assert!(!nodes.iter().any(|n| n.id == SchemaNodeId::Table("orders".to_string())));
    }

    #[test]
    fn test_flatten_schema_filtered_by_column_name() {
        let schema = test_schema();
        let expanded = HashSet::default();
        let nodes = flatten_schema(&schema, &expanded, "name");

        // "name" is a column in "users" -> users should match
        assert!(nodes.iter().any(|n| n.id == SchemaNodeId::Table("users".to_string())));
    }
}
