//! Frame cost of a seeded element tree under one class of change per frame.
//!
//! Each benchmark builds a [`RandomizedElementTree`] of a fixed shape before timing, then
//! measures frames in which exactly one thing happens: nothing (a notified root that
//! re-renders an unchanged tree), a leaf's style, a nested entity's subtree, the root's
//! layout, or one structural change. Changing one dimension at a time — topology, element
//! count, share of elements backed by their own entity — keeps the scaling readable, and
//! the same fixture runs on any GPUI revision, so it doubles as the before/after harness
//! for changes to rendering.
//!
//! The tree's own work counters are asserted after each measured loop: a faster frame that
//! stopped rendering the changed element is not an improvement.

use std::fmt;

use gpui::{
    BenchAppContext, Context,
    randomized_element_tree::{
        RandomizedElementTree, RandomizedElementTreeConfig, RandomizedElementTreeMutation,
        RandomizedElementTreeMutationKind, RandomizedElementTreeTopology,
    },
};

/// One tree shape: how the elements are arranged, how many there are, and what share of
/// them render through a persistent child entity rather than inline.
#[derive(Clone, Copy)]
struct TreeShape {
    topology: RandomizedElementTreeTopology,
    element_count: usize,
    entity_density: f64,
}

impl fmt::Display for TreeShape {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let topology = match self.topology {
            RandomizedElementTreeTopology::Wide => "wide",
            RandomizedElementTreeTopology::Deep => "deep",
            RandomizedElementTreeTopology::Mixed => "mixed",
            RandomizedElementTreeTopology::Narrow => "narrow",
        };
        write!(
            formatter,
            "{topology}-e{}-entities{}",
            self.element_count,
            (self.entity_density * 100.0).round() as usize
        )
    }
}

const SEED: u64 = 7;
const HANDLER_DENSITY: f64 = 0.25;

/// Element counts scale by 4× so the step from a comfortable frame to a missed 120 Hz
/// budget is visible; `Deep` stops at 256 because a 1024-deep chain overflows the stack
/// without gpui's `stacker` feature and is not a shape real UI takes.
fn shapes() -> Vec<TreeShape> {
    let mut shapes = Vec::new();
    for topology in [
        RandomizedElementTreeTopology::Wide,
        RandomizedElementTreeTopology::Mixed,
        RandomizedElementTreeTopology::Deep,
    ] {
        let counts: &[usize] = if topology == RandomizedElementTreeTopology::Deep {
            &[64, 256]
        } else {
            &[64, 256, 1024]
        };
        for &element_count in counts {
            for entity_density in [0.0, 0.25] {
                shapes.push(TreeShape {
                    topology,
                    element_count,
                    entity_density,
                });
            }
        }
    }
    shapes
}

fn config(shape: TreeShape) -> RandomizedElementTreeConfig {
    RandomizedElementTreeConfig::new(SEED, shape.element_count)
        .with_topology(shape.topology)
        .with_entity_density(shape.entity_density)
        .with_handler_density(HANDLER_DENSITY)
}

/// Builds the tree, draws it once so layout and the scene are warm, and measures
/// `mutate` once per frame. Returns how many frames were measured, after checking that
/// the last frame did render something: a frame that skipped the changed element would
/// be fast and wrong.
fn measure(
    shape: TreeShape,
    cx: &mut BenchAppContext,
    mut mutate: impl FnMut(&mut RandomizedElementTree, &mut Context<RandomizedElementTree>),
) -> usize {
    let config = config(shape);
    let mut window = cx.add_empty_window();
    let tree = window.update(|window, cx| {
        window.replace_root(cx, |_, cx| {
            RandomizedElementTree::new_with_config(config, cx)
        })
    });
    cx.run_until_idle();

    let snapshot = tree.read_with(cx, |tree, _| tree.snapshot());
    assert_eq!(snapshot.descendant_count(), shape.element_count);
    let initial = tree.read_with(cx, |tree, _| tree.work_counters());
    assert!(
        initial.root_render_count() >= 1,
        "the tree must have drawn before timing"
    );
    assert!(
        initial.element_render_count() >= shape.element_count,
        "every generated element renders in the first frame"
    );

    let mut frames = 0;
    cx.bench_renderer(tree.clone(), |tree, _window, cx| {
        tree.reset_work_counters();
        mutate(tree, cx);
        frames += 1;
    });

    let last_frame = tree.read_with(cx, |tree, _| tree.work_counters());
    assert!(
        last_frame.root_render_count() + last_frame.entity_render_count() >= 1,
        "the notified root or entity re-rendered in the last measured frame"
    );
    assert!(last_frame.element_render_count() >= 1);
    frames
}

/// A frame in which the root is notified but nothing in the tree changed. On a renderer
/// that rebuilds every frame this is the whole tree's cost; on one that retains output
/// it is the floor for a frame that has to do nothing.
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/unchanged", fps = 120)]
fn unchanged(shape: &TreeShape, cx: &mut BenchAppContext) {
    let frames = measure(*shape, cx, |_, cx| cx.notify());
    assert!(frames > 0);
}

/// One descendant's background color changes each frame: the smallest possible change,
/// confined to one element and, when it has one, its owning entity.
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/leaf style", fps = 120)]
fn leaf_style(shape: &TreeShape, cx: &mut BenchAppContext) {
    let mut recolored = 0usize;
    let frames = measure(*shape, cx, |tree, cx| {
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
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/leaf bounds", fps = 120)]
fn leaf_bounds(shape: &TreeShape, cx: &mut BenchAppContext) {
    let frames = measure(*shape, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::ChildBounds, cx);
    });
    assert!(frames > 0);
}

/// The root's width and height change each frame: every element's layout is stale.
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/root layout", fps = 120)]
fn root_layout(shape: &TreeShape, cx: &mut BenchAppContext) {
    let frames = measure(*shape, cx, |tree, cx| {
        tree.apply_mutation(RandomizedElementTreeMutationKind::RootBounds, cx);
    });
    assert!(frames > 0);
}

/// One element is inserted or removed per frame, alternating so the tree keeps its size
/// to within one element over the whole loop. Removal takes a leaf, never a subtree.
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/insert-remove", fps = 120)]
fn insert_remove(shape: &TreeShape, cx: &mut BenchAppContext) {
    let mut insert = true;
    let mut inserted = 0usize;
    let mut removed = 0usize;
    let frames = measure(*shape, cx, |tree, cx| {
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
#[gpui::bench(inputs = shapes(), input_name = "tree", group = "RandomizedTree/reorder", fps = 120)]
fn reorder(shape: &TreeShape, cx: &mut BenchAppContext) {
    let mut reordered = 0usize;
    let frames = measure(*shape, cx, |tree, cx| {
        if let RandomizedElementTreeMutation::Reordered { .. } =
            tree.apply_mutation(RandomizedElementTreeMutationKind::Reorder, cx)
        {
            reordered += 1;
        }
    });
    assert!(frames > 0);
    // A `Deep` tree has no parent with two children until an insert creates one, so the
    // harness inserts instead; every other shape reorders on every frame.
    if shape.topology != RandomizedElementTreeTopology::Deep {
        assert_eq!(reordered, frames);
    }
}

gpui::bench_group!(
    benches,
    unchanged,
    leaf_style,
    leaf_bounds,
    root_layout,
    insert_remove,
    reorder
);
gpui::bench_main!(benches);
