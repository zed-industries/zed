use crate::model::Bounds;
use crate::text::{TextMeasurer, TextStyle};

pub(crate) const ARCHITECTURE_LAYOUT_CANVAS_LABEL_WIDTH_SCALE: f64 = 1.055;
pub(crate) const ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE: f64 = 1.01;
pub(crate) const ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_THRESHOLD_PX: f64 = 200.0;
pub(crate) const ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX: f64 = 18.0;
pub(crate) const ARCHITECTURE_CREATE_TEXT_DEFAULT_WRAP_WIDTH_PX: f64 = 200.0;
pub(crate) const ARCHITECTURE_SVG_GROUP_BBOX_EXTRA_PADDING_PX: f64 = 2.5;

#[derive(Debug, Clone)]
pub(crate) struct ArchitectureServiceBoundsEstimate {
    // Actual emitted icon bounds used when grouped service labels should not affect root getBBox.
    pub(crate) emitted_icon_bounds: Bounds,
    // Approximation of Mermaid's final SVG getBBox() for top-level services.
    pub(crate) svg_root_bounds: Bounds,
    // Explicit Cytoscape child contribution phases for compound sizing.
    pub(crate) cytoscape_group_child_contribution: ArchitectureCytoscapeChildContributionBounds,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArchitectureCytoscapeCanvasLabelMetrics {
    pub(crate) width: f64,
    pub(crate) half_width: f64,
    pub(crate) applied_scale: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ArchitectureCytoscapeChildLabelBounds {
    pub(crate) metrics: ArchitectureCytoscapeCanvasLabelMetrics,
    pub(crate) half_width: f64,
    pub(crate) bottom_extension_px: f64,
}

impl ArchitectureCytoscapeChildLabelBounds {
    fn bounds_for_icon(&self, icon_bounds: &Bounds) -> Bounds {
        let center_x = (icon_bounds.min_x + icon_bounds.max_x) / 2.0;
        Bounds {
            min_x: center_x - self.half_width,
            min_y: icon_bounds.min_y,
            max_x: center_x + self.half_width,
            max_y: icon_bounds.max_y + self.bottom_extension_px,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ArchitectureCytoscapeChildContributionBounds {
    pub(crate) body_bounds: Bounds,
    pub(crate) label_bounds: Option<Bounds>,
    pub(crate) union_bounds: Bounds,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ArchitectureNodeBBoxExtras {
    pub(crate) left: f64,
    pub(crate) right: f64,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
}

pub(crate) fn architecture_cytoscape_canvas_label_metrics(
    label: &str,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
) -> ArchitectureCytoscapeCanvasLabelMetrics {
    let m = measurer.measure(label, style);
    let width = m.width.max(0.0);
    let scale = architecture_layout_canvas_label_width_scale(width);
    let half_width = (width * scale) / 2.0;
    let half_width = (half_width * 2.0).round() / 2.0;
    ArchitectureCytoscapeCanvasLabelMetrics {
        width: m.width,
        half_width,
        applied_scale: scale,
    }
}

pub(crate) fn architecture_layout_canvas_label_width_scale(width_px: f64) -> f64 {
    if width_px >= ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_THRESHOLD_PX {
        ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE
    } else {
        ARCHITECTURE_LAYOUT_CANVAS_LABEL_WIDTH_SCALE
    }
}

pub(crate) fn architecture_create_text_bbox_height_px(font_size_px: f64, line_count: usize) -> f64 {
    let font_size_px = font_size_px.max(1.0);
    let extra_lines = line_count.max(1).saturating_sub(1) as f64;
    font_size_px * ((19.0 / 16.0) + extra_lines * 1.1)
}

pub(crate) fn architecture_create_text_root_label_extra_bottom_px(
    font_size_px: f64,
    line_count: usize,
) -> f64 {
    let font_size_px = font_size_px.max(1.0);
    let extra_lines = line_count.max(1).saturating_sub(1) as f64;
    font_size_px * ((24.1875 / 16.0) + extra_lines * 1.1)
}

pub(crate) fn architecture_create_text_bbox_y_range_px(
    font_size_px: f64,
    line_count: usize,
) -> (f64, f64) {
    let height = architecture_create_text_bbox_height_px(font_size_px, line_count);
    let max_y = architecture_create_text_root_label_extra_bottom_px(font_size_px, line_count);
    (max_y - height, max_y)
}

pub(crate) fn architecture_create_text_compound_label_extra_bottom_px(font_size_px: f64) -> f64 {
    font_size_px.max(1.0) + 1.0
}

pub(crate) fn architecture_svg_group_bbox_padding_px(padding_px: f64) -> f64 {
    padding_px.max(0.0) + ARCHITECTURE_SVG_GROUP_BBOX_EXTRA_PADDING_PX
}

fn union_bounds(a: &Bounds, b: &Bounds) -> Bounds {
    Bounds {
        min_x: a.min_x.min(b.min_x),
        min_y: a.min_y.min(b.min_y),
        max_x: a.max_x.max(b.max_x),
        max_y: a.max_y.max(b.max_y),
    }
}

pub(crate) fn architecture_cytoscape_child_contribution_bounds(
    icon_bounds: &Bounds,
    label_bounds: Option<&ArchitectureCytoscapeChildLabelBounds>,
) -> ArchitectureCytoscapeChildContributionBounds {
    let body_bounds = icon_bounds.clone();
    let label_bounds = label_bounds.map(|label| label.bounds_for_icon(&body_bounds));
    let union_bounds = label_bounds
        .as_ref()
        .map(|label| union_bounds(&body_bounds, label))
        .unwrap_or_else(|| body_bounds.clone());

    ArchitectureCytoscapeChildContributionBounds {
        body_bounds,
        label_bounds,
        union_bounds,
    }
}

fn architecture_cytoscape_node_bbox_extra_contribution_bounds(
    icon_size: f64,
    border_px: f64,
    label_bounds: Option<&ArchitectureCytoscapeChildLabelBounds>,
) -> ArchitectureCytoscapeChildContributionBounds {
    let half_icon = icon_size / 2.0;
    let body_bounds = Bounds {
        min_x: -half_icon - border_px,
        min_y: -half_icon - border_px,
        max_x: half_icon + border_px,
        max_y: half_icon + border_px,
    };
    let label_bounds = label_bounds.map(|label| Bounds {
        min_x: -label.half_width - border_px,
        min_y: -half_icon - border_px,
        max_x: label.half_width + border_px,
        max_y: half_icon + label.bottom_extension_px + border_px,
    });
    let union_bounds = label_bounds
        .as_ref()
        .map(|label| union_bounds(&body_bounds, label))
        .unwrap_or_else(|| body_bounds.clone());

    ArchitectureCytoscapeChildContributionBounds {
        body_bounds,
        label_bounds,
        union_bounds,
    }
}

pub(crate) fn architecture_cytoscape_child_label_bounds(
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    font_size_px: f64,
) -> Option<ArchitectureCytoscapeChildLabelBounds> {
    let title = title.map(str::trim).filter(|t| !t.is_empty())?;
    let metrics = architecture_cytoscape_canvas_label_metrics(title, measurer, style);
    Some(ArchitectureCytoscapeChildLabelBounds {
        metrics,
        half_width: metrics.half_width,
        bottom_extension_px: architecture_create_text_compound_label_extra_bottom_px(font_size_px),
    })
}

pub(crate) fn architecture_measure_cytoscape_node_bbox_extras(
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    style: &TextStyle,
    icon_size: f64,
    font_size_px: f64,
) -> ArchitectureNodeBBoxExtras {
    let border = 1.0;
    let half_icon = icon_size / 2.0;
    let label_bounds =
        architecture_cytoscape_child_label_bounds(title, measurer, style, font_size_px);
    let contribution = architecture_cytoscape_node_bbox_extra_contribution_bounds(
        icon_size,
        border,
        label_bounds.as_ref(),
    );
    let half_w = contribution
        .union_bounds
        .max_x
        .abs()
        .max(contribution.union_bounds.min_x.abs());
    let half_w = (half_w * 2.0).round() / 2.0;
    let top = (-contribution.union_bounds.min_y - half_icon).max(0.0);
    let bottom = (contribution.union_bounds.max_y - half_icon).max(0.0);

    if let Some(label_bounds) = &label_bounds {
        let label_half = label_bounds.half_width;

        if std::env::var("MERMAN_ARCH_DEBUG_CY_BBOX").ok().as_deref() == Some("1") {
            eprintln!(
                "[arch-cy-bbox] title={:?} width={:.6} label_half={:.6} scale={:.6} body_bounds=({}, {})-({}, {}) label_bounds={:?} union_bounds=({}, {})-({}, {}) half_w={:.6} extras_lr={:.6} bottom={:.6}",
                title.map(str::trim).unwrap_or(""),
                label_bounds.metrics.width,
                label_half,
                label_bounds.metrics.applied_scale,
                contribution.body_bounds.min_x,
                contribution.body_bounds.min_y,
                contribution.body_bounds.max_x,
                contribution.body_bounds.max_y,
                contribution
                    .label_bounds
                    .as_ref()
                    .map(|b| (b.min_x, b.min_y, b.max_x, b.max_y)),
                contribution.union_bounds.min_x,
                contribution.union_bounds.min_y,
                contribution.union_bounds.max_x,
                contribution.union_bounds.max_y,
                half_w,
                (half_w - half_icon).max(0.0),
                bottom,
            );
        }
    }

    let extra_lr = (half_w - half_icon).max(0.0);
    ArchitectureNodeBBoxExtras {
        left: extra_lr,
        right: extra_lr,
        top,
        bottom,
    }
}

pub(crate) fn architecture_node_bbox_extras_to_manatee(
    extras: ArchitectureNodeBBoxExtras,
) -> manatee::BoundsExtras {
    manatee::BoundsExtras {
        left: extras.left,
        right: extras.right,
        top: extras.top,
        bottom: extras.bottom,
    }
}

pub(crate) fn architecture_estimate_service_bounds<TLine>(
    x: f64,
    y: f64,
    icon_size_px: f64,
    arch_font_size_px: f64,
    svg_font_size_px: f64,
    title: Option<&str>,
    text_measurer: &dyn TextMeasurer,
    text_style: &TextStyle,
    compound_text_style: &TextStyle,
    wrap_svg_words_to_lines: impl Fn(&str, f64, &dyn TextMeasurer, &TextStyle) -> Vec<TLine>,
    svg_line_plain_text: impl Fn(&TLine) -> String,
    measure_svg_text_bbox_x: impl Fn(&str, &TextStyle) -> (f64, f64),
) -> ArchitectureServiceBoundsEstimate
where
    TLine: std::fmt::Debug,
{
    let emitted_icon_bounds = Bounds {
        min_x: x,
        min_y: y,
        max_x: x + icon_size_px,
        max_y: y + icon_size_px,
    };
    let mut svg_root_bounds = emitted_icon_bounds.clone();
    let mut cytoscape_group_child_contribution =
        architecture_cytoscape_child_contribution_bounds(&emitted_icon_bounds, None);
    let debug_service = std::env::var("MERMAN_ARCH_DEBUG_SERVICE_BOUNDS")
        .ok()
        .filter(|value| !value.is_empty());

    if let Some(title) = title.map(str::trim).filter(|t| !t.is_empty()) {
        let lines = wrap_svg_words_to_lines(title, icon_size_px * 1.5, text_measurer, text_style);
        let mut bbox_left_root = 0.0f64;
        let mut bbox_right_root = 0.0f64;
        for line in &lines {
            let s = svg_line_plain_text(line);
            let (l, r) = measure_svg_text_bbox_x(s.as_str(), text_style);
            bbox_left_root = bbox_left_root.max(l);
            bbox_right_root = bbox_right_root.max(r);
        }
        let line_count_root = lines.len().max(1);
        let label_extra_bottom_root =
            architecture_create_text_root_label_extra_bottom_px(svg_font_size_px, line_count_root);

        let cx = x + icon_size_px / 2.0;
        let text_left_root = cx - bbox_left_root;
        let text_right_root = cx + bbox_right_root;
        let text_bottom_root = y + icon_size_px + label_extra_bottom_root;

        svg_root_bounds = Bounds {
            min_x: svg_root_bounds.min_x.min(text_left_root),
            min_y: svg_root_bounds.min_y,
            max_x: svg_root_bounds.max_x.max(text_right_root),
            max_y: svg_root_bounds.max_y.max(text_bottom_root),
        };
        if let Some(cytoscape_label_bounds) = architecture_cytoscape_child_label_bounds(
            Some(title),
            text_measurer,
            compound_text_style,
            arch_font_size_px,
        ) {
            let label_extra_bottom_compound = cytoscape_label_bounds.bottom_extension_px;
            cytoscape_group_child_contribution = architecture_cytoscape_child_contribution_bounds(
                &emitted_icon_bounds,
                Some(&cytoscape_label_bounds),
            );

            if debug_service.as_deref() == Some(title) {
                let label_bounds = cytoscape_group_child_contribution.label_bounds.as_ref();
                eprintln!(
                    "[arch-service-bounds] title={:?} svg_lines={:?} root_lr=({}, {}) root_bottom={} canvas_half={} group_child_bottom={} child_body_bounds=({}, {})-({}, {}) child_label_bounds={:?} group_child_bounds=({}, {})-({}, {}) svg_root_bounds=({}, {})-({}, {})",
                    title,
                    lines,
                    bbox_left_root,
                    bbox_right_root,
                    label_extra_bottom_root,
                    cytoscape_label_bounds.half_width,
                    label_extra_bottom_compound,
                    cytoscape_group_child_contribution.body_bounds.min_x,
                    cytoscape_group_child_contribution.body_bounds.min_y,
                    cytoscape_group_child_contribution.body_bounds.max_x,
                    cytoscape_group_child_contribution.body_bounds.max_y,
                    label_bounds.map(|b| (b.min_x, b.min_y, b.max_x, b.max_y)),
                    cytoscape_group_child_contribution.union_bounds.min_x,
                    cytoscape_group_child_contribution.union_bounds.min_y,
                    cytoscape_group_child_contribution.union_bounds.max_x,
                    cytoscape_group_child_contribution.union_bounds.max_y,
                    svg_root_bounds.min_x,
                    svg_root_bounds.min_y,
                    svg_root_bounds.max_x,
                    svg_root_bounds.max_y,
                );
            }
        }
    }

    ArchitectureServiceBoundsEstimate {
        emitted_icon_bounds,
        svg_root_bounds,
        cytoscape_group_child_contribution,
    }
}

pub(crate) fn architecture_top_level_service_root_bounds(
    estimate: &ArchitectureServiceBoundsEstimate,
    has_incident_edge: bool,
    has_groups: bool,
) -> Bounds {
    if has_groups && !has_incident_edge {
        estimate
            .cytoscape_group_child_contribution
            .union_bounds
            .clone()
    } else {
        estimate.svg_root_bounds.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn architecture_text_constants_match_mermaid() {
        assert!((super::architecture_create_text_bbox_height_px(16.0, 2) - 36.6).abs() < 1e-9);
        assert_eq!(
            super::architecture_create_text_bbox_y_range_px(16.0, 1),
            (5.1875, 24.1875)
        );
        assert!((super::architecture_create_text_bbox_y_range_px(16.0, 2).0 - 5.1875).abs() < 1e-9);
        assert!(
            (super::architecture_create_text_bbox_y_range_px(16.0, 2).1 - 41.7875).abs() < 1e-9
        );
        assert_eq!(
            super::architecture_create_text_compound_label_extra_bottom_px(16.0),
            17.0
        );
        assert_eq!(
            super::architecture_create_text_compound_label_extra_bottom_px(12.0),
            13.0
        );
        assert_eq!(
            super::architecture_create_text_root_label_extra_bottom_px(16.0, 1),
            24.1875
        );
        assert_eq!(super::ARCHITECTURE_LAYOUT_CANVAS_LABEL_WIDTH_SCALE, 1.055);
        assert_eq!(
            super::ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE,
            1.01
        );
        assert_eq!(
            super::ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_THRESHOLD_PX,
            200.0
        );
        assert_eq!(super::ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX, 18.0);
        assert_eq!(super::ARCHITECTURE_CREATE_TEXT_DEFAULT_WRAP_WIDTH_PX, 200.0);
        assert_eq!(super::ARCHITECTURE_SVG_GROUP_BBOX_EXTRA_PADDING_PX, 2.5);
    }

    #[test]
    fn architecture_node_bbox_extras_convert_to_manatee_bounds_extras() {
        let extras = super::ArchitectureNodeBBoxExtras {
            left: 1.5,
            right: 2.5,
            top: 3.5,
            bottom: 4.5,
        };
        let mapped = super::architecture_node_bbox_extras_to_manatee(extras);
        assert_eq!(mapped.left, 1.5);
        assert_eq!(mapped.right, 2.5);
        assert_eq!(mapped.top, 3.5);
        assert_eq!(mapped.bottom, 4.5);
    }

    #[test]
    fn architecture_node_bbox_extra_contribution_preserves_body_label_union_phases() {
        let label_bounds = super::ArchitectureCytoscapeChildLabelBounds {
            metrics: super::ArchitectureCytoscapeCanvasLabelMetrics {
                width: 96.0,
                half_width: 50.0,
                applied_scale: 1.0,
            },
            half_width: 50.0,
            bottom_extension_px: 17.0,
        };

        let contribution = super::architecture_cytoscape_node_bbox_extra_contribution_bounds(
            80.0,
            1.0,
            Some(&label_bounds),
        );

        assert_eq!(contribution.body_bounds.min_x, -41.0);
        assert_eq!(contribution.body_bounds.max_x, 41.0);
        assert_eq!(contribution.body_bounds.min_y, -41.0);
        assert_eq!(contribution.body_bounds.max_y, 41.0);
        let label = contribution
            .label_bounds
            .as_ref()
            .expect("label phase is preserved");
        assert_eq!(label.min_x, -51.0);
        assert_eq!(label.max_x, 51.0);
        assert_eq!(label.min_y, -41.0);
        assert_eq!(label.max_y, 58.0);
        assert_eq!(contribution.union_bounds.min_x, -51.0);
        assert_eq!(contribution.union_bounds.max_x, 51.0);
        assert_eq!(contribution.union_bounds.min_y, -41.0);
        assert_eq!(contribution.union_bounds.max_y, 58.0);

        let extra_lr = contribution.union_bounds.max_x - 40.0;
        let extra_bottom = contribution.union_bounds.max_y - 40.0;
        assert_eq!(extra_lr, 11.0);
        assert_eq!(extra_bottom, 18.0);
    }

    #[test]
    fn architecture_top_level_service_root_bounds_splits_isolated_group_component_phase() {
        fn assert_bounds_eq(actual: crate::model::Bounds, expected: &crate::model::Bounds) {
            assert_eq!(actual.min_x, expected.min_x);
            assert_eq!(actual.min_y, expected.min_y);
            assert_eq!(actual.max_x, expected.max_x);
            assert_eq!(actual.max_y, expected.max_y);
        }

        let estimate = super::ArchitectureServiceBoundsEstimate {
            emitted_icon_bounds: crate::model::Bounds {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 80.0,
                max_y: 80.0,
            },
            svg_root_bounds: crate::model::Bounds {
                min_x: -10.0,
                min_y: 0.0,
                max_x: 90.0,
                max_y: 104.1875,
            },
            cytoscape_group_child_contribution:
                super::ArchitectureCytoscapeChildContributionBounds {
                    body_bounds: crate::model::Bounds {
                        min_x: 0.0,
                        min_y: 0.0,
                        max_x: 80.0,
                        max_y: 80.0,
                    },
                    label_bounds: Some(crate::model::Bounds {
                        min_x: -8.0,
                        min_y: 0.0,
                        max_x: 88.0,
                        max_y: 97.0,
                    }),
                    union_bounds: crate::model::Bounds {
                        min_x: -8.0,
                        min_y: 0.0,
                        max_x: 88.0,
                        max_y: 97.0,
                    },
                },
        };

        assert_bounds_eq(
            super::architecture_top_level_service_root_bounds(&estimate, false, true),
            &estimate.cytoscape_group_child_contribution.union_bounds,
        );
        assert_bounds_eq(
            super::architecture_top_level_service_root_bounds(&estimate, true, true),
            &estimate.svg_root_bounds,
        );
        assert_bounds_eq(
            super::architecture_top_level_service_root_bounds(&estimate, false, false),
            &estimate.svg_root_bounds,
        );
    }

    #[test]
    fn architecture_canvas_label_metrics_report_applied_scale() {
        let style = crate::text::TextStyle {
            font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
            font_size: 16.0,
            font_weight: None,
        };
        let measurer = crate::text::DeterministicTextMeasurer::default();
        let metrics = super::architecture_cytoscape_canvas_label_metrics(
            "This is a deliberately long architecture label probe",
            &measurer,
            &style,
        );
        assert!(metrics.width > 0.0);
        assert!(
            metrics.applied_scale == super::ARCHITECTURE_LAYOUT_CANVAS_LABEL_WIDTH_SCALE
                || metrics.applied_scale
                    == super::ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE
        );
    }

    #[test]
    fn architecture_canvas_label_scale_switches_at_long_label_threshold() {
        assert_eq!(
            super::architecture_layout_canvas_label_width_scale(199.999),
            super::ARCHITECTURE_LAYOUT_CANVAS_LABEL_WIDTH_SCALE
        );
        assert_eq!(
            super::architecture_layout_canvas_label_width_scale(200.0),
            super::ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE
        );
        assert_eq!(
            super::architecture_layout_canvas_label_width_scale(320.0),
            super::ARCHITECTURE_LAYOUT_CANVAS_LONG_LABEL_WIDTH_SCALE
        );
    }

    #[test]
    fn architecture_cytoscape_child_label_bounds_centralize_compound_child_label_phase() {
        let style = crate::text::TextStyle {
            font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
            font_size: 12.0,
            font_weight: None,
        };
        let measurer = crate::text::DeterministicTextMeasurer::default();

        let label_bounds = super::architecture_cytoscape_child_label_bounds(
            Some("API gateway"),
            &measurer,
            &style,
            12.0,
        )
        .expect("non-empty title has Cytoscape child label bounds");
        let direct_metrics =
            super::architecture_cytoscape_canvas_label_metrics("API gateway", &measurer, &style);

        assert_eq!(label_bounds.metrics.width, direct_metrics.width);
        assert_eq!(label_bounds.half_width, direct_metrics.half_width);
        assert_eq!(
            label_bounds.metrics.applied_scale,
            direct_metrics.applied_scale
        );
        assert_eq!(label_bounds.bottom_extension_px, 13.0);
        assert!(
            super::architecture_cytoscape_child_label_bounds(Some("   "), &measurer, &style, 12.0)
                .is_none()
        );
    }

    #[test]
    fn architecture_cytoscape_child_label_bounds_extend_icon_bounds_by_phase() {
        let label_bounds = super::ArchitectureCytoscapeChildLabelBounds {
            metrics: super::ArchitectureCytoscapeCanvasLabelMetrics {
                width: 96.0,
                half_width: 50.0,
                applied_scale: 1.0,
            },
            half_width: 50.0,
            bottom_extension_px: 17.0,
        };
        let icon_bounds = crate::model::Bounds {
            min_x: 10.0,
            min_y: 20.0,
            max_x: 90.0,
            max_y: 100.0,
        };

        let bounds = label_bounds.bounds_for_icon(&icon_bounds);
        assert_eq!(bounds.min_x, 0.0);
        assert_eq!(bounds.min_y, 20.0);
        assert_eq!(bounds.max_x, 100.0);
        assert_eq!(bounds.max_y, 117.0);
    }

    #[test]
    fn architecture_cytoscape_child_contribution_bounds_preserve_body_label_union_phases() {
        let icon_bounds = crate::model::Bounds {
            min_x: 10.0,
            min_y: 20.0,
            max_x: 90.0,
            max_y: 100.0,
        };

        let without_label =
            super::architecture_cytoscape_child_contribution_bounds(&icon_bounds, None);
        assert_eq!(without_label.body_bounds.min_x, icon_bounds.min_x);
        assert_eq!(without_label.body_bounds.max_y, icon_bounds.max_y);
        assert!(without_label.label_bounds.is_none());
        assert_eq!(without_label.union_bounds.min_x, icon_bounds.min_x);
        assert_eq!(without_label.union_bounds.max_y, icon_bounds.max_y);

        let label_bounds = super::ArchitectureCytoscapeChildLabelBounds {
            metrics: super::ArchitectureCytoscapeCanvasLabelMetrics {
                width: 96.0,
                half_width: 50.0,
                applied_scale: 1.0,
            },
            half_width: 50.0,
            bottom_extension_px: 17.0,
        };
        let with_label = super::architecture_cytoscape_child_contribution_bounds(
            &icon_bounds,
            Some(&label_bounds),
        );

        let child_label = with_label
            .label_bounds
            .as_ref()
            .expect("label phase is preserved");
        assert_eq!(child_label.min_x, 0.0);
        assert_eq!(child_label.max_y, 117.0);
        assert_eq!(with_label.body_bounds.min_x, icon_bounds.min_x);
        assert_eq!(with_label.union_bounds.min_x, 0.0);
        assert_eq!(with_label.union_bounds.max_y, 117.0);
    }

    #[test]
    fn architecture_svg_group_bbox_padding_adds_headless_cytoscape_extra() {
        assert_eq!(super::architecture_svg_group_bbox_padding_px(0.0), 2.5);
        assert_eq!(super::architecture_svg_group_bbox_padding_px(12.0), 14.5);
        assert_eq!(super::architecture_svg_group_bbox_padding_px(-7.0), 2.5);
    }
}
