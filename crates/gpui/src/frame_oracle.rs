use std::{rc::Rc, time::Duration};

use crate::{
    AnyView, AnyWindowHandle, App, Context, FrameSnapshot, InputEvent, IntoElement, Keystroke,
    Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Render,
    ScrollDelta, ScrollWheelEvent, Size, TestAppContext, TouchPhase, Window, WindowAppearance,
    WindowHandle,
};

/// A frame producer selectable by the oracle harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Engine {
    /// GPUI's current element-walking frame producer.
    Walk,
}

/// An input or invalidation applied to a scripted frame.
#[derive(Clone, Debug)]
pub enum FrameStep {
    /// Moves the synthetic pointer.
    MouseMove {
        /// Pointer position in window coordinates.
        position: Point<Pixels>,
        /// Keyboard modifiers held during the move.
        modifiers: Modifiers,
    },
    /// Presses a mouse button.
    MouseDown {
        /// Mouse button to press.
        button: MouseButton,
        /// Pointer position in window coordinates.
        position: Point<Pixels>,
        /// Keyboard modifiers held during the press.
        modifiers: Modifiers,
    },
    /// Releases a mouse button.
    MouseUp {
        /// Mouse button to release.
        button: MouseButton,
        /// Pointer position in window coordinates.
        position: Point<Pixels>,
        /// Keyboard modifiers held during the release.
        modifiers: Modifiers,
    },
    /// Scrolls at a synthetic pointer position.
    Scroll {
        /// Pointer position in window coordinates.
        position: Point<Pixels>,
        /// Pixel scroll delta.
        delta: Point<Pixels>,
        /// Keyboard modifiers held during the scroll.
        modifiers: Modifiers,
    },
    /// Dispatches one parsed keystroke.
    Key(String),
    /// Notifies a view returned by the script builder.
    Notify(usize),
    /// Changes the window's viewport size.
    Resize(Size<Pixels>),
    /// Changes the platform appearance.
    Appearance(WindowAppearance),
    /// Advances the deterministic test clock.
    AdvanceTime(Duration),
}

/// The view graph and notification targets produced by a [`FrameScript`].
pub struct FrameScriptUi {
    root: AnyView,
    notification_targets: Vec<AnyView>,
}

impl FrameScriptUi {
    /// Creates scripted UI with an ordered list of targets for [`FrameStep::Notify`].
    pub fn new(root: impl Into<AnyView>, notification_targets: Vec<AnyView>) -> Self {
        Self {
            root: root.into(),
            notification_targets,
        }
    }
}

type FrameScriptBuilder = dyn Fn(&mut Window, &mut App) -> FrameScriptUi;

/// A deterministic UI builder and ordered sequence of frame-producing steps.
#[derive(Clone)]
pub struct FrameScript {
    build: Rc<FrameScriptBuilder>,
    steps: Vec<FrameStep>,
}

impl FrameScript {
    /// Creates a frame script.
    pub fn new(
        build: impl Fn(&mut Window, &mut App) -> FrameScriptUi + 'static,
        steps: Vec<FrameStep>,
    ) -> Self {
        Self {
            build: Rc::new(build),
            steps,
        }
    }

    /// Returns the script's ordered steps.
    pub fn steps(&self) -> &[FrameStep] {
        &self.steps
    }
}

struct FrameScriptRoot {
    ui: FrameScriptUi,
}

impl Render for FrameScriptRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.ui.root.clone()
    }
}

/// Drives one independently-created window through a [`FrameScript`].
pub struct Harness {
    engine: Engine,
    window: WindowHandle<FrameScriptRoot>,
    snapshot: FrameSnapshot,
}

impl Harness {
    /// Builds an independent window and captures its initial frame.
    pub fn new(
        engine: Engine,
        script: &FrameScript,
        cx: &mut TestAppContext,
    ) -> anyhow::Result<Self> {
        let build = script.build.clone();
        let window = cx.add_window(move |window, cx| FrameScriptRoot {
            ui: build(window, cx),
        });
        let snapshot = draw_and_snapshot(window.into(), cx)?;
        Ok(Self {
            engine,
            window,
            snapshot,
        })
    }

    /// Applies one step, drains deterministic work, and captures the resulting frame.
    pub fn apply(&mut self, step: &FrameStep, cx: &mut TestAppContext) -> anyhow::Result<()> {
        match self.engine {
            Engine::Walk => self.apply_walk(step, cx)?,
        }
        cx.run_until_parked();
        self.snapshot = draw_and_snapshot(self.window.into(), cx)?;
        Ok(())
    }

    /// Returns the most recently captured frame.
    pub fn snapshot(&self) -> &FrameSnapshot {
        &self.snapshot
    }

    fn apply_walk(&self, step: &FrameStep, cx: &mut TestAppContext) -> anyhow::Result<()> {
        match step {
            FrameStep::MouseMove {
                position,
                modifiers,
            } => dispatch_input(
                self.window.into(),
                MouseMoveEvent {
                    position: *position,
                    pressed_button: None,
                    modifiers: *modifiers,
                },
                cx,
            )?,
            FrameStep::MouseDown {
                button,
                position,
                modifiers,
            } => dispatch_input(
                self.window.into(),
                MouseDownEvent {
                    button: *button,
                    position: *position,
                    modifiers: *modifiers,
                    click_count: 1,
                    first_mouse: false,
                },
                cx,
            )?,
            FrameStep::MouseUp {
                button,
                position,
                modifiers,
            } => dispatch_input(
                self.window.into(),
                MouseUpEvent {
                    button: *button,
                    position: *position,
                    modifiers: *modifiers,
                    click_count: 1,
                },
                cx,
            )?,
            FrameStep::Scroll {
                position,
                delta,
                modifiers,
            } => dispatch_input(
                self.window.into(),
                ScrollWheelEvent {
                    position: *position,
                    delta: ScrollDelta::Pixels(*delta),
                    modifiers: *modifiers,
                    touch_phase: TouchPhase::Moved,
                },
                cx,
            )?,
            FrameStep::Key(key) => {
                let keystroke = Keystroke::parse(key)?;
                self.window.update(cx, |_, window, cx| {
                    window.dispatch_keystroke(keystroke, cx);
                })?;
            }
            FrameStep::Notify(index) => {
                self.window.update(cx, |root, _, cx| {
                    let target = root.ui.notification_targets.get(*index).ok_or_else(|| {
                        anyhow::anyhow!("notification target {index} is not defined")
                    })?;
                    App::notify(cx, target.entity_id());
                    Ok::<_, anyhow::Error>(())
                })??;
            }
            FrameStep::Resize(size) => {
                cx.simulate_window_resize(self.window.into(), *size);
            }
            FrameStep::Appearance(appearance) => {
                cx.simulate_window_appearance(self.window.into(), *appearance);
            }
            FrameStep::AdvanceTime(duration) => {
                cx.executor().advance_clock(*duration);
            }
        }
        Ok(())
    }
}

/// Applies all steps to two independent harnesses and compares every frame.
pub fn assert_walk_vs_walk(script: &FrameScript, cx: &mut TestAppContext) -> anyhow::Result<()> {
    let mut left = Harness::new(Engine::Walk, script, cx)?;
    let mut right = Harness::new(Engine::Walk, script, cx)?;
    assert_snapshots_equal("initial frame", left.snapshot(), right.snapshot())?;

    for (index, step) in script.steps().iter().enumerate() {
        if let FrameStep::AdvanceTime(duration) = step {
            cx.executor().advance_clock(*duration);
            cx.run_until_parked();
            left.snapshot = draw_and_snapshot(left.window.into(), cx)?;
            right.snapshot = draw_and_snapshot(right.window.into(), cx)?;
        } else {
            left.apply(step, cx)?;
            right.apply(step, cx)?;
        }
        assert_snapshots_equal(
            &format!("step {index}: {step:?}"),
            left.snapshot(),
            right.snapshot(),
        )?;
    }
    Ok(())
}

fn dispatch_input(
    window: AnyWindowHandle,
    event: impl InputEvent,
    cx: &mut TestAppContext,
) -> anyhow::Result<()> {
    window.update(cx, |_, window, cx| {
        window.dispatch_event(event.to_platform_input(), cx);
    })?;
    Ok(())
}

fn draw_and_snapshot(
    window: AnyWindowHandle,
    cx: &mut TestAppContext,
) -> anyhow::Result<FrameSnapshot> {
    window.update(cx, |_, window, cx| {
        window.draw(cx).clear(cx);
        window.frame_snapshot()
    })
}

fn assert_snapshots_equal(
    label: &str,
    left: &FrameSnapshot,
    right: &FrameSnapshot,
) -> anyhow::Result<()> {
    if left == right {
        return Ok(());
    }
    anyhow::bail!("{label}\n{}", left.pretty_diff(right, 32))
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use crate::{
        Animation, AnimationExt as _, AppContext as _, FocusHandle, InteractiveElement as _,
        ListAlignment, ListState, ParentElement as _, RenderImage, StatefulInteractiveElement as _,
        Styled as _, UniformListScrollHandle, anchored, assert_walk_vs_walk, canvas, deferred, div,
        fill, img, list, point, px, rgba, size, svg, uniform_list,
    };
    use image::{Frame, ImageBuffer, Rgba};
    use smallvec::SmallVec;

    use super::*;

    struct StorybookLeaf {
        value: usize,
    }

    impl Render for StorybookLeaf {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
                .id("leaf")
                .h(px(24.))
                .bg(rgba(0x334455ff))
                .child(format!("leaf {}", self.value))
        }
    }

    struct Storybook {
        leaf: crate::Entity<StorybookLeaf>,
        focus: FocusHandle,
        list_state: ListState,
        uniform_scroll: UniformListScrollHandle,
        image: Arc<RenderImage>,
        clicks: usize,
    }

    impl Render for Storybook {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            let list_state = self.list_state.clone();
            div()
                .id("storybook")
                .key_context("Storybook")
                .track_focus(&self.focus)
                .focus(|style| style.bg(rgba(0x224466ff)))
                .size_full()
                .overflow_y_scroll()
                .child(
                    div()
                        .id("interactive")
                        .w(px(160.))
                        .h(px(48.))
                        .bg(rgba(0x222222ff))
                        .hover(|style| style.bg(rgba(0x444444ff)))
                        .active(|style| style.bg(rgba(0x666666ff)))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.clicks += 1;
                            window.focus(&this.focus, cx);
                            cx.notify();
                        }))
                        .child(format!("clicks {}", self.clicks)),
                )
                .child(self.leaf.clone())
                .child(
                    uniform_list(
                        "uniform",
                        8,
                        cx.processor(|_, range: std::ops::Range<usize>, _, _| {
                            range
                                .map(|index| div().h(px(20.)).child(format!("uniform {index}")))
                                .collect()
                        }),
                    )
                    .h(px(60.))
                    .track_scroll(&self.uniform_scroll),
                )
                .child(
                    list(list_state, |index, _, _| {
                        div()
                            .h(px(18. + index as f32))
                            .child(format!("list {index}"))
                            .into_any_element()
                    })
                    .h(px(64.)),
                )
                .child(img(self.image.clone()).size(px(12.)))
                .child(
                    svg()
                        .path("icons/arrow_circle.svg")
                        .size(px(12.))
                        .text_color(rgba(0xffffffff)),
                )
                .child(
                    canvas(
                        |bounds, _, _| bounds,
                        |bounds, _, window, _| {
                            window.paint_quad(fill(bounds, rgba(0x8899aaff)));
                        },
                    )
                    .size(px(20.)),
                )
                .child(deferred(
                    anchored()
                        .position(point(px(8.), px(8.)))
                        .child(div().id("anchor").size(px(16.)).bg(rgba(0xaabbccff))),
                ))
                .child(div().with_animation(
                    "storybook-spinner",
                    Animation::new(Duration::from_secs(1)).repeat(),
                    |element, delta| {
                        element
                            .size(px(14.))
                            .bg(rgba(0xff8800ff))
                            .opacity(0.5 + delta * 0.5)
                    },
                ))
        }
    }

    fn storybook_script() -> FrameScript {
        let image = Arc::new(RenderImage::new(SmallVec::from_elem(
            Frame::new(ImageBuffer::from_pixel(1, 1, Rgba([64, 128, 192, 255]))),
            1,
        )));
        FrameScript::new(
            move |_, cx| {
                let leaf = cx.new(|_| StorybookLeaf { value: 0 });
                let root = cx.new(|cx| Storybook {
                    leaf: leaf.clone(),
                    focus: cx.focus_handle(),
                    list_state: ListState::new(6, ListAlignment::Top, px(8.)),
                    uniform_scroll: UniformListScrollHandle::new(),
                    image: image.clone(),
                    clicks: 0,
                });
                FrameScriptUi::new(root.clone(), vec![leaf.into(), root.into()])
            },
            vec![
                FrameStep::Notify(0),
                FrameStep::Notify(1),
                FrameStep::MouseMove {
                    position: point(px(20.), px(20.)),
                    modifiers: Modifiers::default(),
                },
                FrameStep::MouseDown {
                    button: MouseButton::Left,
                    position: point(px(20.), px(20.)),
                    modifiers: Modifiers::default(),
                },
                FrameStep::MouseUp {
                    button: MouseButton::Left,
                    position: point(px(20.), px(20.)),
                    modifiers: Modifiers::default(),
                },
                FrameStep::Scroll {
                    position: point(px(40.), px(120.)),
                    delta: point(px(0.), px(-24.)),
                    modifiers: Modifiers::default(),
                },
                FrameStep::AdvanceTime(Duration::from_millis(16)),
                FrameStep::Resize(size(px(720.), px(480.))),
                FrameStep::Appearance(WindowAppearance::Dark),
                FrameStep::MouseMove {
                    position: point(px(300.), px(300.)),
                    modifiers: Modifiers::default(),
                },
            ],
        )
    }

    #[crate::test(iterations = 100)]
    fn walk_vs_walk_storybook(cx: &mut TestAppContext) {
        assert_walk_vs_walk(&storybook_script(), cx).unwrap_or_else(|error| panic!("{error:#}"));
    }

    #[crate::property_test]
    fn walk_vs_walk_generated(
        cx: &mut TestAppContext,
        #[strategy = 1usize..16] node_count: usize,
        #[strategy = crate::proptest::collection::vec(0u8..4, 0..12)] raw_steps: Vec<u8>,
    ) {
        let script = FrameScript::new(
            move |_, cx| {
                let root = cx.new(move |_| GeneratedTree { node_count });
                FrameScriptUi::new(root.clone(), vec![root.into()])
            },
            raw_steps
                .into_iter()
                .map(|step| match step {
                    0 => FrameStep::Notify(0),
                    1 => FrameStep::Resize(size(px(320.), px(240.))),
                    2 => FrameStep::MouseMove {
                        position: point(px(8.), px(8.)),
                        modifiers: Modifiers::default(),
                    },
                    _ => FrameStep::AdvanceTime(Duration::from_millis(1)),
                })
                .collect(),
        );
        assert_walk_vs_walk(&script, cx).unwrap_or_else(|error| panic!("{error:#}"));
    }

    struct GeneratedTree {
        node_count: usize,
    }

    impl Render for GeneratedTree {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().children((0..self.node_count).map(|index| {
                div()
                    .id(index)
                    .h(px(4. + (index % 3) as f32))
                    .bg(rgba(0x102030ff + index as u32))
            }))
        }
    }
}
