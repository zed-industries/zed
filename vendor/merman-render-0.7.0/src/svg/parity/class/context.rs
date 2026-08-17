use super::super::timing::RenderTimings;
use super::*;
use rustc_hash::FxHashMap;
use web_time::Duration;

#[derive(Debug, Default, Clone)]
pub(super) struct ClassRenderDetails {
    pub clusters: Duration,
    pub edge_paths: Duration,
    pub edge_curve: Duration,
    pub edge_points_json: Duration,
    pub edge_points_b64: Duration,
    pub edge_labels: Duration,
    pub nodes: Duration,
    pub notes_sanitize: Duration,
    pub path_bounds: Duration,
    pub path_bounds_calls: usize,
}

pub(super) struct ClassRenderLookups<'a> {
    pub class_nodes_by_id: FxHashMap<&'a str, &'a ClassSvgNode>,
    pub relations_by_id: FxHashMap<&'a str, &'a ClassSvgRelation>,
    pub relation_index_by_id: FxHashMap<&'a str, usize>,
    pub note_by_id: FxHashMap<&'a str, &'a ClassSvgNote>,
    pub iface_by_id: FxHashMap<&'a str, &'a ClassSvgInterface>,
}

impl<'a> ClassRenderLookups<'a> {
    pub(super) fn new(model: &'a ClassSvgModel) -> Self {
        let mut class_nodes_by_id: FxHashMap<&str, &ClassSvgNode> = FxHashMap::default();
        class_nodes_by_id.reserve(model.classes.len());
        for (id, n) in &model.classes {
            class_nodes_by_id.insert(id.as_str(), n);
        }

        let mut relations_by_id: FxHashMap<&str, &ClassSvgRelation> = FxHashMap::default();
        relations_by_id.reserve(model.relations.len());
        for r in &model.relations {
            relations_by_id.insert(r.id.as_str(), r);
        }

        let mut relation_index_by_id: FxHashMap<&str, usize> = FxHashMap::default();
        relation_index_by_id.reserve(model.relations.len());
        for (idx, r) in model.relations.iter().enumerate() {
            relation_index_by_id.insert(r.id.as_str(), idx + 1);
        }

        let mut note_by_id: FxHashMap<&str, &ClassSvgNote> = FxHashMap::default();
        note_by_id.reserve(model.notes.len());
        for n in &model.notes {
            note_by_id.insert(n.id.as_str(), n);
        }

        let mut iface_by_id: FxHashMap<&str, &ClassSvgInterface> = FxHashMap::default();
        iface_by_id.reserve(model.interfaces.len());
        for i in &model.interfaces {
            iface_by_id.insert(i.id.as_str(), i);
        }

        Self {
            class_nodes_by_id,
            relations_by_id,
            relation_index_by_id,
            note_by_id,
            iface_by_id,
        }
    }
}

pub(super) fn emit_class_render_timing(
    timings: &RenderTimings,
    detail: &ClassRenderDetails,
    layout: &ClassDiagramV2Layout,
) {
    eprintln!(
        "[render-timing] diagram=classDiagram total={:?} deserialize={:?} build_ctx={:?} viewbox={:?} render_svg={:?} finalize={:?} clusters={:?} edge_paths={:?} edge_curve={:?} edge_points_json={:?} edge_points_b64={:?} edge_labels={:?} nodes={:?} notes_sanitize={:?} path_bounds={:?} path_bounds_calls={} nodes_count={} edges_count={} clusters_count={}",
        timings.total,
        timings.deserialize_model,
        timings.build_ctx,
        timings.viewbox,
        timings.render_svg,
        timings.finalize_svg,
        detail.clusters,
        detail.edge_paths,
        detail.edge_curve,
        detail.edge_points_json,
        detail.edge_points_b64,
        detail.edge_labels,
        detail.nodes,
        detail.notes_sanitize,
        detail.path_bounds,
        detail.path_bounds_calls,
        layout.nodes.len(),
        layout.edges.len(),
        layout.clusters.len(),
    );
}
