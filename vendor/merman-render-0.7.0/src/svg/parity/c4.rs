use super::*;
use merman_core::diagrams::c4::{
    C4BoundaryRenderModel, C4DiagramRenderModel, C4RelRenderModel, C4ShapeRenderModel,
};
type C4SvgModelShape = C4ShapeRenderModel;
type C4SvgModelBoundary = C4BoundaryRenderModel;
type C4SvgModelRel = C4RelRenderModel;

// C4 diagram SVG renderer implementation (split from parity.rs).

fn c4_css(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let mut out = parts.css_prefix;
    let person_border = theme_color(
        effective_config,
        "personBorder",
        "hsl(240, 60%, 86.2745098039%)",
    );
    let person_bkg = theme_color(effective_config, "personBkg", "#ECECFF");
    let _ = write!(
        &mut out,
        r#"#{} .person{{stroke:{};fill:{};}}"#,
        id, person_border, person_bkg
    );
    out.push_str(&parts.root_rule);
    out
}

fn c4_config_string(cfg: &serde_json::Value, key: &str) -> Option<String> {
    config_string(cfg, &["c4", key])
}

fn c4_config_color(cfg: &serde_json::Value, key: &str, fallback: &str) -> String {
    c4_config_string(cfg, key).unwrap_or_else(|| fallback.to_string())
}

fn c4_config_font_family(cfg: &serde_json::Value, type_key: &str) -> String {
    c4_config_string(cfg, &format!("{type_key}FontFamily"))
        .map(|s| s.trim().trim_end_matches(';').trim().to_string())
        .unwrap_or_else(|| r#""Open Sans", sans-serif"#.to_string())
}

fn c4_config_font_size(cfg: &serde_json::Value, type_key: &str, fallback: f64) -> f64 {
    config_f64(cfg, &["c4", &format!("{type_key}FontSize")]).unwrap_or(fallback)
}

fn c4_config_font_weight(cfg: &serde_json::Value, type_key: &str) -> String {
    c4_config_string(cfg, &format!("{type_key}FontWeight")).unwrap_or_else(|| "normal".to_string())
}

fn c4_type_text_length_px(type_c4_shape: &str) -> Option<f64> {
    match type_c4_shape {
        "component" => Some(73.0),
        "component_db" => Some(93.0),
        "container" => Some(63.0),
        "container_db" => Some(83.0),
        "external_component" => Some(122.0),
        "external_component_db" => Some(142.0),
        "external_container" => Some(112.0),
        "external_container_db" => Some(132.0),
        "external_container_queue" => Some(152.0),
        "external_person" => Some(100.0),
        "external_system" => Some(101.0),
        "external_system_db" => Some(121.0),
        "external_system_queue" => Some(141.0),
        "person" => Some(50.0),
        "system" => Some(52.0),
        "system_db" => Some(72.0),
        "system_queue" => Some(92.0),
        _ => None,
    }
}

struct C4TspanText<'a> {
    content: &'a str,
    x: f64,
    y: f64,
    width: f64,
    font_family: &'a str,
    font_size: f64,
    font_weight: &'a str,
    attrs: &'a [(&'a str, &'a str)],
}

fn c4_write_text_by_tspan(out: &mut String, text: C4TspanText<'_>) {
    let C4TspanText {
        content,
        x,
        y,
        width,
        font_family,
        font_size,
        font_weight,
        attrs,
    } = text;
    let x = x + width / 2.0;
    let mut style = String::new();
    let _ = write!(
        &mut style,
        "text-anchor: middle; font-size: {}px; font-weight: {}; font-family: {};",
        fmt(font_size.max(1.0)),
        font_weight,
        font_family
    );

    let normalized = content
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("<br>", "\n");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let n = lines.len().max(1) as f64;

    for (i, line) in lines.iter().enumerate() {
        let dy = (i as f64) * font_size - (font_size * (n - 1.0)) / 2.0;
        let dy_s = fmt(dy);

        let _ = write!(
            out,
            r#"<text x="{}" y="{}" dominant-baseline="middle""#,
            fmt(x),
            fmt(y)
        );
        for (k, v) in attrs {
            let _ = write!(out, r#" {k}="{v}""#);
        }
        let _ = write!(
            out,
            r#" style="{}"><tspan dy="{}" alignment-baseline="mathematical">{}</tspan></text>"#,
            escape_attr(&style),
            dy_s,
            escape_xml(line)
        );
    }
}

pub(super) fn render_c4_diagram_svg_typed(
    layout: &crate::model::C4DiagramLayout,
    model: &C4DiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    _measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let diagram_id_esc = escape_xml(diagram_id);

    let diagram_margin_x = config_f64(effective_config, &["c4", "diagramMarginX"]).unwrap_or(50.0);
    let diagram_margin_y = config_f64(effective_config, &["c4", "diagramMarginY"]).unwrap_or(10.0);
    let use_max_width = effective_config
        .get("c4")
        .and_then(|v| v.get("useMaxWidth"))
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let bounds = layout.bounds.clone().unwrap_or(Bounds {
        min_x: diagram_margin_x,
        min_y: diagram_margin_y,
        max_x: diagram_margin_x + layout.width.max(1.0),
        max_y: diagram_margin_y + layout.height.max(1.0),
    });
    let box_w = (bounds.max_x - bounds.min_x).max(1.0);
    let box_h = (bounds.max_y - bounds.min_y).max(1.0);
    let width = (box_w + 2.0 * diagram_margin_x).max(1.0);
    let height = (box_h + 2.0 * diagram_margin_y).max(1.0);

    let title = diagram_title
        .map(|s| s.to_string())
        .or_else(|| layout.title.clone())
        .or_else(|| model.title.clone())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let extra_vert_for_title = if title.is_some() { 60.0 } else { 0.0 };

    let viewbox_x = bounds.min_x - diagram_margin_x;
    let viewbox_y = -(diagram_margin_y + extra_vert_for_title);

    let aria_roledescription = options.aria_roledescription.as_deref().unwrap_or("c4");
    let aria_roledescription_attr = escape_attr(aria_roledescription);

    let mut root_viewbox = format!(
        "{} {} {} {}",
        fmt(viewbox_x),
        fmt(viewbox_y),
        fmt(width),
        fmt(height + extra_vert_for_title)
    );
    let mut root_max_w = fmt_string(width);
    let mut root_w_attr = fmt_string(width);
    let mut root_h_attr = fmt_string(height + extra_vert_for_title);

    apply_root_viewport_override(
        diagram_id,
        &mut root_viewbox,
        &mut root_w_attr,
        &mut root_h_attr,
        &mut root_max_w,
        crate::generated::c4_root_overrides_11_12_2::lookup_c4_root_viewport_override,
    );

    let aria_describedby = model
        .acc_descr
        .as_ref()
        .map(|s| s.trim_end_matches('\n'))
        .filter(|s| !s.trim().is_empty())
        .map(|_| format!("chart-desc-{diagram_id_esc}"));
    let aria_labelledby = model
        .acc_title
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|_| format!("chart-title-{diagram_id_esc}"));

    let mut out = String::new();
    if use_max_width {
        let style_attr = format!("max-width: {root_max_w}px; background-color: white;");
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Percent100,
                style_attr: Some(style_attr.as_str()),
                viewbox_attr: Some(root_viewbox.as_str()),
                aria_labelledby: aria_labelledby.as_deref(),
                aria_describedby: aria_describedby.as_deref(),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, &aria_roledescription_attr)
            },
        );
    } else {
        root_svg::push_svg_root_open(
            &mut out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Fixed(&root_w_attr),
                height_attr: Some(&root_h_attr),
                style_attr: Some("background-color: white;"),
                viewbox_attr: Some(root_viewbox.as_str()),
                aria_labelledby: aria_labelledby.as_deref(),
                aria_describedby: aria_describedby.as_deref(),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, &aria_roledescription_attr)
            },
        );
    }

    if let Some(title) = model
        .acc_title
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{id}">{text}</title>"#,
            id = diagram_id_esc,
            text = escape_xml(title)
        );
    }
    if let Some(descr) = model
        .acc_descr
        .as_deref()
        .map(|s| s.trim_end_matches('\n'))
        .filter(|s| !s.trim().is_empty())
    {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{id}">{text}</desc>"#,
            id = diagram_id_esc,
            text = escape_xml(descr)
        );
    }

    let css = c4_css(diagram_id, effective_config);
    let _ = write!(&mut out, r#"<style>{}</style>"#, css);
    out.push_str("<g/>");

    const C4_DATABASE_SYMBOL_D_11_12_2: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/c4_database_d_11_12_2.txt"
    ));

    let _ = write!(
        &mut out,
        r#"<defs><symbol id="{}" width="24" height="24"><path transform="scale(.5)" d="M2 2v13h20v-13h-20zm18 11h-16v-9h16v9zm-10.228 6l.466-1h3.524l.467 1h-4.457zm14.228 3h-24l2-6h2.104l-1.33 4h18.45l-1.297-4h2.073l2 6zm-5-10h-14v-7h14v7z"/></symbol></defs>"#,
        escape_attr(&scoped_svg_id(diagram_id, "computer"))
    );
    let _ = write!(
        &mut out,
        r#"<defs><symbol id="{}" fill-rule="evenodd" clip-rule="evenodd"><path transform="scale(.5)" d="{}"/></symbol></defs>"#,
        escape_attr(&scoped_svg_id(diagram_id, "database")),
        escape_attr(C4_DATABASE_SYMBOL_D_11_12_2.trim())
    );
    let _ = write!(
        &mut out,
        r#"<defs><symbol id="{}" width="24" height="24"><path transform="scale(.5)" d="M12 2c5.514 0 10 4.486 10 10s-4.486 10-10 10-10-4.486-10-10 4.486-10 10-10zm0-2c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm5.848 12.459c.202.038.202.333.001.372-1.907.361-6.045 1.111-6.547 1.111-.719 0-1.301-.582-1.301-1.301 0-.512.77-5.447 1.125-7.445.034-.192.312-.181.343.014l.985 6.238 5.394 1.011z"/></symbol></defs>"#,
        escape_attr(&scoped_svg_id(diagram_id, "clock"))
    );

    let mut shape_meta: std::collections::HashMap<&str, &C4SvgModelShape> =
        std::collections::HashMap::new();
    for s in &model.shapes {
        shape_meta.insert(s.alias.as_str(), s);
    }
    let mut boundary_meta: std::collections::HashMap<&str, &C4SvgModelBoundary> =
        std::collections::HashMap::new();
    for b in &model.boundaries {
        boundary_meta.insert(b.alias.as_str(), b);
    }
    let mut rel_meta: std::collections::HashMap<(&str, &str), &C4SvgModelRel> =
        std::collections::HashMap::new();
    for r in &model.rels {
        rel_meta.insert((r.from_alias.as_str(), r.to_alias.as_str()), r);
    }

    const PERSON_IMG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAIAAADYYG7QAAACD0lEQVR4Xu2YoU4EMRCGT+4j8Ai8AhaH4QHgAUjQuFMECUgMIUgwJAgMhgQsAYUiJCiQIBBY+EITsjfTdme6V24v4c8vyGbb+ZjOtN0bNcvjQXmkH83WvYBWto6PLm6v7p7uH1/w2fXD+PBycX1Pv2l3IdDm/vn7x+dXQiAubRzoURa7gRZWd0iGRIiJbOnhnfYBQZNJjNbuyY2eJG8fkDE3bbG4ep6MHUAsgYxmE3nVs6VsBWJSGccsOlFPmLIViMzLOB7pCVO2AtHJMohH7Fh6zqitQK7m0rJvAVYgGcEpe//PLdDz65sM4pF9N7ICcXDKIB5Nv6j7tD0NoSdM2QrU9Gg0ewE1LqBhHR3BBdvj2vapnidjHxD/q6vd7Pvhr31AwcY8eXMTXAKECZZJFXuEq27aLgQK5uLMohCenGGuGewOxSjBvYBqeG6B+Nqiblggdjnc+ZXDy+FNFpFzw76O3UBAROuXh6FoiAcf5g9eTvUgzy0nWg6I8cXHRUpg5bOVBCo+KDpFajOf23GgPme7RSQ+lacIENUgJ6gg1k6HjgOlqnLqip4tEuhv0hNEMXUD0clyXE3p6pZA0S2nnvTlXwLJEZWlb7cTQH1+USgTN4VhAenm/wea1OCAOmqo6fE1WCb9WSKBah+rbUWPWAmE2Rvk0ApiB45eOyNAzU8xcTvj8KvkKEoOaIYeHNA3ZuygAvFMUO0AAAAASUVORK5CYII=";
    const EXTERNAL_PERSON_IMG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAwCAIAAADYYG7QAAAB6ElEQVR4Xu2YLY+EMBCG9+dWr0aj0Wg0Go1Go0+j8Xdv2uTCvv1gpt0ebHKPuhDaeW4605Z9mJvx4AdXUyTUdd08z+u6flmWZRnHsWkafk9DptAwDPu+f0eAYtu2PEaGWuj5fCIZrBAC2eLBAnRCsEkkxmeaJp7iDJ2QMDdHsLg8SxKFEJaAo8lAXnmuOFIhTMpxxKATebo4UiFknuNo4OniSIXQyRxEA3YsnjGCVEjVXD7yLUAqxBGUyPv/Y4W2beMgGuS7kVQIBycH0fD+oi5pezQETxdHKmQKGk1eQEYldK+jw5GxPfZ9z7Mk0Qnhf1W1m3w//EUn5BDmSZsbR44QQLBEqrBHqOrmSKaQAxdnLArCrxZcM7A7ZKs4ioRq8LFC+NpC3WCBJsvpVw5edm9iEXFuyNfxXAgSwfrFQ1c0iNda8AdejvUgnktOtJQQxmcfFzGglc5WVCj7oDgFqU18boeFSs52CUh8LE8BIVQDT1ABrB0HtgSEYlX5doJnCwv9TXocKCaKbnwhdDKPq4lf3SwU3HLq4V/+WYhHVMa/3b4IlfyikAduCkcBc7mQ3/z/Qq/cTuikhkzB12Ae/mcJC9U+Vo8Ej1gWAtgbeGgFsAMHr50BIWOLCbezvhpBFUdY6EJuJ/QDW0XoMX60zZ0AAAAASUVORK5CYII=";

    for s in &layout.shapes {
        let meta = shape_meta.get(s.alias.as_str()).copied();
        let (default_bg_color, default_border_color) = if s.type_c4_shape.starts_with("external_") {
            ("#999999", "#8A8A8A")
        } else {
            ("#08427B", "#073B6F")
        };
        let bg_color = meta.and_then(|m| m.bg_color.clone()).unwrap_or_else(|| {
            c4_config_color(
                effective_config,
                &format!("{}_bg_color", s.type_c4_shape),
                default_bg_color,
            )
        });
        let border_color = meta
            .and_then(|m| m.border_color.clone())
            .unwrap_or_else(|| {
                c4_config_color(
                    effective_config,
                    &format!("{}_border_color", s.type_c4_shape),
                    default_border_color,
                )
            });
        let font_color = meta
            .and_then(|m| m.font_color.clone())
            .unwrap_or_else(|| "#FFFFFF".to_string());

        out.push_str(r#"<g class="person-man">"#);

        match s.type_c4_shape.as_str() {
            "system_db"
            | "external_system_db"
            | "container_db"
            | "external_container_db"
            | "component_db"
            | "external_component_db" => {
                let half = s.width / 2.0;
                let d1 = format!(
                    "M{},{}c0,-10 {},-10 {},-10c0,0 {},0 {},10l0,{}c0,10 -{},10 -{},10c0,0 -{},0 -{},-10l0,-{}",
                    fmt(s.x),
                    fmt(s.y),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(s.height),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(s.height)
                );
                let d2 = format!(
                    "M{},{}c0,10 {},10 {},10c0,0 {},0 {},-10",
                    fmt(s.x),
                    fmt(s.y),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half)
                );
                let _ = write!(
                    &mut out,
                    r#"<path fill="{}" stroke-width="0.5" stroke="{}" d="{}"/>"#,
                    escape_attr(&bg_color),
                    escape_attr(&border_color),
                    escape_attr(&d1)
                );
                let _ = write!(
                    &mut out,
                    r#"<path fill="none" stroke-width="0.5" stroke="{}" d="{}"/>"#,
                    escape_attr(&border_color),
                    escape_attr(&d2)
                );
            }
            "system_queue"
            | "external_system_queue"
            | "container_queue"
            | "external_container_queue"
            | "component_queue"
            | "external_component_queue" => {
                let half = s.height / 2.0;
                let d1 = format!(
                    "M{},{}l{},0c5,0 5,{} 5,{}c0,0 0,{} -5,{}l-{},0c-5,0 -5,-{} -5,-{}c0,0 0,-{} 5,-{}",
                    fmt(s.x),
                    fmt(s.y),
                    fmt(s.width),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(s.width),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                );
                let d2 = format!(
                    "M{},{}c-5,0 -5,{} -5,{}c0,{} 5,{} 5,{}",
                    fmt(s.x + s.width),
                    fmt(s.y),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half),
                    fmt(half)
                );
                let _ = write!(
                    &mut out,
                    r#"<path fill="{}" stroke-width="0.5" stroke="{}" d="{}"/>"#,
                    escape_attr(&bg_color),
                    escape_attr(&border_color),
                    escape_attr(&d1)
                );
                let _ = write!(
                    &mut out,
                    r#"<path fill="none" stroke-width="0.5" stroke="{}" d="{}"/>"#,
                    escape_attr(&border_color),
                    escape_attr(&d2)
                );
            }
            _ => {
                let _ = write!(
                    &mut out,
                    r#"<rect x="{}" y="{}" fill="{}" stroke="{}" width="{}" height="{}" rx="2.5" ry="2.5" stroke-width="0.5"/>"#,
                    fmt(s.x),
                    fmt(s.y),
                    escape_attr(&bg_color),
                    escape_attr(&border_color),
                    fmt(s.width),
                    fmt(s.height)
                );
            }
        }

        let type_family = c4_config_font_family(effective_config, &s.type_c4_shape);
        let type_size = c4_config_font_size(effective_config, &s.type_c4_shape, 14.0) - 2.0;
        let type_text_length = c4_type_text_length_px(&s.type_c4_shape)
            .unwrap_or_else(|| s.type_block.width.round().max(0.0));
        let _ = write!(
            &mut out,
            r#"<text fill="{}" font-family="{}" font-size="{}" font-style="italic" lengthAdjust="spacing" textLength="{}" x="{}" y="{}">{}</text>"#,
            escape_attr(&font_color),
            escape_attr(&type_family),
            fmt(type_size.max(1.0)),
            fmt(type_text_length),
            fmt(s.x + s.width / 2.0 - type_text_length / 2.0),
            fmt(s.y + s.type_block.y),
            escape_xml(&format!("<<{}>>", s.type_c4_shape))
        );

        if matches!(s.type_c4_shape.as_str(), "person" | "external_person") {
            let href = if s.type_c4_shape == "external_person" {
                EXTERNAL_PERSON_IMG
            } else {
                PERSON_IMG
            };
            let _ = write!(
                &mut out,
                r#"<image width="48" height="48" x="{}" y="{}" xlink:href="{}"/>"#,
                fmt(s.x + s.width / 2.0 - 24.0),
                fmt(s.y + s.image.y),
                escape_attr(href)
            );
        }

        let label_family = c4_config_font_family(effective_config, &s.type_c4_shape);
        let label_weight = "bold";
        let label_size = c4_config_font_size(effective_config, &s.type_c4_shape, 14.0) + 2.0;
        c4_write_text_by_tspan(
            &mut out,
            C4TspanText {
                content: &s.label.text,
                x: s.x,
                y: s.y + s.label.y,
                width: s.width,
                font_family: &label_family,
                font_size: label_size,
                font_weight: label_weight,
                attrs: &[("fill", &font_color)],
            },
        );

        let body_family = c4_config_font_family(effective_config, &s.type_c4_shape);
        let body_weight = c4_config_font_weight(effective_config, &s.type_c4_shape);
        let body_size = c4_config_font_size(effective_config, &s.type_c4_shape, 14.0);

        if let Some(techn) = &s.techn {
            if !techn.text.trim().is_empty() {
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &techn.text,
                        x: s.x,
                        y: s.y + techn.y,
                        width: s.width,
                        font_family: &body_family,
                        font_size: body_size,
                        font_weight: &body_weight,
                        attrs: &[("fill", &font_color), ("font-style", "italic")],
                    },
                );
            }
        } else if let Some(ty) = &s.ty {
            if !ty.text.trim().is_empty() {
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &ty.text,
                        x: s.x,
                        y: s.y + ty.y,
                        width: s.width,
                        font_family: &body_family,
                        font_size: body_size,
                        font_weight: &body_weight,
                        attrs: &[("fill", &font_color), ("font-style", "italic")],
                    },
                );
            }
        }

        if let Some(descr) = &s.descr {
            if !descr.text.trim().is_empty() {
                let descr_family = c4_config_font_family(effective_config, "person");
                let descr_weight = c4_config_font_weight(effective_config, "person");
                let descr_size = c4_config_font_size(effective_config, "person", 14.0);
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &descr.text,
                        x: s.x,
                        y: s.y + descr.y,
                        width: s.width,
                        font_family: &descr_family,
                        font_size: descr_size,
                        font_weight: &descr_weight,
                        attrs: &[("fill", &font_color)],
                    },
                );
            }
        }

        out.push_str("</g>");
    }

    for b in &layout.boundaries {
        if b.alias == "global" {
            continue;
        }
        let meta = boundary_meta.get(b.alias.as_str()).copied();
        let fill_color = meta
            .and_then(|m| m.bg_color.clone())
            .unwrap_or_else(|| "none".to_string());
        let stroke_color = meta
            .and_then(|m| m.border_color.clone())
            .unwrap_or_else(|| "#444444".to_string());
        let is_node_type = meta.and_then(|m| m.node_type.as_deref()).is_some();

        out.push_str("<g>");
        if is_node_type {
            let _ = write!(
                &mut out,
                r#"<rect x="{}" y="{}" fill="{}" stroke="{}" width="{}" height="{}" rx="2.5" ry="2.5" stroke-width="1"/>"#,
                fmt(b.x),
                fmt(b.y),
                escape_attr(&fill_color),
                escape_attr(&stroke_color),
                fmt(b.width),
                fmt(b.height)
            );
        } else {
            let _ = write!(
                &mut out,
                r#"<rect x="{}" y="{}" fill="{}" stroke="{}" width="{}" height="{}" rx="2.5" ry="2.5" stroke-width="1" stroke-dasharray="7.0,7.0"/>"#,
                fmt(b.x),
                fmt(b.y),
                escape_attr(&fill_color),
                escape_attr(&stroke_color),
                fmt(b.width),
                fmt(b.height)
            );
        }

        let boundary_family = c4_config_font_family(effective_config, "boundary");
        let boundary_weight = "bold";
        let boundary_size = c4_config_font_size(effective_config, "boundary", 14.0) + 2.0;
        c4_write_text_by_tspan(
            &mut out,
            C4TspanText {
                content: &b.label.text,
                x: b.x,
                y: b.y + b.label.y,
                width: b.width,
                font_family: &boundary_family,
                font_size: boundary_size,
                font_weight: boundary_weight,
                attrs: &[("fill", "#444444")],
            },
        );
        if let Some(ty) = &b.ty {
            if !ty.text.trim().is_empty() {
                let boundary_type_weight = c4_config_font_weight(effective_config, "boundary");
                let boundary_type_size = c4_config_font_size(effective_config, "boundary", 14.0);
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &ty.text,
                        x: b.x,
                        y: b.y + ty.y,
                        width: b.width,
                        font_family: &boundary_family,
                        font_size: boundary_type_size,
                        font_weight: &boundary_type_weight,
                        attrs: &[("fill", "#444444")],
                    },
                );
            }
        }
        if let Some(descr) = &b.descr {
            if !descr.text.trim().is_empty() {
                let descr_weight = c4_config_font_weight(effective_config, "boundary");
                let descr_size =
                    (c4_config_font_size(effective_config, "boundary", 14.0) - 2.0).max(1.0);
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &descr.text,
                        x: b.x,
                        y: b.y + descr.y,
                        width: b.width,
                        font_family: &boundary_family,
                        font_size: descr_size,
                        font_weight: &descr_weight,
                        attrs: &[("fill", "#444444")],
                    },
                );
            }
        }

        out.push_str("</g>");
    }

    let arrowhead_id = scoped_svg_id(diagram_id, "arrowhead");
    let arrowend_id = scoped_svg_id(diagram_id, "arrowend");
    let crosshead_id = scoped_svg_id(diagram_id, "crosshead");
    let filled_head_id = scoped_svg_id(diagram_id, "filled-head");
    let arrowhead_url = scoped_svg_url(diagram_id, "arrowhead");
    let arrowend_url = scoped_svg_url(diagram_id, "arrowend");
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{}" refX="9" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 0 0 L 10 5 L 0 10 z"/></marker></defs>"#,
        escape_attr(&arrowhead_id)
    );
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{}" refX="1" refY="5" markerUnits="userSpaceOnUse" markerWidth="12" markerHeight="12" orient="auto"><path d="M 10 0 L 0 5 L 10 10 z"/></marker></defs>"#,
        escape_attr(&arrowend_id)
    );
    let _ = write!(
        &mut out,
        r##"<defs><marker id="{}" markerWidth="15" markerHeight="8" orient="auto" refX="16" refY="4"><path fill="black" stroke="#000000" stroke-width="1px" d="M 9,2 V 6 L16,4 Z" style="stroke-dasharray: 0, 0;"/><path fill="none" stroke="#000000" stroke-width="1px" d="M 0,1 L 6,7 M 6,1 L 0,7" style="stroke-dasharray: 0, 0;"/></marker></defs>"##,
        escape_attr(&crosshead_id)
    );
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{}" refX="18" refY="7" markerWidth="20" markerHeight="28" orient="auto"><path d="M 18,7 L9,13 L14,7 L9,1 Z"/></marker></defs>"#,
        escape_attr(&filled_head_id)
    );

    out.push_str("<g>");
    for (idx, rel) in layout.rels.iter().enumerate() {
        let meta = rel_meta.get(&(rel.from.as_str(), rel.to.as_str())).copied();
        let text_color = meta
            .and_then(|m| m.text_color.clone())
            .unwrap_or_else(|| "#444444".to_string());
        let stroke_color = meta
            .and_then(|m| m.line_color.clone())
            .unwrap_or_else(|| "#444444".to_string());
        let offset_x = rel.offset_x.unwrap_or(0) as f64;
        let offset_y = rel.offset_y.unwrap_or(0) as f64;

        if idx == 0 {
            let _ = write!(
                &mut out,
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke-width="1" stroke="{}""#,
                fmt(rel.start_point.x),
                fmt(rel.start_point.y),
                fmt(rel.end_point.x),
                fmt(rel.end_point.y),
                escape_attr(&stroke_color)
            );
            if rel.rel_type != "rel_b" {
                let _ = write!(&mut out, r#" marker-end="{}""#, escape_attr(&arrowhead_url));
            }
            if rel.rel_type == "birel" || rel.rel_type == "rel_b" {
                let _ = write!(
                    &mut out,
                    r#" marker-start="{}""#,
                    escape_attr(&arrowend_url)
                );
            }
            out.push_str(r#" style="fill: none;"/>"#);
        } else {
            let cx = rel.start_point.x + (rel.end_point.x - rel.start_point.x) / 2.0
                - (rel.end_point.x - rel.start_point.x) / 4.0;
            let cy = rel.start_point.y + (rel.end_point.y - rel.start_point.y) / 2.0;
            let d = format!(
                "M{} {} Q{} {} {} {}",
                fmt(rel.start_point.x),
                fmt(rel.start_point.y),
                fmt(cx),
                fmt(cy),
                fmt(rel.end_point.x),
                fmt(rel.end_point.y)
            );
            let _ = write!(
                &mut out,
                r#"<path fill="none" stroke-width="1" stroke="{}" d="{}""#,
                escape_attr(&stroke_color),
                escape_attr(&d)
            );
            if rel.rel_type != "rel_b" {
                let _ = write!(&mut out, r#" marker-end="{}""#, escape_attr(&arrowhead_url));
            }
            if rel.rel_type == "birel" || rel.rel_type == "rel_b" {
                let _ = write!(
                    &mut out,
                    r#" marker-start="{}""#,
                    escape_attr(&arrowend_url)
                );
            }
            out.push_str("/>");
        }

        let midx = rel.start_point.x.min(rel.end_point.x)
            + (rel.end_point.x - rel.start_point.x).abs() / 2.0
            + offset_x;
        let midy = rel.start_point.y.min(rel.end_point.y)
            + (rel.end_point.y - rel.start_point.y).abs() / 2.0
            + offset_y;

        let message_family = c4_config_font_family(effective_config, "message");
        let message_weight = c4_config_font_weight(effective_config, "message");
        let message_size = c4_config_font_size(effective_config, "message", 12.0);
        c4_write_text_by_tspan(
            &mut out,
            C4TspanText {
                content: &rel.label.text,
                x: midx,
                y: midy,
                width: rel.label.width,
                font_family: &message_family,
                font_size: message_size,
                font_weight: &message_weight,
                attrs: &[("fill", &text_color)],
            },
        );

        if let Some(techn) = &rel.techn {
            if !techn.text.trim().is_empty() {
                let techn_text = format!("[{}]", techn.text);
                c4_write_text_by_tspan(
                    &mut out,
                    C4TspanText {
                        content: &techn_text,
                        x: midx,
                        y: midy + message_size + 5.0,
                        width: rel.label.width.max(techn.width),
                        font_family: &message_family,
                        font_size: message_size,
                        font_weight: &message_weight,
                        attrs: &[("fill", &text_color), ("font-style", "italic")],
                    },
                );
            }
        }
    }
    out.push_str("</g>");

    if let Some(title) = title {
        let title_x = (width - 2.0 * diagram_margin_x) / 2.0 - 4.0 * diagram_margin_x;
        let title_y = bounds.min_y + diagram_margin_y;
        let _ = write!(
            &mut out,
            r#"<text x="{}" y="{}">{}</text>"#,
            fmt(title_x),
            fmt(title_y),
            escape_xml(&title)
        );
    }

    out.push_str("</svg>");
    Ok(out)
}

pub(super) fn render_c4_diagram_svg(
    layout: &crate::model::C4DiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: C4DiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_c4_diagram_svg_typed(
        layout,
        &model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn c4_css_honors_mermaid_11_15_person_theme_options() {
        let css = c4_css(
            "c4",
            &json!({
                "themeVariables": {
                    "personBorder": "#112233",
                    "personBkg": "#445566",
                    "textColor": "#778899"
                }
            }),
        );

        assert!(css.contains("#c4{"));
        assert!(css.contains("fill:#778899;"));
        assert!(css.contains("#c4 .person{stroke:#112233;fill:#445566;}"));
    }

    #[test]
    fn c4_type_text_length_rules_stay_local() {
        let cases = [
            ("component", Some(73.0)),
            ("component_db", Some(93.0)),
            ("container", Some(63.0)),
            ("external_container_queue", Some(152.0)),
            ("external_person", Some(100.0)),
            ("person", Some(50.0)),
            ("system", Some(52.0)),
            ("system_db", Some(72.0)),
            ("system_queue", Some(92.0)),
            ("unknown", None),
        ];

        for (shape, expected) in cases {
            assert_eq!(c4_type_text_length_px(shape), expected, "{shape}");
        }
    }
}
