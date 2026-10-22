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
//! The tree's own work counters are asserted after each measured loop: a faster frame that
//! stopped rendering the changed element is not an improvement.

use std::{
    cell::{Cell, RefCell},
    fmt,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::prelude::*;
use gpui::{
    BenchAppContext, Context, Entity, Window,
    private::rand::{Rng as _, SeedableRng as _, rngs::StdRng},
    randomized_element_tree::{
        ChangeLocality, RandomizedElementTree, RandomizedElementTreeBounds,
        RandomizedElementTreeConfig, RandomizedElementTreeMutation,
        RandomizedElementTreeMutationKind, RandomizedElementTreeTopology, RecoloredChildren,
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

/// Builds the tree, draws it once so layout and the scene are warm, and measures
/// `mutate` once per frame. Returns how many frames were measured, after checking that
/// the frames did render something: a frame that skipped the changed element would be
/// fast and wrong.
fn measure(
    group: &str,
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Context<RandomizedElementTree>),
) -> usize {
    measure_in_window(group, input, cx, |tree, _, cx| mutate(tree, cx)).frames
}

/// What a measured loop did: frames measured, and the entity renders the frames showed.
struct Measured {
    frames: usize,
    entity_renders: usize,
}

fn measure_in_window(
    group: &str,
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Window, &mut Context<RandomizedElementTree>),
) -> Measured {
    let config = input.config;
    if std::env::var("GPUI_VERIFY_REUSE_DIFFERENTIAL").is_ok_and(|value| value == "1") {
        verify_tree_reuse(config.with_cached_views(true), cx);
    }
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

    let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());
    assert_eq!(snapshot.descendant_count(), config.element_count());
    let initial = tree.read_with(cx, |tree, _| tree.work_counters());
    assert!(
        initial.root_render_count() >= 1,
        "the tree must have drawn before timing"
    );
    assert!(
        initial.element_render_count() >= config.element_count(),
        "every generated element renders in the first frame"
    );
    // Each iteration reads what the previous frame rendered before resetting for its own
    // mutation. Frame pacing may coalesce two mutations into one drawn frame, the more
    // often the faster the frames, so some observation windows legitimately see no draw;
    // a renderer that skipped the changed element would see none in any of them.
    let mut frames = 0;
    let mut frames_that_rendered = 0;
    let mut entity_renders = 0;
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

        let previous = tree.work_counters();
        entity_renders += previous.entity_render_count();
        if previous.root_render_count() + previous.entity_render_count() >= 1
            && previous.element_render_count() >= 1
        {
            frames_that_rendered += 1;
        }
        tree.reset_work_counters();
        mutate(tree, window, cx);
        frames += 1;
    });
    window.update(|window, _| report_scene_stats(group, input.family, window));
    assert!(
        frames_that_rendered * 2 >= frames,
        "the notified root or entity re-rendered in {frames_that_rendered} of {frames} frames"
    );
    record_for_summary(group, input, summary);
    Measured {
        frames,
        entity_renders,
    }
}

fn report_scene_stats(group: &str, input: &str, window: &Window) {
    if std::env::var("GPUI_BENCH_SCENE_STATS").as_deref() == Ok("1") {
        let scene = window.rendered_scene();
        eprintln!(
            "scene stats {group}/{input}: batches={}, paths={}, surfaces={}, work={:?}",
            scene.batches().count(),
            scene.paths.len(),
            scene.surfaces.len(),
            window.view_reuse_stats(),
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
    let measured = measure_in_window("full refresh", input, cx, |_, window, _| window.refresh());
    assert!(measured.frames > 0);
    // A full refresh renders every entity; a renderer that replayed them would render none.
    if input.config.entity_density() >= 0.25 {
        assert!(
            measured.entity_renders > 0,
            "a full refresh re-renders entities"
        );
    }
}

/// A frame in which the root is notified but nothing in the tree changed. On a renderer
/// that rebuilds every frame this is the whole tree's cost; on one that retains output
/// it is the floor for a frame that has to do nothing.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/unchanged", fps = 120)]
fn unchanged(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let frames = measure("unchanged", input, cx, |_, cx| cx.notify());
    assert!(frames > 0);
}

/// One descendant's background color changes each frame: the smallest possible change,
/// confined to one element and, when it has one, its owning entity.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/leaf style", fps = 120)]
fn leaf_style(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let mut recolored = 0usize;
    let frames = measure("leaf style", input, cx, |tree, cx| {
        if tree.recolor_first_leaf(cx).is_some() {
            recolored += 1;
        }
    });
    assert!(frames > 0);
    assert_eq!(
        recolored, frames,
        "a nonempty tree recolors a child every frame"
    );
}

/// A descendant's width and height change each frame, so its siblings and ancestors
/// re-lay out even where their own content did not change.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/leaf bounds", fps = 120)]
fn leaf_bounds(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let frames = measure("leaf bounds", input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
    assert!(frames > 0);
}

/// The root's width and height change each frame: every element's layout is stale.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/root layout", fps = 120)]
fn root_layout(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let frames = measure("root layout", input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::RootBounds, cx);
    });
    assert!(frames > 0);
}

/// One element is inserted or removed per frame, alternating so the tree keeps its size
/// to within one element over the whole loop. Removal takes a leaf, never a subtree.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/insert-remove", fps = 120)]
fn insert_remove(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let mut insert = true;
    let mut inserted = 0usize;
    let mut removed = 0usize;
    let frames = measure("insert-remove", input, cx, |tree, cx| {
        let kind = if insert {
            RandomizedElementTreeMutationKind::Insert
        } else {
            RandomizedElementTreeMutationKind::RemoveLeaf
        };
        match tree.apply_mutation(kind, cx) {
            RandomizedElementTreeMutation::Inserted { .. } => inserted += 1,
            RandomizedElementTreeMutation::Removed {
                removed_element_count,
                ..
            } => {
                assert_eq!(removed_element_count, 1, "leaf removal removes one element");
                removed += 1;
            }
            other => panic!("structural mutation produced {other:?}"),
        }
        insert = !insert;
    });
    assert!(frames > 0);
    assert_eq!(inserted + removed, frames);
    assert!(inserted.abs_diff(removed) <= 1);
}

/// A descendant moves among its siblings each frame; the set of elements is unchanged
/// but their order, and therefore every sibling's position, is not.
#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "RandomizedTree/reorder", fps = 120)]
fn reorder(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = &family.sample(&mut rng);
    let mut reordered = 0usize;
    let frames = measure("reorder", input, cx, |tree, cx| {
        if let RandomizedElementTreeMutation::Reordered { .. } =
            tree.apply_mutation(RandomizedElementTreeMutationKind::Reorder, cx)
        {
            reordered += 1;
        }
    });
    assert!(frames > 0);
    // A `Deep` tree has no parent with two children until an insert creates one, so the
    // harness inserts instead; every other shape reorders on every frame.
    if input.config.topology() != RandomizedElementTreeTopology::Deep {
        assert_eq!(reordered, frames);
    }
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

fn cached_input(family: &TreeFamily, rng: &mut StdRng) -> TreeInput {
    let mut input = family.sample(rng);
    input.config = input.config.with_cached_views(true);
    input
}

fn verify_tree_reuse(config: RandomizedElementTreeConfig, cx: &mut BenchAppContext) {
    let mut reused_window = cx.add_empty_window();
    let mut fresh_window = cx.add_empty_window();
    let reused = reused_window.update(|window, cx| {
        window.set_force_view_reconstruction(false);
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
    let fresh = fresh_window.update(|window, cx| {
        window.set_force_view_reconstruction(true);
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
    for frame in 0..16 {
        if frame > 0 {
            for root in [&reused, &fresh] {
                root.update(cx, |tree, cx| {
                    let kind = match frame % 6 {
                        0 => {
                            cx.notify();
                            return;
                        }
                        1 => RandomizedElementTreeMutationKind::ChildColor,
                        2 => RandomizedElementTreeMutationKind::ChildBounds,
                        3 => RandomizedElementTreeMutationKind::Insert,
                        4 => RandomizedElementTreeMutationKind::RemoveLeaf,
                        _ => RandomizedElementTreeMutationKind::Reorder,
                    };
                    tree.apply_mutation(kind, cx);
                });
            }
        }
        cx.run_until_idle();
        let reused_snapshot = reused_window.update(|window, cx| {
            window.view_reuse_snapshot(|id| {
                if id == reused.entity_id() {
                    "root".into()
                } else {
                    format!(
                        "element-{}",
                        reused
                            .read(cx)
                            .view_element_identity(id)
                            .expect("fixture view")
                    )
                }
            })
        });
        let fresh_snapshot = fresh_window.update(|window, cx| {
            window.view_reuse_snapshot(|id| {
                if id == fresh.entity_id() {
                    "root".into()
                } else {
                    format!(
                        "element-{}",
                        fresh
                            .read(cx)
                            .view_element_identity(id)
                            .expect("fixture view")
                    )
                }
            })
        });
        reused_snapshot
            .compare(&fresh_snapshot, frame)
            .expect("differential tree output");
    }
    reused_window.update(|window, _| window.remove_window());
    fresh_window.update(|window, _| window.remove_window());
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/full refresh", fps = 120)]
fn cached_full_refresh(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let measured = measure_in_window("cached full refresh", &input, cx, |_, window, _| {
        window.refresh()
    });
    assert!(measured.frames > 0);
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/leaf style", fps = 120)]
fn cached_leaf_style(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let frames = measure("cached leaf style", &input, cx, |tree, cx| {
        tree.recolor_first_leaf(cx)
            .expect("nonempty benchmark tree");
    });
    assert!(frames > 0);
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/leaf bounds", fps = 120)]
fn cached_leaf_bounds(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let frames = measure("cached leaf bounds", &input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
    assert!(frames > 0);
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/insert-remove", fps = 120)]
fn cached_insert_remove(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let mut insert = true;
    let frames = measure("cached insert-remove", &input, cx, |tree, cx| {
        tree.apply_mutation(
            if insert {
                RandomizedElementTreeMutationKind::Insert
            } else {
                RandomizedElementTreeMutationKind::RemoveLeaf
            },
            cx,
        );
        insert = !insert;
    });
    assert!(frames > 0);
}

#[gpui::bench(inputs = families(), input_name = "tree", iterations = 2, group = "CachedTree/reorder", fps = 120)]
fn cached_reorder(family: &TreeFamily, mut rng: StdRng, cx: &mut BenchAppContext) {
    let input = cached_input(family, &mut rng);
    let frames = measure("cached reorder", &input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::Reorder, cx);
    });
    assert!(frames > 0);
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
    let mut last = RecoloredChildren {
        recolored: 0,
        notified: 0,
    };
    let group = format!(
        "changing share f{}-{}",
        input.share_percent,
        input.locality_name()
    );
    let frames = measure(&group, tree, cx, |tree, cx| {
        if per_frame == 0 {
            cx.notify();
        } else {
            last = tree.recolor_children(per_frame, input.locality, cx);
        }
    });
    assert!(frames > 0);
    if per_frame > 0 {
        assert_eq!(last.recolored, per_frame.min(element_count));
        assert!(last.notified >= 1);
    }
}

struct TextPanel {
    index: usize,
    value: u32,
    renders: Rc<Cell<usize>>,
}

impl Render for TextPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
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
    renders: Rc<Cell<usize>>,
    font_family: gpui::SharedString,
}

impl TextPanelRoot {
    fn new(
        panel_count: usize,
        font_family: &'static str,
        root_renders: &Rc<Cell<usize>>,
        panel_renders: &Rc<Cell<usize>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            panels: (0..panel_count)
                .map(|index| {
                    cx.new(|_| TextPanel {
                        index,
                        value: 0,
                        renders: panel_renders.clone(),
                    })
                })
                .collect(),
            renders: root_renders.clone(),
            font_family: font_family.into(),
        }
    }
}

impl Render for TextPanelRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
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

struct CachedTextPanelRoot(TextPanelRoot, bool);

impl Render for CachedTextPanelRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .id("text-panels")
            .flex()
            .flex_wrap()
            .font_family(self.0.font_family.clone())
            .text_size(gpui::px(12.))
            .text_color(gpui::rgb(0xffffff))
            .child("0123456789")
            .children(self.0.panels.iter().cloned().map(|panel| {
                let mut style = gpui::StyleRefinement::default();
                style.size.width = Some(gpui::px(220.).into());
                style.size.height = Some(gpui::px(192.).into());
                if self.1 {
                    panel.cached(style).into_any_element()
                } else {
                    panel.into_any_element()
                }
            }))
    }
}

fn verify_text_reuse(panel_count: usize, font_family: &'static str, cx: &mut BenchAppContext) {
    let mut reused_window = cx.add_empty_window();
    let mut fresh_window = cx.add_empty_window();
    let create = |window: &mut Window, cx: &mut gpui::App, force| {
        window.set_force_view_reconstruction(force);
        window.set_view_retention(!force);
        window.replace_root(cx, |_, cx| {
            CachedTextPanelRoot(
                TextPanelRoot {
                    panels: (0..panel_count)
                        .map(|index| {
                            cx.new(|_| TextPanel {
                                index,
                                value: 0,
                                renders: Rc::new(Cell::new(0)),
                            })
                        })
                        .collect(),
                    renders: Rc::new(Cell::new(0)),
                    font_family: font_family.into(),
                },
                false,
            )
        })
    };
    let reused = reused_window.update(|window, cx| create(window, cx, false));
    let fresh = fresh_window.update(|window, cx| create(window, cx, true));
    for frame in 0..16 {
        if frame > 0 {
            for root in [&reused, &fresh] {
                root.update(cx, |root, cx| {
                    for index in [0, panel_count / 2] {
                        root.0.panels.get(index).expect("configured panel").update(
                            cx,
                            |panel, cx| {
                                panel.value += 1;
                                cx.notify();
                            },
                        );
                    }
                });
            }
        }
        cx.run_until_idle();
        let capture = |window: &mut gpui::BenchWindowContext,
                       root: &Entity<CachedTextPanelRoot>| {
            window.update(|window, cx| {
                window.view_reuse_snapshot(|id| {
                    if id == root.entity_id() {
                        "root".into()
                    } else {
                        let index = root
                            .read(cx)
                            .0
                            .panels
                            .iter()
                            .position(|panel| panel.entity_id() == id)
                            .expect("fixture panel");
                        format!("panel-{index}")
                    }
                })
            })
        };
        capture(&mut reused_window, &reused)
            .compare(&capture(&mut fresh_window, &fresh), frame)
            .expect("differential text output");
    }
    reused_window.update(|window, _| window.remove_window());
    fresh_window.update(|window, _| window.remove_window());
}

fn text_panel_counts() -> Vec<usize> {
    assert!(
        !std::env::var("GPUI_RETAINED_DISPATCH").is_ok_and(|value| value == "1"),
        "this mainline text benchmark supports fresh rendering only; unset GPUI_RETAINED_DISPATCH"
    );
    if cfg!(all(
        target_os = "linux",
        not(any(feature = "x11", feature = "wayland"))
    )) {
        eprintln!("TextPanels requires x11 or wayland on Linux for production text shaping");
        Vec::new()
    } else {
        vec![8, 32, 96]
    }
}

#[gpui::bench(inputs = text_panel_counts(), input_name = "panels", group = "TextPanels/two updates", fps = 120)]
fn text_panel_updates(panel_count: &usize, cx: &mut BenchAppContext) {
    measure_text_panel_updates(*panel_count, "monospace", cx);
}

fn installed_font_panel_counts() -> Vec<usize> {
    if cfg!(target_os = "linux") {
        text_panel_counts()
    } else {
        Vec::new()
    }
}

#[gpui::bench(inputs = installed_font_panel_counts(), input_name = "panels", group = "TextPanels/installed font two updates", fps = 120)]
fn installed_font_panel_updates(panel_count: &usize, cx: &mut BenchAppContext) {
    let font = gpui::font("Noto Sans Mono");
    let text_system = cx.read(|cx| cx.text_system().clone());
    assert!(
        text_system
            .all_font_names()
            .iter()
            .any(|family| family == font.family.as_ref()),
        "the installed-font comparison requires Noto Sans Mono"
    );
    let font_id = text_system.resolve_font(&font);
    assert_eq!(
        text_system.get_font_for_id(font_id).map(|font| font.family),
        Some(font.family),
        "the comparison font must resolve without falling back"
    );
    measure_text_panel_updates(*panel_count, "Noto Sans Mono", cx);
}

thread_local! {
    static LAST_TEXT_PANELS: Cell<Option<(usize, &'static str)>> = const { Cell::new(None) };
}

fn measure_text_panel_updates(
    panel_count: usize,
    font_family: &'static str,
    cx: &mut BenchAppContext,
) {
    if std::env::var("GPUI_VERIFY_REUSE_DIFFERENTIAL").is_ok_and(|value| value == "1") {
        verify_text_reuse(panel_count, font_family, cx);
    }
    let mut window = cx.add_empty_window();
    let root_renders = Rc::new(Cell::new(0));
    let panel_renders = Rc::new(Cell::new(0));
    let root = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| {
            TextPanelRoot::new(panel_count, font_family, &root_renders, &panel_renders, cx)
        })
    });
    cx.run_until_idle();
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    window.update(|window, _| {
        assert_layout_cache_mode(window);
        let scene = window.rendered_scene();
        let glyph_count = scene.monochrome_sprites.len()
            + scene.subpixel_sprites.len()
            + scene.polychrome_sprites.len();
        assert!(
            glyph_count > 0,
            "the text workload must produce glyph sprites: viewport {:?}, quads {}, root renders {}, panel renders {}",
            window.viewport_size(),
            scene.quads.len(),
            root_renders.get(),
            panel_renders.get()
        );
        if LAST_TEXT_PANELS.with(|last| last.replace(Some((panel_count, font_family))))
            != Some((panel_count, font_family))
        {
            eprintln!(
                "text panels: {panel_count}, requested family {font_family}, viewport {:?}, quads {}, glyphs {glyph_count}, initial root renders {}, panel renders {}",
                window.viewport_size(),
                scene.quads.len(),
                root_renders.get(),
                panel_renders.get()
            );
        }
    });
    let initial_panel_renders = panel_renders.get();
    let mut updates = 0usize;
    cx.bench_renderer(root, |root, _, cx| {
        for index in [0, panel_count / 2] {
            root.panels
                .get(index)
                .expect("configured panel")
                .update(cx, |panel, cx| {
                    panel.value = (panel.value + 1) % 1_000_000;
                    cx.notify();
                });
        }
        updates += 1;
    });
    window.update(|window, _| report_scene_stats("TextPanels", &panel_count.to_string(), window));
    assert!(updates > 0);
    assert!(
        panel_renders.get() > initial_panel_renders,
        "updated panels must render"
    );
    let incremental = window.update(|window, _| snapshot_glyphs(window));
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    assert_eq!(
        incremental,
        window.update(|window, _| snapshot_glyphs(window)),
        "updated glyph output must match a full redraw of the final model"
    );
}

fn assert_layout_cache_mode(window: &Window) {
    assert_eq!(
        window.layout_cache_enabled(),
        std::env::var("GPUI_LAYOUT_CACHE").as_deref() == Ok("1")
            || std::env::var("GPUI_RETAIN_VIEWS").as_deref() == Ok("1"),
        "the benchmark must exercise the requested layout-cache mode"
    );
}

#[derive(Debug, PartialEq)]
enum GlyphAppearance {
    Monochrome(gpui::Hsla, gpui::TransformationMatrix),
    Subpixel(gpui::Hsla, gpui::TransformationMatrix),
    Polychrome(gpui::PaddedBool32, f32, gpui::Corners<gpui::ScaledPixels>),
}

#[derive(Debug, PartialEq)]
struct GlyphSnapshot {
    order: gpui::DrawOrder,
    pad: u32,
    bounds: gpui::Bounds<gpui::ScaledPixels>,
    content_mask: gpui::ContentMask<gpui::ScaledPixels>,
    tile: gpui::AtlasTile,
    appearance: GlyphAppearance,
}

fn snapshot_glyphs(window: &Window) -> Vec<GlyphSnapshot> {
    let scene = window.rendered_scene();
    // Compare the same fields as the former Debug-string snapshots without formatting
    // thousands of floats and allocating one string per glyph after every sample.
    scene
        .monochrome_sprites
        .iter()
        .map(|sprite| GlyphSnapshot {
            order: sprite.order,
            pad: sprite.pad,
            bounds: sprite.bounds,
            content_mask: sprite.content_mask,
            tile: sprite.tile,
            appearance: GlyphAppearance::Monochrome(sprite.color, sprite.transformation),
        })
        .chain(scene.subpixel_sprites.iter().map(|sprite| GlyphSnapshot {
            order: sprite.order,
            pad: sprite.pad,
            bounds: sprite.bounds,
            content_mask: sprite.content_mask,
            tile: sprite.tile,
            appearance: GlyphAppearance::Subpixel(sprite.color, sprite.transformation),
        }))
        .chain(scene.polychrome_sprites.iter().map(|sprite| GlyphSnapshot {
            order: sprite.order,
            pad: sprite.pad,
            bounds: sprite.bounds,
            content_mask: sprite.content_mask,
            tile: sprite.tile,
            appearance: GlyphAppearance::Polychrome(
                sprite.grayscale,
                sprite.opacity,
                sprite.corner_radii,
            ),
        }))
        .collect()
}

struct FirstPopulatedFrame {
    window: gpui::AsyncWindowContext,
    root: Entity<TextPanelRoot>,
    panel_count: usize,
    root_renders: Rc<Cell<usize>>,
    panel_renders: Rc<Cell<usize>>,
    expected_frames: usize,
}

impl FirstPopulatedFrame {
    fn attach(
        window: &mut Window,
        cx: &mut gpui::App,
        panel_count: usize,
        root_renders: Rc<Cell<usize>>,
        panel_renders: Rc<Cell<usize>>,
    ) -> Self {
        let root = window.replace_root(cx, |_, cx| {
            TextPanelRoot::new(
                panel_count,
                "Noto Sans Mono",
                &root_renders,
                &panel_renders,
                cx,
            )
        });
        Self {
            window: window.to_async(cx),
            root,
            panel_count,
            root_renders,
            panel_renders,
            expected_frames: 1,
        }
    }

    fn validate(&mut self) {
        assert_eq!(self.root_renders.get(), self.expected_frames);
        assert_eq!(
            self.panel_renders.get(),
            self.panel_count * self.expected_frames
        );
        self.window
            .update(|window, cx| {
                assert_layout_cache_mode(window);
                let root = self.root.read(cx);
                assert_eq!(root.panels.len(), self.panel_count);
                assert!(root.panels.iter().all(|panel| panel.read(cx).value == 0));
                let scene = window.rendered_scene();
                assert!(
                    scene.monochrome_sprites.len()
                        + scene.subpixel_sprites.len()
                        + scene.polychrome_sprites.len()
                        > 0,
                    "the first populated frame must paint glyphs"
                );
            })
            .expect("first populated window must remain open");
    }
}

impl Drop for FirstPopulatedFrame {
    fn drop(&mut self) {
        // Criterion drops the task output after stopping measurement. Keep the
        // root and window alive until then, including validation and teardown.
        self.validate();
        self.window
            .update(|window, _| window.remove_window())
            .expect("remove first populated window");
    }
}

thread_local! {
    static DESCRIBED_FIRST_POPULATED: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

fn first_populated_panel_counts() -> Vec<usize> {
    for variable in ["GPUI_RETAINED_DISPATCH", "GPUI_VIEW_RETENTION"] {
        assert!(
            std::env::var_os(variable).is_none(),
            "first populated fresh-rendering measurements require {variable} unset"
        );
    }
    installed_font_panel_counts()
}

#[gpui::bench(inputs = first_populated_panel_counts(), input_name = "panels", group = "TextPanels/first populated installed font", fps = 120)]
fn first_populated_text_panels(panel_count: &usize, cx: &mut BenchAppContext) {
    let panel_count = *panel_count;
    let font = gpui::font("Noto Sans Mono");
    let text_system = cx.read(|cx| cx.text_system().clone());
    assert!(
        text_system
            .all_font_names()
            .iter()
            .any(|family| family == font.family.as_ref()),
        "the first populated comparison requires Noto Sans Mono"
    );
    let font_id = text_system.resolve_font(&font);
    assert_eq!(
        text_system.get_font_for_id(font_id).map(|font| font.family),
        Some(font.family),
        "the first populated font must resolve without falling back"
    );

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
        // Sample before validation snapshots/logging allocate. Font resolution
        // above is intentionally shared setup, not window-held memory.
        let before_window = bench_metrics::allocation_stats();
        let mut window = cx.add_empty_window();
        let before_population = bench_metrics::allocation_stats();
        let mut populated = window.update(|window, cx| {
            FirstPopulatedFrame::attach(
                window,
                cx,
                panel_count,
                Rc::new(Cell::new(0)),
                Rc::new(Cell::new(0)),
            )
        });
        cx.run_until_idle();
        let after_population = bench_metrics::allocation_stats();
        populated.validate();
        window.update(|window, _| {
            let scene = window.rendered_scene();
            eprintln!(
                "first populated panels: {panel_count}, family Noto Sans Mono, viewport {:?}, quads {}, glyphs {}, root renders {}, panel renders {}; GPUI_RETAINED_DISPATCH={:?}, GPUI_VIEW_RETENTION={:?}; requested GPUI_LAYOUT_CACHE={:?}, actual layout cache {}",
                window.viewport_size(),
                scene.quads.len(),
                scene.monochrome_sprites.len() + scene.subpixel_sprites.len() + scene.polychrome_sprites.len(),
                populated.root_renders.get(),
                populated.panel_renders.get(),
                std::env::var_os("GPUI_RETAINED_DISPATCH"),
                std::env::var_os("GPUI_VIEW_RETENTION"),
                std::env::var_os("GPUI_LAYOUT_CACHE"),
                window.layout_cache_enabled(),
            );
        });
        match before_window.zip(before_population).zip(after_population) {
            Some(((before, empty), after)) => eprintln!(
                "first populated setup live heap: before window {} B, empty window {} B, populated {} B; window+setup delta {} B, population delta {} B (process-wide estimates, not exclusive window ownership)",
                before.live_bytes(),
                empty.live_bytes(),
                after.live_bytes(),
                after.live_bytes() - before.live_bytes(),
                after.live_bytes() - empty.live_bytes(),
            ),
            None => eprintln!("first populated setup live heap: allocator statistics unavailable"),
        }
        window.update(|window, _| {
            report_scene_stats(
                "TextPanels first populated",
                &panel_count.to_string(),
                window,
            );
        });
        let initial = window.update(|window, _| snapshot_glyphs(window));
        window.update(|window, _| window.refresh());
        cx.run_until_idle();
        populated.expected_frames = 2;
        assert_eq!(populated.root_renders.get(), 2);
        assert_eq!(populated.panel_renders.get(), panel_count * 2);
        assert_eq!(
            initial,
            window.update(|window, _| snapshot_glyphs(window)),
            "first populated glyph output must match a full redraw"
        );
        drop(populated);
        cx.run_until_idle();
    }

    cx.bench_batched_task(
        |cx| {
            (
                cx.add_empty_window(),
                Rc::new(Cell::new(0)),
                Rc::new(Cell::new(0)),
            )
        },
        |(window, root_renders, panel_renders), _| {
            let populated = window.update(|window, cx| {
                FirstPopulatedFrame::attach(
                    window,
                    cx,
                    panel_count,
                    root_renders.clone(),
                    panel_renders.clone(),
                )
            });
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
    renders: Rc<Cell<usize>>,
}

impl Render for ChurnPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
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
    renders: usize,
    expected_panel_renders: usize,
    panel_renders: Rc<Cell<usize>>,
}

impl ChurnRoot {
    fn new(workload: Churn, cx: &mut Context<Self>) -> Self {
        let panel_renders = Rc::new(Cell::new(0));
        Self {
            panels: (0..96)
                .map(|index| {
                    cx.new(|_| ChurnPanel {
                        index,
                        frame: 0,
                        workload,
                        renders: panel_renders.clone(),
                    })
                })
                .collect(),
            workload,
            frame: 0,
            renders: 0,
            expected_panel_renders: 96,
            panel_renders,
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
                    let previous = self.panels.get_mut(index).expect("replacement panel");
                    let replacement = cx.new(|_| ChurnPanel {
                        index,
                        frame: self.frame,
                        workload: self.workload,
                        renders: self.panel_renders.clone(),
                    });
                    assert_ne!(previous.entity_id(), replacement.entity_id());
                    *previous = replacement;
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
        self.expected_panel_renders += self.visible_panels();
        cx.notify();
    }

    fn validate(&self) {
        assert_eq!(self.renders, self.frame + 1);
        assert_eq!(self.panel_renders.get(), self.expected_panel_renders);
        assert_eq!(self.panels.len(), 96);
    }
}

impl Render for ChurnRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        gpui::div()
            .id("churn-root")
            .flex()
            .flex_wrap()
            .font_family("Noto Sans Mono")
            .text_size(gpui::px(12.))
            .text_color(gpui::rgb(0xffffff))
            .child("0123456789")
            .children(self.panels.iter().take(self.visible_panels()).cloned())
    }
}

fn churn_workloads() -> Vec<Churn> {
    if first_populated_panel_counts().is_empty() {
        Vec::new()
    } else {
        vec![Churn::Replace, Churn::Remount, Churn::Id, Churn::Text]
    }
}

thread_local! {
    static DESCRIBED_CHURN: RefCell<Vec<Churn>> = const { RefCell::new(Vec::new()) };
}

#[gpui::bench(inputs = churn_workloads(), input_name = "workload", group = "LayoutChurn", fps = 120)]
fn layout_churn(workload: &Churn, cx: &mut BenchAppContext) {
    let font = gpui::font("Noto Sans Mono");
    let text_system = cx.read(|cx| cx.text_system().clone());
    assert!(
        text_system
            .all_font_names()
            .contains(&font.family.to_string())
    );
    let font_id = text_system.resolve_font(&font);
    assert_eq!(
        text_system.get_font_for_id(font_id).map(|font| font.family),
        Some(font.family)
    );
    let describe = DESCRIBED_CHURN.with(|described| {
        let mut described = described.borrow_mut();
        if described.contains(workload) {
            false
        } else {
            described.push(*workload);
            true
        }
    });
    if describe && std::env::var("GPUI_BENCH_HEAP").as_deref() == Ok("1") {
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
            window.update(|_, cx| root.read(cx).validate());
        }
        // Logging and scene snapshots allocate, so defer both until all heap reads.
        window.update(|window, cx| {
            assert_layout_cache_mode(window);
            root.read(cx).validate();
            assert!(!snapshot_glyphs(window).is_empty());
        });
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
    }
    let mut window = cx.add_empty_window();
    let root =
        window.update(|window, cx| window.replace_root(cx, |_, cx| ChurnRoot::new(*workload, cx)));
    cx.run_until_idle();
    cx.bench_renderer(root.clone(), |root, _, cx| root.advance(cx));
    window.update(|window, cx| {
        assert_layout_cache_mode(window);
        root.read(cx).validate();
        assert!(root.read(cx).frame > 0);
    });
    let incremental = window.update(|window, _| snapshot_glyphs(window));
    window.update(|window, _| window.refresh());
    cx.run_until_idle();
    assert!(!incremental.is_empty());
    assert_eq!(
        incremental,
        window.update(|window, _| snapshot_glyphs(window))
    );
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
    family_summary
);
gpui::bench_main!(benches);
