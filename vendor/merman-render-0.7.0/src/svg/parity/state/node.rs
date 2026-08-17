use super::*;
use crate::generated::state_text_overrides_11_12_2 as state_text_overrides;

pub(super) fn render_state_node_svg(
    out: &mut String,
    ctx: &StateRenderCtx<'_>,
    node_id: &str,
    origin_x: f64,
    origin_y: f64,
    timing_enabled: bool,
    details: &mut StateRenderDetails,
) {
    let Some(node) = ctx.nodes_by_id.get(node_id).copied() else {
        return;
    };
    let Some(ln) = ctx.layout_nodes_by_id.get(node_id).copied() else {
        return;
    };
    if ln.is_cluster {
        return;
    }
    let cx = ln.x - origin_x;
    let cy = ln.y - origin_y;
    let w = ln.width.max(1.0);
    let h = ln.height.max(1.0);

    #[inline]
    fn cached_circle(
        ctx: &StateRenderCtx<'_>,
        key: StateRoughCacheKey,
        build: impl FnOnce() -> String,
    ) -> Arc<String> {
        let existing = { ctx.rough_circle_cache.borrow().get(&key).cloned() };
        if let Some(v) = existing {
            return v;
        }

        if let Some(v) = state_tls_get_circle(key) {
            ctx.rough_circle_cache
                .borrow_mut()
                .insert(key, Arc::clone(&v));
            return v;
        }

        if let Ok(global) = state_global_rough_circle_cache().lock() {
            if let Some(v) = global.get(&key) {
                let v = Arc::clone(v);
                state_tls_put_circle(key, Arc::clone(&v));
                ctx.rough_circle_cache
                    .borrow_mut()
                    .insert(key, Arc::clone(&v));
                return v;
            }
        }

        let built = Arc::new(build());
        let cached = if let Ok(mut global) = state_global_rough_circle_cache().lock() {
            Arc::clone(global.entry(key).or_insert_with(|| Arc::clone(&built)))
        } else {
            Arc::clone(&built)
        };
        state_tls_put_circle(key, Arc::clone(&cached));
        ctx.rough_circle_cache
            .borrow_mut()
            .insert(key, Arc::clone(&cached));
        cached
    }

    #[inline]
    fn cached_paths(
        ctx: &StateRenderCtx<'_>,
        key: StateRoughCacheKey,
        build: impl FnOnce() -> (String, String),
    ) -> (Arc<String>, Arc<String>) {
        let existing = { ctx.rough_paths_cache.borrow().get(&key).cloned() };
        if let Some(v) = existing {
            return v;
        }

        if let Some(v) = state_tls_get_paths(key) {
            ctx.rough_paths_cache
                .borrow_mut()
                .insert(key, (Arc::clone(&v.0), Arc::clone(&v.1)));
            return v;
        }

        if let Ok(global) = state_global_rough_paths_cache().lock() {
            if let Some((fill_d, stroke_d)) = global.get(&key) {
                let v = (Arc::clone(fill_d), Arc::clone(stroke_d));
                state_tls_put_paths(key, (Arc::clone(&v.0), Arc::clone(&v.1)));
                ctx.rough_paths_cache
                    .borrow_mut()
                    .insert(key, (Arc::clone(&v.0), Arc::clone(&v.1)));
                return v;
            }
        }

        let (fill_d, stroke_d) = build();
        let built = (Arc::new(fill_d), Arc::new(stroke_d));
        let cached = if let Ok(mut global) = state_global_rough_paths_cache().lock() {
            let entry = global
                .entry(key)
                .or_insert_with(|| (Arc::clone(&built.0), Arc::clone(&built.1)));
            (Arc::clone(&entry.0), Arc::clone(&entry.1))
        } else {
            (Arc::clone(&built.0), Arc::clone(&built.1))
        };
        state_tls_put_paths(key, (Arc::clone(&cached.0), Arc::clone(&cached.1)));
        ctx.rough_paths_cache
            .borrow_mut()
            .insert(key, (Arc::clone(&cached.0), Arc::clone(&cached.1)));
        cached
    }

    let node_class = if node.css_classes.trim().is_empty() {
        "node".to_string()
    } else {
        format!("node {}", node.css_classes.trim())
    };

    let style_parse_start = timing_enabled.then(web_time::Instant::now);
    let mut shape_decls: Vec<StateInlineDecl<'_>> = Vec::new();
    let mut text_decls: Vec<StateInlineDecl<'_>> = Vec::new();
    let mut fill_override: Option<&str> = None;
    let mut stroke_override: Option<&str> = None;
    let mut stroke_width_override: Option<f64> = None;
    for raw in node
        .css_compiled_styles
        .iter()
        .chain(node.css_styles.iter())
    {
        let Some(d) = state_parse_inline_decl(raw) else {
            continue;
        };
        if d.key.trim().eq_ignore_ascii_case("fill") {
            fill_override = Some(d.val.trim());
        }
        if d.key.trim().eq_ignore_ascii_case("stroke") {
            stroke_override = Some(d.val.trim());
        }
        if d.key.trim().eq_ignore_ascii_case("stroke-width") {
            let val = d.val.trim().trim_end_matches("px").trim();
            if let Ok(v) = val.parse::<f64>() {
                stroke_width_override = Some(v);
            }
        }
        if state_is_text_style_key(d.key) {
            text_decls.push(d);
        } else {
            shape_decls.push(d);
        }
    }
    let shape_style_attr = state_compact_style_attr(&shape_decls);
    let text_style_attr = state_compact_style_attr(&text_decls);
    let div_style_prefix = state_div_style_prefix(&text_decls);
    if let Some(s) = style_parse_start {
        details.leaf_nodes_style_parse += s.elapsed();
    }

    match node.shape.as_str() {
        "stateStart" => {
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r#"<g class="node default" id="{}" transform="translate({}, {})"><circle class="state-start" r="7" width="14" height="14"/></g>"#,
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy)
            );
            drop(_g_emit);
        }
        "stateEnd" => {
            let rough_start = timing_enabled.then(web_time::Instant::now);
            if timing_enabled {
                details.leaf_roughjs_calls += 2;
                details.leaf_roughjs_unique.insert(StateRoughCacheKey {
                    tag: 1,
                    a: 14.0f64.to_bits(),
                    b: 0,
                    seed: ctx.hand_drawn_seed,
                });
                details.leaf_roughjs_unique.insert(StateRoughCacheKey {
                    tag: 2,
                    a: 5.0f64.to_bits(),
                    b: 0,
                    seed: ctx.hand_drawn_seed,
                });
            }
            let outer_key = StateRoughCacheKey {
                tag: 1,
                a: 14.0f64.to_bits(),
                b: 0,
                seed: ctx.hand_drawn_seed,
            };
            let inner_key = StateRoughCacheKey {
                tag: 2,
                a: 5.0f64.to_bits(),
                b: 0,
                seed: ctx.hand_drawn_seed,
            };

            let outer_d = cached_circle(ctx, outer_key, || {
                roughjs_circle_path_d(14.0, ctx.hand_drawn_seed)
                    .unwrap_or_else(|| "M0,0".to_string())
            });
            let inner_d = cached_circle(ctx, inner_key, || {
                roughjs_circle_path_d(5.0, ctx.hand_drawn_seed)
                    .unwrap_or_else(|| "M0,0".to_string())
            });
            if let Some(s) = rough_start {
                details.leaf_nodes_roughjs += s.elapsed();
            }
            let shape_style_escaped = escape_attr(&shape_style_attr);
            let outer_fill = fill_override.unwrap_or(ctx.theme_defaults.end_outer_fill.as_str());
            let outer_stroke = ctx.theme_defaults.end_outer_stroke.as_str();
            let inner_fill = ctx.theme_defaults.inner_end_background.as_str();
            let inner_stroke = ctx.theme_defaults.end_inner_stroke.as_str();
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r##"<g class="node default" id="{}" transform="translate({}, {})"><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="2" fill="none" stroke-dasharray="0 0" style="{}"/><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style=""/><path d="{}" stroke="{}" stroke-width="2" fill="none" stroke-dasharray="0 0" style=""/></g></g></g>"##,
                escape_attr(&node.dom_id),
                fmt(cx),
                fmt(cy),
                outer_d.as_str(),
                escape_attr(outer_fill),
                shape_style_escaped,
                outer_d.as_str(),
                escape_attr(outer_stroke),
                shape_style_escaped,
                inner_d.as_str(),
                escape_attr(inner_fill),
                inner_d.as_str(),
                escape_attr(inner_stroke),
            );
            drop(_g_emit);
        }
        "fork" | "join" => {
            let rough_start = timing_enabled.then(web_time::Instant::now);
            let key = StateRoughCacheKey {
                tag: 3,
                a: w.to_bits(),
                b: h.to_bits(),
                seed: ctx.hand_drawn_seed,
            };
            if timing_enabled {
                details.leaf_roughjs_calls += 1;
                details.leaf_roughjs_unique.insert(key);
            }
            let (fill_d, stroke_d) = cached_paths(ctx, key, || {
                roughjs_paths_for_rect(StateRoughRectSpec {
                    x: -w / 2.0,
                    y: -h / 2.0,
                    w,
                    h,
                    fill: "#333333",
                    stroke: "#333333",
                    stroke_width: 1.3,
                    seed: ctx.hand_drawn_seed,
                })
                .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()))
            });
            if let Some(s) = rough_start {
                details.leaf_nodes_roughjs += s.elapsed();
            }
            let fill_attr =
                fill_override.unwrap_or(ctx.theme_defaults.special_state_color.as_str());
            let stroke_attr =
                stroke_override.unwrap_or(ctx.theme_defaults.special_state_color.as_str());
            let stroke_width_attr = stroke_width_override.unwrap_or(1.3).max(0.0);
            let shape_style_escaped = escape_attr(&shape_style_attr);
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r##"<g class="{}" id="{}" transform="translate({}, {})"><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0" style="{}"/></g></g>"##,
                escape_xml_display(&node_class),
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy),
                fill_d.as_str(),
                escape_xml_display(fill_attr),
                shape_style_escaped,
                stroke_d.as_str(),
                escape_xml_display(stroke_attr),
                fmt_display(stroke_width_attr),
                shape_style_escaped
            );
            drop(_g_emit);
        }
        "choice" => {
            let rough_start = timing_enabled.then(web_time::Instant::now);
            let key = StateRoughCacheKey {
                tag: 4,
                a: w.to_bits(),
                b: h.to_bits(),
                seed: ctx.hand_drawn_seed,
            };
            if timing_enabled {
                details.leaf_roughjs_calls += 1;
                details.leaf_roughjs_unique.insert(key);
            }
            let (fill_d, stroke_d) = cached_paths(ctx, key, || {
                roughjs_paths_for_svg_path(
                    &mermaid_choice_diamond_path_data(w, h),
                    "#ECECFF",
                    "#9370DB",
                    1.3,
                    "0 0",
                    ctx.hand_drawn_seed,
                )
                .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()))
            });
            if let Some(s) = rough_start {
                details.leaf_nodes_roughjs += s.elapsed();
            }

            let fill_attr = fill_override.unwrap_or(ctx.theme_defaults.main_bkg.as_str());
            let stroke_attr = stroke_override.unwrap_or(ctx.theme_defaults.state_border.as_str());
            let stroke_width_attr = stroke_width_override
                .unwrap_or(ctx.theme_defaults.rough_stroke_width_value)
                .max(0.0);
            let shape_style_escaped = escape_attr(&shape_style_attr);
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r##"<g class="{}" id="{}" transform="translate({}, {})"><g><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0" style="{}"/></g></g>"##,
                escape_xml_display(&node_class),
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy),
                fill_d.as_str(),
                escape_xml_display(fill_attr),
                shape_style_escaped,
                stroke_d.as_str(),
                escape_xml_display(stroke_attr),
                fmt_display(stroke_width_attr),
                shape_style_escaped
            );
            drop(_g_emit);
        }
        "note" => {
            let label = state_node_label_text(node);
            let measure_start = timing_enabled.then(web_time::Instant::now);
            let mut metrics = ctx.measurer.measure_wrapped(
                &label,
                &ctx.text_style,
                Some(ctx.html_label_wrapping_width),
                WrapMode::HtmlLike,
            );
            if let Some(s) = measure_start {
                details.leaf_nodes_measure += s.elapsed();
            }
            if let Some(w) = state_text_overrides::lookup_state_note_label_width_px(
                ctx.text_style.font_size,
                label.trim(),
            ) {
                metrics.width = w;
            }
            let lw = metrics.width.max(0.0);
            let lh = metrics.height.max(0.0);
            let rough_start = timing_enabled.then(web_time::Instant::now);
            let key = StateRoughCacheKey {
                tag: 5,
                a: w.to_bits(),
                b: h.to_bits(),
                seed: ctx.hand_drawn_seed,
            };
            if timing_enabled {
                details.leaf_roughjs_calls += 1;
                details.leaf_roughjs_unique.insert(key);
            }
            let (fill_d, stroke_d) = cached_paths(ctx, key, || {
                roughjs_paths_for_rect(StateRoughRectSpec {
                    x: -w / 2.0,
                    y: -h / 2.0,
                    w,
                    h,
                    fill: "#fff5ad",
                    stroke: "#aaaa33",
                    stroke_width: 1.3,
                    seed: ctx.hand_drawn_seed,
                })
                .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()))
            });
            if let Some(s) = rough_start {
                details.leaf_nodes_roughjs += s.elapsed();
            }
            let label_html_start = timing_enabled.then(web_time::Instant::now);
            let label_html = state_node_label_html(&label);
            if let Some(s) = label_html_start {
                details.leaf_nodes_label_html += s.elapsed();
            }
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r##"<g class="{}" id="{}" transform="translate({}, {})"><g class="basic label-container"><path d="{}" stroke="none" stroke-width="0" fill="{}"/><path d="{}" stroke="{}" stroke-width="1.3" fill="none" stroke-dasharray="0 0"/></g><g class="label" style="" transform="translate({}, {})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;">{}</div></foreignObject></g></g>"##,
                escape_xml_display(&node_class),
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy),
                fill_d.as_str(),
                escape_xml_display(ctx.theme_defaults.note_bkg.as_str()),
                stroke_d.as_str(),
                escape_xml_display(ctx.theme_defaults.note_border.as_str()),
                fmt_display(-lw / 2.0),
                fmt_display(-lh / 2.0),
                fmt_display(lw),
                fmt_display(lh),
                fmt_display(ctx.html_label_wrapping_width),
                label_html
            );
            drop(_g_emit);
        }
        "rectWithTitle" => {
            let title = node
                .label
                .as_ref()
                .map(state_value_to_label_text)
                .unwrap_or_else(|| node.id.clone());
            let desc = node
                .description
                .as_ref()
                .map(|v| v.join("\n"))
                .unwrap_or_default();
            // Mermaid renders `rectWithTitle` labels as HTML `<span>` (nowrap) with
            // `padding-right: 1px` and no explicit `line-height`, so their measured height matches
            // SVG `getBBox()` (19px at 16px font size) rather than the 1.5em HTML `<p>` height.
            let measure_start = timing_enabled.then(web_time::Instant::now);
            let title_metrics =
                ctx.measurer
                    .measure_wrapped(&title, &ctx.text_style, None, WrapMode::SvgLike);
            let desc_metrics =
                ctx.measurer
                    .measure_wrapped(&desc, &ctx.text_style, None, WrapMode::SvgLike);
            if let Some(s) = measure_start {
                details.leaf_nodes_measure += s.elapsed();
            }

            let padding = ctx.state_padding;
            let half_pad = (padding / 2.0).max(0.0);
            let top_pad = state_text_overrides::state_rect_with_title_top_pad_px(padding);
            let gap = state_text_overrides::state_rect_with_title_gap_px(padding);

            // Mirror `padding-right: 1px` in upstream HTML.
            let title_w = state_text_overrides::rect_with_title_span_effective_width_px(
                ctx.text_style.font_size,
                title.trim(),
                title_metrics.width,
            );
            let title_h = state_text_overrides::rect_with_title_span_effective_height_px(
                ctx.text_style.font_size,
                title.trim(),
                title_metrics.height,
            );
            let desc_w = state_text_overrides::rect_with_title_span_effective_width_px(
                ctx.text_style.font_size,
                desc.trim(),
                desc_metrics.width,
            );
            let desc_h = state_text_overrides::rect_with_title_span_effective_height_px(
                ctx.text_style.font_size,
                desc.trim(),
                desc_metrics.height,
            );
            let inner_w = (w - padding).max(0.0);
            let title_x = ((inner_w - title_w) / 2.0).max(0.0);
            let desc_x = ((inner_w - desc_w) / 2.0).max(0.0);
            let desc_y = title_h + gap;
            let divider_y = -h / 2.0 + top_pad + title_h + 1.0;
            let label_html_start = timing_enabled.then(web_time::Instant::now);
            let title_html = state_node_label_inline_html(&title);
            let desc_html = state_node_label_inline_html(&desc);
            if let Some(s) = label_html_start {
                details.leaf_nodes_label_html += s.elapsed();
            }
            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r#"<g class="{}" id="{}" transform="translate({}, {})"><g><rect class="outer title-state" style="" x="{}" y="{}" width="{}" height="{}"/><line class="divider" x1="{}" x2="{}" y1="{}" y2="{}"/></g><g class="label" style="" transform="translate({}, {})"><foreignObject width="{}" height="{}" transform="translate( {}, 0)"><div xmlns="http://www.w3.org/1999/xhtml" style="display: inline-block; padding-right: {}px; white-space: nowrap;">{}</div></foreignObject><foreignObject width="{}" height="{}" transform="translate( {}, {})"><div xmlns="http://www.w3.org/1999/xhtml" style="display: inline-block; padding-right: {}px; white-space: nowrap;">{}</div></foreignObject></g></g>"#,
                escape_xml_display(&node_class),
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy),
                fmt_display(-w / 2.0),
                fmt_display(-h / 2.0),
                fmt_display(w),
                fmt_display(h),
                fmt_display(-w / 2.0),
                fmt_display(w / 2.0),
                fmt_display(divider_y),
                fmt_display(divider_y),
                fmt_display(-w / 2.0 + half_pad),
                fmt_display(-h / 2.0 + top_pad),
                fmt_display(title_w),
                fmt_display(title_h),
                fmt_display(title_x),
                fmt_display(state_text_overrides::state_rect_with_title_span_padding_right_px()),
                title_html,
                fmt_display(desc_w),
                fmt_display(desc_h),
                fmt_display(desc_x),
                fmt_display(desc_y),
                fmt_display(state_text_overrides::state_rect_with_title_span_padding_right_px()),
                desc_html
            );
            drop(_g_emit);
        }
        _ => {
            let label = state_node_label_text(node);

            fn parse_css_px_f64(v: &str) -> Option<f64> {
                let t = v.trim();
                let t = t.trim_end_matches(';').trim();
                let t = t.trim_end_matches("!important").trim();
                let t = t.trim_end_matches("px").trim();
                t.parse::<f64>().ok()
            }

            let mut measure_style = ctx.text_style.clone();
            let mut has_metrics_style: bool = false;
            let mut italic: bool = false;

            for d in &text_decls {
                let k = d.key.trim().to_ascii_lowercase();
                let v = d.val.trim().trim_end_matches(';').trim();
                let v_no_imp = v.trim_end_matches("!important").trim();
                match k.as_str() {
                    "font-weight" => {
                        if !v_no_imp.is_empty() {
                            measure_style.font_weight = Some(v_no_imp.to_string());
                            has_metrics_style = true;
                        }
                    }
                    "font-style" => {
                        let lower = v_no_imp.to_ascii_lowercase();
                        if lower.contains("italic") || lower.contains("oblique") {
                            italic = true;
                            has_metrics_style = true;
                        }
                    }
                    "font-size" => {
                        if let Some(px) = parse_css_px_f64(v_no_imp) {
                            if px.is_finite() && px > 0.0 {
                                measure_style.font_size = px;
                                has_metrics_style = true;
                            }
                        }
                    }
                    "font-family" => {
                        if !v_no_imp.is_empty() {
                            measure_style.font_family = Some(v_no_imp.to_string());
                            has_metrics_style = true;
                        }
                    }
                    _ => {}
                }
            }

            let measure_start = timing_enabled.then(web_time::Instant::now);
            let mut metrics = ctx.measurer.measure_wrapped(
                &label,
                &measure_style,
                Some(ctx.html_label_wrapping_width),
                WrapMode::HtmlLike,
            );
            if let Some(s) = measure_start {
                details.leaf_nodes_measure += s.elapsed();
            }

            if italic {
                metrics.width +=
                    crate::text::mermaid_default_italic_width_delta_px(&label, &measure_style);
            }
            metrics.width +=
                crate::text::mermaid_default_bold_width_delta_px(&label, &measure_style);

            if metrics.width.is_finite() {
                metrics.width = metrics.width.min(ctx.html_label_wrapping_width);
            }
            metrics.width = crate::text::round_to_1_64_px(metrics.width);
            if metrics.width.is_finite() {
                metrics.width = metrics.width.min(ctx.html_label_wrapping_width);
            }

            if !has_metrics_style {
                if let Some(w) =
                    crate::generated::state_text_overrides_11_12_2::lookup_state_node_label_width_px(
                        measure_style.font_size,
                        label.trim(),
                    )
                {
                    metrics.width = w;
                }
            }

            let bold = measure_style
                .font_weight
                .as_deref()
                .is_some_and(|s| s.to_ascii_lowercase().contains("bold"));
            if let Some(w) =
                crate::generated::state_text_overrides_11_12_2::lookup_state_node_label_width_px_styled(
                    measure_style.font_size,
                    label.trim(),
                    bold,
                    italic,
                )
            {
                metrics.width = w;
            }

            let has_classdef_border_style = node
                .css_compiled_styles
                .iter()
                .any(|s| s.trim_start().to_ascii_lowercase().starts_with("border:"));

            // Mermaid@11.12.2 browser baselines show a surprising `getBoundingClientRect()` inflation
            // for `classDef`-styled border nodes: even a single-line `<p>` label can measure as `72px`
            // tall. Mirror that behavior here to avoid relying on string-keyed height overrides.
            if has_classdef_border_style && (measure_style.font_size - 16.0).abs() <= 0.01 {
                let trimmed = label.trim();
                let is_single_line = !trimmed.contains('\n')
                    && !trimmed.to_ascii_lowercase().contains("<br")
                    && !trimmed.is_empty();
                if is_single_line && (metrics.height - 24.0).abs() <= 0.01 {
                    metrics.height = metrics.height.max(72.0);
                }
            }
            let lw = metrics.width.max(0.0);
            let lh = metrics.height.max(0.0);

            let mut link_open = String::new();
            let mut link_close = String::new();
            if let Some(links) = ctx.links.get(node_id) {
                let mut push_link = |link: &StateSvgLink| {
                    let url = link.url.trim();
                    let tooltip = link.tooltip.trim();
                    let title_attr = if tooltip.is_empty() {
                        String::new()
                    } else {
                        format!(r#" title="{}""#, escape_attr(tooltip))
                    };

                    if !url.is_empty() && (ctx.security_level_loose || state_link_href_allowed(url))
                    {
                        link_open.push_str(&format!(
                            r#"<a xlink:href="{}"{}>"#,
                            escape_attr(url),
                            title_attr
                        ));
                        link_close.push_str("</a>");
                        return;
                    }

                    link_open.push_str(&format!(r#"<a{}>"#, title_attr));
                    link_close.push_str("</a>");
                };

                match links {
                    StateSvgLinks::One(link) => push_link(link),
                    StateSvgLinks::Many(list) => {
                        for link in list {
                            push_link(link);
                        }
                    }
                }
            }

            let fill_attr = fill_override.unwrap_or(ctx.theme_defaults.state_bkg.as_str());
            let stroke_attr = stroke_override.unwrap_or(ctx.theme_defaults.state_border.as_str());
            let stroke_width_attr = stroke_width_override
                .unwrap_or(ctx.theme_defaults.rough_stroke_width_value)
                .max(0.0);

            let rough_start = timing_enabled.then(web_time::Instant::now);
            let key = StateRoughCacheKey {
                tag: 6,
                a: w.to_bits(),
                b: h.to_bits(),
                seed: ctx.hand_drawn_seed,
            };
            if timing_enabled {
                details.leaf_roughjs_calls += 1;
                details.leaf_roughjs_unique.insert(key);
            }
            let (fill_d, stroke_d) = cached_paths(ctx, key, || {
                roughjs_paths_for_svg_path(
                    &mermaid_rounded_rect_path_data(w, h),
                    "#ECECFF",
                    "#9370DB",
                    1.3,
                    "0 0",
                    ctx.hand_drawn_seed,
                )
                .unwrap_or_else(|| ("M0,0".to_string(), "M0,0".to_string()))
            });
            if let Some(s) = rough_start {
                details.leaf_nodes_roughjs += s.elapsed();
            }

            let label_span_style = if text_style_attr.is_empty() {
                None
            } else {
                Some(text_style_attr.as_str())
            };
            let label_html_start = timing_enabled.then(web_time::Instant::now);
            let label_html = state_node_label_html_with_style(&label, label_span_style);
            if let Some(s) = label_html_start {
                details.leaf_nodes_label_html += s.elapsed();
            }

            let div_style = if metrics.line_count > 1 {
                format!(
                    r#"{}display: table; white-space: break-spaces; line-height: 1.5; max-width: {}px; text-align: center; width: {}px;"#,
                    div_style_prefix,
                    fmt(ctx.html_label_wrapping_width),
                    fmt(lw),
                )
            } else {
                format!(
                    r#"{}display: table-cell; white-space: nowrap; line-height: 1.5; max-width: {}px; text-align: center;"#,
                    div_style_prefix,
                    fmt(ctx.html_label_wrapping_width)
                )
            };

            let _g_emit = detail_guard(timing_enabled, &mut details.leaf_nodes_emit);
            let _ = write!(
                out,
                r##"<g class="{}" id="{}" transform="translate({}, {})"><g class="basic label-container outer-path"><path d="{}" stroke="none" stroke-width="0" fill="{}" style="{}"/><path d="{}" stroke="{}" stroke-width="{}" fill="none" stroke-dasharray="0 0" style="{}"/></g>{}<g class="label" style="{}" transform="translate({}, {})"><rect/><foreignObject width="{}" height="{}"><div xmlns="http://www.w3.org/1999/xhtml" style="{}">{}</div></foreignObject></g>{}</g>"##,
                escape_xml_display(&node_class),
                escape_xml_display(&node.dom_id),
                fmt_display(cx),
                fmt_display(cy),
                fill_d.as_str(),
                escape_xml_display(fill_attr),
                escape_xml_display(&shape_style_attr),
                stroke_d.as_str(),
                escape_xml_display(stroke_attr),
                fmt_display(stroke_width_attr),
                escape_xml_display(&shape_style_attr),
                link_open,
                escape_xml_display(&text_style_attr),
                fmt_display(-lw / 2.0),
                fmt_display(-lh / 2.0),
                fmt_display(lw),
                fmt_display(lh),
                div_style,
                label_html,
                link_close
            );
            drop(_g_emit);
        }
    }
}
