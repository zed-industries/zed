pub mod connected_el;
pub mod connected_view;
pub mod mappings;
pub mod modal;
pub mod terminal_view;

#[cfg(test)]
use alacritty_terminal::term::cell::Cell;
#[cfg(test)]
use alacritty_terminal::Grid;

use alacritty_terminal::{
    ansi::{ClearMode, Handler},
    config::{Config, Program, PtyConfig},
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
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use mappings::keys::might_convert;
use modal::deploy_modal;
use settings::{Settings, Shell};
use std::{collections::HashMap, fmt::Display, path::PathBuf, sync::Arc, time::Duration};
use terminal_view::TerminalView;
use thiserror::Error;

use gpui::{
    geometry::vector::{vec2f, Vector2F},
    keymap::Keystroke,
    ClipboardItem, CursorStyle, Entity, ModelContext, MutableAppContext,
};

use crate::mappings::{
    colors::{get_color_at_index, to_alac_rgb},
    keys::to_esc_str,
};

///Initialize and register all of our action handlers
pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(TerminalView::deploy);
    cx.add_action(deploy_modal);

    connected_view::init(cx);
}

const DEBUG_TERMINAL_WIDTH: f32 = 500.;
const DEBUG_TERMINAL_HEIGHT: f32 = 30.; //This needs to be wide enough that the CI & a local dev's prompt can fill the whole space.
const DEBUG_CELL_WIDTH: f32 = 5.;
const DEBUG_LINE_HEIGHT: f32 = 5.;

///Upward flowing events, for changing the title and such
#[derive(Clone, Copy, Debug)]
pub enum Event {
    TitleChanged,
    CloseTerminal,
    Activate,
    Bell,
    Wakeup,
}

#[derive(Clone, Debug)]
enum InternalEvent {
    TermEvent(AlacTermEvent),
    Resize(TermDimensions),
    Clear,
    Keystroke(Keystroke),
    Paste(String),
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
pub struct TermDimensions {
    cell_width: f32,
    line_height: f32,
    height: f32,
    width: f32,
}

impl TermDimensions {
    pub fn new(line_height: f32, cell_width: f32, size: Vector2F) -> Self {
        TermDimensions {
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
impl Default for TermDimensions {
    fn default() -> Self {
        TermDimensions::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            vec2f(DEBUG_TERMINAL_WIDTH, DEBUG_TERMINAL_HEIGHT),
        )
    }
}

impl Into<WindowSize> for TermDimensions {
    fn into(self) -> WindowSize {
        WindowSize {
            num_lines: self.num_lines() as u16,
            num_cols: self.num_columns() as u16,
            cell_width: self.cell_width() as u16,
            cell_height: self.line_height() as u16,
        }
    }
}

impl Dimensions for TermDimensions {
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
        initial_size: TermDimensions,
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

        let mut env = env.unwrap_or_else(|| HashMap::new());

        //TODO: Properly set the current locale,
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());

        let config = Config {
            pty_config: pty_config.clone(),
            env,
            ..Default::default()
        };

        setup_env(&config);

        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        let (events_tx, events_rx) = unbounded();
        //Set up the terminal...
        let term = Term::new(&config, &initial_size, ZedListener(events_tx.clone()));
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
        };

        Ok(TerminalBuilder {
            terminal,
            events_rx,
        })
    }

    pub fn subscribe(mut self, cx: &mut ModelContext<Terminal>) -> Terminal {
        cx.spawn_weak(|this, mut cx| async move {
            'outer: loop {
                //TODO: Pending GPUI updates, sync this to some higher, smarter system.
                let delay = cx.background().timer(Duration::from_secs_f32(1.0 / 60.));

                let mut events = vec![];

                loop {
                    match self.events_rx.try_next() {
                        //Have a buffered event
                        Ok(Some(e)) => events.push(e),
                        //Channel closed, exit
                        Ok(None) => break 'outer,
                        //Ran out of buffered events
                        Err(_) => break,
                    }
                }
                match this.upgrade(&cx) {
                    Some(this) => {
                        this.update(&mut cx, |this, _cx| {
                            this.push_events(events);
                        });
                    }
                    None => break 'outer,
                }

                delay.await;
            }
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
}

impl Terminal {
    fn push_events(&mut self, events: Vec<AlacTermEvent>) {
        self.events
            .extend(events.into_iter().map(|e| InternalEvent::TermEvent(e)))
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: &InternalEvent,
        term: &mut Term<ZedListener>,
        cx: &mut ModelContext<Self>,
    ) {
        dbg!(event);
        // TODO: Handle is_self_focused in subscription on terminal view
        match event {
            InternalEvent::TermEvent(term_event) => match term_event {
                AlacTermEvent::Wakeup => {
                    cx.emit(Event::Wakeup);
                }
                AlacTermEvent::PtyWrite(out) => self.notify_pty(out.clone()),
                AlacTermEvent::MouseCursorDirty => {
                    //Calculate new cursor style.
                    //TODO: alacritty/src/input.rs:L922-L939
                    //Check on correctly handling mouse events for terminals
                    cx.platform().set_cursor_style(CursorStyle::Arrow); //???
                }
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
                AlacTermEvent::ClipboardLoad(_, format) => self.notify_pty(format(
                    &cx.read_from_clipboard()
                        .map(|ci| ci.text().to_string())
                        .unwrap_or("".to_string()),
                )),
                AlacTermEvent::ColorRequest(index, format) => {
                    let color = term.colors()[*index].unwrap_or_else(|| {
                        let term_style = &cx.global::<Settings>().theme.terminal;
                        to_alac_rgb(get_color_at_index(index, &term_style.colors))
                    });
                    self.notify_pty(format(color))
                }
                AlacTermEvent::CursorBlinkingChange => {
                    //TODO: Set a timer to blink the cursor on and off
                }
                AlacTermEvent::Bell => {
                    cx.emit(Event::Bell);
                }
                AlacTermEvent::Exit => cx.emit(Event::CloseTerminal),
                AlacTermEvent::TextAreaSizeRequest(_) => {
                    println!("Received text area resize request")
                }
            },
            InternalEvent::Resize(new_size) => {
                self.pty_tx
                    .0
                    .send(Msg::Resize(new_size.clone().into()))
                    .ok();

                term.resize(*new_size);
            }
            InternalEvent::Clear => {
                self.notify_pty("\x0c".to_string());
                term.clear_screen(ClearMode::Saved);
            }
            InternalEvent::Keystroke(keystroke) => {
                println!("Trying keystroke: {}", keystroke);
                let esc = to_esc_str(keystroke, term.mode());
                if let Some(esc) = esc {
                    self.notify_pty(esc);
                }
            }
            InternalEvent::Paste(text) => {
                if term.mode().contains(TermMode::BRACKETED_PASTE) {
                    self.notify_pty("\x1b[200~".to_string());
                    self.notify_pty(text.replace('\x1b', "").to_string());
                    self.notify_pty("\x1b[201~".to_string());
                } else {
                    self.notify_pty(text.replace("\r\n", "\r").replace('\n', "\r"));
                }
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

    fn notify_pty(&self, txt: String) {
        self.pty_tx.notify(txt.into_bytes());
    }

    ///Write the Input payload to the tty.
    pub fn write_to_pty(&mut self, input: String) {
        self.events
            .push(InternalEvent::TermEvent(AlacTermEvent::PtyWrite(input)))
    }

    ///Resize the terminal and the PTY.
    pub fn set_size(&mut self, new_size: TermDimensions) {
        self.events.push(InternalEvent::Resize(new_size.into()))
    }

    pub fn clear(&mut self) {
        self.events.push(InternalEvent::Clear)
    }

    pub fn try_keystroke(&mut self, keystroke: &Keystroke) -> bool {
        if might_convert(keystroke) {
            self.events
                .push(InternalEvent::Keystroke(keystroke.clone()));
            true
        } else {
            false
        }
    }

    ///Paste text into the terminal
    pub fn paste(&mut self, text: &str) {
        self.events.push(InternalEvent::Paste(text.to_string()));
    }

    pub fn copy(&mut self) {
        self.events.push(InternalEvent::Copy);
    }

    pub fn render_lock<F, T>(&mut self, cx: &mut ModelContext<Self>, f: F) -> T
    where
        F: FnOnce(RenderableContent, char) -> T,
    {
        println!("RENDER LOCK!");
        let m = self.term.clone(); //Arc clone
        let mut term = m.lock();

        while let Some(e) = self.events.pop() {
            self.process_terminal_event(&e, &mut term, cx)
        }

        let content = term.renderable_content();
        let cursor_text = term.grid()[content.cursor.point].c;

        f(content, cursor_text)
    }

    ///Scroll the terminal
    pub fn scroll(&mut self, scroll: Scroll) {
        self.events.push(InternalEvent::Scroll(scroll));
    }

    pub fn click(&mut self, point: Point, side: Direction, clicks: usize) {
        let selection_type = match clicks {
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

    pub fn drag(&mut self, point: Point, side: Direction) {
        self.events
            .push(InternalEvent::UpdateSelection((point, side)));
    }

    ///TODO: Check if the mouse_down-then-click assumption holds, so this code works as expected
    pub fn mouse_down(&mut self, point: Point, side: Direction) {
        self.events
            .push(InternalEvent::SetSelection(Some(Selection::new(
                SelectionType::Simple,
                point,
                side,
            ))));
    }

    #[cfg(test)]
    fn grid(&self) -> Grid<Cell> {
        self.term.lock().grid().clone()
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

    use gpui::TestAppContext;

    use crate::tests::terminal_test_context::TerminalTestContext;

    ///Basic integration test, can we get the terminal to show up, execute a command,
    //and produce noticable output?
    #[gpui::test(retries = 5)]
    async fn test_terminal(cx: &mut TestAppContext) {
        let mut cx = TerminalTestContext::new(cx, true);

        cx.execute_and_wait("expr 3 + 4", |content, _cx| content.contains("7"))
            .await;
    }
}

//TODO Move this around and clean up the code
mod alacritty_unix {
    use alacritty_terminal::config::Program;
    use gpui::anyhow::{bail, Result};
    use libc;
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
