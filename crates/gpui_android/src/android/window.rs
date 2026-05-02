use std::{
    cell::{Cell, RefCell},
    ops::Range,
    rc::Rc,
    sync::Arc,
    time::{Duration, Instant},
};

use android_activity::input::{ImeOptions, TextInputState, TextSpan};
use anyhow::{Context as _, Result};
use gpui::{
    AnyWindowHandle, AtlasKey, AtlasTile, Bounds, Capslock, DevicePixels, DispatchEventResult,
    GpuSpecs, Modifiers, MouseButton, Pixels, PlatformAtlas, PlatformDisplay, PlatformInput,
    PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel, RequestFrameOptions,
    Scene, ScrollDelta, ScrollWheelEvent, Size, TouchPhase, WindowAppearance,
    WindowBackgroundAppearance, WindowBounds, WindowControlArea, WindowParams, px,
};

/// Stand-in atlas returned by [`AndroidWindow::sprite_atlas`] when no surface
/// is alive — i.e. before `attach_surface`. GPUI calls `sprite_atlas` very
/// early in window setup (synchronously, on the foreground thread, before
/// the activity loop has had a chance to deliver `MainEvent::InitWindow`),
/// so we MUST hand back something. Real glyph uploads will only succeed once
/// the wgpu surface is up and the renderer's atlas takes over.
struct NoopAtlas;

impl PlatformAtlas for NoopAtlas {
    fn get_or_insert_with<'a>(
        &self,
        _key: &AtlasKey,
        _build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<AtlasTile>> {
        Ok(None)
    }

    fn remove(&self, _key: &AtlasKey) {}
}
use gpui_wgpu::{GpuContext, WgpuRenderer, WgpuSurfaceConfig};
use ndk::native_window::NativeWindow;
use parking_lot::Mutex;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, DisplayHandle, HasDisplayHandle, HasWindowHandle,
    HandleError, RawDisplayHandle, RawWindowHandle, WindowHandle,
};

use super::AndroidDisplay;

#[derive(Default)]
struct WindowCallbacks {
    request_frame: Option<Box<dyn FnMut(RequestFrameOptions)>>,
    input: Option<Box<dyn FnMut(PlatformInput) -> DispatchEventResult>>,
    active_status_change: Option<Box<dyn FnMut(bool)>>,
    hover_status_change: Option<Box<dyn FnMut(bool)>>,
    resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    moved: Option<Box<dyn FnMut()>>,
    should_close: Option<Box<dyn FnMut() -> bool>>,
    close: Option<Box<dyn FnOnce()>>,
    appearance_changed: Option<Box<dyn FnMut()>>,
    hit_test_window_control: Option<Box<dyn FnMut() -> Option<WindowControlArea>>>,
}

struct WindowState {
    bounds: Bounds<Pixels>,
    physical_size: Size<DevicePixels>,
    scale_factor: f32,
    title: String,
    input_handler: Option<PlatformInputHandler>,
    is_active: bool,
    is_hovered: bool,
    mouse_position: Point<Pixels>,
    modifiers: Modifiers,
    capslock: Capslock,
    appearance: WindowAppearance,
    /// `Some` only while a surface is alive. Recreated on `MainEvent::InitWindow`,
    /// dropped synchronously on `MainEvent::TerminateWindow` (the makepad pattern
    /// — see `crates/gpui_android/src/android/platform.rs`).
    renderer: Option<WgpuRenderer>,
    /// Atlas exposed via [`PlatformWindow::sprite_atlas`]. Cached separately so
    /// we can return `Arc<dyn PlatformAtlas>` even when the renderer's been
    /// torn down between window cycles (callers may keep handles around).
    sprite_atlas: Option<Arc<dyn PlatformAtlas>>,
    /// Most-recent `GpuSpecs` reported by wgpu, captured the first time the
    /// renderer initialised.
    gpu_specs: Option<GpuSpecs>,
    /// Last `TextInputState` we *received* from the IME. Used to suppress
    /// echo: when we apply an edit and Android re-broadcasts the resulting
    /// state, the broadcast matches `last_ime_state` and we drop it.
    last_ime_state: Option<TextInputStateSnapshot>,
    /// Whether we've asked Android to display the soft keyboard. Compared
    /// against `input_handler.is_some()` once per run-loop iteration in
    /// [`AndroidWindow::reconcile_keyboard`] so the keyboard doesn't
    /// flicker on every frame — GPUI's render cycle calls
    /// `take_input_handler` followed by `set_input_handler` for every
    /// paint, which would otherwise toggle the IME ~60 times per second.
    keyboard_visible: bool,
    /// Bottom inset (in logical pixels) currently occupied by the soft
    /// keyboard. Subtracted from `bounds.size.height` and the
    /// `content_size()` reported to GPUI so layouts shrink the visible area
    /// when the keyboard is up — without this, focused text fields render
    /// behind the IME on edge-to-edge devices (every modern phone) where
    /// `adjustResize` no longer auto-shrinks the GameActivity surface.
    ime_bottom_inset: Pixels,
    /// Y in window-space (logical pixels) of the focused input's cursor /
    /// selection bottom edge, captured the last time `update_ime_position`
    /// fired (driven by `Window::invalidate_character_coordinates`). Read
    /// by [`Self::focused_field_bottom`] so the platform can decide whether
    /// the cursor is hidden behind the keyboard and needs to be scrolled
    /// into view.
    focused_field_bottom_y: Option<Pixels>,
}

#[derive(Clone, PartialEq, Eq)]
struct TextInputStateSnapshot {
    text: String,
    selection_start: usize,
    selection_end: usize,
    compose_start: Option<usize>,
    compose_end: Option<usize>,
}

impl TextInputStateSnapshot {
    fn compose_range(&self) -> Option<Range<usize>> {
        match (self.compose_start, self.compose_end) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
    }
}

impl From<&TextInputState> for TextInputStateSnapshot {
    fn from(state: &TextInputState) -> Self {
        Self {
            text: state.text.clone(),
            selection_start: state.selection.start,
            selection_end: state.selection.end,
            compose_start: state.compose_region.as_ref().map(|s| s.start),
            compose_end: state.compose_region.as_ref().map(|s| s.end),
        }
    }
}

/// Cheap clone-able handle to a `NativeWindow` that satisfies wgpu's
/// `HasWindowHandle + HasDisplayHandle + Send + Sync + Clone + Debug`
/// requirements. Wrapping `NativeWindow` directly works (NDK 0.9 already
/// provides the impls), but going through this struct lets us decouple the
/// window from its surface lifetime: when Android destroys the surface we
/// just drop our `Renderer`, the held `NativeWindow` is released by `Drop`,
/// and the next `InitWindow` event hands us a fresh one.
#[derive(Clone, Debug)]
struct WindowSurface(NativeWindow);

impl HasWindowHandle for WindowSurface {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let handle = AndroidNdkWindowHandle::new(self.0.ptr().cast());
        // SAFETY: the underlying ANativeWindow is kept alive for the
        // lifetime of `self` via the refcounted `NativeWindow` handle.
        Ok(unsafe { WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(handle)) })
    }
}

impl HasDisplayHandle for WindowSurface {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        // SAFETY: Android exposes a single global display via the
        // zero-sized `AndroidDisplayHandle`; no per-process resource is
        // attached, so the borrow lifetime is trivially valid.
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new()))
        })
    }
}

pub(crate) struct AndroidWindow {
    handle: AnyWindowHandle,
    display: Rc<dyn PlatformDisplay>,
    state: RefCell<WindowState>,
    callbacks: RefCell<WindowCallbacks>,
    /// Shared GPU context across all GPUI windows (we only ever have one on
    /// Android today, but the renderer API expects this shape).
    gpu_context: GpuContext,
    /// Lock-protected handle to the live native window. Held in a `Mutex` so
    /// the JNI-driven event-thread can swap it in/out without `RefCell`'s
    /// thread-locality complaints.
    surface: Mutex<Option<WindowSurface>>,
    /// Most-recent finger position observed during a touch drag, used to
    /// synthesise [`PlatformInput::ScrollWheel`] events with phase
    /// [`TouchPhase::Moved`] so `overflow_scroll` containers can react. We
    /// reset to `None` on `MouseUp`/`MouseDown` so a fresh gesture starts
    /// from a clean delta.
    last_scroll_pos: Cell<Option<Point<Pixels>>>,
    /// Recent (position, motion-event-time) samples taken during an active
    /// touch drag. Times are in nanoseconds since boot (the same epoch as
    /// `MotionEvent::event_time`), **not** wall-clock — see
    /// [`super::input::Translated::Motion`] for why. Bounded to a small
    /// rolling window — old samples drift away from the true release-time
    /// velocity and just smear the answer.
    drag_samples: RefCell<Vec<(Point<Pixels>, i64)>>,
    /// Active fling animation state. `Some` between a high-velocity
    /// `MouseUp` and the moment the velocity decays below
    /// [`FLING_MIN_SPEED`]. While set, [`Self::tick_fling`] emits one
    /// `ScrollWheel` per run-loop iteration so containers keep scrolling
    /// after the finger lifted.
    fling: RefCell<Option<FlingState>>,
    /// Mirrors whether the surface is currently alive. Read by `draw` to skip
    /// frames that arrive after `surfaceDestroyed`.
    surface_alive: Cell<bool>,
}

/// Velocity (in logical pixels per second) at which a fling stops. Below
/// this point friction is doing more work than user-visible motion, so we
/// emit `TouchPhase::Ended` and drop the fling state.
const FLING_MIN_SPEED: f32 = 30.0;
/// Exponential friction coefficient. `velocity *= exp(-FRICTION_K * dt)` per
/// tick. Picked to feel close to Android `OverScroller` defaults — half-life
/// of ~170ms, so a fast fling settles in under a second.
const FRICTION_K: f32 = 4.0;
/// How far back to look when computing release velocity. Older samples
/// reflect the user's movement *during* the drag rather than at release;
/// 80ms is the window AOSP's `VelocityTracker` uses by default.
const VELOCITY_WINDOW: Duration = Duration::from_millis(80);
/// Max number of drag samples to keep. We only need enough to cover
/// [`VELOCITY_WINDOW`] at typical 60–120Hz touch rates.
const DRAG_SAMPLE_BUDGET: usize = 16;

struct FlingState {
    /// Pixels-per-second velocity, decayed exponentially per tick.
    velocity_x: f32,
    velocity_y: f32,
    /// Time of the previous tick — used to size each tick's delta correctly
    /// even if frames arrive irregularly.
    last_tick: Instant,
    /// Position fed into the synthesised `ScrollWheel.position` field. We
    /// advance it each tick so hit-testing tracks where the imaginary
    /// "finger" would be if it kept going.
    position: Point<Pixels>,
    modifiers: Modifiers,
}

impl AndroidWindow {
    pub(crate) fn new(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<dyn PlatformDisplay>,
        scale_factor: f32,
        gpu_context: GpuContext,
    ) -> Self {
        let bounds = params.bounds;
        let physical_size = Size {
            width: DevicePixels((f32::from(bounds.size.width) * scale_factor) as i32),
            height: DevicePixels((f32::from(bounds.size.height) * scale_factor) as i32),
        };

        Self {
            handle,
            display,
            state: RefCell::new(WindowState {
                bounds,
                physical_size,
                scale_factor,
                title: String::new(),
                input_handler: None,
                is_active: true,
                is_hovered: false,
                mouse_position: Point::default(),
                modifiers: Modifiers::default(),
                capslock: Capslock::default(),
                appearance: WindowAppearance::Light,
                renderer: None,
                // Hand callers a NoopAtlas until `attach_surface` swaps in
                // the real wgpu-backed atlas. This avoids the panic that
                // happens when GPUI's window setup queries the atlas before
                // the activity loop has delivered `MainEvent::InitWindow`.
                sprite_atlas: Some(Arc::new(NoopAtlas) as Arc<dyn PlatformAtlas>),
                gpu_specs: None,
                last_ime_state: None,
                keyboard_visible: false,
                ime_bottom_inset: px(0.0),
                focused_field_bottom_y: None,
            }),
            callbacks: RefCell::new(WindowCallbacks::default()),
            gpu_context,
            surface: Mutex::new(None),
            surface_alive: Cell::new(false),
            last_scroll_pos: Cell::new(None),
            drag_samples: RefCell::new(Vec::with_capacity(DRAG_SAMPLE_BUDGET)),
            fling: RefCell::new(None),
        }
    }

    /// Construct from an `AndroidDisplay` handle; convenience for callers that
    /// know they are using GPUI's stock display type.
    #[allow(dead_code)]
    pub(crate) fn from_display(
        handle: AnyWindowHandle,
        params: WindowParams,
        display: Rc<AndroidDisplay>,
        scale_factor: f32,
        gpu_context: GpuContext,
    ) -> Self {
        Self::new(handle, params, display, scale_factor, gpu_context)
    }

    /// Returns the handle this window was opened with. Useful for tests.
    #[allow(dead_code)]
    pub(crate) fn handle(&self) -> AnyWindowHandle {
        self.handle
    }

    /// Called from [`super::AndroidPlatform`] when Android publishes a new
    /// surface (`MainEvent::InitWindow` or `MainEvent::Resume` with a
    /// non-null window). Initialises a [`WgpuRenderer`] tied to the new
    /// surface, or — if a renderer already exists from a previous
    /// `attach_surface` cycle — *re-binds* the existing one to the new
    /// surface via [`WgpuRenderer::replace_surface`].
    ///
    /// **Why re-bind instead of recreate.** Returning from a child
    /// Activity (file picker, photo picker, Settings, …) tears down the
    /// `ANativeWindow` and gives us a fresh one. Dropping the renderer
    /// in that path is fatal: GPUI's Scene caches `AtlasTextureId`s into
    /// the *old* atlas, and the next paint after re-attach asks the new
    /// atlas about texture indices it doesn't have, which panics with
    /// "index out of bounds" inside [`WgpuAtlas::get_texture_info`].
    /// `replace_surface` keeps the device, queue, atlas and pipelines —
    /// only the surface itself swaps out, so the cached IDs stay valid.
    pub(crate) fn attach_surface(
        &self,
        native_window: NativeWindow,
        physical_size: Size<DevicePixels>,
    ) -> Result<()> {
        let surface = WindowSurface(native_window);
        *self.surface.lock() = Some(surface.clone());
        self.surface_alive.set(true);

        let config = WgpuSurfaceConfig {
            size: physical_size,
            transparent: false,
            preferred_present_mode: Some(gpui_wgpu::wgpu::PresentMode::Mailbox),
        };

        let mut state = self.state.borrow_mut();
        state.physical_size = physical_size;
        if let Some(renderer) = state.renderer.as_mut() {
            // Same renderer / device / atlas, fresh surface. Pull the
            // wgpu instance out of the shared `GpuContext`; it's the
            // same one the renderer was built with, so the device
            // recognises it.
            let context_borrow = self.gpu_context.borrow();
            let context = context_borrow
                .as_ref()
                .context("WgpuContext not initialised on attach_surface re-bind")?;
            renderer
                .replace_surface(&surface, config, &context.instance)
                .context("failed to re-bind Android surface on resume")?;
        } else {
            // First attach in the window's lifetime — build the renderer.
            let renderer = WgpuRenderer::new(self.gpu_context.clone(), &surface, config, None)
                .context("failed to initialise WgpuRenderer for the Android surface")?;
            let gpu_specs = renderer.gpu_specs();
            let sprite_atlas: Arc<dyn PlatformAtlas> = renderer.sprite_atlas().clone();
            state.renderer = Some(renderer);
            state.sprite_atlas = Some(sprite_atlas);
            state.gpu_specs = Some(gpu_specs);
        }
        Ok(())
    }

    /// Called from [`super::AndroidPlatform`] on `MainEvent::TerminateWindow`
    /// or `MainEvent::Pause` to release surface-bound GPU resources before
    /// the JVM destroys the `ANativeWindow`.
    ///
    /// **Keeps the renderer alive** — only the surface-bound state inside
    /// it is unconfigured. The atlas, device, queue and pipelines all
    /// survive the trip, so when [`Self::attach_surface`] runs next
    /// (after the picker / Settings activity returns), GPUI's cached
    /// scene sprites resolve to the same atlas they were uploaded to.
    pub(crate) fn detach_surface(&self) {
        self.surface_alive.set(false);
        let mut state = self.state.borrow_mut();
        if let Some(renderer) = state.renderer.as_mut() {
            // Just unconfigure — `destroy()` would drop the GPU
            // resources, and `replace_surface` on next attach would
            // panic without them. The wgpu `Surface` inside the
            // renderer still holds an internal refcount on the
            // `ANativeWindow` until `replace_surface` swaps it out, so
            // dropping our own `WindowSurface` reference here is safe.
            renderer.unconfigure_surface();
        }
        drop(state);
        *self.surface.lock() = None;
    }

    /// Notify the window of a new content rect (the area not covered by
    /// system bars or the IME). For now this is plumbed onto the bounds
    /// reported via `content_size()`; once GPUI gains a first-class
    /// "safe area inset" concept it should be exposed there directly.
    pub(crate) fn update_content_rect(
        &self,
        rect: android_activity::Rect,
        scale_factor: f32,
    ) {
        let logical_origin = Point {
            x: px(rect.left as f32 / scale_factor),
            y: px(rect.top as f32 / scale_factor),
        };
        let logical_size = Size {
            width: px(((rect.right - rect.left).max(0)) as f32 / scale_factor),
            height: px(((rect.bottom - rect.top).max(0)) as f32 / scale_factor),
        };
        let resize_payload = {
            let mut state = self.state.borrow_mut();
            state.bounds = Bounds {
                origin: logical_origin,
                size: logical_size,
            };
            apply_ime_inset(&mut state)
        };
        // GPUI's resize handler can re-enter the window (e.g. via
        // `scale_factor()`), so the state borrow above must drop before
        // invoking it. Same pattern in every other dispatch_* method.
        invoke_callback(self, |callbacks| {
            if let Some(resize) = callbacks.resize.as_mut() {
                resize(resize_payload, scale_factor);
            }
        });
    }

    /// Update the bottom IME inset (in logical pixels) and re-emit a resize
    /// so GPUI's layout shrinks. Called after the soft keyboard transitions
    /// visible / hidden and when Android publishes a new
    /// `MainEvent::InsetsChanged`.
    pub(crate) fn update_ime_inset(&self, ime_bottom_inset_logical: f32) {
        let new_inset = px(ime_bottom_inset_logical.max(0.0));
        let scale_factor;
        let resize_payload = {
            let mut state = self.state.borrow_mut();
            if (state.ime_bottom_inset - new_inset).abs() < px(0.5) {
                return;
            }
            state.ime_bottom_inset = new_inset;
            scale_factor = state.scale_factor;
            apply_ime_inset(&mut state)
        };
        invoke_callback(self, |callbacks| {
            if let Some(resize) = callbacks.resize.as_mut() {
                resize(resize_payload, scale_factor);
            }
        });
    }

    /// Logical bottom inset currently occupied by the soft keyboard. Used by
    /// the platform run loop to decide whether the IME is visible enough to
    /// warrant scrolling the focused field into view.
    pub(crate) fn ime_bottom_inset(&self) -> Pixels {
        self.state.borrow().ime_bottom_inset
    }

    /// True while we've asked Android to show the soft keyboard. Used by
    /// the platform run loop to skip the IME inset poll on idle frames.
    pub(crate) fn keyboard_visible(&self) -> bool {
        self.state.borrow().keyboard_visible
    }

    /// Last bounds reported to `update_ime_position`, in window-space
    /// logical pixels. Returned as the `y` of the bottom edge so the caller
    /// can decide whether the field is fully above the IME or partially
    /// obscured.
    #[allow(dead_code)]
    pub(crate) fn focused_field_bottom(&self) -> Option<Pixels> {
        self.state.borrow().focused_field_bottom_y
    }

    /// Update the window's logical bounds + physical pixel size from a
    /// configuration change (rotation, fold, font-scale).
    pub(crate) fn update_size(&self, new_size: Size<DevicePixels>, scale_factor: f32) {
        let logical = Size {
            width: px(new_size.width.0 as f32 / scale_factor),
            height: px(new_size.height.0 as f32 / scale_factor),
        };
        let resize_payload = {
            let mut state = self.state.borrow_mut();
            state.physical_size = new_size;
            state.scale_factor = scale_factor;
            state.bounds = Bounds {
                origin: Point::default(),
                size: logical,
            };
            if let Some(renderer) = state.renderer.as_mut() {
                renderer.update_drawable_size(new_size);
            }
            apply_ime_inset(&mut state)
        };
        invoke_callback(self, |callbacks| {
            if let Some(resize) = callbacks.resize.as_mut() {
                resize(resize_payload, scale_factor);
            }
        });
    }

    /// Forward a non-touch input event (keyboard, etc.) to the registered
    /// handler. Touch / mouse events go through
    /// [`Self::dispatch_motion_input`] instead because they need to carry
    /// the `MotionEvent` timestamp for fling-velocity estimation.
    pub(crate) fn dispatch_input(&self, event: PlatformInput) -> DispatchEventResult {
        self.invoke_input_callback(event)
    }

    /// Forward a touch / mouse event to the registered handler, plus a
    /// synthesised `ScrollWheel` companion when the gesture is part of a
    /// drag. `event_time_nanos` comes from the source `MotionEvent` — we
    /// use it instead of `Instant::now()` because Android batches pointer
    /// events into a single `InputAvailable` poll, so wall-clock
    /// timestamps would all land within a millisecond of each other and
    /// `release_velocity` would compute nonsense (we observed
    /// ~30000 px/s for a 1500 px/s gesture before this fix).
    ///
    /// Click handlers still fire normally on quick taps because GPUI's
    /// hit-test only requires `MouseDown` / `MouseUp` to land on the same
    /// element — true for taps, not for drags.
    pub(crate) fn dispatch_motion_input(
        &self,
        event: PlatformInput,
        event_time_nanos: i64,
    ) -> DispatchEventResult {
        if let PlatformInput::MouseMove(ev) = &event {
            self.state.borrow_mut().mouse_position = ev.position;
        }
        let scroll_companion = self.scroll_companion(&event, event_time_nanos);

        let result = self.invoke_input_callback(event);

        if let Some(scroll_event) = scroll_companion {
            self.invoke_input_callback(scroll_event);
        }

        result
    }

    /// Invokes the registered `on_input` callback with `event`, returning
    /// the dispatch result. Centralises the borrow scope so the caller
    /// can't accidentally hold `self.callbacks` across a re-entrant call.
    fn invoke_input_callback(&self, event: PlatformInput) -> DispatchEventResult {
        let mut callbacks = self.callbacks.borrow_mut();
        if let Some(handler) = callbacks.input.as_mut() {
            handler(event)
        } else {
            DispatchEventResult::default()
        }
    }

    /// Build a `ScrollWheel` event mirroring this touch event, or `None` if
    /// the input isn't part of a left-button drag. On `MouseDown` we cancel
    /// any in-flight fling; on `MouseMove` we accumulate samples for
    /// velocity estimation; on `MouseUp` we either start a fling (if the
    /// release velocity is significant) or emit `TouchPhase::Ended` and
    /// stop. `event_time_nanos` is the source `MotionEvent`'s
    /// monotonic-clock timestamp (nanoseconds), threaded through so
    /// release-velocity calculations don't get smeared by event batching.
    fn scroll_companion(
        &self,
        event: &PlatformInput,
        event_time_nanos: i64,
    ) -> Option<PlatformInput> {
        match event {
            PlatformInput::MouseDown(ev) if ev.button == MouseButton::Left => {
                // A new touch always interrupts the ongoing fling.
                self.fling.borrow_mut().take();
                self.last_scroll_pos.set(Some(ev.position));
                let mut samples = self.drag_samples.borrow_mut();
                samples.clear();
                samples.push((ev.position, event_time_nanos));
                Some(PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position: ev.position,
                    delta: ScrollDelta::Pixels(Point::default()),
                    modifiers: ev.modifiers,
                    touch_phase: TouchPhase::Started,
                }))
            }
            PlatformInput::MouseMove(ev) if ev.pressed_button == Some(MouseButton::Left) => {
                let last = self.last_scroll_pos.replace(Some(ev.position))?;
                {
                    let mut samples = self.drag_samples.borrow_mut();
                    samples.push((ev.position, event_time_nanos));
                    if samples.len() > DRAG_SAMPLE_BUDGET {
                        let drop = samples.len() - DRAG_SAMPLE_BUDGET;
                        samples.drain(..drop);
                    }
                }
                let delta = Point {
                    // GPUI's scroll handler does `scroll_offset.y += delta.y`
                    // and clamps `scroll_offset.y` to `[-scroll_max, 0]`.
                    // For natural touch (finger-up reveals content below),
                    // `delta.y` must be negative when the finger moves up,
                    // which is exactly `current - previous`.
                    x: ev.position.x - last.x,
                    y: ev.position.y - last.y,
                };
                Some(PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position: ev.position,
                    delta: ScrollDelta::Pixels(delta),
                    modifiers: ev.modifiers,
                    touch_phase: TouchPhase::Moved,
                }))
            }
            PlatformInput::MouseUp(ev) if ev.button == MouseButton::Left => {
                self.last_scroll_pos.take()?;
                let samples = std::mem::take(&mut *self.drag_samples.borrow_mut());
                let (vx, vy) = release_velocity(&samples);
                let speed = (vx * vx + vy * vy).sqrt();
                if speed > FLING_MIN_SPEED {
                    *self.fling.borrow_mut() = Some(FlingState {
                        velocity_x: vx,
                        velocity_y: vy,
                        last_tick: Instant::now(),
                        position: ev.position,
                        modifiers: ev.modifiers,
                    });
                    // Don't emit `Ended` yet — the fling is the gesture's
                    // continuation. `tick_fling` will emit `Ended` when
                    // friction pulls the velocity below threshold.
                    None
                } else {
                    Some(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: ev.position,
                        delta: ScrollDelta::Pixels(Point::default()),
                        modifiers: ev.modifiers,
                        touch_phase: TouchPhase::Ended,
                    }))
                }
            }
            _ => None,
        }
    }

    /// Advance the fling animation by one frame, if any. Called once per
    /// run-loop iteration from `AndroidPlatform::run` (the same place
    /// `dispatch_request_frame` is driven), so the cadence matches the
    /// device's vsync.
    pub(crate) fn tick_fling(&self) {
        let event = {
            let mut fling_borrow = self.fling.borrow_mut();
            let Some(fling) = fling_borrow.as_mut() else {
                return;
            };
            let now = Instant::now();
            let dt = now.duration_since(fling.last_tick).as_secs_f32();
            if dt <= 0.0 {
                return;
            }
            fling.last_tick = now;

            let decay = (-FRICTION_K * dt).exp();
            fling.velocity_x *= decay;
            fling.velocity_y *= decay;

            let speed =
                (fling.velocity_x * fling.velocity_x + fling.velocity_y * fling.velocity_y).sqrt();

            let position = fling.position;
            let modifiers = fling.modifiers;
            if speed < FLING_MIN_SPEED {
                *fling_borrow = None;
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position,
                    delta: ScrollDelta::Pixels(Point::default()),
                    modifiers,
                    touch_phase: TouchPhase::Ended,
                })
            } else {
                let delta = Point {
                    x: px(fling.velocity_x * dt),
                    y: px(fling.velocity_y * dt),
                };
                fling.position.x += delta.x;
                fling.position.y += delta.y;
                PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position: fling.position,
                    delta: ScrollDelta::Pixels(delta),
                    modifiers,
                    touch_phase: TouchPhase::Moved,
                })
            }
        };
        self.invoke_input_callback(event);
    }

    /// Translate an android-activity `TextInputState` (the IME's view of the
    /// editor's text) into GPUI's `PlatformInputHandler` calls.
    ///
    /// We diff the new state against the last one we observed and apply only
    /// the delta — passing `state.text` whole-cloth would mean re-inserting
    /// the entire IME buffer at the cursor on every keystroke (because the
    /// generic `EntityInputHandler::replace_text_in_range` impl falls back
    /// to inserting at the selection when the range is `None`). Composing
    /// edits go through `replace_and_mark_text_in_range` so the IME's
    /// preedit shows up underlined; finalised edits go through
    /// `replace_text_in_range`; transitions out of compose state call
    /// `unmark_text`.
    ///
    /// **Echo prevention.** The IME re-broadcasts `TextInputState` every
    /// time a side updates it — including after our own `set_text_input_state`
    /// pushes. We snapshot the state we apply and ignore re-deliveries that
    /// match it bit-for-bit, mirroring the makepad pattern.
    pub(crate) fn dispatch_text_event(&self, state: TextInputState) {
        log::info!(
            "dispatch_text_event: text={:?} compose={:?} selection={:?}",
            state.text,
            state.compose_region,
            state.selection,
        );
        let snapshot = TextInputStateSnapshot::from(&state);
        // Take a snapshot of the last state and the handler so the
        // PlatformInputHandler (which re-enters `state.borrow*` via the
        // foreground executor) doesn't deadlock on our outer borrow.
        let (previous, mut handler) = {
            let mut state_borrow = self.state.borrow_mut();
            if state_borrow.last_ime_state.as_ref() == Some(&snapshot) {
                return;
            }
            let previous = state_borrow.last_ime_state.clone();
            (previous, state_borrow.input_handler.take())
        };
        let Some(handler_ref) = handler.as_mut() else {
            log::warn!("dispatch_text_event: no input handler bound — text dropped");
            // Stash the snapshot so we don't keep re-warning for the same
            // delivery on every echo.
            self.state.borrow_mut().last_ime_state = Some(snapshot);
            return;
        };

        let prev_text = previous.as_ref().map(|s| s.text.as_str()).unwrap_or("");
        let prev_compose = previous.as_ref().and_then(|s| s.compose_range());
        let new_text = state.text.as_str();
        let new_compose = state
            .compose_region
            .as_ref()
            .map(|span| span.start..span.end);

        let (diff_range, inserted) = utf16_diff(prev_text, new_text);
        let text_changed = !inserted.is_empty() || diff_range.start != diff_range.end;
        let compose_changed = prev_compose != new_compose;

        log::info!(
            "dispatch_text_event: diff range={:?} inserted={:?} compose_changed={}",
            diff_range,
            inserted,
            compose_changed,
        );

        let edited = if text_changed {
            let inserted_utf16_len = utf16_len(&inserted);
            let inserted_end_utf16 = diff_range.start + inserted_utf16_len;
            // Mark the just-inserted span if the IME's compose region
            // exactly covers it. Otherwise fall back to plain replace, then
            // (below) handle compose_changed separately so the marked
            // region transitions cleanly.
            let mark_inserted = matches!(
                &new_compose,
                Some(c) if c.start == diff_range.start && c.end == inserted_end_utf16
            );
            if mark_inserted {
                handler_ref.replace_and_mark_text_in_range(
                    Some(diff_range),
                    &inserted,
                    Some(inserted_utf16_len..inserted_utf16_len),
                );
            } else {
                handler_ref.replace_text_in_range(Some(diff_range), &inserted);
                // If compose appeared but doesn't match the insertion, give
                // up on marking — falling through here is better than
                // mis-aligning the marked region with the document.
            }
            true
        } else if compose_changed && new_compose.is_none() {
            // Pure compose-region clear (e.g. IME finalised without text
            // change). Drop any existing marked range so subsequent edits
            // don't keep underlining stale text.
            handler_ref.unmark_text();
            true
        } else {
            false
        };

        // After applying the IME's delta, check whether the editor's
        // post-edit text actually matches what the IME sent us. If it
        // does (the common case — normal typing), do nothing: the IME's
        // view and the editor's view are already in sync, and pushing
        // anything back would cancel the IME's composing state mid-word
        // and make Samsung's keyboard re-commit its in-progress word as
        // fresh text (random duplicate letters).
        //
        // If they differ — number-field filtering, max-length truncation,
        // regex validation, etc. — push the editor's authoritative state
        // so the next IME delta diffs against the editor's view, not the
        // IME's stale buffer. Composition has to be cancelled in this
        // path because the editor's post-filter content can't be a valid
        // continuation of the IME's pre-filter compose region anyway.
        let edited_state = if edited {
            let mut adjusted: Option<Range<usize>> = None;
            let editor_text = handler_ref
                .text_for_range(0..usize::MAX, &mut adjusted)
                .unwrap_or_default();
            if editor_text == new_text {
                None
            } else {
                let selection = handler_ref
                    .selected_text_range(true)
                    .map(|s| s.range)
                    .unwrap_or(0..0);
                let utf16_total = utf16_len(&editor_text);
                let sel_start = selection.start.min(utf16_total);
                let sel_end = selection.end.min(utf16_total);
                Some(TextInputState {
                    text: editor_text,
                    selection: TextSpan {
                        start: sel_start,
                        end: sel_end,
                    },
                    compose_region: None,
                })
            }
        } else {
            None
        };

        // Restore the handler so the IME push (which can echo a TextEvent
        // synchronously on some Android versions) can re-enter
        // dispatch_text_event without panicking.
        {
            let mut state_borrow = self.state.borrow_mut();
            state_borrow.input_handler = handler;
            state_borrow.last_ime_state = Some(snapshot);
        }

        if let Some(authoritative) = edited_state
            && let Some(app) = super::android_app()
        {
            let our_snapshot = TextInputStateSnapshot::from(&authoritative);
            app.set_text_input_state(authoritative);
            // Treat our push as the new "last delivery" so the IME's echo
            // diffs to a no-op instead of replaying the same edit.
            self.state.borrow_mut().last_ime_state = Some(our_snapshot);
        }
    }

    pub(crate) fn dispatch_request_frame(&self, options: RequestFrameOptions) {
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.request_frame.as_mut() {
                handler(options);
            }
        });
    }

    pub(crate) fn dispatch_active_status(&self, active: bool) {
        self.state.borrow_mut().is_active = active;
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.active_status_change.as_mut() {
                handler(active);
            }
        });
    }

    pub(crate) fn set_appearance(&self, appearance: WindowAppearance) {
        self.state.borrow_mut().appearance = appearance;
        invoke_callback(self, |callbacks| {
            if let Some(handler) = callbacks.appearance_changed.as_mut() {
                handler();
            }
        });
    }

    /// Mutates `self.state` through the existing `RefCell` rather than
    /// requiring `&mut self`, because [`AndroidWindowHandle`] (the
    /// `Box<dyn PlatformWindow>` GPUI receives) wraps an `Rc<Self>` and
    /// `Rc::get_mut` would refuse — the platform also keeps a strong
    /// handle so it can route lifecycle events. The `&mut self` trait
    /// methods that delegate here exist only to satisfy
    /// [`PlatformWindow`].
    ///
    /// Only updates the stored handler — the actual `show_soft_input` /
    /// `hide_soft_input` JNI calls are deferred to
    /// [`Self::reconcile_keyboard`] so the per-frame `take_input_handler`
    /// → `set_input_handler` cycle GPUI runs doesn't flicker the IME.
    pub(crate) fn set_input_handler_inner(&self, input_handler: PlatformInputHandler) {
        self.state.borrow_mut().input_handler = Some(input_handler);
    }

    pub(crate) fn take_input_handler_inner(&self) -> Option<PlatformInputHandler> {
        // Don't clear `last_ime_state` here — GPUI runs a take/set cycle on
        // every paint, so resetting per-frame would defeat diff-based delta
        // application in `dispatch_text_event`. The state is reset instead
        // when the soft keyboard transitions hidden in
        // [`Self::reconcile_keyboard`].
        self.state.borrow_mut().input_handler.take()
    }

    /// Sync the soft-keyboard visibility with whatever input handler the
    /// last fully-drawn frame ended up registering. Called once per
    /// run-loop iteration from `AndroidPlatform::run`, *after*
    /// `dispatch_request_frame` has had a chance to push GPUI through its
    /// `take → draw → set` cycle. Without this debouncer the IME would
    /// pop and dismiss on every frame.
    pub(crate) fn reconcile_keyboard(&self) {
        let want_visible = self.state.borrow().input_handler.is_some();
        let already_visible = self.state.borrow().keyboard_visible;
        // Drain the widget-supplied IME descriptor every iteration so that
        // focus-switching between e.g. a text field and a number field
        // updates the keyboard layout even when the IME was already up
        // (which short-circuits the visibility-transition branch below).
        let requested = super::ime::take_requested_ime_type();
        let Some(app) = super::android_app() else {
            return;
        };
        if let Some((input_type, action)) = requested {
            app.set_ime_editor_info(input_type, action, ImeOptions::IME_FLAG_NO_FULLSCREEN);
            // Editor-info changes alone are silently ignored by Samsung's
            // IME (and others) until `InputMethodManager.restartInput`
            // runs — without this call, switching from a text field to a
            // number field while the keyboard is up wouldn't change the
            // keyboard layout. Cheap to call always; restartInput on a
            // visible IME is what natively-built EditTexts do too.
            super::ime::restart_input(&app);
        }
        if want_visible == already_visible {
            return;
        }
        if want_visible {
            // No widget claimed this focus → apply the default text
            // editor info so the IME has *something* to work with.
            if requested.is_none() {
                let (input_type, action, options) = super::ime::default_descriptor();
                app.set_ime_editor_info(input_type, action, options);
            }
            app.show_soft_input(false);
            // Sync the IME's view of the editor with what the input handler
            // thinks is current. Without this the IME starts with an empty
            // buffer and the first delta would replace position 0 — even if
            // the editor's cursor is somewhere else entirely.
            self.push_editor_state_to_ime(&app);
        } else {
            app.hide_soft_input(false);
            // Forget the IME's last state so the next time the keyboard
            // pops we re-baseline against the editor's *then-current* text
            // rather than diffing against a stale buffer.
            self.state.borrow_mut().last_ime_state = None;
        }
        self.state.borrow_mut().keyboard_visible = want_visible;
    }

    /// Send the editor's current text + selection to the IME so subsequent
    /// `TextInputState` deliveries diff against a known baseline. Called
    /// from [`Self::reconcile_keyboard`] when the keyboard becomes visible.
    fn push_editor_state_to_ime(&self, app: &android_activity::AndroidApp) {
        // Borrow the input handler exactly the way `dispatch_text_event`
        // does — temporarily moving it out of the cell so the handler's
        // foreground-executor re-entry can borrow `state` without panicking.
        let mut handler = self.state.borrow_mut().input_handler.take();
        let Some(handler_ref) = handler.as_mut() else {
            return;
        };
        let selection = handler_ref
            .selected_text_range(true)
            .map(|s| s.range)
            .unwrap_or(0..0);
        let mut adjusted: Option<Range<usize>> = None;
        // `text_for_range` clamps to the document length, so an unbounded
        // upper bound returns the full document.
        let text = handler_ref
            .text_for_range(0..usize::MAX, &mut adjusted)
            .unwrap_or_default();
        // Restore the handler before we hit the IME — `set_text_input_state`
        // can echo a `TextEvent` synchronously on some Android versions, and
        // `dispatch_text_event` needs the handler back in place when it
        // does.
        self.state.borrow_mut().input_handler = handler;

        let utf16_total = utf16_len(&text);
        let sel_start = selection.start.min(utf16_total);
        let sel_end = selection.end.min(utf16_total);
        let pushed = TextInputState {
            text,
            selection: TextSpan {
                start: sel_start,
                end: sel_end,
            },
            compose_region: None,
        };
        let snapshot = TextInputStateSnapshot::from(&pushed);
        app.set_text_input_state(pushed);
        // Treat our own push as the new "last delivery" so the very first
        // echo from the IME diffs to a no-op.
        self.state.borrow_mut().last_ime_state = Some(snapshot);
    }
}

/// Compute the UTF-16 range in `old` and the replacement text such that
/// applying `replace_text_in_range(diff_range, inserted)` to a document
/// equal to `old` produces `new`. Operates in UTF-16 code units because
/// that's the unit `PlatformInputHandler` ranges are documented in (matches
/// macOS's `NSTextInputClient` and the Android `CharSequence` surface that
/// android-activity's `TextSpan` indexes into).
fn utf16_diff(old: &str, new: &str) -> (Range<usize>, String) {
    let old_u16: Vec<u16> = old.encode_utf16().collect();
    let new_u16: Vec<u16> = new.encode_utf16().collect();
    let prefix = old_u16
        .iter()
        .zip(new_u16.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = old_u16[prefix..]
        .iter()
        .rev()
        .zip(new_u16[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let old_end = old_u16.len() - suffix;
    let new_end = new_u16.len() - suffix;
    let inserted = String::from_utf16(&new_u16[prefix..new_end]).unwrap_or_default();
    (prefix..old_end, inserted)
}

fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

/// Invoke a callback on the window's `WindowCallbacks` while making sure no
/// `state` borrow is live. The user-supplied callback may re-enter the
/// window's `PlatformWindow` methods, which all read `self.state`; without
/// this scoping pattern those re-entries would panic with
/// "RefCell already mutably borrowed".
fn invoke_callback(window: &AndroidWindow, f: impl FnOnce(&mut WindowCallbacks)) {
    debug_assert!(window.state.try_borrow().is_ok());
    let mut callbacks = window.callbacks.borrow_mut();
    f(&mut callbacks);
}

/// Compute the logical content size GPUI should see, given the current
/// content rect and the IME inset. Subtracts the IME inset from the bottom
/// of the bounds so layouts shrink while the keyboard is up. Returns the
/// post-inset size; the bounds origin / width are unchanged.
fn apply_ime_inset(state: &mut WindowState) -> Size<Pixels> {
    let mut size = state.bounds.size;
    let inset = state.ime_bottom_inset;
    if inset > px(0.0) {
        size.height = (size.height - inset).max(px(0.0));
    }
    size
}

/// Estimate the finger's velocity at release time from a window of recent
/// `(position, event_time_nanos)` samples. Returns `(vx, vy)` in logical
/// pixels per second.
///
/// Older samples are dropped because the finger's mid-drag speed is rarely
/// what the user means when they "throw" the content; the last
/// [`VELOCITY_WINDOW`] of motion is closer to release-time intent. If
/// fewer than two samples land in the window we widen to all available
/// samples; if there are still fewer than two we report zero velocity (no
/// fling).
///
/// Times are in nanoseconds since boot — the same epoch as
/// `MotionEvent::event_time` — *not* `Instant::now()`. Using wall-clock
/// here gives garbage velocities because Android batches pointer events
/// into one `InputAvailable` poll.
fn release_velocity(samples: &[(Point<Pixels>, i64)]) -> (f32, f32) {
    if samples.len() < 2 {
        return (0.0, 0.0);
    }
    let last = samples.last().expect("checked len above");
    let window_nanos = VELOCITY_WINDOW.as_nanos() as i64;
    let cutoff = last.1.saturating_sub(window_nanos);
    let first_idx = samples
        .iter()
        .position(|(_, t)| *t >= cutoff)
        .unwrap_or(samples.len() - 1);
    let first = &samples[first_idx];
    let dt_secs = (last.1 - first.1) as f32 / 1.0e9;
    if dt_secs <= 0.0 {
        return (0.0, 0.0);
    }
    let dx = f32::from(last.0.x) - f32::from(first.0.x);
    let dy = f32::from(last.0.y) - f32::from(first.0.y);
    (dx / dt_secs, dy / dt_secs)
}

impl HasWindowHandle for AndroidWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // We cannot return a borrowed handle that survives across the lock
        // guard's drop; for the common cases (wgpu builds the surface inside
        // `attach_surface`) callers don't need this. Return `Unavailable` so
        // misuse fails loudly instead of silently dangling.
        Err(HandleError::Unavailable)
    }
}

impl HasDisplayHandle for AndroidWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        // SAFETY: same as `impl HasDisplayHandle for WindowSurface` —
        // Android's display handle is a zero-sized marker with no
        // resource attached, so the borrow lifetime is trivially valid.
        Ok(unsafe {
            DisplayHandle::borrow_raw(RawDisplayHandle::Android(AndroidDisplayHandle::new()))
        })
    }
}

impl PlatformWindow for AndroidWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        let state = self.state.borrow();
        let mut bounds = state.bounds;
        if state.ime_bottom_inset > px(0.0) {
            bounds.size.height = (bounds.size.height - state.ime_bottom_inset).max(px(0.0));
        }
        bounds
    }

    fn is_maximized(&self) -> bool {
        true
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Fullscreen(self.bounds())
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds().size
    }

    fn resize(&mut self, _size: Size<Pixels>) {
        // Android decides window size — we cannot resize on demand. Recorded
        // for symmetry with the trait but otherwise a no-op.
    }

    fn scale_factor(&self) -> f32 {
        self.state.borrow().scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        self.state.borrow().appearance
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        Some(self.display.clone())
    }

    fn mouse_position(&self) -> Point<Pixels> {
        self.state.borrow().mouse_position
    }

    fn modifiers(&self) -> Modifiers {
        self.state.borrow().modifiers
    }

    fn capslock(&self) -> Capslock {
        self.state.borrow().capslock
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.set_input_handler_inner(input_handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.take_input_handler_inner()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {
        self.state.borrow_mut().is_active = true;
    }

    fn is_active(&self) -> bool {
        self.state.borrow().is_active
    }

    fn is_hovered(&self) -> bool {
        self.state.borrow().is_hovered
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, title: &str) {
        self.state.borrow_mut().title = title.to_owned();
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {}

    fn is_fullscreen(&self) -> bool {
        true
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.callbacks.borrow_mut().request_frame = Some(callback);
    }

    fn on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        self.callbacks.borrow_mut().input = Some(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().active_status_change = Some(callback);
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.borrow_mut().hover_status_change = Some(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.callbacks.borrow_mut().resize = Some(callback);
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().moved = Some(callback);
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.borrow_mut().should_close = Some(callback);
    }

    fn on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
        self.callbacks.borrow_mut().hit_test_window_control = Some(callback);
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.borrow_mut().close = Some(callback);
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.borrow_mut().appearance_changed = Some(callback);
    }

    fn draw(&self, scene: &Scene) {
        if !self.surface_alive.get() {
            return;
        }
        if let Some(renderer) = self.state.borrow_mut().renderer.as_mut() {
            renderer.draw(scene);
        }
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        // `sprite_atlas` is initialised in the `WindowState` constructor with
        // a `NoopAtlas`, so this `unwrap_or_else` arm should never fire.
        // Keep it as a safety net — a panic here would tear down the JVM
        // process during early window bring-up.
        self.state
            .borrow()
            .sprite_atlas
            .clone()
            .unwrap_or_else(|| Arc::new(NoopAtlas) as Arc<dyn PlatformAtlas>)
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        self.state
            .borrow()
            .renderer
            .as_ref()
            .map(|r| r.supports_dual_source_blending())
            .unwrap_or(false)
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        self.state.borrow().gpu_specs.clone()
    }

    fn update_ime_position(&self, bounds: Bounds<Pixels>) {
        // We can't push a cursor-anchor rect to the IME (GameActivity has
        // no Rust-visible View handle), but we *can* use this to remember
        // where the focused field sits so the run loop can keep it visible
        // when the keyboard appears.
        self.state.borrow_mut().focused_field_bottom_y = Some(bounds.bottom());
    }

    fn set_ime_kind(&self, kind: gpui::ImeKind) {
        use android_activity::input::{InputType, TextInputAction};
        // The IME module's thread-local is drained by
        // `reconcile_keyboard`, which calls `set_ime_editor_info` then
        // `restartInput` so the keyboard layout actually swaps even
        // when the IME is already visible.
        let (input_type, action) = match kind {
            gpui::ImeKind::Text => (
                InputType::TYPE_CLASS_TEXT
                    | InputType::TYPE_TEXT_FLAG_MULTI_LINE
                    | InputType::TYPE_TEXT_FLAG_NO_SUGGESTIONS,
                TextInputAction::None,
            ),
            gpui::ImeKind::Number => (
                InputType::TYPE_CLASS_NUMBER
                    | InputType::TYPE_NUMBER_FLAG_DECIMAL
                    | InputType::TYPE_NUMBER_FLAG_SIGNED,
                TextInputAction::Done,
            ),
        };
        super::ime::request_ime_input_type(input_type, action);
    }

    fn play_system_bell(&self) {
        if let Some(app) = super::android_app() {
            super::bell::ring(&app);
        }
    }
}
