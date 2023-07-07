#[cfg(feature = "neovim")]
use std::ops::{Deref, DerefMut};
use std::{ops::Range, path::PathBuf};

#[cfg(feature = "neovim")]
use async_compat::Compat;
#[cfg(feature = "neovim")]
use async_trait::async_trait;
#[cfg(feature = "neovim")]
use gpui::keymap_matcher::Keystroke;

use language::Point;

#[cfg(feature = "neovim")]
use nvim_rs::{
    create::tokio::new_child_cmd, error::LoopError, Handler, Neovim, UiAttachOptions, Value,
};
#[cfg(feature = "neovim")]
use parking_lot::ReentrantMutex;
use serde::{Deserialize, Serialize};
#[cfg(feature = "neovim")]
use tokio::{
    process::{Child, ChildStdin, Command},
    task::JoinHandle,
};

use crate::state::Mode;
use collections::VecDeque;

// Neovim doesn't like to be started simultaneously from multiple threads. We use this lock
// to ensure we are only constructing one neovim connection at a time.
#[cfg(feature = "neovim")]
static NEOVIM_LOCK: ReentrantMutex<()> = ReentrantMutex::new(());

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum NeovimData {
    Put { state: String },
    Key(String),
    Get { state: String, mode: Option<Mode> },
}

pub struct NeovimConnection {
    data: VecDeque<NeovimData>,
    #[cfg(feature = "neovim")]
    test_case_id: String,
    #[cfg(feature = "neovim")]
    nvim: Neovim<nvim_rs::compat::tokio::Compat<ChildStdin>>,
    #[cfg(feature = "neovim")]
    _join_handle: JoinHandle<Result<(), Box<LoopError>>>,
    #[cfg(feature = "neovim")]
    _child: Child,
}

impl NeovimConnection {
    pub async fn new(test_case_id: String) -> Self {
        #[cfg(feature = "neovim")]
        let handler = NvimHandler {};
        #[cfg(feature = "neovim")]
        let (nvim, join_handle, child) = Compat::new(async {
            // Ensure we don't create neovim connections in parallel
            let _lock = NEOVIM_LOCK.lock();
            let (nvim, join_handle, child) = new_child_cmd(
                &mut Command::new("nvim").arg("--embed").arg("--clean"),
                handler,
            )
            .await
            .expect("Could not connect to neovim process");

            nvim.ui_attach(100, 100, &UiAttachOptions::default())
                .await
                .expect("Could not attach to ui");

            // Makes system act a little more like zed in terms of indentation
            nvim.set_option("smartindent", nvim_rs::Value::Boolean(true))
                .await
                .expect("Could not set smartindent on startup");

            (nvim, join_handle, child)
        })
        .await;

        Self {
            #[cfg(feature = "neovim")]
            data: Default::default(),
            #[cfg(not(feature = "neovim"))]
            data: Self::read_test_data(&test_case_id),
            #[cfg(feature = "neovim")]
            test_case_id,
            #[cfg(feature = "neovim")]
            nvim,
            #[cfg(feature = "neovim")]
            _join_handle: join_handle,
            #[cfg(feature = "neovim")]
            _child: child,
        }
    }

    // Sends a keystroke to the neovim process.
    #[cfg(feature = "neovim")]
    pub async fn send_keystroke(&mut self, keystroke_text: &str) {
        let keystroke = Keystroke::parse(keystroke_text).unwrap();
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

        self.data
            .push_back(NeovimData::Key(keystroke_text.to_string()));
        self.nvim
            .input(&key)
            .await
            .expect("Could not input keystroke");
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn send_keystroke(&mut self, keystroke_text: &str) {
        if matches!(self.data.front(), Some(NeovimData::Get { .. })) {
            self.data.pop_front();
        }
        assert_eq!(
            self.data.pop_front(),
            Some(NeovimData::Key(keystroke_text.to_string())),
            "operation does not match recorded script. re-record with --features=neovim"
        );
    }

    #[cfg(feature = "neovim")]
    pub async fn set_state(&mut self, marked_text: &str) {
        let (text, selection) = parse_state(&marked_text);

        let nvim_buffer = self
            .nvim
            .get_current_buf()
            .await
            .expect("Could not get neovim buffer");
        let lines = text
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

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

        if !selection.is_empty() {
            panic!("Setting neovim state with non empty selection not yet supported");
        }
        let cursor = selection.start;
        nvim_window
            .set_cursor((cursor.row as i64 + 1, cursor.column as i64))
            .await
            .expect("Could not set nvim cursor position");

        if let Some(NeovimData::Get { mode, state }) = self.data.back() {
            if *mode == Some(Mode::Normal) && *state == marked_text {
                return;
            }
        }
        self.data.push_back(NeovimData::Put {
            state: marked_text.to_string(),
        })
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn set_state(&mut self, marked_text: &str) {
        if let Some(NeovimData::Get { mode, state: text }) = self.data.front() {
            if *mode == Some(Mode::Normal) && *text == marked_text {
                return;
            }
            self.data.pop_front();
        }
        assert_eq!(
            self.data.pop_front(),
            Some(NeovimData::Put {
                state: marked_text.to_string()
            }),
            "operation does not match recorded script. re-record with --features=neovim"
        );
    }

    #[cfg(feature = "neovim")]
    pub async fn state(&mut self) -> (Option<Mode>, String, Range<Point>) {
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

        let cursor_row: u32 = self
            .nvim
            .command_output("echo line('.')")
            .await
            .unwrap()
            .parse::<u32>()
            .unwrap()
            - 1; // Neovim rows start at 1
        let cursor_col: u32 = self
            .nvim
            .command_output("echo col('.')")
            .await
            .unwrap()
            .parse::<u32>()
            .unwrap()
            - 1; // Neovim columns start at 1

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

        let (start, end) = if let Some(Mode::Visual { .. }) = mode {
            self.nvim
                .input("<escape>")
                .await
                .expect("Could not exit visual mode");
            let nvim_buffer = self
                .nvim
                .get_current_buf()
                .await
                .expect("Could not get neovim buffer");
            let (start_row, start_col) = nvim_buffer
                .get_mark("<")
                .await
                .expect("Could not get selection start");
            let (end_row, end_col) = nvim_buffer
                .get_mark(">")
                .await
                .expect("Could not get selection end");
            self.nvim
                .input("gv")
                .await
                .expect("Could not reselect visual selection");

            if cursor_row == start_row as u32 - 1 && cursor_col == start_col as u32 {
                (
                    Point::new(end_row as u32 - 1, end_col as u32),
                    Point::new(start_row as u32 - 1, start_col as u32),
                )
            } else {
                (
                    Point::new(start_row as u32 - 1, start_col as u32),
                    Point::new(end_row as u32 - 1, end_col as u32),
                )
            }
        } else {
            (
                Point::new(cursor_row, cursor_col),
                Point::new(cursor_row, cursor_col),
            )
        };

        let state = NeovimData::Get {
            mode,
            state: encode_range(&text, start..end),
        };

        if self.data.back() != Some(&state) {
            self.data.push_back(state.clone());
        }

        (mode, text, start..end)
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn state(&mut self) -> (Option<Mode>, String, Range<Point>) {
        if let Some(NeovimData::Get { state: text, mode }) = self.data.front() {
            let (text, range) = parse_state(text);
            (*mode, text, range)
        } else {
            panic!("operation does not match recorded script. re-record with --features=neovim");
        }
    }

    pub async fn selection(&mut self) -> Range<Point> {
        self.state().await.2
    }

    pub async fn mode(&mut self) -> Option<Mode> {
        self.state().await.0
    }

    pub async fn text(&mut self) -> String {
        self.state().await.1
    }

    fn test_data_path(test_case_id: &str) -> PathBuf {
        let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_path.push("test_data");
        data_path.push(format!("{}.json", test_case_id));
        data_path
    }

    #[cfg(not(feature = "neovim"))]
    fn read_test_data(test_case_id: &str) -> VecDeque<NeovimData> {
        let path = Self::test_data_path(test_case_id);
        let json = std::fs::read_to_string(path).expect(
            "Could not read test data. Is it generated? Try running test with '--features neovim'",
        );

        let mut result = VecDeque::new();
        for line in json.lines() {
            result.push_back(
                serde_json::from_str(line)
                    .expect("invalid test data. regenerate it with '--features neovim'"),
            );
        }
        result
    }

    #[cfg(feature = "neovim")]
    fn write_test_data(test_case_id: &str, data: &VecDeque<NeovimData>) {
        let path = Self::test_data_path(test_case_id);
        let mut json = Vec::new();
        for entry in data {
            serde_json::to_writer(&mut json, entry).unwrap();
            json.push(b'\n');
        }
        std::fs::create_dir_all(path.parent().unwrap())
            .expect("could not create test data directory");
        std::fs::write(path, json).expect("could not write out test data");
    }
}

#[cfg(feature = "neovim")]
impl Deref for NeovimConnection {
    type Target = Neovim<nvim_rs::compat::tokio::Compat<ChildStdin>>;

    fn deref(&self) -> &Self::Target {
        &self.nvim
    }
}

#[cfg(feature = "neovim")]
impl DerefMut for NeovimConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nvim
    }
}

#[cfg(feature = "neovim")]
impl Drop for NeovimConnection {
    fn drop(&mut self) {
        Self::write_test_data(&self.test_case_id, &self.data);
    }
}

#[cfg(feature = "neovim")]
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

fn parse_state(marked_text: &str) -> (String, Range<Point>) {
    let (text, ranges) = util::test::marked_text_ranges(marked_text, true);
    let byte_range = ranges[0].clone();
    let mut point_range = Point::zero()..Point::zero();
    let mut ix = 0;
    let mut position = Point::zero();
    for c in text.chars().chain(['\0']) {
        if ix == byte_range.start {
            point_range.start = position;
        }
        if ix == byte_range.end {
            point_range.end = position;
        }
        let len_utf8 = c.len_utf8();
        ix += len_utf8;
        if c == '\n' {
            position.row += 1;
            position.column = 0;
        } else {
            position.column += len_utf8 as u32;
        }
    }
    (text, point_range)
}

#[cfg(feature = "neovim")]
fn encode_range(text: &str, range: Range<Point>) -> String {
    let mut byte_range = 0..0;
    let mut ix = 0;
    let mut position = Point::zero();
    for c in text.chars().chain(['\0']) {
        if position == range.start {
            byte_range.start = ix;
        }
        if position == range.end {
            byte_range.end = ix;
        }
        let len_utf8 = c.len_utf8();
        ix += len_utf8;
        if c == '\n' {
            position.row += 1;
            position.column = 0;
        } else {
            position.column += len_utf8 as u32;
        }
    }
    util::test::generate_marked_text(text, &[byte_range], true)
}
