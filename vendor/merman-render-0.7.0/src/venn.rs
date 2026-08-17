//! Source-backed Venn layout kernel and diagram adapter.
//!
//! This module ports the layout/geometry path used by `@upsetjs/venn.js@2.0.0` and the minimal
//! `fmin@0.0.4` optimizer helpers it depends on. The diagram adapter is intentionally thin: it
//! projects the core render model into the same helper layout surface that Mermaid consumes before
//! SVG emission.

use crate::config::{config_bool, config_f64};
use crate::model::{
    Bounds as LayoutBounds, VennAreaLayout, VennCircleLayout, VennDiagramLayout,
    VennTextAreaLayout, VennTextDebugCellLayout, VennTextNodeLayout,
};
use crate::{Error, Result};
use indexmap::IndexMap;
use merman_core::diagrams::venn::VennDiagramRenderModel;
use ryu_js::Buffer;
use std::collections::{HashMap, HashSet};
use std::f64::consts::{PI, TAU};

const SMALL: f64 = 1e-10;
const DEFAULT_SVG_WIDTH: f64 = 800.0;
const DEFAULT_SVG_HEIGHT: f64 = 450.0;
const DEFAULT_PADDING: f64 = 15.0;
const REFERENCE_WIDTH: f64 = 1600.0;

pub fn layout_venn_diagram(
    semantic: &serde_json::Value,
    diagram_title: Option<&str>,
    effective_config: &serde_json::Value,
) -> Result<VennDiagramLayout> {
    let model: VennDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    layout_venn_diagram_typed(&model, diagram_title, effective_config)
}

pub fn layout_venn_diagram_typed(
    model: &VennDiagramRenderModel,
    diagram_title: Option<&str>,
    effective_config: &serde_json::Value,
) -> Result<VennDiagramLayout> {
    let width = config_f64(effective_config, &["venn", "width"])
        .unwrap_or(DEFAULT_SVG_WIDTH)
        .max(1.0);
    let height = config_f64(effective_config, &["venn", "height"])
        .unwrap_or(DEFAULT_SVG_HEIGHT)
        .max(1.0);
    let padding = config_f64(effective_config, &["venn", "padding"])
        .unwrap_or(DEFAULT_PADDING)
        .max(0.0);
    let use_max_width = config_bool(effective_config, &["venn", "useMaxWidth"]).unwrap_or(true);
    let use_debug_layout =
        config_bool(effective_config, &["venn", "useDebugLayout"]).unwrap_or(false);
    let title = model
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .or_else(|| {
            diagram_title
                .map(str::trim)
                .filter(|title| !title.is_empty())
        });
    let scale = width / REFERENCE_WIDTH;
    let title_height = if title.is_some() { 48.0 * scale } else { 0.0 };
    let diagram_height = (height - title_height).max(1.0);

    let areas = model
        .subsets
        .iter()
        .map(|subset| VennArea {
            sets: subset.sets.clone(),
            size: subset.size,
            weight: None,
            label: subset.label.clone(),
        })
        .collect::<Vec<_>>();
    let layout_areas = if areas.is_empty() {
        Vec::new()
    } else {
        compute_venn_layout(
            &areas,
            &VennLayoutOptions {
                width,
                height: diagram_height,
                padding,
                ..Default::default()
            },
        )
        .map_err(|err| Error::InvalidModel {
            message: err.to_string(),
        })?
    };

    let areas = layout_areas
        .iter()
        .map(|area| VennAreaLayout {
            sets: area.data.sets.clone(),
            size: area.data.size,
            label: area.data.label.clone(),
            text_x: area.text.x.floor(),
            text_y: area.text.y.floor(),
            text_disjoint: area.text.disjoint,
            circles: area
                .circles
                .iter()
                .map(|circle| VennCircleLayout {
                    set: circle.set.clone(),
                    x: circle.x,
                    y: circle.y,
                    radius: circle.radius,
                })
                .collect(),
            path: area.path.clone(),
            distinct_path: area.distinct_path.clone(),
        })
        .collect::<Vec<_>>();
    let layout_by_key = layout_areas
        .iter()
        .map(|area| (stable_sets_key(&area.data.sets), area))
        .collect::<HashMap<_, _>>();
    let (text_areas, text_nodes) =
        layout_text_nodes(model, &layout_by_key, scale, use_debug_layout);

    Ok(VennDiagramLayout {
        bounds: Some(LayoutBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: width,
            max_y: height,
        }),
        width,
        height,
        diagram_height,
        title_height,
        scale,
        padding,
        use_max_width,
        use_debug_layout,
        areas,
        text_areas,
        text_nodes,
    })
}

fn layout_text_nodes(
    model: &VennDiagramRenderModel,
    layout_by_key: &HashMap<String, &VennLayoutArea>,
    scale: f64,
    use_debug_layout: bool,
) -> (Vec<VennTextAreaLayout>, Vec<VennTextNodeLayout>) {
    let mut nodes_by_area: IndexMap<
        String,
        Vec<&merman_core::diagrams::venn::VennTextNodeRenderModel>,
    > = IndexMap::new();
    for node in &model.text_nodes {
        nodes_by_area
            .entry(stable_sets_key(&node.sets))
            .or_default()
            .push(node);
    }

    let mut text_areas = Vec::new();
    let mut text_nodes = Vec::new();
    for (key, nodes) in nodes_by_area {
        let Some(area) = layout_by_key.get(&key).copied() else {
            continue;
        };
        if area.circles.is_empty() {
            continue;
        }

        let center_x = area.text.x;
        let center_y = area.text.y;
        let min_circle_radius = area
            .circles
            .iter()
            .map(|circle| circle.radius)
            .fold(f64::INFINITY, f64::min);
        let inner_radius_raw = area
            .circles
            .iter()
            .map(|circle| {
                circle.radius
                    - ((center_x - circle.x).powi(2) + (center_y - circle.y).powi(2)).sqrt()
            })
            .fold(f64::INFINITY, f64::min);
        let mut inner_radius = if inner_radius_raw.is_finite() {
            inner_radius_raw.max(0.0)
        } else {
            0.0
        };
        if inner_radius == 0.0 && min_circle_radius.is_finite() {
            inner_radius = min_circle_radius * 0.6;
        }

        let inner_width = (80.0 * scale).max(inner_radius * 2.0 * 0.95);
        let inner_height = (60.0 * scale).max(inner_radius * 2.0 * 0.95);
        let has_label = area
            .data
            .label
            .as_deref()
            .is_some_and(|label| !label.is_empty());
        let label_offset_base = if has_label {
            (32.0 * scale).min(inner_radius * 0.25)
        } else {
            0.0
        };
        let label_offset = label_offset_base + if nodes.len() <= 2 { 30.0 * scale } else { 0.0 };
        let start_x = center_x - inner_width / 2.0;
        let start_y = center_y - inner_height / 2.0 + label_offset;
        let cols = (nodes.len() as f64).sqrt().ceil().max(1.0) as usize;
        let rows = nodes.len().div_ceil(cols).max(1);
        let cell_width = inner_width / cols as f64;
        let cell_height = inner_height / rows as f64;

        let mut debug_cells = Vec::new();
        for (index, node) in nodes.iter().enumerate() {
            let col = index % cols;
            let row = index / cols;
            let cell_x = start_x + cell_width * col as f64;
            let cell_y = start_y + cell_height * row as f64;
            if use_debug_layout {
                debug_cells.push(VennTextDebugCellLayout {
                    x: cell_x,
                    y: cell_y,
                    width: cell_width,
                    height: cell_height,
                });
            }

            let x = start_x + cell_width * (col as f64 + 0.5);
            let y = start_y + cell_height * (row as f64 + 0.5);
            let box_width = cell_width * 0.9;
            let box_height = cell_height * 0.9;
            text_nodes.push(VennTextNodeLayout {
                sets: node.sets.clone(),
                id: node.id.clone(),
                label: node.label.clone(),
                x: x - box_width / 2.0,
                y: y - box_height / 2.0,
                width: box_width,
                height: box_height,
            });
        }

        text_areas.push(VennTextAreaLayout {
            sets: area.data.sets.clone(),
            center_x,
            center_y,
            inner_radius,
            font_size: 40.0 * scale,
            debug_cells,
        });
    }

    (text_areas, text_nodes)
}

fn stable_sets_key(sets: &[String]) -> String {
    sets.join("|")
}

#[derive(Debug, Clone, PartialEq)]
pub struct VennArea {
    pub sets: Vec<String>,
    pub size: f64,
    pub weight: Option<f64>,
    pub label: Option<String>,
}

impl VennArea {
    pub fn new(sets: impl IntoIterator<Item = impl Into<String>>, size: f64) -> Self {
        Self {
            sets: sets.into_iter().map(Into::into).collect(),
            size,
            weight: None,
            label: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VennPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VennCircle {
    pub set: String,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VennArc {
    pub circle: VennCircle,
    pub width: f64,
    pub p1: VennPoint,
    pub p2: VennPoint,
    pub large: bool,
    pub sweep: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VennTextPoint {
    pub x: f64,
    pub y: f64,
    pub disjoint: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VennLayoutArea {
    pub data: VennArea,
    pub text: VennTextPoint,
    pub circles: Vec<VennCircle>,
    pub arcs: Vec<VennArc>,
    pub path: String,
    pub distinct_path: String,
}

#[derive(Debug, Clone)]
pub struct VennLayoutOptions {
    pub width: f64,
    pub height: f64,
    pub padding: f64,
    pub normalize: bool,
    pub orientation: f64,
    pub scale_to_fit: Option<f64>,
    pub symmetrical_text_centre: bool,
    pub distinct: bool,
    pub round: Option<usize>,
    pub max_iterations: usize,
    pub restarts: usize,
    pub random_seed: u64,
}

impl Default for VennLayoutOptions {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 350.0,
            padding: 15.0,
            normalize: true,
            orientation: PI / 2.0,
            scale_to_fit: None,
            symmetrical_text_centre: false,
            distinct: false,
            // `diagram.js` helper defaults `round` to 2.
            round: Some(2),
            max_iterations: 500,
            restarts: 10,
            random_seed: 1,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VennLayoutError {
    #[error("missing pairwise overlap information for Venn set `{0}`")]
    MissingPairwiseOverlap(String),
    #[error("Venn layout requires at least one single-set area")]
    EmptySingleSetAreas,
}

pub type VennLayoutResult<T> = std::result::Result<T, VennLayoutError>;
pub type VennSolution = IndexMap<String, VennCircle>;

type VennResult<T> = VennLayoutResult<T>;
type Solution = VennSolution;

#[derive(Debug, Clone)]
struct AreaStats {
    area: f64,
    arcs: Vec<VennArc>,
}

#[derive(Debug, Clone)]
struct IntersectionPoint {
    point: VennPoint,
    parent_index: [usize; 2],
    angle: f64,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

#[derive(Debug, Clone)]
struct OptimParams {
    max_iterations: Option<usize>,
    min_error_delta: Option<f64>,
}

#[derive(Debug, Clone)]
struct OptimResult {
    x: Vec<f64>,
}

#[derive(Debug, Clone)]
struct CgState {
    x: Vec<f64>,
    fx: f64,
    fxprime: Vec<f64>,
}

#[derive(Debug, Clone)]
struct XorShift64Star {
    state: u64,
}

impl XorShift64Star {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D_u64)
    }

    fn next_f64_unit(&mut self) -> f64 {
        let u = self.next_u64() >> 11;
        (u as f64) / ((1u64 << 53) as f64)
    }
}

pub fn compute_venn_layout(
    data: &[VennArea],
    options: &VennLayoutOptions,
) -> VennLayoutResult<Vec<VennLayoutArea>> {
    let mut solution = venn(data, options)?;
    if options.normalize {
        solution = normalize_solution(&solution, options.orientation);
    }
    let circles = scale_solution(
        &solution,
        options.width,
        options.height,
        options.padding,
        options.scale_to_fit,
    );
    let text_centres = compute_text_centres(&circles, data, options.symmetrical_text_centre);

    let mut helpers = Vec::with_capacity(data.len());
    for area in data {
        let area_circles: Vec<VennCircle> = area
            .sets
            .iter()
            .filter_map(|set| circles.get(set).cloned())
            .collect();
        let arcs = intersection_area_arcs(&area_circles);
        let path = arcs_to_path(&arcs, options.round);
        let text = text_centres
            .get(&sets_key(&area.sets))
            .copied()
            .unwrap_or(VennTextPoint {
                x: 0.0,
                y: 0.0,
                disjoint: true,
            });
        helpers.push(VennLayoutArea {
            data: area.clone(),
            text,
            circles: area_circles,
            arcs,
            path,
            distinct_path: String::new(),
        });
    }

    for i in 0..helpers.len() {
        let mut distinct_path = helpers[i].path.clone();
        for j in 0..helpers.len() {
            if helpers[j].data.sets.len() > helpers[i].data.sets.len()
                && helpers[i]
                    .data
                    .sets
                    .iter()
                    .all(|set| helpers[j].data.sets.contains(set))
            {
                distinct_path.push(' ');
                distinct_path.push_str(&helpers[j].path);
            }
        }
        helpers[i].distinct_path = distinct_path;
    }

    Ok(helpers)
}

pub fn venn(sets: &[VennArea], options: &VennLayoutOptions) -> VennLayoutResult<VennSolution> {
    let areas = add_missing_areas(sets, options.distinct);
    let mut circles = best_initial_layout(&areas, options)?;
    let setids: Vec<String> = circles.keys().cloned().collect();
    let mut initial = Vec::with_capacity(setids.len() * 2);
    for setid in &setids {
        let circle = &circles[setid];
        initial.push(circle.x);
        initial.push(circle.y);
    }

    let params = OptimParams {
        max_iterations: Some(options.max_iterations),
        min_error_delta: None,
    };
    let solution = nelder_mead(
        |values| {
            let mut current = Solution::new();
            for (i, setid) in setids.iter().enumerate() {
                let base = &circles[setid];
                current.insert(
                    setid.clone(),
                    VennCircle {
                        set: setid.clone(),
                        x: values[2 * i],
                        y: values[2 * i + 1],
                        radius: base.radius,
                    },
                );
            }
            loss_function(&current, &areas)
        },
        &initial,
        &params,
    );

    for (i, setid) in setids.iter().enumerate() {
        if let Some(circle) = circles.get_mut(setid) {
            circle.x = solution.x[2 * i];
            circle.y = solution.x[2 * i + 1];
        }
    }
    Ok(circles)
}

fn add_missing_areas(areas: &[VennArea], distinct: bool) -> Vec<VennArea> {
    let mut out = areas.to_vec();

    if distinct {
        let mut count: HashMap<String, f64> = HashMap::new();
        for area in &out {
            for i in 0..area.sets.len() {
                let si = &area.sets[i];
                *count.entry(si.clone()).or_insert(0.0) += area.size;
                for j in i + 1..area.sets.len() {
                    let sj = &area.sets[j];
                    *count.entry(format!("{si};{sj}")).or_insert(0.0) += area.size;
                    *count.entry(format!("{sj};{si}")).or_insert(0.0) += area.size;
                }
            }
        }
        for area in &mut out {
            if area.sets.len() < 3 {
                if let Some(size) = count.get(&area.sets.join(";")).copied() {
                    area.size = size;
                }
            }
        }
    }

    let mut ids = Vec::new();
    let mut pairs = HashSet::new();
    for area in &out {
        if area.sets.len() == 1 {
            ids.push(area.sets[0].clone());
        } else if area.sets.len() == 2 {
            let a = &area.sets[0];
            let b = &area.sets[1];
            pairs.insert(format!("{a};{b}"));
            pairs.insert(format!("{b};{a}"));
        }
    }

    ids.sort();
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let a = &ids[i];
            let b = &ids[j];
            if !pairs.contains(&format!("{a};{b}")) {
                out.push(VennArea::new([a.clone(), b.clone()], 0.0));
            }
        }
    }
    out
}

pub fn distance_from_intersect_area(r1: f64, r2: f64, overlap: f64) -> f64 {
    if r1.min(r2).powi(2) * PI <= overlap + SMALL {
        return (r1 - r2).abs();
    }
    bisect(
        |distance| circle_overlap(r1, r2, distance) - overlap,
        0.0,
        r1 + r2,
        100,
        1e-10,
    )
}

fn best_initial_layout(areas: &[VennArea], options: &VennLayoutOptions) -> VennResult<Solution> {
    let mut initial = greedy_layout(areas)?;
    if areas.len() >= 8 {
        let constrained = constrained_mds_layout(areas, options)?;
        let constrained_loss = loss_function(&constrained, areas);
        let greedy_loss = loss_function(&initial, areas);
        if constrained_loss + 1e-8 < greedy_loss {
            initial = constrained;
        }
    }
    Ok(initial)
}

pub fn greedy_layout(areas: &[VennArea]) -> VennLayoutResult<VennSolution> {
    let mut circles = Solution::new();
    let mut set_overlaps: IndexMap<String, Vec<SetOverlap>> = IndexMap::new();

    for area in areas {
        if area.sets.len() == 1 {
            let set = area.sets[0].clone();
            circles.insert(
                set.clone(),
                VennCircle {
                    set: set.clone(),
                    x: 1e10,
                    y: 1e10,
                    radius: (area.size / PI).sqrt(),
                },
            );
            set_overlaps.insert(set, Vec::new());
        }
    }
    if circles.is_empty() {
        return Err(VennLayoutError::EmptySingleSetAreas);
    }

    for area in areas.iter().filter(|a| a.sets.len() == 2) {
        let left = &area.sets[0];
        let right = &area.sets[1];
        let Some(left_circle) = circles.get(left) else {
            continue;
        };
        let Some(right_circle) = circles.get(right) else {
            continue;
        };
        let mut weight = area.weight.unwrap_or(1.0);
        if area.size + SMALL
            >= (left_circle.radius * left_circle.radius * PI)
                .min(right_circle.radius * right_circle.radius * PI)
        {
            weight = 0.0;
        }
        if let Some(overlaps) = set_overlaps.get_mut(left) {
            overlaps.push(SetOverlap {
                set: right.clone(),
                size: area.size,
                weight,
            });
        }
        if let Some(overlaps) = set_overlaps.get_mut(right) {
            overlaps.push(SetOverlap {
                set: left.clone(),
                size: area.size,
                weight,
            });
        }
    }

    let mut most_overlapped: Vec<MostOverlapped> = set_overlaps
        .iter()
        .map(|(set, overlaps)| MostOverlapped {
            set: set.clone(),
            size: overlaps.iter().map(|o| o.size * o.weight).sum(),
        })
        .collect();
    most_overlapped.sort_by(|a, b| b.size.total_cmp(&a.size));

    let mut positioned = HashSet::new();
    let first = most_overlapped[0].set.clone();
    position_set(
        &mut circles,
        &mut positioned,
        &first,
        VennPoint { x: 0.0, y: 0.0 },
    );

    for item in most_overlapped.iter().skip(1) {
        let set_index = &item.set;
        let mut overlap: Vec<SetOverlap> = set_overlaps
            .get(set_index)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|o| positioned.contains(&o.set))
            .collect();
        overlap.sort_by(|a, b| b.size.total_cmp(&a.size));
        if overlap.is_empty() {
            return Err(VennLayoutError::MissingPairwiseOverlap(set_index.clone()));
        }

        let set = circles[set_index].clone();
        let mut points = Vec::new();
        for j in 0..overlap.len() {
            let p1 = &circles[&overlap[j].set];
            let d1 = distance_from_intersect_area(set.radius, p1.radius, overlap[j].size);
            points.push(VennPoint {
                x: p1.x + d1,
                y: p1.y,
            });
            points.push(VennPoint {
                x: p1.x - d1,
                y: p1.y,
            });
            points.push(VennPoint {
                x: p1.x,
                y: p1.y + d1,
            });
            points.push(VennPoint {
                x: p1.x,
                y: p1.y - d1,
            });

            for k in j + 1..overlap.len() {
                let p2 = &circles[&overlap[k].set];
                let d2 = distance_from_intersect_area(set.radius, p2.radius, overlap[k].size);
                points.extend(circle_circle_intersection(
                    &VennCircle {
                        set: String::new(),
                        x: p1.x,
                        y: p1.y,
                        radius: d1,
                    },
                    &VennCircle {
                        set: String::new(),
                        x: p2.x,
                        y: p2.y,
                        radius: d2,
                    },
                ));
            }
        }

        let mut best_loss = 1e50;
        let mut best_point = points[0];
        for point in points {
            if let Some(circle) = circles.get_mut(set_index) {
                circle.x = point.x;
                circle.y = point.y;
            }
            let local_loss = loss_function(&circles, areas);
            if local_loss < best_loss {
                best_loss = local_loss;
                best_point = point;
            }
        }

        position_set(&mut circles, &mut positioned, set_index, best_point);
    }

    Ok(circles)
}

#[derive(Debug, Clone)]
struct SetOverlap {
    set: String,
    size: f64,
    weight: f64,
}

#[derive(Debug, Clone)]
struct MostOverlapped {
    set: String,
    size: f64,
}

fn position_set(
    circles: &mut Solution,
    positioned: &mut HashSet<String>,
    set: &str,
    point: VennPoint,
) {
    if let Some(circle) = circles.get_mut(set) {
        circle.x = point.x;
        circle.y = point.y;
    }
    positioned.insert(set.to_string());
}

fn constrained_mds_layout(areas: &[VennArea], options: &VennLayoutOptions) -> VennResult<Solution> {
    let mut sets = Vec::new();
    let mut setids = HashMap::new();
    for area in areas {
        if area.sets.len() == 1 {
            setids.insert(area.sets[0].clone(), sets.len());
            sets.push(area.clone());
        }
    }
    if sets.is_empty() {
        return Err(VennLayoutError::EmptySingleSetAreas);
    }

    let (mut distances, constraints) = get_distance_matrices(areas, &sets, &setids);
    let norm =
        norm2(&distances.iter().map(|row| norm2(row)).collect::<Vec<_>>()) / distances.len() as f64;
    if !norm.is_finite() || norm == 0.0 {
        return greedy_layout(areas);
    }
    for row in &mut distances {
        for value in row {
            *value /= norm;
        }
    }

    let mut rng = XorShift64Star::new(options.random_seed);
    let mut best: Option<CgState> = None;
    for _ in 0..options.restarts {
        let initial: Vec<f64> = (0..distances.len() * 2)
            .map(|_| rng.next_f64_unit())
            .collect();
        let current = conjugate_gradient(
            |x, fxprime| constrained_mds_gradient(x, fxprime, &distances, &constraints),
            &initial,
            Some(options.max_iterations),
        );
        if best.as_ref().is_none_or(|b| current.fx < b.fx) {
            best = Some(current);
        }
    }

    let positions = best
        .map(|b| b.x)
        .unwrap_or_else(|| vec![0.0; distances.len() * 2]);
    let mut circles = Solution::new();
    for (i, set) in sets.iter().enumerate() {
        let setid = set.sets[0].clone();
        circles.insert(
            setid.clone(),
            VennCircle {
                set: setid,
                x: positions[2 * i] * norm,
                y: positions[2 * i + 1] * norm,
                radius: (set.size / PI).sqrt(),
            },
        );
    }
    Ok(circles)
}

fn get_distance_matrices(
    areas: &[VennArea],
    sets: &[VennArea],
    setids: &HashMap<String, usize>,
) -> (Vec<Vec<f64>>, Vec<Vec<i8>>) {
    let mut distances = vec![vec![0.0; sets.len()]; sets.len()];
    let mut constraints = vec![vec![0; sets.len()]; sets.len()];

    for current in areas.iter().filter(|a| a.sets.len() == 2) {
        let Some(&left) = setids.get(&current.sets[0]) else {
            continue;
        };
        let Some(&right) = setids.get(&current.sets[1]) else {
            continue;
        };
        let r1 = (sets[left].size / PI).sqrt();
        let r2 = (sets[right].size / PI).sqrt();
        let distance = distance_from_intersect_area(r1, r2, current.size);
        distances[left][right] = distance;
        distances[right][left] = distance;

        let mut constraint = 0;
        if current.size + SMALL >= sets[left].size.min(sets[right].size) {
            constraint = 1;
        } else if current.size <= SMALL {
            constraint = -1;
        }
        constraints[left][right] = constraint;
        constraints[right][left] = constraint;
    }

    (distances, constraints)
}

fn constrained_mds_gradient(
    x: &[f64],
    fxprime: &mut [f64],
    distances: &[Vec<f64>],
    constraints: &[Vec<i8>],
) -> f64 {
    fxprime.fill(0.0);
    let mut loss = 0.0;
    for i in 0..distances.len() {
        let xi = x[2 * i];
        let yi = x[2 * i + 1];
        for j in i + 1..distances.len() {
            let xj = x[2 * j];
            let yj = x[2 * j + 1];
            let dij = distances[i][j];
            let constraint = constraints[i][j];
            let squared_distance = (xj - xi).powi(2) + (yj - yi).powi(2);
            let distance = squared_distance.sqrt();
            let delta = squared_distance - dij * dij;

            if (constraint > 0 && distance <= dij) || (constraint < 0 && distance >= dij) {
                continue;
            }

            loss += 2.0 * delta * delta;
            fxprime[2 * i] += 4.0 * delta * (xi - xj);
            fxprime[2 * i + 1] += 4.0 * delta * (yi - yj);
            fxprime[2 * j] += 4.0 * delta * (xj - xi);
            fxprime[2 * j + 1] += 4.0 * delta * (yj - yi);
        }
    }
    loss
}

pub fn loss_function(circles: &VennSolution, overlaps: &[VennArea]) -> f64 {
    let mut output = 0.0;
    for area in overlaps {
        if area.sets.len() == 1 {
            continue;
        }
        let overlap = if area.sets.len() == 2 {
            let Some(left) = circles.get(&area.sets[0]) else {
                continue;
            };
            let Some(right) = circles.get(&area.sets[1]) else {
                continue;
            };
            circle_overlap(left.radius, right.radius, distance(left, right))
        } else {
            let area_circles: Vec<VennCircle> = area
                .sets
                .iter()
                .filter_map(|set| circles.get(set).cloned())
                .collect();
            intersection_area(&area_circles)
        };
        let weight = area.weight.unwrap_or(1.0);
        output += weight * (overlap - area.size) * (overlap - area.size);
    }
    output
}

pub fn normalize_solution(solution: &VennSolution, orientation: f64) -> VennSolution {
    let circles: Vec<VennCircle> = solution.values().cloned().collect();
    let mut clusters: Vec<Cluster> = disjoint_cluster(circles)
        .into_iter()
        .map(|mut circles| {
            orientate_circles(&mut circles, orientation);
            let bounds = get_bounding_box(&circles);
            let size = (bounds.x_max - bounds.x_min) * (bounds.y_max - bounds.y_min);
            Cluster {
                circles,
                bounds,
                size,
            }
        })
        .collect();

    if clusters.is_empty() {
        return Solution::new();
    }
    clusters.sort_by(|a, b| b.size.total_cmp(&a.size));
    let mut circles = clusters[0].circles.clone();
    let mut return_bounds = clusters[0].bounds;
    let spacing = (return_bounds.x_max - return_bounds.x_min) / 50.0;

    let mut index = 1;
    while index < clusters.len() {
        add_cluster(
            clusters.get(index),
            true,
            false,
            &mut circles,
            &return_bounds,
            spacing,
        );
        add_cluster(
            clusters.get(index + 1),
            false,
            true,
            &mut circles,
            &return_bounds,
            spacing,
        );
        add_cluster(
            clusters.get(index + 2),
            true,
            true,
            &mut circles,
            &return_bounds,
            spacing,
        );
        index += 3;
        return_bounds = get_bounding_box(&circles);
    }

    circles_to_solution(circles)
}

#[derive(Debug, Clone)]
struct Cluster {
    circles: Vec<VennCircle>,
    bounds: Bounds,
    size: f64,
}

fn add_cluster(
    cluster: Option<&Cluster>,
    right: bool,
    bottom: bool,
    circles: &mut Vec<VennCircle>,
    return_bounds: &Bounds,
    spacing: f64,
) {
    let Some(cluster) = cluster else {
        return;
    };
    let bounds = cluster.bounds;
    let mut x_offset = if right {
        return_bounds.x_max - bounds.x_min + spacing
    } else {
        let mut offset = return_bounds.x_max - bounds.x_max;
        let centering =
            (bounds.x_max - bounds.x_min) / 2.0 - (return_bounds.x_max - return_bounds.x_min) / 2.0;
        if centering < 0.0 {
            offset += centering;
        }
        offset
    };
    let mut y_offset = if bottom {
        return_bounds.y_max - bounds.y_min + spacing
    } else {
        let mut offset = return_bounds.y_max - bounds.y_max;
        let centering =
            (bounds.y_max - bounds.y_min) / 2.0 - (return_bounds.y_max - return_bounds.y_min) / 2.0;
        if centering < 0.0 {
            offset += centering;
        }
        offset
    };
    if !x_offset.is_finite() {
        x_offset = 0.0;
    }
    if !y_offset.is_finite() {
        y_offset = 0.0;
    }
    for circle in &cluster.circles {
        let mut c = circle.clone();
        c.x += x_offset;
        c.y += y_offset;
        circles.push(c);
    }
}

fn orientate_circles(circles: &mut [VennCircle], orientation: f64) {
    circles.sort_by(|a, b| b.radius.total_cmp(&a.radius));
    if let Some(largest) = circles.first().cloned() {
        for circle in circles.iter_mut() {
            circle.x -= largest.x;
            circle.y -= largest.y;
        }
    }

    if circles.len() == 2 {
        let dist = distance(&circles[0], &circles[1]);
        if dist < (circles[1].radius - circles[0].radius).abs() {
            circles[1].x = circles[0].x + circles[0].radius - circles[1].radius - SMALL;
            circles[1].y = circles[0].y;
        }
    }

    if circles.len() > 1 {
        let rotation = circles[1].x.atan2(circles[1].y) - orientation;
        let c = rotation.cos();
        let s = rotation.sin();
        for circle in circles.iter_mut() {
            let x = circle.x;
            let y = circle.y;
            circle.x = c * x - s * y;
            circle.y = s * x + c * y;
        }
    }

    if circles.len() > 2 {
        let mut angle = circles[2].x.atan2(circles[2].y) - orientation;
        while angle < 0.0 {
            angle += TAU;
        }
        while angle > TAU {
            angle -= TAU;
        }
        if angle > PI {
            let slope = circles[1].y / (SMALL + circles[1].x);
            for circle in circles.iter_mut() {
                let d = (circle.x + slope * circle.y) / (1.0 + slope * slope);
                circle.x = 2.0 * d - circle.x;
                circle.y = 2.0 * d * slope - circle.y;
            }
        }
    }
}

pub fn disjoint_cluster(circles: Vec<VennCircle>) -> Vec<Vec<VennCircle>> {
    let n = circles.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for i in 0..n {
        for j in i + 1..n {
            if distance(&circles[i], &circles[j]) + SMALL < circles[i].radius + circles[j].radius {
                let x_root = find(&mut parent, j);
                let y_root = find(&mut parent, i);
                parent[x_root] = y_root;
            }
        }
    }

    let mut order = Vec::new();
    let mut grouped: IndexMap<usize, Vec<VennCircle>> = IndexMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        if !grouped.contains_key(&root) {
            order.push(root);
        }
        grouped.entry(root).or_default().push(circles[i].clone());
    }
    order
        .into_iter()
        .filter_map(|root| grouped.shift_remove(&root))
        .collect()
}

pub fn scale_solution(
    solution: &VennSolution,
    width: f64,
    height: f64,
    padding: f64,
    scale_to_fit: Option<f64>,
) -> VennSolution {
    let circles: Vec<VennCircle> = solution.values().cloned().collect();
    let width = width - 2.0 * padding;
    let height = height - 2.0 * padding;
    let bounds = get_bounding_box(&circles);
    if bounds.x_max == bounds.x_min || bounds.y_max == bounds.y_min {
        return solution.clone();
    }

    let (x_scaling, y_scaling) = if let Some(scale_to_fit) = scale_to_fit {
        let to_scale_diameter = (scale_to_fit / PI).sqrt() * 2.0;
        (width / to_scale_diameter, height / to_scale_diameter)
    } else {
        (
            width / (bounds.x_max - bounds.x_min),
            height / (bounds.y_max - bounds.y_min),
        )
    };
    let scaling = y_scaling.min(x_scaling);
    let x_offset = (width - (bounds.x_max - bounds.x_min) * scaling) / 2.0;
    let y_offset = (height - (bounds.y_max - bounds.y_min) * scaling) / 2.0;

    circles_to_solution(
        circles
            .into_iter()
            .map(|circle| VennCircle {
                set: circle.set,
                radius: scaling * circle.radius,
                x: padding + x_offset + (circle.x - bounds.x_min) * scaling,
                y: padding + y_offset + (circle.y - bounds.y_min) * scaling,
            })
            .collect(),
    )
}

fn get_bounding_box(circles: &[VennCircle]) -> Bounds {
    let mut bounds = Bounds {
        x_min: f64::INFINITY,
        x_max: f64::NEG_INFINITY,
        y_min: f64::INFINITY,
        y_max: f64::NEG_INFINITY,
    };
    for circle in circles {
        bounds.x_min = bounds.x_min.min(circle.x - circle.radius);
        bounds.x_max = bounds.x_max.max(circle.x + circle.radius);
        bounds.y_min = bounds.y_min.min(circle.y - circle.radius);
        bounds.y_max = bounds.y_max.max(circle.y + circle.radius);
    }
    bounds
}

fn circles_to_solution(circles: Vec<VennCircle>) -> Solution {
    let mut solution = Solution::new();
    for circle in circles {
        solution.insert(circle.set.clone(), circle);
    }
    solution
}

pub fn intersection_area(circles: &[VennCircle]) -> f64 {
    intersection_area_stats(circles).area
}

fn intersection_area_stats(circles: &[VennCircle]) -> AreaStats {
    if circles.is_empty() {
        return AreaStats {
            area: 0.0,
            arcs: Vec::new(),
        };
    }

    let intersection_points = get_intersection_points(circles);
    let mut inner_points: Vec<IntersectionPoint> = intersection_points
        .into_iter()
        .filter(|p| contained_in_circles(p.point, circles))
        .collect();

    let mut arc_area = 0.0;
    let mut polygon_area = 0.0;
    let mut arcs = Vec::new();

    if inner_points.len() > 1 {
        let center = get_center(inner_points.iter().map(|p| p.point));
        for point in &mut inner_points {
            point.angle = (point.point.x - center.x).atan2(point.point.y - center.y);
        }
        inner_points.sort_by(|a, b| b.angle.total_cmp(&a.angle));

        let mut p2 = inner_points[inner_points.len() - 1].clone();
        for p1 in &inner_points {
            polygon_area += (p2.point.x + p1.point.x) * (p1.point.y - p2.point.y);
            let mid_point = VennPoint {
                x: (p1.point.x + p2.point.x) / 2.0,
                y: (p1.point.y + p2.point.y) / 2.0,
            };
            let mut arc: Option<VennArc> = None;

            for parent in p1.parent_index {
                if !p2.parent_index.contains(&parent) {
                    continue;
                }
                let circle = &circles[parent];
                let a1 = (p1.point.x - circle.x).atan2(p1.point.y - circle.y);
                let a2 = (p2.point.x - circle.x).atan2(p2.point.y - circle.y);
                let mut angle_diff = a2 - a1;
                if angle_diff < 0.0 {
                    angle_diff += TAU;
                }
                let a = a2 - angle_diff / 2.0;
                let mut width = distance_points(
                    mid_point,
                    VennPoint {
                        x: circle.x + circle.radius * a.sin(),
                        y: circle.y + circle.radius * a.cos(),
                    },
                );
                if width > circle.radius * 2.0 {
                    width = circle.radius * 2.0;
                }
                if arc.as_ref().is_none_or(|current| current.width > width) {
                    arc = Some(VennArc {
                        circle: circle.clone(),
                        width,
                        p1: p1.point,
                        p2: p2.point,
                        large: width > circle.radius,
                        sweep: true,
                    });
                }
            }

            if let Some(arc) = arc {
                arc_area += circle_area(arc.circle.radius, arc.width);
                arcs.push(arc);
                p2 = p1.clone();
            }
        }
    } else {
        let mut smallest = &circles[0];
        for circle in circles.iter().skip(1) {
            if circle.radius < smallest.radius {
                smallest = circle;
            }
        }

        let mut disjoint = false;
        for circle in circles {
            if distance(circle, smallest) > (smallest.radius - circle.radius).abs() {
                disjoint = true;
                break;
            }
        }

        if disjoint {
            arc_area = 0.0;
            polygon_area = 0.0;
        } else {
            arc_area = smallest.radius * smallest.radius * PI;
            arcs.push(VennArc {
                circle: smallest.clone(),
                p1: VennPoint {
                    x: smallest.x,
                    y: smallest.y + smallest.radius,
                },
                p2: VennPoint {
                    x: smallest.x - SMALL,
                    y: smallest.y + smallest.radius,
                },
                width: smallest.radius * 2.0,
                large: true,
                sweep: true,
            });
        }
    }

    polygon_area /= 2.0;
    AreaStats {
        area: arc_area + polygon_area,
        arcs,
    }
}

fn contained_in_circles(point: VennPoint, circles: &[VennCircle]) -> bool {
    circles
        .iter()
        .all(|circle| distance_point_circle(point, circle) < circle.radius + SMALL)
}

fn get_intersection_points(circles: &[VennCircle]) -> Vec<IntersectionPoint> {
    let mut out = Vec::new();
    for i in 0..circles.len() {
        for j in i + 1..circles.len() {
            for point in circle_circle_intersection(&circles[i], &circles[j]) {
                out.push(IntersectionPoint {
                    point,
                    parent_index: [i, j],
                    angle: 0.0,
                });
            }
        }
    }
    out
}

pub fn circle_area(r: f64, width: f64) -> f64 {
    r * r * (1.0 - width / r).acos() - (r - width) * (width * (2.0 * r - width)).sqrt()
}

pub fn distance(p1: &VennCircle, p2: &VennCircle) -> f64 {
    distance_points(
        VennPoint { x: p1.x, y: p1.y },
        VennPoint { x: p2.x, y: p2.y },
    )
}

fn distance_point_circle(point: VennPoint, circle: &VennCircle) -> f64 {
    distance_points(
        point,
        VennPoint {
            x: circle.x,
            y: circle.y,
        },
    )
}

fn distance_points(p1: VennPoint, p2: VennPoint) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn circle_overlap(r1: f64, r2: f64, d: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return PI * r1.min(r2) * r1.min(r2);
    }
    let w1 = r1 - (d * d - r2 * r2 + r1 * r1) / (2.0 * d);
    let w2 = r2 - (d * d - r1 * r1 + r2 * r2) / (2.0 * d);
    circle_area(r1, w1) + circle_area(r2, w2)
}

pub fn circle_circle_intersection(p1: &VennCircle, p2: &VennCircle) -> Vec<VennPoint> {
    let d = distance(p1, p2);
    let r1 = p1.radius;
    let r2 = p2.radius;
    if d >= r1 + r2 || d <= (r1 - r2).abs() {
        return Vec::new();
    }

    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h = (r1 * r1 - a * a).sqrt();
    let x0 = p1.x + (a * (p2.x - p1.x)) / d;
    let y0 = p1.y + (a * (p2.y - p1.y)) / d;
    let rx = -(p2.y - p1.y) * (h / d);
    let ry = -(p2.x - p1.x) * (h / d);

    vec![
        VennPoint {
            x: x0 + rx,
            y: y0 - ry,
        },
        VennPoint {
            x: x0 - rx,
            y: y0 + ry,
        },
    ]
}

fn get_center(points: impl IntoIterator<Item = VennPoint>) -> VennPoint {
    let mut count = 0usize;
    let mut center = VennPoint { x: 0.0, y: 0.0 };
    for point in points {
        center.x += point.x;
        center.y += point.y;
        count += 1;
    }
    if count > 0 {
        center.x /= count as f64;
        center.y /= count as f64;
    }
    center
}

fn compute_text_centres(
    circles: &Solution,
    areas: &[VennArea],
    symmetrical_text_centre: bool,
) -> HashMap<String, VennTextPoint> {
    let overlapped = get_overlapping_circles(circles);
    let mut out = HashMap::new();
    for area in areas {
        let areaids: HashSet<&str> = area.sets.iter().map(String::as_str).collect();
        let mut exclude = HashSet::new();
        for set in &area.sets {
            if let Some(overlaps) = overlapped.get(set) {
                for overlap in overlaps {
                    exclude.insert(overlap.as_str());
                }
            }
        }

        let mut interior = Vec::new();
        let mut exterior = Vec::new();
        for (setid, circle) in circles {
            if areaids.contains(setid.as_str()) {
                interior.push(circle.clone());
            } else if !exclude.contains(setid.as_str()) {
                exterior.push(circle.clone());
            }
        }
        out.insert(
            sets_key(&area.sets),
            compute_text_centre(&interior, &exterior, symmetrical_text_centre),
        );
    }
    out
}

pub fn compute_text_centre(
    interior: &[VennCircle],
    exterior: &[VennCircle],
    symmetrical_text_centre: bool,
) -> VennTextPoint {
    if interior.is_empty() {
        return VennTextPoint {
            x: 0.0,
            y: 0.0,
            disjoint: true,
        };
    }

    let mut points = Vec::new();
    for c in interior {
        points.push(VennPoint { x: c.x, y: c.y });
        points.push(VennPoint {
            x: c.x + c.radius / 2.0,
            y: c.y,
        });
        points.push(VennPoint {
            x: c.x - c.radius / 2.0,
            y: c.y,
        });
        points.push(VennPoint {
            x: c.x,
            y: c.y + c.radius / 2.0,
        });
        points.push(VennPoint {
            x: c.x,
            y: c.y - c.radius / 2.0,
        });
    }

    let mut initial = points[0];
    let mut margin = circle_margin(points[0], interior, exterior);
    for point in points.into_iter().skip(1) {
        let m = circle_margin(point, interior, exterior);
        if m >= margin {
            initial = point;
            margin = m;
        }
    }

    let solution = nelder_mead(
        |p| -circle_margin(VennPoint { x: p[0], y: p[1] }, interior, exterior),
        &[initial.x, initial.y],
        &OptimParams {
            max_iterations: Some(500),
            min_error_delta: Some(1e-10),
        },
    )
    .x;

    let ret = VennTextPoint {
        x: if symmetrical_text_centre {
            0.0
        } else {
            solution[0]
        },
        y: solution[1],
        disjoint: false,
    };

    let valid_interior = interior.iter().all(|circle| {
        distance_point_circle(VennPoint { x: ret.x, y: ret.y }, circle) <= circle.radius
    });
    let valid_exterior = exterior.iter().all(|circle| {
        distance_point_circle(VennPoint { x: ret.x, y: ret.y }, circle) >= circle.radius
    });
    if valid_interior && valid_exterior {
        return ret;
    }

    if interior.len() == 1 {
        return VennTextPoint {
            x: interior[0].x,
            y: interior[0].y,
            disjoint: false,
        };
    }

    let area_stats = intersection_area_stats(interior);
    if area_stats.arcs.is_empty() {
        return VennTextPoint {
            x: 0.0,
            y: -1000.0,
            disjoint: true,
        };
    }
    if area_stats.arcs.len() == 1 {
        return VennTextPoint {
            x: area_stats.arcs[0].circle.x,
            y: area_stats.arcs[0].circle.y,
            disjoint: false,
        };
    }
    if !exterior.is_empty() {
        return compute_text_centre(interior, &[], false);
    }

    let center = get_center(area_stats.arcs.iter().map(|arc| arc.p1));
    VennTextPoint {
        x: center.x,
        y: center.y,
        disjoint: false,
    }
}

fn circle_margin(current: VennPoint, interior: &[VennCircle], exterior: &[VennCircle]) -> f64 {
    let mut margin = interior[0].radius - distance_point_circle(current, &interior[0]);
    for circle in interior.iter().skip(1) {
        margin = margin.min(circle.radius - distance_point_circle(current, circle));
    }
    for circle in exterior {
        margin = margin.min(distance_point_circle(current, circle) - circle.radius);
    }
    margin
}

fn get_overlapping_circles(circles: &Solution) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = circles
        .keys()
        .map(|setid| (setid.clone(), Vec::new()))
        .collect();
    let ids: Vec<String> = circles.keys().cloned().collect();
    for i in 0..ids.len() {
        let ci = &ids[i];
        let a = &circles[ci];
        for cj in ids.iter().skip(i + 1) {
            let b = &circles[cj];
            let d = distance(a, b);
            if d + b.radius <= a.radius + SMALL {
                out.entry(cj.clone()).or_default().push(ci.clone());
            } else if d + a.radius <= b.radius + SMALL {
                out.entry(ci.clone()).or_default().push(cj.clone());
            }
        }
    }
    out
}

fn intersection_area_arcs(circles: &[VennCircle]) -> Vec<VennArc> {
    if circles.is_empty() {
        return Vec::new();
    }
    intersection_area_stats(circles).arcs
}

pub fn intersection_area_path(circles: &[VennCircle], round: Option<usize>) -> String {
    arcs_to_path(&intersection_area_arcs(circles), round)
}

fn arcs_to_path(arcs: &[VennArc], round: Option<usize>) -> String {
    if arcs.is_empty() {
        return "M 0 0".to_string();
    }
    if arcs.len() == 1 {
        let circle = &arcs[0].circle;
        return circle_path(
            round_path_value(circle.x, round),
            round_path_value(circle.y, round),
            round_path_value(circle.radius, round),
        );
    }

    let mut out = String::new();
    out.push_str("\nM ");
    push_js_number(&mut out, round_path_value(arcs[0].p2.x, round));
    out.push(' ');
    push_js_number(&mut out, round_path_value(arcs[0].p2.y, round));
    for arc in arcs {
        out.push_str(" \nA ");
        let radius = round_path_value(arc.circle.radius, round);
        push_js_number(&mut out, radius);
        out.push(' ');
        push_js_number(&mut out, radius);
        out.push_str(" 0 ");
        out.push(if arc.large { '1' } else { '0' });
        out.push(' ');
        out.push(if arc.sweep { '1' } else { '0' });
        out.push(' ');
        push_js_number(&mut out, round_path_value(arc.p1.x, round));
        out.push(' ');
        push_js_number(&mut out, round_path_value(arc.p1.y, round));
    }
    out
}

fn circle_path(x: f64, y: f64, r: f64) -> String {
    let mut out = String::new();
    out.push_str("\nM ");
    push_js_number(&mut out, x);
    out.push(' ');
    push_js_number(&mut out, y);
    out.push_str(" \nm ");
    push_js_number(&mut out, -r);
    out.push_str(" 0 \na ");
    push_js_number(&mut out, r);
    out.push(' ');
    push_js_number(&mut out, r);
    out.push_str(" 0 1 0 ");
    push_js_number(&mut out, r * 2.0);
    out.push_str(" 0 \na ");
    push_js_number(&mut out, r);
    out.push(' ');
    push_js_number(&mut out, r);
    out.push_str(" 0 1 0 ");
    push_js_number(&mut out, -r * 2.0);
    out.push_str(" 0");
    out
}

fn round_path_value(v: f64, round: Option<usize>) -> f64 {
    let Some(round) = round else {
        return v;
    };
    let factor = 10_f64.powi(round as i32);
    let out = ((v * factor) + 0.5).floor() / factor;
    if out == -0.0 { 0.0 } else { out }
}

fn push_js_number(out: &mut String, mut v: f64) {
    if !v.is_finite() {
        out.push('0');
        return;
    }
    if v == -0.0 {
        v = 0.0;
    }
    let mut buf = Buffer::new();
    out.push_str(buf.format_finite(v));
}

fn sets_key(sets: &[String]) -> String {
    sets.join(",")
}

fn bisect<F>(mut f: F, mut a: f64, b: f64, max_iterations: usize, tolerance: f64) -> f64
where
    F: FnMut(f64) -> f64,
{
    let f_a = f(a);
    let f_b = f(b);
    let mut delta = b - a;
    if f_a * f_b > 0.0 {
        return a;
    }
    if f_a == 0.0 {
        return a;
    }
    if f_b == 0.0 {
        return b;
    }
    for _ in 0..max_iterations {
        delta /= 2.0;
        let mid = a + delta;
        let f_mid = f(mid);
        if f_mid * f_a >= 0.0 {
            a = mid;
        }
        if delta.abs() < tolerance || f_mid == 0.0 {
            return mid;
        }
    }
    a + delta
}

fn nelder_mead<F>(mut f: F, x0: &[f64], parameters: &OptimParams) -> OptimResult
where
    F: FnMut(&[f64]) -> f64,
{
    let n = x0.len();
    let max_iterations = parameters.max_iterations.unwrap_or(n * 200);
    let non_zero_delta = 1.05;
    let zero_delta = 0.001;
    let min_error_delta = parameters.min_error_delta.unwrap_or(1e-6);
    let min_tolerance = parameters.min_error_delta.unwrap_or(1e-5);
    let rho = 1.0;
    let chi = 2.0;
    let psi = -0.5;
    let sigma = 0.5;

    let mut simplex = Vec::with_capacity(n + 1);
    simplex.push(SimplexPoint {
        x: x0.to_vec(),
        fx: f(x0),
    });
    for i in 0..n {
        let mut point = x0.to_vec();
        point[i] = if point[i] != 0.0 {
            point[i] * non_zero_delta
        } else {
            zero_delta
        };
        let fx = f(&point);
        simplex.push(SimplexPoint { x: point, fx });
    }

    let mut centroid = x0.to_vec();
    let mut reflected = x0.to_vec();
    let mut contracted = x0.to_vec();
    let mut expanded = x0.to_vec();

    for _ in 0..max_iterations {
        simplex.sort_by(|a, b| a.fx.total_cmp(&b.fx));
        let mut max_diff: f64 = 0.0;
        if simplex.len() > 1 {
            for i in 0..n {
                max_diff = max_diff.max((simplex[0].x[i] - simplex[1].x[i]).abs());
            }
        }
        if (simplex[0].fx - simplex[n].fx).abs() < min_error_delta && max_diff < min_tolerance {
            break;
        }

        for i in 0..n {
            centroid[i] = 0.0;
            for point in simplex.iter().take(n) {
                centroid[i] += point.x[i];
            }
            centroid[i] /= n as f64;
        }

        let worst = simplex[n].x.clone();
        weighted_sum(&mut reflected, 1.0 + rho, &centroid, -rho, &worst);
        let reflected_fx = f(&reflected);
        if reflected_fx < simplex[0].fx {
            weighted_sum(&mut expanded, 1.0 + chi, &centroid, -chi, &worst);
            let expanded_fx = f(&expanded);
            if expanded_fx < reflected_fx {
                update_simplex(&mut simplex[n], &expanded, expanded_fx);
            } else {
                update_simplex(&mut simplex[n], &reflected, reflected_fx);
            }
        } else if reflected_fx >= simplex[n - 1].fx {
            let mut should_reduce = false;
            if reflected_fx > simplex[n].fx {
                weighted_sum(&mut contracted, 1.0 + psi, &centroid, -psi, &worst);
                let contracted_fx = f(&contracted);
                if contracted_fx < simplex[n].fx {
                    update_simplex(&mut simplex[n], &contracted, contracted_fx);
                } else {
                    should_reduce = true;
                }
            } else {
                weighted_sum(
                    &mut contracted,
                    1.0 - psi * rho,
                    &centroid,
                    psi * rho,
                    &worst,
                );
                let contracted_fx = f(&contracted);
                if contracted_fx < reflected_fx {
                    update_simplex(&mut simplex[n], &contracted, contracted_fx);
                } else {
                    should_reduce = true;
                }
            }

            if should_reduce {
                if sigma >= 1.0 {
                    break;
                }
                let best = simplex[0].x.clone();
                for point in simplex.iter_mut().skip(1) {
                    let current = point.x.clone();
                    weighted_sum(&mut point.x, 1.0 - sigma, &best, sigma, &current);
                    point.fx = f(&point.x);
                }
            }
        } else {
            update_simplex(&mut simplex[n], &reflected, reflected_fx);
        }
    }

    simplex.sort_by(|a, b| a.fx.total_cmp(&b.fx));
    OptimResult {
        x: simplex[0].x.clone(),
    }
}

#[derive(Debug, Clone)]
struct SimplexPoint {
    x: Vec<f64>,
    fx: f64,
}

fn update_simplex(point: &mut SimplexPoint, value: &[f64], fx: f64) {
    point.x.copy_from_slice(value);
    point.fx = fx;
}

fn conjugate_gradient<F>(mut f: F, initial: &[f64], max_iterations: Option<usize>) -> CgState
where
    F: FnMut(&[f64], &mut [f64]) -> f64,
{
    let mut current = CgState {
        x: initial.to_vec(),
        fx: 0.0,
        fxprime: initial.to_vec(),
    };
    let mut next = current.clone();
    let mut yk = initial.to_vec();
    let mut pk = current.fxprime.clone();
    let mut a = 1.0;
    let max_iterations = max_iterations.unwrap_or(initial.len() * 20);

    current.fx = f(&current.x, &mut current.fxprime);
    scale(&mut pk, &current.fxprime, -1.0);

    for _ in 0..max_iterations {
        a = wolfe_line_search(&mut f, &pk, &current, &mut next, a);
        if a == 0.0 {
            scale(&mut pk, &current.fxprime, -1.0);
        } else {
            weighted_sum(&mut yk, 1.0, &next.fxprime, -1.0, &current.fxprime);
            let delta_k = dot(&current.fxprime, &current.fxprime);
            let beta_k = (dot(&yk, &next.fxprime) / delta_k).max(0.0);
            let old_pk = pk.clone();
            weighted_sum(&mut pk, beta_k, &old_pk, -1.0, &next.fxprime);
            std::mem::swap(&mut current, &mut next);
        }

        if norm2(&current.fxprime) <= 1e-5 {
            break;
        }
    }

    current
}

fn wolfe_line_search<F>(
    f: &mut F,
    pk: &[f64],
    current: &CgState,
    next: &mut CgState,
    mut a: f64,
) -> f64
where
    F: FnMut(&[f64], &mut [f64]) -> f64,
{
    let phi0 = current.fx;
    let phi_prime0 = dot(&current.fxprime, pk);
    let mut phi_old = phi0;
    let mut a0 = 0.0;
    let c1 = 1e-6;
    let c2 = 0.1;
    if a == 0.0 {
        a = 1.0;
    }

    for iteration in 0..10 {
        weighted_sum(&mut next.x, 1.0, &current.x, a, pk);
        next.fx = f(&next.x, &mut next.fxprime);
        let phi = next.fx;
        let phi_prime = dot(&next.fxprime, pk);
        if phi > phi0 + c1 * a * phi_prime0 || (iteration > 0 && phi >= phi_old) {
            return zoom(
                f, pk, current, next, a0, a, phi_old, phi0, phi_prime0, c1, c2,
            );
        }
        if phi_prime.abs() <= -c2 * phi_prime0 {
            return a;
        }
        if phi_prime >= 0.0 {
            return zoom(f, pk, current, next, a, a0, phi, phi0, phi_prime0, c1, c2);
        }
        phi_old = phi;
        a0 = a;
        a *= 2.0;
    }
    a
}

#[allow(clippy::too_many_arguments)]
fn zoom<F>(
    f: &mut F,
    pk: &[f64],
    current: &CgState,
    next: &mut CgState,
    mut a_lo: f64,
    mut a_high: f64,
    mut phi_lo: f64,
    phi0: f64,
    phi_prime0: f64,
    c1: f64,
    c2: f64,
) -> f64
where
    F: FnMut(&[f64], &mut [f64]) -> f64,
{
    for _ in 0..16 {
        let a = (a_lo + a_high) / 2.0;
        weighted_sum(&mut next.x, 1.0, &current.x, a, pk);
        next.fx = f(&next.x, &mut next.fxprime);
        let phi = next.fx;
        let phi_prime = dot(&next.fxprime, pk);
        if phi > phi0 + c1 * a * phi_prime0 || phi >= phi_lo {
            a_high = a;
        } else {
            if phi_prime.abs() <= -c2 * phi_prime0 {
                return a;
            }
            if phi_prime * (a_high - a_lo) >= 0.0 {
                a_high = a_lo;
            }
            a_lo = a;
            phi_lo = phi;
        }
    }
    0.0
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(a, b)| a * b).sum()
}

fn norm2(a: &[f64]) -> f64 {
    dot(a, a).sqrt()
}

fn scale(ret: &mut [f64], value: &[f64], c: f64) {
    for (out, value) in ret.iter_mut().zip(value) {
        *out = value * c;
    }
}

fn weighted_sum(ret: &mut [f64], w1: f64, v1: &[f64], w2: f64, v2: &[f64]) {
    for ((out, v1), v2) in ret.iter_mut().zip(v1).zip(v2) {
        *out = w1 * v1 + w2 * v2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(set: &str, x: f64, y: f64, radius: f64) -> VennCircle {
        VennCircle {
            set: set.to_string(),
            x,
            y,
            radius,
        }
    }

    fn area(sets: &[&str], size: f64) -> VennArea {
        VennArea::new(sets.iter().copied(), size)
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {actual} to be within {tolerance} of {expected}"
        );
    }

    #[test]
    fn circle_area_matches_upstream_cases() {
        assert_close(circle_area(10.0, 0.0), 0.0, 1e-9);
        assert_close(circle_area(10.0, 10.0), (PI * 10.0 * 10.0) / 2.0, 1e-9);
        assert_close(circle_area(10.0, 20.0), PI * 10.0 * 10.0, 1e-9);
    }

    #[test]
    fn circle_overlap_matches_upstream_cases() {
        assert_close(circle_overlap(10.0, 10.0, 200.0), 0.0, 1e-9);
        assert_close(circle_overlap(10.0, 10.0, 0.0), PI * 10.0 * 10.0, 1e-9);
        assert_close(circle_overlap(10.0, 5.0, 5.0), PI * 5.0 * 5.0, 1e-9);
    }

    #[test]
    fn circle_circle_intersection_matches_upstream_cases() {
        assert!(
            circle_circle_intersection(&c("a", 0.0, 3.0, 10.0), &c("b", 3.0, 0.0, 20.0)).is_empty()
        );
        assert!(
            circle_circle_intersection(&c("a", 0.0, 0.0, 10.0), &c("b", 21.0, 0.0, 10.0))
                .is_empty()
        );

        let points = circle_circle_intersection(&c("a", 0.0, 0.0, 10.0), &c("b", 10.0, 0.0, 10.0));
        assert_eq!(points.len(), 2);
        assert_close(points[0].x, 5.0, 1e-9);
        assert_close(points[1].x, 5.0, 1e-9);
        assert_close(points[0].y, -points[1].y, 1e-9);

        let points = circle_circle_intersection(&c("a", 15.0, 5.0, 10.0), &c("b", 20.0, 0.0, 10.0));
        assert_eq!(points.len(), 2);
    }

    #[test]
    fn intersection_area_matches_upstream_regressions() {
        let circles = vec![
            c("0", 0.909, 0.905, 0.548),
            c("1", 0.765, 0.382, 0.703),
            c("2", 0.63, 0.019, 0.449),
            c("3", 0.21, 0.755, 0.656),
            c("4", 0.276, 0.723, 1.145),
            c("5", 0.141, 0.585, 0.419),
        ];
        assert_eq!(intersection_area(&circles), 0.0);

        let circles = vec![
            c("0", 0.426, 0.882, 0.944),
            c("1", 0.24, 0.685, 0.992),
            c("2", 0.01, 0.909, 1.161),
            c("3", 0.54, 0.475, 0.41),
        ];
        assert_close(intersection_area(&circles), 0.41 * 0.41 * PI, 1e-9);

        let circles = vec![
            c("0", 0.501, 0.32, 0.629),
            c("1", 0.945, 0.022, 1.015),
            c("2", 0.021, 0.863, 0.261),
            c("3", 0.528, 0.09, 0.676),
        ];
        assert_close(intersection_area(&circles), 0.0008914, 0.0001);

        let circles = vec![
            c("0", 9.154829758385864, 0.0, 8.481629223064205),
            c(
                "1",
                5.806079662851866,
                7.4438023223126795,
                15.274853405932202,
            ),
            c(
                "2",
                9.484491297623553,
                4.064806303558571,
                10.280023453913834,
            ),
            c(
                "3",
                10.56492833796709,
                3.0723147554880175,
                8.812923024107548,
            ),
        ];
        assert_close(intersection_area(&circles), 10.96362, 1e-5);

        let circles = vec![
            c(
                "0",
                -0.0014183481763938425,
                0.0006071174738860746,
                510.3115834996166,
            ),
            c(
                "1",
                875.0163281608848,
                0.0007003612396158774,
                465.1793581792228,
            ),
            c(
                "2",
                462.7394999567192,
                387.9359963330729,
                172.62633992134658,
            ),
        ];
        assert!(!intersection_area(&circles).is_nan());
    }

    #[test]
    fn greedy_layout_matches_upstream_zero_loss_cases() {
        let cases = vec![
            vec![
                area(&["0"], 0.7746543297103429),
                area(&["1"], 0.1311252856844238),
                area(&["2"], 0.2659942131443344),
                area(&["3"], 0.44600866168641723),
                area(&["0", "1"], 0.02051532092950205),
                area(&["0", "2"], 0.0),
                area(&["0", "3"], 0.0),
                area(&["1", "2"], 0.0),
                area(&["1", "3"], 0.07597023820511245),
                area(&["2", "3"], 0.0),
            ],
            vec![
                area(&["0"], 0.5299368855059736),
                area(&["1"], 0.03364187025606481),
                area(&["2"], 0.3121450394871512),
                area(&["3"], 0.0514397361783036),
                area(&["0", "1"], 0.013912447645582351),
                area(&["0", "2"], 0.005903647141469598),
                area(&["0", "3"], 0.0514397361783036),
                area(&["1", "2"], 0.012138157839477597),
                area(&["1", "3"], 0.008010688232481479),
                area(&["2", "3"], 0.0),
            ],
            vec![
                area(&["0"], 1.7288584050841396),
                area(&["1"], 0.040875831658950056),
                area(&["2"], 2.587146019782323),
                area(&["0", "1"], 0.040875831658950056),
                area(&["0", "2"], 0.5114617575187569),
                area(&["1", "2"], 0.040875831658950056),
            ],
        ];

        for areas in cases {
            let circles = greedy_layout(&areas).expect("greedy layout");
            assert_close(loss_function(&circles, &areas), 0.0, 1e-8);
        }
    }

    #[test]
    fn distance_from_intersect_area_round_trips_overlap() {
        for (r1, r2, overlap) in [
            (1.9544100476116797, 2.256758334191025, 11.0),
            (111.06512962798197, 113.32348546565727, 1218.0),
            (44.456564007075, 149.4335753619362, 2799.0),
            (592.89, 134.75, 56995.0),
            (139.50778247443944, 32.892784970851956, 3399.0),
            (4.886025119029199, 5.077706251929807, 75.0),
        ] {
            let d = distance_from_intersect_area(r1, r2, overlap);
            assert_close(circle_overlap(r1, r2, d), overlap, 1e-7);
        }
    }

    #[test]
    fn normalize_solution_places_disjoint_circles_close_together() {
        let mut solution = Solution::new();
        solution.insert("0".to_string(), c("0", 0.0, 0.0, 0.5));
        solution.insert("1".to_string(), c("1", 1e10, 0.0, 1.5));

        let normalized = normalize_solution(&solution, PI / 2.0);
        assert!(distance(&normalized["0"], &normalized["1"]) < 2.1);
    }

    #[test]
    fn disjoint_clusters_match_upstream_case() {
        let input = vec![
            c(
                "0",
                0.8047033110633492,
                0.9396705999970436,
                0.47156485118903224,
            ),
            c(
                "1",
                0.7961132447235286,
                0.014027722179889679,
                0.14554832570720466,
            ),
            c(
                "2",
                0.28841276094317436,
                0.98081015329808,
                0.9851036085514352,
            ),
            c(
                "3",
                0.7689983483869582,
                0.2899463507346809,
                0.7210563338827342,
            ),
        ];

        assert_eq!(disjoint_cluster(input).len(), 1);
    }

    #[test]
    fn compute_text_centre_matches_upstream_cases() {
        let center = compute_text_centre(&[c("0", 0.0, 0.0, 1.0)], &[], false);
        assert_close(center.x, 0.0, 1e-9);
        assert_close(center.y, 0.0, 1e-9);

        let center = compute_text_centre(&[c("0", 0.0, 0.0, 1.0)], &[c("1", 0.0, 1.0, 1.0)], false);
        assert_close(center.x, 0.0, 1e-4);
        assert_close(center.y, -0.5, 1e-6);
    }

    #[test]
    fn compute_venn_layout_returns_typed_helper_surface() {
        let areas = vec![
            area(&["A"], 10.0),
            area(&["B"], 10.0),
            area(&["A", "B"], 3.0),
        ];
        let layout = compute_venn_layout(&areas, &VennLayoutOptions::default()).expect("layout");
        assert_eq!(layout.len(), 3);
        assert_eq!(layout[0].data.sets, vec!["A"]);
        assert!(!layout[0].path.is_empty());
        assert_eq!(layout[2].circles.len(), 2);
        assert!(!layout[2].text.disjoint);
    }
}
