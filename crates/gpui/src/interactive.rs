use crate::{
    point, seal::Sealed, Empty, IntoElement, Keystroke, Modifiers, Pixels, Point, Render,
    ViewContext,
};
use smallvec::SmallVec;
use std::{any::Any, fmt::Debug, ops::Deref, path::PathBuf};

/// An event from a platform input source.
pub trait InputEvent: Sealed + 'static {
    /// Convert this event into the platform input enum.
    fn to_platform_input(self) -> PlatformInput;
}

/// A key event from the platform.
pub trait KeyEvent: InputEvent {}

/// A mouse event from the platform.
pub trait MouseEvent: InputEvent {}

/// The key down event equivalent for the platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyDownEvent {
    /// The keystroke that was generated.
    pub keystroke: Keystroke,

    /// Whether the key is currently held down.
    pub is_held: bool,
}

impl Sealed for KeyDownEvent {}
impl InputEvent for KeyDownEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::KeyDown(self)
    }
}
impl KeyEvent for KeyDownEvent {}

/// The key up event equivalent for the platform.
#[derive(Clone, Debug)]
pub struct KeyUpEvent {
    /// The keystroke that was released.
    pub keystroke: Keystroke,
}

impl Sealed for KeyUpEvent {}
impl InputEvent for KeyUpEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::KeyUp(self)
    }
}
impl KeyEvent for KeyUpEvent {}

/// The modifiers changed event equivalent for the platform.
#[derive(Clone, Debug, Default)]
pub struct ModifiersChangedEvent {
    /// The new state of the modifier keys
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
    /// The touch started.
    Started,
    /// The touch event is moving.
    #[default]
    Moved,
    /// The touch phase has ended
    Ended,
}

/// A mouse down event from the platform
#[derive(Clone, Debug, Default)]
pub struct MouseDownEvent {
    /// Which mouse button was pressed.
    pub button: MouseButton,

    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The modifiers that were held down when the mouse was pressed.
    pub modifiers: Modifiers,

    /// The number of times the button has been clicked.
    pub click_count: usize,
}

impl Sealed for MouseDownEvent {}
impl InputEvent for MouseDownEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseDown(self)
    }
}
impl MouseEvent for MouseDownEvent {}

/// A mouse up event from the platform
#[derive(Clone, Debug, Default)]
pub struct MouseUpEvent {
    /// Which mouse button was released.
    pub button: MouseButton,

    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The modifiers that were held down when the mouse was released.
    pub modifiers: Modifiers,

    /// The number of times the button has been clicked.
    pub click_count: usize,
}

impl Sealed for MouseUpEvent {}
impl InputEvent for MouseUpEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::MouseUp(self)
    }
}
impl MouseEvent for MouseUpEvent {}

/// A click event, generated when a mouse button is pressed and released.
#[derive(Clone, Debug, Default)]
pub struct ClickEvent {
    /// The mouse event when the button was pressed.
    pub down: MouseDownEvent,

    /// The mouse event when the button was released.
    pub up: MouseUpEvent,
}

/// An enum representing the mouse button that was pressed.
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum MouseButton {
    /// The left mouse button.
    Left,

    /// The right mouse button.
    Right,

    /// The middle mouse button.
    Middle,

    /// A navigation button, such as back or forward.
    Navigate(NavigationDirection),
}

impl MouseButton {
    /// Get all the mouse buttons in a list.
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

/// A navigation direction, such as back or forward.
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum NavigationDirection {
    /// The back button.
    Back,

    /// The forward button.
    Forward,
}

impl Default for NavigationDirection {
    fn default() -> Self {
        Self::Back
    }
}

/// A mouse move event from the platform
#[derive(Clone, Debug, Default)]
pub struct MouseMoveEvent {
    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The mouse button that was pressed, if any.
    pub pressed_button: Option<MouseButton>,

    /// The modifiers that were held down when the mouse was moved.
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
    /// Returns true if the left mouse button is currently held down.
    pub fn dragging(&self) -> bool {
        self.pressed_button == Some(MouseButton::Left)
    }
}

/// A mouse wheel event from the platform
#[derive(Clone, Debug, Default)]
pub struct ScrollWheelEvent {
    /// The position of the mouse on the window.
    pub position: Point<Pixels>,

    /// The change in scroll wheel position for this event.
    pub delta: ScrollDelta,

    /// The modifiers that were held down when the mouse was moved.
    pub modifiers: Modifiers,

    /// The phase of the touch event.
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

/// The scroll delta for a scroll wheel event.
#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    /// An exact scroll delta in pixels.
    Pixels(Point<Pixels>),
    /// An inexact scroll delta in lines.
    Lines(Point<f32>),
}

impl Default for ScrollDelta {
    fn default() -> Self {
        Self::Lines(Default::default())
    }
}

impl ScrollDelta {
    /// Returns true if this is a precise scroll delta in pixels.
    pub fn precise(&self) -> bool {
        match self {
            ScrollDelta::Pixels(_) => true,
            ScrollDelta::Lines(_) => false,
        }
    }

    /// Converts this scroll event into exact pixels.
    pub fn pixel_delta(&self, line_height: Pixels) -> Point<Pixels> {
        match self {
            ScrollDelta::Pixels(delta) => *delta,
            ScrollDelta::Lines(delta) => point(line_height * delta.x, line_height * delta.y),
        }
    }

    /// Combines two scroll deltas into one.
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

/// A mouse exit event from the platform, generated when the mouse leaves the window.
/// The position generated should be just outside of the window's bounds.
#[derive(Clone, Debug, Default)]
pub struct MouseExitEvent {
    /// The position of the mouse relative to the window.
    pub position: Point<Pixels>,
    /// The mouse button that was pressed, if any.
    pub pressed_button: Option<MouseButton>,
    /// The modifiers that were held down when the mouse was moved.
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

/// A collection of paths from the platform, such as from a file drop.
#[derive(Debug, Clone, Default)]
pub struct ExternalPaths(pub(crate) SmallVec<[PathBuf; 2]>);

impl ExternalPaths {
    /// Convert this collection of paths into a slice.
    pub fn paths(&self) -> &[PathBuf] {
        &self.0
    }
}

impl Render for ExternalPaths {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        // the platform will render icons for the dragged files
        Empty
    }
}

/// A file drop event from the platform, generated when files are dragged and dropped onto the window.
#[derive(Debug, Clone)]
pub enum FileDropEvent {
    /// The files have entered the window.
    Entered {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
        /// The paths of the files that are being dragged.
        paths: ExternalPaths,
    },
    /// The files are being dragged over the window
    Pending {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
    },
    /// The files have been dropped onto the window.
    Submit {
        /// The position of the mouse relative to the window.
        position: Point<Pixels>,
    },
    /// The user has stopped dragging the files over the window.
    Exited,
}

impl Sealed for FileDropEvent {}
impl InputEvent for FileDropEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::FileDrop(self)
    }
}
impl MouseEvent for FileDropEvent {}

/// An enum corresponding to all kinds of platform input events.
#[derive(Clone, Debug)]
pub enum PlatformInput {
    /// A key was pressed.
    KeyDown(KeyDownEvent),
    /// A key was released.
    KeyUp(KeyUpEvent),
    /// The keyboard modifiers were changed.
    ModifiersChanged(ModifiersChangedEvent),
    /// The mouse was pressed.
    MouseDown(MouseDownEvent),
    /// The mouse was released.
    MouseUp(MouseUpEvent),
    /// The mouse was moved.
    MouseMove(MouseMoveEvent),
    /// The mouse exited the window.
    MouseExited(MouseExitEvent),
    /// The scroll wheel was used.
    ScrollWheel(ScrollWheelEvent),
    /// Files were dragged and dropped onto the window.
    FileDrop(FileDropEvent),
}

impl PlatformInput {
    pub(crate) fn mouse_event(&self) -> Option<&dyn Any> {
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

    pub(crate) fn keyboard_event(&self) -> Option<&dyn Any> {
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

        cx.dispatch_keystroke(*window, Keystroke::parse("a").unwrap());
        cx.dispatch_keystroke(*window, Keystroke::parse("ctrl-g").unwrap());

        window
            .update(cx, |test_view, _| {
                assert!(test_view.saw_key_down || test_view.saw_action);
                assert!(test_view.saw_key_down);
                assert!(test_view.saw_action);
            })
            .unwrap();
    }
}
