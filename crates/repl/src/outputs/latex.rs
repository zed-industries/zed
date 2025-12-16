use anyhow::{Context as _, Result, anyhow};
use gpui::{App, ClipboardItem, Hsla, Pixels, RenderImage, Window, img, px};
use settings::Settings;
use std::sync::{Arc, OnceLock};
use theme::ThemeSettings;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source as TypstSource, VirtualPath};
use typst::text::{Font, FontBook, FontInfo};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World, compile};
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
        version: typst::syntax::package::PackageVersion {
            major: 0,
            minor: 2,
            patch: 4,
        },
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

struct SharedFonts {
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
}

fn shared_fonts() -> &'static SharedFonts {
    static FONTS: OnceLock<SharedFonts> = OnceLock::new();
    FONTS.get_or_init(|| {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        for data in typst_assets::fonts() {
            let bytes = Bytes::new(data.to_vec());
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }

        load_system_fonts(&mut book, &mut fonts);

        SharedFonts {
            book: LazyHash::new(book),
            fonts,
        }
    })
}

fn load_system_fonts(book: &mut FontBook, fonts: &mut Vec<Font>) {
    let mut database = fontdb::Database::new();
    database.load_system_fonts();

    for face in database.faces() {
        let path = match &face.source {
            fontdb::Source::File(path) | fontdb::Source::SharedFile(path, _) => path,
            fontdb::Source::Binary(_) => continue,
        };

        let Ok(data) = std::fs::read(path) else {
            continue;
        };

        let info = database.with_face_data(face.id, FontInfo::new);
        if let Some(Some(info)) = info {
            if let Some(font) = Font::new(Bytes::new(data), face.index) {
                book.push(info);
                fonts.push(font);
            }
        }
    }
}

struct LatexWorld {
    library: LazyHash<Library>,
    source: TypstSource,
    mitex_package: PackageSpec,
}

impl LatexWorld {
    fn new(typst_content: &str) -> Self {
        let main_id = FileId::new(None, VirtualPath::new("/main.typ"));
        let source = TypstSource::new(main_id, typst_content.to_string());

        Self {
            library: LazyHash::new(Library::default()),
            source,
            mitex_package: mitex_package_spec(),
        }
    }
}

impl World for LatexWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &shared_fonts().book
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
        shared_fonts().fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

fn hsla_to_typst_color(color: Hsla) -> String {
    let rgba = color.to_rgb();
    format!(
        "rgb({}, {}, {})",
        (rgba.r * 255.0) as u8,
        (rgba.g * 255.0) as u8,
        (rgba.b * 255.0) as u8
    )
}

fn render_latex_to_svg(latex: &str, font_size: f32, text_color: Hsla) -> Result<String> {
    let typst_math = mitex::convert_math(latex, None)
        .map_err(|error| anyhow!("{}", error))
        .context("converting LaTeX to Typst math")?;

    let color = hsla_to_typst_color(text_color);
    let escaped_math = typst_math.replace('\\', "\\\\").replace('"', "\\\"");

    let typst_source = format!(
        r#"#import "@preview/mitex:0.2.4": mitex-scope
#set page(width: auto, height: auto, margin: 8pt, fill: none)
#set text(size: {font_size}pt, fill: {color})
#eval("$" + "{escaped_math}" + "$", scope: mitex-scope)"#
    );

    let world = LatexWorld::new(&typst_source);
    let result = compile::<PagedDocument>(&world);

    let document = result.output.map_err(|errors| {
        let messages: Vec<_> = errors.iter().map(|e| e.message.to_string()).collect();
        anyhow!("{}", messages.join("; "))
    }).context("compiling Typst document")?;

    let page = document
        .pages
        .first()
        .ok_or_else(|| anyhow!("no pages generated"))?;

    Ok(typst_svg::svg(page))
}

pub struct LatexView {
    raw_latex: String,
    width: Pixels,
    height: Pixels,
    image: Arc<RenderImage>,
}

impl LatexView {
    pub fn from(latex: &str, cx: &App) -> Result<Self> {
        let settings = ThemeSettings::get_global(cx);
        let font_size: f32 = settings.buffer_font_size(cx).into();
        let text_color = cx.theme().colors().text;

        let svg = render_latex_to_svg(latex, font_size, text_color)?;

        let renderer = cx.svg_renderer();
        let image = renderer
            .render_single_frame(svg.as_bytes(), 1.0, true)
            .context("rendering LaTeX SVG")?;

        let size = image.size(0);
        let width = px(size.width.0 as f32 / SVG_SCALE_FACTOR);
        let height = px(size.height.0 as f32 / SVG_SCALE_FACTOR);

        Ok(Self {
            raw_latex: latex.to_string(),
            width,
            height,
            image,
        })
    }
}

impl Render for LatexView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h(self.height)
            .w(self.width)
            .child(img(self.image.clone()))
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
    use super::*;

    fn test_color() -> Hsla {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 1.0,
        }
    }

    #[test]
    fn test_simple_fraction() {
        let result = render_latex_to_svg(r"\frac{1}{2}", 14.0, test_color());
        let svg = result.expect("render_latex_to_svg failed");
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_pythagorean_theorem() {
        let result = mitex::convert_math(r"x^2 + y^2 = z^2", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_square_root() {
        let result = render_latex_to_svg(r"\sqrt{\pi}", 14.0, test_color());
        result.expect("render_latex_to_svg failed");
    }

    #[test]
    fn test_integral() {
        let result = mitex::convert_math(r"\int_0^\infty e^{-x^2} dx", None);
        assert!(result.is_ok());
    }
}
