use super::{Node, TitleKind};

pub(super) fn parse_shape_data_yaml(
    yaml_body: &str,
) -> std::result::Result<serde_yaml::Value, String> {
    let yaml_data = if yaml_body.contains('\n') {
        format!("{yaml_body}\n")
    } else {
        format!("{{\n{yaml_body}\n}}")
    };
    serde_yaml::from_str(&yaml_data).map_err(|e| format!("{e}"))
}

const MERMAID_SHAPES_11_12_2: &[&str] = &[
    "anchor",
    "bang",
    "bolt",
    "bow-rect",
    "bow-tie-rectangle",
    "brace",
    "brace-l",
    "brace-r",
    "braces",
    "card",
    "choice",
    "circ",
    "circle",
    "classBox",
    "cloud",
    "collate",
    "com-link",
    "comment",
    "cross-circ",
    "crossed-circle",
    "curv-trap",
    "curved-trapezoid",
    "cyl",
    "cylinder",
    "das",
    "data-store",
    "database",
    "datastore",
    "db",
    "dbl-circ",
    "decision",
    "defaultMindmapNode",
    "delay",
    "diam",
    "diamond",
    "disk",
    "display",
    "div-proc",
    "div-rect",
    "divided-process",
    "divided-rectangle",
    "doc",
    "docs",
    "document",
    "documents",
    "double-circle",
    "doublecircle",
    "erBox",
    "event",
    "extract",
    "f-circ",
    "filled-circle",
    "flag",
    "flip-tri",
    "flipped-triangle",
    "fork",
    "forkJoin",
    "fr-circ",
    "fr-rect",
    "framed-circle",
    "framed-rectangle",
    "h-cyl",
    "half-rounded-rectangle",
    "hex",
    "hexagon",
    "horizontal-cylinder",
    "hourglass",
    "icon",
    "iconCircle",
    "iconRounded",
    "iconSquare",
    "imageSquare",
    "in-out",
    "internal-storage",
    "inv-trapezoid",
    "inv_trapezoid",
    "join",
    "junction",
    "kanbanItem",
    "labelRect",
    "lean-l",
    "lean-left",
    "lean-r",
    "lean-right",
    "lean_left",
    "lean_right",
    "lightning-bolt",
    "lin-cyl",
    "lin-doc",
    "lin-proc",
    "lin-rect",
    "lined-cylinder",
    "lined-document",
    "lined-process",
    "lined-rectangle",
    "loop-limit",
    "manual",
    "manual-file",
    "manual-input",
    "mindmapCircle",
    "notch-pent",
    "notch-rect",
    "notched-pentagon",
    "notched-rectangle",
    "note",
    "odd",
    "out-in",
    "paper-tape",
    "pill",
    "prepare",
    "priority",
    "proc",
    "process",
    "processes",
    "procs",
    "question",
    "rect",
    "rectWithTitle",
    "rect_left_inv_arrow",
    "rectangle",
    "requirementBox",
    "rounded",
    "roundedRect",
    "shaded-process",
    "sl-rect",
    "sloped-rectangle",
    "sm-circ",
    "small-circle",
    "squareRect",
    "st-doc",
    "st-rect",
    "stacked-document",
    "stacked-rectangle",
    "stadium",
    "start",
    "state",
    "stateEnd",
    "stateStart",
    "stop",
    "stored-data",
    "subproc",
    "subprocess",
    "subroutine",
    "summary",
    "tag-doc",
    "tag-proc",
    "tag-rect",
    "tagged-document",
    "tagged-process",
    "tagged-rectangle",
    "terminal",
    "text",
    "trap-b",
    "trap-t",
    "trapezoid",
    "trapezoid-bottom",
    "trapezoid-top",
    "tri",
    "triangle",
    "win-pane",
    "window-pane",
];

fn is_valid_shape_11_12_2(shape: &str) -> bool {
    MERMAID_SHAPES_11_12_2.binary_search(&shape).is_ok()
}

pub(super) fn yaml_to_string(v: &serde_yaml::Value) -> Option<String> {
    match v {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub(super) fn yaml_to_bool(v: &serde_yaml::Value) -> Option<bool> {
    match v {
        serde_yaml::Value::Bool(b) => Some(*b),
        serde_yaml::Value::String(s) => match s.trim() {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

fn yaml_to_f64(v: &serde_yaml::Value) -> Option<f64> {
    match v {
        serde_yaml::Value::Number(n) => n.as_f64(),
        serde_yaml::Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

fn sanitize_shape_label_type(label_type: Option<&str>) -> TitleKind {
    match label_type {
        Some("text") => TitleKind::Text,
        Some("string") => TitleKind::String,
        Some("markdown") => TitleKind::Markdown,
        _ => TitleKind::Markdown,
    }
}

pub(super) fn apply_shape_data_to_node(
    node: &mut Node,
    yaml_body: &str,
) -> std::result::Result<(), String> {
    // If shapeData is attached to a node reference, Mermaid has already decided this is a node.
    let v = parse_shape_data_yaml(yaml_body)?;
    let map = match v.as_mapping() {
        Some(m) => m,
        None => return Ok(()),
    };

    let mut provided_label: Option<String> = None;
    let mut provided_label_type: Option<TitleKind> = None;
    for (k, v) in map {
        let Some(key) = k.as_str() else { continue };
        match key {
            "shape" => {
                let Some(shape) = v.as_str() else { continue };
                if shape != shape.to_lowercase() || shape.contains('_') {
                    return Err(format!(
                        "No such shape: {shape}. Shape names should be lowercase."
                    ));
                }
                if !is_valid_shape_11_12_2(shape) {
                    return Err(format!("No such shape: {shape}."));
                }
                node.shape = Some(shape.to_string());
            }
            "label" => {
                if let Some(label) = yaml_to_string(v) {
                    provided_label = Some(label.clone());
                    node.label = Some(label);
                }
            }
            "labelType" => {
                provided_label_type = Some(sanitize_shape_label_type(yaml_to_string(v).as_deref()));
            }
            "icon" => {
                if let Some(icon) = yaml_to_string(v) {
                    node.icon = Some(icon);
                }
            }
            "form" => {
                if let Some(form) = yaml_to_string(v) {
                    node.form = Some(form);
                }
            }
            "pos" => {
                if let Some(pos) = yaml_to_string(v) {
                    node.pos = Some(pos);
                }
            }
            "img" => {
                if let Some(img) = yaml_to_string(v) {
                    node.img = Some(img);
                }
            }
            "constraint" => {
                if let Some(constraint) = yaml_to_string(v) {
                    node.constraint = Some(constraint);
                }
            }
            "w" => {
                if let Some(w) = yaml_to_f64(v) {
                    node.asset_width = Some(w);
                }
            }
            "h" => {
                if let Some(h) = yaml_to_f64(v) {
                    node.asset_height = Some(h);
                }
            }
            _ => {}
        }
    }
    if provided_label.is_some() {
        node.label_type = provided_label_type.unwrap_or(TitleKind::Markdown);
    }

    // Mermaid clears the default label when an icon or img is set without an explicit label.
    let has_visual = node.icon.is_some() || node.img.is_some();
    let label_is_empty_or_missing = provided_label
        .as_deref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true);
    if has_visual && label_is_empty_or_missing {
        let current_text = node.label.as_deref().unwrap_or(node.id.as_str());
        if current_text == node.id {
            node.label = Some(String::new());
            node.label_type = TitleKind::Text;
        }
    }

    Ok(())
}
