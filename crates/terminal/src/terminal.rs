pub mod connected_el;
pub mod connected_view;
pub mod mappings;
pub mod modal;
pub mod terminal_view;

use alacritty_terminal::{
    ansi::{ClearMode, Handler},
    config::{Config, Program, PtyConfig, Scrolling},
    event::{Event as AlacTermEvent, EventListener, Notify, WindowSize},
    event_loop::{EventLoop, Msg, Notifier},
    grid::{Dimensions, Scroll},
    index::{Direction, Point},
    selection::{Selection, SelectionType},
    sync::FairMutex,
    term::{RenderableContent, TermMode},
    tty::{self, setup_env},
    Term,
};
use anyhow::{bail, Result};

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    FutureExt,
};

use mappings::mouse::{
    alt_scroll, mouse_button_report, mouse_moved_report, mouse_point, mouse_side, scroll_report,
};
use modal::deploy_modal;
use settings::{AlternateScroll, Settings, Shell, TerminalBlink};
use std::{collections::HashMap, fmt::Display, ops::Sub, path::PathBuf, sync::Arc, time::Duration};
use thiserror::Error;

use gpui::{
    geometry::vector::{vec2f, Vector2F},
    keymap::Keystroke,
    ClipboardItem, Entity, ModelContext, MouseButtonEvent, MouseMovedEvent, MutableAppContext,
    ScrollWheelEvent,
};

use crate::mappings::{
    colors::{get_color_at_index, to_alac_rgb},
    keys::to_esc_str,
};

///Initialize and register all of our action handlers
pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(deploy_modal);

    terminal_view::init(cx);
    connected_view::init(cx);
}

///Scrolling is unbearably sluggish by default. Alacritty supports a configurable
///Scroll multiplier that is set to 3 by default. This will be removed when I
///Implement scroll bars.
pub const ALACRITTY_SCROLL_MULTIPLIER: f32 = 3.;

const DEBUG_TERMINAL_WIDTH: f32 = 500.;
const DEBUG_TERMINAL_HEIGHT: f32 = 30.;
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

///Upward flowing events, for changing the title and such
#[derive(Clone, Copy, Debug)]
pub enum Event {
    TitleChanged,
    CloseTerminal,
    Bell,
    Wakeup,
    BlinkChanged,
}

#[derive(Clone, Debug)]
enum InternalEvent {
    TermEvent(AlacTermEvent),
    Resize(TerminalSize),
    Clear,
    Scroll(Scroll),
    SetSelection(Option<Selection>),
    UpdateSelection((Point, Direction)),
    Copy,
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalSize {
    cell_width: f32,
    line_height: f32,
    height: f32,
    width: f32,
}

impl TerminalSize {
    pub fn new(line_height: f32, cell_width: f32, size: Vector2F) -> Self {
        TerminalSize {
            cell_width,
            line_height,
            width: size.x(),
            height: size.y(),
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }
}
impl Default for TerminalSize {
    fn default() -> Self {
        TerminalSize::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            vec2f(DEBUG_TERMINAL_WIDTH, DEBUG_TERMINAL_HEIGHT),
        )
    }
}

impl From<TerminalSize> for WindowSize {
    fn from(val: TerminalSize) -> Self {
        WindowSize {
            num_lines: val.num_lines() as u16,
            num_cols: val.num_columns() as u16,
            cell_width: val.cell_width() as u16,
            cell_height: val.line_height() as u16,
        }
    }
}

impl Dimensions for TerminalSize {
    fn total_lines(&self) -> usize {
        self.screen_lines() //TODO: Check that this is fine. This is supposed to be for the back buffer...
    }

    fn screen_lines(&self) -> usize {
        self.num_lines()
    }

    fn columns(&self) -> usize {
        self.num_columns()
    }
}

#[derive(Error, Debug)]
pub struct TerminalError {
    pub directory: Option<PathBuf>,
    pub shell: Option<Shell>,
    pub source: std::io::Error,
}

impl TerminalError {
    pub fn fmt_directory(&self) -> String {
        self.directory
            .clone()
            .map(|path| {
                match path
                    .into_os_string()
                    .into_string()
                    .map_err(|os_str| format!("<non-utf8 path> {}", os_str.to_string_lossy()))
                {
                    Ok(s) => s,
                    Err(s) => s,
                }
            })
            .unwrap_or_else(|| {
                let default_dir =
                    dirs::home_dir().map(|buf| buf.into_os_string().to_string_lossy().to_string());
                match default_dir {
                    Some(dir) => format!("<none specified, using home directory> {}", dir),
                    None => "<none specified, could not find home directory>".to_string(),
                }
            })
    }

    pub fn shell_to_string(&self) -> Option<String> {
        self.shell.as_ref().map(|shell| match shell {
            Shell::System => "<system shell>".to_string(),
            Shell::Program(p) => p.to_string(),
            Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
        })
    }

    pub fn fmt_shell(&self) -> String {
        self.shell
            .clone()
            .map(|shell| match shell {
                Shell::System => {
                    let mut buf = [0; 1024];
                    let pw = alacritty_unix::get_pw_entry(&mut buf).ok();

                    match pw {
                        Some(pw) => format!("<system defined shell> {}", pw.shell),
                        None => "<could not access the password file>".to_string(),
                    }
                }
                Shell::Program(s) => s,
                Shell::WithArguments { program, args } => format!("{} {}", program, args.join(" ")),
            })
            .unwrap_or_else(|| {
                let mut buf = [0; 1024];
                let pw = alacritty_unix::get_pw_entry(&mut buf).ok();
                match pw {
                    Some(pw) => {
                        format!("<none specified, using system defined shell> {}", pw.shell)
                    }
                    None => "<none specified, could not access the password file> {}".to_string(),
                }
            })
    }
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_string: String = self.fmt_directory();
        let shell = self.fmt_shell();

        write!(
            f,
            "Working directory: {} Shell command: `{}`, IOError: {}",
            dir_string, shell, self.source
        )
    }
}

pub struct TerminalBuilder {
    terminal: Terminal,
    events_rx: UnboundedReceiver<AlacTermEvent>,
}

impl TerminalBuilder {
    pub fn new(
        working_directory: Option<PathBuf>,
        shell: Option<Shell>,
        env: Option<HashMap<String, String>>,
        initial_size: TerminalSize,
        blink_settings: Option<TerminalBlink>,
        alternate_scroll: &AlternateScroll,
    ) -> Result<TerminalBuilder> {
        let pty_config = {
            let alac_shell = shell.clone().and_then(|shell| match shell {
                Shell::System => None,
                Shell::Program(program) => Some(Program::Just(program)),
                Shell::WithArguments { program, args } => Some(Program::WithArgs { program, args }),
            });

            PtyConfig {
                shell: alac_shell,
                working_directory: working_directory.clone(),
                hold: false,
            }
        };

        let mut env = env.unwrap_or_default();

        //TODO: Properly set the current locale,
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());

        let alac_scrolling = Scrolling::default();
        // alac_scrolling.set_history((BACK_BUFFER_SIZE * 2) as u32);

        let config = Config {
            pty_config: pty_config.clone(),
            env,
            scrolling: alac_scrolling,
            ..Default::default()
        };

        setup_env(&config);

        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        //TODO: Remove with a bounded sender which can be dispatched on &self
        let (events_tx, events_rx) = unbounded();
        //Set up the terminal...
        let mut term = Term::new(&config, &initial_size, ZedListener(events_tx.clone()));

        //Start off blinking if we need to
        if let Some(TerminalBlink::On) = blink_settings {
            term.set_mode(alacritty_terminal::ansi::Mode::BlinkingCursor)
        }

        //Start alternate_scroll if we need to
        if let AlternateScroll::On = alternate_scroll {
            term.set_mode(alacritty_terminal::ansi::Mode::AlternateScroll)
        } else {
            //Alacritty turns it on by default, so we need to turn it off.
            term.unset_mode(alacritty_terminal::ansi::Mode::AlternateScroll)
        }

        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = match tty::new(&pty_config, initial_size.into(), None) {
            Ok(pty) => pty,
            Err(error) => {
                bail!(TerminalError {
                    directory: working_directory,
                    shell,
                    source: error,
                });
            }
        };

        let shell_txt = {
            match shell {
                Some(Shell::System) | None => {
                    let mut buf = [0; 1024];
                    let pw = alacritty_unix::get_pw_entry(&mut buf).unwrap();
                    pw.shell.to_string()
                }
                Some(Shell::Program(program)) => program,
                Some(Shell::WithArguments { program, args }) => {
                    format!("{} {}", program, args.join(" "))
                }
            }
        };

        //And connect them together
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_config.hold,
            false,
        );

        //Kick things off
        let pty_tx = event_loop.channel();
        let _io_thread = event_loop.spawn();

        let terminal = Terminal {
            pty_tx: Notifier(pty_tx),
            term,
            events: vec![],
            title: shell_txt.clone(),
            default_title: shell_txt,
            last_mode: TermMode::NONE,
            cur_size: initial_size,
            last_mouse: None,
            last_offset: 0,
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    pub fn subscribe(mut self, cx: &mut ModelContext<Terminal>) -> Terminal {
        //Event loop
        cx.spawn_weak(|this, mut cx| async move {
            use futures::StreamExt;

            while let Some(event) = self.events_rx.next().await {
                this.upgrade(&cx)?.update(&mut cx, |this, cx| {
                    //Process the first event immediately for lowered latency
                    this.process_event(&event, cx);
                });

                'outer: loop {
                    let mut events = vec![];
                    let mut timer = cx.background().timer(Duration::from_millis(4)).fuse();

                    loop {
                        futures::select_biased! {
                            _ = timer => break,
                            event = self.events_rx.next() => {
                                if let Some(event) = event {
                                    events.push(event);
                                    if events.len() > 100 {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            },
                        }
                    }

                    if events.is_empty() {
                        smol::future::yield_now().await;
                        break 'outer;
                    } else {
                        this.upgrade(&cx)?.update(&mut cx, |this, cx| {
                            for event in events {
                                this.process_event(&event, cx);
                            }
                        });
                        smol::future::yield_now().await;
                    }
                }
            }

            Some(())
        })
        .detach();

        self.terminal
    }
}

pub struct Terminal {
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    events: Vec<InternalEvent>,
    default_title: String,
    title: String,
    cur_size: TerminalSize,
    last_mode: TermMode,
    last_offset: usize,
    last_mouse: Option<(Point, Direction)>,
}

impl Terminal {
    fn process_event(&mut self, event: &AlacTermEvent, cx: &mut ModelContext<Self>) {
        match event {
            AlacTermEvent::Title(title) => {
                self.title = title.to_string();
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.title = self.default_title.clone();
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new(data.to_string()))
            }
            AlacTermEvent::ClipboardLoad(_, format) => self.write_to_pty(format(
                &cx.read_from_clipboard()
                    .map(|ci| ci.text().to_string())
                    .unwrap_or_else(|| "".to_string()),
            )),
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(out.clone()),
            AlacTermEvent::TextAreaSizeRequest(format) => {
                self.write_to_pty(format(self.cur_size.into()))
            }
            AlacTermEvent::CursorBlinkingChange => {
                cx.emit(Event::BlinkChanged);
            }
            AlacTermEvent::Bell => {
                cx.emit(Event::Bell);
            }
            AlacTermEvent::Exit => cx.emit(Event::CloseTerminal),
            AlacTermEvent::MouseCursorDirty => {
                //NOOP, Handled in render
            }
            AlacTermEvent::Wakeup => {
                cx.emit(Event::Wakeup);
                cx.notify();
            }
            AlacTermEvent::ColorRequest(_, _) => {
                self.events.push(InternalEvent::TermEvent(event.clone()))
            }
        }
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: &InternalEvent,
        term: &mut Term<ZedListener>,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            InternalEvent::TermEvent(term_event) => {
                if let AlacTermEvent::ColorRequest(index, format) = term_event {
                    let color = term.colors()[*index].unwrap_or_else(|| {
                        let term_style = &cx.global::<Settings>().theme.terminal;
                        to_alac_rgb(get_color_at_index(index, &term_style.colors))
                    });
                    self.write_to_pty(format(color))
                }
            }
            InternalEvent::Resize(new_size) => {
                self.cur_size = *new_size;

                self.pty_tx.0.send(Msg::Resize((*new_size).into())).ok();

                term.resize(*new_size);
            }
            InternalEvent::Clear => {
                self.write_to_pty("\x0c".to_string());
                term.clear_screen(ClearMode::Saved);
            }
            InternalEvent::Scroll(scroll) => term.scroll_display(*scroll),
            InternalEvent::SetSelection(sel) => term.selection = sel.clone(),
            InternalEvent::UpdateSelection((point, side)) => {
                if let Some(mut selection) = term.selection.take() {
                    selection.update(*point, *side);
                    term.selection = Some(selection);
                }
            }

            InternalEvent::Copy => {
                if let Some(txt) = term.selection_to_string() {
                    cx.write_to_clipboard(ClipboardItem::new(txt))
                }
            }
        }
    }

    pub fn input(&mut self, input: String) {
        self.events.push(InternalEvent::Scroll(Scroll::Bottom));
        self.events.push(InternalEvent::SetSelection(None));
        self.write_to_pty(input);
    }

    ///Write the Input payload to the tty.
    fn write_to_pty(&self, input: String) {
        self.pty_tx.notify(input.into_bytes());
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_size: TerminalSize) {
        self.events.push(InternalEvent::Resize(new_size))
    }

    pub fn clear(&mut self) {
        self.events.push(InternalEvent::Clear)
    }

    pub fn try_keystroke(&mut self, keystroke: &Keystroke) -> bool {
        let esc = to_esc_str(keystroke, &self.last_mode);
        if let Some(esc) = esc {
            self.input(esc);
            true
        } else {
            false
        }
    }

    ///Paste text into the terminal
    pub fn paste(&mut self, text: &str) {
        let paste_text = if self.last_mode.contains(TermMode::BRACKETED_PASTE) {
            format!("{}{}{}", "\x1b[200~", text.replace('\x1b', ""), "\x1b[201~")
        } else {
            text.replace("\r\n", "\r").replace('\n', "\r")
        };
        self.input(paste_text)
    }

    pub fn copy(&mut self) {
        self.events.push(InternalEvent::Copy);
    }

    pub fn render_lock<F, T>(&mut self, cx: &mut ModelContext<Self>, f: F) -> T
    where
        F: FnOnce(RenderableContent, char) -> T,
    {
        let m = self.term.clone(); //Arc clone
        let mut term = m.lock();

        while let Some(e) = self.events.pop() {
            self.process_terminal_event(&e, &mut term, cx)
        }

        self.last_mode = *term.mode();

        let content = term.renderable_content();

        self.last_offset = content.display_offset;

        let cursor_text = term.grid()[content.cursor.point].c;

        f(content, cursor_text)
    }

    pub fn focus_in(&self) {
        if self.last_mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[I".to_string());
        }
    }

    pub fn focus_out(&self) {
        if self.last_mode.contains(TermMode::FOCUS_IN_OUT) {
            self.write_to_pty("\x1b[O".to_string());
        }
    }

    pub fn mouse_changed(&mut self, point: Point, side: Direction) -> bool {
        match self.last_mouse {
            Some((old_point, old_side)) => {
                if old_point == point && old_side == side {
                    false
                } else {
                    self.last_mouse = Some((point, side));
                    true
                }
            }
            None => {
                self.last_mouse = Some((point, side));
                true
            }
        }
    }

    pub fn mouse_mode(&self, shift: bool) -> bool {
        self.last_mode.intersects(TermMode::MOUSE_MODE) && !shift
    }

    pub fn mouse_move(&mut self, e: &MouseMovedEvent, origin: Vector2F) {
        let position = e.position.sub(origin);

        let point = mouse_point(position, self.cur_size, self.last_offset);
        let side = mouse_side(position, self.cur_size);

        if self.mouse_changed(point, side) && self.mouse_mode(e.shift) {
            if let Some(bytes) = mouse_moved_report(point, e, self.last_mode) {
                self.pty_tx.notify(bytes);
            }
        }
    }

    pub fn mouse_drag(&mut self, e: MouseMovedEvent, origin: Vector2F) {
        let position = e.position.sub(origin);

        if !self.mouse_mode(e.shift) {
            let point = mouse_point(position, self.cur_size, self.last_offset);
            let side = mouse_side(position, self.cur_size);

            self.events
                .push(InternalEvent::UpdateSelection((point, side)));
        }
    }

    pub fn mouse_down(&mut self, e: &MouseButtonEvent, origin: Vector2F) {
        let position = e.position.sub(origin);

        let point = mouse_point(position, self.cur_size, self.last_offset);
        let side = mouse_side(position, self.cur_size);

        if self.mouse_mode(e.shift) {
            if let Some(bytes) = mouse_button_report(point, e, true, self.last_mode) {
                self.pty_tx.notify(bytes);
            }
        } else {
            self.events
                .push(InternalEvent::SetSelection(Some(Selection::new(
                    SelectionType::Simple,
                    point,
                    side,
                ))));
        }
    }

    pub fn left_click(&mut self, e: &MouseButtonEvent, origin: Vector2F) {
        let position = e.position.sub(origin);

        if !self.mouse_mode(e.shift) {
            let point = mouse_point(position, self.cur_size, self.last_offset);
            let side = mouse_side(position, self.cur_size);

            let selection_type = match e.click_count {
                0 => return, //This is a release
                1 => Some(SelectionType::Simple),
                2 => Some(SelectionType::Semantic),
                3 => Some(SelectionType::Lines),
                _ => None,
            };

            let selection =
                selection_type.map(|selection_type| Selection::new(selection_type, point, side));

            self.events.push(InternalEvent::SetSelection(selection));
        }
    }

    pub fn mouse_up(&mut self, e: &MouseButtonEvent, origin: Vector2F) {
        let position = e.position.sub(origin);

        if self.mouse_mode(e.shift) {
            let point = mouse_point(position, self.cur_size, self.last_offset);

            if let Some(bytes) = mouse_button_report(point, e, false, self.last_mode) {
                self.pty_tx.notify(bytes);
            }
        } else {
            // Seems pretty standard to automatically copy on mouse_up for terminals,
            // so let's do that here
            self.copy();
        }
    }

    ///Scroll the terminal
    pub fn scroll(&mut self, scroll: &ScrollWheelEvent, origin: Vector2F) {
        if self.mouse_mode(scroll.shift) {
            //TODO: Currently this only sends the current scroll reports as they come in. Alacritty
            //Sends the *entire* scroll delta on *every* scroll event, only resetting it when
            //The scroll enters 'TouchPhase::Started'. Do I need to replicate this?
            //This would be consistent with a scroll model based on 'distance from origin'...
            let scroll_lines = (scroll.delta.y() / self.cur_size.line_height) as i32;
            let point = mouse_point(scroll.position.sub(origin), self.cur_size, self.last_offset);

            if let Some(scrolls) = scroll_report(point, scroll_lines as i32, scroll, self.last_mode)
            {
                for scroll in scrolls {
                    self.pty_tx.notify(scroll);
                }
            };
        } else if self
            .last_mode
            .contains(TermMode::ALT_SCREEN | TermMode::ALTERNATE_SCROLL)
            && !scroll.shift
        {
            //TODO: See above TODO, also applies here.
            let scroll_lines = ((scroll.delta.y() * ALACRITTY_SCROLL_MULTIPLIER)
                / self.cur_size.line_height) as i32;

            self.pty_tx.notify(alt_scroll(scroll_lines))
        } else {
            let scroll_lines = ((scroll.delta.y() * ALACRITTY_SCROLL_MULTIPLIER)
                / self.cur_size.line_height) as i32;
            if scroll_lines != 0 {
                let scroll = Scroll::Delta(scroll_lines);
                self.events.push(InternalEvent::Scroll(scroll));
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }
}

impl Entity for Terminal {
    type Event = Event;
}

#[cfg(test)]
mod tests {
    pub mod terminal_test_context;
}

//TODO Move this around and clean up the code
mod alacritty_unix {
    use alacritty_terminal::config::Program;
    use gpui::anyhow::{bail, Result};

    use std::ffi::CStr;
    use std::mem::MaybeUninit;
    use std::ptr;

    #[derive(Debug)]
    pub struct Passwd<'a> {
        _name: &'a str,
        _dir: &'a str,
        pub shell: &'a str,
    }

    /// Return a Passwd struct with pointers into the provided buf.
    ///
    /// # Unsafety
    ///
    /// If `buf` is changed while `Passwd` is alive, bad thing will almost certainly happen.
    pub fn get_pw_entry(buf: &mut [i8; 1024]) -> Result<Passwd<'_>> {
        // Create zeroed passwd struct.
        let mut entry: MaybeUninit<libc::passwd> = MaybeUninit::uninit();

        let mut res: *mut libc::passwd = ptr::null_mut();

        // Try and read the pw file.
        let uid = unsafe { libc::getuid() };
        let status = unsafe {
            libc::getpwuid_r(
                uid,
                entry.as_mut_ptr(),
                buf.as_mut_ptr() as *mut _,
                buf.len(),
                &mut res,
            )
        };
        let entry = unsafe { entry.assume_init() };

        if status < 0 {
            bail!("getpwuid_r failed");
        }

        if res.is_null() {
            bail!("pw not found");
        }

        // Sanity check.
        assert_eq!(entry.pw_uid, uid);

        // Build a borrowed Passwd struct.
        Ok(Passwd {
            _name: unsafe { CStr::from_ptr(entry.pw_name).to_str().unwrap() },
            _dir: unsafe { CStr::from_ptr(entry.pw_dir).to_str().unwrap() },
            shell: unsafe { CStr::from_ptr(entry.pw_shell).to_str().unwrap() },
        })
    }

    #[cfg(target_os = "macos")]
    pub fn _default_shell(pw: &Passwd<'_>) -> Program {
        let shell_name = pw.shell.rsplit('/').next().unwrap();
        let argv = vec![
            String::from("-c"),
            format!("exec -a -{} {}", shell_name, pw.shell),
        ];

        Program::WithArgs {
            program: "/bin/bash".to_owned(),
            args: argv,
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn default_shell(pw: &Passwd<'_>) -> Program {
        Program::Just(env::var("SHELL").unwrap_or_else(|_| pw.shell.to_owned()))
    }
}
