use std::sync::Arc;

use alacritty_terminal::{
    ansi::Color as AnsiColor,
    config::{Config, Program, PtyConfig},
    event::{Event as AlacTermEvent, EventListener, Notify},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Indexed,
    index::Point,
    sync::FairMutex,
    term::{
        cell::{Cell, Flags},
        color::Rgb,
        SizeInfo,
    },
    tty, Term,
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use gpui::{
    actions,
    color::Color,
    elements::*,
    fonts::{with_font_cache, HighlightStyle, TextStyle, Underline},
    geometry::{rect::RectF, vector::vec2f},
    impl_internal_actions,
    json::json,
    platform::CursorStyle,
    text_layout::Line,
    ClipboardItem, Entity,
    Event::KeyDown,
    MutableAppContext, Quad, View, ViewContext,
};
use ordered_float::OrderedFloat;
use project::{Project, ProjectPath};
use settings::Settings;
use smallvec::SmallVec;
use workspace::{Item, Workspace};

//ASCII Control characters on a keyboard
//Consts -> Structs -> Impls -> Functions, Vaguely in order of importance
const ETX_CHAR: char = 3_u8 as char; //'End of text', the control code for 'ctrl-c'
const TAB_CHAR: char = 9_u8 as char;
const CARRIAGE_RETURN_CHAR: char = 13_u8 as char;
const ESC_CHAR: char = 27_u8 as char;
const DEL_CHAR: char = 127_u8 as char;
const LEFT_SEQ: &str = "\x1b[D";
const RIGHT_SEQ: &str = "\x1b[C";
const UP_SEQ: &str = "\x1b[A";
const DOWN_SEQ: &str = "\x1b[B";
const CLEAR_SEQ: &str = "\x1b[2J";
const DEFAULT_TITLE: &str = "Terminal";

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Input(String);

actions!(
    terminal,
    [Deploy, SIGINT, ESCAPE, Quit, DEL, RETURN, LEFT, RIGHT, UP, DOWN, TAB, Clear]
);
impl_internal_actions!(terminal, [Input]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
    cx.add_action(Terminal::write_to_pty);
    cx.add_action(Terminal::send_sigint);
    cx.add_action(Terminal::escape);
    cx.add_action(Terminal::quit);
    cx.add_action(Terminal::del);
    cx.add_action(Terminal::carriage_return); //TODO figure out how to do this properly. Should we be checking the terminal mode?
    cx.add_action(Terminal::left);
    cx.add_action(Terminal::right);
    cx.add_action(Terminal::up);
    cx.add_action(Terminal::down);
    cx.add_action(Terminal::tab);
    cx.add_action(Terminal::clear);
}

#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

///A terminal renderer.
struct Terminal {
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    title: String,
    has_new_content: bool,
    has_bell: bool, //Currently using iTerm bell, show bell emoji in tab until input is received
}

enum ZedTermEvent {
    TitleChanged,
    CloseTerminal,
}

impl Entity for Terminal {
    type Event = ZedTermEvent;
}

impl Terminal {
    ///Create a new Terminal view. This spawns a task, a thread, and opens the TTY devices
    fn new(cx: &mut ViewContext<Self>) -> Self {
        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        let (events_tx, mut events_rx) = unbounded();
        cx.spawn_weak(|this, mut cx| async move {
            while let Some(event) = events_rx.next().await {
                match this.upgrade(&cx) {
                    Some(handle) => {
                        handle.update(&mut cx, |this, cx| {
                            this.process_terminal_event(event, cx);
                            cx.notify();
                        });
                    }
                    None => break,
                }
            }
        })
        .detach();

        //TODO: Load from settings
        let pty_config = PtyConfig {
            shell: Some(Program::Just("zsh".to_string())),
            working_directory: None,
            hold: false,
        };

        //TODO: Properly configure this
        let config = Config {
            pty_config: pty_config.clone(),
            ..Default::default()
        };

        //TODO: derive this
        let size_info = SizeInfo::new(400., 100.0, 5., 5., 0., 0., false);

        //Set up the terminal...
        let term = Term::new(&config, size_info, ZedListener(events_tx.clone()));
        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = tty::new(&pty_config, &size_info, None).expect("Could not create tty");

        //And connect them together
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_config.hold,
            false,
        );

        //Kick things off
        let pty_tx = Notifier(event_loop.channel());
        let _io_thread = event_loop.spawn();
        Terminal {
            title: DEFAULT_TITLE.to_string(),
            term,
            pty_tx,
            has_new_content: false,
            has_bell: false,
        }
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: alacritty_terminal::event::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            AlacTermEvent::Wakeup => {
                if !cx.is_self_focused() {
                    //Need to figure out how to trigger a redraw when not in focus
                    self.has_new_content = true; //Change tab content
                    cx.emit(ZedTermEvent::TitleChanged);
                } else {
                    cx.notify()
                }
            }
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(&Input(out), cx),
            //TODO:
            //What this is supposed to do is check the cursor state, then set it on the platform.
            //See Processor::reset_mouse_cursor() and Processor::cursor_state() in alacritty/src/input.rs
            //to see how this is Calculated. Question: Does this flow make sense with how GPUI hadles
            //the mouse?
            AlacTermEvent::MouseCursorDirty => {
                //Calculate new cursor style.
                //Check on correctly handling mouse events for terminals
                cx.platform().set_cursor_style(CursorStyle::Arrow); //???
                println!("Mouse cursor dirty")
            }
            AlacTermEvent::Title(title) => {
                self.title = title;
                cx.emit(ZedTermEvent::TitleChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.title = DEFAULT_TITLE.to_string();
                cx.emit(ZedTermEvent::TitleChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new(data))
            }
            AlacTermEvent::ClipboardLoad(_, format) => self.write_to_pty(
                &Input(format(
                    &cx.read_from_clipboard()
                        .map(|ci| ci.text().to_string())
                        .unwrap_or("".to_string()),
                )),
                cx,
            ),
            AlacTermEvent::ColorRequest(index, format) => {
                //TODO test this as well
                //TODO: change to getting the display colors, like alacrityy, instead of a default
                let color = self.term.lock().colors()[index].unwrap_or(Rgb::default());
                self.write_to_pty(&Input(format(color)), cx)
            }
            AlacTermEvent::CursorBlinkingChange => {
                //So, it's our job to set a timer and cause the cursor to blink here
                //Which means that I'm going to put this off until someone @ Zed looks at it
            }
            AlacTermEvent::Bell => {
                self.has_bell = true;
                cx.emit(ZedTermEvent::TitleChanged);
            }
            AlacTermEvent::Exit => self.quit(&Quit, cx),
        }
    }

    ///Create a new Terminal
    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        workspace.add_item(Box::new(cx.add_view(|cx| Terminal::new(cx))), cx);
    }

    ///Send the shutdown message to Alacritty
    fn shutdown_pty(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }

    fn quit(&mut self, _: &Quit, cx: &mut ViewContext<Self>) {
        cx.emit(ZedTermEvent::CloseTerminal);
    }

    fn write_to_pty(&mut self, input: &Input, cx: &mut ViewContext<Self>) {
        //iTerm bell behavior, bell stays until terminal is interacted with
        self.has_bell = false;
        cx.emit(ZedTermEvent::TitleChanged);
        self.pty_tx.notify(input.0.clone().into_bytes());
    }

    fn up(&mut self, _: &UP, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(UP_SEQ.to_string()), cx);
    }

    fn down(&mut self, _: &DOWN, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DOWN_SEQ.to_string()), cx);
    }

    fn tab(&mut self, _: &TAB, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(TAB_CHAR.to_string()), cx);
    }

    fn send_sigint(&mut self, _: &SIGINT, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ETX_CHAR.to_string()), cx);
    }

    fn escape(&mut self, _: &ESCAPE, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(ESC_CHAR.to_string()), cx);
    }

    fn del(&mut self, _: &DEL, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DEL_CHAR.to_string()), cx);
    }

    fn carriage_return(&mut self, _: &RETURN, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(CARRIAGE_RETURN_CHAR.to_string()), cx);
    }

    fn left(&mut self, _: &LEFT, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(LEFT_SEQ.to_string()), cx);
    }

    fn right(&mut self, _: &RIGHT, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(RIGHT_SEQ.to_string()), cx);
    }

    fn clear(&mut self, _: &Clear, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(CLEAR_SEQ.to_string()), cx);
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.shutdown_pty();
    }
}

impl View for Terminal {
    fn ui_name() -> &'static str {
        "Terminal"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let _theme = cx.global::<Settings>().theme.clone();

        TerminalEl::new(self.term.clone())
            .contained()
            // .with_style(theme.terminal.container)
            .boxed()
    }

    fn on_focus(&mut self, _: &mut ViewContext<Self>) {
        self.has_new_content = false;
    }
}

impl Item for Terminal {
    fn tab_content(&self, tab_theme: &theme::Tab, cx: &gpui::AppContext) -> ElementBox {
        let settings = cx.global::<Settings>();
        let search_theme = &settings.theme.search; //TODO properly integrate themes

        let mut flex = Flex::row();

        if self.has_bell {
            flex.add_child(
                Svg::new("icons/zap.svg")
                    .with_color(tab_theme.label.text.color)
                    .constrained()
                    .with_width(search_theme.tab_icon_width)
                    .aligned()
                    .boxed(),
            );
        };

        flex.with_child(
            Label::new(self.title.clone(), tab_theme.label.clone())
                .aligned()
                .contained()
                .with_margin_left(if self.has_bell {
                    search_theme.tab_icon_spacing
                } else {
                    0.
                })
                .boxed(),
        )
        .boxed()
    }

    fn project_path(&self, _cx: &gpui::AppContext) -> Option<ProjectPath> {
        None
    }

    fn project_entry_ids(&self, _cx: &gpui::AppContext) -> SmallVec<[project::ProjectEntryId; 3]> {
        SmallVec::new()
    }

    fn is_singleton(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn set_nav_history(&mut self, _: workspace::ItemNavHistory, _: &mut ViewContext<Self>) {}

    fn can_save(&self, _cx: &gpui::AppContext) -> bool {
        false
    }

    fn save(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save should not have been called");
    }

    fn save_as(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _abs_path: std::path::PathBuf,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        unreachable!("save_as should not have been called");
    }

    fn reload(
        &mut self,
        _project: gpui::ModelHandle<Project>,
        _cx: &mut ViewContext<Self>,
    ) -> gpui::Task<gpui::anyhow::Result<()>> {
        gpui::Task::ready(Ok(()))
    }

    fn is_dirty(&self, _: &gpui::AppContext) -> bool {
        self.has_new_content
    }

    fn should_update_tab_on_event(event: &Self::Event) -> bool {
        matches!(event, &ZedTermEvent::TitleChanged)
    }

    fn should_close_item_on_event(event: &Self::Event) -> bool {
        matches!(event, &ZedTermEvent::CloseTerminal)
    }
}

struct TerminalEl {
    term: Arc<FairMutex<Term<ZedListener>>>,
}

impl TerminalEl {
    fn new(term: Arc<FairMutex<Term<ZedListener>>>) -> TerminalEl {
        TerminalEl { term }
    }
}

struct LayoutState {
    lines: Vec<Line>,
    line_height: f32,
    cursor: RectF,
}
/* TODO point calculation for selection
 * take the current point's x:
 * - subtract padding
 * - divide by cell width
 * - take the minimum of the x coord and the last colum of the size info
 * Take the current point's y:
 * - Subtract padding
 * - Divide by cell height
 * - Take the minimum of the y coord and the last line
 *
 * With this x and y, pass to term::viewport_to_point (module function)
 * Also pass in the display offset from the term.grid().display_offset()
 * (Display offset is for scrolling)
 */

/* TODO Selection
 * 1. On click, calculate the single, double, and triple click based on timings
 * 2. Convert mouse location to a terminal point
 * 3. Generate each of the three kinds of selection needed
 * 4. Assign a selection to the terminal's selection variable
 * How to render?
 * 1. On mouse moved, calculate a terminal point
 * 2. if (lmb_pressed || rmb_pressed) && (self.ctx.modifiers().shift()  || !self.ctx.mouse_mode()
 * 3. Take the selection from the terminal, call selection.update(), and put it back
 */

/* TODO Scroll
 * 1. Convert scroll to a pixel delta (alacritty/src/input > Processor::mouse_wheel_input)
 * 2. Divide by cell height
 * 3. Create an alacritty_terminal::Scroll::Delta() object and call `self.terminal.scroll_display(scroll);`
 * 4. Maybe do a cx.notify, just in case.
 * 5. Also update the selected area, just check out for the logic alacritty/src/event.rs > ActionContext::scroll
 */
impl Element for TerminalEl {
    type LayoutState = LayoutState;
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        cx: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        let size = constraint.max;
        //Get terminal content
        let mut term = self.term.lock();

        //Set up text rendering

        let text_style = with_font_cache(cx.font_cache.clone(), || TextStyle {
            color: Color::white(),
            ..Default::default()
        });
        let line_height = cx.font_cache.line_height(text_style.font_size);
        let em_width = cx
            .font_cache()
            .em_width(text_style.font_id, text_style.font_size);

        term.resize(SizeInfo::new(
            size.x(),
            size.y(),
            em_width,
            line_height,
            0.,
            0.,
            false,
        ));

        let content = term.renderable_content();

        let cursor = RectF::new(
            vec2f(
                content.cursor.point.column.0 as f32 * em_width,
                content.cursor.point.line.0 as f32 * line_height,
            ),
            vec2f(em_width, line_height),
        );

        let mut lines: Vec<(String, Option<HighlightStyle>)> = vec![];
        let mut last_line = 0;

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

            let new_highlight = make_style_from_cell(fg, flags);
            HighlightStyle {
                color: Some(alac_color_to_gpui_color(fg)),
                ..Default::default()
            };

            if line != last_line {
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
            last_line as usize,
        );

        (
            constraint.max,
            LayoutState {
                lines: shaped_lines,
                line_height,
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
        let mut origin = bounds.origin();

        for line in &layout.lines {
            let boundaries = RectF::new(origin, vec2f(bounds.width(), layout.line_height));

            if boundaries.intersects(visible_bounds) {
                line.paint(origin, visible_bounds, layout.line_height, cx);
            }

            origin.set_y(boundaries.max_y());
        }

        let new_origin = bounds.origin() + layout.cursor.origin();
        let new_cursor = RectF::new(new_origin, layout.cursor.size());

        cx.scene.push_quad(Quad {
            bounds: new_cursor,
            background: Some(Color::red()),
            border: Default::default(),
            corner_radius: 0.,
        });
    }

    fn dispatch_event(
        &mut self,
        event: &gpui::Event,
        _bounds: gpui::geometry::rect::RectF,
        _visible_bounds: gpui::geometry::rect::RectF,
        _layout: &mut Self::LayoutState,
        _paint: &mut Self::PaintState,
        cx: &mut gpui::EventContext,
    ) -> bool {
        match event {
            KeyDown {
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
