use pathfinder_geometry::vector::Vector2F;
use std::rc::Rc;
use std::sync::Arc;

struct App {
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

pub struct WindowContext {}

impl App {
    fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(super::current::FontSystem::new()),
        }
    }
}

impl super::App for App {
    fn dispatcher(&self) -> Arc<dyn super::Dispatcher> {
        self.dispatcher.clone()
    }

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn open_window(
        &self,
        options: super::WindowOptions,
        _executor: Rc<super::executor::Foreground>,
    ) -> anyhow::Result<Box<dyn super::Window>> {
        Ok(Box::new(Window::new(options.bounds.size())))
    }

    fn fonts(&self) -> std::sync::Arc<dyn super::FontSystem> {
        self.fonts.clone()
    }
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

pub fn app() -> impl super::App {
    App::new()
}
