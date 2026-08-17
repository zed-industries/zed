use super::*;

// Class diagram SVG renderer implementation (split from parity.rs).

type Rect = merman_core::geom::Box2;

mod bounds;

mod context;
use context::{ClassRenderDetails, ClassRenderLookups, emit_class_render_timing};

mod css;
use css::class_css;

mod debug_svg;
pub(super) fn render_class_diagram_v2_debug_svg(
    layout: &ClassDiagramV2Layout,
    options: &SvgRenderOptions,
) -> String {
    debug_svg::render_class_diagram_v2_debug_svg(layout, options)
}

mod defs;
use defs::{class_markers, push_class_gradient, push_class_shadow_defs};

mod edge;

mod groups;

mod interface;

mod label;

mod namespace;

mod node;

mod nodes;

mod note;

mod rough;

mod root;

mod settings;

mod viewbox;

type ClassSvgModel = merman_core::models::class_diagram::ClassDiagram;
type ClassSvgNode = merman_core::models::class_diagram::ClassNode;
type ClassSvgRelation = merman_core::models::class_diagram::ClassRelation;
type ClassSvgNote = merman_core::models::class_diagram::ClassNote;
type ClassSvgInterface = merman_core::models::class_diagram::ClassInterface;

mod render;

pub(super) fn render_class_diagram_v2_svg(
    layout: &ClassDiagramV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render::render_class_diagram_v2_svg_impl(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_class_diagram_v2_svg_model_with_config(
    layout: &ClassDiagramV2Layout,
    model: &ClassSvgModel,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render::render_class_diagram_v2_svg_model_impl_with_config(
        layout,
        model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}
