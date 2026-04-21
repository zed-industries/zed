use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ::util::ResultExt;
use anyhow::Result;
use gpui::*;
use windows::Win32::{
    Foundation::*,
    Graphics::{DirectManipulation::*, Gdi::*},
    System::Com::*,
    UI::{Input::Pointer::*, WindowsAndMessaging::*},
};

use crate::*;

/// Default viewport size in pixels. The actual content size doesn't matter
/// because we're using the viewport only for gesture recognition, not for
/// visual output.
const DEFAULT_VIEWPORT_SIZE: i32 = 1000;

pub(crate) struct DirectManipulationHandler {
    manager: IDirectManipulationManager,
    update_manager: IDirectManipulationUpdateManager,
    viewport: IDirectManipulationViewport,
    _handler_cookie: u32,
    window: HWND,
    scale_factor: Rc<Cell<f32>>,
    pending_events: Rc<RefCell<Vec<PlatformInput>>>,
}

impl DirectManipulationHandler {
    pub fn new(window: HWND, scale_factor: f32) -> Result<Self> {
        unsafe {
            let manager: IDirectManipulationManager =
                CoCreateInstance(&DirectManipulationManager, None, CLSCTX_INPROC_SERVER)?;

            let update_manager: IDirectManipulationUpdateManager = manager.GetUpdateManager()?;

            let viewport: IDirectManipulationViewport = manager.CreateViewport(None, window)?;

            let configuration = DIRECTMANIPULATION_CONFIGURATION_INTERACTION
                | DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X
                | DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y
                | DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA
                | DIRECTMANIPULATION_CONFIGURATION_RAILS_X
                | DIRECTMANIPULATION_CONFIGURATION_RAILS_Y
                | DIRECTMANIPULATION_CONFIGURATION_SCALING;
            viewport.ActivateConfiguration(configuration)?;

            viewport.SetViewportOptions(
                DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE
                    | DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING,
            )?;

            let mut rect = RECT {
                left: 0,
                top: 0,
                right: DEFAULT_VIEWPORT_SIZE,
                bottom: DEFAULT_VIEWPORT_SIZE,
            };
            viewport.SetViewportRect(&mut rect)?;

            manager.Activate(window)?;
            viewport.Enable()?;

            let scale_factor = Rc::new(Cell::new(scale_factor));
            let pending_events = Rc::new(RefCell::new(Vec::new()));

            let event_handler: IDirectManipulationViewportEventHandler =
                DirectManipulationEventHandler::new(
                    window,
                    Rc::clone(&scale_factor),
                    Rc::clone(&pending_events),
                )
                .into();

            let handler_cookie = viewport.AddEventHandler(Some(window), &event_handler)?;

            update_manager.Update(None)?;

            Ok(Self {
                manager,
                update_manager,
                viewport,
                _handler_cookie: handler_cookie,
                window,
                scale_factor,
                pending_events,
            })
        }
    }

    pub fn set_scale_factor(&self, scale_factor: f32) {
        self.scale_factor.set(scale_factor);
    }

    pub fn on_pointer_hit_test(&self, wparam: WPARAM) {
        unsafe {
            let pointer_id = wparam.loword() as u32;
            let mut pointer_type = POINTER_INPUT_TYPE::default();
            if GetPointerType(pointer_id, &mut pointer_type).is_ok() && pointer_type == PT_TOUCHPAD
            {
                self.viewport.SetContact(pointer_id).log_err();
            }
        }
    }

    pub fn update(&self) {
        unsafe {
            self.update_manager.Update(None).log_err();
        }
    }

    pub fn drain_events(&self) -> Vec<PlatformInput> {
        std::mem::take(&mut *self.pending_events.borrow_mut())
    }
}

impl Drop for DirectManipulationHandler {
    fn drop(&mut self) {
        unsafe {
            self.viewport.Stop().log_err();
            self.viewport.Abandon().log_err();
            self.manager.Deactivate(self.window).log_err();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GestureKind {
    None,
    Scroll,
    Pinch,
}

#[windows_core::implement(IDirectManipulationViewportEventHandler)]
struct DirectManipulationEventHandler {
    window: HWND,
    scale_factor: Rc<Cell<f32>>,
    gesture_kind: Cell<GestureKind>,
    last_scale: Cell<f32>,
    last_x_offset: Cell<f32>,
    last_y_offset: Cell<f32>,
    scroll_phase: Cell<TouchPhase>,
    pending_events: Rc<RefCell<Vec<PlatformInput>>>,
}

impl DirectManipulationEventHandler {
    fn new(
        window: HWND,
        scale_factor: Rc<Cell<f32>>,
        pending_events: Rc<RefCell<Vec<PlatformInput>>>,
    ) -> Self {
        Self {
            window,
            scale_factor,
            gesture_kind: Cell::new(GestureKind::None),
            last_scale: Cell::new(1.0),
            last_x_offset: Cell::new(0.0),
            last_y_offset: Cell::new(0.0),
            scroll_phase: Cell::new(TouchPhase::Started),
            pending_events,
        }
    }

    fn end_gesture(&self) {
        let position = self.mouse_position();
        let modifiers = current_modifiers();
        match self.gesture_kind.get() {
            GestureKind::Scroll => {
                self.pending_events
                    .borrow_mut()
                    .push(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position,
                        delta: ScrollDelta::Pixels(point(px(0.0), px(0.0))),
                        modifiers,
                        touch_phase: TouchPhase::Ended,
                    }));
            }
            GestureKind::Pinch => {
                self.pending_events
                    .borrow_mut()
                    .push(PlatformInput::Pinch(PinchEvent {
                        position,
                        delta: 0.0,
                        modifiers,
                        phase: TouchPhase::Ended,
                    }));
            }
            GestureKind::None => {}
        }
        self.gesture_kind.set(GestureKind::None);
    }

    fn mouse_position(&self) -> Point<Pixels> {
        let scale_factor = self.scale_factor.get();
        unsafe {
            let mut point: POINT = std::mem::zeroed();
            let _ = GetCursorPos(&mut point);
            let _ = ScreenToClient(self.window, &mut point);
            logical_point(point.x as f32, point.y as f32, scale_factor)
        }
    }
}

impl IDirectManipulationViewportEventHandler_Impl for DirectManipulationEventHandler_Impl {
    fn OnViewportStatusChanged(
        &self,
        viewport: windows_core::Ref<'_, IDirectManipulationViewport>,
        current: DIRECTMANIPULATION_STATUS,
        previous: DIRECTMANIPULATION_STATUS,
    ) -> windows_core::Result<()> {
        if current == previous {
            return Ok(());
        }

        // A new gesture interrupted inertia, so end the old sequence.
        if current == DIRECTMANIPULATION_RUNNING && previous == DIRECTMANIPULATION_INERTIA {
            self.end_gesture();
        }

        if current == DIRECTMANIPULATION_READY {
            self.end_gesture();

            // Reset the content transform so the viewport is ready for the next gesture.
            // ZoomToRect triggers a second RUNNING -> READY cycle, so prevent an infinite loop here.
            if self.last_scale.get() != 1.0
                || self.last_x_offset.get() != 0.0
                || self.last_y_offset.get() != 0.0
            {
                if let Some(viewport) = viewport.as_ref() {
                    unsafe {
                        viewport
                            .ZoomToRect(
                                0.0,
                                0.0,
                                DEFAULT_VIEWPORT_SIZE as f32,
                                DEFAULT_VIEWPORT_SIZE as f32,
                                false,
                            )
                            .log_err();
                    }
                }
            }

            self.last_scale.set(1.0);
            self.last_x_offset.set(0.0);
            self.last_y_offset.set(0.0);
        }

        Ok(())
    }

    fn OnViewportUpdated(
        &self,
        _viewport: windows_core::Ref<'_, IDirectManipulationViewport>,
    ) -> windows_core::Result<()> {
        Ok(())
    }

    fn OnContentUpdated(
        &self,
        _viewport: windows_core::Ref<'_, IDirectManipulationViewport>,
        content: windows_core::Ref<'_, IDirectManipulationContent>,
    ) -> windows_core::Result<()> {
        let content = content.as_ref().ok_or(E_POINTER)?;

        // Get the 6-element content transform: [scale, 0, 0, scale, tx, ty]
        let mut xform = [0.0f32; 6];
        unsafe {
            content.GetContentTransform(&mut xform)?;
        }

        let scale = xform[0];
        let scale_factor = self.scale_factor.get();
        let x_offset = xform[4] / scale_factor;
        let y_offset = xform[5] / scale_factor;

        if scale == 0.0 {
            return Ok(());
        }

        let last_scale = self.last_scale.get();
        let last_x = self.last_x_offset.get();
        let last_y = self.last_y_offset.get();

        if float_equals(scale, last_scale)
            && float_equals(x_offset, last_x)
            && float_equals(y_offset, last_y)
        {
            return Ok(());
        }

        let position = self.mouse_position();
        let modifiers = current_modifiers();

        // Direct Manipulation reports both translation and scale in every content update.
        // Translation values can shift during a pinch due to the zoom center shifting.
        // We classify each gesture as either scroll or pinch and only emit one type of event.
        // We allow Scroll -> Pinch (a pinch can start with a small pan) but not the reverse.
        if !float_equals(scale, 1.0) {
            if self.gesture_kind.get() != GestureKind::Pinch {
                self.end_gesture();
                self.gesture_kind.set(GestureKind::Pinch);
                self.pending_events
                    .borrow_mut()
                    .push(PlatformInput::Pinch(PinchEvent {
                        position,
                        delta: 0.0,
                        modifiers,
                        phase: TouchPhase::Started,
                    }));
            }
        } else if self.gesture_kind.get() == GestureKind::None {
            self.gesture_kind.set(GestureKind::Scroll);
            self.scroll_phase.set(TouchPhase::Started);
        }

        match self.gesture_kind.get() {
            GestureKind::Scroll => {
                let dx = x_offset - last_x;
                let dy = y_offset - last_y;
                let touch_phase = self.scroll_phase.get();
                self.scroll_phase.set(TouchPhase::Moved);
                self.pending_events
                    .borrow_mut()
                    .push(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position,
                        delta: ScrollDelta::Pixels(point(px(dx), px(dy))),
                        modifiers,
                        touch_phase,
                    }));
            }
            GestureKind::Pinch => {
                let scale_delta = scale / last_scale;
                self.pending_events
                    .borrow_mut()
                    .push(PlatformInput::Pinch(PinchEvent {
                        position,
                        delta: scale_delta - 1.0,
                        modifiers,
                        phase: TouchPhase::Moved,
                    }));
            }
            GestureKind::None => {}
        }

        self.last_scale.set(scale);
        self.last_x_offset.set(x_offset);
        self.last_y_offset.set(y_offset);

        Ok(())
    }
}

fn float_equals(f1: f32, f2: f32) -> bool {
    const EPSILON_SCALE: f32 = 0.00001;
    (f1 - f2).abs() < EPSILON_SCALE * f1.abs().max(f2.abs()).max(EPSILON_SCALE)
}
