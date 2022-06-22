use std::sync::Arc;

use alacritty_terminal::{
    config::{Config, Program, PtyConfig},
    event::{Event, EventListener, Notify},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Indexed,
    index::Point,
    sync::FairMutex,
    term::{cell::Cell, SizeInfo},
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
    fonts::{with_font_cache, TextStyle},
    geometry::{rect::RectF, vector::vec2f},
    impl_internal_actions,
    json::json,
    text_layout::Line,
    Entity,
    Event::KeyDown,
    MutableAppContext, Quad, View, ViewContext,
};
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
const DEFAULT_TITLE: &str = "Terminal";

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct Input(String);

actions!(
    terminal,
    [
        Deploy,
        SIGINT,
        ESCAPE,
        Quit,
        DEL,
        RETURN,
        LEFT,
        RIGHT,
        HistoryBack,
        HistoryForward,
        AutoComplete
    ]
);
impl_internal_actions!(terminal, [Input]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Terminal::deploy);
    cx.add_action(Terminal::write_to_pty);
    cx.add_action(Terminal::send_sigint); //TODO figure out how to do this properly
    cx.add_action(Terminal::escape);
    cx.add_action(Terminal::quit);
    cx.add_action(Terminal::del);
    cx.add_action(Terminal::carriage_return);
    cx.add_action(Terminal::left);
    cx.add_action(Terminal::right);
    cx.add_action(Terminal::history_back);
    cx.add_action(Terminal::history_forward);
    cx.add_action(Terminal::autocomplete);
}

#[derive(Clone)]
pub struct ZedListener(UnboundedSender<Event>);

impl EventListener for ZedListener {
    fn send_event(&self, event: Event) {
        self.0.unbounded_send(event).ok();
    }
}

struct Terminal {
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    title: String,
}

impl Entity for Terminal {
    type Event = ();
}

impl Terminal {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        //Spawn a task so the Alacritty EventLoop to communicate with us
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
        }
    }

    fn deploy(workspace: &mut Workspace, _: &Deploy, cx: &mut ViewContext<Workspace>) {
        workspace.add_item(Box::new(cx.add_view(|cx| Terminal::new(cx))), cx);
    }

    fn process_terminal_event(
        &mut self,
        event: alacritty_terminal::event::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            Event::Wakeup => cx.notify(),
            Event::PtyWrite(out) => self.write_to_pty(&Input(out), cx),
            Event::MouseCursorDirty => todo!(), //I think this is outside of Zed's loop
            Event::Title(title) => self.title = title,
            Event::ResetTitle => self.title = DEFAULT_TITLE.to_string(),
            Event::ClipboardStore(_, _) => todo!(),
            Event::ClipboardLoad(_, _) => todo!(),
            Event::ColorRequest(_, _) => todo!(),
            Event::CursorBlinkingChange => todo!(),
            Event::Bell => todo!(),
            Event::Exit => todo!(),
            Event::MouseCursorDirty => todo!(),
        }
        //
    }

    fn shutdown_pty(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }

    fn history_back(&mut self, _: &HistoryBack, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(UP_SEQ.to_string()), cx);

        //Noop.. for now...
        //This might just need to be forwarded to the terminal?
        //Behavior changes based on mode...
    }

    fn history_forward(&mut self, _: &HistoryForward, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(DOWN_SEQ.to_string()), cx);
        //Noop.. for now...
        //This might just need to be forwarded to the terminal by the pty?
        //Behvaior changes based on mode
    }

    fn autocomplete(&mut self, _: &AutoComplete, cx: &mut ViewContext<Self>) {
        self.write_to_pty(&Input(TAB_CHAR.to_string()), cx);
        //Noop.. for now...
        //This might just need to be forwarded to the terminal by the pty?
        //Behvaior changes based on mode
    }

    fn write_to_pty(&mut self, input: &Input, _cx: &mut ViewContext<Self>) {
        self.pty_tx.notify(input.0.clone().into_bytes());
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

    fn quit(&mut self, _: &Quit, _cx: &mut ViewContext<Self>) {
        //TODO
        // cx.dispatch_action(cx.window_id(), workspace::CloseItem());
    }

    // ShowHistory,
    // AutoComplete
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

        // //Dual owned system from Neovide
        // let mut block_width = cursor_row_layout.x_for_index(cursor_column + 1) - cursor_character_x;
        // if block_width == 0.0 {
        //     block_width = layout.em_width;
        // }
        let cursor = RectF::new(
            vec2f(
                content.cursor.point.column.0 as f32 * em_width,
                content.cursor.point.line.0 as f32 * line_height,
            ),
            vec2f(em_width, line_height),
        );

        // let cursor = Cursor {
        //     color: selection_style.cursor,
        //     block_width,
        //     origin: content_origin + vec2f(x, y),
        //     line_height: layout.line_height,
        //     shape: self.cursor_shape,
        //     block_text,
        // }

        let mut lines = vec![];
        let mut cur_line = vec![];
        let mut last_line = 0;
        for cell in content.display_iter {
            let Indexed {
                point: Point { line, .. },
                cell: Cell { c, .. },
            } = cell;

            if line != last_line {
                lines.push(cur_line);
                cur_line = vec![];
                last_line = line.0;
            }
            cur_line.push(c);
        }
        let line = lines
            .into_iter()
            .map(|char_vec| char_vec.into_iter().collect::<String>())
            .fold("".to_string(), |grid, line| grid + &line + "\n");

        let chunks = vec![(&line[..], None)].into_iter();

        let shaped_lines = layout_highlighted_chunks(
            chunks,
            &text_style,
            cx.text_layout_cache,
            &cx.font_cache,
            usize::MAX,
            line.matches('\n').count() + 1,
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

impl Item for Terminal {
    fn tab_content(&self, style: &theme::Tab, cx: &gpui::AppContext) -> ElementBox {
        let settings = cx.global::<Settings>();
        let search_theme = &settings.theme.search;
        Flex::row()
            .with_child(
                Label::new(self.title.clone(), style.label.clone())
                    .aligned()
                    .contained()
                    .with_margin_left(search_theme.tab_icon_spacing)
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
}
