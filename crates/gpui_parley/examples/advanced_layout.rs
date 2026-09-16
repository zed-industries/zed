//! Showcases Parley layout features that the shared GPUI `TextSystem` SPI does
//! not expose: text-indent, per-line boxes (excluded regions), and
//! character-count line breaking.
//!
//! These are Parley-native capabilities. GPUI's `TextSystem` trait is a shaping
//! boundary (`layout_line` / `layout_wrapped_line`), so features like these need
//! a Parley-specific surface rather than the shared trait.

use gpui_parley::{FONT_FAMILY, font_context};
use parley::{
    Alignment, AlignmentOptions, FontFamily, IndentOptions, LayoutContext, StyleProperty, YieldData,
};

const TEXT: &str = "The quick brown fox jumps over the lazy dog while the bright sun shines down on the quiet meadow near the river.";

fn build(
    font_context: &mut parley::FontContext,
    layout_context: &mut LayoutContext,
    text: &str,
    size: f32,
) -> parley::Layout<[u8; 4]> {
    let mut builder = layout_context.ranged_builder(font_context, text, 1.0, true);
    builder.push_default(StyleProperty::FontFamily(FontFamily::from(FONT_FAMILY)));
    builder.push_default(StyleProperty::FontSize(size));
    builder.build(text)
}

fn main() {
    let mut font_context = font_context();
    let mut layout_context = LayoutContext::new();

    println!("== 1. text-indent (CSS `text-indent`) ==");
    let mut layout = build(&mut font_context, &mut layout_context, TEXT, 16.0);
    layout.set_text_indent(32.0, IndentOptions::default());
    layout.break_all_lines(Some(240.0));
    layout.align(Alignment::Start, AlignmentOptions::default());
    let first_glyph_x = layout.lines().next().and_then(|line| {
        line.items().find_map(|item| match item {
            parley::PositionedLayoutItem::GlyphRun(run) => {
                run.positioned_glyphs().next().map(|glyph| glyph.x)
            }
            _ => None,
        })
    });
    println!(
        "  {} lines; first line first glyph x = {first_glyph_x:?} (indented by 32)",
        layout.lines().len()
    );

    println!("\n== 2. per-line boxes (narrow second line = excluded region) ==");
    let mut layout = build(&mut font_context, &mut layout_context, TEXT, 16.0);
    let mut breaker = layout.break_lines();
    breaker.state_mut().set_layout_max_advance(f32::MAX);
    let mut line = 0;
    loop {
        let width = if line == 1 { 120.0 } else { 260.0 };
        breaker.state_mut().set_line_max_advance(width);
        match breaker.break_next() {
            Some(YieldData::LineBreak(data)) => {
                println!(
                    "  line {line}: width_cap={width:.0}, advance={:.1}, height={:.1}",
                    data.advance, data.line_height
                );
                line += 1;
            }
            Some(_) => {}
            None => break,
        }
        if line >= 6 {
            break;
        }
    }

    println!("\n== 3. character-count breaking (max 16 chars per line) ==");
    let mut layout = build(&mut font_context, &mut layout_context, TEXT, 16.0);
    let mut breaks = 0;
    {
        let mut breaker = layout.break_lines();
        while breaker.break_next_with_length(16).is_some() {
            breaks += 1;
        }
    }
    layout.align(Alignment::Start, AlignmentOptions::default());
    println!("  produced {breaks} lines");
}
