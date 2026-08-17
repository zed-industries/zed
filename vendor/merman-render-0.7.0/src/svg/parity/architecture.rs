use super::*;
use crate::architecture_metrics::{
    ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX, architecture_estimate_service_bounds,
    architecture_top_level_service_root_bounds,
};

mod edges;
mod foreign_object;
mod geometry;
mod icons;
mod labels;
mod model;
mod nodes;
mod root;
mod settings;
mod viewport;

use self::edges::{ArchitectureEdgeRenderContext, push_architecture_edges};
use self::geometry::{GroupRect, GroupRectComputer, bounds_from_rect, extend_bounds};
use self::labels::{svg_line_plain_text, wrap_svg_words_to_lines};
use self::model::{ArchitectureModel, ArchitectureModelAccess};
use self::nodes::{
    ArchitectureNodeRenderContext, push_architecture_groups,
    push_architecture_services_and_junctions,
};
use self::root::{
    ArchitectureRootOpenContext, architecture_a11y_nodes, push_architecture_root_open,
};
use self::settings::ArchitectureRenderSettings;
use self::viewport::{ArchitectureRootViewportContext, finalize_architecture_root_viewport};

// Architecture diagram SVG renderer implementation (split from parity.rs).

fn timing_section<'a>(
    enabled: bool,
    dst: &'a mut web_time::Duration,
) -> Option<super::timing::TimingGuard<'a>> {
    enabled.then(|| super::timing::TimingGuard::new(dst))
}

struct ArchitectureRenderRequest<'a, M: ArchitectureModelAccess> {
    layout: &'a ArchitectureDiagramLayout,
    model: &'a M,
    effective_config: &'a serde_json::Value,
    sanitize_config_opt: Option<&'a merman_core::MermaidConfig>,
    options: &'a SvgRenderOptions,
}

struct ArchitectureTimingState<'a> {
    enabled: bool,
    timings: &'a mut super::timing::RenderTimings,
    total_start: web_time::Instant,
}

pub(super) fn render_architecture_diagram_svg_typed_with_config(
    layout: &ArchitectureDiagramLayout,
    model: &merman_core::diagrams::architecture::ArchitectureDiagramRenderModel,
    effective_config: &merman_core::MermaidConfig,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::timing::render_timing_enabled();
    let mut timings = super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();

    render_architecture_diagram_svg_with_model(
        ArchitectureRenderRequest {
            layout,
            model,
            effective_config: effective_config.as_value(),
            sanitize_config_opt: Some(effective_config),
            options,
        },
        ArchitectureTimingState {
            enabled: timing_enabled,
            timings: &mut timings,
            total_start,
        },
    )
}

pub(super) fn render_architecture_diagram_svg(
    layout: &ArchitectureDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::timing::render_timing_enabled();
    let mut timings = super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();
    let model: ArchitectureModel = {
        let _g = timing_section(timing_enabled, &mut timings.deserialize_model);
        crate::json::from_value_ref(semantic)?
    };
    render_architecture_diagram_svg_with_model(
        ArchitectureRenderRequest {
            layout,
            model: &model,
            effective_config,
            sanitize_config_opt: None,
            options,
        },
        ArchitectureTimingState {
            enabled: timing_enabled,
            timings: &mut timings,
            total_start,
        },
    )
}

pub(super) fn render_architecture_diagram_svg_with_config(
    layout: &ArchitectureDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    options: &SvgRenderOptions,
) -> Result<String> {
    let timing_enabled = super::timing::render_timing_enabled();
    let mut timings = super::timing::RenderTimings::default();
    let total_start = web_time::Instant::now();
    let model: ArchitectureModel = {
        let _g = timing_section(timing_enabled, &mut timings.deserialize_model);
        crate::json::from_value_ref(semantic)?
    };
    render_architecture_diagram_svg_with_model(
        ArchitectureRenderRequest {
            layout,
            model: &model,
            effective_config: effective_config.as_value(),
            sanitize_config_opt: Some(effective_config),
            options,
        },
        ArchitectureTimingState {
            enabled: timing_enabled,
            timings: &mut timings,
            total_start,
        },
    )
}

fn render_architecture_diagram_svg_with_model<M: ArchitectureModelAccess>(
    req: ArchitectureRenderRequest<'_, M>,
    timing: ArchitectureTimingState<'_>,
) -> Result<String> {
    let ArchitectureRenderRequest {
        layout,
        model,
        effective_config,
        sanitize_config_opt,
        options,
    } = req;
    let ArchitectureTimingState {
        enabled: timing_enabled,
        timings,
        total_start,
    } = timing;

    fn section<'a>(
        enabled: bool,
        dst: &'a mut web_time::Duration,
    ) -> Option<super::timing::TimingGuard<'a>> {
        enabled.then(|| super::timing::TimingGuard::new(dst))
    }

    let _g_render_svg = section(timing_enabled, &mut timings.render_svg);

    let diagram_id = options.diagram_id.as_deref().unwrap_or("architecture");
    let settings = ArchitectureRenderSettings::from_config(diagram_id, effective_config);
    let ArchitectureRenderSettings {
        css,
        icon_size_px,
        half_icon,
        padding_px,
        arch_font_size_px,
        svg_font_size_px,
        use_max_width,
        text_style,
        compound_text_style,
    } = settings.clone();
    let sanitize_config_owned: merman_core::MermaidConfig;
    let sanitize_config = match sanitize_config_opt {
        Some(cfg) => cfg,
        None => {
            sanitize_config_owned =
                merman_core::MermaidConfig::from_value(effective_config.clone());
            &sanitize_config_owned
        }
    };

    let mut node_xy: rustc_hash::FxHashMap<&str, (f64, f64)> = rustc_hash::FxHashMap::default();
    for n in &layout.nodes {
        node_xy.insert(n.id.as_str(), (n.x, n.y));
    }

    let text_measurer = crate::text::VendoredFontMetricsTextMeasurer::default();

    let a11y = architecture_a11y_nodes(diagram_id, model.acc_title(), model.acc_descr());

    // Mermaid Architecture uses `setupGraphViewbox()` which expands the viewBox based on the
    // SVG's `getBBox()` plus `architecture.padding`. We approximate the effective `getBBox()` by
    // computing a conservative bounds over the elements we emit.
    let mut content_bounds: Option<Bounds> = None;

    // Mermaid `createText()` emits SVG `<text y="-10.1">` + `<tspan y="-0.1em" dy="1.1em">...`.
    //
    // In Chromium, `text.getBBox()` has:
    // - per-line height ~= 19px at 16px font size
    // - additional lines stacked by `dy="1.1em"` (i.e. 17.6px at 16px font size)
    //
    // Model this geometry in a scale-stable way so `setupGraphViewbox(svg.getBBox() + padding)`
    // aligns in `parity-root` comparisons without browser-dependent measurement.
    // Empirical bottom extension (beyond the icon bottom) of Mermaid `createText()` output for a
    // single-line label at 16px in Chromium, as observed in upstream Architecture baselines.
    //
    // This is notably larger than just `fontSize`, due to `createText()` using `<text y="-10.1">`
    // and wrapper attributes like `dy="1em"`; Chromium's `getBBox()` includes that geometry.
    // Cytoscape compound bounds (`node.boundingBox()`) include labels but do *not* match
    // Chromium's `text.getBBox()` exactly. In upstream Mermaid Architecture, group rectangles
    // sized from Cytoscape compound bounds tend to extend below the icon by roughly
    // `(fontSize + 1px)` for single-line service labels.
    //
    // If we reuse the larger root `getBBox()` extension for compounds, nested/group-heavy
    // fixtures get a systematic viewBox height inflation (~7.1875px at 16px).

    // Mermaid singleton top-level `iconText` services render 18px lower than the nominal
    // layout origin; keep the emitted transform and root bbox estimate in sync.
    let groups_len = model.groups_len();
    let edges_len = model.edges_len();
    let service_count = model.services().count();
    let junction_count = model.junctions().count();
    let singleton_icon_text_service_id =
        if groups_len == 0 && service_count == 1 && junction_count == 0 && edges_len == 0 {
            model.services().next().and_then(|service| {
                if service.in_group.is_none()
                    && service
                        .icon_text
                        .map(str::trim)
                        .is_some_and(|text: &str| !text.is_empty())
                {
                    Some(service.id)
                } else {
                    None
                }
            })
        } else {
            None
        };
    let singleton_icon_text_offset_y = |service_id: &str| {
        if singleton_icon_text_service_id == Some(service_id) {
            ARCHITECTURE_SERVICE_LABEL_BOTTOM_EXTENSION_PX
        } else {
            0.0
        }
    };

    let mut services_with_edges: rustc_hash::FxHashSet<&str> = rustc_hash::FxHashSet::default();
    for edge in model.edges() {
        services_with_edges.insert(edge.lhs_id);
        services_with_edges.insert(edge.rhs_id);
    }

    let mut service_bounds: rustc_hash::FxHashMap<&str, Bounds> = rustc_hash::FxHashMap::default();
    for svc in model.services() {
        let (x, y) = node_xy.get(svc.id).copied().unwrap_or((0.0, 0.0));
        let y = y + singleton_icon_text_offset_y(svc.id);
        let estimate = architecture_estimate_service_bounds(
            x,
            y,
            icon_size_px,
            arch_font_size_px,
            svg_font_size_px,
            svc.title,
            &text_measurer,
            &text_style,
            &compound_text_style,
            wrap_svg_words_to_lines,
            |line| svg_line_plain_text(line.as_slice()),
            |line, style| text_measurer.measure_svg_text_bbox_x(line, style),
        );
        let b_full = if svc.in_group.is_some() {
            estimate
                .cytoscape_group_child_contribution
                .union_bounds
                .clone()
        } else {
            estimate.svg_root_bounds.clone()
        };
        // Group rectangles (compound nodes) are sized by Cytoscape to include service labels, so
        // extending the root `getBBox()` estimate with *in-group* label bounds can double-count
        // and inflate the final `viewBox` / `max-width` in parity-root comparisons.
        //
        // Keep full label bounds for group sizing, but only union label extents into the root
        // viewport bounds when the service is not inside a group.
        service_bounds.insert(svc.id, b_full.clone());
        if svc.in_group.is_none() {
            // Connected top-level services still use the SVG root `createText()` label model.
            // Isolated top-level services in diagrams that also have groups behave like separate
            // Cytoscape components for root extent purposes; using the larger SVG root label phase
            // overcounts the disconnected-islands baseline while broadening the rule regresses
            // singleton/iconText rows.
            let root_bounds = architecture_top_level_service_root_bounds(
                &estimate,
                services_with_edges.contains(svc.id),
                groups_len > 0,
            );
            extend_bounds(&mut content_bounds, root_bounds);
        } else {
            extend_bounds(&mut content_bounds, estimate.emitted_icon_bounds);
        }
    }

    let mut junction_bounds: rustc_hash::FxHashMap<&str, Bounds> = rustc_hash::FxHashMap::default();
    for junction in model.junctions() {
        let (x, y) = node_xy.get(junction.id).copied().unwrap_or((0.0, 0.0));
        let b = bounds_from_rect(x, y, icon_size_px, icon_size_px);
        junction_bounds.insert(junction.id, b.clone());
        extend_bounds(&mut content_bounds, b);
    }

    // Groups (outer rects, including nested groups).
    let mut child_groups: rustc_hash::FxHashMap<&str, Vec<&str>> = rustc_hash::FxHashMap::default();
    for g in model.groups() {
        if let Some(parent) = g.in_group {
            child_groups.entry(parent).or_default().push(g.id);
        }
    }
    for v in child_groups.values_mut() {
        v.sort_unstable();
    }

    let mut services_in_group: rustc_hash::FxHashMap<&str, Vec<&str>> =
        rustc_hash::FxHashMap::default();
    for svc in model.services() {
        if let Some(parent) = svc.in_group {
            services_in_group.entry(parent).or_default().push(svc.id);
        }
    }
    for v in services_in_group.values_mut() {
        v.sort_unstable();
    }

    let mut junctions_in_group: rustc_hash::FxHashMap<&str, Vec<&str>> =
        rustc_hash::FxHashMap::default();
    for junction in model.junctions() {
        if let Some(parent) = junction.in_group {
            junctions_in_group
                .entry(parent)
                .or_default()
                .push(junction.id);
        }
    }
    for v in junctions_in_group.values_mut() {
        v.sort_unstable();
    }

    let mut group_rects_computer = GroupRectComputer::new(
        icon_size_px,
        padding_px,
        &services_in_group,
        &junctions_in_group,
        &child_groups,
        &service_bounds,
        &junction_bounds,
    );
    for g in model.groups() {
        let _ = group_rects_computer.compute(g.id);
    }

    let mut group_rects: Vec<GroupRect<'_>> = Vec::with_capacity(model.groups_len());
    for g in model.groups() {
        if let Some(b) = group_rects_computer.get(g.id) {
            group_rects.push(GroupRect {
                id: g.id,
                x: b.min_x,
                y: b.min_y,
                w: (b.max_x - b.min_x).max(1.0),
                h: (b.max_y - b.min_y).max(1.0),
                icon: g.icon,
                title: g.title,
            });
            extend_bounds(&mut content_bounds, b.clone());
        }
    }

    let is_empty = service_count == 0
        && junction_count == 0
        && model.groups_len() == 0
        && model.edges_len() == 0;

    let mut out = String::new();
    push_architecture_root_open(ArchitectureRootOpenContext {
        out: &mut out,
        diagram_id,
        css: css.as_str(),
        a11y: &a11y,
        is_empty,
        use_max_width,
        half_icon,
        icon_size_px,
    });
    // Edge bounds and DOM emission live in `architecture/edges.rs`.
    {
        let mut edge_render_ctx = ArchitectureEdgeRenderContext {
            out: &mut out,
            diagram_id,
            layout,
            model,
            node_xy: &node_xy,
            settings: &settings,
            text_measurer: &text_measurer,
            content_bounds: &mut content_bounds,
            junction_bounds: &junction_bounds,
        };
        push_architecture_edges(&mut edge_render_ctx);
    }
    out.push_str("</g>");

    {
        let mut node_render_ctx = ArchitectureNodeRenderContext {
            out: &mut out,
            diagram_id,
            model,
            node_xy: &node_xy,
            settings: &settings,
            text_measurer: &text_measurer,
            sanitize_config,
            icon_registry: options.icon_registry.as_deref(),
            content_bounds: &mut content_bounds,
            singleton_icon_text_service_id,
        };
        push_architecture_services_and_junctions(&mut node_render_ctx);
        push_architecture_groups(&mut node_render_ctx, &group_rects);
    }

    out.push_str("</svg>\n");

    if !is_empty {
        out = finalize_architecture_root_viewport(ArchitectureRootViewportContext {
            out,
            diagram_id,
            model,
            content_bounds,
            padding_px,
            icon_size_px,
            use_max_width,
            apply_root_overrides: options.apply_root_overrides,
        });
    }

    drop(_g_render_svg);

    timings.total = total_start.elapsed();
    if timing_enabled {
        eprintln!(
            "[render-timing] diagram=architecture total={:?} deserialize={:?} build_ctx={:?} viewbox={:?} render_svg={:?} finalize={:?}",
            timings.total,
            timings.deserialize_model,
            timings.build_ctx,
            timings.viewbox,
            timings.render_svg,
            timings.finalize_svg,
        );
    }

    Ok(out)
}
