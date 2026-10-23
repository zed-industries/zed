use std::borrow::Cow;
use std::ops::Range;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use hayro::hayro_interpret::font::{Glyph, GlyphRun};
use hayro::hayro_interpret::hayro_cmap::BfString;
use hayro::hayro_interpret::util::TransformExt as _;
use hayro::hayro_interpret::{
    BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageDrawProps,
    InterpreterCache, InterpreterSettings, SoftMask, interpret_page,
};
use hayro::hayro_syntax::{LoadPdfError, Pdf};
use hayro::kurbo::{Affine, BezPath, Point, Rect};
use hayro::vello_cpu::color::palette::css::WHITE;
use hayro::{PixmapSettings, RenderCache, RenderSettings};
use smallvec::SmallVec;
use unicode_bidi::{BidiInfo, Level};
use unicode_normalization::UnicodeNormalization as _;

/// Glyph outlines and advances in hayro are expressed in units of 1/1000 em.
const GLYPH_UNITS_PER_EM: f64 = 1000.0;
const GLYPH_ASCENT: f64 = 0.8 * GLYPH_UNITS_PER_EM;
const GLYPH_DESCENT: f64 = -0.2 * GLYPH_UNITS_PER_EM;
/// Type3 glyphs report no advance width, so assume half an em.
const TYPE3_FALLBACK_ADVANCE: f64 = 0.5 * GLYPH_UNITS_PER_EM;
/// A horizontal gap wider than this fraction of the font size counts as a word break when the
/// PDF positions words without explicit space glyphs.
const WORD_GAP_THRESHOLD: f32 = 0.15;

/// Size of a page in PDF points, after the page's rotation is applied.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSize {
    pub width: f32,
    pub height: f32,
}

pub struct PdfDocument {
    pdf: Pdf,
    page_sizes: Vec<PageSize>,
}

impl PdfDocument {
    pub fn parse(bytes: Vec<u8>) -> Result<Self> {
        let pdf = Pdf::new(Arc::new(bytes)).map_err(|error| match error {
            LoadPdfError::Decryption(_) => {
                anyhow!("Password-protected PDF files are not supported")
            }
            LoadPdfError::Invalid => anyhow!("The file is not a valid PDF document"),
        })?;
        let page_sizes = pdf
            .pages()
            .iter()
            .map(|page| {
                let (width, height) = page.render_dimensions();
                PageSize { width, height }
            })
            .collect::<Vec<_>>();
        if page_sizes.is_empty() {
            return Err(anyhow!("The PDF document has no pages"));
        }
        Ok(Self { pdf, page_sizes })
    }

    pub fn page_sizes(&self) -> &[PageSize] {
        &self.page_sizes
    }

    pub fn page_count(&self) -> usize {
        self.page_sizes.len()
    }

    pub fn extract_text(&self, page_index: usize) -> Result<PageText> {
        let page = self
            .pdf
            .pages()
            .get(page_index)
            .with_context(|| format!("page {page_index} is out of range"))?;
        let (width, height) = page.render_dimensions();
        let interpreter_cache = InterpreterCache::new();
        let mut context = Context::new(
            page.initial_transform(true).to_kurbo(),
            Rect::new(0.0, 0.0, width as f64, height as f64),
            &interpreter_cache,
            page.xref(),
            InterpreterSettings::default(),
        );
        let mut collector = TextCollector::default();
        interpret_page(page, &mut context, &mut collector);
        Ok(PageText::new(collector.glyphs))
    }
}

/// Renders pages of one document. The render cache is not thread-safe, so a renderer lives on
/// the thread that created it.
pub struct PageRenderer<'a> {
    document: &'a PdfDocument,
    cache: RenderCache<'a>,
}

/// Pixels in GPUI's image layout: BGRA with premultiplied alpha.
pub struct RenderedPixels {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

impl<'a> PageRenderer<'a> {
    pub fn new(document: &'a PdfDocument) -> Self {
        Self {
            document,
            cache: RenderCache::new(),
        }
    }

    pub fn render(&self, page_index: usize, scale: f32) -> Result<RenderedPixels> {
        let page = self
            .document
            .pdf
            .pages()
            .get(page_index)
            .with_context(|| format!("page {page_index} is out of range"))?;
        let pixmap = hayro::render(
            page,
            &self.cache,
            &InterpreterSettings::default(),
            &RenderSettings::default(),
            &PixmapSettings {
                x_scale: scale,
                y_scale: scale,
                bg_color: WHITE,
            },
        );
        let width = pixmap.width() as u32;
        let height = pixmap.height() as u32;
        let mut bgra = Vec::with_capacity(pixmap.data().len() * 4);
        for pixel in pixmap.data() {
            bgra.extend_from_slice(&[pixel.b, pixel.g, pixel.r, pixel.a]);
        }
        Ok(RenderedPixels {
            width,
            height,
            bgra,
        })
    }
}

/// One glyph, positioned in page space: PDF points with the origin at the top-left corner.
#[derive(Clone, Debug)]
pub struct TextGlyph {
    /// `None` when the PDF has no Unicode mapping for the glyph.
    pub text: Option<String>,
    pub bounds: Rect,
    pub font_size: f32,
}

#[derive(Clone, Debug)]
pub struct TextLine {
    pub glyphs: Range<usize>,
    pub bounds: Rect,
}

/// The text of a page grouped into visual lines. Lines keep their content-stream order, and the
/// glyphs of each line are sorted from left to right.
#[derive(Clone, Debug, Default)]
pub struct PageText {
    glyphs: Vec<TextGlyph>,
    lines: Vec<TextLine>,
    /// Whether most letters on the page are right-to-left. A short line such as a company name
    /// after an Arabic word has more Latin letters than Arabic ones, so the direction of mixed
    /// lines comes from the page rather than from the line.
    is_right_to_left: bool,
}

impl PageText {
    fn new(mut glyphs: Vec<TextGlyph>) -> Self {
        let mut lines: Vec<TextLine> = Vec::new();
        for (index, glyph) in glyphs.iter().enumerate() {
            match lines.last_mut() {
                Some(line) if continues_line(line, glyph) => {
                    line.glyphs.end = index + 1;
                    line.bounds = line.bounds.union(glyph.bounds);
                }
                _ => lines.push(TextLine {
                    glyphs: index..index + 1,
                    bounds: glyph.bounds,
                }),
            }
        }
        // Right-to-left text is stored in visual order by some producers and in reading order by
        // others; sorting by position gives one order to select in and to convert from.
        for line in &lines {
            glyphs[line.glyphs.clone()].sort_by(|a, b| a.bounds.x0.total_cmp(&b.bounds.x0));
        }
        let (right_to_left, left_to_right) = glyphs
            .iter()
            .filter_map(|glyph| glyph.text.as_deref())
            .flat_map(str::chars)
            .fold((0, 0), |(right_to_left, left_to_right), character| {
                if is_right_to_left(character) {
                    (right_to_left + 1, left_to_right)
                } else if character.is_alphabetic() {
                    (right_to_left, left_to_right + 1)
                } else {
                    (right_to_left, left_to_right)
                }
            });
        Self {
            glyphs,
            lines,
            is_right_to_left: right_to_left > left_to_right,
        }
    }

    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Returns the caret offset (a position between glyphs, in `0..=len`) nearest to `point`.
    pub fn caret_at(&self, point: Point) -> Option<usize> {
        let line = self.lines.iter().min_by(|a, b| {
            distance_squared(a.bounds, point).total_cmp(&distance_squared(b.bounds, point))
        })?;
        let caret = self.glyphs[line.glyphs.clone()]
            .iter()
            .position(|glyph| point.x < glyph.bounds.center().x)
            .map_or(line.glyphs.end, |offset| line.glyphs.start + offset);
        Some(caret)
    }

    /// Returns one rectangle per line covered by the glyphs in `range`.
    pub fn selection_rects(&self, range: Range<usize>) -> Vec<Rect> {
        self.lines
            .iter()
            .filter_map(|line| {
                let start = line.glyphs.start.max(range.start);
                let end = line.glyphs.end.min(range.end);
                (start < end).then(|| {
                    self.glyphs[start..end]
                        .iter()
                        .map(|glyph| glyph.bounds)
                        .reduce(|union, bounds| union.union(bounds))
                })?
            })
            .collect()
    }

    pub fn text_in_range(&self, range: Range<usize>) -> String {
        let mut lines = Vec::new();
        for line in &self.lines {
            let start = line.glyphs.start.max(range.start);
            let end = line.glyphs.end.min(range.end);
            if start >= end {
                continue;
            }
            let mut line_text = String::new();
            let mut previous: Option<&TextGlyph> = None;
            for glyph in &self.glyphs[start..end] {
                let Some(glyph_text) = glyph.text.as_deref() else {
                    continue;
                };
                if let Some(previous) = previous
                    && needs_word_break(previous, glyph)
                    && !line_text.ends_with(char::is_whitespace)
                    && !glyph_text.starts_with(char::is_whitespace)
                {
                    line_text.push(' ');
                }
                line_text.push_str(glyph_text);
                previous = Some(glyph);
            }
            lines.push(visual_to_reading_order(&line_text, self.is_right_to_left).into_owned());
        }
        lines.join("\n")
    }
}

fn continues_line(line: &TextLine, glyph: &TextGlyph) -> bool {
    let overlap = line.bounds.y1.min(glyph.bounds.y1) - line.bounds.y0.max(glyph.bounds.y0);
    let height = glyph.bounds.height().min(line.bounds.height());
    height > 0.0 && overlap > height * 0.5
}

fn is_right_to_left(character: char) -> bool {
    matches!(
        character as u32,
        0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF
    )
}

/// Converts a line of text in left-to-right visual order into reading order. Lines without
/// right-to-left characters are returned unchanged.
fn visual_to_reading_order(line: &str, is_right_to_left_page: bool) -> Cow<'_, str> {
    if !line.chars().any(is_right_to_left) {
        return Cow::Borrowed(line);
    }
    let paragraph_level = if is_right_to_left_page {
        Level::rtl()
    } else {
        Level::ltr()
    };
    // The bidi algorithm maps reading order to visual order. Within one line that mapping is its
    // own inverse, so applying it to visual text yields reading order.
    let bidi_info = BidiInfo::new(line, Some(paragraph_level));
    let Some(paragraph) = bidi_info.paragraphs.first() else {
        return Cow::Borrowed(line);
    };
    let reordered = bidi_info.reorder_line(paragraph, paragraph.range.clone());
    // PDFs often store Arabic as presentation forms (contextual glyph shapes); copied text
    // should contain the base letters instead.
    Cow::Owned(
        reordered
            .chars()
            .flat_map(|character| {
                let is_presentation_form =
                    matches!(character as u32, 0xFB50..=0xFDFF | 0xFE70..=0xFEFF);
                let mut normalized = SmallVec::<[char; 2]>::new();
                if is_presentation_form {
                    normalized.extend(std::iter::once(character).nfkc());
                } else {
                    normalized.push(character);
                }
                normalized
            })
            .collect(),
    )
}

fn needs_word_break(previous: &TextGlyph, glyph: &TextGlyph) -> bool {
    let font_size = previous.font_size.max(glyph.font_size).max(1.0);
    let gap = (glyph.bounds.x0 - previous.bounds.x1) as f32;
    gap > font_size * WORD_GAP_THRESHOLD
}

fn distance_squared(rect: Rect, point: Point) -> f64 {
    let dx = (rect.x0 - point.x).max(point.x - rect.x1).max(0.0);
    let dy = (rect.y0 - point.y).max(point.y - rect.y1).max(0.0);
    dx * dx + dy * dy
}

#[derive(Default)]
struct TextCollector {
    glyphs: Vec<TextGlyph>,
}

impl<'a> Device<'a> for TextCollector {
    fn draw_path(&mut self, _: &BezPath, _: DrawProps<'a>, _: &DrawMode) {}

    fn push_clip_path(&mut self, _: &ClipPath) {}

    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}

    fn draw_image(&mut self, _: Image<'a, '_>, _: ImageDrawProps<'a>) {}

    fn pop_clip(&mut self) {}

    fn pop_transparency_group(&mut self) {}

    fn draw_glyph_run(&mut self, run: &GlyphRun<'_, 'a>, props: DrawProps<'a>, _: &DrawMode) {
        for glyph in run.glyphs() {
            let transform: Affine = props.transform * glyph.transform();
            let advance = match &**glyph {
                Glyph::Outline(outline) => outline
                    .advance_width()
                    .map_or(TYPE3_FALLBACK_ADVANCE, f64::from),
                Glyph::Type3(_) => TYPE3_FALLBACK_ADVANCE,
            };
            let bounds =
                transform.transform_rect_bbox(Rect::new(0.0, GLYPH_DESCENT, advance, GLYPH_ASCENT));
            let origin = transform * Point::ZERO;
            let font_size = (transform * Point::new(0.0, GLYPH_UNITS_PER_EM) - origin).hypot();
            let text = glyph.as_unicode().map(|unicode| match unicode {
                BfString::Char(character) => character.to_string(),
                BfString::String(string) => string,
            });
            self.glyphs.push(TextGlyph {
                text,
                bounds,
                font_size: font_size as f32,
            });
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Builds a one-page PDF that shows each `(x, y, text)` entry with Helvetica 12pt.
    /// Coordinates are PDF user space: origin at the bottom-left of a 612x792 page.
    pub(crate) fn build_pdf(lines: &[(f32, f32, &str)]) -> Vec<u8> {
        let mut content = String::from("BT\n/F1 12 Tf\n");
        for (x, y, text) in lines {
            content.push_str(&format!("1 0 0 1 {x} {y} Tm\n({text}) Tj\n"));
        }
        content.push_str("ET\n");

        let objects = [
            "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
            "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R \
             /Resources << /Font << /F1 5 0 R >> >> >>"
                .to_string(),
            format!(
                "<< /Length {} >>\nstream\n{content}endstream",
                content.len()
            ),
            "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        ];

        let mut pdf = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::new();
        for (index, object) in objects.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", index + 1).as_bytes());
        }
        let xref_offset = pdf.len();
        pdf.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
        pdf.extend_from_slice(b"0000000000 65535 f \n");
        for offset in offsets {
            pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        pdf.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n",
                objects.len() + 1
            )
            .as_bytes(),
        );
        pdf
    }

    fn page_text(lines: &[(f32, f32, &str)]) -> PageText {
        let document = PdfDocument::parse(build_pdf(lines)).expect("test PDF should parse");
        document
            .extract_text(0)
            .expect("text extraction should succeed")
    }

    #[test]
    fn test_parse_reports_page_sizes() {
        let document = PdfDocument::parse(build_pdf(&[])).expect("test PDF should parse");
        assert_eq!(
            document.page_sizes(),
            &[PageSize {
                width: 612.0,
                height: 792.0
            }]
        );
    }

    #[test]
    fn test_parse_rejects_invalid_data() {
        let error = PdfDocument::parse(b"not a pdf".to_vec())
            .err()
            .expect("invalid data should fail");
        assert_eq!(error.to_string(), "The file is not a valid PDF document");
    }

    #[test]
    fn test_render_produces_opaque_bgra_pixels() {
        let document = PdfDocument::parse(build_pdf(&[(72.0, 720.0, "Hello")]))
            .expect("test PDF should parse");
        let pixels = PageRenderer::new(&document)
            .render(0, 0.5)
            .expect("render should succeed");
        assert_eq!((pixels.width, pixels.height), (306, 396));
        assert_eq!(pixels.bgra.len(), 306 * 396 * 4);
        assert_eq!(&pixels.bgra[..4], &[255, 255, 255, 255]);
    }

    #[test]
    fn test_extracted_text_keeps_lines_and_positions() {
        let text = page_text(&[(72.0, 720.0, "Hello World"), (72.0, 700.0, "Second line")]);
        assert_eq!(
            text.text_in_range(0..text.len()),
            "Hello World\nSecond line"
        );

        let first = &text.glyphs[0];
        assert!((first.bounds.x0 - 72.0).abs() < 0.01);
        // 792 - 720 = 72pt from the top edge is the baseline; the glyph box extends above it.
        assert!(first.bounds.y0 < 72.0 && first.bounds.y1 > 72.0);
        assert!((first.font_size - 12.0).abs() < 0.01);
    }

    #[test]
    fn test_word_gaps_without_space_glyphs_become_spaces() {
        let text = page_text(&[(72.0, 720.0, "Hello"), (120.0, 720.0, "World")]);
        assert_eq!(text.text_in_range(0..text.len()), "Hello World");
    }

    fn glyph(text: &str, x: f64) -> TextGlyph {
        TextGlyph {
            text: Some(text.to_string()),
            bounds: Rect::new(x, 100.0, x + 6.0, 112.0),
            font_size: 12.0,
        }
    }

    #[test]
    fn test_right_to_left_text_is_copied_in_reading_order() {
        // "سلام" stored in visual order with presentation forms, the way many PDFs store Arabic.
        let visual = PageText::new(vec![
            glyph("\u{FEE1}", 10.0),
            glyph("\u{FE8D}", 16.0),
            glyph("\u{FEDD}", 22.0),
            glyph("\u{FEB1}", 28.0),
        ]);
        assert_eq!(visual.text_in_range(0..4), "سلام");

        // The same word stored in reading order, so the glyphs move from right to left.
        let reading = PageText::new(vec![
            glyph("س", 28.0),
            glyph("ل", 22.0),
            glyph("ا", 16.0),
            glyph("م", 10.0),
        ]);
        assert_eq!(reading.text_in_range(0..4), "سلام");

        // On a right-to-left page, Latin words keep their own order and follow the Arabic word
        // they come after in reading order, even when the line has more Latin letters.
        let mixed = PageText::new(vec![
            glyph("m", 10.0),
            glyph("a", 16.0),
            glyph("c", 22.0),
            glyph("O", 28.0),
            glyph("S", 34.0),
            glyph("م", 50.0),
            glyph("ا", 56.0),
            glyph("ل", 62.0),
            glyph("س", 68.0),
            glyph_on_line("ب", 10.0, 200.0),
            glyph_on_line("ي", 16.0, 200.0),
            glyph_on_line("ت", 22.0, 200.0),
        ]);
        assert_eq!(mixed.text_in_range(0..9), "سلام macOS");

        // On a left-to-right page, an Arabic word inside an English line only turns around itself.
        let english = PageText::new(vec![
            glyph("O", 10.0),
            glyph("K", 16.0),
            glyph("م", 40.0),
            glyph("ا", 46.0),
            glyph("ل", 52.0),
            glyph("س", 58.0),
            glyph("n", 80.0),
            glyph("o", 86.0),
            glyph("w", 92.0),
        ]);
        assert_eq!(english.text_in_range(0..9), "OK سلام now");
    }

    fn glyph_on_line(text: &str, x: f64, y: f64) -> TextGlyph {
        TextGlyph {
            bounds: Rect::new(x, y, x + 6.0, y + 12.0),
            ..glyph(text, x)
        }
    }

    #[test]
    fn test_caret_and_selection() {
        let text = page_text(&[(72.0, 720.0, "ABCDEF"), (72.0, 700.0, "GHIJ")]);
        let glyph_b = text.glyphs[1].bounds;
        let glyph_e = text.glyphs[4].bounds;

        // A point on the left half of a glyph puts the caret before it.
        let start = text
            .caret_at(Point::new(glyph_b.x0 + 0.5, glyph_b.center().y))
            .expect("caret should be found");
        let end = text
            .caret_at(Point::new(glyph_e.x0 + 0.5, glyph_e.center().y))
            .expect("caret should be found");
        assert_eq!((start, end), (1, 4));
        assert_eq!(text.text_in_range(start..end), "BCD");

        // Points past the end of a line put the caret after its last glyph.
        let line_end = text
            .caret_at(Point::new(500.0, glyph_b.center().y))
            .expect("caret should be found");
        assert_eq!(line_end, 6);

        let rects = text.selection_rects(3..8);
        assert_eq!(rects.len(), 2);
        assert!(rects[0].y1 <= rects[1].y0 + 1.0);
    }
}
