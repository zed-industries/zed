use crate::fonts::FontId;
use pathfinder_geometry::{
    rect,
    vector::{vec2f, vec2i, Vector2F},
};
use std::rc::Rc;
use std::sync::Arc;

struct App {
    dispatcher: Arc<dyn super::Dispatcher>,
}

struct Dispatcher;
struct FontSystem;

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
        }
    }
}

impl super::App for App {
    fn dispatcher(&self) -> Arc<dyn super::Dispatcher> {
        self.dispatcher.clone()
    }

    fn activate(&self, ignoring_other_apps: bool) {}

    fn open_window(
        &self,
        options: super::WindowOptions,
        executor: Rc<super::executor::Foreground>,
    ) -> anyhow::Result<Box<dyn super::Window>> {
        Ok(Box::new(Window::new(options.bounds.size())))
    }

    fn fonts(&self) -> std::sync::Arc<dyn super::FontSystem> {
        Arc::new(FontSystem)
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

impl super::FontSystem for FontSystem {
    fn load_family(&self, name: &str) -> anyhow::Result<Vec<FontId>> {
        Ok(vec![FontId(0)])
    }

    fn select_font(
        &self,
        font_ids: &[FontId],
        properties: &font_kit::properties::Properties,
    ) -> anyhow::Result<FontId> {
        Ok(font_ids[0])
    }

    fn font_metrics(&self, font_id: FontId) -> font_kit::metrics::Metrics {
        font_kit::metrics::Metrics {
            units_per_em: 1,
            ascent: 0.,
            descent: 0.,
            line_gap: 0.,
            underline_position: 1.,
            underline_thickness: 1.,
            cap_height: 12.,
            x_height: 12.,
            bounding_box: rect::RectF::new(vec2f(0., 0.), vec2f(10., 10.)),
        }
    }

    fn typographic_bounds(
        &self,
        font_id: FontId,
        glyph_id: crate::fonts::GlyphId,
    ) -> anyhow::Result<rect::RectF> {
        Ok(rect::RectF::new(vec2f(0., 0.), vec2f(0., 0.)))
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<crate::fonts::GlyphId> {
        Some(0)
    }

    fn rasterize_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: crate::fonts::GlyphId,
        subpixel_shift: Vector2F,
        scale_factor: f32,
    ) -> Option<(rect::RectI, Vec<u8>)> {
        Some((rect::RectI::new(vec2i(0, 0), vec2i(0, 0)), vec![]))
    }

    fn layout_str(
        &self,
        text: &str,
        font_size: f32,
        runs: &[(std::ops::Range<usize>, FontId)],
    ) -> crate::text_layout::Line {
        crate::text_layout::Line {
            width: 0.,
            runs: vec![],
            len: 0,
            font_size: 12.,
        }
    }
}

pub fn app() -> impl super::App {
    App::new()
}
