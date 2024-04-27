use std::sync::Arc;

use alacritty_terminal::tty::{self, setup_env, EventedReadWrite};
use gpui::AnyWindowHandle;
pub use polling;
use polling::{Event as PollingEvent, PollMode, Poller};
pub use portable_pty;
use portable_pty::{native_pty_system, PtyPair, PtySize, PtySystem};

use crate::TerminalSize;

pub struct HeadlessTerminal {
    _pty_system: Box<dyn PtySystem + Send>,
    pub pair: PtyPair,
    // pub tty: tty::Pty,
    // pub poll: Arc<Poller>,
}

impl HeadlessTerminal {
    pub fn new(
        _window: Option<AnyWindowHandle>,
        size: Option<TerminalSize>,
    ) -> anyhow::Result<Self> {
        // setup_env();

        // let poll = Arc::new(Poller::new().unwrap());
        // let poll_opts = PollMode::Level;
        // let interest = PollingEvent::readable(0);

        // let mut pty = tty::new(
        //     &alacritty_terminal::tty::Options {
        //         shell: None,
        //         working_directory: Some("/tmp".into()),
        //         hold: false,
        //         env: Default::default(),
        //     },
        //     size.unwrap_or_else(|| TerminalSize::default()).into(),
        //     0,
        //     // window.window_id().as_u64(),
        // )
        // .unwrap();

        // // Register TTY through EventedRW interface.
        // if let Err(err) = unsafe { pty.register(&poll, interest, poll_opts) } {
        //     anyhow::bail!("Failed to register TTY: {}", err);
        // }

        // let pty_system = native_pty_system();
        // let pair = pty_system
        //     .openpty(PtySize {
        //         rows: size.map(|s| s.num_lines() as u16).unwrap_or(24),
        //         cols: size.map(|s| s.num_columns() as u16).unwrap_or(80),
        //         pixel_width: size.map(|s| s.width().0 as u16).unwrap_or(0),
        //         pixel_height: size.map(|s| s.height().0 as u16).unwrap_or(0),
        //     })
        //     .unwrap();

        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: size.map(|s| s.num_lines() as u16).unwrap_or(24),
                cols: size.map(|s| s.num_columns() as u16).unwrap_or(80),
                pixel_width: size.map(|s| s.width().0 as u16).unwrap_or(0),
                pixel_height: size.map(|s| s.height().0 as u16).unwrap_or(0),
            })
            .unwrap();

        Ok(Self {
            _pty_system: pty_system,
            pair,
            // tty: pty,
            // poll: poll,
        })
    }
}
