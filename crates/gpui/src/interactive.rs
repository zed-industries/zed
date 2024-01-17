use crate::{
    point, seal::Sealed, IntoElement, Keystroke, Modifiers, Pixels, Point, Render, ViewContext,
};
use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, ops::Deref, path::PathBuf};

pub trait InputEvent: Sealed + 'static {
    fn to_platform_input(self) -> PlatformInput;
}
pub trait KeyEvent: InputEvent {}
pub trait MouseEvent: InputEvent {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDownEvent {
    pub keystroke: Keystroke,
    pub is_held: bool,
}

impl Sealed for KeyDownEvent {}
impl InputEvent for KeyDownEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::KeyDown(self)
    }
}
impl KeyEvent for KeyDownEvent {}

#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    pub keystroke: Keystroke,
}

impl Sealed for KeyUpEvent {}
impl InputEvent for KeyUpEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::KeyUp(self)
    }
}
impl KeyEvent for KeyUpEvent {}

#[derive(Clone, Debug, Default)]
pub struct ModifiersChangedEvent {
    pub modifiers: Modifiers,
}

impl Sealed for ModifiersChangedEvent {}
impl InputEvent for ModifiersChangedEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::ModifiersChanged(self)
    }
}
impl KeyEvent for ModifiersChangedEvent {}

impl Deref for ModifiersChangedEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

/// The phase of a touch motion event.
/// Based on the winit enum of the same name.
#[derive(Clone, Copy, Debug, Default)]
pub enum TouchPhase {
    Started,
    #[default]
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

impl Sealed for MouseDownEvent {}
impl InputEvent for MouseDownEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseDown(self)
    }
}
impl MouseEvent for MouseDownEvent {}

#[derive(Clone, Debug, Default)]
pub struct MouseUpEvent {
    pub button: MouseButton,
    pub position: Point<Pixels>,
    pub modifiers: Modifiers,
    pub click_count: usize,
}

impl Sealed for MouseUpEvent {}
impl InputEvent for MouseUpEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseUp(self)
    }
}
impl MouseEvent for MouseUpEvent {}

#[derive(Clone, Debug, Default)]
pub struct ClickEvent {
    pub down: MouseDownEvent,
    pub up: MouseUpEvent,
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

impl Sealed for MouseMoveEvent {}
impl InputEvent for MouseMoveEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseMove(self)
    }
}
impl MouseEvent for MouseMoveEvent {}

impl MouseMoveEvent {
    pub fn dragging(&self) -> bool {
        self.pressed_button == Some(MouseButton::Left)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScrollWheelEvent {
    pub position: Point<Pixels>,
    pub delta: ScrollDelta,
    pub modifiers: Modifiers,
    pub touch_phase: TouchPhase,
}

impl Sealed for ScrollWheelEvent {}
impl InputEvent for ScrollWheelEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::ScrollWheel(self)
    }
}
impl MouseEvent for ScrollWheelEvent {}

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

    pub fn coalesce(self, other: ScrollDelta) -> ScrollDelta {
        match (self, other) {
            (ScrollDelta::Pixels(px_a), ScrollDelta::Pixels(px_b)) => {
                ScrollDelta::Pixels(px_a + px_b)
            }

            (ScrollDelta::Lines(lines_a), ScrollDelta::Lines(lines_b)) => {
                ScrollDelta::Lines(lines_a + lines_b)
            }

            _ => other,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MouseExitEvent {
    pub position: Point<Pixels>,
    pub pressed_button: Option<MouseButton>,
    pub modifiers: Modifiers,
}

impl Sealed for MouseExitEvent {}
impl InputEvent for MouseExitEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseExited(self)
    }
}
impl MouseEvent for MouseExitEvent {}

impl Deref for MouseExitEvent {
    type Target = Modifiers;

    fn deref(&self) -> &Self::Target {
        &self.modifiers
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExternalPaths(pub(crate) SmallVec<[PathBuf; 2]>);

impl ExternalPaths {
    pub fn paths(&self) -> &[PathBuf] {
        &self.0
    }
}

impl Render for ExternalPaths {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        () // Intentionally left empty because the platform will render icons for the dragged files
    }
}

#[derive(Debug, Clone)]
pub enum FileDropEvent {
    Entered {
        position: Point<Pixels>,
        paths: ExternalPaths,
    },
    Pending {
        position: Point<Pixels>,
    },
    Submit {
        position: Point<Pixels>,
    },
    Exited,
}

impl Sealed for FileDropEvent {}
impl InputEvent for FileDropEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::FileDrop(self)
    }
}
impl MouseEvent for FileDropEvent {}

#[derive(Clone, Debug)]
pub enum PlatformInput {
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

impl PlatformInput {
    pub fn position(&self) -> Option<Point<Pixels>> {
        match self {
            PlatformInput::KeyDown { .. } => None,
            PlatformInput::KeyUp { .. } => None,
            PlatformInput::ModifiersChanged { .. } => None,
            PlatformInput::MouseDown(event) => Some(event.position),
            PlatformInput::MouseUp(event) => Some(event.position),
            PlatformInput::MouseMove(event) => Some(event.position),
            PlatformInput::MouseExited(event) => Some(event.position),
            PlatformInput::ScrollWheel(event) => Some(event.position),
            PlatformInput::FileDrop(FileDropEvent::Exited) => None,
            PlatformInput::FileDrop(
                FileDropEvent::Entered { position, .. }
                | FileDropEvent::Pending { position, .. }
                | FileDropEvent::Submit { position, .. },
            ) => Some(*position),
        }
    }

    pub fn mouse_event(&self) -> Option<&dyn Any> {
        match self {
            PlatformInput::KeyDown { .. } => None,
            PlatformInput::KeyUp { .. } => None,
            PlatformInput::ModifiersChanged { .. } => None,
            PlatformInput::MouseDown(event) => Some(event),
            PlatformInput::MouseUp(event) => Some(event),
            PlatformInput::MouseMove(event) => Some(event),
            PlatformInput::MouseExited(event) => Some(event),
            PlatformInput::ScrollWheel(event) => Some(event),
            PlatformInput::FileDrop(event) => Some(event),
        }
    }

    pub fn keyboard_event(&self) -> Option<&dyn Any> {
        match self {
            PlatformInput::KeyDown(event) => Some(event),
            PlatformInput::KeyUp(event) => Some(event),
            PlatformInput::ModifiersChanged(event) => Some(event),
            PlatformInput::MouseDown(_) => None,
            PlatformInput::MouseUp(_) => None,
            PlatformInput::MouseMove(_) => None,
            PlatformInput::MouseExited(_) => None,
            PlatformInput::ScrollWheel(_) => None,
            PlatformInput::FileDrop(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        self as gpui, div, Element, FocusHandle, InteractiveElement, IntoElement, KeyBinding,
        Keystroke, ParentElement, Render, TestAppContext, VisualContext,
    };

    struct TestView {
        saw_key_down: bool,
        saw_action: bool,
        focus_handle: FocusHandle,
    }

    actions!(test, [TestAction]);

    impl Render for TestView {
        fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl Element {
            div().id("testview").child(
                div()
                    .key_context("parent")
                    .on_key_down(cx.listener(|this, _, cx| {
                        cx.stop_propagation();
                        this.saw_key_down = true
                    }))
                    .on_action(
                        cx.listener(|this: &mut TestView, _: &TestAction, _| {
                            this.saw_action = true
                        }),
                    )
                    .child(
                        div()
                            .key_context("nested")
                            .track_focus(&self.focus_handle)
                            .into_element(),
                    ),
            )
        }
    }

    #[gpui::test]
    fn test_on_events(cx: &mut TestAppContext) {
        let window = cx.update(|cx| {
            cx.open_window(Default::default(), |cx| {
                cx.new_view(|cx| TestView {
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

        cx.dispatch_keystroke(*window, Keystroke::parse("a").unwrap(), false);
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
