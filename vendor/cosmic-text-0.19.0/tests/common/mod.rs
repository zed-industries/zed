use std::path::PathBuf;

use cosmic_text::{
    fontdb::Database, Align, Attrs, AttrsOwned, Buffer, Color, Ellipsize, Family, FontSystem,
    Metrics, Shaping, SwashCache, Wrap,
};
use tiny_skia::{Paint, Pixmap, Rect, Transform};

/// The test configuration.
/// The text in the test will be rendered as image using the one of the fonts found under the
/// `fonts` directory in this repository.
/// The image will then be compared to an image with the name `name` under the `tests/images`
/// directory in this repository.
/// If the images do not match the test will fail.
/// NOTE: if an environment variable `GENERATE_IMAGES` is set, the test will create and save
/// the images instead.
#[derive(Debug)]
pub struct DrawTestCfg {
    /// The name of the test.
    /// Will be used for the image name under the `tests/images` directory in this repository.
    name: String,
    /// The text to render to image
    text: String,
    /// Optional rich text spans. When set, `set_rich_text` is used instead of `set_text`.
    /// Each entry is `(text, attrs)` for one styled span.
    rich_spans: Option<Vec<(String, AttrsOwned)>>,
    /// The name, details of the font to be used.
    /// Expected to be one of the fonts found under the `fonts` directory in this repository.
    font: AttrsOwned,

    font_size: f32,
    line_height: f32,
    canvas_width: u32,
    canvas_height: u32,
    wrap: Wrap,
    ellipsize: Ellipsize,
    alignment: Option<Align>,
}

impl Default for DrawTestCfg {
    fn default() -> Self {
        let font = Attrs::new().family(Family::Serif);
        Self {
            name: "default".into(),
            font: AttrsOwned::new(&font),
            text: "".into(),
            rich_spans: None,
            font_size: 16.0,
            line_height: 20.0,
            canvas_width: 300,
            canvas_height: 300,
            wrap: Wrap::WordOrGlyph,
            ellipsize: Ellipsize::None,
            alignment: None,
        }
    }
}

impl DrawTestCfg {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn rich_text(
        mut self,
        spans: impl IntoIterator<Item = (&'static str, Attrs<'static>)>,
    ) -> Self {
        self.rich_spans = Some(
            spans
                .into_iter()
                .map(|(text, attrs)| (text.to_string(), AttrsOwned::new(&attrs)))
                .collect(),
        );
        self
    }

    pub fn font_attrs(mut self, attrs: Attrs) -> Self {
        self.font = AttrsOwned::new(&attrs);
        self
    }

    pub fn font_size(mut self, font_size: f32, line_height: f32) -> Self {
        self.font_size = font_size;
        self.line_height = line_height;
        self
    }

    pub fn canvas(mut self, width: u32, height: u32) -> Self {
        self.canvas_width = width;
        self.canvas_height = height;
        self
    }

    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn ellipsize(mut self, ellipsize: Ellipsize) -> Self {
        self.ellipsize = ellipsize;
        self
    }

    pub fn alignment(mut self, alignment: Option<Align>) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn validate_text_rendering(self) {
        let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        // Create a db with just the fonts in our fonts dir to make sure we only test those
        let fonts_path = PathBuf::from(&repo_dir).join("fonts");
        let mut font_db = Database::new();
        font_db.load_fonts_dir(fonts_path);
        let mut font_system = FontSystem::new_with_locale_and_db("En-US".into(), font_db);
        let mut swash_cache = SwashCache::new();
        let metrics = Metrics::new(self.font_size, self.line_height);
        let mut buffer = Buffer::new(&mut font_system, metrics);
        let mut buffer = buffer.borrow_with(&mut font_system);
        let margins = 5;
        buffer.set_wrap(self.wrap);
        buffer.set_ellipsize(self.ellipsize);
        buffer.set_size(
            Some((self.canvas_width - margins * 2) as f32),
            Some((self.canvas_height - margins * 2) as f32),
        );
        if let Some(ref spans) = self.rich_spans {
            buffer.set_rich_text(
                spans
                    .iter()
                    .map(|(text, attrs)| (text.as_str(), attrs.as_attrs())),
                &self.font.as_attrs(),
                Shaping::Advanced,
                self.alignment,
            );
        } else {
            buffer.set_text(
                &self.text,
                &self.font.as_attrs(),
                Shaping::Advanced,
                self.alignment,
            );
        }

        // Black
        let text_color = Color::rgb(0x00, 0x00, 0x00);

        let mut pixmap = Pixmap::new(self.canvas_width, self.canvas_height).unwrap();
        pixmap.fill(tiny_skia::Color::WHITE);

        buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
            let mut paint = Paint {
                anti_alias: true,
                ..Paint::default()
            };
            paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
            let rect = Rect::from_xywh(
                (x + margins as i32) as f32,
                (y + margins as i32) as f32,
                w as f32,
                h as f32,
            )
            .unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        });

        let image_name = format!("{}.png", self.name);
        let reference_image_path = PathBuf::from(&repo_dir)
            .join("tests")
            .join("images")
            .join(image_name);

        let generate_images = std::env::var("GENERATE_IMAGES")
            .map(|v| {
                let val = v.trim().to_ascii_lowercase();
                ["t", "true", "1"].iter().any(|&v| v == val)
            })
            .unwrap_or_default();

        if generate_images {
            pixmap.save_png(reference_image_path).unwrap();
        } else {
            let reference_image_data = std::fs::read(reference_image_path).unwrap();
            let image_data = pixmap.encode_png().unwrap();
            assert_eq!(
                reference_image_data, image_data,
                "rendering failed of {self:?}"
            )
        }
    }
}
