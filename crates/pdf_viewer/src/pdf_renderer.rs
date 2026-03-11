use std::sync::Arc;

use anyhow::{Result, anyhow};
use gpui::RenderImage;
use hayro::hayro_interpret::InterpreterSettings;
use hayro::hayro_syntax::Pdf;
use hayro::text::extract_text;
use hayro::vello_cpu::color::AlphaColor;
use hayro::vello_cpu::color::Srgb;

use hayro::RenderSettings;
use image::Frame;
use smallvec::SmallVec;

// Font-size-relative threshold for same-line detection in hit testing.
// Glyphs with a vertical gap smaller than this fraction of font size are
// considered part of the same visual line.
const SAME_LINE_THRESHOLD: f32 = 0.8;

// Horizontal gap (as fraction of font size) beyond which a synthetic
// space is inserted between consecutive same-line glyphs.  Handles PDFs
// that position words without explicit space glyphs.
const SPACE_DIST: f32 = 0.15;


/// Dimensions of a single PDF page in PDF points (1/72 inch).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageDimensions {
    pub width: f32,
    pub height: f32,
}

/// Lightweight metadata extracted from a PDF without rendering any pages.
#[derive(Clone, Debug)]
pub struct PdfMetadata {
    pub page_count: usize,
    pub page_dimensions: Vec<PageDimensions>,
}

pub struct RenderedPage {
    pub image: Arc<RenderImage>,
    pub page_width: u32,
    pub page_height: u32,
}

pub fn open_pdf(pdf_bytes: &[u8]) -> Result<Pdf> {
    Pdf::new(pdf_bytes.to_vec()).map_err(|err| anyhow!("Failed to parse PDF document: {err:?}"))
}

/// Parse the PDF and extract page count + dimensions without rendering.
/// This is fast (~6ms for a 98-page document).
pub fn parse_metadata(pdf_bytes: &[u8]) -> Result<PdfMetadata> {
    let pdf = open_pdf(pdf_bytes)?;
    let pages = pdf.pages();

    let mut page_dimensions = Vec::new();
    for page in pages.iter() {
        let (width, height) = page.render_dimensions();
        page_dimensions.push(PageDimensions { width, height });
    }

    Ok(PdfMetadata {
        page_count: page_dimensions.len(),
        page_dimensions,
    })
}

fn pixmap_to_render_image(pixmap: &hayro::vello_cpu::Pixmap) -> Result<Arc<RenderImage>> {
    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;

    // hayro's Pixmap pixel data is premultiplied RGBA.
    // GPUI expects BGRA with premultiplied alpha, so swap R and B.
    let raw_pixels = pixmap.data();
    let mut bgra_pixels: Vec<u8> = Vec::with_capacity(raw_pixels.len() * 4);
    for pixel in raw_pixels {
        bgra_pixels.push(pixel.b);
        bgra_pixels.push(pixel.g);
        bgra_pixels.push(pixel.r);
        bgra_pixels.push(pixel.a);
    }

    let image_buffer =
        image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(width, height, bgra_pixels)
            .ok_or_else(|| anyhow!("Failed to create image buffer from rendered PDF page"))?;

    let render_image = Arc::new(RenderImage::new(SmallVec::from_elem(
        Frame::new(image_buffer),
        1,
    )));

    Ok(render_image)
}

#[derive(Clone)]
pub struct TextGlyph {
    pub character: String,
    /// Position in PDF points, top-left origin.
    pub x: f32,
    pub y: f32,
    /// Advance width from the PDF text state.
    pub width: f32,
    /// Visual font size in device-space units, accounting for all transforms.
    pub font_size: f32,
    /// `true` when this glyph is the first inside a new block-level
    /// PDF structure element (heading, paragraph, list item, etc.).
    /// Derived from marked-content tags in the PDF content stream.
    pub is_block_start: bool,
}

#[derive(Clone)]
pub struct PageTextLayout {
    /// Glyphs sorted in reading order (top-to-bottom, left-to-right).
    pub glyphs: Vec<TextGlyph>,
}

impl PageTextLayout {
    pub fn full_text(&self) -> String {
        self.merge_glyphs(0, self.glyphs.len())
    }

    pub fn selected_text(&self, start_index: usize, end_index: usize) -> String {
        let (start, end) = if start_index > end_index {
            (end_index, start_index)
        } else {
            (start_index, end_index)
        };
        let clamped_start = start.min(self.glyphs.len());
        let clamped_end = end.min(self.glyphs.len());
        self.merge_glyphs(clamped_start, clamped_end)
    }

    pub fn glyph_index_at_point(&self, x: f32, y: f32) -> Option<usize> {
        if self.glyphs.is_empty() {
            return None;
        }

        let mut lines: Vec<(f32, f32, Vec<usize>)> = Vec::new();
        for (index, glyph) in self.glyphs.iter().enumerate() {
            let threshold = glyph.font_size * SAME_LINE_THRESHOLD;
            let found = lines
                .iter_mut()
                .find(|(line_y, _, _)| (glyph.y - *line_y).abs() < threshold);
            if let Some((running_y, max_font_size, indices)) = found {
                *max_font_size = max_font_size.max(glyph.font_size);
                let count = indices.len() as f32;
                *running_y = (*running_y * count + glyph.y) / (count + 1.0);
                indices.push(index);
            } else {
                lines.push((glyph.y, glyph.font_size, vec![index]));
            }
        }

        let mut closest_line_index = 0;
        let mut closest_distance = f32::MAX;
        for (line_index, (average_y, _, _)) in lines.iter().enumerate() {
            let distance = (*average_y - y).abs();
            if distance < closest_distance {
                closest_distance = distance;
                closest_line_index = line_index;
            }
        }

        let (_, _, ref line_indices) = lines[closest_line_index];

        let mut nearest_index = line_indices[0];
        let mut nearest_distance = f32::MAX;
        for &glyph_index in line_indices {
            let glyph = &self.glyphs[glyph_index];
            if x >= glyph.x && x <= glyph.x + glyph.width {
                return Some(glyph_index);
            }
            let center_x = glyph.x + glyph.width / 2.0;
            let distance = (center_x - x).abs();
            if distance < nearest_distance {
                nearest_distance = distance;
                nearest_index = glyph_index;
            }
        }

        Some(nearest_index)
    }

    fn merge_glyphs(&self, start: usize, end: usize) -> String {
        if start >= end || start >= self.glyphs.len() {
            return String::new();
        }
        let end = end.min(self.glyphs.len());
        let slice = &self.glyphs[start..end];
        let mut result = String::new();
        let has_structure_tags = slice.iter().any(|g| g.is_block_start);

        for (index, glyph) in slice.iter().enumerate() {
            if index > 0 {
                let previous = &slice[index - 1];
                let size = previous.font_size.max(glyph.font_size).max(1.0);
                let same_line =
                    (glyph.y - previous.y).abs() / size < SAME_LINE_THRESHOLD;

                if same_line {
                    // Same line — insert a synthetic space for large
                    // horizontal gaps (PDFs without explicit space glyphs).
                    let expected_x = previous.x + previous.width;
                    let spacing = (glyph.x - expected_x) / size;
                    if spacing > SPACE_DIST && !result.ends_with(' ') {
                        result.push(' ');
                    }
                } else if has_structure_tags && glyph.is_block_start {
                    // New block-level element (heading, paragraph, list
                    // item) — insert a paragraph break.
                    result.push('\n');
                } else {
                    // Different line, no structural signal — soft line
                    // wrap within a paragraph; join with a space.
                    if !result.ends_with(' ') {
                        result.push(' ');
                    }
                }
            }

            // Append the glyph text, collapsing runs of whitespace.
            for ch in glyph.character.chars() {
                if ch.is_whitespace() {
                    if !result.ends_with(' ') && !result.ends_with('\n') {
                        result.push(' ');
                    }
                } else {
                    result.push(ch);
                }
            }
        }

        result.trim_end().to_string()
    }
}

pub fn extract_page_text(pdf: &Pdf, page_index: usize) -> Result<PageTextLayout> {
    let pages = pdf.pages();
    let page = pages
        .get(page_index)
        .ok_or_else(|| anyhow!("Page index {page_index} out of range"))?;

    let settings = InterpreterSettings::default();
    let spans = extract_text(page, &settings);

    let mut glyphs: Vec<TextGlyph> = spans
        .iter()
        .filter(|span| !span.is_artifact)
        .flat_map(|span| {
            let mut first = true;
            span.glyphs.iter().map(move |glyph_position| {
                let is_block_start = first && span.is_block_start;
                first = false;
                TextGlyph {
                    character: glyph_position.text.clone(),
                    x: glyph_position.x as f32,
                    y: glyph_position.y as f32,
                    width: (glyph_position.advance_x as f32).max(0.0),
                    font_size: span.font_size_device,
                    is_block_start,
                }
            })
        })
        .collect();

    glyphs.sort_by(|a, b| {
        a.y.partial_cmp(&b.y)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal))
    });

    Ok(PageTextLayout { glyphs })
}

pub fn render_single_page(
    pdf: &Pdf,
    page_index: usize,
    scale_factor: f32,
    bg_color: AlphaColor<Srgb>,
) -> Result<RenderedPage> {
    let pages = pdf.pages();
    let all_pages: Vec<_> = pages.iter().collect();
    let page = all_pages
        .get(page_index)
        .ok_or_else(|| anyhow!("Page index {page_index} out of range"))?;

    let interpreter_settings = InterpreterSettings::default();
    let render_settings = RenderSettings {
        x_scale: scale_factor,
        y_scale: scale_factor,
        bg_color,
        ..Default::default()
    };

    let pixmap = hayro::render(page, &interpreter_settings, &render_settings);
    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;
    let image = pixmap_to_render_image(&pixmap)?;

    Ok(RenderedPage {
        image,
        page_width: width,
        page_height: height,
    })
}



#[cfg(all(test, feature = "test-bench"))]
#[path = "pdf_renderer_bench.rs"]
mod pdf_renderer_bench;

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid PDF with the given text content.
    /// Each entry in `lines` is a `(x, y, text)` tuple in PDF coordinates
    /// (origin at bottom-left, Y increases upward).
    fn build_pdf(lines: &[(f32, f32, &str)]) -> Vec<u8> {
        let mut stream = String::new();
        stream.push_str("BT\n/F1 12 Tf\n");
        for (x, y, text) in lines {
            // Move to absolute position via Td from origin each time.
            // We reset by ending and restarting the text object.
            stream.push_str(&format!("{x} {y} Td\n({text}) Tj\n"));
            // Return to origin so next Td is absolute.
            stream.push_str(&format!("{} {} Td\n", -x, -y));
        }
        stream.push_str("ET\n");

        let stream_bytes = stream.as_bytes();
        let stream_length = stream_bytes.len();

        // Hand-craft a minimal PDF. Object offsets don't need to be
        // precise — hayro's parser is tolerant of linearisation gaps.
        let mut pdf = Vec::new();
        let header = b"%PDF-1.4\n";
        pdf.extend_from_slice(header);

        let obj1_offset = pdf.len();
        pdf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n\n");

        let obj2_offset = pdf.len();
        pdf.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n\n");

        let obj3_offset = pdf.len();
        pdf.extend_from_slice(
            b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792]\n   \
              /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n\n",
        );

        let obj4_offset = pdf.len();
        pdf.extend_from_slice(format!("4 0 obj\n<< /Length {stream_length} >>\nstream\n").as_bytes());
        pdf.extend_from_slice(stream_bytes);
        pdf.extend_from_slice(b"\nendstream\nendobj\n\n");

        let obj5_offset = pdf.len();
        pdf.extend_from_slice(
            b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n\n",
        );

        let xref_offset = pdf.len();
        pdf.extend_from_slice(b"xref\n0 6\n");
        pdf.extend_from_slice(format!("{:010} 65535 f \n", 0).as_bytes());
        for offset in [obj1_offset, obj2_offset, obj3_offset, obj4_offset, obj5_offset] {
            pdf.extend_from_slice(format!("{:010} 00000 n \n", offset).as_bytes());
        }

        pdf.extend_from_slice(b"\ntrailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n");
        pdf.extend_from_slice(format!("{xref_offset}\n").as_bytes());
        pdf.extend_from_slice(b"%%EOF\n");

        pdf
    }

    #[test]
    fn text_extraction_single_line() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "Hello World")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        let text = layout.full_text();
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn text_extraction_multiple_lines() {
        let pdf_bytes = build_pdf(&[
            (72.0, 720.0, "First line"),
            (72.0, 706.0, "Second line"),
        ]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        let text = layout.full_text();
        // Lines at normal leading (14pt gap at 12pt font) without structure
        // tags are joined with a space.
        assert_eq!(text, "First line Second line");
    }

    #[test]
    fn text_extraction_preserves_spaces() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "one two  three")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        let text = layout.full_text();
        assert!(
            text.contains("one") && text.contains("two") && text.contains("three"),
            "Expected words 'one', 'two', 'three' in extracted text, got: {text:?}"
        );
    }

    #[test]
    fn text_extraction_font_size_nonzero() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "Test")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        for glyph in &layout.glyphs {
            assert!(
                glyph.font_size > 1.0,
                "Glyph '{}' has font_size {}, expected > 1.0 (UNITS_PER_EM correction missing?)",
                glyph.character,
                glyph.font_size,
            );
        }
    }

    #[test]
    fn text_extraction_glyph_widths_nonzero() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "Hello")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        for glyph in &layout.glyphs {
            if glyph.character != " " {
                assert!(
                    glyph.width > 0.1,
                    "Glyph '{}' has width {}, expected > 0.1",
                    glyph.character,
                    glyph.width,
                );
            }
        }
    }

    #[test]
    fn selected_text_subset() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "ABCDEF")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        assert_eq!(layout.glyphs.len(), 6);
        // Select glyphs 1..4 → "BCD"
        let selected = layout.selected_text(1, 4);
        assert_eq!(selected, "BCD");
    }

    #[test]
    fn selected_text_reversed_range() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "ABCDEF")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        // Reversed selection (drag upward) should produce the same text.
        let forward = layout.selected_text(1, 4);
        let reversed = layout.selected_text(4, 1);
        assert_eq!(forward, reversed);
    }

    #[test]
    fn untagged_pdf_joins_lines() {
        // Without structure tags, different lines are joined with spaces
        // regardless of gap size (no heuristic paragraph detection).
        let pdf_bytes = build_pdf(&[
            (72.0, 720.0, "Heading"),
            (72.0, 690.0, "Body text"),
        ]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        let text = layout.full_text();
        assert_eq!(text, "Heading Body text");
    }

    #[test]
    fn glyph_hit_test_finds_correct_glyph() {
        let pdf_bytes = build_pdf(&[(72.0, 720.0, "ABC")]);
        let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
        let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

        assert_eq!(layout.glyphs.len(), 3);

        // Hit the center of glyph 'B' (index 1).
        let b_glyph = &layout.glyphs[1];
        let hit_x = b_glyph.x + b_glyph.width / 2.0;
        let hit_y = b_glyph.y;
        let index = layout.glyph_index_at_point(hit_x, hit_y);
        assert_eq!(index, Some(1), "Expected to hit glyph 'B' at index 1");
    }


}