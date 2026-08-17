use crate::Result;
use crate::config::{config_f64 as cfg_f64, config_string as cfg_string};
use crate::model::{Bounds, KanbanDiagramLayout, KanbanItemLayout, KanbanSectionLayout};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use merman_core::diagrams::kanban::{KanbanDiagramRenderModel, KanbanRenderNode};

pub(crate) const KANBAN_SECTION_LABEL_HEIGHT_BASELINE_PX: f64 = 25.0;
pub(crate) const KANBAN_SECTION_PADDING_PX: f64 = 10.0;
pub(crate) const KANBAN_LABEL_FOREIGN_OBJECT_HEIGHT_PX: f64 = 24.0;
const KANBAN_ITEM_ONE_ROW_HEIGHT_PX: f64 = 44.0;
const KANBAN_ITEM_TWO_ROW_HEIGHT_PX: f64 = 56.0;

fn kanban_text_style(effective_config: &serde_json::Value) -> TextStyle {
    let font_family = cfg_string(effective_config, &["fontFamily"])
        .or_else(|| cfg_string(effective_config, &["themeVariables", "fontFamily"]))
        .or_else(|| Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()));
    let font_size =
        crate::config::config_theme_or_root_font_size_px(effective_config, 16.0).max(1.0);
    TextStyle {
        font_family,
        font_size,
        font_weight: None,
    }
}

pub fn layout_kanban_diagram(
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<KanbanDiagramLayout> {
    let model: KanbanDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_kanban_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_kanban_diagram_typed(
    model: &KanbanDiagramRenderModel,
    effective_config: &serde_json::Value,
    measurer: &dyn TextMeasurer,
) -> Result<KanbanDiagramLayout> {
    let section_width = cfg_f64(effective_config, &["kanban", "sectionWidth"])
        .unwrap_or(200.0)
        .max(1.0);

    // Mermaid 11.12.2 has a bug: `kanbanRenderer` uses `conf.mindmap.padding`/`useMaxWidth` when
    // calling `setupGraphViewbox`. Mirror that behavior for parity.
    let viewbox_padding = cfg_f64(effective_config, &["mindmap", "padding"])
        .or_else(|| cfg_f64(effective_config, &["kanban", "padding"]))
        .unwrap_or(8.0)
        .max(0.0);

    let padding = KANBAN_SECTION_PADDING_PX;
    let section_rect_y = -(section_width * 3.0) / 2.0;

    let legend_style = kanban_text_style(effective_config);
    let font_scale = legend_style.font_size / 16.0;
    let section_label_height_baseline = KANBAN_SECTION_LABEL_HEIGHT_BASELINE_PX * font_scale;
    let label_foreign_object_height = KANBAN_LABEL_FOREIGN_OBJECT_HEIGHT_PX * font_scale;
    let item_one_row_height = KANBAN_ITEM_ONE_ROW_HEIGHT_PX * font_scale;
    let item_two_row_height = KANBAN_ITEM_TWO_ROW_HEIGHT_PX * font_scale;

    let mut max_label_height = section_label_height_baseline;
    let mut sections: Vec<KanbanSectionLayout> = Vec::new();
    let mut items: Vec<KanbanItemLayout> = Vec::new();

    let section_nodes: Vec<&KanbanRenderNode> = model.nodes.iter().filter(|n| n.is_group).collect();
    for (i, section) in section_nodes.iter().enumerate() {
        let index = (i + 1) as i64;
        let center_x = section_width * (index as f64) + ((index - 1) as f64 * padding) / 2.0;
        let center_y = 0.0;

        let label_metrics = measurer.measure_wrapped(
            &section.label,
            &legend_style,
            Some(section_width),
            WrapMode::HtmlLike,
        );
        let label_height = label_metrics.height.max(label_foreign_object_height);
        max_label_height = max_label_height.max(label_height);

        sections.push(KanbanSectionLayout {
            id: section.id.clone(),
            label: section.label.clone(),
            index,
            center_x,
            center_y,
            width: section_width,
            rect_y: section_rect_y,
            rect_height: (section_width * 3.0).max(1.0),
            rx: 5.0,
            ry: 5.0,
            label_width: label_metrics.width.max(0.0),
            label_height,
        });
    }

    for section in sections.iter_mut() {
        let top = section_rect_y + max_label_height;
        let mut y = top;

        let section_items: Vec<&KanbanRenderNode> = model
            .nodes
            .iter()
            .filter(|n| n.parent_id.as_deref() == Some(section.id.as_str()))
            .collect();

        for item in section_items {
            let width = (section_width - 1.5 * padding).max(1.0);
            let inner_max_w = (width - padding).max(0.0);

            // Mermaid's kanban items are rendered via `kanbanItem.ts`, which uses HTML labels for
            // the title and applies `max-width` clamping when the content needs wrapping. Mirror
            // that behavior so item heights match the upstream bbox-based layout.
            let item_label_style = legend_style.clone();
            let raw_title_metrics =
                measurer.measure_wrapped(&item.label, &item_label_style, None, WrapMode::HtmlLike);
            let title_metrics = if inner_max_w > 0.0 && raw_title_metrics.width > inner_max_w {
                measurer.measure_wrapped(
                    &item.label,
                    &item_label_style,
                    Some(inner_max_w),
                    WrapMode::HtmlLike,
                )
            } else {
                raw_title_metrics
            };

            let has_details_row = item.ticket.is_some() || item.assigned.is_some();
            let base_height = if has_details_row {
                item_two_row_height
            } else {
                item_one_row_height
            };
            let extra_title_height = (title_metrics.height - label_foreign_object_height).max(0.0);
            let height = base_height + extra_title_height;

            let center_x = section.center_x;
            let center_y = y + height / 2.0;

            items.push(KanbanItemLayout {
                id: item.id.clone(),
                label: item.label.clone(),
                parent_id: section.id.clone(),
                center_x,
                center_y,
                width,
                height: height.max(1.0),
                rx: 5.0,
                ry: 5.0,
                ticket: item.ticket.clone(),
                assigned: item.assigned.clone(),
                priority: item.priority.clone(),
                icon: item.icon.clone(),
            });

            y = center_y + height / 2.0 + padding / 2.0;
        }

        let min_section_height = 50.0 * font_scale;
        let height = (y - top + 3.0 * padding).max(min_section_height)
            + (max_label_height - section_label_height_baseline);
        section.rect_height = height.max(1.0);
    }

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for s in &sections {
        let left = s.center_x - s.width / 2.0;
        let right = left + s.width;
        let top = s.rect_y;
        let bottom = s.rect_y + s.rect_height;
        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
    }
    for n in &items {
        let left = n.center_x - n.width / 2.0;
        let right = n.center_x + n.width / 2.0;
        let top = n.center_y - n.height / 2.0;
        let bottom = n.center_y + n.height / 2.0;
        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
    }

    let bounds = if min_x.is_finite() && min_y.is_finite() && max_x.is_finite() && max_y.is_finite()
    {
        Some(Bounds {
            min_x: min_x - viewbox_padding,
            min_y: min_y - viewbox_padding,
            max_x: max_x + viewbox_padding,
            max_y: max_y + viewbox_padding,
        })
    } else {
        None
    };

    Ok(KanbanDiagramLayout {
        bounds,
        section_width,
        padding,
        max_label_height,
        viewbox_padding,
        sections,
        items,
    })
}

#[cfg(test)]
mod tests {
    use super::layout_kanban_diagram;
    use crate::text::DeterministicTextMeasurer;
    use serde_json::json;

    #[test]
    fn kanban_geometry_constants_match_mermaid() {
        assert_eq!(super::KANBAN_SECTION_LABEL_HEIGHT_BASELINE_PX, 25.0);
        assert_eq!(super::KANBAN_SECTION_PADDING_PX, 10.0);
        assert_eq!(super::KANBAN_LABEL_FOREIGN_OBJECT_HEIGHT_PX, 24.0);
        assert_eq!(super::KANBAN_ITEM_ONE_ROW_HEIGHT_PX, 44.0);
        assert_eq!(super::KANBAN_ITEM_TWO_ROW_HEIGHT_PX, 56.0);
    }

    #[test]
    fn kanban_layout_uses_mermaid_padding() {
        let semantic = json!({
            "type": "kanban",
            "nodes": [
                {"id": "todo", "label": "Todo", "isGroup": true},
                {"id": "doing", "label": "Doing", "isGroup": true},
                {"id": "task-1", "label": "Task", "parentId": "todo"}
            ]
        });
        let measurer = DeterministicTextMeasurer {
            char_width_factor: 8.0,
            line_height_factor: 16.0,
        };

        let layout = layout_kanban_diagram(&semantic, &json!({}), &measurer).unwrap();

        assert_eq!(layout.padding, super::KANBAN_SECTION_PADDING_PX);
        assert_eq!(
            layout.items[0].width,
            layout.section_width - 1.5 * super::KANBAN_SECTION_PADDING_PX
        );
    }
}
