use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use editor::DisplayPoint;
use gpui::keymap::Keystroke;

#[cfg(feature = "neovim")]
use async_compat::Compat;
#[cfg(feature = "neovim")]
use async_trait::async_trait;
#[cfg(feature = "neovim")]
use nvim_rs::{
    create::tokio::new_child_cmd, error::LoopError, Handler, Neovim, UiAttachOptions, Value,
};
#[cfg(feature = "neovim")]
use tokio::{
    process::{Child, ChildStdin, Command},
    task::JoinHandle,
};

use crate::state::Mode;

use super::{NeovimBackedBindingTestContext, VimTestContext};

pub struct NeovimBackedTestContext<'a> {
    cx: VimTestContext<'a>,
    test_case_id: &'static str,
    data_counter: usize,
    #[cfg(feature = "neovim")]
    nvim: Neovim<nvim_rs::compat::tokio::Compat<ChildStdin>>,
    #[cfg(feature = "neovim")]
    _join_handle: JoinHandle<Result<(), Box<LoopError>>>,
    #[cfg(feature = "neovim")]
    _child: Child,
}

impl<'a> NeovimBackedTestContext<'a> {
    pub async fn new(
        test_case_id: &'static str,
        cx: &'a mut gpui::TestAppContext,
    ) -> NeovimBackedTestContext<'a> {
        let cx = VimTestContext::new(cx, true).await;

        #[cfg(feature = "neovim")]
        let handler = NvimHandler {};
        #[cfg(feature = "neovim")]
        let (nvim, join_handle, child) = Compat::new(async {
            let (nvim, join_handle, child) = new_child_cmd(
                &mut Command::new("nvim").arg("--embed").arg("--clean"),
                handler,
            )
            .await
            .expect("Could not connect to neovim process");

            nvim.ui_attach(100, 100, &UiAttachOptions::default())
                .await
                .expect("Could not attach to ui");

            (nvim, join_handle, child)
        })
        .await;

        let result = Self {
            cx,
            test_case_id,
            data_counter: 0,
            #[cfg(feature = "neovim")]
            nvim,
            #[cfg(feature = "neovim")]
            _join_handle: join_handle,
            #[cfg(feature = "neovim")]
            _child: child,
        };

        #[cfg(feature = "neovim")]
        {
            result.clear_test_data()
        }

        result
    }

    pub async fn simulate_shared_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();

        #[cfg(feature = "neovim")]
        {
            let special = keystroke.shift
                || keystroke.ctrl
                || keystroke.alt
                || keystroke.cmd
                || keystroke.key.len() > 1;
            let start = if special { "<" } else { "" };
            let shift = if keystroke.shift { "S-" } else { "" };
            let ctrl = if keystroke.ctrl { "C-" } else { "" };
            let alt = if keystroke.alt { "M-" } else { "" };
            let cmd = if keystroke.cmd { "D-" } else { "" };
            let end = if special { ">" } else { "" };

            let key = format!("{start}{shift}{ctrl}{alt}{cmd}{}{end}", keystroke.key);

            self.nvim
                .input(&key)
                .await
                .expect("Could not input keystroke");
        }

        let window_id = self.window_id;
        self.cx.dispatch_keystroke(window_id, keystroke, false);
    }

    pub async fn simulate_shared_keystrokes<const COUNT: usize>(
        &mut self,
        keystroke_texts: [&str; COUNT],
    ) {
        for keystroke_text in keystroke_texts.into_iter() {
            self.simulate_shared_keystroke(keystroke_text).await;
        }
    }

    pub async fn set_shared_state(&mut self, marked_text: &str) {
        self.set_state(marked_text, Mode::Normal);

        #[cfg(feature = "neovim")]
        {
            let cursor_point =
                self.editor(|editor, cx| editor.selections.newest::<language::Point>(cx));
            let nvim_buffer = self
                .nvim
                .get_current_buf()
                .await
                .expect("Could not get neovim buffer");
            let mut lines = self
                .buffer_text()
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<_>>();

            if lines.len() > 1 {
                // Add final newline which is missing from buffer_text
                lines.push("".to_string());
            }

            nvim_buffer
                .set_lines(0, -1, false, lines)
                .await
                .expect("Could not set nvim buffer text");

            self.nvim
                .input("<escape>")
                .await
                .expect("Could not send escape to nvim");
            self.nvim
                .input("<escape>")
                .await
                .expect("Could not send escape to nvim");

            let nvim_window = self
                .nvim
                .get_current_win()
                .await
                .expect("Could not get neovim window");
            nvim_window
                .set_cursor((
                    cursor_point.head().row as i64 + 1,
                    cursor_point.head().column as i64,
                ))
                .await
                .expect("Could not set nvim cursor position");
        }
    }

    pub async fn assert_state_matches(&mut self) {
        assert_eq!(self.neovim_text().await, self.buffer_text());

        let zed_head = self.update_editor(|editor, cx| editor.selections.newest_display(cx).head());
        assert_eq!(self.neovim_head().await, zed_head);

        if let Some(neovim_mode) = self.neovim_mode().await {
            assert_eq!(neovim_mode, self.mode());
        }
    }

    #[cfg(feature = "neovim")]
    pub async fn neovim_text(&mut self) -> String {
        let nvim_buffer = self
            .nvim
            .get_current_buf()
            .await
            .expect("Could not get neovim buffer");
        let text = nvim_buffer
            .get_lines(0, -1, false)
            .await
            .expect("Could not get buffer text")
            .join("\n");

        self.write_test_data(text.clone(), "text");
        text
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn neovim_text(&mut self) -> String {
        self.read_test_data("text")
    }

    #[cfg(feature = "neovim")]
    pub async fn neovim_head(&mut self) -> DisplayPoint {
        let nvim_row: u32 = self
            .nvim
            .command_output("echo line('.')")
            .await
            .unwrap()
            .parse::<u32>()
            .unwrap()
            - 1; // Neovim rows start at 1
        let nvim_column: u32 = self
            .nvim
            .command_output("echo col('.')")
            .await
            .unwrap()
            .parse::<u32>()
            .unwrap()
            - 1; // Neovim columns start at 1

        let serialized = format!("{},{}", nvim_row.to_string(), nvim_column.to_string());
        self.write_test_data(serialized, "head");

        DisplayPoint::new(nvim_row, nvim_column)
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn neovim_head(&mut self) -> DisplayPoint {
        let serialized = self.read_test_data("head");
        let mut components = serialized.split(',');
        let nvim_row = components.next().unwrap().parse::<u32>().unwrap();
        let nvim_column = components.next().unwrap().parse::<u32>().unwrap();

        DisplayPoint::new(nvim_row, nvim_column)
    }

    #[cfg(feature = "neovim")]
    pub async fn neovim_mode(&mut self) -> Option<Mode> {
        let nvim_mode_text = self
            .nvim
            .get_mode()
            .await
            .expect("Could not get mode")
            .into_iter()
            .find_map(|(key, value)| {
                if key.as_str() == Some("mode") {
                    Some(value.as_str().unwrap().to_owned())
                } else {
                    None
                }
            })
            .expect("Could not find mode value");

        let mode = match nvim_mode_text.as_ref() {
            "i" => Some(Mode::Insert),
            "n" => Some(Mode::Normal),
            "v" => Some(Mode::Visual { line: false }),
            "V" => Some(Mode::Visual { line: true }),
            _ => None,
        };

        let serialized = serde_json::to_string(&mode).expect("Could not serialize mode");

        self.write_test_data(serialized, "mode");

        mode
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn neovim_mode(&mut self) -> Option<Mode> {
        let serialized = self.read_test_data("mode");
        serde_json::from_str(&serialized).expect("Could not deserialize test data")
    }

    fn test_data_directory(&self) -> PathBuf {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("test_data");
        data_path.push(self.test_case_id);
        data_path
    }

    fn next_data_path(&mut self, kind: &str) -> PathBuf {
        let mut data_path = self.test_data_directory();
        data_path.push(format!("{}{}.txt", self.data_counter, kind));
        self.data_counter += 1;
        data_path
    }

    #[cfg(not(feature = "neovim"))]
    fn read_test_data(&mut self, kind: &str) -> String {
        let path = self.next_data_path(kind);
        std::fs::read_to_string(path).expect(
            "Could not read test data. Is it generated? Try running test with '--features neovim'",
        )
    }

    #[cfg(feature = "neovim")]
    fn write_test_data(&mut self, data: String, kind: &str) {
        let path = self.next_data_path(kind);
        std::fs::create_dir_all(path.parent().unwrap())
            .expect("Could not create test data directory");
        std::fs::write(path, data).expect("Could not write out test data");
    }

    #[cfg(feature = "neovim")]
    fn clear_test_data(&self) {
        // If the path does not exist, no biggy, we will create it
        std::fs::remove_dir_all(self.test_data_directory()).ok();
    }

    pub async fn assert_binding_matches<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
    ) {
        dbg!(keystrokes, initial_state);
        self.set_shared_state(initial_state).await;
        self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_state_matches().await;
    }

    pub fn binding<const COUNT: usize>(
        self,
        keystrokes: [&'static str; COUNT],
    ) -> NeovimBackedBindingTestContext<'a, COUNT> {
        NeovimBackedBindingTestContext::new(keystrokes, self)
    }
}

#[derive(Clone)]
struct NvimHandler {}

#[cfg(feature = "neovim")]
#[async_trait]
impl Handler for NvimHandler {
    type Writer = nvim_rs::compat::tokio::Compat<ChildStdin>;

    async fn handle_request(
        &self,
        _event_name: String,
        _arguments: Vec<Value>,
        _neovim: Neovim<Self::Writer>,
    ) -> Result<Value, Value> {
        unimplemented!();
    }

    async fn handle_notify(
        &self,
        _event_name: String,
        _arguments: Vec<Value>,
        _neovim: Neovim<Self::Writer>,
    ) {
    }
}

impl<'a> Deref for NeovimBackedTestContext<'a> {
    type Target = VimTestContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.cx
    }
}

impl<'a> DerefMut for NeovimBackedTestContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cx
    }
}
