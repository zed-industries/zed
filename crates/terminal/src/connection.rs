mod keymappings;

use alacritty_terminal::{
    ansi::{ClearMode, Handler},
    config::{Config, Program, PtyConfig},
    event::{Event as AlacTermEvent, EventListener, Notify},
    event_loop::{EventLoop, Msg, Notifier},
    grid::Scroll,
    sync::FairMutex,
    term::{SizeInfo, TermMode},
    tty::{self, setup_env},
    Term,
};
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use settings::{Settings, Shell};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use gpui::{keymap::Keystroke, ClipboardItem, CursorStyle, Entity, ModelContext};

use crate::color_translation::{get_color_at_index, to_alac_rgb};

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

pub struct TerminalConnection {
    pub pty_tx: Notifier,
    pub term: Arc<FairMutex<Term<ZedListener>>>,
    pub title: String,
    pub associated_directory: Option<PathBuf>,
}

impl TerminalConnection {
    pub fn new(
        working_directory: Option<PathBuf>,
        shell: Option<Shell>,
        env_vars: Option<Vec<(String, String)>>,
        initial_size: SizeInfo,
        cx: &mut ModelContext<Self>,
    ) -> TerminalConnection {
        let pty_config = {
            let shell = shell.and_then(|shell| match shell {
                Shell::System => None,
                Shell::Program(program) => Some(Program::Just(program)),
                Shell::WithArguments { program, args } => Some(Program::WithArgs { program, args }),
            });

            PtyConfig {
                shell,
                working_directory: working_directory.clone(),
                hold: false,
            }
        };

        let mut env: HashMap<String, String> = HashMap::new();
        if let Some(envs) = env_vars {
            for (var, val) in envs {
                env.insert(var, val);
            }
        }

        //TODO: Properly set the current locale,
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());

        let config = Config {
            pty_config: pty_config.clone(),
            env,
            ..Default::default()
        };

        setup_env(&config);

        //Spawn a task so the Alacritty EventLoop can communicate with us in a view context
        let (events_tx, mut events_rx) = unbounded();

        //Set up the terminal...
        let term = Term::new(&config, initial_size, ZedListener(events_tx.clone()));
        let term = Arc::new(FairMutex::new(term));

        //Setup the pty...
        let pty = {
            if let Some(pty) = tty::new(&pty_config, &initial_size, None).ok() {
                pty
            } else {
                let pty_config = PtyConfig {
                    shell: None,
                    working_directory: working_directory.clone(),
                    ..Default::default()
                };

                tty::new(&pty_config, &initial_size, None)
                    .expect("Failed with default shell too :(")
            }
        };

        let shell = {
            let mut buf = [0; 1024];
            let pw = alacritty_unix::get_pw_entry(&mut buf).unwrap();
            pw.shell.to_string()
            // alacritty_unix::default_shell(&pw)
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

        cx.spawn_weak(|this, mut cx| async move {
            //Listen for terminal events
            while let Some(event) = events_rx.next().await {
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

        TerminalConnection {
            pty_tx: Notifier(pty_tx),
            term,
            title: shell.to_string(),
            associated_directory: working_directory,
        }
    }

    ///Takes events from Alacritty and translates them to behavior on this view
    fn process_terminal_event(
        &mut self,
        event: alacritty_terminal::event::Event,
        cx: &mut ModelContext<Self>,
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
        }
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    pub fn write_to_pty(&mut self, input: String) {
        self.write_bytes_to_pty(input.into_bytes());
    }

    ///Write the Input payload to the tty. This locks the terminal so we can scroll it.
    fn write_bytes_to_pty(&mut self, input: Vec<u8>) {
        self.term.lock().scroll_display(Scroll::Bottom);
        self.pty_tx.notify(input);
    }

    ///Resize the terminal and the PTY. This locks the terminal.
    pub fn set_size(&mut self, new_size: SizeInfo) {
        self.pty_tx.0.send(Msg::Resize(new_size)).ok();
        self.term.lock().resize(new_size);
    }

    pub fn clear(&mut self) {
        self.write_to_pty("\x0c".into());
        self.term.lock().clear_screen(ClearMode::Saved);
    }

    pub fn try_keystroke(&mut self, keystroke: &Keystroke) -> bool {
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
    pub fn paste(&mut self, text: &str) {
        if self.term.lock().mode().contains(TermMode::BRACKETED_PASTE) {
            self.write_to_pty("\x1b[200~".to_string());
            self.write_to_pty(text.replace('\x1b', "").to_string());
            self.write_to_pty("\x1b[201~".to_string());
        } else {
            self.write_to_pty(text.replace("\r\n", "\r").replace('\n', "\r"));
        }
    }
}

impl Drop for TerminalConnection {
    fn drop(&mut self) {
        self.pty_tx.0.send(Msg::Shutdown).ok();
    }
}

impl Entity for TerminalConnection {
    type Event = Event;
}

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
