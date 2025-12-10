use std::sync::Arc;
use std::sync::OnceLock;

use anyhow::Result;
use fontdb::{Database, Source};
use gpui::{App, ClipboardItem, Hsla, RenderImage, Window, img};
use settings::Settings;
use theme::ThemeSettings;
use typst::compile;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source as TypstSource, VirtualPath, package::PackageSpec};
use typst::text::{Font, FontBook, FontInfo};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use ui::{IntoElement, Styled, div, prelude::*};

use crate::outputs::OutputContent;

const SVG_SCALE_FACTOR: f32 = 2.0;

const MITEX_TYPST_TOML: &str = include_str!("mitex_package/typst.toml");
const MITEX_LIB: &str = include_str!("mitex_package/lib.typ");
const MITEX_MAIN: &str = include_str!("mitex_package/mitex.typ");
const MITEX_SPECS_MOD: &str = include_str!("mitex_package/specs/mod.typ");
const MITEX_SPECS_PRELUDE: &str = include_str!("mitex_package/specs/prelude.typ");
const MITEX_SPECS_LATEX_STANDARD: &str = include_str!("mitex_package/specs/latex/standard.typ");

fn mitex_package_spec() -> PackageSpec {
    PackageSpec {
        namespace: "preview".into(),
        name: "mitex".into(),
        version: typst::syntax::package::PackageVersion { major: 0, minor: 2, patch: 4 },
    }
}

fn get_mitex_file(path: &str) -> Option<&'static str> {
    match path {
        "/typst.toml" => Some(MITEX_TYPST_TOML),
        "/lib.typ" => Some(MITEX_LIB),
        "/mitex.typ" => Some(MITEX_MAIN),
        "/specs/mod.typ" => Some(MITEX_SPECS_MOD),
        "/specs/prelude.typ" => Some(MITEX_SPECS_PRELUDE),
        "/specs/latex/standard.typ" => Some(MITEX_SPECS_LATEX_STANDARD),
        _ => None,
    }
}

struct FontSlot {
    path: std::path::PathBuf,
    index: u32,
    font: OnceLock<Option<Font>>,
}

impl FontSlot {
    fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                let data = std::fs::read(&self.path).ok()?;
                Font::new(Bytes::new(data), self.index)
            })
            .clone()
    }
}

struct LatexWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    source: TypstSource,
    fonts: Vec<FontSource>,
    mitex_package: PackageSpec,
}

struct EmbeddedFont {
    data: &'static [u8],
    index: u32,
    font: OnceLock<Option<Font>>,
}

impl EmbeddedFont {
    fn new(data: &'static [u8], index: u32) -> Self {
        Self {
            data,
            index,
            font: OnceLock::new(),
        }
    }

    fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| Font::new(Bytes::new(self.data.to_vec()), self.index))
            .clone()
    }
}

enum FontSource {
    System(FontSlot),
    Embedded(EmbeddedFont),
}

impl FontSource {
    fn get(&self) -> Option<Font> {
        match self {
            FontSource::System(slot) => slot.get(),
            FontSource::Embedded(embedded) => embedded.get(),
        }
    }
}

impl LatexWorld {
    fn new(typst_content: &str) -> Result<Self> {
        let mut book = FontBook::new();
        let mut fonts: Vec<FontSource> = Vec::new();

        // First, add embedded fonts from typst-assets (includes math fonts)
        for data in typst_assets::fonts() {
            for (index, font) in Font::iter(Bytes::new(data.to_vec())).enumerate() {
                book.push(font.info().clone());
                fonts.push(FontSource::Embedded(EmbeddedFont::new(data, index as u32)));
            }
        }

        // Then add system fonts
        let mut db = Database::new();
        db.load_system_fonts();

        for face in db.faces() {
            let path = match &face.source {
                Source::File(path) | Source::SharedFile(path, _) => path.clone(),
                Source::Binary(_) => continue,
            };

            let info = db.with_face_data(face.id, FontInfo::new);
            if let Some(Some(info)) = info {
                book.push(info);
                fonts.push(FontSource::System(FontSlot {
                    path,
                    index: face.index,
                    font: OnceLock::new(),
                }));
            }
        }

        let main_id = FileId::new(None, VirtualPath::new("/main.typ"));
        let source = TypstSource::new(main_id, typst_content.to_string());

        Ok(Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            source,
            fonts,
            mitex_package: mitex_package_spec(),
        })
    }
}

impl World for LatexWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<TypstSource> {
        if id == self.source.id() {
            return Ok(self.source.clone());
        }

        if let Some(package) = id.package() {
            if package == &self.mitex_package {
                let path = id.vpath().as_rooted_path().to_string_lossy();
                if let Some(content) = get_mitex_file(&path) {
                    return Ok(TypstSource::new(id, content.to_string()));
                }
            }
        }

        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(package) = id.package() {
            if package == &self.mitex_package {
                let path = id.vpath().as_rooted_path().to_string_lossy();
                if let Some(content) = get_mitex_file(&path) {
                    return Ok(Bytes::new(content.as_bytes().to_vec()));
                }
            }
        }

        Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).and_then(|slot| slot.get())
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

fn hsla_to_typst_rgb(color: Hsla) -> String {
    let rgba = color.to_rgb();
    format!(
        "rgb({}, {}, {})",
        (rgba.r * 255.0) as u8,
        (rgba.g * 255.0) as u8,
        (rgba.b * 255.0) as u8
    )
}

fn latex_to_svg(latex: &str, font_size_pt: f32, text_color: Hsla) -> Result<String> {
    let typst_math = mitex::convert_math(latex, None)
        .map_err(|e| anyhow::anyhow!("Failed to convert LaTeX to Typst: {}", e))?;

    let color_str = hsla_to_typst_rgb(text_color);

    let typst_content = format!(
        r#"#import "@preview/mitex:0.2.4": mitex-scope
#set page(width: auto, height: auto, margin: 8pt, fill: none)
#set text(size: {font_size}pt, fill: {color})
#eval("$" + "{math}" + "$", scope: mitex-scope)"#,
        font_size = font_size_pt,
        color = color_str,
        math = typst_math.replace('\\', "\\\\").replace('"', "\\\"")
    );

    let world = LatexWorld::new(&typst_content)?;

    let result = compile::<PagedDocument>(&world);

    let document = result
        .output
        .map_err(|errors| {
            let error_messages: Vec<String> = errors
                .iter()
                .map(|e| e.message.to_string())
                .collect();
            anyhow::anyhow!("Typst compilation failed: {}", error_messages.join(", "))
        })?;

    if document.pages.is_empty() {
        return Err(anyhow::anyhow!("No pages generated"));
    }

    let svg = typst_svg::svg(&document.pages[0]);
    Ok(svg)
}

pub struct LatexView {
    raw_latex: String,
    height: u32,
    width: u32,
    image: Arc<RenderImage>,
}

impl LatexView {
    pub fn from(latex_data: &str, cx: &App) -> Result<Self> {
        let settings = ThemeSettings::get_global(cx);
        let font_size: f32 = settings.buffer_font_size(cx).into();
        let text_color = cx.theme().colors().text;

        let svg = latex_to_svg(latex_data, font_size, text_color)?;

        let renderer = cx.svg_renderer();
        let image = renderer.render_single_frame(svg.as_bytes(), 1.0, true)?;

        let size = image.size(0);
        let width = (size.width.0 as f32 / SVG_SCALE_FACTOR) as u32;
        let height = (size.height.0 as f32 / SVG_SCALE_FACTOR) as u32;

        Ok(LatexView {
            raw_latex: latex_data.to_string(),
            height,
            width,
            image,
        })
    }
}

impl Render for LatexView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let line_height = window.line_height();

        let (height, width) = if self.height as f32 / f32::from(line_height) == u8::MAX as f32 {
            let height = u8::MAX as f32 * line_height;
            let width = self.width as f32 * height / self.height as f32;
            (height, width)
        } else {
            (self.height.into(), self.width.into())
        };

        let image = self.image.clone();

        div().h(height).w(width).child(img(image))
    }
}

impl OutputContent for LatexView {
    fn clipboard_content(&self, _window: &Window, _cx: &App) -> Option<ClipboardItem> {
        Some(ClipboardItem::new_string(self.raw_latex.clone()))
    }

    fn has_clipboard_content(&self, _window: &Window, _cx: &App) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::latex_to_svg;
    use gpui::Hsla;

    #[test]
    fn test_latex_to_typst_conversion() {
        let result = mitex::convert_math(r"\frac{1}{2}", None);
        assert!(result.is_ok());
        let typst = result.unwrap();
        println!("frac output: {}", typst);
        assert!(!typst.is_empty());
    }

    #[test]
    fn test_simple_latex_conversion() {
        let result = mitex::convert_math(r"x^2 + y^2 = z^2", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sqrt_conversion() {
        let result = mitex::convert_math(r"\sqrt{\pi}", None);
        println!("sqrt output: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_integral_conversion() {
        let result = mitex::convert_math(r"\int_0^\infty e^{-x^2} dx", None);
        println!("integral output: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_latex_to_svg_simple() {
        let test_color = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        };
        let result = latex_to_svg(r"\frac{1}{2}", 14.0, test_color);
        println!("SVG result: {:?}", result.as_ref().map(|s| &s[..100.min(s.len())]));
        assert!(result.is_ok());
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_latex_to_svg_sqrt() {
        let test_color = Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        };
        let result = latex_to_svg(r"\sqrt{\pi}", 14.0, test_color);
        println!("sqrt SVG result: {:?}", result.as_ref().map(|s| &s[..100.min(s.len())]));
        assert!(result.is_ok());
    }
}
