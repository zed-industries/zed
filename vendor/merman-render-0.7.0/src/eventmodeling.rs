use crate::Result;
use crate::config::{config_bool, config_f64};
use crate::model::{
    Bounds, EventModelingBoxLayout, EventModelingDiagramLayout, EventModelingRelationLayout,
    EventModelingSwimlaneLayout,
};
use crate::svg::render_theme::EventModelingTheme;
use crate::text::{TextMeasurer, TextStyle, split_html_br_lines, wrap_label_like_mermaid_lines};
use crate::theme::PresentationTheme;
use merman_core::diagrams::eventmodeling::{
    EventModelingDataEntityRenderModel, EventModelingDiagramRenderModel,
    EventModelingFrameRenderModel,
};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};

const SWIMLANE_MIN_HEIGHT: f64 = 70.0;
const SWIMLANE_PADDING: f64 = 15.0;
const SWIMLANE_GAP: f64 = 10.0;
const BOX_PADDING: f64 = 10.0;
const BOX_OVERLAP: f64 = 90.0;
const BOX_MIN_WIDTH: f64 = 80.0;
const BOX_MAX_WIDTH: f64 = 450.0;
const BOX_MIN_HEIGHT: f64 = 80.0;
const BOX_MAX_HEIGHT: f64 = 750.0;
const CONTENT_START_X: f64 = 250.0;
const TEXT_MAX_WIDTH: f64 = 430.0;
const BOX_TEXT_PADDING: f64 = 10.0;
const TEXT_FONT_SIZE: f64 = 16.0;
const HTML_LABEL_TEXT_WIDTH_OFFSET: f64 = 6.0;
const HTML_LABEL_DATA_WIDTH_SCALE: f64 = 1.047;
const HTML_LABEL_BBOX_LINE_HEIGHT: f64 = 19.0;

#[derive(Debug, Clone)]
struct EventModelingConfig {
    padding: f64,
    use_max_width: bool,
}

pub fn layout_eventmodeling_diagram(
    semantic: &Value,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<EventModelingDiagramLayout> {
    let model: EventModelingDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_eventmodeling_diagram_typed(&model, effective_config, measurer)
}

pub fn layout_eventmodeling_diagram_typed(
    model: &EventModelingDiagramRenderModel,
    effective_config: &Value,
    measurer: &dyn TextMeasurer,
) -> Result<EventModelingDiagramLayout> {
    let cfg = eventmodeling_config(effective_config);
    let theme = PresentationTheme::new(effective_config).eventmodeling();
    let data_entities: HashMap<&str, &EventModelingDataEntityRenderModel> = model
        .data_entities
        .iter()
        .map(|entity| (entity.name.as_str(), entity))
        .collect();

    let mut swimlanes: BTreeMap<i64, SwimlaneState> = BTreeMap::new();
    let mut boxes = Vec::new();
    let mut frame_to_box = HashMap::new();
    let mut relation_specs = Vec::new();
    let mut previous_swimlane_index = None;
    let mut max_r: f64 = 0.0;

    for (index, frame) in model.frames.iter().enumerate() {
        let text = frame_text(frame, &data_entities);
        let text_dimension = measure_frame_text(frame, &data_entities, measurer);
        let event_width = text_dimension.width + 2.0 * BOX_TEXT_PADDING;
        let event_height = text_dimension.height + 2.0 * BOX_TEXT_PADDING;
        let width = event_width.clamp(BOX_MIN_WIDTH, BOX_MAX_WIDTH) + 2.0 * BOX_PADDING;
        let height = event_height.clamp(BOX_MIN_HEIGHT, BOX_MAX_HEIGHT) + 2.0 * BOX_PADDING;
        let swimlane_props = calculate_swimlane_props(frame, &swimlanes);
        let swimlane = swimlanes
            .entry(swimlane_props.index)
            .or_insert_with(|| SwimlaneState {
                index: swimlane_props.index,
                label: swimlane_props.label.clone(),
                namespace: swimlane_props.namespace.clone(),
                r: 0.0,
                y: 0.0,
                height: SWIMLANE_MIN_HEIGHT,
                max_height: SWIMLANE_MIN_HEIGHT,
            });

        let x = calculate_x(swimlane, previous_swimlane_index, boxes.last());
        let r = x + width + BOX_PADDING;
        swimlane.r = x + width;
        swimlane.max_height = swimlane.max_height.max(height);
        swimlane.height = swimlane.max_height.max(SWIMLANE_MIN_HEIGHT) + 2.0 * SWIMLANE_PADDING;
        max_r = max_r.max(swimlane.r).max(r);

        let visual = entity_visual_props(&theme, &frame.model_entity_type);
        let box_state = BoxState {
            index,
            frame_name: frame.name.clone(),
            frame_kind: frame.frame_kind.clone(),
            model_entity_type: frame.model_entity_type.clone(),
            entity_identifier: frame.entity_identifier.clone(),
            text,
            x,
            width,
            height,
            fill: visual.fill,
            stroke: visual.stroke,
            swimlane_index: swimlane.index,
            r,
        };
        let target_box_idx = boxes.len();
        frame_to_box.insert(frame.name.clone(), target_box_idx);
        boxes.push(box_state);

        if frame.frame_kind != "resetframe" && !(index == 0 && frame.source_frames.is_empty()) {
            if frame.source_frames.is_empty() {
                if let Some(source_idx) =
                    find_previous_cross_swimlane_box(&boxes, swimlane_props.index, index)
                {
                    relation_specs.push((source_idx, target_box_idx));
                }
            } else {
                for source_name in &frame.source_frames {
                    if let Some(source_idx) = frame_to_box.get(source_name).copied() {
                        relation_specs.push((source_idx, target_box_idx));
                    }
                }
            }
        }

        previous_swimlane_index = Some(swimlane_props.index);
        recalculate_swimlane_y(&mut swimlanes);
    }

    recalculate_swimlane_y(&mut swimlanes);
    let swimlane_width = max_r + SWIMLANE_PADDING;
    let swimlane_layouts: Vec<_> = swimlanes
        .values()
        .map(|swimlane| EventModelingSwimlaneLayout {
            index: swimlane.index,
            label: swimlane.label.clone(),
            namespace: swimlane.namespace.clone(),
            x: 0.0,
            y: swimlane.y,
            width: swimlane_width.max(1.0),
            height: swimlane.height,
        })
        .collect();

    let box_layouts: Vec<_> = boxes
        .iter()
        .map(|box_state| {
            let swimlane = &swimlanes[&box_state.swimlane_index];
            EventModelingBoxLayout {
                index: box_state.index,
                frame_name: box_state.frame_name.clone(),
                frame_kind: box_state.frame_kind.clone(),
                model_entity_type: box_state.model_entity_type.clone(),
                entity_identifier: box_state.entity_identifier.clone(),
                text: box_state.text.clone(),
                x: box_state.x,
                y: swimlane.y + SWIMLANE_PADDING,
                width: box_state.width,
                height: box_state.height,
                fill: box_state.fill.clone(),
                stroke: box_state.stroke.clone(),
                swimlane_index: box_state.swimlane_index,
            }
        })
        .collect();

    let relation_stroke = theme.relation_stroke.clone();
    let relation_layouts: Vec<_> = relation_specs
        .into_iter()
        .map(|(source_idx, target_idx)| {
            let source = &box_layouts[source_idx];
            let target = &box_layouts[target_idx];
            let upwards = source.y > target.y;
            let x1 = source.x + (source.width * 2.0) / 3.0;
            let x2 = target.x + target.width / 3.0;
            let (y1, y2) = if upwards {
                (source.y, target.y + target.height)
            } else {
                (source.y + source.height, target.y)
            };
            EventModelingRelationLayout {
                source_frame: source.frame_name.clone(),
                target_frame: target.frame_name.clone(),
                x1,
                y1,
                x2,
                y2,
                stroke: relation_stroke.clone(),
            }
        })
        .collect();

    let mut bounds = BoundsAcc::new();
    for swimlane in &swimlane_layouts {
        bounds.include_rect(swimlane.x, swimlane.y, swimlane.width, swimlane.height);
    }
    for box_layout in &box_layouts {
        bounds.include_rect(
            box_layout.x,
            box_layout.y,
            box_layout.width,
            box_layout.height,
        );
    }
    for relation in &relation_layouts {
        bounds.include_point(relation.x1, relation.y1);
        bounds.include_point(relation.x2, relation.y2);
    }

    let padded = bounds.finish(cfg.padding).unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 1.0,
        max_y: 1.0,
    });
    let total_width = (padded.max_x - padded.min_x).max(1.0);
    let total_height = (padded.max_y - padded.min_y).max(1.0);
    let viewbox_x = padded.min_x;
    let viewbox_y = padded.min_y;

    Ok(EventModelingDiagramLayout {
        bounds: Some(padded),
        total_width,
        total_height,
        viewbox_x,
        viewbox_y,
        padding: cfg.padding,
        use_max_width: cfg.use_max_width,
        swimlanes: swimlane_layouts,
        boxes: box_layouts,
        relations: relation_layouts,
    })
}

#[derive(Debug, Clone)]
struct SwimlaneProps {
    index: i64,
    label: String,
    namespace: Option<String>,
}

#[derive(Debug, Clone)]
struct SwimlaneState {
    index: i64,
    label: String,
    namespace: Option<String>,
    r: f64,
    y: f64,
    height: f64,
    max_height: f64,
}

#[derive(Debug, Clone)]
struct BoxState {
    index: usize,
    frame_name: String,
    frame_kind: String,
    model_entity_type: String,
    entity_identifier: String,
    text: String,
    x: f64,
    width: f64,
    height: f64,
    fill: String,
    stroke: String,
    swimlane_index: i64,
    r: f64,
}

#[derive(Debug, Clone)]
struct VisualProps {
    fill: String,
    stroke: String,
}

#[derive(Debug, Clone, Copy)]
struct TextDimension {
    width: f64,
    height: f64,
}

fn calculate_swimlane_props(
    frame: &EventModelingFrameRenderModel,
    swimlanes: &BTreeMap<i64, SwimlaneState>,
) -> SwimlaneProps {
    let namespace = extract_namespace(&frame.entity_identifier);
    match frame.model_entity_type.as_str() {
        "ui" | "pcr" | "processor" => {
            namespaced_or_default_swimlane(swimlanes, namespace, 0, 100, "UI/Automation", "UI/A: ")
        }
        "rmo" | "readmodel" | "cmd" | "command" => namespaced_or_default_swimlane(
            swimlanes,
            namespace,
            100,
            200,
            "Command/Read Model",
            "C/RM: ",
        ),
        "evt" | "event" => {
            namespaced_or_default_swimlane(swimlanes, namespace, 200, 300, "Events", "Stream: ")
        }
        _ => namespaced_or_default_swimlane(swimlanes, namespace, 200, 300, "Events", "Stream: "),
    }
}

fn namespaced_or_default_swimlane(
    swimlanes: &BTreeMap<i64, SwimlaneState>,
    namespace: Option<String>,
    boundary_min: i64,
    boundary_max: i64,
    default_label: &str,
    prefix: &str,
) -> SwimlaneProps {
    if let Some(namespace) = namespace {
        if let Some(swimlane) =
            find_swimlane_by_namespace(swimlanes, &namespace, boundary_min, boundary_max)
        {
            return SwimlaneProps {
                index: swimlane.index,
                label: swimlane.label.clone(),
                namespace: Some(namespace),
            };
        }

        SwimlaneProps {
            index: find_next_available_index(swimlanes, boundary_min, boundary_max),
            label: format!("{prefix}{namespace}"),
            namespace: Some(namespace),
        }
    } else {
        SwimlaneProps {
            index: boundary_min,
            label: default_label.to_string(),
            namespace: None,
        }
    }
}

fn extract_namespace(entity_identifier: &str) -> Option<String> {
    let mut parts = entity_identifier.split('.');
    let namespace = parts.next()?;
    let name = parts.next()?;
    parts
        .next()
        .is_none()
        .then(|| (!namespace.is_empty() && !name.is_empty()).then(|| namespace.to_string()))
        .flatten()
}

fn extract_name(entity_identifier: &str) -> &str {
    let mut parts = entity_identifier.split('.');
    let Some(first) = parts.next() else {
        return entity_identifier;
    };
    let Some(second) = parts.next() else {
        return entity_identifier;
    };
    if parts.next().is_none() {
        second
    } else {
        first
    }
}

fn find_swimlane_by_namespace<'a>(
    swimlanes: &'a BTreeMap<i64, SwimlaneState>,
    namespace: &str,
    boundary_min: i64,
    boundary_max: i64,
) -> Option<&'a SwimlaneState> {
    swimlanes.values().find(|swimlane| {
        swimlane.index > boundary_min
            && swimlane.index < boundary_max
            && swimlane.namespace.as_deref() == Some(namespace)
    })
}

fn find_next_available_index(
    swimlanes: &BTreeMap<i64, SwimlaneState>,
    boundary_min: i64,
    boundary_max: i64,
) -> i64 {
    swimlanes
        .keys()
        .copied()
        .filter(|index| *index > boundary_min && *index < boundary_max)
        .fold(boundary_min, i64::max)
        + 1
}

fn calculate_x(
    swimlane: &SwimlaneState,
    previous_swimlane_index: Option<i64>,
    last_box: Option<&BoxState>,
) -> f64 {
    if previous_swimlane_index.is_none() {
        return CONTENT_START_X;
    }
    if previous_swimlane_index == Some(swimlane.index) && swimlane.r > 0.0 {
        return swimlane.r + BOX_PADDING;
    }
    if let Some(last_box) = last_box {
        return last_box.r - BOX_OVERLAP + BOX_PADDING;
    }
    CONTENT_START_X
}

fn find_previous_cross_swimlane_box(
    boxes: &[BoxState],
    target_swimlane_index: i64,
    line_index: usize,
) -> Option<usize> {
    if line_index == 0 {
        return None;
    }
    (0..line_index)
        .rev()
        .find(|idx| boxes[*idx].swimlane_index != target_swimlane_index)
}

fn recalculate_swimlane_y(swimlanes: &mut BTreeMap<i64, SwimlaneState>) {
    let mut next_y = 0.0;
    for swimlane in swimlanes.values_mut() {
        swimlane.y = next_y;
        next_y += swimlane.height + SWIMLANE_GAP;
    }
}

fn frame_text(
    frame: &EventModelingFrameRenderModel,
    data_entities: &HashMap<&str, &EventModelingDataEntityRenderModel>,
) -> String {
    let mut text = extract_name(&frame.entity_identifier).to_string();
    if let Some(data) = frame.data_inline_value.as_deref() {
        text.push('\n');
        text.push_str(data);
    } else if let Some(reference) = frame.data_reference.as_deref() {
        if let Some(entity) = data_entities.get(reference) {
            text.push('\n');
            text.push_str(&entity.data_block_value);
        }
    }
    text
}

fn measure_frame_text(
    frame: &EventModelingFrameRenderModel,
    data_entities: &HashMap<&str, &EventModelingDataEntityRenderModel>,
    measurer: &dyn TextMeasurer,
) -> TextDimension {
    let style = TextStyle {
        font_size: TEXT_FONT_SIZE,
        font_weight: Some("700".to_string()),
        ..Default::default()
    };
    let (html, has_data) = frame_label_html_for_measurement(frame, data_entities, measurer, &style);
    let mut dimension = measure_eventmodeling_label_html(&html, measurer, &style);
    if has_data {
        dimension.width = (dimension.width * HTML_LABEL_DATA_WIDTH_SCALE) / 3.0;
    } else {
        dimension.width += HTML_LABEL_TEXT_WIDTH_OFFSET;
    }
    TextDimension {
        width: dimension.width.min(TEXT_MAX_WIDTH),
        height: dimension.height,
    }
}

fn measure_eventmodeling_label_html(
    html: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> TextDimension {
    let sans_style = TextStyle {
        font_family: Some("sans-serif".to_string()),
        font_size: style.font_size,
        font_weight: style.font_weight.clone(),
    };
    let default_font_style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: style.font_size,
        font_weight: style.font_weight.clone(),
    };
    let sans = measure_eventmodeling_label_html_with_style(html, measurer, &sans_style);
    let default_font =
        measure_eventmodeling_label_html_with_style(html, measurer, &default_font_style);

    if sans.height > default_font.height
        && sans.width > default_font.width
        && sans.line_height > default_font.line_height
    {
        sans.into_dimension()
    } else {
        default_font.into_dimension()
    }
}

#[derive(Debug, Clone, Copy)]
struct EventModelingHtmlTextDimension {
    width: f64,
    height: f64,
    line_height: f64,
}

impl EventModelingHtmlTextDimension {
    fn into_dimension(self) -> TextDimension {
        TextDimension {
            width: self.width,
            height: self.height,
        }
    }
}

fn measure_eventmodeling_label_html_with_style(
    html: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> EventModelingHtmlTextDimension {
    let mut width: f64 = 0.0;
    let mut height: f64 = 0.0;
    let mut line_height: f64 = 0.0;
    for line in split_html_br_lines(html) {
        let line = line.replace(['\r', '\n'], " ");
        width = width.max(
            measurer
                .measure_svg_simple_text_bbox_width_px(&line, style)
                .round(),
        );
        let current_height = measurer
            .measure_svg_simple_text_bbox_height_px(&line, style)
            .round()
            .max(HTML_LABEL_BBOX_LINE_HEIGHT);
        height += current_height;
        line_height = line_height.max(current_height);
    }
    EventModelingHtmlTextDimension {
        width,
        height,
        line_height,
    }
}

fn frame_label_html_for_measurement(
    frame: &EventModelingFrameRenderModel,
    data_entities: &HashMap<&str, &EventModelingDataEntityRenderModel>,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> (String, bool) {
    let title = wrap_eventmodeling_label(extract_name(&frame.entity_identifier), measurer, style);
    let mut html = format!("<b>{title}</b>");
    let data = frame
        .data_inline_value
        .as_deref()
        .or_else(|| {
            frame
                .data_reference
                .as_deref()
                .and_then(|reference| data_entities.get(reference))
                .map(|entity| entity.data_block_value.as_str())
        })
        .map(normalize_eventmodeling_data_for_measurement);

    if let Some(data) = data {
        let wrapped_data = wrap_eventmodeling_label(&data, measurer, style).replace(' ', "&nbsp;");
        html.push_str(
            r#"<br/><br/><code style="text-align: left; display: block;max-width:430px">"#,
        );
        html.push_str(&wrapped_data);
        if frame.data_reference.is_some() {
            html.push_str("<br/>");
        }
        html.push_str("</code>");
        (html, true)
    } else {
        (html, false)
    }
}

fn wrap_eventmodeling_label(label: &str, measurer: &dyn TextMeasurer, style: &TextStyle) -> String {
    wrap_label_like_mermaid_lines(label, measurer, style, TEXT_MAX_WIDTH).join("<br/>")
}

fn normalize_eventmodeling_data_for_measurement(raw: &str) -> String {
    let trimmed = raw.trim();
    let without_outer_braces = trimmed
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(trimmed);
    without_outer_braces.trim().to_string()
}

fn entity_visual_props(theme: &EventModelingTheme, entity_type: &str) -> VisualProps {
    match entity_type {
        "ui" => VisualProps {
            fill: theme.ui_fill.clone(),
            stroke: theme.ui_stroke.clone(),
        },
        "pcr" | "processor" => VisualProps {
            fill: theme.processor_fill.clone(),
            stroke: theme.processor_stroke.clone(),
        },
        "rmo" | "readmodel" => VisualProps {
            fill: theme.read_model_fill.clone(),
            stroke: theme.read_model_stroke.clone(),
        },
        "cmd" | "command" => VisualProps {
            fill: theme.command_fill.clone(),
            stroke: theme.command_stroke.clone(),
        },
        "evt" | "event" => VisualProps {
            fill: theme.event_fill.clone(),
            stroke: theme.event_stroke.clone(),
        },
        _ => VisualProps {
            fill: "red".to_string(),
            stroke: "black".to_string(),
        },
    }
}

fn eventmodeling_config(effective_config: &Value) -> EventModelingConfig {
    EventModelingConfig {
        padding: config_f64(effective_config, &["eventmodeling", "padding"])
            .unwrap_or(30.0)
            .max(0.0),
        use_max_width: config_bool(effective_config, &["eventmodeling", "useMaxWidth"])
            .unwrap_or(true),
    }
}

#[derive(Debug, Clone, Copy)]
struct BoundsAcc {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
    has_value: bool,
}

impl BoundsAcc {
    fn new() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            has_value: false,
        }
    }

    fn include_point(&mut self, x: f64, y: f64) {
        if !self.has_value {
            self.min_x = x;
            self.max_x = x;
            self.min_y = y;
            self.max_y = y;
            self.has_value = true;
            return;
        }
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn include_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.include_point(x, y);
        self.include_point(x + width, y + height);
    }

    fn finish(self, padding: f64) -> Option<Bounds> {
        self.has_value.then_some(Bounds {
            min_x: self.min_x - padding,
            min_y: self.min_y - padding,
            max_x: self.max_x + padding,
            max_y: self.max_y + padding,
        })
    }
}
