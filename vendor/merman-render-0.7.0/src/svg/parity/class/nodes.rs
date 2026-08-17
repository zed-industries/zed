use super::context::ClassRenderDetails;
use super::groups::{
    ClassClusterEdgeGroupsRenderContext, ClassClusterEdgeGroupsRenderState,
    render_class_split_edge_groups,
};
use super::interface::{
    ClassInterfaceRenderContext, ClassInterfaceRenderState, render_class_interface_node,
};
use super::label::class_apply_inline_styles;
use super::namespace::{
    ClassNamespaceSubgraphState, ClassNodeRenderOrder, build_class_node_render_order,
    class_namespace_root_offset, class_render_parent_for_id, close_class_namespace_subgraph,
    render_class_namespace_clusters_in_root, transition_class_namespace_subgraph,
};
use super::node::{
    ClassHtmlNodeBodyContext, ClassNodeBasicContainerContext, ClassNodeRenderPosition,
    ClassNodeRenderState, ClassSvgNodeBodyContext, render_class_html_node_body,
    render_class_node_basic_container, render_class_node_shell_open, render_class_svg_node_body,
};
use super::note::{ClassNoteRenderContext, ClassNoteRenderState, render_class_note_node};
use super::settings::ClassRenderSettings;
use super::*;
use super::{ClassSvgInterface, ClassSvgModel, ClassSvgNode, ClassSvgNote};
use crate::model::{Bounds, ClassDiagramV2Layout, LayoutEdge};
use rustc_hash::FxHashMap;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct ClassNodeRootOffsets {
    nodes_root_dx: f64,
    nodes_root_dy: f64,
    namespace_root_dx: f64,
    namespace_root_dy: f64,
    in_namespace_root: bool,
}

pub(super) struct ClassNodesRenderState<'a> {
    pub(super) out: &'a mut String,
    pub(super) content_bounds: &'a mut Option<Bounds>,
    pub(super) detail: &'a mut ClassRenderDetails,
    pub(super) sanitize_config: &'a mut Option<merman_core::MermaidConfig>,
    pub(super) borrowed_sanitize_config: Option<&'a merman_core::MermaidConfig>,
}

pub(super) struct ClassNodesRenderContext<'a> {
    pub(super) layout: &'a ClassDiagramV2Layout,
    pub(super) model: &'a ClassSvgModel,
    pub(super) class_nodes_by_id: &'a FxHashMap<&'a str, &'a ClassSvgNode>,
    pub(super) note_by_id: &'a FxHashMap<&'a str, &'a ClassSvgNote>,
    pub(super) iface_by_id: &'a FxHashMap<&'a str, &'a ClassSvgInterface>,
    pub(super) settings: &'a ClassRenderSettings,
    pub(super) effective_config: &'a serde_json::Value,
    pub(super) diagram_id: &'a str,
    pub(super) measurer: &'a dyn TextMeasurer,
    pub(super) content_tx: f64,
    pub(super) content_ty: f64,
    pub(super) timing_enabled: bool,
    pub(super) wrap_nodes_root: bool,
    pub(super) single_namespace_id: Option<&'a str>,
    pub(super) render_namespaces_as_subgraphs: bool,
    pub(super) nodes_root_dx: f64,
    pub(super) nodes_root_dy: f64,
}

struct ClassNamespaceTreePlan<'a> {
    namespace_order: Vec<&'a str>,
    roots: Vec<&'a str>,
    children_by_ns: HashMap<&'a str, Vec<&'a str>>,
    direct_node_counts: HashMap<&'a str, usize>,
    root_by_node_id: HashMap<&'a str, Option<&'a str>>,
}

pub(super) fn render_class_nodes(
    state: ClassNodesRenderState<'_>,
    ctx: &ClassNodesRenderContext<'_>,
) {
    let ClassNodesRenderState {
        out,
        content_bounds,
        detail,
        sanitize_config,
        borrowed_sanitize_config,
    } = state;
    let ClassNodeRenderOrder {
        layout_nodes_by_id,
        ordered_ids,
        namespace_key_set,
        clusters_by_id,
    } = build_class_node_render_order(
        ctx.layout,
        ctx.model,
        ctx.class_nodes_by_id,
        ctx.note_by_id,
        ctx.iface_by_id,
        ctx.wrap_nodes_root,
        ctx.single_namespace_id,
        ctx.render_namespaces_as_subgraphs,
    );

    let mut inner_nodes_group_open = ctx.wrap_nodes_root;
    let mut namespace_subgraph_state = ClassNamespaceSubgraphState::default();
    for id in ordered_ids {
        if ctx.wrap_nodes_root && inner_nodes_group_open {
            let parent = class_render_parent_for_id(
                id,
                ctx.class_nodes_by_id,
                ctx.note_by_id,
                ctx.iface_by_id,
            );
            let should_be_inner = ctx.single_namespace_id.is_some_and(|ns| parent == Some(ns));
            if !should_be_inner {
                // Close the nested wrapper, then continue emitting remaining nodes at the outer level.
                out.push_str("</g>"); // inner nodes
                out.push_str("</g>"); // inner root
                inner_nodes_group_open = false;
            }
        }

        if ctx.render_namespaces_as_subgraphs {
            let parent = class_render_parent_for_id(
                id,
                ctx.class_nodes_by_id,
                ctx.note_by_id,
                ctx.iface_by_id,
            );
            let parent = parent.filter(|p| namespace_key_set.contains(p));
            transition_class_namespace_subgraph(
                out,
                content_bounds,
                &mut namespace_subgraph_state,
                parent,
                &clusters_by_id,
                ctx.diagram_id,
                ctx.settings.look.as_str(),
            );
        }

        let (active_nodes_root_dx, active_nodes_root_dy) =
            if ctx.wrap_nodes_root && inner_nodes_group_open {
                (ctx.nodes_root_dx, ctx.nodes_root_dy)
            } else {
                (0.0, 0.0)
            };
        let (active_namespace_root_dx, active_namespace_root_dy) =
            namespace_subgraph_state.root_offset.unwrap_or((0.0, 0.0));

        let in_namespace_root = ctx.render_namespaces_as_subgraphs
            && namespace_subgraph_state.active_subgraph.is_some();
        render_class_node_id(
            ClassNodesRenderState {
                out,
                content_bounds,
                detail,
                sanitize_config,
                borrowed_sanitize_config,
            },
            ctx,
            &layout_nodes_by_id,
            id,
            ClassNodeRootOffsets {
                nodes_root_dx: active_nodes_root_dx,
                nodes_root_dy: active_nodes_root_dy,
                namespace_root_dx: active_namespace_root_dx,
                namespace_root_dy: active_namespace_root_dy,
                in_namespace_root,
            },
        );
    }

    if ctx.render_namespaces_as_subgraphs {
        close_class_namespace_subgraph(out, &mut namespace_subgraph_state);
    }

    if inner_nodes_group_open {
        out.push_str("</g>"); // inner nodes
        out.push_str("</g>"); // inner root
    }
}

pub(super) fn render_class_namespace_subgraph_body(
    state: ClassNodesRenderState<'_>,
    ctx: &ClassNodesRenderContext<'_>,
    edge_ctx: &ClassClusterEdgeGroupsRenderContext<'_>,
) {
    let ClassNodesRenderState {
        out,
        content_bounds,
        detail,
        sanitize_config,
        borrowed_sanitize_config,
    } = state;

    let ClassNodeRenderOrder {
        layout_nodes_by_id,
        ordered_ids,
        namespace_key_set: _,
        clusters_by_id,
    } = build_class_node_render_order(
        ctx.layout,
        ctx.model,
        ctx.class_nodes_by_id,
        ctx.note_by_id,
        ctx.iface_by_id,
        false,
        None,
        true,
    );

    let plan = build_class_namespace_tree_plan(
        ctx,
        &ordered_ids,
        ctx.class_nodes_by_id,
        ctx.note_by_id,
        ctx.iface_by_id,
    );
    let (outer_edges, edges_by_root) =
        bucket_class_namespace_edges(edge_ctx.edges, edge_ctx, &plan);
    let outer_has_edges = !outer_edges.is_empty();
    let outer_split = render_class_split_edges_for_namespace(
        out,
        content_bounds,
        detail,
        edge_ctx,
        &outer_edges,
        0.0,
        0.0,
        false,
    );
    out.push_str(&outer_split.edge_labels);
    if !outer_has_edges {
        out.push_str(&outer_split.edge_paths);
    }

    out.push_str(r#"<g class="nodes">"#);
    for ns_id in &plan.roots {
        render_class_namespace_root(
            out,
            content_bounds,
            detail,
            sanitize_config,
            borrowed_sanitize_config,
            ctx,
            edge_ctx,
            &layout_nodes_by_id,
            &ordered_ids,
            &clusters_by_id,
            &plan,
            &edges_by_root,
            ns_id,
        );
    }
    for id in &ordered_ids {
        if plan.root_by_node_id.get(id).copied().flatten().is_some() {
            continue;
        }
        render_class_node_id(
            ClassNodesRenderState {
                out,
                content_bounds,
                detail,
                sanitize_config,
                borrowed_sanitize_config,
            },
            ctx,
            &layout_nodes_by_id,
            id,
            ClassNodeRootOffsets {
                nodes_root_dx: 0.0,
                nodes_root_dy: 0.0,
                namespace_root_dx: 0.0,
                namespace_root_dy: 0.0,
                in_namespace_root: false,
            },
        );
    }
    out.push_str("</g>");

    if outer_has_edges {
        out.push_str(&outer_split.edge_paths);
    }
}

fn build_class_namespace_tree_plan<'a>(
    ctx: &ClassNodesRenderContext<'a>,
    ordered_ids: &[&'a str],
    class_nodes_by_id: &FxHashMap<&'a str, &'a ClassSvgNode>,
    note_by_id: &FxHashMap<&'a str, &'a ClassSvgNote>,
    iface_by_id: &FxHashMap<&'a str, &'a ClassSvgInterface>,
) -> ClassNamespaceTreePlan<'a> {
    let namespace_order = crate::class::class_namespace_ids_in_decl_order(ctx.model);
    let namespace_set = namespace_order.iter().copied().collect::<HashSet<_>>();
    let mut roots = Vec::new();
    let mut children_by_ns: HashMap<&str, Vec<&str>> = HashMap::new();
    for ns_id in &namespace_order {
        let parent = ctx
            .model
            .namespaces
            .get(*ns_id)
            .and_then(|ns| ns.parent.as_deref())
            .filter(|parent| namespace_set.contains(parent));
        if let Some(parent) = parent {
            children_by_ns.entry(parent).or_default().push(*ns_id);
        } else {
            roots.push(*ns_id);
        }
    }

    let mut direct_node_counts: HashMap<&str, usize> = HashMap::new();
    for id in ordered_ids {
        if let Some(parent) =
            class_render_parent_for_id(id, class_nodes_by_id, note_by_id, iface_by_id)
                .filter(|parent| namespace_set.contains(parent))
        {
            *direct_node_counts.entry(parent).or_insert(0) += 1;
        }
    }

    let mut root_by_node_id = HashMap::new();
    for id in ordered_ids {
        let root = class_render_parent_for_id(id, class_nodes_by_id, note_by_id, iface_by_id)
            .filter(|parent| namespace_set.contains(parent))
            .and_then(|parent| {
                class_namespace_flat_render_root(parent, ctx.model, &direct_node_counts)
            });
        root_by_node_id.insert(*id, root);
    }

    ClassNamespaceTreePlan {
        namespace_order,
        roots,
        children_by_ns,
        direct_node_counts,
        root_by_node_id,
    }
}

fn class_namespace_flat_render_root<'a>(
    ns_id: &'a str,
    model: &'a ClassSvgModel,
    direct_node_counts: &HashMap<&'a str, usize>,
) -> Option<&'a str> {
    let mut cur = Some(ns_id);
    let mut selected = None;
    while let Some(id) = cur {
        if direct_node_counts.get(id).copied().unwrap_or(0) > 0 {
            selected = Some(id);
        }
        cur = model.namespaces.get(id).and_then(|ns| ns.parent.as_deref());
    }
    selected.or(Some(ns_id))
}

fn class_namespace_is_descendant_or_self(
    ns_id: &str,
    ancestor: &str,
    model: &ClassSvgModel,
) -> bool {
    if ns_id == ancestor {
        return true;
    }
    let mut cur = model
        .namespaces
        .get(ns_id)
        .and_then(|ns| ns.parent.as_deref());
    while let Some(parent) = cur {
        if parent == ancestor {
            return true;
        }
        cur = model
            .namespaces
            .get(parent)
            .and_then(|ns| ns.parent.as_deref());
    }
    false
}

fn bucket_class_namespace_edges<'a>(
    edges: &'a [LayoutEdge],
    edge_ctx: &ClassClusterEdgeGroupsRenderContext<'a>,
    plan: &ClassNamespaceTreePlan<'a>,
) -> (Vec<LayoutEdge>, HashMap<&'a str, Vec<LayoutEdge>>) {
    let mut outer_edges = Vec::new();
    let mut edges_by_root: HashMap<&str, Vec<LayoutEdge>> = HashMap::new();

    for edge in edges {
        let mut from_root = plan
            .root_by_node_id
            .get(edge.from.as_str())
            .copied()
            .flatten();
        let mut to_root = plan
            .root_by_node_id
            .get(edge.to.as_str())
            .copied()
            .flatten();
        if from_root.is_none() || to_root.is_none() {
            if let Some(rel) = edge_ctx.relations_by_id.get(edge.id.as_str()).copied() {
                from_root = from_root.or_else(|| {
                    plan.root_by_node_id
                        .get(rel.id1.as_str())
                        .copied()
                        .flatten()
                });
                to_root = to_root.or_else(|| {
                    plan.root_by_node_id
                        .get(rel.id2.as_str())
                        .copied()
                        .flatten()
                });
            }
        }

        match (from_root, to_root) {
            (Some(from_root), Some(to_root)) if from_root == to_root => {
                edges_by_root
                    .entry(from_root)
                    .or_default()
                    .push(edge.clone());
            }
            _ => outer_edges.push(edge.clone()),
        }
    }

    (outer_edges, edges_by_root)
}

fn render_class_split_edges_for_namespace(
    out: &mut String,
    content_bounds: &mut Option<Bounds>,
    detail: &mut ClassRenderDetails,
    edge_ctx: &ClassClusterEdgeGroupsRenderContext<'_>,
    edges: &[LayoutEdge],
    root_dx: f64,
    root_dy: f64,
    in_namespace_root: bool,
) -> super::groups::ClassSplitEdgeGroups {
    let local_ctx = ClassClusterEdgeGroupsRenderContext {
        clusters: edge_ctx.clusters,
        edges,
        relations_by_id: edge_ctx.relations_by_id,
        relation_index_by_id: edge_ctx.relation_index_by_id,
        marker_url_prefix: edge_ctx.marker_url_prefix,
        diagram_id: edge_ctx.diagram_id,
        content_tx: if in_namespace_root {
            edge_ctx.content_tx - root_dx
        } else {
            edge_ctx.content_tx
        },
        content_ty: if in_namespace_root {
            edge_ctx.content_ty - root_dy
        } else {
            edge_ctx.content_ty
        },
        edge_use_html_labels: edge_ctx.edge_use_html_labels,
        look: edge_ctx.look,
        timing_enabled: edge_ctx.timing_enabled,
    };
    render_class_split_edge_groups(
        ClassClusterEdgeGroupsRenderState {
            out,
            content_bounds,
            detail,
        },
        &local_ctx,
        if in_namespace_root { root_dx } else { 0.0 },
        if in_namespace_root { root_dy } else { 0.0 },
    )
}

#[allow(clippy::too_many_arguments)]
fn render_class_namespace_root(
    out: &mut String,
    content_bounds: &mut Option<Bounds>,
    detail: &mut ClassRenderDetails,
    sanitize_config: &mut Option<merman_core::MermaidConfig>,
    borrowed_sanitize_config: Option<&merman_core::MermaidConfig>,
    ctx: &ClassNodesRenderContext<'_>,
    edge_ctx: &ClassClusterEdgeGroupsRenderContext<'_>,
    layout_nodes_by_id: &FxHashMap<&str, &crate::model::LayoutNode>,
    ordered_ids: &[&str],
    clusters_by_id: &HashMap<&str, &crate::model::LayoutCluster>,
    plan: &ClassNamespaceTreePlan<'_>,
    edges_by_root: &HashMap<&str, Vec<LayoutEdge>>,
    ns_id: &str,
) {
    enum NamespaceRootFrame<'a> {
        Enter(&'a str),
        Close { has_edges: bool, edge_paths: String },
    }

    let mut stack = vec![NamespaceRootFrame::Enter(ns_id)];
    while let Some(frame) = stack.pop() {
        match frame {
            NamespaceRootFrame::Enter(ns_id) => {
                let Some(root_cluster) = clusters_by_id.get(ns_id).copied() else {
                    continue;
                };
                let (root_dx, root_dy) = class_namespace_root_offset(root_cluster);
                let _ = write!(
                    out,
                    r#"<g class="root" transform="translate({}, {})">"#,
                    fmt(root_dx),
                    fmt(root_dy)
                );

                let flatten_descendants =
                    plan.direct_node_counts.get(ns_id).copied().unwrap_or(0) > 0;
                let cluster_ids = if flatten_descendants {
                    plan.namespace_order
                        .iter()
                        .copied()
                        .filter(|candidate| {
                            class_namespace_is_descendant_or_self(candidate, ns_id, ctx.model)
                        })
                        .collect::<Vec<_>>()
                } else {
                    vec![ns_id]
                };
                render_class_namespace_clusters_in_root(
                    out,
                    content_bounds,
                    clusters_by_id,
                    &cluster_ids,
                    super::namespace::ClassNamespaceClusterGroupContext {
                        diagram_id: ctx.diagram_id,
                        content_tx: ctx.content_tx,
                        content_ty: ctx.content_ty,
                        bounds_dx: 0.0,
                        bounds_dy: 0.0,
                        look: ctx.settings.look.as_str(),
                        timing_enabled: ctx.timing_enabled,
                    },
                    ns_id,
                    root_dx,
                    root_dy,
                );

                let edges = edges_by_root
                    .get(ns_id)
                    .map(|edges| edges.as_slice())
                    .unwrap_or(&[]);
                let has_edges = !edges.is_empty();
                let split = render_class_split_edges_for_namespace(
                    out,
                    content_bounds,
                    detail,
                    edge_ctx,
                    edges,
                    root_dx,
                    root_dy,
                    true,
                );
                out.push_str(&split.edge_labels);
                if !has_edges {
                    out.push_str(&split.edge_paths);
                }

                out.push_str(r#"<g class="nodes">"#);
                if flatten_descendants {
                    for id in ordered_ids {
                        if plan.root_by_node_id.get(id).copied().flatten() != Some(ns_id) {
                            continue;
                        }
                        render_class_node_id(
                            ClassNodesRenderState {
                                out,
                                content_bounds,
                                detail,
                                sanitize_config,
                                borrowed_sanitize_config,
                            },
                            ctx,
                            layout_nodes_by_id,
                            id,
                            ClassNodeRootOffsets {
                                nodes_root_dx: 0.0,
                                nodes_root_dy: 0.0,
                                namespace_root_dx: root_dx,
                                namespace_root_dy: root_dy,
                                in_namespace_root: true,
                            },
                        );
                    }
                    out.push_str("</g>");
                    if has_edges {
                        out.push_str(&split.edge_paths);
                    }
                    out.push_str("</g>");
                    continue;
                }

                stack.push(NamespaceRootFrame::Close {
                    has_edges,
                    edge_paths: split.edge_paths,
                });
                if let Some(children) = plan.children_by_ns.get(ns_id) {
                    for child in children.iter().rev() {
                        stack.push(NamespaceRootFrame::Enter(child));
                    }
                }
            }
            NamespaceRootFrame::Close {
                has_edges,
                edge_paths,
            } => {
                out.push_str("</g>");
                if has_edges {
                    out.push_str(&edge_paths);
                }
                out.push_str("</g>");
            }
        }
    }
}

fn render_class_node_id(
    state: ClassNodesRenderState<'_>,
    ctx: &ClassNodesRenderContext<'_>,
    layout_nodes_by_id: &FxHashMap<&str, &crate::model::LayoutNode>,
    id: &str,
    offsets: ClassNodeRootOffsets,
) {
    let ClassNodesRenderState {
        out,
        content_bounds,
        detail,
        sanitize_config,
        borrowed_sanitize_config,
    } = state;
    let settings = ctx.settings;

    let Some(n) = layout_nodes_by_id.get(id).copied() else {
        return;
    };

    let node_tx = if offsets.in_namespace_root {
        n.x - offsets.namespace_root_dx
    } else {
        n.x + ctx.content_tx
    };
    let node_ty = if offsets.in_namespace_root {
        n.y + ctx.content_ty - offsets.namespace_root_dy
    } else {
        n.y + ctx.content_ty
    };
    let node_bounds_tx = node_tx + offsets.namespace_root_dx + offsets.nodes_root_dx;
    let node_bounds_ty = node_ty + offsets.namespace_root_dy + offsets.nodes_root_dy;
    let position = ClassNodeRenderPosition {
        node_tx,
        node_ty,
        node_bounds_tx,
        node_bounds_ty,
    };

    if let Some(note) = ctx.note_by_id.get(n.id.as_str()).copied() {
        let stats = render_class_note_node(
            ClassNoteRenderState {
                out,
                content_bounds,
                sanitize_config,
                borrowed_sanitize_config,
            },
            note,
            n,
            position,
            &ClassNoteRenderContext {
                diagram_id: ctx.diagram_id,
                effective_config: ctx.effective_config,
                measurer: ctx.measurer,
                text_style: &settings.text_style,
                line_height: settings.line_height,
                use_html_labels: settings.diagram_use_html_labels,
                look: settings.look.as_str(),
                timing_enabled: ctx.timing_enabled,
            },
        );
        detail.notes_sanitize += stats.notes_sanitize;
        detail.path_bounds += stats.path_bounds;
        detail.path_bounds_calls += stats.path_bounds_calls;
        return;
    }

    if let Some(iface) = ctx.iface_by_id.get(n.id.as_str()).copied() {
        render_class_interface_node(
            ClassInterfaceRenderState {
                out,
                content_bounds,
            },
            iface,
            n,
            position,
            &ClassInterfaceRenderContext {
                diagram_id: ctx.diagram_id,
                measurer: ctx.measurer,
                text_style: &settings.text_style,
                line_height: settings.line_height,
                look: settings.look.as_str(),
            },
        );
        return;
    }

    let Some(node) = ctx.class_nodes_by_id.get(n.id.as_str()).copied() else {
        return;
    };

    let node_inline_styles = class_apply_inline_styles(node);
    let node_style_attr = node_inline_styles.style_attr.as_str();
    let node_fill = node_inline_styles
        .fill
        .unwrap_or(settings.default_node_fill.as_str());
    let node_stroke = node_inline_styles
        .stroke
        .unwrap_or(settings.default_node_stroke.as_str());
    let node_stroke_width = node_inline_styles
        .stroke_width
        .unwrap_or("1.3")
        .trim_end_matches("px")
        .trim();
    let node_stroke_dasharray = node_inline_styles.stroke_dasharray.unwrap_or("0 0");

    let node_link_open =
        render_class_node_shell_open(out, node, position, ctx.diagram_id, settings.look.as_str());
    let basic_container = render_class_node_basic_container(
        ClassNodeRenderState {
            out,
            content_bounds,
        },
        node,
        n,
        position,
        &ClassNodeBasicContainerContext {
            diagram_id: ctx.diagram_id,
            node_style_attr,
            node_fill,
            node_stroke,
            node_stroke_width,
            node_stroke_dasharray,
            timing_enabled: ctx.timing_enabled,
        },
    );
    detail.path_bounds += basic_container.stats.path_bounds;
    detail.path_bounds_calls += basic_container.stats.path_bounds_calls;

    if settings.diagram_use_html_labels {
        let html_stats = render_class_html_node_body(
            ClassNodeRenderState {
                out,
                content_bounds,
            },
            position,
            node,
            basic_container.geometry,
            ctx.layout
                .class_row_metrics_by_id
                .get(n.id.as_str())
                .map(|rows| rows.as_ref()),
            &ClassHtmlNodeBodyContext {
                measurer: ctx.measurer,
                text_style: &settings.text_style,
                html_calc_text_style: &settings.html_calc_text_style,
                line_height: settings.line_height,
                class_padding: settings.class_padding,
                hide_empty_members_box: settings.hide_empty_members_box,
                node_style_attr,
                node_stroke,
                node_stroke_width,
                node_stroke_dasharray,
                timing_enabled: ctx.timing_enabled,
            },
        );
        detail.path_bounds += html_stats.path_bounds;
        detail.path_bounds_calls += html_stats.path_bounds_calls;
    } else {
        let svg_stats = render_class_svg_node_body(
            ClassNodeRenderState {
                out,
                content_bounds,
            },
            position,
            node,
            basic_container.geometry,
            &ClassSvgNodeBodyContext {
                measurer: ctx.measurer,
                text_style: &settings.text_style,
                font_size: settings.font_size,
                wrap_probe_font_size: settings.wrap_probe_font_size,
                class_padding: settings.class_padding,
                hide_empty_members_box: settings.hide_empty_members_box,
                node_style_attr,
                node_stroke,
                node_stroke_width,
                node_stroke_dasharray,
                timing_enabled: ctx.timing_enabled,
            },
        );
        detail.path_bounds += svg_stats.path_bounds;
        detail.path_bounds_calls += svg_stats.path_bounds_calls;
    }

    out.push_str("</g>");
    if node_link_open {
        out.push_str("</a>");
    }
}
