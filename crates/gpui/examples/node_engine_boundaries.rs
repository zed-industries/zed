use gpui::{
    AnyElement, App, Bounds, Context, Div, Element, ElementId, Entity, FocusHandle,
    GlobalElementId, InspectorElementId, LayoutId, Pixels, Stateful, Window, WindowBounds,
    WindowOptions, anchored, deferred, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;
use std::{cell::Cell, rc::Rc};

fn button(id: &'static str, label: impl Into<gpui::SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .px_3()
        .py_2()
        .rounded_md()
        .bg(rgb(0x334155))
        .cursor_pointer()
        .hover(|style| style.bg(rgb(0x475569)))
        .child(label.into())
}

#[derive(Default)]
struct PhaseCounts {
    layout_requests: Cell<usize>,
    prepaints: Cell<usize>,
    paints: Cell<usize>,
}

struct Trace {
    counts: Rc<PhaseCounts>,
    label: &'static str,
    child: AnyElement,
}

impl IntoElement for Trace {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl Element for Trace {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        self.counts
            .layout_requests
            .set(self.counts.layout_requests.get() + 1);
        println!(
            "{} request_layout #{}",
            self.label,
            self.counts.layout_requests.get()
        );
        (self.child.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.counts.prepaints.set(self.counts.prepaints.get() + 1);
        println!(
            "{} prepaint #{} {bounds:?}",
            self.label,
            self.counts.prepaints.get()
        );
        self.child.prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.counts.paints.set(self.counts.paints.get() + 1);
        println!("{} paint #{}", self.label, self.counts.paints.get());
        self.child.paint(window, cx);
    }
}

struct LocalState {
    phases: Rc<PhaseCounts>,
    count: usize,
    mount: usize,
    focus: FocusHandle,
}

struct CounterCard {
    label: &'static str,
    revision: usize,
    mounts: Rc<Cell<usize>>,
}

impl gpui::Component for CounterCard {
    fn render(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state("counter", cx, |_, cx| {
            let mount = self.mounts.get() + 1;
            self.mounts.set(mount);
            LocalState {
                phases: Rc::default(),
                count: 0,
                mount,
                focus: cx.focus_handle(),
            }
        });
        let local = state.read(cx);
        let count = local.count;
        let mount = local.mount;
        let focus = local.focus.clone();
        let phases = local.phases.clone();
        println!(
            "{} render: mount={mount}, count={count}, props={}",
            self.label, self.revision
        );
        let revision = self.revision;
        Trace {
            counts: phases,
            label: self.label,
            child: div()
                .id("card")
                .track_focus(&focus)
                .on_key_down({
                    let state = state.clone();
                    move |event, _, cx| {
                        if event.keystroke.key == "enter" {
                            println!("keyboard handler parent input = {revision}");
                            state.update(cx, |state, cx| {
                                state.count += 1;
                                cx.notify();
                            });
                            cx.stop_propagation();
                        }
                    }
                })
                .flex()
                .flex_col()
                .gap_2()
                .w_full()
                .p_4()
                .rounded_md()
                .bg(rgb(0x1e293b))
                .border_2()
                .border_color(rgb(0x475569))
                .focus(|style| style.border_color(rgb(0xfbbf24)))
                .child(format!("{} · mount {mount}", self.label))
                .child(format!("Local count {count} · parent input {revision}"))
                .child(
                    button("increment", "Increment local state / focus").on_click(
                        move |_, window, cx| {
                            window.focus(&focus, cx);
                            state.update(cx, |state, cx| {
                                state.count += 1;
                                cx.notify();
                            });
                        },
                    ),
                )
                .child(
                    div()
                        .id("hover")
                        .p_3()
                        .bg(rgb(0x164e63))
                        .hover(|style| style.bg(rgb(0x0e7490)))
                        .child("Hover: only this strip changes color"),
                )
                .child(
                    button("captured-input", "Print captured parent input")
                        .on_click(move |_, _, _| println!("handler parent input = {revision}")),
                )
                .into_any_element(),
        }
    }
}

struct Dependency {
    value: usize,
}

struct EntityLeaf {
    phases: Rc<PhaseCounts>,
    label: &'static str,
    dependency: Entity<Dependency>,
    renders: usize,
    clicks: usize,
}

impl Render for EntityLeaf {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        let value = self.dependency.read(cx).value;
        println!("{} render #{}", self.label, self.renders);
        Trace {
            counts: self.phases.clone(),
            label: self.label,
            child: div()
                .flex()
                .flex_col()
                .size_full()
                .p_3()
                .gap_2()
                .bg(rgb(0x312e81))
                .child(self.label)
                .child(format!("Dependency {value} · renders {}", self.renders))
                .child(
                    button(
                        "notify",
                        format!("Notify this entity · clicks {}", self.clicks),
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.clicks += 1;
                        cx.notify();
                    })),
                )
                .into_any_element(),
        }
    }
}

struct BoundaryLab {
    revision: usize,
    renders: usize,
    reversed: bool,
    show_first: bool,
    wide: bool,
    alternate_style: bool,
    overlay: bool,
    mounts: Rc<Cell<usize>>,
    dependency: Entity<Dependency>,
    leaves: [Entity<EntityLeaf>; 2],
}

impl Render for BoundaryLab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.renders += 1;
        println!("\nROOT render #{}", self.renders);
        let mut labels = vec!["A", "B"];
        if self.reversed {
            labels.reverse();
        }
        let cards = labels
            .into_iter()
            .filter(|label| self.show_first || *label != "A")
            .map(|label| {
                div()
                    .id(label)
                    .w(px(if label == "A" && self.wide {
                        430.
                    } else {
                        320.
                    }))
                    .flex_shrink_0()
                    .child(gpui::component(
                        "counter",
                        CounterCard {
                            label,
                            revision: self.revision,
                            mounts: self.mounts.clone(),
                        },
                    ))
            });
        div().id("lab").size_full().overflow_y_scroll().flex().flex_col().p_6().gap_4()
            .bg(rgb(0x0f172a)).text_color(rgb(0xe2e8f0))
            .child(div().text_2xl().child("Node engine · boundary lab"))
            .child(format!("Root renders {} · parent input {}", self.renders, self.revision))
            .child(match window.retained_node_stats() {
                Some(stats) => format!("Retained engine · previous frame: {} rebuilt, {} reused subtrees, {} live scopes", stats.rebuilt_scopes, stats.reused_subtrees, stats.live_nodes),
                None => "Legacy engine · phase traces go to the terminal".to_string(),
            })
            .child(div().flex().flex_wrap().gap_2()
                .child(button("props", "Change parent input").on_click(cx.listener(|this, _, _, cx| { this.revision += 1; cx.notify(); })))
                .child(button("reorder", "Swap A / B").on_click(cx.listener(|this, _, _, cx| { this.reversed = !this.reversed; cx.notify(); })))
                .child(button("mount", if self.show_first { "Remove A" } else { "Reinsert A" }).on_click(cx.listener(|this, _, _, cx| { this.show_first = !this.show_first; cx.notify(); })))
                .child(button("width", "Toggle A width").on_click(cx.listener(|this, _, _, cx| { this.wide = !this.wide; cx.notify(); })))
                .child(button("style", "Toggle inherited text size").on_click(cx.listener(|this, _, _, cx| { this.alternate_style = !this.alternate_style; cx.notify(); })))
                .child(button("dependency", "Notify shared dependency").on_click(cx.listener(|this, _, _, cx| {
                    this.dependency.update(cx, |dependency, cx| { dependency.value += 1; cx.notify(); });
                })))
                .child(button("overlay", "Toggle deferred overlay").on_click(cx.listener(|this, _, _, cx| { this.overlay = !this.overlay; cx.notify(); })))
            )
            .child(div().flex().flex_col().gap_4().text_size(px(if self.alternate_style { 18. } else { 14. }))
                .child(div().flex().gap_4().children(cards))
                .child(div().flex().gap_4().children(self.leaves.iter().map(|leaf| {
                    div().w(px(320.)).h(px(155.)).child(leaf.clone())
                }))))
            .child(div().relative().h(px(75.)).bg(rgb(0x7f1d1d)).p_3()
                .child("Ordinary paint below the cards; deferred output must cover this.")
                .when(self.overlay, |element| element.child(deferred(anchored().child(
                    div().w(px(480.)).p_4().bg(rgb(0x0f766e)).shadow_lg()
                        .child(button("close-overlay", "Deferred overlay · click to close")
                            .on_click(cx.listener(|this, _, _, cx| { this.overlay = false; cx.notify(); })))
                )).priority(1))))
            .child("Try: increment A → swap → remove A → reinsert A. Swap preserves state; remount resets it.")
            .child("Increment to focus a card; Enter increments it again. Its yellow border must move with it. Resize or scroll the window and check hit targets.")
            .child("Notify an entity: its parent must rerender. Notify the shared dependency: both entity views must update.")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1050.), px(860.)), cx);
        let result = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    let dependency = cx.new(|_| Dependency { value: 0 });
                    let leaves = ["Entity left", "Entity right"].map(|label| {
                        cx.new(|_| EntityLeaf {
                            phases: Rc::default(),
                            label,
                            dependency: dependency.clone(),
                            renders: 0,
                            clicks: 0,
                        })
                    });
                    BoundaryLab {
                        revision: 0,
                        renders: 0,
                        reversed: false,
                        show_first: true,
                        wide: false,
                        alternate_style: false,
                        overlay: false,
                        mounts: Rc::new(Cell::new(0)),
                        dependency,
                        leaves,
                    }
                })
            },
        );
        if let Err(error) = result {
            eprintln!("Could not open boundary lab: {error}");
            return;
        }
        cx.activate(true);
    });
}
