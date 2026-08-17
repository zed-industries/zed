use super::*;
use crate::flowchart::flowchart_label_metrics_for_layout;

#[test]
fn html_br_trims_trailing_space_before_break_for_flowchart_labels() {
    let plain =
        crate::flowchart::flowchart_label_plain_text_for_layout("Hexagon <br> end", "text", true);
    assert_eq!(plain, "Hexagon\nend");

    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let m = measurer.measure_wrapped(&plain, &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(m.width, 60.984375);
    assert_eq!(m.height, 48.0);
}

#[test]
fn flowchart_html_text_extraction_preserves_bare_comparison_symbols() {
    let plain = crate::flowchart::flowchart_label_plain_text_for_layout(
        "标题 Unicode — 測試 &amp; &lt; &gt; and x < y > z",
        "text",
        true,
    );
    assert_eq!(plain, "标题 Unicode — 測試 & < > and x < y > z");
}

#[test]
fn flowchart_html_unicode_entity_title_width_matches_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "标题 Unicode — 測試 & < >",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(metrics.width, 190.578125);
    assert_eq!(metrics.height, 24.0);
    assert_eq!(metrics.line_count, 1);

    let plain_cjk = measurer.measure_wrapped("负责人审批", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(plain_cjk.width, 80.0);
}

#[test]
fn flowchart_html_unicode_block_fallback_widths_match_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let emoji = measurer.measure_wrapped("emoji: 😀😅👍", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(emoji.width, 117.625);
    assert_eq!(emoji.height, 24.0);

    let rtl = measurer.measure_wrapped("rtl: שלום-עולם", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(rtl.width, 95.296875);
    assert_eq!(rtl.height, 24.0);

    let cjk_hangul = measurer.measure_wrapped(
        "中文 / 日本語 / 한글",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(cjk_hangul.width, 148.0625);
    assert_eq!(cjk_hangul.height, 24.0);

    let path = measurer.measure_wrapped(
        "Path: C:\\Temp\\merman\\out.svg (Windows-style)",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(path.width, 203.15625);
    assert_eq!(path.height, 72.0);
}

#[test]
fn markdown_strong_width_matches_flowchart_table() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let regular_html = measurer.measure_wrapped("Two", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(regular_html.width, 27.578125);

    let strong_html = measure_markdown_with_flowchart_bold_deltas(
        &measurer,
        "**Two**",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(strong_html.width, 30.109375);

    let regular_svg = measurer.measure_wrapped("Two", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(regular_svg.width, 28.984375);

    let strong_svg = measure_markdown_with_flowchart_bold_deltas(
        &measurer,
        "**Two**",
        &style,
        Some(200.0),
        WrapMode::SvgLike,
    );
    // Mermaid's SVG cluster-title probe for `` `**Two**` `` lands on the same total width as the
    // HTML-label measurement, even though the regular SVG token baseline is wider.
    assert_eq!(strong_svg.width, strong_html.width);
    assert_eq!(strong_svg.width - regular_svg.width, 1.125);
}

#[test]
fn flowchart_html_unwrapped_width_matches_upstream_at_30px() {
    // Mermaid upstream fixture:
    // fixtures/upstream-svgs/flowchart/upstream_flowchart_v2_bigger_font_from_classes_spec.svg
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 30.0,
        font_weight: None,
    };

    let m = measurer.measure_wrapped("I am a circle", &style, None, WrapMode::HtmlLike);
    assert_eq!(m.width, 167.03125);
    assert_eq!(m.height, 45.0);
    assert_eq!(m.line_count, 1);
}

#[test]
fn flowchart_html_fontawesome_icon_width_uses_nominal_boundary() {
    // Model standard FontAwesome icons using Mermaid 11.15's inline FA box width instead of
    // the browser's per-icon glyph advance.
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let html = "<p><i class=\"fa fa-car\"></i> Car</p>";
    let m = measure_html_with_flowchart_bold_deltas(
        &measurer,
        html,
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(m.width, 49.03125);
    assert_eq!(m.height, 24.0);
    assert_eq!(m.line_count, 1);
}

#[test]
fn flowchart_html_fontawesome_custom_pack_icon_width_uses_nominal_boundary() {
    // Mermaid 11.15 keeps the inline icon box width even for the documented custom-pack example.
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let html = "<p><i class=\"fab fa-truck-bold\"></i> a custom icon</p>";
    let m = measure_html_with_flowchart_bold_deltas(
        &measurer,
        html,
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(m.width, 124.046875);
    assert_eq!(m.height, 24.0);
    assert_eq!(m.line_count, 1);
}

#[test]
fn fontawesome_icon_substitution_matches_mermaid_source_boundaries() {
    assert_eq!(
        replace_fontawesome_icons("This is an icon: fa:fa-user and fab:fa-github"),
        r#"This is an icon: <i class="fa fa-user"></i> and <i class="fab fa-github"></i>"#
    );
    assert_eq!(
        replace_fontawesome_icons("Icons galore: fa:fa-arrow-right, fak:fa-truck, fas:fa-home"),
        r#"Icons galore: <i class="fa fa-arrow-right"></i>, <i class="fak fa-truck"></i>, <i class="fas fa-home"></i>"#
    );
    assert_eq!(
        replace_fontawesome_icons(
            "Here is a long icon: fak:fa-truck-driving-long-winding-road in use"
        ),
        r#"Here is a long icon: <i class="fak fa-truck-driving-long-winding-road"></i> in use"#
    );
    assert_eq!(
        replace_fontawesome_icons("no icons: faa:fa-user fa:fa- fa:fa-éclair"),
        "no icons: faa:fa-user fa:fa- fa:fa-éclair"
    );
    assert_eq!(
        replace_fontawesome_icons("prefix can match inside text: xfa:fa-user!"),
        r#"prefix can match inside text: x<i class="fa fa-user"></i>!"#
    );
}

#[test]
fn flowchart_label_metrics_for_layout_fontawesome_uses_nominal_boundary() {
    // Non-markdown Flowchart icon labels should use the same HTML fragment measurement path as
    // emitted `<foreignObject>` content, with the same Mermaid 11.15 icon width boundary.
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let m = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "fa:fa-car Car",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(m.width, 49.03125);
    assert_eq!(m.height, 24.0);
    assert_eq!(m.line_count, 1);
}

#[test]
fn flowchart_label_metrics_plain_car_uses_dom_text_width() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let m = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "Car",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(m.width, 24.203125);
    assert_eq!(m.height, 24.0);
    assert_eq!(m.line_count, 1);
}

#[test]
fn flowchart_label_metrics_for_layout_fontawesome_icon_only_lines_match_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let twitter = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "fa:fa-twitter<br/>for peace",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(twitter.width, 68.234375);
    assert_eq!(twitter.height, 48.0);
    assert_eq!(twitter.line_count, 2);

    let camera = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "fa:fa-camera-retro<br/>capture<br/>moments",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(camera.width, 65.421875);
    assert_eq!(camera.height, 72.0);
    assert_eq!(camera.line_count, 3);
}

#[test]
fn flowchart_label_metrics_for_layout_fontawesome_wraps_icon_start_like_upstream() {
    // Mermaid upstream fixture:
    // fixtures/upstream-svgs/flowchart/upstream_cypress_flowchart_handdrawn_spec_fhd7_should_render_a_flowchart_full_of_icons_007.svg
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let database = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: r"fa:fa-database [DBServer\SharedDbInstance]",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(database.width, 208.96875);
    assert_eq!(database.height, 48.0);
    assert_eq!(database.line_count, 2);

    let support_db = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: r"fa:fa-circle [DBServer\SharedDbInstance].[SupportDb]",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(support_db.width, 214.84375);
    assert_eq!(support_db.height, 72.0);
    assert_eq!(support_db.line_count, 3);
}

#[test]
fn courier_html_flowchart_label_width_matches_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let node = measurer.measure_wrapped("Christmas", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(node.width, 86.421875);
    assert_eq!(node.height, 24.0);

    let edge = measurer.measure_wrapped("Get money", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(edge.width, 86.421875);
    assert_eq!(edge.height, 24.0);
}

#[test]
fn default_font_flowchart_html_width_overrides_match_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let edge_a = measurer.measure_wrapped("A to B", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(edge_a.width, 42.1875);
    assert_eq!(edge_a.height, 24.0);

    let edge_b = measurer.measure_wrapped("B to C", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(edge_b.width, 43.203125);
    assert_eq!(edge_b.height, 24.0);

    let node = measurer.measure_wrapped("A: (Edge Text)", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(node.width, 101.046875);
    assert_eq!(node.height, 24.0);

    let cluster = measurer.measure_wrapped("Inner B", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(cluster.width, 50.765625);
    assert_eq!(cluster.height, 24.0);

    let edge = measurer.measure_wrapped(
        "very long edge label",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(edge.width, 145.09375);
    assert_eq!(edge.height, 24.0);

    let post = measurer.measure_wrapped("post", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(post.width, 30.328125);
    assert_eq!(post.height, 24.0);

    let dense_cluster =
        measurer.measure_wrapped("Dense Cluster", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(dense_cluster.width, 98.109375);
    assert_eq!(dense_cluster.height, 24.0);

    let outside2 = measurer.measure_wrapped("outside2", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(outside2.width, 60.75);
    assert_eq!(outside2.height, 24.0);

    for level in ["Level 1", "Level 2", "Level 3", "Level 4"] {
        let metrics = measurer.measure_wrapped(level, &style, Some(200.0), WrapMode::HtmlLike);
        assert_eq!(metrics.width, 51.328125, "{level}");
        assert_eq!(metrics.height, 24.0, "{level}");
    }

    let subgraph_title =
        measurer.measure_wrapped("Subgraph Title", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(subgraph_title.width, 103.171875);
    assert_eq!(subgraph_title.height, 24.0);

    let edge_label =
        measurer.measure_wrapped("Edge Label", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(edge_label.width, 77.9375);
    assert_eq!(edge_label.height, 24.0);

    let node_label_b =
        measurer.measure_wrapped("Node Label B", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(node_label_b.width, 94.0);
    assert_eq!(node_label_b.height, 24.0);

    let custom = measurer.measure_wrapped("custom", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(custom.width, 51.359375);
    assert_eq!(custom.height, 24.0);

    let stacked_rectangle =
        measurer.measure_wrapped("stacked-rectangle", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(stacked_rectangle.width, 128.578125);
    assert_eq!(stacked_rectangle.height, 24.0);
}

#[test]
fn default_font_repeated_glyph_html_runs_match_browser_lattice() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    for (text, expected) in [
        ("sss", 19.4375),
        ("sssssssssssssssssssssss", 148.96875),
        ("tttssssssssssssssssssssss", 161.515625),
        ("tttsssssssssssssssssssssss", 168.0),
        ("tttssssssssssssssssssssssss", 174.46875),
    ] {
        let metrics = measurer.measure_wrapped(text, &style, Some(200.0), WrapMode::HtmlLike);
        assert_eq!(metrics.width, expected, "{text}");
        assert_eq!(metrics.height, 24.0, "{text}");
    }
}

#[test]
fn flowchart_multiline_html_label_uses_widest_browser_line_width() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();

    let metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: "Let me thinksssssx<br/>sssssssssssssssssssuuu<br />tttsssssssssssssssssssssss",
            label_type: "text",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );

    assert_eq!(metrics.width, 168.0);
    assert_eq!(metrics.height, 72.0);
    assert_eq!(metrics.line_count, 3);
}

#[test]
fn default_font_paired_ascii_punctuation_reuses_counterpart_width() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let open_brace = measurer.measure_wrapped("{", &style, None, WrapMode::HtmlLike);
    let close_brace = measurer.measure_wrapped("}", &style, None, WrapMode::HtmlLike);
    assert_eq!(open_brace.width, close_brace.width);
    assert_eq!(open_brace.width, 5.875);

    let bracketed = measurer.measure_wrapped(
        "brackets: [x] {y} (z)",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(bracketed.width, 140.1875);
}

#[test]
fn default_font_missing_v_comma_kern_matches_upstream_jsonish_text() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let jsonish = measurer.measure_wrapped(
        "json: {k: v, n: 1}",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(jsonish.width, 115.09375);
    assert_eq!(jsonish.height, 24.0);
}

#[test]
fn flowchart_html_c1_controls_measure_like_chromium_replacement_glyphs() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let default_style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let courier_style = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let owner_review = "è´\u{9f}è´£äººå®¡æ\u{89}¹";
    let owner_default = measurer.measure_wrapped(
        owner_review,
        &default_style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(owner_default.width, 128.953125);
    assert_eq!(owner_default.height, 24.0);

    let owner_courier = measurer.measure_wrapped(
        owner_review,
        &courier_style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(owner_courier.width, 144.078125);
    assert_eq!(owner_courier.height, 24.0);

    let submit = "æ\u{8f}\u{90}äº¤ç\u{94}³è¯·";
    let submit_default =
        measurer.measure_wrapped(submit, &default_style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(submit_default.width, 104.140625);
    assert_eq!(submit_default.height, 24.0);

    let submit_courier =
        measurer.measure_wrapped(submit, &courier_style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(submit_courier.width, 115.21875);
    assert_eq!(submit_courier.height, 24.0);

    let end = "ç»\u{93}æ\u{9d}\u{9f}";
    let end_default =
        measurer.measure_wrapped(end, &default_style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(end_default.width, 53.4375);
    assert_eq!(end_default.height, 24.0);
}

#[test]
fn flowchart_html_wrapped_measurement_does_not_leak_other_diagram_overrides() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let wrapped = measurer.measure_wrapped("plain", &style, Some(200.0), WrapMode::HtmlLike);
    let unwrapped = measurer.measure_wrapped("plain", &style, None, WrapMode::HtmlLike);
    assert_eq!(wrapped.width, 35.34375);
    assert_eq!(wrapped.height, 24.0);
    assert_eq!(unwrapped.width, 35.34375);
}

#[test]
fn flowchart_html_default_font_tightens_missing_space_before_capital_a_pairs() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let step = measurer.measure_wrapped("Step A", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(step.width, 45.0625);

    let option = measurer.measure_wrapped("Option A", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(option.width, 61.3125);

    let inner = measurer.measure_wrapped("Inner A", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(inner.width, 50.265625);
}

#[test]
fn flowchart_html_default_font_fills_missing_browser_kerning_pairs() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let a_source = measurer.measure_wrapped("A (source)", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(a_source.width, 71.796875);

    let b_source = measurer.measure_wrapped("B (source)", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(b_source.width, 72.3125);

    let c_source = measurer.measure_wrapped("C (source)", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(c_source.width, 72.828125);

    let transform = measurer.measure_wrapped("Transform", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(transform.width, 71.375);

    let top_cluster =
        measurer.measure_wrapped("Top Cluster", &style, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(top_cluster.width, 80.40625);
}

#[test]
fn flowchart_html_default_font_weight_bold_uses_shared_metrics() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let regular = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let bold = TextStyle {
        font_weight: Some("bold".to_string()),
        ..regular.clone()
    };

    let d_regular = measurer.measure_wrapped("D", &regular, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(d_regular.width, 9.8125);
    let d_bold = measurer.measure_wrapped("D", &bold, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(d_bold.width, 10.28125);

    let e_regular = measurer.measure_wrapped("E", &regular, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(e_regular.width, 8.578125);
    let e_bold = measurer.measure_wrapped("E", &bold, Some(200.0), WrapMode::HtmlLike);
    assert_eq!(e_bold.width, 9.109375);
}

#[test]
fn flowchart_svg_cluster_title_precise_width_matches_upstream_wrapped_text() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let width = measure_flowchart_svg_like_precise_width_px(
        &measurer,
        "A very long cluster title with punctuation: (a/b/c)",
        &style,
        Some(200.0),
    );
    assert_eq!(width, 186.90625);
}

#[test]
fn flowchart_html_subgraph_title_punctuation_wraps_at_spaces_like_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let title = "Title: with punctuation (a/b/c) + dashes - and spaces";
    let metrics = measurer.measure_wrapped(title, &style, Some(200.0), WrapMode::HtmlLike);

    assert_eq!(metrics.width, 200.0);
    assert_eq!(metrics.height, 72.0);
    assert_eq!(metrics.line_count, 3);
}

#[test]
fn flowchart_svg_cluster_title_precise_width_matches_upstream_single_line_text() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let width = measure_flowchart_svg_like_precise_width_px(
        &measurer,
        "Subgraph Title",
        &style,
        Some(200.0),
    );
    assert_eq!(width, 103.1875);
}

#[test]
fn flowchart_svg_cluster_title_precise_width_matches_upstream_one() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let width = measure_flowchart_svg_like_precise_width_px(&measurer, "One", &style, Some(200.0));
    assert_eq!(width, 28.25);
}

#[test]
fn flowchart_svg_edge_label_width_matches_upstream_single_line_text() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let metrics = measurer.measure_wrapped("Edge Label", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(metrics.width, 77.9375);
    assert_eq!(metrics.height, 19.0);
}

#[test]
fn flowchart_svg_node_label_width_overrides_match_repeat_offenders() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let node_label = measurer.measure_wrapped("Node Label", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(node_label.width, 80.125);

    let node_label_b =
        measurer.measure_wrapped("Node Label B", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(node_label_b.width, 94.0);

    let cfg = merman_core::MermaidConfig::default();
    let b = flowchart_label_metrics_for_layout(crate::flowchart::FlowchartLabelMetricsRequest {
        measurer: &measurer,
        raw_label: "b",
        label_type: "text",
        style: &style,
        max_width_px: Some(200.0),
        wrap_mode: WrapMode::SvgLike,
        config: &cfg,
        math_renderer: None,
        preserve_string_whitespace_height: false,
        whole_label_font_style: None,
    });
    assert_eq!(b.width, 8.921875);
}

#[test]
fn courier_svg_edge_label_width_matches_upstream() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let metrics = measurer.measure_wrapped("Get money", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(metrics.width, 86.421875);
    assert_eq!(metrics.height, 18.0);
    assert_eq!(metrics.line_count, 1);
}

#[test]
fn courier_html_dotted_identifier_overflows_without_dot_wrapping() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let metrics = measurer.measure_wrapped(
        "SAM.CommonFA.CommonFAFinanceBudget",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(metrics.width, 326.609375);
    assert_eq!(metrics.height, 24.0);
    assert_eq!(metrics.line_count, 1);
}

#[test]
fn default_font_html_hyphenated_compound_wraps_like_browser() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let metrics = measurer.measure_wrapped(
        "This is a label for half-rounded-rectangle shape",
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );

    assert_eq!(metrics.width, 200.0);
    assert_eq!(metrics.height, 48.0);
    assert_eq!(metrics.line_count, 2);
}

#[test]
fn flowchart_svg_edge_label_background_y_matches_upstream_fonts() {
    let trebuchet = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let courier = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let courier_stack = TextStyle {
        font_family: Some("\"Courier New\", courier, monospace;".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    assert_eq!(flowchart_svg_edge_label_background_y_px(&trebuchet), -1.0);
    assert_eq!(flowchart_svg_edge_label_background_y_px(&courier), 0.0);
    assert_eq!(
        flowchart_svg_edge_label_background_y_px(&courier_stack),
        0.0
    );
}

#[test]
fn svg_title_bbox_vertical_extents_use_courier_profile_for_courier_stacks() {
    let trebuchet = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 18.0,
        font_weight: None,
    };
    let courier = TextStyle {
        font_family: Some("courier".to_string()),
        font_size: 18.0,
        font_weight: None,
    };
    let courier_stack = TextStyle {
        font_family: Some("\"Courier New\", courier, monospace;".to_string()),
        font_size: 18.0,
        font_weight: None,
    };

    assert_eq!(
        svg_title_bbox_vertical_extents_px(&courier_stack),
        svg_title_bbox_vertical_extents_px(&courier)
    );
    assert_ne!(
        svg_title_bbox_vertical_extents_px(&courier_stack),
        svg_title_bbox_vertical_extents_px(&trebuchet)
    );
}

#[test]
fn default_font_extra_html_override_table_keeps_special_characters_stable() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let metrics = measurer.measure_wrapped("special characters", &style, None, WrapMode::HtmlLike);
    assert_eq!(metrics.width, 129.9375);
    assert_eq!(metrics.height, 24.0);
}

#[test]
fn html_width_pruned_literals_use_font_metrics_fallback() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let block = measurer.measure_wrapped("Block 1", &style, None, WrapMode::HtmlLike);
    assert_eq!(block.width, 51.5625);

    let flowchart = measurer.measure_wrapped("Circle shape", &style, None, WrapMode::HtmlLike);
    assert_eq!(flowchart.width, 87.8125);
}

#[test]
fn flowchart_svg_width_uses_override_for_pruned_literals() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let end = measurer.measure_wrapped("End", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(end.width, 26.234375);

    let edge_label = measurer.measure_wrapped("edge label", &style, Some(200.0), WrapMode::SvgLike);
    assert_eq!(edge_label.width, 74.71875);
}

#[test]
fn flowchart_title_bbox_uses_symmetric_shared_advance() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 18.0,
        font_weight: None,
    };

    let (left, right) = measurer.measure_svg_title_bbox_x("Simple flowchart", &style);
    assert_eq!(left, 68.3359375);
    assert_eq!(right, 68.3359375);
    assert_eq!(
        measurer.measure_svg_simple_text_bbox_width_px("Simple flowchart", &style),
        137.5244140625
    );
}

#[test]
fn sequence_svg_overrides_keep_literal_br_with_backslash_t_single_line() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif;".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    // Mermaid `lineBreakRegex` should not treat this as a `<br>` break because `\\t` is a
    // literal backslash + `t`, not whitespace.
    let text = "multiline<br \\t/>text";
    let m = measurer.measure_wrapped(text, &style, None, WrapMode::SvgLikeSingleRun);
    assert_eq!(m.line_count, 1);
    assert_eq!(m.width, 132.0);
}

#[test]
fn sequence_svg_overrides_measure_final_simple_bbox_widths() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif;".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let prefix = "This is a longer message that should be wrapped by Mermaid's default behavior";
    assert_eq!(
        measurer.measure_svg_simple_text_bbox_width_px(prefix, &style),
        511.0
    );

    let no_wrap = "This message should not wrap even if it is long long long long long";
    assert_eq!(
        measurer.measure_svg_simple_text_bbox_width_px(no_wrap, &style),
        432.0
    );

    assert_eq!(
        measurer.measure_svg_simple_text_bbox_width_px("very-long-participant-label", &style),
        174.0
    );
    assert_eq!(
        measurer.measure_svg_simple_text_bbox_width_px("another-long-participant-label", &style),
        192.0
    );
}

#[test]
fn sequence_wrap_uses_exact_single_line_evidence_only_when_it_fits() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif;".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let text = "This is a longer message that should be wrapped by Mermaid's default behavior";

    let exact = measurer.measure_svg_simple_text_bbox_width_px(text, &style);
    let probe = measurer.measure_svg_simple_text_bbox_width_for_wrap_px(text, &style);
    assert_eq!(exact, 511.0);
    assert!(exact + 4.0 < probe);

    let single = wrap_label_like_mermaid_lines_floored_bbox(text, &measurer, &style, exact + 31.0);
    assert_eq!(single, vec![text.to_string()]);

    let wrapped = wrap_label_like_mermaid_lines_floored_bbox(text, &measurer, &style, 200.0);
    assert!(
        wrapped.len() > 1,
        "narrow labels should still use the normal Mermaid wrapLabel flow"
    );
}

#[test]
fn wrap_label_like_mermaid_does_not_split_escaped_br() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif;".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let lines =
        wrap_label_like_mermaid_lines("multiline<br>using #lt;br#gt;", &measurer, &style, 10_000.0);
    assert_eq!(
        lines,
        vec!["multiline".to_string(), "using #lt;br#gt;".to_string()],
        "wrapLabel should short-circuit when explicit `<br>` breaks are present, and must not treat escaped `#lt;br#gt;` as a break"
    );
}

#[test]
fn flowchart_label_metrics_for_layout_measures_markdown_inline_html_like_mermaid() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();
    let markdown = "This is **bold** </br>and <strong>strong</strong>";
    assert!(mermaid_markdown_contains_html_tags(markdown));

    let html = mermaid_markdown_to_html_label_fragment(markdown, true);
    let html_metrics = measure_html_with_flowchart_bold_deltas(
        &measurer,
        &html,
        &style,
        Some(200.0),
        WrapMode::HtmlLike,
    );
    assert_eq!(html_metrics.width, 82.125);
    assert_eq!(html_metrics.height, 48.0);
    assert_eq!(html_metrics.line_count, 2);

    let metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: markdown,
            label_type: "markdown",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );
    assert_eq!(metrics.width, 82.125);
    assert_eq!(metrics.height, 48.0);
    assert_eq!(metrics.line_count, 2);
}

#[test]
fn flowchart_html_markdown_inline_bold_delta_can_force_extra_wrap_line() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();
    let markdown = "This is **bold** </br>and <strong>strong</strong> for braces shape";

    let metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: markdown,
            label_type: "markdown",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );

    assert_eq!(metrics.width, 200.0);
    assert_eq!(metrics.height, 72.0);
    assert_eq!(metrics.line_count, 3);
}

#[test]
fn flowchart_html_markdown_metrics_preserve_paragraph_break_height() {
    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };
    let cfg = merman_core::MermaidConfig::default();
    let markdown = "The dog in **the** hog.(1).. a a a a *very long text* about it\nWord!\n\nAnother line with many, many words. Another line with many, many words. Another line with many, many words. Another line with many, many words. Another line with many, many words. Another line with many, many words. Another line with many, many words. Another line with many, many words. ";

    let metrics = crate::flowchart::flowchart_label_metrics_for_layout(
        crate::flowchart::FlowchartLabelMetricsRequest {
            measurer: &measurer,
            raw_label: markdown,
            label_type: "markdown",
            style: &style,
            max_width_px: Some(200.0),
            wrap_mode: WrapMode::HtmlLike,
            config: &cfg,
            math_renderer: None,
            preserve_string_whitespace_height: false,
            whole_label_font_style: None,
        },
    );

    assert_eq!(metrics.width, 200.0);
    assert_eq!(metrics.height, 384.0);
    assert_eq!(metrics.line_count, 16);
}

#[test]
fn markdown_svg_wrapping_keeps_raw_html_tags_literal_but_wraps_like_mermaid() {
    use MermaidMarkdownWordType::*;

    let measurer = VendoredFontMetricsTextMeasurer::default();
    let style = TextStyle {
        font_family: Some("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
        font_size: 16.0,
        font_weight: None,
    };

    let lines = mermaid_markdown_to_wrapped_word_lines(
        &measurer,
        "This is **bold** </br>and <strong>strong</strong>",
        &style,
        Some(200.0),
        WrapMode::SvgLike,
    );
    assert_eq!(
        lines,
        vec![
            vec![
                ("This".to_string(), Normal),
                ("is".to_string(), Normal),
                ("bold".to_string(), Strong),
            ],
            vec![
                ("and".to_string(), Normal),
                ("<strong>".to_string(), Normal),
                ("strong".to_string(), Normal),
            ],
            vec![("</strong>".to_string(), Normal)],
        ]
    );
}
