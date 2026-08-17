use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct DiagramBounds {
    pub(super) min_x: f64,
    pub(super) min_y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
}

impl DiagramBounds {
    pub(super) fn from_view_box(min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        Self {
            min_x: finite_or(min_x, 0.0),
            min_y: finite_or(min_y, 0.0),
            width: viewport_dimension(width),
            height: viewport_dimension(height),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ViewBox {
    pub(super) min_x: f64,
    pub(super) min_y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
}

impl ViewBox {
    pub(super) fn new(min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        Self {
            min_x: finite_or(min_x, 0.0),
            min_y: finite_or(min_y, 0.0),
            width: viewport_dimension(width),
            height: viewport_dimension(height),
        }
    }

    fn from_bounds(bounds: DiagramBounds) -> Self {
        Self::new(bounds.min_x, bounds.min_y, bounds.width, bounds.height)
    }

    pub(super) fn attr(self) -> String {
        format!(
            "{} {} {} {}",
            fmt(self.min_x),
            fmt(self.min_y),
            fmt(self.width),
            fmt(self.height)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct RootSvgOverrides {
    pub(super) view_box: ViewBox,
    pub(super) max_width: String,
}

impl RootSvgOverrides {
    #[cfg(test)]
    pub(super) fn from_attrs(viewbox_attr: &str, max_width: &str) -> Option<Self> {
        Some(Self {
            view_box: parse_viewbox_attr(viewbox_attr)?,
            max_width: max_width.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct RootViewportPlan {
    pub(super) view_box: ViewBox,
    pub(super) width: Option<String>,
    pub(super) height: Option<String>,
    pub(super) style: Option<String>,
}

impl RootViewportPlan {
    pub(super) fn viewbox_attr(&self) -> String {
        self.view_box.attr()
    }
}

pub(super) fn resolve_root_overrides(
    explicit: Option<&RootSvgOverrides>,
    family_default: Option<&RootSvgOverrides>,
) -> Option<RootSvgOverrides> {
    explicit.or(family_default).cloned()
}

pub(super) fn build_root_viewport_plan(
    bounds: DiagramBounds,
    root_overrides: Option<&RootSvgOverrides>,
    responsive: bool,
) -> RootViewportPlan {
    let computed_view_box = ViewBox::from_bounds(bounds);
    let (view_box, max_width) = if let Some(root_overrides) = root_overrides {
        (root_overrides.view_box, root_overrides.max_width.clone())
    } else {
        (computed_view_box, fmt_string(computed_view_box.width))
    };

    if responsive {
        RootViewportPlan {
            view_box,
            width: Some("100%".to_string()),
            height: None,
            style: Some(format!(
                "max-width: {max_width}px; background-color: white;"
            )),
        }
    } else {
        RootViewportPlan {
            view_box,
            width: Some(fmt_string(view_box.width)),
            height: Some(fmt_string(view_box.height)),
            style: Some("background-color: white;".to_string()),
        }
    }
}

pub(super) fn push_svg_root_open_with_viewport_plan(
    out: &mut String,
    attrs: SvgRootAttrs<'_>,
    plan: &RootViewportPlan,
) {
    let SvgRootAttrs {
        diagram_id,
        class,
        width: _,
        height_attr: _,
        style_attr: _,
        viewbox_attr: _,
        style_viewbox_order,
        extra_attrs,
        aria_roledescription,
        aria_labelledby,
        aria_describedby,
        after_roledescription_attrs,
        tail_attrs,
        fixed_height_placement,
        trailing_newline,
        aria_attr_order,
    } = attrs;

    let viewbox_attr = plan.viewbox_attr();
    let width = match plan.width.as_deref() {
        None => SvgRootWidth::None,
        Some("100%") => SvgRootWidth::Percent100,
        Some(width) => SvgRootWidth::Fixed(width),
    };

    push_svg_root_open(
        out,
        SvgRootAttrs {
            diagram_id,
            class,
            width,
            height_attr: plan.height.as_deref(),
            style_attr: plan.style.as_deref(),
            viewbox_attr: Some(viewbox_attr.as_str()),
            style_viewbox_order,
            extra_attrs,
            aria_roledescription,
            aria_labelledby,
            aria_describedby,
            after_roledescription_attrs,
            tail_attrs,
            fixed_height_placement,
            trailing_newline,
            aria_attr_order,
        },
    );
}

#[cfg(test)]
fn parse_viewbox_attr(viewbox_attr: &str) -> Option<ViewBox> {
    let mut parts = viewbox_attr.split_whitespace();
    let min_x = parts.next()?.parse::<f64>().ok()?;
    let min_y = parts.next()?.parse::<f64>().ok()?;
    let width = parts.next()?.parse::<f64>().ok()?;
    let height = parts.next()?.parse::<f64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(ViewBox::new(min_x, min_y, width, height))
}

fn finite_or(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn viewport_dimension(value: f64) -> f64 {
    finite_or(value, 1.0).max(1.0)
}

pub(super) enum SvgRootWidth<'a> {
    None,
    Percent100,
    Fixed(&'a str),
}

pub(super) enum SvgRootStyleViewBoxOrder {
    StyleThenViewBox,
    ViewBoxThenStyle,
}

pub(super) enum SvgRootAriaAttrOrder {
    DescribedbyThenLabelledby,
    LabelledbyThenDescribedby,
}

pub(super) enum SvgRootFixedHeightPlacement {
    BeforeXmlns,
    AfterXmlns,
    AfterClass,
}

pub(super) struct SvgRootAttrs<'a> {
    pub(super) diagram_id: &'a str,
    pub(super) class: Option<&'a str>,
    pub(super) width: SvgRootWidth<'a>,
    pub(super) height_attr: Option<&'a str>,
    pub(super) style_attr: Option<&'a str>,
    pub(super) viewbox_attr: Option<&'a str>,
    pub(super) style_viewbox_order: SvgRootStyleViewBoxOrder,
    pub(super) extra_attrs: &'a [(&'a str, &'a str)],
    pub(super) aria_roledescription: &'a str,
    pub(super) aria_labelledby: Option<&'a str>,
    pub(super) aria_describedby: Option<&'a str>,
    pub(super) after_roledescription_attrs: &'a [(&'a str, &'a str)],
    pub(super) tail_attrs: &'a [(&'a str, &'a str)],
    pub(super) fixed_height_placement: SvgRootFixedHeightPlacement,
    pub(super) trailing_newline: bool,
    pub(super) aria_attr_order: SvgRootAriaAttrOrder,
}

impl<'a> SvgRootAttrs<'a> {
    pub(super) fn new(diagram_id: &'a str, aria_roledescription: &'a str) -> Self {
        Self {
            diagram_id,
            class: None,
            width: SvgRootWidth::None,
            height_attr: None,
            style_attr: None,
            viewbox_attr: None,
            style_viewbox_order: SvgRootStyleViewBoxOrder::StyleThenViewBox,
            extra_attrs: &[],
            aria_roledescription,
            aria_labelledby: None,
            aria_describedby: None,
            after_roledescription_attrs: &[],
            tail_attrs: &[],
            fixed_height_placement: SvgRootFixedHeightPlacement::BeforeXmlns,
            trailing_newline: true,
            aria_attr_order: SvgRootAriaAttrOrder::DescribedbyThenLabelledby,
        }
    }
}

pub(super) fn push_svg_root_open(out: &mut String, attrs: SvgRootAttrs<'_>) {
    let SvgRootAttrs {
        diagram_id,
        class,
        width,
        height_attr,
        style_attr,
        viewbox_attr,
        style_viewbox_order,
        extra_attrs,
        aria_roledescription,
        aria_labelledby,
        aria_describedby,
        after_roledescription_attrs,
        tail_attrs,
        fixed_height_placement,
        trailing_newline,
        aria_attr_order,
    } = attrs;

    // Keep attribute order stable (helps strict-mode diffs) and match existing renderers:
    // id, width/height (with configurable fixed-height placement), xmlns, class?,
    // style?/viewBox (configurable), extra-attrs..., role, aria-roledescription, aria-*, tail-attrs..., >\n?
    let mut deferred_height_after_class: Option<&str> = None;
    out.push_str(r#"<svg id=""#);
    escape_xml_into(out, diagram_id);
    match width {
        SvgRootWidth::None => {
            out.push_str(r#"" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#);
        }
        SvgRootWidth::Percent100 => {
            out.push_str(
                r#"" width="100%" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#,
            );
        }
        SvgRootWidth::Fixed(w) => {
            out.push_str(r#"" width=""#);
            out.push_str(w);
            out.push('"');
            match fixed_height_placement {
                SvgRootFixedHeightPlacement::BeforeXmlns => {
                    out.push_str(r#" height=""#);
                    out.push_str(height_attr.unwrap_or("0"));
                    out.push('"');
                    out.push_str(
                        r#" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#,
                    );
                }
                SvgRootFixedHeightPlacement::AfterXmlns => {
                    out.push_str(
                        r#" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#,
                    );
                    out.push_str(r#" height=""#);
                    out.push_str(height_attr.unwrap_or("0"));
                    out.push('"');
                }
                SvgRootFixedHeightPlacement::AfterClass => {
                    out.push_str(
                        r#" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink""#,
                    );
                    deferred_height_after_class = Some(height_attr.unwrap_or("0"));
                }
            }
        }
    }

    if let Some(class) = class {
        out.push_str(r#" class=""#);
        out.push_str(class);
        out.push('"');
    }
    if let Some(h) = deferred_height_after_class.take() {
        out.push_str(r#" height=""#);
        out.push_str(h);
        out.push('"');
    }
    match style_viewbox_order {
        SvgRootStyleViewBoxOrder::StyleThenViewBox => {
            if let Some(style_attr) = style_attr {
                out.push_str(r#" style=""#);
                out.push_str(style_attr);
                out.push('"');
            }
            if let Some(viewbox_attr) = viewbox_attr {
                out.push_str(r#" viewBox=""#);
                out.push_str(viewbox_attr);
                out.push('"');
            }
        }
        SvgRootStyleViewBoxOrder::ViewBoxThenStyle => {
            if let Some(viewbox_attr) = viewbox_attr {
                out.push_str(r#" viewBox=""#);
                out.push_str(viewbox_attr);
                out.push('"');
            }
            if let Some(style_attr) = style_attr {
                out.push_str(r#" style=""#);
                out.push_str(style_attr);
                out.push('"');
            }
        }
    }
    if let Some(h) = deferred_height_after_class.take() {
        out.push_str(r#" height=""#);
        out.push_str(h);
        out.push('"');
    }

    for (k, v) in extra_attrs {
        out.push(' ');
        out.push_str(k);
        out.push_str(r#"=""#);
        out.push_str(v);
        out.push('"');
    }

    out.push_str(r#" role="graphics-document document" aria-roledescription=""#);
    out.push_str(aria_roledescription);
    out.push('"');
    for (k, v) in after_roledescription_attrs {
        out.push(' ');
        out.push_str(k);
        out.push_str(r#"=""#);
        out.push_str(v);
        out.push('"');
    }
    match aria_attr_order {
        SvgRootAriaAttrOrder::DescribedbyThenLabelledby => {
            if let Some(v) = aria_describedby {
                out.push_str(r#" aria-describedby=""#);
                out.push_str(v);
                out.push('"');
            }
            if let Some(v) = aria_labelledby {
                out.push_str(r#" aria-labelledby=""#);
                out.push_str(v);
                out.push('"');
            }
        }
        SvgRootAriaAttrOrder::LabelledbyThenDescribedby => {
            if let Some(v) = aria_labelledby {
                out.push_str(r#" aria-labelledby=""#);
                out.push_str(v);
                out.push('"');
            }
            if let Some(v) = aria_describedby {
                out.push_str(r#" aria-describedby=""#);
                out.push_str(v);
                out.push('"');
            }
        }
    }

    for (k, v) in tail_attrs {
        out.push(' ');
        out.push_str(k);
        out.push_str(r#"=""#);
        out.push_str(v);
        out.push('"');
    }

    out.push('>');
    if trailing_newline {
        out.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_viewport_plan_prefers_explicit_override() {
        let explicit = RootSvgOverrides::from_attrs("-1.5 2 320 180", "319.75").unwrap();
        let family_default = RootSvgOverrides::from_attrs("0 0 40 20", "40").unwrap();
        let resolved = resolve_root_overrides(Some(&explicit), Some(&family_default));
        let plan = build_root_viewport_plan(
            DiagramBounds::from_view_box(0.0, 0.0, 10.0, 10.0),
            resolved.as_ref(),
            true,
        );

        assert_eq!(plan.view_box, ViewBox::new(-1.5, 2.0, 320.0, 180.0));
        assert_eq!(plan.width.as_deref(), Some("100%"));
        assert_eq!(plan.height.as_deref(), None);
        assert_eq!(
            plan.style.as_deref(),
            Some("max-width: 319.75px; background-color: white;")
        );
        assert_eq!(plan.viewbox_attr(), "-1.5 2 320 180");
    }

    #[test]
    fn root_viewport_plan_uses_family_override_when_explicit_missing() {
        let family_default = RootSvgOverrides::from_attrs("3 4 50 60", "49.5").unwrap();
        let resolved = resolve_root_overrides(None, Some(&family_default));
        let plan = build_root_viewport_plan(
            DiagramBounds::from_view_box(0.0, 0.0, 10.0, 10.0),
            resolved.as_ref(),
            true,
        );

        assert_eq!(plan.view_box, ViewBox::new(3.0, 4.0, 50.0, 60.0));
        assert_eq!(
            plan.style.as_deref(),
            Some("max-width: 49.5px; background-color: white;")
        );
    }

    #[test]
    fn root_viewport_plan_keeps_fixed_dimensions() {
        let plan = build_root_viewport_plan(
            DiagramBounds::from_view_box(-2.0, 3.0, 42.5, 24.0),
            None,
            false,
        );

        assert_eq!(plan.viewbox_attr(), "-2 3 42.5 24");
        assert_eq!(plan.width.as_deref(), Some("42.5"));
        assert_eq!(plan.height.as_deref(), Some("24"));
        assert_eq!(plan.style.as_deref(), Some("background-color: white;"));
    }

    #[test]
    fn root_viewport_plan_emits_responsive_root_attrs() {
        let plan = build_root_viewport_plan(
            DiagramBounds::from_view_box(-2.0, 0.0, 42.0, 24.0),
            None,
            true,
        );
        let mut out = String::new();

        push_svg_root_open_with_viewport_plan(
            &mut out,
            SvgRootAttrs {
                trailing_newline: false,
                ..SvgRootAttrs::new("root-id", "treeView")
            },
            &plan,
        );

        assert!(out.contains(r#"width="100%""#));
        assert!(!out.contains(r#"height=""#));
        assert!(out.contains(r#"style="max-width: 42px; background-color: white;""#));
        assert!(out.contains(r#"viewBox="-2 0 42 24""#));
    }
}
