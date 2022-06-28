use alacritty_terminal::{
    ansi::Color as AnsiColor,
    event_loop::Msg,
    grid::{Indexed, Scroll},
    index::Point,
    sync::FairMutex,
    term::{
        cell::{Cell, Flags},
        RenderableCursor, SizeInfo,
    },
    Term,
};
use gpui::{
    color::Color,
    elements::*,
    fonts::{HighlightStyle, TextStyle, Underline},
    geometry::{rect::RectF, vector::vec2f},
    json::json,
    text_layout::Line,
    Event, PaintContext, Quad,
};
use mio_extras::channel::Sender;
use ordered_float::OrderedFloat;
use settings::Settings;
use std::sync::Arc;

use crate::{Input, ZedListener};

const DEBUG_GRID: bool = false;
const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

pub struct TerminalEl {
    term: Arc<FairMutex<Term<ZedListener>>>,
    pty_tx: Sender<Msg>,
    size: SizeInfo,
}

impl TerminalEl {
    pub fn new(
        term: Arc<FairMutex<Term<ZedListener>>>,
        pty_tx: Sender<Msg>,
        size: SizeInfo,
    ) -> TerminalEl {
        TerminalEl { term, pty_tx, size }
    }
}

pub struct LayoutState {
    lines: Vec<Line>,
    line_height: f32,
    em_width: f32,
    cursor: Option<RectF>,
}

impl Element for TerminalEl {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let size = constraint.max;
        let settings = cx.global::<Settings>();
        let theme = &settings.theme.editor;
        //Get terminal
        let mut term = self.term.lock();

        //Set up text rendering
        let font_cache = cx.font_cache();

        let font_family_id = settings.buffer_font_family;
        let font_family_name = cx.font_cache().family_name(font_family_id).unwrap();
        let font_properties = Default::default();
        let font_id = font_cache
            .select_font(font_family_id, &font_properties)
            .unwrap();
        let font_size = settings.buffer_font_size;

        let text_style = TextStyle {
            color: theme.text_color,
            font_family_id: settings.buffer_font_family,
            font_family_name,
            font_id,
            font_size,
            font_properties: Default::default(),
            underline: Default::default(),
        };

        let line_height = cx.font_cache.line_height(text_style.font_size);
        let em_width = cx
            .font_cache()
            .em_width(text_style.font_id, text_style.font_size)
            + 2.;

        //Resize terminal
        let new_size = SizeInfo::new(size.x(), size.y(), em_width, line_height, 0., 0., false);
        if !new_size.eq(&self.size) {
            self.pty_tx.send(Msg::Resize(new_size)).ok();
            term.resize(new_size);
            self.size = new_size;
        }

        //Start rendering
        let content = term.renderable_content();

        let mut cursor = None;
        let mut lines: Vec<(String, Option<HighlightStyle>)> = vec![];
        let mut last_line = 0;
        let mut line_count = 1;
        let mut cur_chunk = String::new();

        let mut cur_highlight = HighlightStyle {
            color: Some(Color::white()),
            ..Default::default()
        };

        for cell in content.display_iter {
            let Indexed {
                point: Point { line, .. },
                cell: Cell {
                    c, fg, flags, .. // TODO: Add bg and flags
                }, //TODO: Learn what 'CellExtra does'
            } = cell;

            if cell.point == content.cursor.point {
                cursor = make_cursor(em_width, line_height, content.cursor);
            }

            let new_highlight = make_style_from_cell(fg, flags);

            if line != last_line {
                line_count += 1;
                cur_chunk.push('\n');
                last_line = line.0;
            }

            if new_highlight != cur_highlight {
                lines.push((cur_chunk.clone(), Some(cur_highlight.clone())));
                cur_chunk.clear();
                cur_highlight = new_highlight;
            }
            cur_chunk.push(*c)
        }
        lines.push((cur_chunk, Some(cur_highlight)));

        let shaped_lines = layout_highlighted_chunks(
            lines.iter().map(|(text, style)| (text.as_str(), *style)),
            &text_style,
            cx.text_layout_cache,
            &cx.font_cache,
            usize::MAX,
            line_count,
        );

        (
            constraint.max,
            LayoutState {
                lines: shaped_lines,
                line_height,
                em_width,
                cursor,
            },
        )
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let origin = bounds.origin() + vec2f(layout.em_width, 0.);

        let mut line_origin = origin;
        for line in &layout.lines {
            let boundaries = RectF::new(line_origin, vec2f(bounds.width(), layout.line_height));

            if boundaries.intersects(visible_bounds) {
                line.paint(line_origin, visible_bounds, layout.line_height, cx);
            }

            line_origin.set_y(boundaries.max_y());
        }

        if let Some(c) = layout.cursor {
            let new_origin = origin + c.origin();
            let new_cursor = RectF::new(new_origin, c.size());
            cx.scene.push_quad(Quad {
                bounds: new_cursor,
                background: Some(Color::red()),
                border: Default::default(),
                corner_radius: 0.,
            });
        }

        if DEBUG_GRID {
            draw_debug_grid(bounds, layout, cx);
        }
    }

    fn dispatch_event(
        &mut self,
        event: &gpui::Event,
        _bounds: gpui::geometry::rect::RectF,
        _visible_bounds: gpui::geometry::rect::RectF,
        layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        cx: &mut gpui::EventContext,
    ) -> bool {
        match event {
            Event::ScrollWheel { delta, .. } => {
                let vertical_scroll =
                    (delta.y() / layout.line_height) * ALACRITTY_SCROLL_MULTIPLIER;
                let scroll = Scroll::Delta(vertical_scroll.round() as i32);
                self.term.lock().scroll_display(scroll);
                true
            }
            Event::KeyDown {
                input: Some(input), ..
            } => {
                cx.dispatch_action(Input(input.to_string()));
                true
            }
            _ => false,
        }
    }

    fn debug(
        &self,
        _bounds: gpui::geometry::rect::RectF,
        _layout: &Self::LayoutState,
        _paint: &Self::PaintState,
        _cx: &gpui::DebugContext,
    ) -> gpui::serde_json::Value {
        json!({
            "type": "TerminalElement",
        })
    }
}

fn make_style_from_cell(fg: &AnsiColor, flags: &Flags) -> HighlightStyle {
    let fg = Some(alac_color_to_gpui_color(fg));
    let underline = if flags.contains(Flags::UNDERLINE) {
        Some(Underline {
            color: fg,
            squiggly: false,
            thickness: OrderedFloat(1.),
        })
    } else {
        None
    };
    HighlightStyle {
        color: fg,
        underline,
        ..Default::default()
    }
}

fn alac_color_to_gpui_color(allac_color: &AnsiColor) -> Color {
    match allac_color {
        alacritty_terminal::ansi::Color::Named(n) => match n {
            alacritty_terminal::ansi::NamedColor::Black => Color::black(),
            alacritty_terminal::ansi::NamedColor::Red => Color::red(),
            alacritty_terminal::ansi::NamedColor::Green => Color::green(),
            alacritty_terminal::ansi::NamedColor::Yellow => Color::yellow(),
            alacritty_terminal::ansi::NamedColor::Blue => Color::blue(),
            alacritty_terminal::ansi::NamedColor::Magenta => Color::new(188, 63, 188, 1),
            alacritty_terminal::ansi::NamedColor::Cyan => Color::new(17, 168, 205, 1),
            alacritty_terminal::ansi::NamedColor::White => Color::white(),
            alacritty_terminal::ansi::NamedColor::BrightBlack => Color::new(102, 102, 102, 1),
            alacritty_terminal::ansi::NamedColor::BrightRed => Color::new(102, 102, 102, 1),
            alacritty_terminal::ansi::NamedColor::BrightGreen => Color::new(35, 209, 139, 1),
            alacritty_terminal::ansi::NamedColor::BrightYellow => Color::new(245, 245, 67, 1),
            alacritty_terminal::ansi::NamedColor::BrightBlue => Color::new(59, 142, 234, 1),
            alacritty_terminal::ansi::NamedColor::BrightMagenta => Color::new(214, 112, 214, 1),
            alacritty_terminal::ansi::NamedColor::BrightCyan => Color::new(41, 184, 219, 1),
            alacritty_terminal::ansi::NamedColor::BrightWhite => Color::new(229, 229, 229, 1),
            alacritty_terminal::ansi::NamedColor::Foreground => Color::white(),
            alacritty_terminal::ansi::NamedColor::Background => Color::black(),
            alacritty_terminal::ansi::NamedColor::Cursor => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimBlack => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimRed => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimGreen => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimYellow => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimBlue => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimMagenta => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimCyan => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimWhite => Color::white(),
            alacritty_terminal::ansi::NamedColor::BrightForeground => Color::white(),
            alacritty_terminal::ansi::NamedColor::DimForeground => Color::white(),
        }, //Theme defined
        alacritty_terminal::ansi::Color::Spec(rgb) => Color::new(rgb.r, rgb.g, rgb.b, 1),
        alacritty_terminal::ansi::Color::Indexed(_) => Color::white(), //Color cube weirdness
    }
}

fn make_cursor(em_width: f32, line_height: f32, cursor: RenderableCursor) -> Option<RectF> {
    Some(RectF::new(
        vec2f(
            cursor.point.column.0 as f32 * em_width,
            cursor.point.line.0 as f32 * line_height,
        ),
        vec2f(em_width, line_height),
    ))
}

fn draw_debug_grid(bounds: RectF, layout: &mut LayoutState, cx: &mut PaintContext) {
    for col in 0..(bounds.0[2] / layout.em_width) as usize {
        let rect_origin = bounds.origin() + vec2f(col as f32 * layout.em_width, 0.);
        let line = RectF::new(rect_origin, vec2f(1., bounds.0[3]));
        cx.scene.push_quad(Quad {
            bounds: line,
            background: Some(Color::green()),
            border: Default::default(),
            corner_radius: 0.,
        });
    }
    for row in 0..(bounds.0[3] / layout.line_height) as usize {
        let rect_origin = bounds.origin() + vec2f(0., row as f32 * layout.line_height);
        let line = RectF::new(rect_origin, vec2f(bounds.0[2], 1.));
        cx.scene.push_quad(Quad {
            bounds: line,
            background: Some(Color::green()),
            border: Default::default(),
            corner_radius: 0.,
        });
    }
}
