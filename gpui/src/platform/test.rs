use pathfinder_geometry::vector::Vector2F;
use std::sync::Arc;
use std::{any::Any, rc::Rc};

struct Platform {
    dispatcher: Arc<dyn super::Dispatcher>,
    fonts: Arc<dyn super::FontSystem>,
}

struct Dispatcher;

pub struct Window {
    size: Vector2F,
    scale_factor: f32,
    current_scene: Option<crate::Scene>,
    event_handlers: Vec<Box<dyn FnMut(super::Event)>>,
    resize_handlers: Vec<Box<dyn FnMut(&mut dyn super::WindowContext)>>,
}

impl Platform {
    fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(super::current::FontSystem::new()),
        }
    }
}

impl super::Platform for Platform {
    fn on_menu_command(&self, _: Box<dyn FnMut(&str, Option<&dyn Any>)>) {}

    fn on_become_active(&self, _: Box<dyn FnMut()>) {}

    fn on_resign_active(&self, _: Box<dyn FnMut()>) {}

    fn on_event(&self, _: Box<dyn FnMut(crate::Event) -> bool>) {}

    fn on_open_files(&self, _: Box<dyn FnMut(Vec<std::path::PathBuf>)>) {}

    fn run(&self, _on_finish_launching: Box<dyn FnOnce() -> ()>) {
        unimplemented!()
    }

    fn dispatcher(&self) -> Arc<dyn super::Dispatcher> {
        self.dispatcher.clone()
    }

    fn fonts(&self) -> std::sync::Arc<dyn super::FontSystem> {
        self.fonts.clone()
    }

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn open_window(
        &self,
        _: usize,
        options: super::WindowOptions,
        _executor: Rc<super::executor::Foreground>,
    ) -> anyhow::Result<Box<dyn super::Window>> {
        Ok(Box::new(Window::new(options.bounds.size())))
    }

    fn key_window_id(&self) -> Option<usize> {
        None
    }

    fn set_menus(&self, _menus: Vec<crate::Menu>) {}

    fn quit(&self) {}

    fn prompt_for_paths(&self, _: super::PathPromptOptions) -> Option<Vec<std::path::PathBuf>> {
        None
    }

    fn copy(&self, _: &str) {}
}

impl Window {
    fn new(size: Vector2F) -> Self {
        Self {
            size,
            event_handlers: Vec::new(),
            resize_handlers: Vec::new(),
            scale_factor: 1.0,
            current_scene: None,
        }
    }
}

impl super::Dispatcher for Dispatcher {
    fn is_main_thread(&self) -> bool {
        true
    }

    fn run_on_main_thread(&self, task: async_task::Runnable) {
        task.run();
    }
}

impl super::WindowContext for Window {
    fn size(&self) -> Vector2F {
        self.size
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn present_scene(&mut self, scene: crate::Scene) {
        self.current_scene = Some(scene);
    }
}

impl super::Window for Window {
    fn on_event(&mut self, callback: Box<dyn FnMut(crate::Event)>) {
        self.event_handlers.push(callback);
    }

    fn on_resize(&mut self, callback: Box<dyn FnMut(&mut dyn super::WindowContext)>) {
        self.resize_handlers.push(callback);
    }
}

pub fn platform() -> impl super::Platform {
    Platform::new()
}
