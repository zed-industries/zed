//! A developer overlay that paints frame-time statistics directly into the
//! scene, bypassing layout, text, and view invalidation entirely (to avoid
//! infinitely triggering new frames).

use crate::{
    BorderStyle, Bounds, ContentMask, Corners, Edges, Hsla, Pixels, Quad, ScaledPixels, Scene,
    Size, point, rgba, size, transparent_black,
};
use std::{collections::VecDeque, time::Duration};

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DebugFrameOverlayMode {
    #[default]
    Hidden,
    Minimal,
    Full,
}

impl DebugFrameOverlayMode {
    /// Returns the next mode in the Hidden → FrameTime → Detailed cycle.
    pub fn next(self) -> Self {
        match self {
            Self::Hidden => Self::Minimal,
            Self::Minimal => Self::Full,
            Self::Full => Self::Hidden,
        }
    }
}

/// The number of most recent draw durations retained for percentile statistics.
const MAX_SAMPLES: usize = 1000;

const GLYPH_WIDTH: usize = 5;
const GLYPH_HEIGHT: usize = 7;
/// Glyph advance and line advance, in font cells.
const CHAR_ADVANCE: f32 = (GLYPH_WIDTH + 1) as f32;
const LINE_ADVANCE: f32 = (GLYPH_HEIGHT + 2) as f32;
/// Padding between the panel edge and the text, in font cells.
const PANEL_PADDING: f32 = 2.0;
/// Margin between the panel and the window corner, in font cells.
const PANEL_MARGIN: f32 = 4.0;
/// Side of one square font cell, in logical pixels.
const CELL_SIZE: f32 = 2.0;

fn text_color() -> Hsla {
    rgba(0x33ff33ff).into()
}

fn panel_color() -> Hsla {
    rgba(0x000000aa).into()
}

pub(crate) struct DebugFrameOverlay {
    mode: DebugFrameOverlayMode,
    draw_durations: VecDeque<Duration>,
    total_frame_count: u64,
}

impl DebugFrameOverlay {
    pub(crate) fn new() -> Self {
        Self {
            mode: DebugFrameOverlayMode::default(),
            draw_durations: VecDeque::new(),
            total_frame_count: 0,
        }
    }

    pub(crate) fn mode(&self) -> DebugFrameOverlayMode {
        self.mode
    }

    pub(crate) fn set_mode(&mut self, mode: DebugFrameOverlayMode) {
        self.mode = mode;
    }

    /// Clears the draw-duration samples, restarting the percentile window.
    /// The total frame count is left untouched.
    pub(crate) fn reset_stats(&mut self) {
        self.draw_durations.clear();
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.mode != DebugFrameOverlayMode::Hidden
    }

    pub(crate) fn record_frame(&mut self, draw_duration: Duration) {
        self.total_frame_count += 1;
        if self.draw_durations.len() >= MAX_SAMPLES {
            self.draw_durations.pop_front();
        }
        self.draw_durations.push_back(draw_duration);
    }

    pub(crate) fn paint(&self, scene: &mut Scene, viewport_size: Size<Pixels>, scale_factor: f32) {
        if !self.is_enabled() {
            return;
        }

        let lines = self.lines();
        let max_line_chars = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        // Ensure at least one physical pixel per cell so the text stays legible
        // at fractional downscale factors.
        let cell = (CELL_SIZE * scale_factor).max(1.0);

        let panel_width = cell * (max_line_chars as f32 * CHAR_ADVANCE + 2.0 * PANEL_PADDING);
        let panel_height = cell * (lines.len() as f32 * LINE_ADVANCE + 2.0 * PANEL_PADDING);
        let viewport = viewport_size.scale(scale_factor);
        let panel_left = viewport.width.0 - panel_width - cell * PANEL_MARGIN;
        let panel_top = cell * PANEL_MARGIN;

        let content_mask = ContentMask {
            bounds: Bounds {
                origin: point(ScaledPixels(0.), ScaledPixels(0.)),
                size: viewport,
            },
        };

        scene.insert_primitive(solid_quad(
            scaled_bounds(panel_left, panel_top, panel_width, panel_height),
            &content_mask,
            panel_color(),
        ));

        let text_color = text_color();
        for (line_index, line) in lines.iter().enumerate() {
            let line_top = panel_top + cell * (PANEL_PADDING + line_index as f32 * LINE_ADVANCE);
            for (char_index, character) in line.chars().enumerate() {
                let Some(rows) = glyph(character) else {
                    continue;
                };
                let glyph_left =
                    panel_left + cell * (PANEL_PADDING + char_index as f32 * CHAR_ADVANCE);
                for (row_index, row) in rows.iter().enumerate() {
                    let row_top = line_top + cell * row_index as f32;
                    // Merge horizontal runs of lit cells into single quads.
                    let mut column = 0;
                    while column < GLYPH_WIDTH {
                        if row & (1 << (GLYPH_WIDTH - 1 - column)) == 0 {
                            column += 1;
                            continue;
                        }
                        let run_start = column;
                        while column < GLYPH_WIDTH && row & (1 << (GLYPH_WIDTH - 1 - column)) != 0 {
                            column += 1;
                        }
                        scene.insert_primitive(solid_quad(
                            scaled_bounds(
                                glyph_left + cell * run_start as f32,
                                row_top,
                                cell * (column - run_start) as f32,
                                cell,
                            ),
                            &content_mask,
                            text_color,
                        ));
                    }
                }
            }
        }
    }

    fn lines(&self) -> Vec<String> {
        let current = self.draw_durations.back().copied();
        match self.mode {
            DebugFrameOverlayMode::Hidden => Vec::new(),
            DebugFrameOverlayMode::Minimal => vec![format_ms(current)],
            DebugFrameOverlayMode::Full => {
                let mut sorted: Vec<Duration> = self.draw_durations.iter().copied().collect();
                sorted.sort_unstable();
                let percentile = |numerator: usize| {
                    (!sorted.is_empty()).then(|| sorted[(sorted.len() - 1) * numerator / 100])
                };
                // Past five digits the count would break the column
                // alignment, so it saturates instead.
                let frame_count = if self.total_frame_count > 99_999 {
                    "LOTS".to_string()
                } else {
                    self.total_frame_count.to_string()
                };
                // Labels are padded to a uniform width so the fixed-width
                // durations start in the same column on every line.
                vec![
                    format!("CUR {}", format_ms(current)),
                    format!("1%  {}", format_ms(percentile(99))),
                    format!("10% {}", format_ms(percentile(90))),
                    format!("MAX {}", format_ms(sorted.last().copied())),
                    format!("FRAMES {frame_count:>5}"),
                ]
            }
        }
    }
}

/// Formats as `abc.d MS`, right-aligned in room for three integer digits and
/// one decimal (padded with spaces, not zeroes), so stacked readouts align.
fn format_ms(duration: Option<Duration>) -> String {
    match duration {
        Some(duration) => {
            let ms = duration.as_secs_f32() * 1000.0;
            format!("{ms:>5.1} MS")
        }
        None => "   -- MS".into(),
    }
}

fn scaled_bounds(left: f32, top: f32, width: f32, height: f32) -> Bounds<ScaledPixels> {
    Bounds {
        origin: point(ScaledPixels(left), ScaledPixels(top)),
        size: size(ScaledPixels(width), ScaledPixels(height)),
    }
}

fn solid_quad(
    bounds: Bounds<ScaledPixels>,
    content_mask: &ContentMask<ScaledPixels>,
    color: Hsla,
) -> Quad {
    Quad {
        order: 0,
        border_style: BorderStyle::Solid,
        bounds,
        content_mask: *content_mask,
        background: color.into(),
        border_color: transparent_black(),
        corner_radii: Corners::default(),
        border_widths: Edges::default(),
    }
}

/// Returns the 5x7 bitmap for the given character, one `u8` of column bits per
/// row with the most significant of the 5 bits leftmost. Only the characters
/// used by the overlay's readouts are defined.
fn glyph(character: char) -> Option<[u8; GLYPH_HEIGHT]> {
    Some(match character {
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '%' => [
            0b11001, 0b11001, 0b00010, 0b00100, 0b01000, 0b10011, 0b10011,
        ],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every character the readouts can produce must have a glyph, or it
    /// would silently render as blank space.
    #[test]
    fn all_rendered_characters_have_glyphs() {
        let mut overlay = DebugFrameOverlay::new();
        overlay.set_mode(DebugFrameOverlayMode::Full);

        let mut lines = Vec::new();
        for duration in [
            Duration::ZERO,
            Duration::from_micros(1),
            Duration::from_micros(8_333),
            Duration::from_millis(123),
            Duration::from_secs(2),
        ] {
            overlay.record_frame(duration);
            lines.extend(overlay.lines());
        }
        // Counts beyond five digits render as "LOTS".
        overlay.total_frame_count = 100_000;
        lines.extend(overlay.lines());

        // An enabled overlay with no samples yet renders placeholders.
        let mut empty = DebugFrameOverlay::new();
        empty.set_mode(DebugFrameOverlayMode::Full);
        lines.extend(empty.lines());

        for line in lines {
            for character in line.chars() {
                assert!(
                    character == ' ' || glyph(character).is_some(),
                    "no glyph for {character:?} in line {line:?}"
                );
            }
        }
    }

    #[test]
    fn percentile_lows_are_reported_as_times() {
        let mut overlay = DebugFrameOverlay::new();
        overlay.set_mode(DebugFrameOverlayMode::Full);
        for milliseconds in 1..=100 {
            overlay.record_frame(Duration::from_millis(milliseconds));
        }
        let lines = overlay.lines();
        assert_eq!(lines[0], "CUR 100.0 MS");
        assert_eq!(lines[1], "1%   99.0 MS");
        assert_eq!(lines[2], "10%  90.0 MS");
        assert_eq!(lines[3], "MAX 100.0 MS");
        assert_eq!(lines[4], "FRAMES   100");
    }

    #[test]
    fn reset_clears_durations_but_keeps_frame_count() {
        let mut overlay = DebugFrameOverlay::new();
        overlay.set_mode(DebugFrameOverlayMode::Full);
        for _ in 0..10 {
            overlay.record_frame(Duration::from_millis(10));
        }
        overlay.reset_stats();
        let lines = overlay.lines();
        assert_eq!(lines[0], "CUR    -- MS");
        assert_eq!(lines[3], "MAX    -- MS");
        assert_eq!(lines[4], "FRAMES    10");
        overlay.record_frame(Duration::from_millis(20));
        let lines = overlay.lines();
        assert_eq!(lines[0], "CUR  20.0 MS");
        assert_eq!(lines[4], "FRAMES    11");
    }

    #[test]
    fn frame_count_accumulates_across_mode_changes() {
        let mut overlay = DebugFrameOverlay::new();
        for _ in 0..3 {
            overlay.record_frame(Duration::from_millis(10));
        }
        overlay.set_mode(DebugFrameOverlayMode::Minimal);
        assert_eq!(overlay.lines(), vec![" 10.0 MS".to_string()]);
        overlay.set_mode(DebugFrameOverlayMode::Full);
        overlay.record_frame(Duration::from_millis(10));
        assert_eq!(overlay.lines()[4], "FRAMES     4");
        overlay.set_mode(DebugFrameOverlayMode::Hidden);
        overlay.record_frame(Duration::from_millis(10));
        overlay.set_mode(DebugFrameOverlayMode::Full);
        assert_eq!(overlay.lines()[4], "FRAMES     5");
    }

    #[test]
    fn frame_count_is_right_aligned_and_saturates() {
        let mut overlay = DebugFrameOverlay::new();
        overlay.set_mode(DebugFrameOverlayMode::Full);
        overlay.record_frame(Duration::from_millis(10));
        assert_eq!(overlay.lines()[4], "FRAMES     1");
        overlay.total_frame_count = 99_999;
        assert_eq!(overlay.lines()[4], "FRAMES 99999");
        overlay.total_frame_count = 100_000;
        assert_eq!(overlay.lines()[4], "FRAMES  LOTS");
    }

    #[test]
    fn toggling_on_shows_previous_frame_immediately() {
        let mut overlay = DebugFrameOverlay::new();
        overlay.record_frame(Duration::from_millis(10));
        overlay.set_mode(DebugFrameOverlayMode::Minimal);
        assert_eq!(overlay.lines(), vec![" 10.0 MS".to_string()]);
    }
}
