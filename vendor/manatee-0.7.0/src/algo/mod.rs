pub mod cose_bilkent;
pub mod fcose;

#[derive(Debug, Clone)]
pub enum Algorithm {
    /// Cytoscape COSE-Bilkent (Mermaid mindmap default).
    CoseBilkent(CoseBilkentOptions),
    /// Cytoscape FCoSE (Mermaid architecture layout).
    Fcose(FcoseOptions),
}

#[derive(Debug, Clone, Default)]
pub struct CoseBilkentOptions {
    /// Seed for deterministic randomness. The upstream JS implementation relies on `Math.random`,
    /// so the Rust port will use a reproducible RNG here.
    pub random_seed: u64,
}

#[derive(Debug, Clone)]
pub struct FcoseOptions {
    pub random_seed: u64,
    /// Optional number of seeded random values to consume before the first FCoSE run.
    ///
    /// Use this to mirror upstream render paths that consume `Math.random()` before layout
    /// construction. When unset, the Rust port preserves its historical default offset behavior.
    pub random_seed_offset: Option<usize>,
    /// Mermaid Architecture runs Cytoscape FCoSE twice (`layout.run()` inside `layoutstop`),
    /// which advances the seeded `Math.random()` stream and can change the final coordinates.
    ///
    /// When enabled, the Rust port mimics that behavior by performing two consecutive runs while
    /// keeping the RNG stream continuous between runs.
    pub rerun: bool,
    /// Whether to initialize the layout with FCoSE's spectral/randomized start positions.
    pub randomize: bool,
    /// FCoSE spectral start node separation. Used only when `randomize` is enabled.
    pub node_separation: Option<f64>,
    /// Maximum FCoSE spring-embedder iterations before the layout stops.
    pub num_iter: Option<usize>,
    /// Override for layout-base/CoSE `DEFAULT_EDGE_LENGTH` (used for repulsion/grid range, overlap
    /// separation buffer, and convergence thresholds).
    ///
    /// In upstream Cytoscape FCoSE, `DEFAULT_EDGE_LENGTH` is derived from the `idealEdgeLength`
    /// option (before inter-graph nesting/smart adjustments), then used by layout-base constants
    /// such as `MIN_REPULSION_DIST` and the FR-grid cell size. Keeping this value aligned is
    /// important for parity with Mermaid-generated SVG baselines.
    pub default_edge_length: Option<f64>,
    pub alignment_constraint: Option<AlignmentConstraint>,
    pub relative_placement_constraint: Vec<RelativePlacementConstraint>,
    /// Optional padding applied around compound (group) bounds when computing compound repulsion.
    pub compound_padding: Option<f64>,
    /// Optional override for the "original component center" used by `aux.relocateComponent(...)`.
    ///
    /// In upstream Cytoscape FCoSE, `originalCenter` comes from `eles.boundingBox()` before the
    /// layout runs, and the final layout is translated so the component's bounding box center
    /// matches that pre-layout center.
    ///
    /// When set, the Rust port uses this value instead of the layout-base bounds center.
    pub relocate_center: Option<(f64, f64)>,
}

impl Default for FcoseOptions {
    fn default() -> Self {
        Self {
            random_seed: 0,
            random_seed_offset: None,
            rerun: false,
            randomize: true,
            node_separation: None,
            num_iter: None,
            default_edge_length: None,
            alignment_constraint: None,
            relative_placement_constraint: Vec::new(),
            compound_padding: None,
            relocate_center: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AlignmentConstraint {
    /// Nodes in each inner vec share the same y coordinate (horizontal alignment).
    pub horizontal: Vec<Vec<String>>,
    /// Nodes in each inner vec share the same x coordinate (vertical alignment).
    pub vertical: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct RelativePlacementConstraint {
    pub left: Option<String>,
    pub right: Option<String>,
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub gap: f64,
}
