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
    cell::RefCell,
    fmt,
    time::{Duration, Instant},
};

use gpui::{
    BenchAppContext, Context, Window,
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
    let before_window = bench_metrics::allocation_stats();
    let mut window = cx.add_empty_window();
    let tree = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
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
        if let RandomizedElementTreeMutation::ChildColor { .. } =
            tree.apply_mutation(RandomizedElementTreeMutationKind::ChildColor, cx)
        {
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
    unchanged,
    leaf_style,
    leaf_bounds,
    root_layout,
    insert_remove,
    reorder,
    changing_share,
    family_summary
);
gpui::bench_main!(benches);
