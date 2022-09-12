use crate::{
    geometry::vector::{vec2f, Vector2F},
    platform::{self, mac::renderer::Renderer},
    Event, FontSystem, Scene,
};
use cocoa::{
    appkit::{
        NSSquareStatusItemLength, NSStatusBar, NSStatusItem, NSView, NSViewHeightSizable,
        NSViewWidthSizable, NSWindow,
    },
    base::{id, nil, YES},
    foundation::NSSize,
};
use foreign_types::ForeignTypeRef;
use objc::{msg_send, rc::StrongPtr, sel, sel_impl};
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct StatusItem(Rc<RefCell<StatusItemState>>);

struct StatusItemState {
    native_item: StrongPtr,
    renderer: Renderer,
    event_callback: Option<Box<dyn FnMut(Event) -> bool>>,
}

impl StatusItem {
    pub fn add(fonts: Arc<dyn FontSystem>) -> Self {
        unsafe {
            let renderer = Renderer::new(fonts);
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let native_item =
                StrongPtr::retain(status_bar.statusItemWithLength_(NSSquareStatusItemLength));

            let button = native_item.button();
            button.setAutoresizingMask_(NSViewWidthSizable | NSViewHeightSizable);
            button.setWantsBestResolutionOpenGLSurface_(YES);
            button.setLayer(renderer.layer().as_ptr() as id);

            Self(Rc::new(RefCell::new(StatusItemState {
                native_item,
                renderer,
                event_callback: None,
            })))
        }
    }
}

impl platform::Window for StatusItem {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        self.0.borrow_mut().event_callback = Some(callback);
    }

    fn on_active_status_change(&mut self, _: Box<dyn FnMut(bool)>) {}

    fn on_resize(&mut self, _: Box<dyn FnMut()>) {}

    fn on_fullscreen(&mut self, _: Box<dyn FnMut(bool)>) {}

    fn on_should_close(&mut self, _: Box<dyn FnMut() -> bool>) {}

    fn on_close(&mut self, _: Box<dyn FnOnce()>) {}

    fn set_input_handler(&mut self, _: Box<dyn crate::InputHandler>) {}

    fn prompt(
        &self,
        _: crate::PromptLevel,
        _: &str,
        _: &[&str],
    ) -> postage::oneshot::Receiver<usize> {
        panic!()
    }

    fn activate(&self) {}

    fn set_title(&mut self, _: &str) {}

    fn set_edited(&mut self, _: bool) {}

    fn show_character_palette(&self) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_full_screen(&self) {}

    fn size(&self) -> Vector2F {
        self.0.borrow().size()
    }

    fn scale_factor(&self) -> f32 {
        self.0.borrow().scale_factor()
    }

    fn titlebar_height(&self) -> f32 {
        0.
    }

    fn present_scene(&mut self, scene: Scene) {
        self.0.borrow_mut().renderer.render(&scene);
    }
}

impl StatusItemState {
    fn size(&self) -> Vector2F {
        let NSSize { width, height, .. } = unsafe { NSView::frame(self.native_item.button()) }.size;
        vec2f(width as f32, height as f32)
    }

    fn scale_factor(&self) -> f32 {
        unsafe {
            let window: id = msg_send![self.native_item.button(), window];
            window.screen().backingScaleFactor() as f32
        }
    }
}
