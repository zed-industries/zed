use std::{cell::Cell, rc::Rc, sync::Arc, time::Instant};

use futures::channel::oneshot;
use raw_window_handle::{HandleError, HasDisplayHandle, HasWindowHandle};

use crate::{
    App, Bounds, DispatchTree, GpuSpecs, Modifiers, Pixels, PlatformAtlas, PlatformDisplay,
    PlatformInput, PlatformInputHandler, PlatformWindow, Point, PromptLevel, ScaledPixels, Scene,
    Size, SubscriberSet, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    taffy::TaffyLayoutEngine,
};

use super::{DispatchEventResult, Frame, Window, WindowInvalidator};

pub(crate) fn fake_window(window: &Window, cx: &App) -> Window {
    Window {
        handle: window.handle.clone(),
        invalidator: WindowInvalidator::new(),
        removed: false,
        platform_window: Box::new(UnbackedWindow),
        display_id: None,
        sprite_atlas: window.sprite_atlas.clone(),
        text_system: window.text_system.clone(),
        rem_size: window.rem_size,
        rem_size_override_stack: Default::default(),
        viewport_size: Default::default(),
        layout_engine: Some(TaffyLayoutEngine::new()),
        root: None,
        element_id_stack: Default::default(),
        text_style_stack: Default::default(),
        rendered_entity_stack: Default::default(),
        element_offset_stack: Default::default(),
        element_opacity: None,
        content_mask_stack: Default::default(),
        requested_autoscroll: None,
        rendered_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
        next_frame: Frame::new(DispatchTree::new(cx.keymap.clone(), cx.actions.clone())),
        next_hitbox_id: Default::default(),
        next_tooltip_id: Default::default(),
        tooltip_bounds: None,
        next_frame_callbacks: Default::default(),
        dirty_views: Default::default(),
        focus_listeners: SubscriberSet::new(),
        focus_lost_listeners: SubscriberSet::new(),
        default_prevented: false,
        mouse_position: window.mouse_position,
        mouse_hit_test: Default::default(),
        modifiers: window.modifiers,
        scale_factor: window.scale_factor,
        bounds_observers: SubscriberSet::new(),
        appearance: window.appearance,
        appearance_observers: SubscriberSet::new(),
        active: Default::default(),
        hovered: Default::default(),
        needs_present: Default::default(),
        last_input_timestamp: Rc::new(Cell::new(Instant::now())),
        refreshing: false,
        activation_observers: SubscriberSet::new(),
        focus: None,
        focus_enabled: window.focus_enabled,
        pending_input: None,
        pending_modifier: window.pending_modifier.clone(),
        pending_input_observers: SubscriberSet::new(),
        prompt: None,
    }
}

struct UnbackedWindow;

impl HasDisplayHandle for UnbackedWindow {
    fn display_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::DisplayHandle<'_>, HandleError> {
        unimplemented!()
    }
}

impl HasWindowHandle for UnbackedWindow {
    fn window_handle(
        &self,
    ) -> std::result::Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        unimplemented!()
    }
}

impl PlatformWindow for UnbackedWindow {
    fn bounds(&self) -> Bounds<Pixels> {
        unimplemented!()
    }

    fn is_maximized(&self) -> bool {
        unimplemented!()
    }

    fn window_bounds(&self) -> WindowBounds {
        unimplemented!()
    }

    fn content_size(&self) -> Size<Pixels> {
        unimplemented!()
    }

    fn resize(&mut self, _size: Size<Pixels>) {
        unimplemented!()
    }

    fn scale_factor(&self) -> f32 {
        unimplemented!()
    }

    fn appearance(&self) -> WindowAppearance {
        unimplemented!()
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        unimplemented!()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        unimplemented!()
    }

    fn modifiers(&self) -> Modifiers {
        unimplemented!()
    }

    fn set_input_handler(&mut self, _input_handler: PlatformInputHandler) {
        unimplemented!()
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        unimplemented!()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _msg: &str,
        _detail: Option<&str>,
        _answers: &[&str],
    ) -> Option<oneshot::Receiver<usize>> {
        unimplemented!()
    }

    fn activate(&self) {
        unimplemented!()
    }

    fn is_active(&self) -> bool {
        unimplemented!()
    }

    fn is_hovered(&self) -> bool {
        unimplemented!()
    }

    fn set_title(&mut self, _title: &str) {
        unimplemented!()
    }

    fn set_background_appearance(&self, _background_appearance: WindowBackgroundAppearance) {
        unimplemented!()
    }

    fn minimize(&self) {
        unimplemented!()
    }

    fn zoom(&self) {
        unimplemented!()
    }

    fn toggle_fullscreen(&self) {
        unimplemented!()
    }

    fn is_fullscreen(&self) -> bool {
        unimplemented!()
    }

    fn on_request_frame(&self, _callback: Box<dyn FnMut(crate::RequestFrameOptions)>) {
        unimplemented!()
    }

    fn on_input(&self, _callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {
        unimplemented!()
    }

    fn on_active_status_change(&self, _callback: Box<dyn FnMut(bool)>) {
        unimplemented!()
    }

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {
        unimplemented!()
    }

    fn on_resize(&self, _callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        unimplemented!()
    }

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {
        unimplemented!()
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        unimplemented!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn draw(&self, _scene: &Scene) {
        unimplemented!()
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        unimplemented!()
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        unimplemented!()
    }

    fn update_ime_position(&self, _bounds: Bounds<ScaledPixels>) {
        unimplemented!()
    }
}
