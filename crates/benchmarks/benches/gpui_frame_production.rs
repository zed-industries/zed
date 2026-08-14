use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, AppContext as _, BenchAppContext, Context, Entity,
    InteractiveElement as _, IntoElement, ParentElement as _, Render, Styled as _, Window, div,
    prelude::FluentBuilder as _, px, rgba,
};

struct PaneLeaf {
    generation: usize,
}

impl Render for PaneLeaf {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(px(2.))
            .bg(rgba(0x334455ff + self.generation as u32))
    }
}

struct NestedPanes {
    leaf: Entity<PaneLeaf>,
    divider: f32,
}

impl Render for NestedPanes {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut tree = div().child(self.leaf.clone()).into_any_element();
        for depth in 0..500 {
            tree = div()
                .flex()
                .when(depth % 2 == 0, |element| element.flex_row())
                .when(depth % 2 != 0, |element| element.flex_col())
                .child(
                    div()
                        .w(px(1. + self.divider))
                        .h(px(1. + self.divider))
                        .bg(rgba(0x556677ff)),
                )
                .child(tree)
                .into_any_element();
        }
        tree
    }
}

fn nested_panes(cx: &mut BenchAppContext) -> (Entity<NestedPanes>, Entity<PaneLeaf>) {
    let mut window = cx.add_empty_window();
    window.update(|window, cx| {
        let leaf = cx.new(|_| PaneLeaf { generation: 0 });
        let root = window.replace_root(cx, |_, _| NestedPanes {
            leaf: leaf.clone(),
            divider: 0.,
        });
        (root, leaf)
    })
}

#[gpui::bench]
fn layout_leaf_notify(cx: &mut BenchAppContext) {
    let (root, leaf) = nested_panes(cx);
    cx.bench_renderer(root, move |_, _, cx| {
        leaf.update(cx, |leaf, cx| {
            leaf.generation = leaf.generation.wrapping_add(1);
            cx.notify();
        });
    });
}

#[gpui::bench(inputs = ["leaf_notify", "divider_drag"], group = "GPUI draw lane")]
fn draw_lane(variant: &&str, cx: &mut BenchAppContext) {
    let (root, leaf) = nested_panes(cx);
    let divider_drag = *variant == "divider_drag";
    cx.bench_renderer(root, move |root, _, cx| {
        if divider_drag {
            root.divider = (root.divider + 1.) % 4.;
            cx.notify();
        } else {
            leaf.update(cx, |leaf, cx| {
                leaf.generation = leaf.generation.wrapping_add(1);
                cx.notify();
            });
        }
    });
}

struct Spinner;

impl Render for Spinner {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().with_animation(
            "present-spinner",
            Animation::new(Duration::from_millis(800)).repeat_synced(),
            |element, delta| {
                element
                    .size(px(16.))
                    .opacity(0.25 + delta * 0.75)
                    .bg(rgba(0xff8800ff))
            },
        )
    }
}

#[gpui::bench]
fn present_spinner(cx: &mut BenchAppContext) {
    let mut window = cx.add_empty_window();
    let spinner = window.update(|window, cx| window.replace_root(cx, |_, _| Spinner));
    let scene_size = std::rc::Rc::new(std::cell::Cell::new(0));
    cx.bench_renderer(spinner, {
        let scene_size = scene_size.clone();
        move |_, window, cx| {
            window.refresh();
            cx.notify();
            scene_size.set(window.frame_snapshot().scene_len());
        }
    });
    eprintln!(
        "present spinner scene paint operations: {}",
        scene_size.get()
    );
}

struct StorybookBench {
    frame: usize,
}

impl Render for StorybookBench {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex().flex_col().children((0..50).map(|index| {
            div()
                .id(index)
                .h(px(8. + (index % 5) as f32))
                .bg(rgba(0x102030ff + (self.frame + index) as u32))
                .child(format!("storybook {index}"))
        }))
    }
}

#[gpui::bench(fps = 120)]
fn storybook_300_frames(cx: &mut BenchAppContext) {
    let mut window = cx.add_empty_window();
    let storybook =
        window.update(|window, cx| window.replace_root(cx, |_, _| StorybookBench { frame: 0 }));
    cx.bench_iter(move |cx| {
        for _ in 0..300 {
            storybook.update(cx, |storybook, cx| {
                storybook.frame = storybook.frame.wrapping_add(1);
                cx.notify();
            });
        }
    });
}

gpui::bench_group!(
    benches,
    layout_leaf_notify,
    draw_lane,
    present_spinner,
    storybook_300_frames
);
gpui::bench_main!(benches);
