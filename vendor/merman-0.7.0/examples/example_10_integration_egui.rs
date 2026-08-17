use eframe::egui::{self, ColorImage, TextureHandle};
use merman::render::{
    HeadlessRenderer,
    raster::{RasterFitBox, RasterOptions, svg_to_png},
};
use std::path::Path;

const DEFAULT_SOURCE: &str = r#"flowchart TD
    Source[Mermaid source] --> Engine[HeadlessRenderer]
    Engine --> Svg[SVG export]
    Engine --> Png[PNG preview texture]
"#;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Merman egui Integration",
        options,
        Box::new(|cc| Ok(Box::new(MermanEguiApp::new(cc)))),
    )
}

struct MermanEguiApp {
    source: String,
    renderer: HeadlessRenderer,
    raster: RasterOptions,
    svg: Option<String>,
    png: Option<Vec<u8>>,
    texture: Option<TextureHandle>,
    status: String,
    dirty: bool,
}

impl MermanEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Keep renderer and raster options as host state instead of rebuilding policy per frame.
        let mut app = Self {
            source: DEFAULT_SOURCE.to_string(),
            renderer: HeadlessRenderer::new()
                .with_strict_parsing()
                .with_diagram_id("egui-integration-preview"),
            raster: RasterOptions::default()
                .with_fit_to(RasterFitBox::contain(960, 640))
                .with_scale(1.0)
                .with_background("white"),
            svg: None,
            png: None,
            texture: None,
            status: String::new(),
            dirty: true,
        };
        app.render(&cc.egui_ctx);
        app
    }

    fn render(&mut self, ctx: &egui::Context) {
        match self.render_inner(ctx) {
            Ok(()) => {
                self.status = "rendered preview".to_string();
                self.dirty = false;
            }
            Err(err) => {
                self.status = err;
                self.texture = None;
                self.svg = None;
                self.png = None;
            }
        }
    }

    fn render_inner(&mut self, ctx: &egui::Context) -> Result<(), String> {
        // The preview path rasterizes the resvg-safe SVG contract used by non-browser hosts.
        let svg = self
            .renderer
            .render_svg_resvg_safe_sync(&self.source)
            .map_err(|err| format!("render failed: {err}"))?
            .ok_or_else(|| "no Mermaid diagram detected".to_string())?;
        let png =
            svg_to_png(&svg, &self.raster).map_err(|err| format!("PNG preview failed: {err}"))?;
        let image = png_to_color_image(&png)?;
        self.texture =
            Some(ctx.load_texture("merman-preview", image, egui::TextureOptions::LINEAR));
        self.svg = Some(svg);
        self.png = Some(png);
        Ok(())
    }

    fn save_svg(&mut self) {
        let Some(svg) = &self.svg else {
            self.status = "nothing to save; render first".to_string();
            return;
        };
        let path = Path::new("target/merman-egui-preview.svg");
        match write_file(path, svg.as_bytes()) {
            Ok(()) => self.status = format!("wrote {}", path.display()),
            Err(err) => self.status = format!("save SVG failed: {err}"),
        }
    }

    fn save_png(&mut self) {
        let Some(png) = &self.png else {
            self.status = "nothing to save; render first".to_string();
            return;
        };
        let path = Path::new("target/merman-egui-preview.png");
        match write_file(path, png) {
            Ok(()) => self.status = format!("wrote {}", path.display()),
            Err(err) => self.status = format!("save PNG failed: {err}"),
        }
    }
}

impl eframe::App for MermanEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Render").clicked() {
                    self.render(ctx);
                }
                if ui.button("Save SVG").clicked() {
                    self.save_svg();
                }
                if ui.button("Save PNG").clicked() {
                    self.save_png();
                }
                ui.label(&self.status);
            });
        });

        egui::SidePanel::left("source")
            .resizable(true)
            .default_width(380.0)
            .show(ctx, |ui| {
                ui.heading("Mermaid source");
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.source)
                        .code_editor()
                        .desired_rows(28)
                        .lock_focus(true),
                );
                if response.changed() {
                    self.dirty = true;
                }
                if self.dirty {
                    ui.label("changed; press Render to refresh");
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PNG preview");
            if let Some(texture) = &self.texture {
                let available = ui.available_size();
                let image_size = fit_size(texture.size_vec2(), available);
                ui.image((texture.id(), image_size));
            } else {
                ui.label("No preview available.");
            }
        });
    }
}

fn fit_size(source: egui::Vec2, bounds: egui::Vec2) -> egui::Vec2 {
    // Fit into the visible panel without upscaling a smaller preview texture.
    if source.x <= 0.0 || source.y <= 0.0 || bounds.x <= 0.0 || bounds.y <= 0.0 {
        return source;
    }
    let scale = (bounds.x / source.x).min(bounds.y / source.y).min(1.0);
    source * scale
}

fn png_to_color_image(bytes: &[u8]) -> Result<ColorImage, String> {
    let decoder = png::Decoder::new(bytes);
    let mut reader = decoder
        .read_info()
        .map_err(|err| format!("invalid PNG preview: {err}"))?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|err| format!("invalid PNG frame: {err}"))?;
    let pixels = &buf[..info.buffer_size()];

    match info.color_type {
        png::ColorType::Rgba => Ok(ColorImage::from_rgba_unmultiplied(
            [info.width as usize, info.height as usize],
            pixels,
        )),
        png::ColorType::Rgb => {
            let mut rgba = Vec::with_capacity((info.width as usize) * (info.height as usize) * 4);
            for rgb in pixels.chunks_exact(3) {
                rgba.extend_from_slice(&[rgb[0], rgb[1], rgb[2], 255]);
            }
            Ok(ColorImage::from_rgba_unmultiplied(
                [info.width as usize, info.height as usize],
                &rgba,
            ))
        }
        other => Err(format!("unsupported PNG preview color type: {other:?}")),
    }
}

fn write_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, bytes)
}
