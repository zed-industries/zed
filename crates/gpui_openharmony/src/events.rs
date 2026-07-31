use gpui::{
    AppLifecyclePhase, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    PlatformInput, TouchEvent, TouchId, TouchPhase, point, px,
};
use std::ffi::c_void;
use std::ptr::NonNull;

/// A host-supplied event from the OpenHarmony side.
///
/// This is the contract between the OpenHarmony native runtime and the GPUI
/// platform implementation. The host is responsible for converting raw
/// XComponent, mouse, or key events into this portable form and calling
/// [`OpenHarmonyPlatform::dispatch_event`](crate::OpenHarmonyPlatform::dispatch_event).
#[derive(Clone, Debug)]
pub enum OpenHarmonyHostEvent {
    SurfaceCreated(SurfaceInfo),
    SurfaceChanged(SurfaceInfo),
    SurfaceDestroyed,
    Touch(OpenHarmonyTouchEvent),
    KeyDown(OpenHarmonyKeyEvent),
    KeyUp(OpenHarmonyKeyEvent),
    Lifecycle(AppLifecyclePhase),
    MemoryWarning,
}

/// A pointer to an OpenHarmony native window (`OHNativeWindow`).
///
/// It is `Send` and `Sync` because it only carries an address, and the host
/// is responsible for the validity and lifetime of the underlying pointer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NativeWindow(usize);

unsafe impl Send for NativeWindow {}
unsafe impl Sync for NativeWindow {}

impl NativeWindow {
    /// Creates a new `NativeWindow` from a raw pointer.
    ///
    /// # Safety
    /// The pointer must be a valid `OHNativeWindow` that outlives this value.
    pub unsafe fn new(ptr: *mut c_void) -> Self {
        Self(ptr as usize)
    }

    /// Returns the raw pointer value.
    pub fn as_ptr(&self) -> *mut c_void {
        self.0 as *mut c_void
    }

    /// Returns the pointer as a `NonNull<c_void>` if it is non-null.
    pub fn as_nonnull(&self) -> Option<NonNull<c_void>> {
        NonNull::new(self.as_ptr())
    }
}

/// Information about an OpenHarmony surface supplied by the host.
#[derive(Clone, Debug)]
pub struct SurfaceInfo {
    /// The `OHNativeWindow` pointer for this surface.
    pub native_window: NativeWindow,
    /// Surface size in logical pixels.
    pub size: gpui::Size<gpui::Pixels>,
    /// Display scale factor for the surface.
    pub scale_factor: f32,
}

/// A touch phase in the OpenHarmony host contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenHarmonyTouchPhase {
    Down,
    Up,
    Move,
    Cancel,
}

/// A touch event in the OpenHarmony host contract.
#[derive(Clone, Debug)]
pub struct OpenHarmonyTouchEvent {
    pub id: u64,
    pub phase: OpenHarmonyTouchPhase,
    pub x: f32,
    pub y: f32,
    pub force: f32,
}

/// A key event in the OpenHarmony host contract.
#[derive(Clone, Debug)]
pub struct OpenHarmonyKeyEvent {
    pub key: String,
    pub modifiers: Modifiers,
}

impl OpenHarmonyHostEvent {
    /// Convert a host touch event into the GPUI [`PlatformInput`] form.
    pub fn to_platform_input(self) -> Option<PlatformInput> {
        match self {
            OpenHarmonyHostEvent::Touch(touch) => {
                let phase = match touch.phase {
                    OpenHarmonyTouchPhase::Down => TouchPhase::Started,
                    OpenHarmonyTouchPhase::Up => TouchPhase::Ended,
                    OpenHarmonyTouchPhase::Move => TouchPhase::Moved,
                    OpenHarmonyTouchPhase::Cancel => TouchPhase::Cancelled,
                };
                let force = if touch.force > 0.0 {
                    Some(touch.force.clamp(0.0, 1.0))
                } else {
                    None
                };
                Some(PlatformInput::Touch(TouchEvent {
                    id: TouchId(touch.id),
                    phase,
                    position: point(px(touch.x), px(touch.y)),
                    force,
                }))
            }
            OpenHarmonyHostEvent::KeyDown(key) => {
                let keystroke = gpui::Keystroke {
                    modifiers: key.modifiers,
                    key: key.key,
                    key_char: None,
                };
                Some(PlatformInput::KeyDown(gpui::KeyDownEvent {
                    keystroke,
                    is_held: false,
                    prefer_character_input: false,
                }))
            }
            OpenHarmonyHostEvent::KeyUp(key) => {
                let keystroke = gpui::Keystroke {
                    modifiers: key.modifiers,
                    key: key.key,
                    key_char: None,
                };
                Some(PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }))
            }
            _ => None,
        }
    }
}

/// Converts a touch event into a mouse move or mouse down/up event for
/// components that expect mouse input. GPUI handles raw touch on mobile,
/// but some interactions still expect mouse-compatible events.
pub(crate) fn touch_to_mouse(touch: &OpenHarmonyTouchEvent) -> Option<PlatformInput> {
    let position = point(px(touch.x), px(touch.y));
    let modifiers = Modifiers::default();
    match touch.phase {
        OpenHarmonyTouchPhase::Down => Some(PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count: 1,
            first_mouse: true,
        })),
        OpenHarmonyTouchPhase::Up => Some(PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count: 1,
        })),
        OpenHarmonyTouchPhase::Move => Some(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: Some(MouseButton::Left),
            modifiers,
        })),
        OpenHarmonyTouchPhase::Cancel => None,
    }
}
