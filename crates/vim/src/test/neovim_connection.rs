use std::path::PathBuf;
#[cfg(feature = "neovim")]
use std::{
    cmp,
    ops::{Deref, DerefMut, Range},
};

#[cfg(feature = "neovim")]
use async_compat::Compat;
#[cfg(feature = "neovim")]
use async_trait::async_trait;
#[cfg(feature = "neovim")]
use gpui::Keystroke;

#[cfg(feature = "neovim")]
use language::Point;

#[cfg(feature = "neovim")]
use nvim_rs::{
    Handler, Neovim, UiAttachOptions, Value, create::tokio::new_child_cmd, error::LoopError,
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
    Get { state: String, mode: Mode },
    ReadRegister { name: char, value: String },
    Exec { command: String },
    SetOption { value: String },
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
                Command::new("nvim")
                    .arg("--embed")
                    .arg("--clean")
                    // disable swap (otherwise after about 1000 test runs you run out of swap file names)
                    .arg("-n")
                    // disable writing files (just in case)
                    .arg("-m"),
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
        let mut keystroke = Keystroke::parse(keystroke_text).unwrap();

        if keystroke.key == "<" {
            keystroke.key = "lt".to_string()
        }

        let special = keystroke.modifiers.shift
            || keystroke.modifiers.control
            || keystroke.modifiers.alt
            || keystroke.modifiers.platform
            || keystroke.key.len() > 1;
        let start = if special { "<" } else { "" };
        let shift = if keystroke.modifiers.shift { "S-" } else { "" };
        let ctrl = if keystroke.modifiers.control {
            "C-"
        } else {
            ""
        };
        let alt = if keystroke.modifiers.alt { "M-" } else { "" };
        let cmd = if keystroke.modifiers.platform {
            "D-"
        } else {
            ""
        };
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
        let (text, selections) = parse_state(marked_text);

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

        if selections.len() != 1 {
            panic!("must have one selection");
        }
        let selection = &selections[0];

        let cursor = selection.start;
        nvim_window
            .set_cursor((cursor.row as i64 + 1, cursor.column as i64))
            .await
            .expect("Could not set nvim cursor position");

        if !selection.is_empty() {
            self.nvim
                .input("v")
                .await
                .expect("could not enter visual mode");

            let cursor = selection.end;
            nvim_window
                .set_cursor((cursor.row as i64 + 1, cursor.column as i64))
                .await
                .expect("Could not set nvim cursor position");
        }

        if let Some(NeovimData::Get { mode, state }) = self.data.back()
            && *mode == Mode::Normal
            && *state == marked_text
        {
            return;
        }
        self.data.push_back(NeovimData::Put {
            state: marked_text.to_string(),
        })
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn set_state(&mut self, marked_text: &str) {
        if let Some(NeovimData::Get { mode, state: text }) = self.data.front() {
            if *mode == Mode::Normal && *text == marked_text {
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
    pub async fn set_option(&mut self, value: &str) {
        self.nvim
            .command_output(format!("set {}", value).as_str())
            .await
            .unwrap();

        self.data.push_back(NeovimData::SetOption {
            value: value.to_string(),
        })
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn set_option(&mut self, value: &str) {
        if let Some(NeovimData::Get { .. }) = self.data.front() {
            self.data.pop_front();
        };
        assert_eq!(
            self.data.pop_front(),
            Some(NeovimData::SetOption {
                value: value.to_string(),
            }),
            "operation does not match recorded script. re-record with --features=neovim"
        );
    }

    #[cfg(feature = "neovim")]
    pub async fn exec(&mut self, value: &str) {
        self.nvim.command_output(value).await.unwrap();

        self.data.push_back(NeovimData::Exec {
            command: value.to_string(),
        })
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn exec(&mut self, value: &str) {
        if let Some(NeovimData::Get { .. }) = self.data.front() {
            self.data.pop_front();
        };
        assert_eq!(
            self.data.pop_front(),
            Some(NeovimData::Exec {
                command: value.to_string(),
            }),
            "operation does not match recorded script. re-record with --features=neovim"
        );
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn read_register(&mut self, register: char) -> String {
        if let Some(NeovimData::Get { .. }) = self.data.front() {
            self.data.pop_front();
        };
        if let Some(NeovimData::ReadRegister { name, value }) = self.data.pop_front()
            && name == register
        {
            return value;
        }

        panic!("operation does not match recorded script. re-record with --features=neovim")
    }

    #[cfg(feature = "neovim")]
    pub async fn read_register(&mut self, name: char) -> String {
        let value = self
            .nvim
            .command_output(format!("echo getreg('{}')", name).as_str())
            .await
            .unwrap();

        self.data.push_back(NeovimData::ReadRegister {
            name,
            value: value.clone(),
        });

        value
    }

    #[cfg(feature = "neovim")]
    async fn read_position(&mut self, cmd: &str) -> u32 {
        self.nvim
            .command_output(cmd)
            .await
            .unwrap()
            .parse::<u32>()
            .unwrap()
    }

    #[cfg(feature = "neovim")]
    pub async fn state(&mut self) -> (Mode, String) {
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

        // nvim columns are 1-based, so -1.
        let mut cursor_row = self.read_position("echo line('.')").await - 1;
        let mut cursor_col = self.read_position("echo col('.')").await - 1;
        let mut selection_row = self.read_position("echo line('v')").await - 1;
        let mut selection_col = self.read_position("echo col('v')").await - 1;
        let total_rows = self.read_position("echo line('$')").await - 1;

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
            "i" => Mode::Insert,
            "n" => Mode::Normal,
            "v" => Mode::Visual,
            "V" => Mode::VisualLine,
            "R" => Mode::Replace,
            "\x16" => Mode::VisualBlock,
            _ => panic!("unexpected vim mode: {nvim_mode_text}"),
        };

        let mut selections = Vec::new();
        // Vim uses the index of the first and last character in the selection
        // Zed uses the index of the positions between the characters, so we need
        // to add one to the end in visual mode.
        match mode {
            Mode::VisualBlock if selection_row != cursor_row => {
                // in zed we fake a block selection by using multiple cursors (one per line)
                // this code emulates that.
                // to deal with casees where the selection is not perfectly rectangular we extract
                // the content of the selection via the "a register to get the shape correctly.
                self.nvim.input("\"aygv").await.unwrap();
                let content = self.nvim.command_output("echo getreg('a')").await.unwrap();
                let lines = content.split('\n').collect::<Vec<_>>();
                let top = cmp::min(selection_row, cursor_row);
                let left = cmp::min(selection_col, cursor_col);
                for row in top..=cmp::max(selection_row, cursor_row) {
                    let content = if row - top >= lines.len() as u32 {
                        ""
                    } else {
                        lines[(row - top) as usize]
                    };
                    let line_len = self
                        .read_position(format!("echo strlen(getline({}))", row + 1).as_str())
                        .await;

                    if left > line_len {
                        continue;
                    }

                    let start = Point::new(row, left);
                    let end = Point::new(row, left + content.len() as u32);
                    if cursor_col >= selection_col {
                        selections.push(start..end)
                    } else {
                        selections.push(end..start)
                    }
                }
            }
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                if (selection_row, selection_col) > (cursor_row, cursor_col) {
                    let selection_line_length =
                        self.read_position("echo strlen(getline(line('v')))").await;
                    if selection_line_length > selection_col {
                        selection_col += 1;
                    } else if selection_row < total_rows {
                        selection_col = 0;
                        selection_row += 1;
                    }
                } else {
                    let cursor_line_length =
                        self.read_position("echo strlen(getline(line('.')))").await;
                    if cursor_line_length > cursor_col {
                        cursor_col += 1;
                    } else if cursor_row < total_rows {
                        cursor_col = 0;
                        cursor_row += 1;
                    }
                }
                selections.push(
                    Point::new(selection_row, selection_col)..Point::new(cursor_row, cursor_col),
                )
            }
            Mode::Insert | Mode::Normal | Mode::Replace => selections
                .push(Point::new(selection_row, selection_col)..Point::new(cursor_row, cursor_col)),
            Mode::HelixNormal | Mode::HelixSelect => unreachable!(),
        }

        let ranges = encode_ranges(&text, &selections);
        let state = NeovimData::Get {
            mode,
            state: ranges.clone(),
        };

        if self.data.back() != Some(&state) {
            self.data.push_back(state);
        }

        (mode, ranges)
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn state(&mut self) -> (Mode, String) {
        if let Some(NeovimData::Get { state: raw, mode }) = self.data.front() {
            (*mode, raw.to_string())
        } else {
            panic!("operation does not match recorded script. re-record with --features=neovim");
        }
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

#[cfg(feature = "neovim")]
fn parse_state(marked_text: &str) -> (String, Vec<Range<Point>>) {
    let (text, ranges) = util::test::marked_text_ranges(marked_text, true);
    let point_ranges = ranges
        .into_iter()
        .map(|byte_range| {
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
            point_range
        })
        .collect::<Vec<_>>();
    (text, point_ranges)
}

#[cfg(feature = "neovim")]
fn encode_ranges(text: &str, point_ranges: &Vec<Range<Point>>) -> String {
    let byte_ranges = point_ranges
        .iter()
        .map(|range| {
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
            byte_range
        })
        .collect::<Vec<_>>();
    util::test::generate_marked_text(text, &byte_ranges[..], true)
}
