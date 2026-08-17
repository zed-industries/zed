use super::super::*;

pub(super) fn class_marker_name(ty: i32, is_start: bool) -> Option<&'static str> {
    // Mermaid class diagram relationType constants.
    // -1 = none, 0 = aggregation, 1 = extension, 2 = composition, 3 = dependency, 4 = lollipop
    match ty {
        0 => Some(if is_start {
            "aggregationStart"
        } else {
            "aggregationEnd"
        }),
        1 => Some(if is_start {
            "extensionStart"
        } else {
            "extensionEnd"
        }),
        2 => Some(if is_start {
            "compositionStart"
        } else {
            "compositionEnd"
        }),
        3 => Some(if is_start {
            "dependencyStart"
        } else {
            "dependencyEnd"
        }),
        4 => Some(if is_start {
            "lollipopStart"
        } else {
            "lollipopEnd"
        }),
        _ => None,
    }
}

pub(super) fn class_markers(out: &mut String, diagram_id: &str, diagram_marker_class: &str) {
    // Match Mermaid unified output: multiple <defs> wrappers, one marker each.
    struct MarkerContext<'a> {
        out: &'a mut String,
        diagram_id: &'a str,
        diagram_marker_class: &'a str,
    }

    enum MarkerShape<'a> {
        Path(&'a str),
        PathWithViewBox(&'a str, &'a str),
        Polygon(&'a str),
        Circle { stroke_width: Option<&'a str> },
    }

    struct MarkerSpec<'a> {
        name: &'a str,
        kind: &'a str,
        ref_x: &'a str,
        ref_y: &'a str,
        marker_w: &'a str,
        marker_h: &'a str,
        marker_units: Option<&'a str>,
        view_box: Option<&'a str>,
        wrap_defs: bool,
        shape: MarkerShape<'a>,
    }

    fn marker(ctx: &mut MarkerContext<'_>, spec: MarkerSpec<'_>) {
        if spec.wrap_defs {
            ctx.out.push_str("<defs>");
        }
        match spec.shape {
            MarkerShape::Path(d) | MarkerShape::PathWithViewBox(d, _) => {
                let _ = write!(
                    ctx.out,
                    r#"<marker id="{}_{}-{}" class="marker {} {}" refX="{}" refY="{}" markerWidth="{}" markerHeight="{}" orient="auto""#,
                    escape_xml_display(ctx.diagram_id),
                    escape_xml_display(ctx.diagram_marker_class),
                    escape_xml_display(spec.name),
                    escape_xml_display(spec.kind),
                    escape_xml_display(ctx.diagram_marker_class),
                    spec.ref_x,
                    spec.ref_y,
                    spec.marker_w,
                    spec.marker_h,
                );
                if let Some(marker_units) = spec.marker_units {
                    let _ = write!(ctx.out, r#" markerUnits="{}""#, marker_units);
                }
                if let Some(view_box) = spec.view_box {
                    let _ = write!(ctx.out, r#" viewBox="{}""#, view_box);
                }
                if let MarkerShape::PathWithViewBox(_, path_view_box) = spec.shape {
                    let _ = write!(
                        ctx.out,
                        r#"><path d="{}" viewBox="{}"/></marker>"#,
                        escape_xml_display(d),
                        path_view_box
                    );
                } else {
                    let _ = write!(
                        ctx.out,
                        r#"><path d="{}"/></marker>"#,
                        escape_xml_display(d)
                    );
                }
            }
            MarkerShape::Polygon(points) => {
                let _ = write!(
                    ctx.out,
                    r#"<marker id="{}_{}-{}" class="marker {} {}" refX="{}" refY="{}" markerWidth="{}" markerHeight="{}" orient="auto""#,
                    escape_xml_display(ctx.diagram_id),
                    escape_xml_display(ctx.diagram_marker_class),
                    escape_xml_display(spec.name),
                    escape_xml_display(spec.kind),
                    escape_xml_display(ctx.diagram_marker_class),
                    spec.ref_x,
                    spec.ref_y,
                    spec.marker_w,
                    spec.marker_h,
                );
                if let Some(marker_units) = spec.marker_units {
                    let _ = write!(ctx.out, r#" markerUnits="{}""#, marker_units);
                }
                if let Some(view_box) = spec.view_box {
                    let _ = write!(ctx.out, r#" viewBox="{}""#, view_box);
                }
                let _ = write!(
                    ctx.out,
                    r#"><polygon points="{}"/></marker>"#,
                    escape_xml_display(points)
                );
            }
            MarkerShape::Circle { stroke_width } => {
                let _ = write!(
                    ctx.out,
                    r#"<marker id="{}_{}-{}" class="marker {} {}" refX="{}" refY="{}" markerWidth="{}" markerHeight="{}" orient="auto""#,
                    escape_xml_display(ctx.diagram_id),
                    escape_xml_display(ctx.diagram_marker_class),
                    escape_xml_display(spec.name),
                    escape_xml_display(spec.kind),
                    escape_xml_display(ctx.diagram_marker_class),
                    spec.ref_x,
                    spec.ref_y,
                    spec.marker_w,
                    spec.marker_h,
                );
                if let Some(marker_units) = spec.marker_units {
                    let _ = write!(ctx.out, r#" markerUnits="{}""#, marker_units);
                }
                if let Some(view_box) = spec.view_box {
                    let _ = write!(ctx.out, r#" viewBox="{}""#, view_box);
                }
                ctx.out
                    .push_str(r#"><circle fill="transparent" cx="7" cy="7" r="6""#);
                if let Some(stroke_width) = stroke_width {
                    let _ = write!(ctx.out, r#" stroke-width="{}""#, stroke_width);
                }
                ctx.out.push_str("/></marker>");
            }
        }
        if spec.wrap_defs {
            ctx.out.push_str("</defs>");
        }
    }

    let mut ctx = MarkerContext {
        out,
        diagram_id,
        diagram_marker_class,
    };

    marker(
        &mut ctx,
        MarkerSpec {
            name: "aggregationStart",
            kind: "aggregation",
            ref_x: "18",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "aggregationEnd",
            kind: "aggregation",
            ref_x: "1",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "aggregationStart-margin",
            kind: "aggregation",
            ref_x: "15",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "aggregationEnd-margin",
            kind: "aggregation",
            ref_x: "1",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );

    marker(
        &mut ctx,
        MarkerSpec {
            name: "extensionStart",
            kind: "extension",
            ref_x: "18",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 1,7 L18,13 V 1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "extensionEnd",
            kind: "extension",
            ref_x: "1",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 1,1 V 13 L18,7 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "extensionStart-margin",
            kind: "extension",
            ref_x: "18",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: Some("0 0 20 14"),
            wrap_defs: false,
            shape: MarkerShape::Polygon("10,7 18,13 18,1"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "extensionEnd-margin",
            kind: "extension",
            ref_x: "9",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: Some("0 0 20 14"),
            wrap_defs: true,
            shape: MarkerShape::Polygon("10,1 10,13 18,7"),
        },
    );

    marker(
        &mut ctx,
        MarkerSpec {
            name: "compositionStart",
            kind: "composition",
            ref_x: "18",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "compositionEnd",
            kind: "composition",
            ref_x: "1",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "compositionStart-margin",
            kind: "composition",
            ref_x: "15",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::PathWithViewBox("M 18,7 L9,13 L1,7 L9,1 Z", "0 0 15 15"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "compositionEnd-margin",
            kind: "composition",
            ref_x: "3.5",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L1,7 L9,1 Z"),
        },
    );

    marker(
        &mut ctx,
        MarkerSpec {
            name: "dependencyStart",
            kind: "dependency",
            ref_x: "6",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 5,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "dependencyEnd",
            kind: "dependency",
            ref_x: "13",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L14,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "dependencyStart-margin",
            kind: "dependency",
            ref_x: "4",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 5,7 L9,13 L1,7 L9,1 Z"),
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "dependencyEnd-margin",
            kind: "dependency",
            ref_x: "16",
            ref_y: "7",
            marker_w: "20",
            marker_h: "28",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Path("M 18,7 L9,13 L14,7 L9,1 Z"),
        },
    );

    marker(
        &mut ctx,
        MarkerSpec {
            name: "lollipopStart",
            kind: "lollipop",
            ref_x: "13",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Circle { stroke_width: None },
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "lollipopEnd",
            kind: "lollipop",
            ref_x: "1",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: None,
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Circle { stroke_width: None },
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "lollipopStart-margin",
            kind: "lollipop",
            ref_x: "13",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Circle {
                stroke_width: Some("2"),
            },
        },
    );
    marker(
        &mut ctx,
        MarkerSpec {
            name: "lollipopEnd-margin",
            kind: "lollipop",
            ref_x: "1",
            ref_y: "7",
            marker_w: "190",
            marker_h: "240",
            marker_units: Some("userSpaceOnUse"),
            view_box: None,
            wrap_defs: true,
            shape: MarkerShape::Circle {
                stroke_width: Some("2"),
            },
        },
    );
}

pub(super) fn push_class_shadow_defs(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    let flood_color = effective_config_value
        .get("theme")
        .and_then(|v| v.as_str())
        .filter(|theme| theme.contains("dark"))
        .map(|_| "#FFFFFF")
        .unwrap_or("#000000");
    let diagram_id = escape_xml(diagram_id);
    let _ = write!(
        out,
        r#"<defs><filter id="{}-drop-shadow" height="130%" width="130%"><feDropShadow dx="4" dy="4" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs><defs><filter id="{}-drop-shadow-small" height="150%" width="150%"><feDropShadow dx="2" dy="2" stdDeviation="0" flood-opacity="0.06" flood-color="{}"/></filter></defs>"#,
        diagram_id.as_str(),
        flood_color,
        diagram_id.as_str(),
        flood_color
    );
}

pub(super) fn push_class_gradient(
    out: &mut String,
    diagram_id: &str,
    effective_config_value: &serde_json::Value,
) {
    if !config_bool(effective_config_value, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return;
    }

    let gradient_start =
        config_string(effective_config_value, &["themeVariables", "gradientStart"])
            .or_else(|| {
                config_string(
                    effective_config_value,
                    &["themeVariables", "primaryBorderColor"],
                )
            })
            .unwrap_or_else(|| "#9370DB".to_string());
    let gradient_stop = config_string(effective_config_value, &["themeVariables", "gradientStop"])
        .or_else(|| {
            config_string(
                effective_config_value,
                &["themeVariables", "secondaryBorderColor"],
            )
        })
        .unwrap_or_else(|| gradient_start.clone());

    let diagram_id = escape_xml(diagram_id);
    let gradient_start = escape_xml(&gradient_start);
    let gradient_stop = escape_xml(&gradient_stop);
    let _ = write!(
        out,
        r#"<linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient>"#,
        diagram_id.as_str(),
        gradient_start.as_str(),
        gradient_stop.as_str()
    );
}
