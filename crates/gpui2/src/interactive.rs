use crate::{
    div, point, Div, Element, FocusHandle, Keystroke, Modifiers, Pixels, Point, Render, RenderOnce,
    ViewContext,
};
use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, marker::PhantomData, ops::Deref, path::PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDownEvent {
    pub keystroke: Keystroke,
    pub is_held: bool,
}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub keystroke: Keystroke,
}

#[derive(Clone, Debug, Default)]
pub struct ModifiersChangedEvent {
    pub modifiers: Modifiers,
}

impl Deref for ModifiersChangedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name.
#[derive(Clone, Copy, Debug)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
}

#[derive(Clone, Debug, Default)]
pub struct MouseDownEvent {
    pub button: MouseButton,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
}

pub struct Drag<S, R, V, E>
where
    R: Fn(&mut V, &mut ViewContext<V>) -> E,
    V: 'static,
    E: RenderOnce,
{
    pub state: S,
    pub render_drag_handle: R,
    view_element_types: PhantomData<(V, E)>,
}

impl<S, R, V, E> Drag<S, R, V, E>
where
    R: Fn(&mut V, &mut ViewContext<V>) -> E,
    V: 'static,
    E: Element,
{
    pub fn new(state: S, render_drag_handle: R) -> Self {
        Drag {
            state,
            render_drag_handle,
            view_element_types: Default::default(),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Navigate(NavigationDirection),
}

impl MouseButton {
    pub fn all() -> Vec<Self> {
        vec![
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::Navigate(NavigationDirection::Back),
            MouseButton::Navigate(NavigationDirection::Forward),
        ]
    }
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NavigationDirection {
    Back,
    Forward,
}

impl Default for NavigationDirection {
    fn default() -> Self {
        Self::Back
    }
}

#[derive(Clone, Debug, Default)]
pub struct MouseMoveEvent {
    pub position: Point<Pixels>,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

#[derive(Clone, Debug)]
pub struct ScrollWheelEvent {
    pub position: Point<Pixels>,
    pub delta: ScrollDelta,
    pub modifiers: Modifiers,
    pub touch_phase: TouchPhase,
}

impl Deref for ScrollWheelEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    Pixels(Point<Pixels>),
    Lines(Point<f32>),
}

impl Default for ScrollDelta {
    fn default() -> Self {
        Self::Lines(Default::default())
    }
}

impl ScrollDelta {
    pub fn precise(&self) -> bool {
        match self {
            ScrollDelta::Pixels(_) => true,
            ScrollDelta::Lines(_) => false,
        }
    }

    pub fn pixel_delta(&self, line_height: Pixels) -> Point<Pixels> {
        match self {
            ScrollDelta::Pixels(delta) => *delta,
            ScrollDelta::Lines(delta) => point(line_height * delta.x, line_height * delta.y),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MouseExitEvent {
    pub position: Point<Pixels>,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

impl Deref for MouseExitEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExternalPaths(pub(crate) SmallVec<[PathBuf; 2]>);

impl Render for ExternalPaths {
    type Element = Div;

    fn render(&mut self, _: &mut ViewContext<Self>) -> Self::Element {
        div() // Intentionally left empty because the platform will render icons for the dragged files
    }
}

#[derive(Debug, Clone)]
pub enum FileDropEvent {
    Entered {
        position: Point<Pixels>,
        files: ExternalPaths,
    },
    Pending {
        position: Point<Pixels>,
    },
    Submit {
        position: Point<Pixels>,
    },
    Exited,
}

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyDown(KeyDownEvent),
    KeyUp(KeyUpEvent),
    ModifiersChanged(ModifiersChangedEvent),
    MouseDown(MouseDownEvent),
    MouseUp(MouseUpEvent),
    MouseMove(MouseMoveEvent),
    MouseExited(MouseExitEvent),
    ScrollWheel(ScrollWheelEvent),
    FileDrop(FileDropEvent),
}

impl InputEvent {
    pub fn position(&self) -> Option<Point<Pixels>> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event.position),
            InputEvent::MouseUp(event) => Some(event.position),
            InputEvent::MouseMove(event) => Some(event.position),
            InputEvent::MouseExited(event) => Some(event.position),
            InputEvent::ScrollWheel(event) => Some(event.position),
            InputEvent::FileDrop(FileDropEvent::Exited) => None,
            InputEvent::FileDrop(
                FileDropEvent::Entered { position, .. }
                | FileDropEvent::Pending { position, .. }
                | FileDropEvent::Submit { position, .. },
            ) => Some(*position),
        }
    }

    pub fn mouse_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown { .. } => None,
            InputEvent::KeyUp { .. } => None,
            InputEvent::ModifiersChanged { .. } => None,
            InputEvent::MouseDown(event) => Some(event),
            InputEvent::MouseUp(event) => Some(event),
            InputEvent::MouseMove(event) => Some(event),
            InputEvent::MouseExited(event) => Some(event),
            InputEvent::ScrollWheel(event) => Some(event),
            InputEvent::FileDrop(event) => Some(event),
        }
    }

    pub fn keyboard_event<'a>(&'a self) -> Option<&'a dyn Any> {
        match self {
            InputEvent::KeyDown(event) => Some(event),
            InputEvent::KeyUp(event) => Some(event),
            InputEvent::ModifiersChanged(event) => Some(event),
            InputEvent::MouseDown(_) => None,
            InputEvent::MouseUp(_) => None,
            InputEvent::MouseMove(_) => None,
            InputEvent::MouseExited(_) => None,
            InputEvent::ScrollWheel(_) => None,
            InputEvent::FileDrop(_) => None,
        }
    }
}

pub struct FocusEvent {
    pub blurred: Option<FocusHandle>,
    pub focused: Option<FocusHandle>,
}

#[cfg(test)]
mod test {
    use crate::{
        self as gpui, div, Div, FocusHandle, InteractiveElement, KeyBinding, Keystroke,
        ParentElement, Render, RenderOnce, Stateful, TestAppContext, VisualContext,
    };

    struct TestView {
        saw_key_down: bool,
        saw_action: bool,
        focus_handle: FocusHandle,
    }

    actions!(TestAction);

    impl Render for TestView {
        type Element = Stateful<Div>;

        fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> Self::Element {
            div().id("testview").child(
                div()
                    .key_context("parent")
                    .on_key_down(cx.listener(|this, _, _| this.saw_key_down = true))
                    .on_action(
                        cx.callback(|this: &mut TestView, _: &TestAction, _| {
                            this.saw_action = true
                        }),
                    )
                    .child(
                        div()
                            .key_context("nested")
                            .track_focus(&self.focus_handle)
                            .render_once(),
                    ),
            )
        }
    }

    #[gpui::test]
    fn test_on_events(cx: &mut TestAppContext) {
        let window = cx.update(|cx| {
            cx.open_window(Default::default(), |cx| {
                cx.build_view(|cx| TestView {
                    saw_key_down: false,
                    saw_action: false,
                    focus_handle: cx.focus_handle(),
                })
            })
        });

        cx.update(|cx| {
            cx.bind_keys(vec![KeyBinding::new("ctrl-g", TestAction, Some("parent"))]);
        });

        window
            .update(cx, |test_view, cx| cx.focus(&test_view.focus_handle))
            .unwrap();

        cx.dispatch_keystroke(*window, Keystroke::parse("ctrl-g").unwrap(), false);

        window
            .update(cx, |test_view, _| {
                assert!(test_view.saw_key_down || test_view.saw_action);
                assert!(test_view.saw_key_down);
                assert!(test_view.saw_action);
            })
            .unwrap();
    }
}
