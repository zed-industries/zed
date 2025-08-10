#[cfg(test)]
mod tests {
    use alacritty_terminal::{
        event::{Event, EventListener, WindowSize},
        event_loop::EventLoop,
        grid::Dimensions as _,
        index::{Column, Line},
        sync::FairMutex,
        term::{Config, Term},
        tty::{self, Options, Shell},
    };
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    struct Size {
        columns: Column,
        lines: Line,
    }

    impl alacritty_terminal::grid::Dimensions for Size {
        fn total_lines(&self) -> usize {
            self.lines.0 as usize
        }

        fn screen_lines(&self) -> usize {
            self.lines.0 as usize
        }

        fn columns(&self) -> usize {
            self.columns.0 as usize
        }
    }

    #[derive(Clone)]
    struct DebugEventListener {
        events: Arc<Mutex<Vec<Event>>>,
        event_count: Arc<Mutex<usize>>,
    }

    impl DebugEventListener {
        fn new() -> Self {
            Self {
                events: Arc::new(Mutex::new(Vec::new())),
                event_count: Arc::new(Mutex::new(0)),
            }
        }

        fn get_events(&self) -> Vec<Event> {
            self.events.lock().unwrap().clone()
        }

        fn get_event_count(&self) -> usize {
            *self.event_count.lock().unwrap()
        }
    }

    impl EventListener for DebugEventListener {
        fn send_event(&self, event: Event) {
            let mut count = self.event_count.lock().unwrap();
            *count += 1;
            let event_num = *count;

            println!("\n=== Event #{} ===", event_num);
            match &event {
                Event::Wakeup => {
                    dbg!("Event::Wakeup");
                }
                Event::PtyWrite(data) => {
                    dbg!("Event::PtyWrite");
                    dbg!(format!("  Data length: {} bytes", data.len()));
                    dbg!(format!("  Data as string: {}", data));
                    dbg!(format!(
                        "  Raw bytes (first 50): {:?}",
                        &data.as_bytes()[..data.len().min(50)]
                    ));
                }
                Event::MouseCursorDirty => {
                    dbg!("Event::MouseCursorDirty");
                }
                Event::Title(title) => {
                    dbg!(format!("Event::Title(\"{}\")", title));
                }
                Event::ResetTitle => {
                    dbg!("Event::ResetTitle");
                }
                Event::ClipboardStore(clipboard_type, data) => {
                    dbg!(format!(
                        "Event::ClipboardStore({:?}, \"{}\")",
                        clipboard_type, data
                    ));
                }
                Event::ClipboardLoad(clipboard_type, _format) => {
                    dbg!(format!(
                        "Event::ClipboardLoad({:?}, <formatter>)",
                        clipboard_type
                    ));
                }
                Event::ColorRequest(index, _format) => {
                    dbg!(format!(
                        "Event::ColorRequest(index: {}, format: <formatter>)",
                        index
                    ));
                }
                Event::ChildExit(code) => {
                    dbg!(format!("Event::ChildExit(code: {:?})", code));
                }
                Event::Exit => {
                    dbg!("Event::Exit");
                }
                Event::CursorBlinkingChange => {
                    dbg!("Event::CursorBlinkingChange");
                }
                Event::Bell => {
                    dbg!("Event::Bell");
                }
                Event::TextAreaSizeRequest(_format) => {
                    dbg!("Event::TextAreaSizeRequest(<formatter>)");
                }
            }

            let mut events = self.events.lock().unwrap();
            events.push(event);
        }
    }

    #[test]
    fn test_term_with_pty_integration() {
        let window_size = WindowSize {
            num_lines: 24,
            num_cols: 80,
            cell_width: 1,
            cell_height: 1,
        };

        let shell = Shell::new("ls".to_string(), vec!["-la".to_string()]);
        let options = Options {
            shell: Some(shell),
            working_directory: None,
            drain_on_exit: true,
            env: HashMap::new(),
        };

        let pty = tty::new(&options, window_size.into(), 0).expect("Failed to create PTY");

        let event_listener = DebugEventListener::new();

        let config = Config::default();

        let size = Size {
            columns: Column(window_size.num_cols as usize),
            lines: Line(window_size.num_lines as i32),
        };
        let term = Arc::new(FairMutex::new(Term::new(
            config,
            &size,
            event_listener.clone(),
        )));
        let event_loop =
            EventLoop::new(term.clone(), event_listener.clone(), pty, true, false).unwrap();
        let handle = event_loop.spawn();
        handle.join().unwrap();

        let total_events = event_listener.get_event_count();

        println!("\n=== Event Summary ===");
        println!("Total events captured: {}", total_events);

        println!("\n=== Terminal Grid Content ===");
        let term_lock = term.lock();
        let grid = term_lock.grid();

        println!(
            "Grid dimensions: {} columns × {} lines",
            grid.columns(),
            grid.screen_lines()
        );
        println!("┌{}┐", "─".repeat(grid.columns()));

        for line_idx in 0..grid.screen_lines() {
            let line = Line(line_idx as i32);
            print!("│");

            let mut line_content = String::new();
            for col_idx in 0..grid.columns() {
                let col = Column(col_idx);
                let cell = &grid[line][col];
                let ch = cell.c;

                if ch == '\0' || ch.is_control() {
                    line_content.push(' ');
                } else {
                    line_content.push(ch);
                }
            }

            let trimmed = line_content.trim_end();
            print!("{}", trimmed);

            let padding = grid.columns() - trimmed.len();
            print!("{}", " ".repeat(padding));
            println!("│");
        }

        println!("└{}┘", "─".repeat(grid.columns()));
        println!("\nIntegrated Term + PTY test completed successfully!");
    }
}
