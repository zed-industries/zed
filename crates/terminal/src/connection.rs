mod keymappings;

use alacritty_terminal::{
    ansi::{ClearMode, Handler},
    config::{Config, Program, PtyConfig},
    event::{Event as AlacTermEvent, EventListener, Notify, WindowSize},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Scroll,
    index::{Direction, Point},
    selection::{Selection, SelectionType},
    sync::FairMutex,
    term::{test::TermSize, RenderableContent, TermMode},
    tty::{self, setup_env},
    Term,
};
use anyhow::{bail, Result};
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    StreamExt,
};
use settings::{Settings, Shell};
use std::{collections::HashMap, fmt::Display, path::PathBuf, sync::Arc};
use thiserror::Error;

use gpui::{keymap::Keystroke, ClipboardItem, CursorStyle, Entity, ModelContext};

use crate::{
    color_translation::{get_color_at_index, to_alac_rgb},
    terminal_element::TerminalDimensions,
};

use self::keymappings::to_esc_str;

const DEFAULT_TITLE: &str = "Terminal";

///Upward flowing events, for changing the title and such
#[derive(Copy, Clone, Debug)]
pub enum Event {
    TitleChanged,
    CloseTerminal,
    Activate,
    Wakeup,
    Bell,
    KeyInput,
}

///A translation struct for Alacritty to communicate with us from their event loop
#[derive(Clone)]
pub struct ZedListener(UnboundedSender<AlacTermEvent>);

impl EventListener for ZedListener {
    fn send_event(&self, event: AlacTermEvent) {
        self.0.unbounded_send(event).ok();
    }
}

#[derive(Error, Debug)]
pub struct TerminalError {
    pub directory: Option<PathBuf>,
    pub shell: Option<Shell>,
    pub source: std::io::Error,
}

impl Display for TerminalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir_string: String = self
            .directory
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
                    Some(dir) => format!("<none specified, using home> {}", dir),
                    None => "<none specified, could not find home>".to_string(),
                }
            });

        let shell = self
            .shell
            .clone()
            .map(|shell| match shell {
                Shell::System => {
                    let mut buf = [0; 1024];
                    let pw = alacritty_unix::get_pw_entry(&mut buf).ok();

                    match pw {
                        Some(pw) => format!("<system defined shell> {}", pw.shell),
                        None => "<could not access system defined shell>".to_string(),
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
                    None => {
                        "<none specified, could not access system defined shell> {}".to_string()
                    }
                }
            });

        write!(
            f,
            "Working directory: {} Shell command: `{}`, IOError: {}",
            dir_string, shell, self.source
        )
    }
}

pub struct DisconnectedPTY {
    terminal: Terminal,
    events_rx: UnboundedReceiver<AlacTermEvent>,
}

impl DisconnectedPTY {
    pub fn new(
        working_directory: Option<PathBuf>,
        shell: Option<Shell>,
        env: Option<HashMap<String, String>>,
        initial_size: TerminalDimensions,
    ) -> Result<DisconnectedPTY> {
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
            title: shell_txt.to_string(),
            associated_directory: working_directory,
        };

        Ok(DisconnectedPTY {
            terminal,
            events_rx,
        })
    }

    pub fn connect(mut self, cx: &mut ModelContext<Terminal>) -> Terminal {
        cx.spawn_weak(|this, mut cx| async move {
            //Listen for terminal events
            while let Some(event) = self.events_rx.next().await {
                match this.upgrade(&cx) {
                    Some(this) => {
                        this.update(&mut cx, |this, cx| {
                            this.process_terminal_event(event, cx);

                            cx.notify();
                        });
                    }
                    None => break,
                }
            }
        })
        .detach();

        self.terminal
    }
}

pub struct Terminal {
    pty_tx: Notifier,
    term: Arc<FairMutex<Term<ZedListener>>>,
    pub title: String,
    pub associated_directory: Option<PathBuf>,
}

impl Terminal {
    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: alacritty_terminal::event::Event,
        cx: &mut ModelContext<Terminal>,
    ) {
        match event {
            // TODO: Handle is_self_focused in subscription on terminal view
            AlacTermEvent::Wakeup => {
                cx.emit(Event::Wakeup);
            }
            AlacTermEvent::PtyWrite(out) => self.write_to_pty(out),
            AlacTermEvent::MouseCursorDirty => {
                //Calculate new cursor style.
                //TODO: alacritty/src/input.rs:L922-L939
                //Check on correctly handling mouse events for terminals
                cx.platform().set_cursor_style(CursorStyle::Arrow); //???
            }
            AlacTermEvent::Title(title) => {
                self.title = title;
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ResetTitle => {
                self.title = DEFAULT_TITLE.to_string();
                cx.emit(Event::TitleChanged);
            }
            AlacTermEvent::ClipboardStore(_, data) => {
                cx.write_to_clipboard(ClipboardItem::new(data))
            }
            AlacTermEvent::ClipboardLoad(_, format) => self.write_to_pty(format(
                &cx.read_from_clipboard()
                    .map(|ci| ci.text().to_string())
                    .unwrap_or("".to_string()),
            )),
            AlacTermEvent::ColorRequest(index, format) => {
                let color = self.term.lock().colors()[index].unwrap_or_else(|| {
                    let term_style = &cx.global::<Settings>().theme.terminal;
                    to_alac_rgb(get_color_at_index(&index, &term_style.colors))
                });
                self.write_to_pty(format(color))
            }
            AlacTermEvent::CursorBlinkingChange => {
                //TODO: Set a timer to blink the cursor on and off
            }
            AlacTermEvent::Bell => {
                cx.emit(Event::Bell);
            }
            AlacTermEvent::Exit => cx.emit(Event::CloseTerminal),
            AlacTermEvent::TextAreaSizeRequest(_) => println!("Received text area resize request"),
        }
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    pub fn write_to_pty(&self, input: String) {
        self.write_bytes_to_pty(input.into_bytes());
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    fn write_bytes_to_pty(&self, input: Vec<u8>) {
        self.term.lock().scroll_display(Scroll::Bottom);
        self.pty_tx.notify(input);
    }

    ///Resize the terminal and the PTY. This locks the terminal.
    pub fn set_size(&self, new_size: WindowSize) {
        self.pty_tx.0.send(Msg::Resize(new_size)).ok();

        let term_size = TermSize::new(new_size.num_cols as usize, new_size.num_lines as usize);
        self.term.lock().resize(term_size);
    }

    pub fn clear(&self) {
        self.write_to_pty("\x0c".into());
        self.term.lock().clear_screen(ClearMode::Saved);
    }

    pub fn try_keystroke(&self, keystroke: &Keystroke) -> bool {
        let guard = self.term.lock();
        let mode = guard.mode();
        let esc = to_esc_str(keystroke, mode);
        drop(guard);
        if esc.is_some() {
            self.write_to_pty(esc.unwrap());
            true
        } else {
            false
        }
    }

    ///Paste text into the terminal
    pub fn paste(&self, text: &str) {
        if self.term.lock().mode().contains(TermMode::BRACKETED_PASTE) {
            self.write_to_pty("\x1b[200~".to_string());
            self.write_to_pty(text.replace('\x1b', "").to_string());
            self.write_to_pty("\x1b[201~".to_string());
        } else {
            self.write_to_pty(text.replace("\r\n", "\r").replace('\n', "\r"));
        }
    }

    pub fn copy(&self) -> Option<String> {
        let term = self.term.lock();
        term.selection_to_string()
    }

    ///Takes the selection out of the terminal
    pub fn take_selection(&self) -> Option<Selection> {
        self.term.lock().selection.take()
    }
    ///Sets the selection object on the terminal
    pub fn set_selection(&self, sel: Option<Selection>) {
        self.term.lock().selection = sel;
    }

    pub fn render_lock<F, T>(&self, new_size: Option<TerminalDimensions>, f: F) -> T
    where
        F: FnOnce(RenderableContent, char) -> T,
    {
        if let Some(new_size) = new_size {
            self.pty_tx.0.send(Msg::Resize(new_size.into())).ok(); //Give the PTY a chance to react to the new size
                                                                   //TODO: Is this bad for performance?
        }

        let mut term = self.term.lock(); //Lock

        if let Some(new_size) = new_size {
            term.resize(new_size); //Reflow
        }

        let content = term.renderable_content();
        let cursor_text = term.grid()[content.cursor.point].c;

        f(content, cursor_text)
    }

    pub fn get_display_offset(&self) -> usize {
        self.term.lock().renderable_content().display_offset
    }

    ///Scroll the terminal
    pub fn scroll(&self, scroll: Scroll) {
        self.term.lock().scroll_display(scroll)
    }

    pub fn click(&self, point: Point, side: Direction, clicks: usize) {
        let selection_type = match clicks {
            0 => return, //This is a release
            1 => Some(SelectionType::Simple),
            2 => Some(SelectionType::Semantic),
            3 => Some(SelectionType::Lines),
            _ => None,
        };

        let selection =
            selection_type.map(|selection_type| Selection::new(selection_type, point, side));

        self.set_selection(selection);
    }

    pub fn drag(&self, point: Point, side: Direction) {
        if let Some(mut selection) = self.take_selection() {
            selection.update(point, side);
            self.set_selection(Some(selection));
        }
    }

    pub fn mouse_down(&self, point: Point, side: Direction) {
        self.set_selection(Some(Selection::new(SelectionType::Simple, point, side)));
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

//TODO Move this around
mod alacritty_unix {
    use alacritty_terminal::config::Program;
    use gpui::anyhow::{bail, Result};
    use libc::{self};
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
