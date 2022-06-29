use alacritty_terminal::{
    ansi::Color as AnsiColor,
    event_loop::Msg,
    grid::{Indexed, Scroll},
    index::Point,
    sync::FairMutex,
    term::{
        cell::{Cell, Flags},
        SizeInfo,
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
    Event, Quad,
};
use mio_extras::channel::Sender;
use ordered_float::OrderedFloat;
use settings::Settings;
use std::sync::Arc;
use theme::TerminalStyle;

use crate::{Input, ZedListener};

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
        let editor_theme = &settings.theme.editor;
        let terminal_theme = &settings.theme.terminal;
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
            color: editor_theme.text_color,
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

            let new_highlight = make_style_from_cell(fg, flags, &terminal_theme);

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

        let cursor_line = content.cursor.point.line.0 + content.display_offset as i32;
        let mut cursor = None;
        if let Some(layout_line) = cursor_line
            .try_into()
            .ok()
            .and_then(|cursor_line: usize| shaped_lines.get(cursor_line))
        {
            let cursor_x = layout_line.x_for_index(content.cursor.point.column.0);
            cursor = Some(RectF::new(
                vec2f(cursor_x, cursor_line as f32 * line_height),
                vec2f(em_width, line_height),
            ));
        }

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
        cx.scene.push_layer(Some(visible_bounds));
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
                background: Some(Color::white()),
                border: Default::default(),
                corner_radius: 0.,
            });
        }

        cx.scene.pop_layer();
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

fn make_style_from_cell(fg: &AnsiColor, flags: &Flags, style: &TerminalStyle) -> HighlightStyle {
    let fg = Some(alac_color_to_gpui_color(fg, style));
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

fn alac_color_to_gpui_color(allac_color: &AnsiColor, style: &TerminalStyle) -> Color {
    match allac_color {
        alacritty_terminal::ansi::Color::Named(n) => match n {
            alacritty_terminal::ansi::NamedColor::Black => style.black,
            alacritty_terminal::ansi::NamedColor::Red => style.red,
            alacritty_terminal::ansi::NamedColor::Green => style.green,
            alacritty_terminal::ansi::NamedColor::Yellow => style.yellow,
            alacritty_terminal::ansi::NamedColor::Blue => style.blue,
            alacritty_terminal::ansi::NamedColor::Magenta => style.magenta,
            alacritty_terminal::ansi::NamedColor::Cyan => style.cyan,
            alacritty_terminal::ansi::NamedColor::White => style.white,
            alacritty_terminal::ansi::NamedColor::BrightBlack => style.bright_black,
            alacritty_terminal::ansi::NamedColor::BrightRed => style.bright_red,
            alacritty_terminal::ansi::NamedColor::BrightGreen => style.bright_green,
            alacritty_terminal::ansi::NamedColor::BrightYellow => style.bright_yellow,
            alacritty_terminal::ansi::NamedColor::BrightBlue => style.bright_blue,
            alacritty_terminal::ansi::NamedColor::BrightMagenta => style.bright_magenta,
            alacritty_terminal::ansi::NamedColor::BrightCyan => style.bright_cyan,
            alacritty_terminal::ansi::NamedColor::BrightWhite => style.bright_white,
            alacritty_terminal::ansi::NamedColor::Foreground => style.foreground,
            alacritty_terminal::ansi::NamedColor::Background => style.background,
            alacritty_terminal::ansi::NamedColor::Cursor => style.cursor,
            alacritty_terminal::ansi::NamedColor::DimBlack => style.dim_black,
            alacritty_terminal::ansi::NamedColor::DimRed => style.dim_red,
            alacritty_terminal::ansi::NamedColor::DimGreen => style.dim_green,
            alacritty_terminal::ansi::NamedColor::DimYellow => style.dim_yellow,
            alacritty_terminal::ansi::NamedColor::DimBlue => style.dim_blue,
            alacritty_terminal::ansi::NamedColor::DimMagenta => style.dim_magenta,
            alacritty_terminal::ansi::NamedColor::DimCyan => style.dim_cyan,
            alacritty_terminal::ansi::NamedColor::DimWhite => style.dim_white,
            alacritty_terminal::ansi::NamedColor::BrightForeground => style.bright_foreground,
            alacritty_terminal::ansi::NamedColor::DimForeground => style.dim_foreground,
        }, //Theme defined
        alacritty_terminal::ansi::Color::Spec(rgb) => Color::new(rgb.r, rgb.g, rgb.b, 1),
        alacritty_terminal::ansi::Color::Indexed(_) => Color::white(), //Color cube weirdness
    }
}
