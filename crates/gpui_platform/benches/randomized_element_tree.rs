//! Frame cost of a seeded element tree under one class of change per frame.
//!
//! Each benchmark builds a [`RandomizedElementTree`] before timing, then measures frames in
//! which exactly one thing happens: nothing (a notified root that re-renders an unchanged
//! tree), a leaf's style, a leaf's bounds, the root's layout, one structural change, or a
//! sibling reorder. The same fixture runs on any GPUI revision, so it doubles as the
//! before/after harness for changes to rendering.
//!
//! Trees come from *families*: bounds on topology, element count and entity density that
//! a seed is drawn against (see [`RandomizedElementTreeBounds`]). Every input is one
//! `family-s<seed>` and its whole shape follows from the seed, so a run over a family's
//! seeds is a sample of that family and any single input reproduces exactly. Filter by
//! family to ask a narrower question, e.g. `--bench randomized_element_tree -- 'tall-'`.
//!
//! The tree's own work counters are asserted after each measured loop: a faster frame that
//! stopped rendering the changed element is not an improvement.

use std::fmt;

use gpui::{
    BenchAppContext, Context, Window,
    randomized_element_tree::{
        ChangeLocality, RandomizedElementTree, RandomizedElementTreeBounds,
        RandomizedElementTreeConfig, RandomizedElementTreeMutation,
        RandomizedElementTreeMutationKind, RandomizedElementTreeTopology, RecoloredChildren,
    },
};

/// How many seeds each family is sampled at. Criterion reports each seed on its own;
/// the family's average is read across them.
const SEEDS_PER_FAMILY: u64 = 6;

/// A named family of trees, and one seed's draw from it. `seed` counts within the family;
/// the config's seed is offset per family so families with overlapping bounds do not
/// draw the same trees.
#[derive(Clone)]
struct TreeInput {
    family: &'static str,
    seed: u64,
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
            "{}-s{}-{topology}-e{}-ent{}",
            self.family,
            self.seed,
            self.config.element_count(),
            (self.config.entity_density() * 100.0).round() as usize
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
fn families() -> Vec<(&'static str, RandomizedElementTreeBounds)> {
    let handlers = 0.1..=0.4;
    vec![
        (
            "any",
            RandomizedElementTreeBounds::new(32..=TALL_MAX_ELEMENTS)
                .with_entity_density(MIN_ENTITY_DENSITY..=0.5)
                .with_handler_density(handlers.clone()),
        ),
        (
            "wide",
            RandomizedElementTreeBounds::new(128..=2048)
                .with_topologies([RandomizedElementTreeTopology::Wide])
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        ),
        (
            "tall",
            RandomizedElementTreeBounds::new(32..=TALL_MAX_ELEMENTS)
                .with_topologies(
                    RandomizedElementTreeTopology::ALL
                        .into_iter()
                        .filter(|topology| topology.is_tall()),
                )
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        ),
        (
            "mixed",
            RandomizedElementTreeBounds::new(128..=1024)
                .with_topologies([RandomizedElementTreeTopology::Mixed])
                .with_entity_density(MIN_ENTITY_DENSITY..=0.25)
                .with_handler_density(handlers.clone()),
        ),
        (
            "dense-entities",
            RandomizedElementTreeBounds::new(128..=1024)
                .with_topologies([
                    RandomizedElementTreeTopology::Wide,
                    RandomizedElementTreeTopology::Mixed,
                ])
                .with_entity_density(0.75..=1.0)
                .with_handler_density(handlers),
        ),
    ]
}

fn inputs() -> Vec<TreeInput> {
    families()
        .into_iter()
        .enumerate()
        .flat_map(|(family_index, (family, bounds))| {
            (0..SEEDS_PER_FAMILY).map(move |seed| TreeInput {
                family,
                seed,
                config: bounds.sample((family_index as u64) << 32 | seed),
            })
        })
        .collect()
}

/// Builds the tree, draws it once so layout and the scene are warm, and measures
/// `mutate` once per frame. Returns how many frames were measured, after checking that
/// the frames did render something: a frame that skipped the changed element would be
/// fast and wrong.
fn measure(
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Context<RandomizedElementTree>),
) -> usize {
    measure_in_window(input, cx, |tree, _, cx| mutate(tree, cx)).frames
}

/// What a measured loop did: frames measured, and the entity renders the frames showed.
struct Measured {
    frames: usize,
    entity_renders: usize,
}

fn measure_in_window(
    input: &TreeInput,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Window, &mut Context<RandomizedElementTree>),
) -> Measured {
    let config = input.config;
    let mut window = cx.add_empty_window();
    let tree = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
    cx.run_until_idle();

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
    cx.bench_renderer(tree, |tree, window, cx| {
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
    Measured {
        frames,
        entity_renders,
    }
}

/// A frame that rebuilds everything: `Window::refresh`, the fallback GPUI uses whenever
/// it cannot tell what changed (window activation, theme or settings changes via
/// `refresh_windows`, an inspector toggle). Nothing in the tree changes, so this is the
/// pure cost of the known-good path, and the renderer's worst case: every view and element
/// rendered again, none reused. Its difference between two GPUI revisions is the price of
/// a full refresh on the newer one.
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/full refresh", fps = 120)]
fn full_refresh(input: &TreeInput, cx: &mut BenchAppContext) {
    let measured = measure_in_window(input, cx, |_, window, _| window.refresh());
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
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/unchanged", fps = 120)]
fn unchanged(input: &TreeInput, cx: &mut BenchAppContext) {
    let frames = measure(input, cx, |_, cx| cx.notify());
    assert!(frames > 0);
}

/// One descendant's background color changes each frame: the smallest possible change,
/// confined to one element and, when it has one, its owning entity.
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/leaf style", fps = 120)]
fn leaf_style(input: &TreeInput, cx: &mut BenchAppContext) {
    let mut recolored = 0usize;
    let frames = measure(input, cx, |tree, cx| {
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
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/leaf bounds", fps = 120)]
fn leaf_bounds(input: &TreeInput, cx: &mut BenchAppContext) {
    let frames = measure(input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
    assert!(frames > 0);
}

/// The root's width and height change each frame: every element's layout is stale.
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/root layout", fps = 120)]
fn root_layout(input: &TreeInput, cx: &mut BenchAppContext) {
    let frames = measure(input, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::RootBounds, cx);
    });
    assert!(frames > 0);
}

/// One element is inserted or removed per frame, alternating so the tree keeps its size
/// to within one element over the whole loop. Removal takes a leaf, never a subtree.
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/insert-remove", fps = 120)]
fn insert_remove(input: &TreeInput, cx: &mut BenchAppContext) {
    let mut insert = true;
    let mut inserted = 0usize;
    let mut removed = 0usize;
    let frames = measure(input, cx, |tree, cx| {
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
#[gpui::bench(inputs = inputs(), input_name = "tree", group = "RandomizedTree/reorder", fps = 120)]
fn reorder(input: &TreeInput, cx: &mut BenchAppContext) {
    let mut reordered = 0usize;
    let frames = measure(input, cx, |tree, cx| {
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

/// A tree, the share of its elements that change each frame, and where those changes sit.
#[derive(Clone)]
struct ChangingShareInput {
    tree: TreeInput,
    share_percent: usize,
    locality: ChangeLocality,
}

impl fmt::Display for ChangingShareInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locality = match self.locality {
            ChangeLocality::Localized => "local",
            ChangeLocality::Spread => "spread",
        };
        write!(
            formatter,
            "{}-f{}-{locality}",
            self.tree, self.share_percent
        )
    }
}

/// Seeds per family in the `changing share` group, which multiplies every tree by five
/// shares; fewer seeds keep the group to a few minutes.
const CHANGING_SHARE_SEEDS: u64 = 3;

/// The share of elements recolored per frame, from one element to all of them. A renderer
/// that retains clean entity subtrees should fall toward the left of this axis; one that
/// redraws everything is flat across it, so the two curves' distance is what retention
/// buys at each share.
///
/// The main line keeps each frame's change inside one entity, as a user action does. Two
/// `spread` points scatter it uniformly instead: the pessimistic bound, where every
/// changed element may dirty a different subtree.
fn changing_share_inputs() -> Vec<ChangingShareInput> {
    inputs()
        .into_iter()
        .filter(|input| input.seed < CHANGING_SHARE_SEEDS)
        .flat_map(|tree| {
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
                tree: tree.clone(),
                share_percent,
                locality,
            })
        })
        .collect()
}

#[gpui::bench(inputs = changing_share_inputs(), input_name = "tree", group = "RandomizedTree/changing share", fps = 120)]
fn changing_share(input: &ChangingShareInput, cx: &mut BenchAppContext) {
    let element_count = input.tree.config.element_count();
    let per_frame = if input.share_percent == 0 {
        0
    } else {
        (input.share_percent * element_count).div_ceil(100).max(1)
    };
    let mut last = RecoloredChildren {
        recolored: 0,
        notified: 0,
    };
    let frames = measure(&input.tree, cx, |tree, cx| {
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

gpui::bench_group!(
    benches,
    full_refresh,
    unchanged,
    leaf_style,
    leaf_bounds,
    root_layout,
    insert_remove,
    reorder,
    changing_share
);
gpui::bench_main!(benches);
