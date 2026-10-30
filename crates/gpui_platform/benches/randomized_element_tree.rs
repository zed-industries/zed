//! Frame cost of a seeded element tree under one class of change per frame.
//!
//! Each benchmark builds a [`RandomizedElementTree`] before timing, then measures frames in
//! which exactly one thing happens: nothing (a notified root that re-renders an unchanged
//! tree), a leaf's style, a leaf's bounds, the root's layout, one structural change, or a
//! sibling reorder. The same fixture runs on any GPUI revision, so it doubles as the
//! before/after harness for changes to rendering.
//!
//! Trees come from *families*: bounds on topology, element count and entity density that
//! a seeded RNG is drawn against (see [`RandomizedElementTreeBounds`]). Each family is an
//! input and `#[gpui::bench]` runs it once per seed, so an id like `tree/wide/seed-3`
//! names one tree whose whole shape follows from the seed, and a run over a family's
//! seeds is a sample of that family. `SEED` and `ITERATIONS` choose the seeds as for
//! `#[gpui::test]`. Filter by family to ask a narrower question, e.g.
//! `--bench randomized_element_tree -- 'tree/tall/'`. Each tree is described once, with
//! the heap its window holds after the first frame, before it is measured.
//!
//! Benchmarks only measure. The randomized tree's tests check that what these workloads
//! render is correct, so a faster frame here still needs those tests to pass.

use std::{
    cell::RefCell,
    fmt,
    time::{Duration, Instant},
};

use gpui::prelude::*;
use gpui::{
    BenchAppContext, Context, Entity, Window,
    private::rand::{Rng as _, SeedableRng as _, rngs::StdRng},
    randomized_element_tree::{
        ChangeLocality, RandomizedElementTree, RandomizedElementTreeBounds,
        RandomizedElementTreeConfig, RandomizedElementTreeMutationKind,
        RandomizedElementTreeTopology,
    },
};

/// A named family of trees: the bounds one seed's tree is drawn against.
#[derive(Clone)]
struct TreeFamily {
    name: &'static str,
    bounds: RandomizedElementTreeBounds,
}

impl TreeFamily {
    /// Draws this family's tree for the seed `rng` started from. Every family consumes
    /// its RNG the same way, so drawing from `rng` directly would give each family the
    /// same relative draw at a given seed (overlapping families the same tree); mixing in
    /// the family's name gives each family its own stream for the same seed.
    fn sample(&self, rng: &mut StdRng) -> TreeInput {
        // FNV-1a, which unlike `DefaultHasher` is fixed across Rust releases, so a seed
        // names the same tree on every toolchain.
        let name_hash = self
            .name
            .bytes()
            .fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| {
                (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
            });
        let mut family_rng = StdRng::seed_from_u64(rng.random::<u64>() ^ name_hash);
        TreeInput {
            family: self.name,
            config: self.bounds.sample(&mut family_rng),
        }
    }
}

impl fmt::Display for TreeFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name)
    }
}

/// One tree drawn from a family. Its `Display` describes the draw, which the benchmark
/// id (family and seed) does not.
struct TreeInput {
    family: &'static str,
    config: RandomizedElementTreeConfig,
}

impl fmt::Display for TreeInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let topology = match self.config.topology() {
            RandomizedElementTreeTopology::Wide => "wide",
            RandomizedElementTreeTopology::Deep => "deep",
            RandomizedElementTreeTopology::Mixed => "mixed",
            RandomizedElementTreeTopology::Narrow => "narrow",
        };
        write!(
            formatter,
            "{} tree: {topology}, {} elements, {}% entities, {}% handlers",
            self.family,
            self.config.element_count(),
            (self.config.entity_density() * 100.0).round() as usize,
            (self.config.handler_density() * 100.0).round() as usize
        )
    }
}

/// Tall trees recurse once per element through layout and paint; without gpui's `stacker`
/// feature a chain of 384 draws and one of 512 overflows a 2 MB stack, so tall families
/// stop well short of that. Real UI is not hundreds of levels deep either.
const TALL_MAX_ELEMENTS: usize = 256;

/// Entities are how GPUI structures a UI, so every family has at least this share of its
/// elements backed by one; a tree with none is a single view, which no renderer can
/// partially reuse and which real UI never is.
const MIN_ENTITY_DENSITY: f64 = 0.05;

/// The families every benchmark samples. Each is a question: what does this class of
/// change cost on a tree like *this*?
fn families() -> Vec<TreeFamily> {
    let handlers = 0.1..=0.4;
    vec![
        TreeFamily {
            name: "any",
            bounds: RandomizedElementTreeBounds::new(32..=TALL_MAX_ELEMENTS)
                .with_entity_density(MIN_ENTITY_DENSITY..=0.5)
                .with_handler_density(handlers.clone()),
        },
        TreeFamily {
            name: "wide",
            bounds: RandomizedElementTreeBounds::new(128..=2048)
                .with_topologies([RandomizedElementTreeTopology::Wide])
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        },
        TreeFamily {
            name: "tall",
            bounds: RandomizedElementTreeBounds::new(32..=TALL_MAX_ELEMENTS)
                .with_topologies(
                    RandomizedElementTreeTopology::ALL
                        .into_iter()
                        .filter(|topology| topology.is_tall()),
                )
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        },
        TreeFamily {
            name: "mixed",
            bounds: RandomizedElementTreeBounds::new(128..=1024)
                .with_topologies([RandomizedElementTreeTopology::Mixed])
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        },
        TreeFamily {
            name: "dense-entities",
            bounds: RandomizedElementTreeBounds::new(128..=1024)
                .with_topologies([
                    RandomizedElementTreeTopology::Wide,
                    RandomizedElementTreeTopology::Mixed,
                ])
                .with_entity_density(0.75..=1.0)
                .with_handler_density(handlers),
        },
    ]
}

/// Builds the tree, draws it twice so layout, the scene and both frame buffers are warm,
/// and measures `mutate` once per frame.
fn measure(
    group: &str,
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Context<RandomizedElementTree>),
) {
    measure_in_window(group, input, cx, |tree, _, cx| mutate(tree, cx));
}

fn measure_in_window(
    group: &str,
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Window, &mut Context<RandomizedElementTree>),
) {
    let config = input.config;
    let before_window = bench_metrics::allocation_stats();
    let mut window = cx.add_empty_window();
    let tree = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
    cx.run_until_idle();
    // Warm both frame buffers and their scratch allocations. A short sample can
    // otherwise measure only the cheaper first buffer after mounting the tree.
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    describe_once(input, before_window, bench_metrics::allocation_stats());

    let mut summary = SeedTotals::default();
    let mut previous_iteration: Option<(Instant, Option<bench_metrics::AllocationStats>)> = None;
    cx.bench_renderer(tree, |tree, window, cx| {
        // From one iteration's start to the next is one whole iteration, the frame
        // included, as Criterion times it; the reads add well under a microsecond.
        let now = Instant::now();
        let stats = bench_metrics::allocation_stats();
        if let Some((started, started_stats)) = previous_iteration {
            summary.elapsed += now - started;
            summary.iterations += 1;
            if let Some((start, end)) = started_stats.zip(stats) {
                summary.allocations += end.allocations - start.allocations;
            }
        }
        previous_iteration = Some((now, stats));

        // The tree records every rendered entity; clear it so the list stays one frame long.
        tree.reset_work_counters();
        mutate(tree, window, cx);
    });
    window.update(|window, _| report_scene_stats(group, input.family, window));
    record_for_summary(group, input, summary);
}

fn report_scene_stats(group: &str, input: &str, window: &Window) {
    if std::env::var("GPUI_BENCH_SCENE_STATS").as_deref() == Ok("1") {
        let scene = window.rendered_scene();
        eprintln!(
            "scene stats {group}/{input}: batches={}, paths={}, surfaces={}",
            scene.batches().count(),
            scene.paths.len(),
            scene.surfaces.len(),
        );
    }
}

thread_local! {
    static LAST_DESCRIBED: RefCell<Option<RandomizedElementTreeConfig>> =
        const { RefCell::new(None) };
}

/// Prints the tree and the heap its window holds after the first frame, once per run of
/// consecutive routine calls on the same tree: Criterion calls a benchmark's routine for
/// warm-up and every sample, and each call builds the same tree.
fn describe_once(
    input: &TreeInput,
    before_window: Option<bench_metrics::AllocationStats>,
    after_first_frame: Option<bench_metrics::AllocationStats>,
) {
    let is_new =
        LAST_DESCRIBED.with(|last| last.borrow_mut().replace(input.config)) != Some(input.config);
    if !is_new {
        return;
    }
    match before_window.zip(after_first_frame) {
        Some((before, after)) => eprintln!(
            "{input}; window holds {:.1} KB after the first frame",
            (after.live_bytes() - before.live_bytes()) as f64 / 1024.
        ),
        None => eprintln!("{input}"),
    }
}

/// One seed's measured iterations, summed over every routine call Criterion makes for it.
#[derive(Clone, Copy, Default)]
struct SeedTotals {
    elapsed: Duration,
    iterations: u64,
    allocations: u64,
}

/// Totals per group and family, each holding one entry per tree (seed), in run order.
type SummaryTotals = Vec<(
    String,
    &'static str,
    Vec<(RandomizedElementTreeConfig, SeedTotals)>,
)>;

thread_local! {
    static SUMMARY: RefCell<SummaryTotals> = const { RefCell::new(Vec::new()) };
}

fn record_for_summary(group: &str, input: &TreeInput, totals: SeedTotals) {
    SUMMARY.with(|summary| {
        let mut summary = summary.borrow_mut();
        let index = match summary.iter().position(|(existing_group, family, _)| {
            existing_group == group && *family == input.family
        }) {
            Some(index) => index,
            None => {
                summary.push((group.to_owned(), input.family, Vec::new()));
                summary.len() - 1
            }
        };
        let seeds = &mut summary[index].2;
        match seeds.iter_mut().find(|(config, _)| *config == input.config) {
            Some((_, seed)) => {
                seed.elapsed += totals.elapsed;
                seed.iterations += totals.iterations;
                seed.allocations += totals.allocations;
            }
            None => seeds.push((input.config, totals)),
        }
    });
}

fn geometric_mean(values: impl IntoIterator<Item = f64>) -> f64 {
    let (log_sum, count) = values
        .into_iter()
        .fold((0.0, 0_u32), |(sum, count), value| {
            (sum + value.max(f64::MIN_POSITIVE).ln(), count + 1)
        });
    (log_sum / f64::from(count.max(1))).exp()
}

/// Prints one line per group and family: the geometric mean across its seeds of the time
/// and allocations per iteration, so a family reads as one number while each seed stays
/// its own reproducible benchmark. Registered as the last target so it runs after every
/// group; prints nothing when a filter matched no randomized tree benchmark.
fn family_summary(_: &mut criterion::Criterion<gpui::BenchMeasurement>) {
    let summary = SUMMARY.with(|summary| summary.take());
    if summary.is_empty() {
        return;
    }
    eprintln!(
        "\nRandomizedTree family summary (geometric mean across seeds; includes Criterion warm-up)"
    );
    eprintln!(
        "  {:<34} {:<15} {:>5} {:>14} {:>14}",
        "group", "family", "seeds", "time/iter", "allocs/iter"
    );
    for (group, family, seeds) in summary {
        let per_iteration = |value: fn(&SeedTotals) -> f64| {
            geometric_mean(
                seeds
                    .iter()
                    .filter(|(_, totals)| totals.iterations > 0)
                    .map(|(_, totals)| value(totals) / totals.iterations as f64),
            )
        };
        let time = per_iteration(|totals| totals.elapsed.as_secs_f64());
        let allocations = per_iteration(|totals| totals.allocations as f64);
        eprintln!(
            "  {group:<34} {family:<15} {:>5} {:>11.1} µs {allocations:>14.0}",
            seeds.len(),
            time * 1e6
        );
    }
}

/// A frame that rebuilds everything: `Window::refresh`, the fallback GPUI uses whenever
/// it cannot tell what changed (window activation, theme or settings changes via
/// `refresh_windows`, an inspector toggle). Nothing in the tree changes, so this is the
/// pure cost of the known-good path, and the renderer's worst case: every view and element
/// rendered again, none reused. Its difference between two GPUI revisions is the price of
/// a full refresh on the newer one.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/full refresh", fps = 120)]
fn full_refresh(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    measure_in_window("full refresh", input, cx, |_, window, _| window.refresh());
}

/// A frame in which the root is notified but nothing in the tree changed. On a renderer
/// that rebuilds every frame this is the whole tree's cost; on one that retains output
/// it is the floor for a frame that has to do nothing.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/unchanged", fps = 120)]
fn unchanged(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    measure("unchanged", input, cx, |_, cx| cx.notify());
}

/// The first leaf's background color alternates each frame: the smallest possible change,
/// confined to one element and, when it has one, its owning entity. Recoloring the same
/// leaf keeps short and long samples measuring the same work.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/leaf style", fps = 120)]
fn leaf_style(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    measure("leaf style", input, cx, |tree, cx| {
        tree.recolor_first_leaf(cx);
    });
}

/// A descendant's width and height change each frame, so its siblings and ancestors
/// re-lay out even where their own content did not change.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/leaf bounds", fps = 120)]
fn leaf_bounds(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    measure("leaf bounds", input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
}

/// The root's width and height change each frame: every element's layout is stale.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/root layout", fps = 120)]
fn root_layout(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    measure("root layout", input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::RootBounds, cx);
    });
}

/// One element is inserted or removed per frame, alternating so the tree keeps its size
/// to within one element over the whole loop. Removal takes a leaf, never a subtree.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/insert-remove", fps = 120)]
fn insert_remove(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let mut insert = true;
    measure("insert-remove", input, cx, |tree, cx| {
        let kind = if insert {
            RandomizedElementTreeMutationKind::Insert
        } else {
            RandomizedElementTreeMutationKind::RemoveLeaf
        };
        tree.apply_mutation(kind, cx);
        insert = !insert;
    });
}

/// A descendant moves among its siblings each frame; the set of elements is unchanged
/// but their order, and therefore every sibling's position, is not.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/reorder", fps = 120)]
fn reorder(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    // A `Deep` tree has no parent with two children until an insert creates one, so the
    // tree inserts instead; every other shape reorders on every frame.
    measure("reorder", input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::Reorder, cx);
    });
}

/// The same tree with every entity boundary wrapped in a definite-size `.cached()` view,
/// as Zed does for panes and docks, so a frame can reuse clean entities' output.
fn cached_input(family: &TreeFamily, rng: &mut StdRng) -> TreeInput {
    let mut input = family.sample(rng);
    input.config = input.config.with_cached_views(true);
    input
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/full refresh", fps = 120)]
fn cached_full_refresh(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    measure_in_window("cached full refresh", &input, cx, |_, window, _| {
        window.refresh()
    });
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/leaf style", fps = 120)]
fn cached_leaf_style(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    measure("cached leaf style", &input, cx, |tree, cx| {
        tree.recolor_first_leaf(cx);
    });
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/leaf bounds", fps = 120)]
fn cached_leaf_bounds(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    measure("cached leaf bounds", &input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/insert-remove", fps = 120)]
fn cached_insert_remove(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let mut insert = true;
    measure("cached insert-remove", &input, cx, |tree, cx| {
        let kind = if insert {
            RandomizedElementTreeMutationKind::Insert
        } else {
            RandomizedElementTreeMutationKind::RemoveLeaf
        };
        tree.apply_mutation(kind, cx);
        insert = !insert;
    });
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/reorder", fps = 120)]
fn cached_reorder(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    measure("cached reorder", &input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::Reorder, cx);
    });
}

/// A tree family, the share of its elements that change each frame, and where those
/// changes sit.
#[derive(Clone)]
struct ChangingShareInput {
    family: TreeFamily,
    share_percent: usize,
    locality: ChangeLocality,
}

impl ChangingShareInput {
    fn locality_name(&self) -> &'static str {
        match self.locality {
            ChangeLocality::Localized => "local",
            ChangeLocality::Spread => "spread",
        }
    }
}

impl fmt::Display for ChangingShareInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locality = self.locality_name();
        write!(
            formatter,
            "{}-f{}-{locality}",
            self.family, self.share_percent
        )
    }
}

/// The share of elements recolored per frame, from one element to all of them. A renderer
/// that retains clean entity subtrees should fall toward the left of this axis; one that
/// redraws everything is flat across it, so the two curves' distance is what retention
/// buys at each share.
///
/// The main line keeps each frame's change inside one entity, as a user action does. Two
/// `spread` points scatter it uniformly instead: the pessimistic bound, where every
/// changed element may dirty a different subtree.
fn changing_share_inputs() -> Vec<ChangingShareInput> {
    families()
        .into_iter()
        .flat_map(|family| {
            [
                (0, ChangeLocality::Localized),
                (1, ChangeLocality::Localized),
                (5, ChangeLocality::Localized),
                (25, ChangeLocality::Localized),
                (100, ChangeLocality::Localized),
                (5, ChangeLocality::Spread),
                (25, ChangeLocality::Spread),
            ]
            .into_iter()
            .map(move |(share_percent, locality)| ChangingShareInput {
                family: family.clone(),
                share_percent,
                locality,
            })
        })
        .collect()
}

// One seed rather than two: every family is multiplied by seven shares.
#[gpui::bench(inputs = changing_share_inputs(), input_name = "tree", iterations = 1, group = "RandomizedTree/changing share", fps = 120)]
fn changing_share(input: &ChangingShareInput, mut rng: StdRng, cx: &mut BenchAppContext) {
    let tree = &input.family.sample(&mut rng);
    let element_count = tree.config.element_count();
    let per_frame = if input.share_percent == 0 {
        0
    } else {
        (input.share_percent * element_count).div_ceil(100).max(1)
    };
    let group = format!(
        "changing share f{}-{}",
        input.share_percent,
        input.locality_name()
    );
    measure(&group, tree, cx, |tree, cx| {
        if per_frame == 0 {
            cx.notify();
        } else {
            tree.recolor_children(per_frame, input.locality, cx);
        }
    });
}

struct TextPanel {
    index: usize,
    value: u32,
}

impl Render for TextPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let value = self.value;
        gpui::div()
            .id(gpui::ElementId::NamedInteger(
                "text-panel".into(),
                self.index as u64,
            ))
            .flex()
            .flex_col()
            .w(gpui::px(220.))
            .h(gpui::px(192.))
            .bg(gpui::rgb(0x181818))
            .overflow_hidden()
            .children((0..64).map(|label| {
                gpui::div()
                    .flex_shrink_0()
                    .h(gpui::px(16.))
                    .child(format!("Metric {label:02}: {value:06}"))
            }))
    }
}

struct TextPanelRoot {
    panels: Vec<Entity<TextPanel>>,
    font_family: gpui::SharedString,
}

impl TextPanelRoot {
    fn new(panel_count: usize, font_family: &'static str, cx: &mut Context<Self>) -> Self {
        Self {
            panels: (0..panel_count)
                .map(|index| cx.new(|_| TextPanel { index, value: 0 }))
                .collect(),
            font_family: font_family.into(),
        }
    }
}

impl Render for TextPanelRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .id("text-panels")
            .flex()
            .flex_wrap()
            .font_family(self.font_family.clone())
            .text_size(gpui::px(12.))
            .text_color(gpui::rgb(0xffffff))
            .child("0123456789")
            .children(self.panels.iter().cloned())
    }
}

/// Whether this build shapes and rasterizes text with the platform's real text system.
/// Without it, text workloads paint no glyphs, so they are skipped rather than measured.
fn text_shaping_available() -> bool {
    if cfg!(all(
        target_os = "linux",
        not(any(feature = "x11", feature = "wayland"))
    )) {
        eprintln!("text benchmarks require x11 or wayland on Linux for production text shaping");
        false
    } else if cfg!(all(target_os = "macos", not(feature = "font-kit"))) {
        eprintln!("text benchmarks require font-kit on macOS for production text shaping");
        false
    } else {
        true
    }
}

fn text_panel_counts() -> Vec<usize> {
    if text_shaping_available() {
        vec![8, 32, 96]
    } else {
        Vec::new()
    }
}

#[gpui::bench(inputs = text_panel_counts(), input_name = "panels", group = "TextPanels/two updates", fps = 120)]
fn text_panel_updates(panel_count: &usize, cx: &mut BenchAppContext) {
    measure_text_panel_updates(*panel_count, "monospace", cx);
}

/// A monospace family the text workloads name explicitly, so they measure shaping and
/// rasterization with one known font rather than whatever the platform's `monospace`
/// fallback resolves to. Linux machines must install it; macOS and Windows ship it.
#[cfg(target_os = "macos")]
const INSTALLED_FONT: &str = "Menlo";
#[cfg(target_os = "windows")]
const INSTALLED_FONT: &str = "Consolas";
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const INSTALLED_FONT: &str = "Noto Sans Mono";

/// Fails setup, before anything is measured, if the named font would fall back to another
/// family: the workload would then measure a different font than its name says.
fn require_installed_font(cx: &BenchAppContext) {
    let font = gpui::font(INSTALLED_FONT);
    let text_system = cx.read(|cx| cx.text_system().clone());
    let font_id = text_system.resolve_font(&font);
    assert_eq!(
        text_system.get_font_for_id(font_id).map(|font| font.family),
        Some(font.family),
        "the text benchmarks require {INSTALLED_FONT} to be installed"
    );
}

#[gpui::bench(inputs = text_panel_counts(), input_name = "panels", group = "TextPanels/installed font two updates", fps = 120)]
fn installed_font_panel_updates(panel_count: &usize, cx: &mut BenchAppContext) {
    require_installed_font(cx);
    measure_text_panel_updates(*panel_count, INSTALLED_FONT, cx);
}

/// Two panels' text changes each frame; the other panels are unchanged.
fn measure_text_panel_updates(
    panel_count: usize,
    font_family: &'static str,
    cx: &mut BenchAppContext,
) {
    let mut window = cx.add_empty_window();
    let root = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| TextPanelRoot::new(panel_count, font_family, cx))
    });
    cx.run_until_idle();
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    cx.bench_renderer(root, |root, _, cx| {
        for index in [0, panel_count / 2] {
            if let Some(panel) = root.panels.get(index) {
                panel.update(cx, |panel, cx| {
                    panel.value = (panel.value + 1) % 1_000_000;
                    cx.notify();
                });
            }
        }
    });
    window.update(|window, _| report_scene_stats("TextPanels", &panel_count.to_string(), window));
    window.update(|window, _| window.remove_window());
}

/// Keeps a populated window alive until Criterion drops the measured task's output,
/// so teardown isn't timed.
struct PopulatedWindow(gpui::AsyncWindowContext);

impl Drop for PopulatedWindow {
    fn drop(&mut self) {
        if let Err(error) = self.0.update(|window, _| window.remove_window()) {
            eprintln!("failed to remove populated window: {error}");
        }
    }
}

fn populate(window: &mut Window, cx: &mut gpui::App, panel_count: usize) -> PopulatedWindow {
    window.replace_root(cx, |_, cx| {
        TextPanelRoot::new(panel_count, INSTALLED_FONT, cx)
    });
    PopulatedWindow(window.to_async(cx))
}

thread_local! {
    static DESCRIBED_FIRST_POPULATED: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

/// Mounting a window full of text and drawing its first frame, from layout through present.
#[gpui::bench(inputs = text_panel_counts(), input_name = "panels", group = "TextPanels/first populated installed font", fps = 120)]
fn first_populated_text_panels(panel_count: &usize, cx: &mut BenchAppContext) {
    let panel_count = *panel_count;
    require_installed_font(cx);

    let describe = DESCRIBED_FIRST_POPULATED.with(|described| {
        let mut described = described.borrow_mut();
        if described.contains(&panel_count) {
            false
        } else {
            described.push(panel_count);
            true
        }
    });
    if describe {
        // Font resolution above is shared setup, not memory the window holds.
        let before_window = bench_metrics::allocation_stats();
        let mut window = cx.add_empty_window();
        let before_population = bench_metrics::allocation_stats();
        let populated = window.update(|window, cx| populate(window, cx, panel_count));
        cx.run_until_idle();
        let after_population = bench_metrics::allocation_stats();
        match before_window.zip(before_population).zip(after_population) {
            Some(((before, empty), after)) => eprintln!(
                "first populated panels: {panel_count}, family {INSTALLED_FONT}; live heap before window {} B, empty window {} B, populated {} B; population delta {} B (process-wide estimates, not exclusive window ownership)",
                before.live_bytes(),
                empty.live_bytes(),
                after.live_bytes(),
                after.live_bytes() - empty.live_bytes(),
            ),
            None => eprintln!("first populated panels: {panel_count}, family {INSTALLED_FONT}"),
        }
        window.update(|window, _| {
            report_scene_stats(
                "TextPanels first populated",
                &panel_count.to_string(),
                window,
            );
        });
        drop(populated);
        cx.run_until_idle();
    }

    cx.bench_batched_task(
        |cx| cx.add_empty_window(),
        |window, _| {
            let populated = window.update(|window, cx| populate(window, cx, panel_count));
            // bench_batched_task delivers pending production frame callbacks
            // before polling this ready task, so layout through present is timed.
            gpui::Task::ready(populated)
        },
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Churn {
    Replace,
    Remount,
    Id,
    Text,
}

impl fmt::Display for Churn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Replace => "replace-8-of-96",
            Self::Remount => "remount-48-of-96",
            Self::Id => "id-and-anonymous-count",
            Self::Text => "text-96",
        })
    }
}

struct ChurnPanel {
    index: usize,
    frame: usize,
    workload: Churn,
}

impl Render for ChurnPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let id = if self.workload == Churn::Id {
            self.frame * 96 + self.index
        } else {
            self.index
        };
        // Checkpoints are multiples of ten, so their live shapes are identical.
        let labels = if self.workload == Churn::Id {
            56 + self.frame % 10
        } else {
            64
        };
        let value = if self.workload == Churn::Id {
            0
        } else {
            self.frame % 1_000_000
        };
        gpui::div()
            .id(gpui::ElementId::NamedInteger(
                "churn-panel".into(),
                id as u64,
            ))
            .flex()
            .flex_col()
            .w(gpui::px(220.))
            .h(gpui::px(192.))
            .bg(gpui::rgb(0x181818))
            .overflow_hidden()
            .children((0..labels).map(|label| {
                gpui::div()
                    .flex_shrink_0()
                    .h(gpui::px(16.))
                    .child(format!("Metric {label:02}: {value:06}"))
            }))
    }
}

struct ChurnRoot {
    panels: Vec<Entity<ChurnPanel>>,
    workload: Churn,
    frame: usize,
}

impl ChurnRoot {
    fn new(workload: Churn, cx: &mut Context<Self>) -> Self {
        Self {
            panels: (0..96)
                .map(|index| {
                    cx.new(|_| ChurnPanel {
                        index,
                        frame: 0,
                        workload,
                    })
                })
                .collect(),
            workload,
            frame: 0,
        }
    }

    fn visible_panels(&self) -> usize {
        if self.workload == Churn::Remount && self.frame % 2 == 1 {
            48
        } else {
            96
        }
    }

    fn advance(&mut self, cx: &mut Context<Self>) {
        self.frame += 1;
        match self.workload {
            Churn::Replace => {
                for offset in 0..8 {
                    let index = ((self.frame - 1) * 8 + offset) % 96;
                    let replacement = cx.new(|_| ChurnPanel {
                        index,
                        frame: self.frame,
                        workload: self.workload,
                    });
                    if let Some(panel) = self.panels.get_mut(index) {
                        *panel = replacement;
                    }
                }
            }
            Churn::Id | Churn::Text => {
                for panel in &self.panels {
                    panel.update(cx, |panel, cx| {
                        panel.frame = self.frame;
                        cx.notify();
                    });
                }
            }
            Churn::Remount => {}
        }
        cx.notify();
    }
}

impl Render for ChurnRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .id("churn-root")
            .flex()
            .flex_wrap()
            .font_family(INSTALLED_FONT)
            .text_size(gpui::px(12.))
            .text_color(gpui::rgb(0xffffff))
            .child("0123456789")
            .children(self.panels.iter().take(self.visible_panels()).cloned())
    }
}

fn churn_workloads() -> Vec<Churn> {
    if text_shaping_available() {
        vec![Churn::Replace, Churn::Remount, Churn::Id, Churn::Text]
    } else {
        Vec::new()
    }
}

#[gpui::bench(inputs = churn_workloads(), input_name = "workload", group = "LayoutChurnHeap", fps = 120)]
fn layout_churn_heap(workload: &Churn, cx: &mut BenchAppContext) {
    require_installed_font(cx);
    // One iteration is a complete 3,000-frame lifetime, not a frame comparison.
    cx.bench_iter(|cx| {
        let before_window = bench_metrics::allocation_stats();
        let mut window = cx.add_empty_window();
        let root = window
            .update(|window, cx| window.replace_root(cx, |_, cx| ChurnRoot::new(*workload, cx)));
        cx.run_until_idle();
        let mut checkpoints = Vec::with_capacity(6);
        checkpoints.push((0, bench_metrics::allocation_stats()));
        for frame in 1..=3000 {
            window.update(|_, cx| root.update(cx, |root, cx| root.advance(cx)));
            cx.run_until_idle();
            if [50, 200, 1000, 2000, 3000].contains(&frame) {
                checkpoints.push((frame, bench_metrics::allocation_stats()));
            }
        }
        // Logging allocates, so it waits until every heap reading is taken.
        for (frame, stats) in checkpoints {
            match before_window.zip(stats) {
                Some((before, after)) => eprintln!(
                    "layout churn {workload}: frame {frame}, live heap {} B, window delta {} B (process-wide requested bytes)",
                    after.live_bytes(),
                    after.live_bytes() - before.live_bytes(),
                ),
                None => eprintln!("layout churn {workload}: frame {frame}, heap unavailable"),
            }
        }
        window.update(|window, _| window.remove_window());
        drop(root);
        cx.run_until_idle();
    });
}

#[gpui::bench(inputs = churn_workloads(), input_name = "workload", group = "LayoutChurn", fps = 120)]
fn layout_churn(workload: &Churn, cx: &mut BenchAppContext) {
    require_installed_font(cx);
    let mut window = cx.add_empty_window();
    let root =
        window.update(|window, cx| window.replace_root(cx, |_, cx| ChurnRoot::new(*workload, cx)));
    cx.run_until_idle();
    cx.bench_renderer(root, |root, _, cx| root.advance(cx));
    window.update(|window, _| window.remove_window());
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ScrollWorkload {
    Text,
    Virtual,
    Nested,
}

impl fmt::Display for ScrollWorkload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Text => "editor-like-text",
            Self::Virtual => "uniform-list",
            Self::Nested => "pane-with-static-siblings",
        })
    }
}

struct ScrollPane {
    workload: ScrollWorkload,
    handle: gpui::ScrollHandle,
    list_handle: gpui::UniformListScrollHandle,
    distance: f32,
    direction: f32,
}

impl ScrollPane {
    fn advance(&mut self, cx: &mut Context<Self>) {
        // Reverse before either boundary: every measured frame changes the viewport,
        // even in long Criterion calibration runs.
        if self.distance >= 6000.0 {
            self.direction = -1.0;
        } else if self.distance <= 600.0 {
            self.direction = 1.0;
        }
        self.distance += self.direction * 12.0;
        self.handle
            .set_offset(gpui::point(gpui::px(0.0), gpui::px(-self.distance)));
        cx.notify();
    }

    fn row(index: usize) -> impl IntoElement {
        gpui::div()
            .flex()
            .h(gpui::px(22.0))
            .flex_shrink_0()
            .child(
                gpui::div()
                    .w(gpui::px(64.0))
                    .text_color(gpui::rgb(0x858585))
                    .child(format!("{index:5}")),
            )
            .child(gpui::div().text_color(gpui::rgb(0xc586c0)).child("let "))
            .child(
                gpui::div()
                    .text_color(gpui::rgb(0x9cdcfe))
                    .child(format!("line_{index}")),
            )
            .child(
                gpui::div()
                    .text_color(gpui::rgb(0xd4d4d4))
                    .child(" = document.visible_lines(viewport);"),
            )
    }
}

impl Render for ScrollPane {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let container = gpui::div()
            .size_full()
            .font_family(INSTALLED_FONT)
            .text_size(gpui::px(14.0))
            .bg(gpui::rgb(0x202020));
        if self.workload == ScrollWorkload::Virtual {
            container
                .child(
                    gpui::uniform_list(
                        "scroll-rows",
                        5000,
                        cx.processor(|_, range: std::ops::Range<usize>, _, _| {
                            range.map(Self::row).collect()
                        }),
                    )
                    .size_full()
                    .track_scroll(&self.list_handle),
                )
                .into_any_element()
        } else {
            container
                .id("scroll-text")
                .overflow_y_scroll()
                .track_scroll(&self.handle)
                .children((0..1000).map(Self::row))
                .into_any_element()
        }
    }
}

struct ScrollStaticPanel {
    status: bool,
}

impl Render for ScrollStaticPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .size_full()
            .bg(gpui::rgb(0x303030))
            .text_color(gpui::rgb(0xcccccc))
            .children((0..if self.status { 1 } else { 40 }).map(|index| {
                gpui::div().h(gpui::px(22.0)).child(format!(
                    "{} {index}",
                    if self.status { "Status" } else { "File" }
                ))
            }))
    }
}

struct ScrollRoot {
    pane: Entity<ScrollPane>,
    sidebar: Entity<ScrollStaticPanel>,
    status: Entity<ScrollStaticPanel>,
    nested: bool,
}

impl Render for ScrollRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .size_full()
            .flex()
            .flex_col()
            .child(
                gpui::div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .when(self.nested, |element| {
                        element.child(
                            gpui::div()
                                .w(gpui::px(220.0))
                                .h_full()
                                .child(self.sidebar.clone()),
                        )
                    })
                    .child(
                        gpui::div()
                            .flex_1()
                            .min_w_0()
                            .h_full()
                            .overflow_hidden()
                            .child(self.pane.clone()),
                    ),
            )
            .when(self.nested, |element| {
                element.child(gpui::div().h(gpui::px(28.0)).child(self.status.clone()))
            })
    }
}

fn scroll_workloads() -> Vec<ScrollWorkload> {
    if text_shaping_available() {
        vec![
            ScrollWorkload::Text,
            ScrollWorkload::Virtual,
            ScrollWorkload::Nested,
        ]
    } else {
        Vec::new()
    }
}

/// Warm pixel scrolling, including the same owner notification as a wheel handler.
/// The text case deliberately materializes lines; the list case is viewport-bounded.
#[gpui::bench(inputs = scroll_workloads(), input_name = "workload", group = "Scroll", fps = 360)]
fn scroll(workload: &ScrollWorkload, cx: &mut BenchAppContext) {
    let mut window = cx.add_empty_window();
    let pane = window.update(|window, cx| {
        let pane = cx.new(|_| {
            let list_handle = gpui::UniformListScrollHandle::new();
            let handle = if *workload == ScrollWorkload::Virtual {
                list_handle.0.borrow().base_handle.clone()
            } else {
                gpui::ScrollHandle::new()
            };
            ScrollPane {
                workload: *workload,
                handle,
                list_handle,
                distance: 600.0,
                direction: 1.0,
            }
        });
        window.replace_root(cx, |_, cx| ScrollRoot {
            pane: pane.clone(),
            sidebar: cx.new(|_| ScrollStaticPanel { status: false }),
            status: cx.new(|_| ScrollStaticPanel { status: true }),
            nested: *workload == ScrollWorkload::Nested,
        });
        pane
    });
    cx.run_until_idle();
    for _ in 0..8 {
        window.update(|_, cx| pane.update(cx, |pane, cx| pane.advance(cx)));
        cx.run_until_idle();
    }
    cx.bench_renderer(pane, |pane, _, cx| pane.advance(cx));
    window.update(|window, _| window.remove_window());
}

// Frames take 0.4 to 2.5 ms, so a 1 s measurement is already hundreds to thousands of
// frames; Criterion's 3 s warm-up and 5 s measurement per id would make the suite take
// most of an hour. Pass `ITERATIONS=6`, `--warm-up-time` or `--measurement-time` for a
// closer comparison.
gpui::bench_group!(
    name = benches;
    config = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1))
        .sample_size(20);
    targets = full_refresh,
    cached_full_refresh,
    cached_leaf_style,
    cached_leaf_bounds,
    cached_insert_remove,
    cached_reorder,
    unchanged,
    leaf_style,
    leaf_bounds,
    root_layout,
    insert_remove,
    reorder,
    changing_share,
    text_panel_updates,
    installed_font_panel_updates,
    first_populated_text_panels,
    layout_churn,
    scroll,
    family_summary
);
gpui::bench_group!(
    name = heap_diagnostics;
    config = criterion::Criterion::default()
        .warm_up_time(Duration::from_millis(1))
        .measurement_time(Duration::from_millis(1))
        .sample_size(10);
    targets = layout_churn_heap
);
gpui::bench_main!(benches, heap_diagnostics);
