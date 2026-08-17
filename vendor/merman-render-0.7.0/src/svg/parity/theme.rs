use super::util::SvgTheme;
use crate::chart_palette::{XyChartPaletteConfig, resolve_xychart_plot_palette};
use serde_json::Value;

#[derive(Debug, Clone)]
pub(super) struct CommonCssTheme {
    pub(super) theme_name: String,
    pub(super) look: String,
    pub(super) font_family_css: String,
    pub(super) font_size_px: f64,
    pub(super) text_color: String,
    pub(super) line_color: String,
    pub(super) error_bkg: String,
    pub(super) error_text: String,
}

impl CommonCssTheme {
    pub(super) fn is_dark_theme(&self) -> bool {
        self.theme_name.contains("dark")
    }

    pub(super) fn is_neo(&self) -> bool {
        self.look == "neo"
    }
}

#[derive(Debug, Clone)]
pub(super) struct NodeDiagramTheme {
    pub(super) common: CommonCssTheme,
    pub(super) node_text_color: String,
    pub(super) title_color: String,
    pub(super) main_bkg: String,
    pub(super) node_border: String,
    pub(super) arrowhead_color: String,
    pub(super) stroke_width: String,
    pub(super) edge_label_background: String,
    pub(super) tertiary: String,
    pub(super) cluster_bkg: String,
    pub(super) cluster_border: String,
}

#[derive(Debug, Clone)]
pub(super) struct ClassDiagramTheme {
    pub(super) common: CommonCssTheme,
    pub(super) class_text: String,
    pub(super) note_text: String,
    pub(super) class_group_text: String,
    pub(super) title_color: String,
    pub(super) text_color: String,
    pub(super) main_bkg: String,
    pub(super) node_border: String,
    pub(super) cluster_bkg: String,
    pub(super) cluster_border: String,
    pub(super) stroke_width: String,
}

#[derive(Debug, Clone)]
pub(super) struct SequenceDiagramTheme {
    pub(super) common: CommonCssTheme,
    pub(super) actor_border: String,
    pub(super) actor_fill: String,
    pub(super) stroke_width: String,
    pub(super) drop_shadow: String,
    pub(super) note_border: String,
    pub(super) note_fill: String,
    pub(super) actor_text: String,
    pub(super) actor_line: String,
    pub(super) signal_color: String,
    pub(super) sequence_number: String,
    pub(super) signal_text: String,
    pub(super) label_box_border: String,
    pub(super) label_box_fill: String,
    pub(super) label_text: String,
    pub(super) loop_text: String,
    pub(super) note_text: String,
    pub(super) activation_fill: String,
    pub(super) activation_border: String,
    pub(super) node_border: String,
    pub(super) note_font_weight: String,
    pub(super) label_box_filter: String,
}

#[derive(Debug, Clone)]
pub(super) struct StateDiagramTheme {
    pub(super) common: CommonCssTheme,
    pub(super) transition_color: String,
    pub(super) node_border: String,
    pub(super) background: String,
    pub(super) main_bkg: String,
    pub(super) alt_background: String,
    pub(super) stroke_width: String,
    pub(super) stroke_width_px: String,
    pub(super) rough_stroke_width_value: f64,
    pub(super) note_border: String,
    pub(super) note_bkg: String,
    pub(super) note_text: String,
    pub(super) label_background: String,
    pub(super) edge_label_background: String,
    pub(super) transition_label_color: String,
    pub(super) special_state_color: String,
    pub(super) inner_end_background: String,
    pub(super) end_outer_fill: String,
    pub(super) end_outer_stroke: String,
    pub(super) end_inner_stroke: String,
    pub(super) composite_background: String,
    pub(super) state_bkg: String,
    pub(super) state_border: String,
    pub(super) composite_title_background: String,
    pub(super) state_label_color: String,
    pub(super) drop_shadow: String,
}

#[derive(Debug, Clone)]
pub(crate) struct XyChartTheme {
    pub(crate) background_color: String,
    pub(crate) title_color: String,
    pub(crate) x_axis_title_color: String,
    pub(crate) x_axis_label_color: String,
    pub(crate) x_axis_tick_color: String,
    pub(crate) x_axis_line_color: String,
    pub(crate) y_axis_title_color: String,
    pub(crate) y_axis_label_color: String,
    pub(crate) y_axis_tick_color: String,
    pub(crate) y_axis_line_color: String,
    pub(crate) plot_color_palette: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct QuadrantChartTheme {
    pub(crate) quadrant1_fill: String,
    pub(crate) quadrant2_fill: String,
    pub(crate) quadrant3_fill: String,
    pub(crate) quadrant4_fill: String,
    pub(crate) quadrant1_text_fill: String,
    pub(crate) quadrant2_text_fill: String,
    pub(crate) quadrant3_text_fill: String,
    pub(crate) quadrant4_text_fill: String,
    pub(crate) quadrant_point_fill: String,
    pub(crate) quadrant_point_text_fill: String,
    pub(crate) quadrant_x_axis_text_fill: String,
    pub(crate) quadrant_y_axis_text_fill: String,
    pub(crate) quadrant_title_fill: String,
    pub(crate) quadrant_internal_border_stroke_fill: String,
    pub(crate) quadrant_external_border_stroke_fill: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TreeViewTheme {
    pub(crate) label_font_size: f64,
    pub(crate) label_font_size_css: String,
    pub(crate) label_color: String,
    pub(crate) line_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TreemapTheme {
    pub(crate) title_color: String,
    pub(crate) label_color: String,
    pub(crate) value_color: String,
    pub(crate) section_stroke_color: String,
    pub(crate) section_stroke_width: String,
    pub(crate) section_fill_color: String,
    pub(crate) leaf_stroke_color: String,
    pub(crate) leaf_stroke_width: String,
    pub(crate) leaf_fill_color: String,
    pub(crate) label_font_size: String,
    pub(crate) value_font_size: String,
    pub(crate) title_font_size: String,
    pub(crate) color_scale: Vec<String>,
    pub(crate) color_scale_peer: Vec<String>,
    pub(crate) color_scale_label: Vec<String>,
    text_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GanttTheme {
    pub(crate) font_family: String,
    pub(crate) text_color: String,
    pub(crate) exclude_bkg_color: String,
    pub(crate) section_bkg_color: String,
    pub(crate) section_bkg_color2: String,
    pub(crate) alt_section_bkg_color: String,
    pub(crate) title_color: String,
    pub(crate) title_text_color: String,
    pub(crate) grid_color: String,
    pub(crate) today_line_color: String,
    pub(crate) task_text_dark_color: String,
    pub(crate) task_text_clickable_color: String,
    pub(crate) task_text_color: String,
    pub(crate) task_bkg_color: String,
    pub(crate) task_border_color: String,
    pub(crate) task_text_outside_color: String,
    pub(crate) active_task_bkg_color: String,
    pub(crate) active_task_border_color: String,
    pub(crate) done_task_border_color: String,
    pub(crate) done_task_bkg_color: String,
    pub(crate) crit_border_color: String,
    pub(crate) crit_bkg_color: String,
    pub(crate) vert_line_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct KanbanSectionTheme {
    pub(crate) section_fill: String,
    pub(crate) c_scale: String,
    pub(crate) c_scale_label: String,
    pub(crate) c_scale_inv: String,
}

#[derive(Debug, Clone)]
pub(crate) struct KanbanTheme {
    pub(crate) text_color: String,
    pub(crate) background: String,
    pub(crate) node_border: String,
    pub(crate) root_fill: String,
    pub(crate) root_label: String,
    pub(crate) sections: Vec<KanbanSectionTheme>,
}

impl TreemapTheme {
    pub(crate) fn readable_leaf_label_fill(
        &self,
        leaf_fill: &str,
        leaf_rect_style: &str,
        leaf_label_fill: String,
    ) -> String {
        if css_color_is_transparent(leaf_fill)
            && !style_has_non_empty_decl(leaf_rect_style, "fill")
            && css_color_is_white_like(&leaf_label_fill)
        {
            self.text_color.clone()
        } else {
            leaf_label_fill
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EventModelingTheme {
    pub(crate) font_family_css: String,
    pub(crate) text_color: String,
    pub(crate) ui_fill: String,
    pub(crate) ui_stroke: String,
    pub(crate) processor_fill: String,
    pub(crate) processor_stroke: String,
    pub(crate) read_model_fill: String,
    pub(crate) read_model_stroke: String,
    pub(crate) command_fill: String,
    pub(crate) command_stroke: String,
    pub(crate) event_fill: String,
    pub(crate) event_stroke: String,
    pub(crate) swimlane_background_fill: String,
    pub(crate) swimlane_background_stroke: String,
    pub(crate) relation_stroke: String,
    pub(crate) arrowhead_fill: String,
}

#[derive(Debug, Clone)]
pub(crate) struct IshikawaTheme {
    pub(crate) line_color: String,
    pub(crate) main_bkg: String,
    pub(crate) text_color: String,
    pub(crate) font_family: String,
}

#[derive(Debug, Clone)]
pub(crate) struct VennTheme {
    pub(crate) font_family_css: String,
    pub(crate) title_color: String,
    pub(crate) set_text_color: String,
    pub(crate) circle_colors: Vec<String>,
    pub(crate) primary_color: String,
    pub(crate) is_dark_theme: bool,
}

impl VennTheme {
    pub(crate) fn circle_text_color(&self, base_color: &str) -> String {
        let Some((r, g, b)) = parse_venn_css_rgb(base_color) else {
            return if self.is_dark_theme {
                "#ffffff".to_string()
            } else {
                "#000000".to_string()
            };
        };
        let adjust = if self.is_dark_theme { 30.0 } else { -30.0 };
        let mix = |channel: u8| -> u8 {
            if adjust > 0.0 {
                (channel as f64 + (255.0 - channel as f64) * (adjust / 100.0))
                    .round()
                    .clamp(0.0, 255.0) as u8
            } else {
                (channel as f64 * (1.0 + adjust / 100.0))
                    .round()
                    .clamp(0.0, 255.0) as u8
            }
        };
        format!("#{:02x}{:02x}{:02x}", mix(r), mix(g), mix(b))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JourneyTheme {
    pub(crate) font_family_css: String,
    pub(crate) text_color: String,
    pub(crate) line_color: String,
    pub(crate) face_color: String,
    pub(crate) main_bkg: String,
    pub(crate) node_border: String,
    pub(crate) arrowhead_color: String,
    pub(crate) edge_label_background: String,
    pub(crate) title_color: String,
    pub(crate) tertiary_color: String,
    pub(crate) border2: String,
    pub(crate) fill_types: Vec<String>,
    pub(crate) actor_colors: Vec<Option<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct RadarTheme {
    pub(crate) font_family_css: String,
    pub(crate) base_font_size_css: String,
    pub(crate) text_color: String,
    pub(crate) line_color: String,
    pub(crate) error_bkg_color: String,
    pub(crate) error_text_color: String,
    pub(crate) title_font_size_css: String,
    pub(crate) title_color: String,
    pub(crate) axis_color: String,
    pub(crate) axis_stroke_width: f64,
    pub(crate) axis_label_font_size: f64,
    pub(crate) graticule_color: String,
    pub(crate) graticule_opacity: f64,
    pub(crate) graticule_stroke_width: f64,
    pub(crate) legend_font_size: f64,
    pub(crate) curve_opacity: f64,
    pub(crate) curve_stroke_width: f64,
    pub(crate) series_colors: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct TimelineSectionTheme {
    pub(crate) c_scale: String,
    pub(crate) c_scale_label: String,
    pub(crate) c_scale_inv: String,
}

#[derive(Debug, Clone)]
pub(crate) struct TimelineTheme {
    pub(crate) is_redux_theme: bool,
    pub(crate) is_dark_theme: bool,
    pub(crate) is_color_theme: bool,
    pub(crate) stroke_width: String,
    pub(crate) font_weight: String,
    pub(crate) main_bkg: String,
    pub(crate) node_border: String,
    pub(crate) drop_shadow: String,
    pub(crate) disabled_fill: String,
    pub(crate) disabled_text_fill: String,
    pub(crate) root_fill: String,
    pub(crate) root_label: String,
    pub(crate) border_colors: Vec<String>,
    pub(crate) sections: Vec<TimelineSectionTheme>,
}

pub(crate) struct PresentationTheme<'a> {
    raw: SvgTheme<'a>,
    common: CommonCssTheme,
}

impl<'a> PresentationTheme<'a> {
    pub(crate) fn new(effective_config: &'a Value) -> Self {
        let raw = SvgTheme::new(effective_config);
        let common = CommonCssTheme {
            theme_name: raw.theme_name(),
            look: raw.look(),
            font_family_css: raw.font_family_css(),
            font_size_px: raw.font_size_px(),
            text_color: raw.color("textColor", "#333"),
            line_color: raw.color("lineColor", "#333333"),
            error_bkg: raw.color("errorBkgColor", "#552222"),
            error_text: raw.color("errorTextColor", "#552222"),
        };

        Self { raw, common }
    }

    pub(crate) fn xychart(&self) -> XyChartTheme {
        let background = self
            .raw
            .optional_nested_color("xyChart", "backgroundColor")
            .or_else(|| self.raw.optional_color("background"))
            .unwrap_or_else(|| "white".to_string());
        let primary_color = self
            .raw
            .optional_color("primaryColor")
            .unwrap_or_else(|| "#ECECFF".to_string());
        let primary_text = self
            .raw
            .optional_color("primaryTextColor")
            .or_else(|| invert_hex_color(&primary_color))
            .unwrap_or_else(|| "#333".to_string());

        XyChartTheme {
            background_color: background.clone(),
            title_color: self
                .raw
                .optional_nested_color("xyChart", "titleColor")
                .unwrap_or_else(|| primary_text.clone()),
            x_axis_title_color: self
                .raw
                .optional_nested_color("xyChart", "xAxisTitleColor")
                .unwrap_or_else(|| primary_text.clone()),
            x_axis_label_color: self
                .raw
                .optional_nested_color("xyChart", "xAxisLabelColor")
                .unwrap_or_else(|| primary_text.clone()),
            x_axis_tick_color: self
                .raw
                .optional_nested_color("xyChart", "xAxisTickColor")
                .unwrap_or_else(|| primary_text.clone()),
            x_axis_line_color: self
                .raw
                .optional_nested_color("xyChart", "xAxisLineColor")
                .unwrap_or_else(|| primary_text.clone()),
            y_axis_title_color: self
                .raw
                .optional_nested_color("xyChart", "yAxisTitleColor")
                .unwrap_or_else(|| primary_text.clone()),
            y_axis_label_color: self
                .raw
                .optional_nested_color("xyChart", "yAxisLabelColor")
                .unwrap_or_else(|| primary_text.clone()),
            y_axis_tick_color: self
                .raw
                .optional_nested_color("xyChart", "yAxisTickColor")
                .unwrap_or_else(|| primary_text.clone()),
            y_axis_line_color: self
                .raw
                .optional_nested_color("xyChart", "yAxisLineColor")
                .unwrap_or_else(|| primary_text.clone()),
            plot_color_palette: resolve_xychart_plot_palette(XyChartPaletteConfig {
                theme_name: self.common.theme_name.clone(),
                plot_color_palette: self
                    .raw
                    .optional_nested_color("xyChart", "plotColorPalette"),
                accent_color: self
                    .raw
                    .optional_nested_color("xyChart", "accentColor")
                    .or_else(|| self.raw.optional_color("primaryColor")),
                background_color: Some(background),
            }),
        }
    }

    pub(crate) fn quadrantchart(&self) -> QuadrantChartTheme {
        let quadrant1_fill = self
            .raw
            .optional_color("primaryColor")
            .unwrap_or_else(|| "#ECECFF".to_string());
        let primary_text = self
            .raw
            .optional_color("primaryTextColor")
            .or_else(|| invert_hex_color(&quadrant1_fill))
            .unwrap_or_else(|| "#131300".to_string());
        let border_stroke = self
            .raw
            .optional_color("primaryBorderColor")
            .and_then(|v| css_color_to_rgb_string(&v))
            .unwrap_or_else(|| "rgb(199, 199, 241)".to_string());
        let quadrant_point_fill = derive_quadrant_point_fill(&quadrant1_fill, &border_stroke);

        let mut theme = QuadrantChartTheme {
            quadrant1_fill: quadrant1_fill.clone(),
            quadrant2_fill: adjust_hex_rgb(&quadrant1_fill, 5)
                .unwrap_or_else(|| "#f1f1ff".to_string()),
            quadrant3_fill: adjust_hex_rgb(&quadrant1_fill, 10)
                .unwrap_or_else(|| "#f6f6ff".to_string()),
            quadrant4_fill: adjust_hex_rgb(&quadrant1_fill, 15)
                .unwrap_or_else(|| "#fbfbff".to_string()),
            quadrant1_text_fill: primary_text.clone(),
            quadrant2_text_fill: adjust_hex_rgb(&primary_text, -5)
                .unwrap_or_else(|| "#0e0e00".to_string()),
            quadrant3_text_fill: adjust_hex_rgb(&primary_text, -10)
                .unwrap_or_else(|| "#090900".to_string()),
            quadrant4_text_fill: adjust_hex_rgb(&primary_text, -15)
                .unwrap_or_else(|| "#040400".to_string()),
            quadrant_point_fill,
            quadrant_point_text_fill: primary_text.clone(),
            quadrant_x_axis_text_fill: primary_text.clone(),
            quadrant_y_axis_text_fill: primary_text.clone(),
            quadrant_title_fill: primary_text,
            quadrant_internal_border_stroke_fill: border_stroke.clone(),
            quadrant_external_border_stroke_fill: border_stroke,
        };

        // Mermaid applies quadrant-specific theme variables as raw CSS tokens.
        // Preserve that verbatim behavior for explicit overrides, but keep the
        // headless defaults derived and valid.
        let set = |field: &mut String, key: &str| {
            if let Some(v) = self.raw.optional_color(key) {
                *field = v;
            }
        };

        set(&mut theme.quadrant1_fill, "quadrant1Fill");
        set(&mut theme.quadrant2_fill, "quadrant2Fill");
        set(&mut theme.quadrant3_fill, "quadrant3Fill");
        set(&mut theme.quadrant4_fill, "quadrant4Fill");

        set(&mut theme.quadrant1_text_fill, "quadrant1TextFill");
        set(&mut theme.quadrant2_text_fill, "quadrant2TextFill");
        set(&mut theme.quadrant3_text_fill, "quadrant3TextFill");
        set(&mut theme.quadrant4_text_fill, "quadrant4TextFill");

        if let Some(v) = self.raw.optional_color("quadrantPointFill") {
            if !is_invalid_css_token(&v) {
                theme.quadrant_point_fill = v;
            }
        }
        set(&mut theme.quadrant_point_text_fill, "quadrantPointTextFill");
        set(
            &mut theme.quadrant_x_axis_text_fill,
            "quadrantXAxisTextFill",
        );
        set(
            &mut theme.quadrant_y_axis_text_fill,
            "quadrantYAxisTextFill",
        );
        set(&mut theme.quadrant_title_fill, "quadrantTitleFill");

        set(
            &mut theme.quadrant_internal_border_stroke_fill,
            "quadrantInternalBorderStrokeFill",
        );
        set(
            &mut theme.quadrant_external_border_stroke_fill,
            "quadrantExternalBorderStrokeFill",
        );

        theme
    }

    pub(crate) fn tree_view(&self) -> TreeViewTheme {
        TreeViewTheme {
            label_font_size: self
                .raw
                .optional_nested_css_px("treeView", "labelFontSize")
                .unwrap_or(16.0)
                .max(1.0),
            label_font_size_css: self
                .raw
                .optional_nested_css_value("treeView", "labelFontSize")
                .unwrap_or_else(|| "16px".to_string()),
            label_color: self
                .raw
                .optional_nested_color("treeView", "labelColor")
                .unwrap_or_else(|| "black".to_string()),
            line_color: self
                .raw
                .optional_nested_color("treeView", "lineColor")
                .unwrap_or_else(|| "black".to_string()),
        }
    }

    pub(crate) fn treemap(&self) -> TreemapTheme {
        let text_color = self.raw.color("textColor", "#333");
        let title_color = self
            .raw
            .optional_root_scoped_string("treemap", "titleColor")
            .or_else(|| self.raw.optional_color("titleColor"))
            .unwrap_or_else(|| text_color.clone());
        let raw_theme_name = self.common.theme_name.clone();
        let default_theme = raw_theme_name == "default";
        let theme_name = raw_theme_name.trim().to_ascii_lowercase();
        let label_text_color = self.raw.color("labelTextColor", "black");
        let label_text_is_calculated = label_text_color.trim() == "calculated";
        let scale_label_color = self.raw.color("scaleLabelColor", &label_text_color);
        let neutral_special_label_color = self.raw.color("cScale1", default_c_scale(1));

        let color_scale = (0..12)
            .map(|i| {
                if default_theme {
                    default_c_scale(i).to_string()
                } else {
                    self.raw.color(&format!("cScale{i}"), default_c_scale(i))
                }
            })
            .collect();
        let color_scale_peer = (0..12)
            .map(|i| {
                if default_theme {
                    default_c_scale_peer(i).to_string()
                } else {
                    self.raw
                        .color(&format!("cScalePeer{i}"), default_c_scale_peer(i))
                }
            })
            .collect();
        let color_scale_label = (0..12)
            .map(|i| {
                self.raw
                    .optional_color(&format!("cScaleLabel{i}"))
                    .unwrap_or_else(|| match theme_name.as_str() {
                        "dark" | "forest" => scale_label_color.clone(),
                        "neutral" => {
                            if i == 0 || i == 2 {
                                neutral_special_label_color.clone()
                            } else {
                                scale_label_color.clone()
                            }
                        }
                        _ => {
                            if label_text_is_calculated {
                                scale_label_color.clone()
                            } else if i == 0 || i == 3 {
                                invert_treemap_label_color_to_hex(&label_text_color)
                                    .unwrap_or_else(|| label_text_color.clone())
                            } else {
                                label_text_color.clone()
                            }
                        }
                    })
            })
            .collect();

        TreemapTheme {
            title_color,
            label_color: self
                .raw
                .optional_root_scoped_string("treemap", "labelColor")
                .unwrap_or_else(|| text_color.clone()),
            value_color: self
                .raw
                .optional_root_scoped_string("treemap", "valueColor")
                .unwrap_or_else(|| text_color.clone()),
            section_stroke_color: self.treemap_style_option("sectionStrokeColor", "black"),
            section_stroke_width: self.treemap_style_option("sectionStrokeWidth", "1"),
            section_fill_color: self.treemap_style_option("sectionFillColor", "#efefef"),
            leaf_stroke_color: self.treemap_style_option("leafStrokeColor", "black"),
            leaf_stroke_width: self.treemap_style_option("leafStrokeWidth", "1"),
            leaf_fill_color: self.treemap_style_option("leafFillColor", "#efefef"),
            label_font_size: self.treemap_style_option("labelFontSize", "12px"),
            value_font_size: self.treemap_style_option("valueFontSize", "10px"),
            title_font_size: self.treemap_style_option("titleFontSize", "14px"),
            color_scale,
            color_scale_peer,
            color_scale_label,
            text_color,
        }
    }

    pub(crate) fn gantt(&self) -> GanttTheme {
        let option = |key: &str, default_value: &str| -> String {
            self.raw
                .optional_color(key)
                .unwrap_or_else(|| default_value.to_string())
        };

        let text_color = self.common.text_color.clone();
        let title_color = option("titleColor", "#333");
        let title_text_color = if title_color.trim().is_empty() {
            text_color.clone()
        } else {
            title_color.clone()
        };

        GanttTheme {
            font_family: self.common.font_family_css.clone(),
            text_color,
            exclude_bkg_color: option("excludeBkgColor", "#eeeeee"),
            section_bkg_color: option("sectionBkgColor", "rgba(102, 102, 255, 0.49)"),
            section_bkg_color2: option("sectionBkgColor2", "#fff400"),
            alt_section_bkg_color: option("altSectionBkgColor", "white"),
            title_color,
            title_text_color,
            grid_color: option("gridColor", "lightgrey"),
            today_line_color: option("todayLineColor", "red"),
            task_text_dark_color: option("taskTextDarkColor", "black"),
            task_text_clickable_color: option("taskTextClickableColor", "#003163"),
            task_text_color: option("taskTextColor", "white"),
            task_bkg_color: option("taskBkgColor", "#8a90dd"),
            task_border_color: option("taskBorderColor", "#534fbc"),
            task_text_outside_color: option("taskTextOutsideColor", "black"),
            active_task_bkg_color: option("activeTaskBkgColor", "#bfc7ff"),
            active_task_border_color: option("activeTaskBorderColor", "#534fbc"),
            done_task_border_color: option("doneTaskBorderColor", "grey"),
            done_task_bkg_color: option("doneTaskBkgColor", "lightgrey"),
            crit_border_color: option("critBorderColor", "#ff8888"),
            crit_bkg_color: option("critBkgColor", "red"),
            vert_line_color: option("vertLineColor", "navy"),
        }
    }

    pub(crate) fn kanban(&self) -> KanbanTheme {
        let dark_mode = self.raw.bool_root_or_theme("darkMode").unwrap_or(false);
        let mut hsl_buf = ryu_js::Buffer::new();
        let sections = (0..12)
            .map(|i| {
                let c_scale = self.raw.color(&format!("cScale{i}"), default_c_scale(i));
                let section_fill = adjust_kanban_section_fill(&c_scale, dark_mode, &mut hsl_buf)
                    .unwrap_or_else(|| c_scale.clone());
                KanbanSectionTheme {
                    section_fill,
                    c_scale,
                    c_scale_label: self
                        .raw
                        .color(&format!("cScaleLabel{i}"), default_c_scale_label(i)),
                    c_scale_inv: self
                        .raw
                        .color(&format!("cScaleInv{i}"), default_c_scale_inv(i)),
                }
            })
            .collect();

        KanbanTheme {
            text_color: self.common.text_color.clone(),
            background: self.raw.color("background", "white"),
            node_border: self.raw.color("nodeBorder", "#9370DB"),
            root_fill: self.raw.color("git0", "hsl(240, 100%, 46.2745098039%)"),
            root_label: self.raw.color("gitBranchLabel0", "#ffffff"),
            sections,
        }
    }

    pub(crate) fn eventmodeling(&self) -> EventModelingTheme {
        EventModelingTheme {
            font_family_css: self.raw.font_family_css_root_first(),
            text_color: self.raw.color("textColor", "#333"),
            ui_fill: self.raw.color("emUiFill", "white"),
            ui_stroke: self.raw.color("emUiStroke", "#dbdada"),
            processor_fill: self.raw.color("emProcessorFill", "#edb3f6"),
            processor_stroke: self.raw.color("emProcessorStroke", "#b88cbf"),
            read_model_fill: self.raw.color("emReadModelFill", "#d3f1a2"),
            read_model_stroke: self.raw.color("emReadModelStroke", "#a3b732"),
            command_fill: self.raw.color("emCommandFill", "#bcd6fe"),
            command_stroke: self.raw.color("emCommandStroke", "#679ac3"),
            event_fill: self.raw.color("emEventFill", "#ffb778"),
            event_stroke: self.raw.color("emEventStroke", "#c19a0f"),
            swimlane_background_fill: self
                .raw
                .optional_color("emSwimlaneBackgroundOdd")
                .or_else(|| self.raw.optional_color("emSwimlaneBackground"))
                .unwrap_or_else(|| "rgb(250,250,250)".to_string()),
            swimlane_background_stroke: self
                .raw
                .optional_color("emSwimlaneBackgroundStroke")
                .or_else(|| self.raw.optional_color("emSwimlaneBorder"))
                .unwrap_or_else(|| "rgb(240,240,240)".to_string()),
            relation_stroke: self.raw.color("emRelationStroke", "#000"),
            arrowhead_fill: self.raw.color("emArrowhead", "#000000"),
        }
    }

    pub(crate) fn ishikawa(&self) -> IshikawaTheme {
        IshikawaTheme {
            line_color: self.raw.color("lineColor", "#333"),
            main_bkg: self.raw.color("mainBkg", "#fff"),
            text_color: self.raw.color("textColor", "#333"),
            font_family: self
                .raw
                .root_or_theme_string("fontFamily", "trebuchet ms, verdana, arial, sans-serif"),
        }
    }

    pub(crate) fn venn(&self) -> VennTheme {
        let background = self.raw.color("background", "#f4f4f4");
        let is_dark_theme = parse_venn_css_rgb(&background)
            .is_some_and(|(r, g, b)| venn_luminance(r, g, b) < 0.45)
            || self.common.theme_name.to_ascii_lowercase().contains("dark");

        VennTheme {
            font_family_css: self.raw.font_family_css_root_first(),
            title_color: self
                .raw
                .optional_color("vennTitleTextColor")
                .or_else(|| self.raw.optional_color("titleColor"))
                .unwrap_or_else(|| "#333".to_string()),
            set_text_color: self
                .raw
                .optional_color("vennSetTextColor")
                .or_else(|| self.raw.optional_color("primaryTextColor"))
                .or_else(|| self.raw.optional_color("textColor"))
                .unwrap_or_else(|| "#333".to_string()),
            circle_colors: (1..=8)
                .filter_map(|index| self.raw.optional_color(&format!("venn{index}")))
                .collect(),
            primary_color: self.raw.color("primaryColor", "#ECECFF"),
            is_dark_theme,
        }
    }

    pub(crate) fn journey(&self) -> JourneyTheme {
        let text_color = self.raw.color("textColor", "#333");

        JourneyTheme {
            font_family_css: self.raw.font_family_css(),
            text_color: text_color.clone(),
            line_color: self.raw.color("lineColor", "#333333"),
            face_color: self.raw.color("faceColor", "#FFF8DC"),
            main_bkg: self.raw.color("mainBkg", "#ECECFF"),
            node_border: self.raw.color("nodeBorder", "#9370DB"),
            arrowhead_color: self.raw.color("arrowheadColor", "#333333"),
            edge_label_background: self
                .raw
                .color("edgeLabelBackground", "rgba(232,232,232, 0.8)"),
            title_color: self.raw.color("titleColor", text_color.as_str()),
            tertiary_color: self
                .raw
                .color("tertiaryColor", "hsl(80, 100%, 96.2745098039%)"),
            border2: self.raw.color("border2", "#aaaa33"),
            fill_types: (0..8)
                .map(|index| {
                    self.raw.color(
                        &format!("fillType{index}"),
                        journey_default_fill_type(index),
                    )
                })
                .collect(),
            actor_colors: (0..6)
                .map(|index| self.raw.optional_color(&format!("actor{index}")))
                .collect(),
        }
    }

    pub(crate) fn radar(&self) -> RadarTheme {
        let font_family_css = self
            .raw
            .optional_color("fontFamily")
            .map(|font_family| crate::config::normalize_css_font_family(&font_family))
            .unwrap_or_else(|| crate::config::MERMAID_DEFAULT_FONT_FAMILY_CSS.to_string());
        let base_font_size_css = self
            .raw
            .optional_value("fontSize")
            .unwrap_or_else(|| "16px".to_string());
        let scoped_string = |key: &str, fallback: &str| {
            self.raw
                .optional_scoped_string("radar", key)
                .unwrap_or_else(|| fallback.to_string())
        };
        let scoped_f64 = |key: &str, fallback: f64| {
            self.raw
                .optional_scoped_f64("radar", key)
                .unwrap_or(fallback)
        };

        RadarTheme {
            font_family_css,
            base_font_size_css: base_font_size_css.clone(),
            text_color: self.raw.color("textColor", "#333"),
            line_color: self.raw.color("lineColor", "#333333"),
            error_bkg_color: self.raw.color("errorBkgColor", "#552222"),
            error_text_color: self.raw.color("errorTextColor", "#552222"),
            title_font_size_css: base_font_size_css,
            title_color: self.raw.color("titleColor", "#333"),
            axis_color: scoped_string("axisColor", "#333333"),
            axis_stroke_width: scoped_f64("axisStrokeWidth", 2.0),
            axis_label_font_size: scoped_f64("axisLabelFontSize", 12.0),
            graticule_color: scoped_string("graticuleColor", "#DEDEDE"),
            graticule_opacity: scoped_f64("graticuleOpacity", 0.3),
            graticule_stroke_width: scoped_f64("graticuleStrokeWidth", 1.0),
            legend_font_size: scoped_f64("legendFontSize", 12.0),
            curve_opacity: scoped_f64("curveOpacity", 0.5),
            curve_stroke_width: scoped_f64("curveStrokeWidth", 2.0),
            series_colors: (0..12)
                .map(|index| {
                    self.raw
                        .color(&format!("cScale{index}"), default_c_scale(index))
                })
                .collect(),
        }
    }

    pub(crate) fn timeline(&self) -> TimelineTheme {
        let theme_name = self.common.theme_name.clone();
        let theme_color_limit = self
            .raw
            .optional_f64("THEME_COLOR_LIMIT")
            .map(|value| value.max(1.0).min(64.0) as usize)
            .unwrap_or(12);
        let label_text_color = self.raw.color("labelTextColor", "black");
        let label_text_is_calculated = label_text_color.trim() == "calculated";
        let scale_label_color = self.raw.color("scaleLabelColor", &label_text_color);
        let mut buf = ryu_js::Buffer::new();
        let sections = (0..theme_color_limit)
            .map(|i| {
                let c_scale = self.raw.color(&format!("cScale{i}"), default_c_scale(i));
                let c_scale_label = self
                    .raw
                    .optional_color(&format!("cScaleLabel{i}"))
                    .unwrap_or_else(|| {
                        if label_text_is_calculated {
                            scale_label_color.clone()
                        } else if i == 0 || i == 3 {
                            invert_timeline_label_color_to_hex(&label_text_color)
                                .unwrap_or_else(|| label_text_color.clone())
                        } else {
                            label_text_color.clone()
                        }
                    });
                let c_scale_inv = self
                    .raw
                    .optional_color(&format!("cScaleInv{i}"))
                    .or_else(|| derive_timeline_c_scale_inv_fallback(&c_scale, &mut buf))
                    .unwrap_or_else(|| c_scale.clone());

                TimelineSectionTheme {
                    c_scale,
                    c_scale_label,
                    c_scale_inv,
                }
            })
            .collect();

        TimelineTheme {
            is_redux_theme: theme_name.contains("redux"),
            is_dark_theme: theme_name.contains("dark"),
            is_color_theme: theme_name.contains("color"),
            stroke_width: self.raw.css_value("strokeWidth", "1"),
            font_weight: self.raw.css_value("fontWeight", "normal"),
            main_bkg: self.raw.color("mainBkg", "#ECECFF"),
            node_border: self.raw.color("nodeBorder", "#9370DB"),
            drop_shadow: self.raw.css_value("dropShadow", "none"),
            disabled_fill: self.raw.color("tertiaryColor", "lightgray"),
            disabled_text_fill: self.raw.color("clusterBorder", "#efefef"),
            root_fill: self.raw.color("git0", "hsl(240, 100%, 46.2745098039%)"),
            root_label: self.raw.color("gitBranchLabel0", "#ffffff"),
            border_colors: self.raw.string_array("borderColorArray"),
            sections,
        }
    }

    pub(super) fn common(&self) -> &CommonCssTheme {
        &self.common
    }

    fn treemap_style_option(&self, key: &str, default_value: &str) -> String {
        self.raw
            .optional_root_scoped_css_value("treemap", key)
            .unwrap_or_else(|| default_value.to_string())
    }

    pub(super) fn node_diagram(&self) -> NodeDiagramTheme {
        let node_border = self.raw.color("nodeBorder", "#9370DB");
        let main_bkg = self.raw.color("mainBkg", "#ECECFF");

        NodeDiagramTheme {
            common: self.common.clone(),
            node_text_color: self
                .raw
                .color("nodeTextColor", self.common.text_color.as_str()),
            title_color: self
                .raw
                .color("titleColor", self.common.text_color.as_str()),
            main_bkg,
            node_border,
            arrowhead_color: self
                .raw
                .color("arrowheadColor", self.common.line_color.as_str()),
            stroke_width: self.raw.css_value("strokeWidth", "1"),
            edge_label_background: self
                .raw
                .color("edgeLabelBackground", "rgba(232,232,232, 0.8)"),
            tertiary: self
                .raw
                .color("tertiaryColor", "hsl(80, 100%, 96.2745098039%)"),
            cluster_bkg: self.raw.color("clusterBkg", "#ffffde"),
            cluster_border: self.raw.color("clusterBorder", "#aaaa33"),
        }
    }

    pub(super) fn class_diagram(&self) -> ClassDiagramTheme {
        let class_text = self.raw.color(
            "classText",
            &self
                .raw
                .color("primaryTextColor", self.common.text_color.as_str()),
        );

        ClassDiagramTheme {
            common: self.common.clone(),
            class_text: class_text.clone(),
            note_text: self.raw.color("noteTextColor", "#333"),
            class_group_text: self
                .raw
                .optional_color("nodeBorder")
                .unwrap_or_else(|| class_text.clone()),
            title_color: self.raw.color("titleColor", "#333"),
            text_color: self.raw.color("textColor", class_text.as_str()),
            main_bkg: self.raw.color("mainBkg", "#ECECFF"),
            node_border: self.raw.color("nodeBorder", "#9370DB"),
            cluster_bkg: self.raw.color("clusterBkg", "#ffffde"),
            cluster_border: self.raw.color("clusterBorder", "#aaaa33"),
            stroke_width: self.raw.css_value("strokeWidth", "1"),
        }
    }

    pub(super) fn sequence_diagram(&self) -> SequenceDiagramTheme {
        let actor_border = self.raw.color("actorBorder", "#9370DB");
        let actor_fill = self.raw.color("actorBkg", "#ECECFF");
        let actor_text = self.raw.color("actorTextColor", "black");

        SequenceDiagramTheme {
            common: self.common.clone(),
            actor_border: actor_border.clone(),
            actor_fill: actor_fill.clone(),
            stroke_width: self.raw.css_value("strokeWidth", "1"),
            drop_shadow: self.raw.css_value("dropShadow", "none"),
            note_border: self.raw.color("noteBorderColor", "#aaaa33"),
            note_fill: self.raw.color("noteBkgColor", "#fff5ad"),
            actor_text: actor_text.clone(),
            actor_line: self.raw.color("actorLineColor", actor_border.as_str()),
            signal_color: self.raw.color("signalColor", "#333"),
            sequence_number: self.raw.color("sequenceNumberColor", "white"),
            signal_text: self.raw.color("signalTextColor", "#333"),
            label_box_border: self.raw.color("labelBoxBorderColor", actor_border.as_str()),
            label_box_fill: self.raw.color("labelBoxBkgColor", actor_fill.as_str()),
            label_text: self.raw.color("labelTextColor", actor_text.as_str()),
            loop_text: self.raw.color("loopTextColor", actor_text.as_str()),
            note_text: self.raw.color("noteTextColor", "black"),
            activation_fill: self.raw.color("activationBkgColor", "#f4f4f4"),
            activation_border: self.raw.color("activationBorderColor", "#666"),
            node_border: self.raw.color("nodeBorder", actor_border.as_str()),
            note_font_weight: self
                .raw
                .optional_value("noteFontWeight")
                .map(|font_weight| format!("font-weight:{};", font_weight))
                .unwrap_or_default(),
            label_box_filter: if self.common.is_neo() {
                self.raw.css_value("dropShadow", "none")
            } else {
                "none".to_string()
            },
        }
    }

    pub(super) fn state_diagram(&self) -> StateDiagramTheme {
        let node_border = self.raw.color("nodeBorder", "#9370DB");
        let main_bkg = self.raw.color("mainBkg", "#ECECFF");
        let background = self.raw.color("background", "white");
        let stroke_width = self.raw.css_value("strokeWidth", "1");
        let stroke_width_px = if stroke_width.trim_end().ends_with("px") {
            stroke_width.clone()
        } else {
            format!("{stroke_width}px")
        };
        let stroke_width_value = stroke_width
            .trim()
            .trim_end_matches("px")
            .trim()
            .parse::<f64>()
            .unwrap_or(1.0)
            .max(0.0);
        let rough_stroke_width_value = if (stroke_width_value - 1.0).abs() <= 1e-9 {
            1.3
        } else {
            stroke_width_value
        };
        let transition_color = self
            .raw
            .color("transitionColor", self.common.line_color.as_str());
        let special_state_color = self
            .raw
            .color("specialStateColor", self.common.line_color.as_str());
        let inner_end_background = self.raw.color("innerEndBackground", node_border.as_str());
        let end_outer_fill = if special_state_color.eq_ignore_ascii_case("#333333") {
            "#ECECFF".to_string()
        } else {
            special_state_color.clone()
        };
        let end_outer_stroke = special_state_color.clone();
        let end_inner_stroke = if background.eq_ignore_ascii_case("white") {
            inner_end_background.clone()
        } else {
            background.clone()
        };

        StateDiagramTheme {
            common: self.common.clone(),
            transition_color,
            node_border: node_border.clone(),
            background: background.clone(),
            main_bkg: main_bkg.clone(),
            alt_background: self.raw.color("altBackground", "#efefef"),
            stroke_width,
            stroke_width_px,
            rough_stroke_width_value,
            note_border: self.raw.color("noteBorderColor", "#aaaa33"),
            note_bkg: self.raw.color("noteBkgColor", "#fff5ad"),
            note_text: self.raw.color("noteTextColor", "black"),
            label_background: self.raw.color("labelBackgroundColor", main_bkg.as_str()),
            edge_label_background: self
                .raw
                .color("edgeLabelBackground", "rgba(232,232,232, 0.8)"),
            transition_label_color: self
                .raw
                .optional_color("transitionLabelColor")
                .or_else(|| self.raw.optional_color("tertiaryTextColor"))
                .unwrap_or_else(|| self.common.text_color.clone()),
            special_state_color,
            inner_end_background,
            end_outer_fill,
            end_outer_stroke,
            end_inner_stroke,
            composite_background: self
                .raw
                .optional_color("compositeBackground")
                .unwrap_or_else(|| background.to_string()),
            state_bkg: self
                .raw
                .optional_color("stateBkg")
                .unwrap_or_else(|| main_bkg.clone()),
            state_border: self
                .raw
                .optional_color("stateBorder")
                .unwrap_or_else(|| node_border.clone()),
            composite_title_background: self
                .raw
                .color("compositeTitleBackground", main_bkg.as_str()),
            state_label_color: self.raw.color("stateLabelColor", "#131300"),
            drop_shadow: self
                .raw
                .optional_value("dropShadow")
                .unwrap_or_else(|| "none".to_string()),
        }
    }
}

fn invert_hex_color(s: &str) -> Option<String> {
    let s = s.trim();
    let hex = s.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(format!("#{:02x}{:02x}{:02x}", 255 - r, 255 - g, 255 - b))
}

fn invert_timeline_label_color_to_hex(color: &str) -> Option<String> {
    let color = color.trim();
    if color.is_empty() {
        return None;
    }
    if color.eq_ignore_ascii_case("black") {
        return Some("#ffffff".to_string());
    }
    if color.eq_ignore_ascii_case("white") {
        return Some("#000000".to_string());
    }
    let hex = color.strip_prefix('#')?.trim();
    let (r, g, b) = match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            (r, g, b)
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            (r, g, b)
        }
        _ => return None,
    };
    Some(format!("#{:02x}{:02x}{:02x}", 255 - r, 255 - g, 255 - b))
}

fn parse_hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let t = s.trim().strip_prefix('#').unwrap_or(s.trim());
    if t.len() != 6 || !t.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let r = u8::from_str_radix(&t[0..2], 16).ok()?;
    let g = u8::from_str_radix(&t[2..4], 16).ok()?;
    let b = u8::from_str_radix(&t[4..6], 16).ok()?;
    Some((r, g, b))
}

fn parse_venn_hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let hex = s.trim().strip_prefix('#')?;
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some((r, g, b))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        _ => None,
    }
}

fn parse_venn_css_rgb(s: &str) -> Option<(u8, u8, u8)> {
    parse_venn_hex_rgb(s).or_else(|| parse_rgb_css(s))
}

fn venn_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn linear(channel: u8) -> f64 {
        let v = channel as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
}

fn adjust_hex_rgb(hex: &str, delta: i16) -> Option<String> {
    let (r, g, b) = parse_hex_rgb(hex)?;
    let adj = |c: u8| -> u8 {
        let v = c as i16 + delta;
        v.clamp(0, 255) as u8
    };
    Some(format!("#{:02x}{:02x}{:02x}", adj(r), adj(g), adj(b)))
}

fn round_1e10(v: f64) -> f64 {
    let v = (v * 1e10).round() / 1e10;
    if v == -0.0 { 0.0 } else { v }
}

fn format_hsl_css(h: f64, s: f64, l: f64, buf: &mut ryu_js::Buffer) -> String {
    let h = buf.format_finite(round_1e10(h)).to_string();
    let s = buf.format_finite(round_1e10(s)).to_string();
    let l = buf.format_finite(round_1e10(l)).to_string();
    format!("hsl({h}, {s}%, {l}%)")
}

fn derive_timeline_c_scale_inv_fallback(c_scale: &str, buf: &mut ryu_js::Buffer) -> Option<String> {
    let (h, s, l) = parse_hsl_css(c_scale)?;
    let h = (h + 180.0) % 360.0;
    let l = (l + 10.0).clamp(0.0, 100.0);
    Some(format_hsl_css(h, s, l, buf))
}

fn default_c_scale(i: usize) -> &'static str {
    match i {
        0 => "hsl(240, 100%, 76.2745098039%)",
        1 => "hsl(60, 100%, 73.5294117647%)",
        2 => "hsl(80, 100%, 76.2745098039%)",
        3 => "hsl(270, 100%, 76.2745098039%)",
        4 => "hsl(300, 100%, 76.2745098039%)",
        5 => "hsl(330, 100%, 76.2745098039%)",
        6 => "hsl(0, 100%, 76.2745098039%)",
        7 => "hsl(30, 100%, 76.2745098039%)",
        8 => "hsl(90, 100%, 76.2745098039%)",
        9 => "hsl(150, 100%, 76.2745098039%)",
        10 => "hsl(180, 100%, 76.2745098039%)",
        _ => "hsl(210, 100%, 76.2745098039%)",
    }
}

fn default_c_scale_peer(i: usize) -> &'static str {
    match i {
        0 => "hsl(240, 100%, 61.2745098039%)",
        1 => "hsl(60, 100%, 48.5294117647%)",
        2 => "hsl(80, 100%, 56.2745098039%)",
        3 => "hsl(270, 100%, 61.2745098039%)",
        4 => "hsl(300, 100%, 61.2745098039%)",
        5 => "hsl(330, 100%, 61.2745098039%)",
        6 => "hsl(0, 100%, 61.2745098039%)",
        7 => "hsl(30, 100%, 61.2745098039%)",
        8 => "hsl(90, 100%, 61.2745098039%)",
        9 => "hsl(150, 100%, 61.2745098039%)",
        10 => "hsl(180, 100%, 61.2745098039%)",
        _ => "hsl(210, 100%, 61.2745098039%)",
    }
}

fn default_c_scale_inv(i: usize) -> &'static str {
    match i {
        0 => "hsl(60, 100%, 86.2745098039%)",
        1 => "hsl(240, 100%, 83.5294117647%)",
        2 => "hsl(260, 100%, 86.2745098039%)",
        3 => "hsl(90, 100%, 86.2745098039%)",
        4 => "hsl(120, 100%, 86.2745098039%)",
        5 => "hsl(150, 100%, 86.2745098039%)",
        6 => "hsl(180, 100%, 86.2745098039%)",
        7 => "hsl(210, 100%, 86.2745098039%)",
        8 => "hsl(270, 100%, 86.2745098039%)",
        9 => "hsl(330, 100%, 86.2745098039%)",
        10 => "hsl(0, 100%, 86.2745098039%)",
        _ => "hsl(30, 100%, 86.2745098039%)",
    }
}

fn default_c_scale_label(i: usize) -> &'static str {
    match i {
        0 | 3 => "#ffffff",
        _ => "black",
    }
}

fn adjust_kanban_section_fill(
    c_scale: &str,
    dark_mode: bool,
    buf: &mut ryu_js::Buffer,
) -> Option<String> {
    let (h, s, l) = parse_hsl_css(c_scale)?;
    let delta = if dark_mode { -10.0 } else { 10.0 };
    Some(format_hsl_css(h, s, (l + delta).clamp(0.0, 100.0), buf))
}

fn journey_default_fill_type(i: usize) -> &'static str {
    match i {
        0 => "#ECECFF",
        1 => "#ffffde",
        2 => "hsl(304, 100%, 96.2745098039%)",
        3 => "hsl(124, 100%, 93.5294117647%)",
        4 => "hsl(176, 100%, 96.2745098039%)",
        5 => "hsl(-4, 100%, 93.5294117647%)",
        6 => "hsl(8, 100%, 96.2745098039%)",
        _ => "hsl(188, 100%, 93.5294117647%)",
    }
}

fn fmt_rgb(r: u8, g: u8, b: u8) -> String {
    format!("rgb({r}, {g}, {b})")
}

fn parse_rgb_css(s: &str) -> Option<(u8, u8, u8)> {
    let inner = s.trim().strip_prefix("rgb(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(|p| p.trim());
    let parse_channel = |part: &str| -> Option<u8> {
        let value = part.parse::<f64>().ok()?;
        if !value.is_finite() {
            return None;
        }
        Some(value.round().clamp(0.0, 255.0) as u8)
    };
    let r = parse_channel(parts.next()?)?;
    let g = parse_channel(parts.next()?)?;
    let b = parse_channel(parts.next()?)?;
    Some((r, g, b))
}

fn parse_hsl_css(s: &str) -> Option<(f64, f64, f64)> {
    let inner = s.trim().strip_prefix("hsl(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(|p| p.trim());
    let h = parts.next()?.parse::<f64>().ok()?;
    let s = parts
        .next()?
        .strip_suffix('%')
        .unwrap_or_default()
        .parse::<f64>()
        .ok()?;
    let l = parts
        .next()?
        .strip_suffix('%')
        .unwrap_or_default()
        .parse::<f64>()
        .ok()?;
    Some((h, s, l))
}

fn hsl_to_rgb_u8(h_deg: f64, s_pct: f64, l_pct: f64) -> Option<(u8, u8, u8)> {
    if !(h_deg.is_finite() && s_pct.is_finite() && l_pct.is_finite()) {
        return None;
    }

    let h = (h_deg / 360.0).rem_euclid(1.0);
    let s = (s_pct / 100.0).clamp(0.0, 1.0);
    let l = (l_pct / 100.0).clamp(0.0, 1.0);

    if s == 0.0 {
        let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
        return Some((v, v, v));
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    let to_u8 = |v: f64| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    Some((to_u8(r), to_u8(g), to_u8(b)))
}

fn css_color_to_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let t = s.trim();
    if let Some(rgb) = parse_rgb_css(t) {
        return Some(rgb);
    }
    if let Some(rgb) = parse_hex_rgb(t) {
        return Some(rgb);
    }
    if let Some((h, s, l)) = parse_hsl_css(t) {
        return hsl_to_rgb_u8(h, s, l);
    }
    None
}

fn css_color_to_rgb_string(s: &str) -> Option<String> {
    let (r, g, b) = css_color_to_rgb(s)?;
    Some(fmt_rgb(r, g, b))
}

fn parse_treemap_css_rgb(color: &str) -> Option<(u8, u8, u8)> {
    let color = color.trim();
    if color.eq_ignore_ascii_case("black") {
        return Some((0, 0, 0));
    }
    if color.eq_ignore_ascii_case("white") {
        return Some((255, 255, 255));
    }
    if let Some(hex) = color.strip_prefix('#') {
        let hex = hex.trim();
        if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            return Some((r, g, b));
        }
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some((r, g, b));
        }
    }
    let lower = color.to_ascii_lowercase();
    if let Some(args) = lower
        .strip_prefix("rgb(")
        .and_then(|value| value.strip_suffix(')'))
    {
        let parts = args
            .split(',')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        if parts.len() >= 3 {
            let r = parts[0].parse::<u16>().ok()?;
            let g = parts[1].parse::<u16>().ok()?;
            let b = parts[2].parse::<u16>().ok()?;
            if r <= 255 && g <= 255 && b <= 255 {
                return Some((r as u8, g as u8, b as u8));
            }
        }
    }
    None
}

fn invert_treemap_label_color_to_hex(color: &str) -> Option<String> {
    let (r, g, b) = parse_treemap_css_rgb(color)?;
    Some(format!(
        "#{:02x}{:02x}{:02x}",
        255u8.saturating_sub(r),
        255u8.saturating_sub(g),
        255u8.saturating_sub(b)
    ))
}

fn css_color_is_transparent(color: &str) -> bool {
    color.trim().eq_ignore_ascii_case("transparent")
}

fn css_color_is_white_like(color: &str) -> bool {
    parse_treemap_css_rgb(color).is_some_and(|(r, g, b)| r >= 250 && g >= 250 && b >= 250)
}

fn style_has_non_empty_decl(style: &str, property: &str) -> bool {
    style.split(';').any(|decl| {
        let Some((key, value)) = decl.split_once(':') else {
            return false;
        };
        key.trim().eq_ignore_ascii_case(property) && !value.trim().is_empty()
    })
}

fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn to_linear(channel: u8) -> f64 {
        let v = channel as f64 / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b)
}

fn rgb_to_hsl_pct(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if (max - min).abs() < f64::EPSILON {
        return (0.0, 0.0, l * 100.0);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if (max - r).abs() < f64::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f64::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    } / 6.0;

    (h * 360.0, s * 100.0, l * 100.0)
}

fn derive_quadrant_point_fill(quadrant1_fill: &str, fallback: &str) -> String {
    let Some((r, g, b)) = css_color_to_rgb(quadrant1_fill) else {
        return fallback.to_string();
    };
    let (h, s, l) = rgb_to_hsl_pct(r, g, b);
    let delta = if relative_luminance(r, g, b) < 0.5 {
        10.0
    } else {
        -10.0
    };
    let adjusted_l = (l + delta).clamp(0.0, 100.0);
    let Some((r, g, b)) = hsl_to_rgb_u8(h, s, adjusted_l) else {
        return fallback.to_string();
    };
    fmt_rgb(r, g, b)
}

fn is_invalid_css_token(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    lower.is_empty()
        || lower.contains("nan")
        || lower.contains("undefined")
        || lower.contains("infinity")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn presentation_theme_node_diagram_uses_shared_fallbacks() {
        let cfg = json!({});
        let theme = PresentationTheme::new(&cfg);
        let node = theme.node_diagram();

        assert_eq!(node.common.text_color, "#333");
        assert_eq!(node.common.line_color, "#333333");
        assert_eq!(node.node_text_color, "#333");
        assert_eq!(node.title_color, "#333");
        assert_eq!(node.main_bkg, "#ECECFF");
        assert_eq!(node.node_border, "#9370DB");
        assert_eq!(node.arrowhead_color, "#333333");
        assert_eq!(node.stroke_width, "1");
    }

    #[test]
    fn presentation_theme_sequence_neo_uses_drop_shadow_for_label_box_filter() {
        let cfg = json!({
            "look": "neo",
            "themeVariables": {
                "dropShadow": "drop-shadow(1px 2px 3px rgba(0,0,0,.4))"
            }
        });
        let theme = PresentationTheme::new(&cfg);

        let sequence = theme.sequence_diagram();
        assert_eq!(
            sequence.label_box_filter,
            "drop-shadow(1px 2px 3px rgba(0,0,0,.4))"
        );
    }

    #[test]
    fn presentation_theme_xychart_resolves_chart_roles() {
        let cfg = json!({
            "theme": "neo",
            "themeVariables": {
                "primaryColor": "#123456",
                "primaryTextColor": "#f8fafc",
                "background": "#010203",
                "xyChart": {
                    "backgroundColor": "#0f172a",
                    "titleColor": "#f43f5e",
                    "xAxisLabelColor": "#22c55e",
                    "plotColorPalette": "#001122, #334455"
                }
            }
        });

        let xychart = PresentationTheme::new(&cfg).xychart();

        assert_eq!(xychart.background_color, "#0f172a");
        assert_eq!(xychart.title_color, "#f43f5e");
        assert_eq!(xychart.x_axis_title_color, "#f8fafc");
        assert_eq!(xychart.x_axis_label_color, "#22c55e");
        assert_eq!(xychart.y_axis_line_color, "#f8fafc");
        assert_eq!(
            xychart.plot_color_palette,
            vec!["#001122".to_string(), "#334455".to_string()]
        );
    }

    #[test]
    fn presentation_theme_quadrantchart_resolves_chart_roles() {
        let cfg = json!({
            "theme": "redux-dark",
            "themeVariables": {
                "primaryColor": "#123456",
                "primaryTextColor": "#f8fafc",
                "primaryBorderColor": "#445566",
                "quadrant1Fill": "#010203",
                "quadrant2Fill": "#020304",
                "quadrant3Fill": "#030405",
                "quadrant4Fill": "#040506",
                "quadrantPointFill": "#facc15",
                "quadrantPointTextFill": "#111827",
                "quadrantXAxisTextFill": "#22c55e",
                "quadrantYAxisTextFill": "#38bdf8",
                "quadrantTitleFill": "#f43f5e",
                "quadrantInternalBorderStrokeFill": "#aabbcc",
                "quadrantExternalBorderStrokeFill": "#ddeeff"
            }
        });

        let quadrant = PresentationTheme::new(&cfg).quadrantchart();

        assert_eq!(quadrant.quadrant1_fill, "#010203");
        assert_eq!(quadrant.quadrant2_fill, "#020304");
        assert_eq!(quadrant.quadrant1_text_fill, "#f8fafc");
        assert_eq!(quadrant.quadrant_point_fill, "#facc15");
        assert_eq!(quadrant.quadrant_point_text_fill, "#111827");
        assert_eq!(quadrant.quadrant_x_axis_text_fill, "#22c55e");
        assert_eq!(quadrant.quadrant_y_axis_text_fill, "#38bdf8");
        assert_eq!(quadrant.quadrant_title_fill, "#f43f5e");
        assert_eq!(quadrant.quadrant_internal_border_stroke_fill, "#aabbcc");
        assert_eq!(quadrant.quadrant_external_border_stroke_fill, "#ddeeff");
    }

    #[test]
    fn presentation_theme_tree_view_resolves_tree_view_roles() {
        let cfg = json!({
            "themeVariables": {
                "treeView": {
                    "labelFontSize": "20px",
                    "labelColor": "#FF0000",
                    "lineColor": "#00FF00"
                }
            }
        });

        let tree_view = PresentationTheme::new(&cfg).tree_view();

        assert_eq!(tree_view.label_font_size, 20.0);
        assert_eq!(tree_view.label_font_size_css, "20px");
        assert_eq!(tree_view.label_color, "#FF0000");
        assert_eq!(tree_view.line_color, "#00FF00");
    }

    #[test]
    fn presentation_theme_tree_view_uses_default_tree_view_roles() {
        let cfg = json!({});

        let tree_view = PresentationTheme::new(&cfg).tree_view();

        assert_eq!(tree_view.label_font_size, 16.0);
        assert_eq!(tree_view.label_font_size_css, "16px");
        assert_eq!(tree_view.label_color, "black");
        assert_eq!(tree_view.line_color, "black");
    }

    #[test]
    fn presentation_theme_treemap_resolves_top_level_treemap_roles() {
        let cfg = json!({
            "theme": "custom",
            "themeVariables": {
                "textColor": "#101010",
                "titleColor": "#202020",
                "labelTextColor": "rgb(10, 20, 30)",
                "cScale0": "#010203",
                "cScalePeer0": "#040506",
                "cScaleLabel0": "#070809"
            },
            "treemap": {
                "titleColor": "#777777",
                "labelColor": "#555555",
                "valueColor": "#666666",
                "sectionStrokeColor": "#111111",
                "sectionStrokeWidth": 2,
                "sectionFillColor": "#222222",
                "leafStrokeColor": "#333333",
                "leafStrokeWidth": "3px",
                "leafFillColor": "#444444",
                "labelFontSize": "13px",
                "valueFontSize": "11px",
                "titleFontSize": "15px"
            }
        });

        let treemap = PresentationTheme::new(&cfg).treemap();

        assert_eq!(treemap.title_color, "#777777");
        assert_eq!(treemap.label_color, "#555555");
        assert_eq!(treemap.value_color, "#666666");
        assert_eq!(treemap.section_stroke_color, "#111111");
        assert_eq!(treemap.section_stroke_width, "2");
        assert_eq!(treemap.section_fill_color, "#222222");
        assert_eq!(treemap.leaf_stroke_color, "#333333");
        assert_eq!(treemap.leaf_stroke_width, "3px");
        assert_eq!(treemap.leaf_fill_color, "#444444");
        assert_eq!(treemap.label_font_size, "13px");
        assert_eq!(treemap.value_font_size, "11px");
        assert_eq!(treemap.title_font_size, "15px");
        assert_eq!(treemap.color_scale[0], "#010203");
        assert_eq!(treemap.color_scale_peer[0], "#040506");
        assert_eq!(treemap.color_scale_label[0], "#070809");
    }

    #[test]
    fn presentation_theme_treemap_uses_text_and_title_fallbacks() {
        let cfg = json!({
            "themeVariables": {
                "textColor": "#101010",
                "titleColor": "#202020"
            }
        });

        let treemap = PresentationTheme::new(&cfg).treemap();

        assert_eq!(treemap.title_color, "#202020");
        assert_eq!(treemap.label_color, "#101010");
        assert_eq!(treemap.value_color, "#101010");
    }

    #[test]
    fn presentation_theme_treemap_uses_default_scales_and_label_inversion() {
        let cfg = json!({
            "themeVariables": {
                "labelTextColor": "rgb(10, 20, 30)"
            }
        });

        let treemap = PresentationTheme::new(&cfg).treemap();

        assert_eq!(treemap.color_scale[0], "hsl(240, 100%, 76.2745098039%)");
        assert_eq!(
            treemap.color_scale_peer[0],
            "hsl(240, 100%, 61.2745098039%)"
        );
        assert_eq!(treemap.color_scale_label[0], "#f5ebe1");
        assert_eq!(treemap.color_scale_label[1], "rgb(10, 20, 30)");
        assert_eq!(treemap.color_scale_label[3], "#f5ebe1");
        assert_eq!(
            treemap.readable_leaf_label_fill("transparent", "", "#ffffff".to_string()),
            "#333"
        );
    }

    #[test]
    fn presentation_theme_gantt_resolves_gantt_roles() {
        let cfg = json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#707070",
                "excludeBkgColor": "#101010",
                "sectionBkgColor": "#202020",
                "sectionBkgColor2": "#303030",
                "altSectionBkgColor": "#404040",
                "titleColor": "#505050",
                "gridColor": "#606060",
                "todayLineColor": "#808080",
                "taskTextDarkColor": "#909090",
                "taskTextClickableColor": "#a0a0a0",
                "taskTextColor": "#b0b0b0",
                "taskBkgColor": "#c0c0c0",
                "taskBorderColor": "#d0d0d0",
                "taskTextOutsideColor": "#e0e0e0",
                "activeTaskBkgColor": "#111111",
                "activeTaskBorderColor": "#222222",
                "doneTaskBorderColor": "#333333",
                "doneTaskBkgColor": "#444444",
                "critBorderColor": "#555555",
                "critBkgColor": "#666666",
                "vertLineColor": "#777777"
            }
        });

        let gantt = PresentationTheme::new(&cfg).gantt();

        assert_eq!(gantt.font_family, r#""ibm plex sans",arial,sans-serif"#);
        assert_eq!(gantt.text_color, "#707070");
        assert_eq!(gantt.exclude_bkg_color, "#101010");
        assert_eq!(gantt.section_bkg_color, "#202020");
        assert_eq!(gantt.section_bkg_color2, "#303030");
        assert_eq!(gantt.alt_section_bkg_color, "#404040");
        assert_eq!(gantt.title_color, "#505050");
        assert_eq!(gantt.title_text_color, "#505050");
        assert_eq!(gantt.grid_color, "#606060");
        assert_eq!(gantt.today_line_color, "#808080");
        assert_eq!(gantt.task_text_dark_color, "#909090");
        assert_eq!(gantt.task_text_clickable_color, "#a0a0a0");
        assert_eq!(gantt.task_text_color, "#b0b0b0");
        assert_eq!(gantt.task_bkg_color, "#c0c0c0");
        assert_eq!(gantt.task_border_color, "#d0d0d0");
        assert_eq!(gantt.task_text_outside_color, "#e0e0e0");
        assert_eq!(gantt.active_task_bkg_color, "#111111");
        assert_eq!(gantt.active_task_border_color, "#222222");
        assert_eq!(gantt.done_task_border_color, "#333333");
        assert_eq!(gantt.done_task_bkg_color, "#444444");
        assert_eq!(gantt.crit_border_color, "#555555");
        assert_eq!(gantt.crit_bkg_color, "#666666");
        assert_eq!(gantt.vert_line_color, "#777777");
    }

    #[test]
    fn presentation_theme_gantt_uses_text_color_for_empty_title_color() {
        let cfg = json!({
            "themeVariables": {
                "textColor": "#707070",
                "titleColor": "   "
            }
        });

        let gantt = PresentationTheme::new(&cfg).gantt();

        assert_eq!(gantt.title_color, "   ");
        assert_eq!(gantt.title_text_color, "#707070");
    }

    #[test]
    fn presentation_theme_kanban_resolves_theme_roles() {
        let cfg = json!({
            "themeVariables": {
                "background": "#0f172a",
                "nodeBorder": "#38bdf8",
                "textColor": "#f8fafc",
                "git0": "#22c55e",
                "gitBranchLabel0": "#020617",
                "cScale0": "hsl(160, 80%, 40%)",
                "cScaleLabel0": "#f8fafc",
                "cScaleInv0": "#111827"
            }
        });

        let kanban = PresentationTheme::new(&cfg).kanban();

        assert_eq!(kanban.text_color, "#f8fafc");
        assert_eq!(kanban.background, "#0f172a");
        assert_eq!(kanban.node_border, "#38bdf8");
        assert_eq!(kanban.root_fill, "#22c55e");
        assert_eq!(kanban.root_label, "#020617");
        assert_eq!(kanban.sections[0].c_scale, "hsl(160, 80%, 40%)");
        assert_eq!(kanban.sections[0].section_fill, "hsl(160, 80%, 50%)");
        assert_eq!(kanban.sections[0].c_scale_label, "#f8fafc");
        assert_eq!(kanban.sections[0].c_scale_inv, "#111827");
    }

    #[test]
    fn presentation_theme_kanban_dark_mode_adjusts_section_fill_down() {
        let cfg = json!({
            "darkMode": true
        });

        let kanban = PresentationTheme::new(&cfg).kanban();

        assert_eq!(kanban.sections[0].c_scale, "hsl(240, 100%, 76.2745098039%)");
        assert_eq!(
            kanban.sections[0].section_fill,
            "hsl(240, 100%, 66.2745098039%)"
        );
        assert_eq!(kanban.sections[0].c_scale_label, "#ffffff");
        assert_eq!(
            kanban.sections[0].c_scale_inv,
            "hsl(60, 100%, 86.2745098039%)"
        );
    }

    #[test]
    fn presentation_theme_timeline_resolves_timeline_roles() {
        let cfg = json!({
            "theme": "redux-color",
            "themeVariables": {
                "THEME_COLOR_LIMIT": 2,
                "strokeWidth": 5,
                "fontWeight": 600,
                "mainBkg": "#111827",
                "nodeBorder": "#38bdf8",
                "dropShadow": "drop-shadow(1px 2px 2px rgba(0,0,0,.4))",
                "tertiaryColor": "#334155",
                "clusterBorder": "#f97316",
                "git0": "#22c55e",
                "gitBranchLabel0": "#020617",
                "borderColorArray": ["#ef4444", "#f59e0b"],
                "cScale0": "#ef4444",
                "cScaleLabel0": "#e879f9",
                "cScaleInv0": "#334155",
                "cScale1": "#172554",
                "cScaleLabel1": "#f8fafc",
                "cScaleInv1": "#475569"
            }
        });

        let timeline = PresentationTheme::new(&cfg).timeline();

        assert!(timeline.is_redux_theme);
        assert!(!timeline.is_dark_theme);
        assert!(timeline.is_color_theme);
        assert_eq!(timeline.stroke_width, "5");
        assert_eq!(timeline.font_weight, "600");
        assert_eq!(timeline.main_bkg, "#111827");
        assert_eq!(timeline.node_border, "#38bdf8");
        assert_eq!(timeline.disabled_fill, "#334155");
        assert_eq!(timeline.disabled_text_fill, "#f97316");
        assert_eq!(timeline.root_fill, "#22c55e");
        assert_eq!(timeline.root_label, "#020617");
        assert_eq!(timeline.border_colors, vec!["#ef4444", "#f59e0b"]);
        assert_eq!(timeline.sections.len(), 2);
        assert_eq!(timeline.sections[0].c_scale, "#ef4444");
        assert_eq!(timeline.sections[0].c_scale_label, "#e879f9");
        assert_eq!(timeline.sections[0].c_scale_inv, "#334155");
        assert_eq!(timeline.sections[1].c_scale, "#172554");
        assert_eq!(timeline.sections[1].c_scale_label, "#f8fafc");
        assert_eq!(timeline.sections[1].c_scale_inv, "#475569");
    }

    #[test]
    fn presentation_theme_timeline_uses_default_timeline_roles() {
        let cfg = json!({});

        let timeline = PresentationTheme::new(&cfg).timeline();

        assert!(!timeline.is_redux_theme);
        assert!(!timeline.is_dark_theme);
        assert!(!timeline.is_color_theme);
        assert_eq!(timeline.stroke_width, "1");
        assert_eq!(timeline.font_weight, "normal");
        assert_eq!(timeline.main_bkg, "#ECECFF");
        assert_eq!(timeline.node_border, "#9370DB");
        assert_eq!(timeline.disabled_fill, "lightgray");
        assert_eq!(timeline.disabled_text_fill, "#efefef");
        assert_eq!(timeline.root_fill, "hsl(240, 100%, 46.2745098039%)");
        assert_eq!(timeline.root_label, "#ffffff");
        assert!(timeline.border_colors.is_empty());
        assert_eq!(timeline.sections.len(), 12);
        assert_eq!(
            timeline.sections[0].c_scale,
            "hsl(240, 100%, 76.2745098039%)"
        );
        assert_eq!(timeline.sections[0].c_scale_label, "#ffffff");
        assert_eq!(
            timeline.sections[0].c_scale_inv,
            "hsl(60, 100%, 86.2745098039%)"
        );
        assert_eq!(timeline.sections[1].c_scale_label, "black");
    }

    #[test]
    fn presentation_theme_eventmodeling_resolves_eventmodeling_roles() {
        let cfg = json!({
            "fontFamily": "Inter, sans-serif",
            "themeVariables": {
                "textColor": "#111111",
                "emUiFill": "#fefefe",
                "emUiStroke": "#222222",
                "emCommandFill": "#DDEEFF",
                "emCommandStroke": "#336699",
                "emSwimlaneBackgroundOdd": "#fafafa",
                "emSwimlaneBackgroundStroke": "#efefef",
                "emRelationStroke": "#135790",
                "emArrowhead": "#02468a"
            }
        });

        let eventmodeling = PresentationTheme::new(&cfg).eventmodeling();

        assert_eq!(eventmodeling.font_family_css, "Inter,sans-serif");
        assert_eq!(eventmodeling.text_color, "#111111");
        assert_eq!(eventmodeling.ui_fill, "#fefefe");
        assert_eq!(eventmodeling.ui_stroke, "#222222");
        assert_eq!(eventmodeling.command_fill, "#DDEEFF");
        assert_eq!(eventmodeling.command_stroke, "#336699");
        assert_eq!(eventmodeling.swimlane_background_fill, "#fafafa");
        assert_eq!(eventmodeling.swimlane_background_stroke, "#efefef");
        assert_eq!(eventmodeling.relation_stroke, "#135790");
        assert_eq!(eventmodeling.arrowhead_fill, "#02468a");
    }

    #[test]
    fn presentation_theme_eventmodeling_uses_default_eventmodeling_roles() {
        let cfg = json!({});

        let eventmodeling = PresentationTheme::new(&cfg).eventmodeling();

        assert_eq!(
            eventmodeling.font_family_css,
            "\"trebuchet ms\",verdana,arial,sans-serif"
        );
        assert_eq!(eventmodeling.text_color, "#333");
        assert_eq!(eventmodeling.ui_fill, "white");
        assert_eq!(eventmodeling.ui_stroke, "#dbdada");
        assert_eq!(eventmodeling.swimlane_background_fill, "rgb(250,250,250)");
        assert_eq!(eventmodeling.swimlane_background_stroke, "rgb(240,240,240)");
        assert_eq!(eventmodeling.relation_stroke, "#000");
        assert_eq!(eventmodeling.arrowhead_fill, "#000000");
    }

    #[test]
    fn presentation_theme_ishikawa_resolves_ishikawa_roles() {
        let cfg = json!({
            "fontFamily": "Inter, sans-serif",
            "themeVariables": {
                "lineColor": "#008800",
                "mainBkg": "#FFFFFF",
                "textColor": "#111111",
                "fontFamily": "Ignored, sans-serif"
            }
        });

        let ishikawa = PresentationTheme::new(&cfg).ishikawa();

        assert_eq!(ishikawa.line_color, "#008800");
        assert_eq!(ishikawa.main_bkg, "#FFFFFF");
        assert_eq!(ishikawa.text_color, "#111111");
        assert_eq!(ishikawa.font_family, "Inter, sans-serif");
    }

    #[test]
    fn presentation_theme_ishikawa_uses_default_ishikawa_roles() {
        let cfg = json!({});

        let ishikawa = PresentationTheme::new(&cfg).ishikawa();

        assert_eq!(ishikawa.line_color, "#333");
        assert_eq!(ishikawa.main_bkg, "#fff");
        assert_eq!(ishikawa.text_color, "#333");
        assert_eq!(
            ishikawa.font_family,
            "trebuchet ms, verdana, arial, sans-serif"
        );
    }

    #[test]
    fn presentation_theme_venn_resolves_venn_roles() {
        let cfg = json!({
            "theme": "dark",
            "fontFamily": "Inter, sans-serif",
            "themeVariables": {
                "background": "#111111",
                "vennTitleTextColor": "#f43f5e",
                "vennSetTextColor": "#22c55e",
                "venn1": "#123456",
                "venn2": "#abcdef",
                "primaryColor": "#987654",
                "primaryTextColor": "#eeeeee",
                "textColor": "#dddddd"
            }
        });

        let venn = PresentationTheme::new(&cfg).venn();

        assert_eq!(venn.font_family_css, "Inter,sans-serif");
        assert_eq!(venn.title_color, "#f43f5e");
        assert_eq!(venn.set_text_color, "#22c55e");
        assert_eq!(venn.circle_colors, vec!["#123456", "#abcdef"]);
        assert_eq!(venn.primary_color, "#987654");
        assert!(venn.is_dark_theme);
        assert_eq!(venn.circle_text_color("#123456"), "#597189");
    }

    #[test]
    fn presentation_theme_venn_uses_default_venn_roles() {
        let cfg = json!({});

        let venn = PresentationTheme::new(&cfg).venn();

        assert_eq!(
            venn.font_family_css,
            "\"trebuchet ms\",verdana,arial,sans-serif"
        );
        assert_eq!(venn.title_color, "#333");
        assert_eq!(venn.set_text_color, "#333");
        assert!(venn.circle_colors.is_empty());
        assert_eq!(venn.primary_color, "#ECECFF");
        assert!(!venn.is_dark_theme);
        assert_eq!(venn.circle_text_color("#abc"), "#77838f");
        assert_eq!(venn.circle_text_color("not-a-color"), "#000000");
    }

    #[test]
    fn presentation_theme_journey_resolves_style_roles() {
        let cfg = json!({
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "textColor": "#101010",
                "lineColor": "#202020",
                "faceColor": "#303030",
                "mainBkg": "#404040",
                "nodeBorder": "#505050",
                "arrowheadColor": "#606060",
                "edgeLabelBackground": "#707070",
                "titleColor": "#808080",
                "tertiaryColor": "#909090",
                "border2": "#a0a0a0",
                "fillType0": "#b0b0b0",
                "fillType1": "#c0c0c0",
                "actor0": "#d0d0d0",
                "actor1": "#e0e0e0"
            }
        });

        let journey = PresentationTheme::new(&cfg).journey();

        assert_eq!(
            journey.font_family_css,
            "\"ibm plex sans\",arial,sans-serif"
        );
        assert_eq!(journey.text_color, "#101010");
        assert_eq!(journey.line_color, "#202020");
        assert_eq!(journey.face_color, "#303030");
        assert_eq!(journey.main_bkg, "#404040");
        assert_eq!(journey.node_border, "#505050");
        assert_eq!(journey.arrowhead_color, "#606060");
        assert_eq!(journey.edge_label_background, "#707070");
        assert_eq!(journey.title_color, "#808080");
        assert_eq!(journey.tertiary_color, "#909090");
        assert_eq!(journey.border2, "#a0a0a0");
        assert_eq!(journey.fill_types[0], "#b0b0b0");
        assert_eq!(journey.fill_types[1], "#c0c0c0");
        assert_eq!(journey.fill_types[7], "hsl(188, 100%, 93.5294117647%)");
        assert_eq!(journey.actor_colors[0].as_deref(), Some("#d0d0d0"));
        assert_eq!(journey.actor_colors[1].as_deref(), Some("#e0e0e0"));
        assert_eq!(journey.actor_colors[5], None);
    }

    #[test]
    fn presentation_theme_journey_uses_default_style_roles() {
        let cfg = json!({});

        let journey = PresentationTheme::new(&cfg).journey();

        assert_eq!(
            journey.font_family_css,
            "\"trebuchet ms\",verdana,arial,sans-serif"
        );
        assert_eq!(journey.text_color, "#333");
        assert_eq!(journey.line_color, "#333333");
        assert_eq!(journey.face_color, "#FFF8DC");
        assert_eq!(journey.main_bkg, "#ECECFF");
        assert_eq!(journey.node_border, "#9370DB");
        assert_eq!(journey.arrowhead_color, "#333333");
        assert_eq!(journey.edge_label_background, "rgba(232,232,232, 0.8)");
        assert_eq!(journey.title_color, "#333");
        assert_eq!(journey.tertiary_color, "hsl(80, 100%, 96.2745098039%)");
        assert_eq!(journey.border2, "#aaaa33");
        assert_eq!(journey.fill_types.len(), 8);
        assert_eq!(journey.fill_types[0], "#ECECFF");
        assert_eq!(journey.fill_types[1], "#ffffde");
        assert_eq!(journey.fill_types[2], "hsl(304, 100%, 96.2745098039%)");
        assert_eq!(
            journey.actor_colors,
            vec![None, None, None, None, None, None]
        );
    }

    #[test]
    fn presentation_theme_radar_resolves_style_roles() {
        let cfg = json!({
            "fontFamily": "Ignored, sans-serif",
            "themeVariables": {
                "fontFamily": "\"ibm plex sans\", arial, sans-serif",
                "fontSize": 18,
                "textColor": "#101010",
                "lineColor": "#111111",
                "errorBkgColor": "#121212",
                "errorTextColor": "#131313",
                "titleColor": "#202020",
                "cScale0": "#303030",
                "radar": {
                    "axisColor": "#404040",
                    "axisStrokeWidth": 2,
                    "axisLabelFontSize": 12,
                    "graticuleColor": "#505050",
                    "graticuleOpacity": 0.3,
                    "graticuleStrokeWidth": 1,
                    "legendFontSize": 12,
                    "curveOpacity": 0.5,
                    "curveStrokeWidth": 2
                }
            },
            "radar": {
                "axisColor": "#606060",
                "axisStrokeWidth": 4,
                "axisLabelFontSize": 14,
                "graticuleColor": "#707070",
                "graticuleOpacity": 0.8,
                "graticuleStrokeWidth": 5,
                "legendFontSize": 16,
                "curveOpacity": 0.9,
                "curveStrokeWidth": 6
            }
        });

        let radar = PresentationTheme::new(&cfg).radar();

        assert_eq!(radar.font_family_css, "\"ibm plex sans\",arial,sans-serif");
        assert_eq!(radar.base_font_size_css, "18");
        assert_eq!(radar.title_font_size_css, "18");
        assert_eq!(radar.text_color, "#101010");
        assert_eq!(radar.line_color, "#111111");
        assert_eq!(radar.error_bkg_color, "#121212");
        assert_eq!(radar.error_text_color, "#131313");
        assert_eq!(radar.title_color, "#202020");
        assert_eq!(radar.axis_color, "#606060");
        assert_eq!(radar.axis_stroke_width, 4.0);
        assert_eq!(radar.axis_label_font_size, 14.0);
        assert_eq!(radar.graticule_color, "#707070");
        assert_eq!(radar.graticule_opacity, 0.8);
        assert_eq!(radar.graticule_stroke_width, 5.0);
        assert_eq!(radar.legend_font_size, 16.0);
        assert_eq!(radar.curve_opacity, 0.9);
        assert_eq!(radar.curve_stroke_width, 6.0);
        assert_eq!(radar.series_colors[0], "#303030");
        assert_eq!(radar.series_colors[11], "hsl(210, 100%, 76.2745098039%)");
    }

    #[test]
    fn presentation_theme_radar_uses_default_style_roles() {
        let cfg = json!({});

        let radar = PresentationTheme::new(&cfg).radar();

        assert_eq!(
            radar.font_family_css,
            "\"trebuchet ms\",verdana,arial,sans-serif"
        );
        assert_eq!(radar.base_font_size_css, "16px");
        assert_eq!(radar.title_font_size_css, "16px");
        assert_eq!(radar.text_color, "#333");
        assert_eq!(radar.line_color, "#333333");
        assert_eq!(radar.error_bkg_color, "#552222");
        assert_eq!(radar.error_text_color, "#552222");
        assert_eq!(radar.title_color, "#333");
        assert_eq!(radar.axis_color, "#333333");
        assert_eq!(radar.axis_stroke_width, 2.0);
        assert_eq!(radar.axis_label_font_size, 12.0);
        assert_eq!(radar.graticule_color, "#DEDEDE");
        assert_eq!(radar.graticule_opacity, 0.3);
        assert_eq!(radar.graticule_stroke_width, 1.0);
        assert_eq!(radar.legend_font_size, 12.0);
        assert_eq!(radar.curve_opacity, 0.5);
        assert_eq!(radar.curve_stroke_width, 2.0);
        assert_eq!(radar.series_colors.len(), 12);
        assert_eq!(radar.series_colors[0], "hsl(240, 100%, 76.2745098039%)");
    }
}
