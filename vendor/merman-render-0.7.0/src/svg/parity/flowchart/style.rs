//! Flowchart style compilation helpers.

use super::*;

#[derive(Debug, Clone)]
pub(in crate::svg::parity) struct FlowchartCompiledStyles {
    pub(super) node_style: String,
    pub(super) label_style: String,
    pub(super) label_div_decls: Vec<(String, String)>,
    pub(super) fill: Option<String>,
    pub(super) stroke: Option<String>,
    pub(super) stroke_width: Option<String>,
    pub(super) stroke_dasharray: Option<String>,
}

pub(in crate::svg::parity) fn flowchart_compile_styles(
    class_defs: &IndexMap<String, Vec<String>>,
    classes: &[String],
    inline_styles_a: &[String],
    inline_styles_b: &[String],
) -> FlowchartCompiledStyles {
    // Ported from Mermaid `handDrawnShapeStyles.compileStyles()` / `styles2String()`:
    // - preserve insertion order of the first occurrence of a key
    // - later occurrences override values, without changing order
    #[derive(Default)]
    struct OrderedMap<'a> {
        order: Vec<(&'a str, &'a str)>,
        idx: FxHashMap<&'a str, usize>,
    }
    impl<'a> OrderedMap<'a> {
        fn set(&mut self, k: &'a str, v: &'a str) {
            if let Some(&i) = self.idx.get(k) {
                self.order[i].1 = v;
                return;
            }
            self.idx.insert(k, self.order.len());
            self.order.push((k, v));
        }
    }

    let mut m: OrderedMap<'_> = OrderedMap::default();

    for c in classes {
        let Some(decls) = class_defs.get(c) else {
            continue;
        };
        for d in decls {
            let Some((k, v)) = parse_style_decl(d) else {
                continue;
            };
            m.set(k, v);
        }
    }

    for d in inline_styles_a.iter().chain(inline_styles_b.iter()) {
        let Some((k, v)) = parse_style_decl(d) else {
            continue;
        };
        m.set(k, v);
    }

    let mut node_style = String::new();
    let mut label_style = String::new();

    let mut label_div_decls: Vec<(String, String)> = Vec::new();

    let mut fill: Option<String> = None;
    let mut stroke: Option<String> = None;
    let mut stroke_width: Option<String> = None;
    let mut stroke_dasharray: Option<String> = None;

    for (k, v) in &m.order {
        let k = *k;
        let v = *v;
        if is_text_style_key(k) {
            if !label_style.is_empty() {
                label_style.push(';');
            }
            let _ = write!(&mut label_style, "{k}:{v} !important");
            label_div_decls.push((k.to_string(), v.to_string()));
        } else {
            if !node_style.is_empty() {
                node_style.push(';');
            }
            let _ = write!(&mut node_style, "{k}:{v} !important");
        }
        match k {
            "fill" => fill = Some(v.to_string()),
            "stroke" => stroke = Some(v.to_string()),
            "stroke-width" => stroke_width = Some(v.to_string()),
            "stroke-dasharray" => stroke_dasharray = Some(v.to_string()),
            _ => {}
        }
    }

    FlowchartCompiledStyles {
        node_style,
        label_style,
        label_div_decls,
        fill,
        stroke,
        stroke_width,
        stroke_dasharray,
    }
}

pub(in crate::svg::parity) fn flowchart_compile_node_styles(
    class_defs: &IndexMap<String, Vec<String>>,
    classes: &[String],
    inline_styles_a: &[String],
    inline_styles_b: &[String],
) -> FlowchartCompiledStyles {
    let effective_classes =
        crate::flowchart::flowchart_effective_node_class_names(class_defs, classes)
            .into_iter()
            .map(|class| class.to_string())
            .collect::<Vec<_>>();
    flowchart_compile_styles(
        class_defs,
        &effective_classes,
        inline_styles_a,
        inline_styles_b,
    )
}

pub(in crate::svg::parity) fn flowchart_label_div_style_prefix(
    styles: &FlowchartCompiledStyles,
    color_as_rgb: bool,
) -> String {
    fn parse_hex_rgb_u8(v: &str) -> Option<(u8, u8, u8)> {
        let v = v.trim();
        let hex = v.strip_prefix('#')?;
        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some((r, g, b))
            }
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some((r, g, b))
            }
            _ => None,
        }
    }

    fn div_style_survives_mermaid_overrides(key: &str) -> bool {
        !matches!(key, "line-height" | "text-align" | "white-space")
    }

    let mut out = String::new();
    for (key, value) in &styles.label_div_decls {
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() || !div_style_survives_mermaid_overrides(key) {
            continue;
        }
        if key == "color" {
            if color_as_rgb {
                if let Some((r, g, b)) = parse_hex_rgb_u8(value) {
                    let _ = write!(&mut out, "color: rgb({r}, {g}, {b}) !important; ");
                } else {
                    let _ = write!(
                        &mut out,
                        "color: {} !important; ",
                        value.to_ascii_lowercase()
                    );
                }
            } else {
                let _ = write!(
                    &mut out,
                    "color: {} !important; ",
                    value.to_ascii_lowercase()
                );
            }
        } else {
            let _ = write!(&mut out, "{key}: {value} !important; ");
        }
    }
    out
}
