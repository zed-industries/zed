use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use collections::{HashMap, HashSet, VecDeque};
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
use serde::{Deserialize, Serialize};
#[cfg(feature = "neovim")]
use tokio::{
    process::{Child, ChildStdin, Command},
    task::JoinHandle,
};
use util::test::marked_text_offsets;

use crate::state::Mode;

use super::{NeovimBackedBindingTestContext, VimTestContext};

pub struct NeovimBackedTestContext<'a> {
    cx: VimTestContext<'a>,
    // Lookup for exempted assertions. Keyed by the insertion text, and with a value indicating which
    // bindings are exempted. If None, all bindings are ignored for that insertion text.
    exemptions: HashMap<String, Option<HashSet<String>>>,
    neovim: NeovimConnection,
}

impl<'a> NeovimBackedTestContext<'a> {
    pub async fn new(cx: &'a mut gpui::TestAppContext) -> NeovimBackedTestContext<'a> {
        let function_name = cx.function_name.clone();
        let cx = VimTestContext::new(cx, true).await;
        Self {
            cx,
            exemptions: Default::default(),
            neovim: NeovimConnection::new(function_name).await,
        }
    }

    pub fn add_initial_state_exemption(&mut self, initial_state: &str) {
        let initial_state = initial_state.to_string();
        // None represents all keybindings being exempted for that initial state
        self.exemptions.insert(initial_state, None);
    }

    pub fn add_keybinding_exemption<const COUNT: usize>(
        &mut self,
        keybinding: [&str; COUNT],
        initial_state: &str,
    ) {
        let initial_state = initial_state.to_string();
        let exempted_keybindings = self
            .exemptions
            .entry(initial_state)
            .or_insert(Some(Default::default()));

        if let Some(exempted_bindings) = exempted_keybindings.as_mut() {
            exempted_bindings.insert(format!("{keybinding:?}"));
        }
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

            self.neovim
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
                .neovim
                .get_current_buf()
                .await
                .expect("Could not get neovim buffer");
            let mut lines = self
                .buffer_text()
                .split('\n')
                .map(|line| line.to_string())
                .collect::<Vec<_>>();

            nvim_buffer
                .set_lines(0, -1, false, lines)
                .await
                .expect("Could not set nvim buffer text");

            self.neovim
                .input("<escape>")
                .await
                .expect("Could not send escape to nvim");
            self.neovim
                .input("<escape>")
                .await
                .expect("Could not send escape to nvim");

            let nvim_window = self
                .neovim
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
        assert_eq!(
            self.neovim.text().await,
            self.buffer_text(),
            "{}",
            self.assertion_context.context()
        );

        let zed_head = self.update_editor(|editor, cx| editor.selections.newest_display(cx).head());
        assert_eq!(
            self.neovim.head().await,
            zed_head,
            "{}",
            self.assertion_context.context()
        );

        if let Some(neovim_mode) = self.neovim.mode().await {
            assert_eq!(
                neovim_mode,
                self.mode(),
                "{}",
                self.assertion_context.context()
            );
        }
    }

    pub async fn assert_binding_matches<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        initial_state: &str,
    ) {
        if let Some(possible_exempted_keystrokes) = self.exemptions.get(initial_state) {
            match possible_exempted_keystrokes {
                Some(exempted_keystrokes) => {
                    if exempted_keystrokes.contains(&format!("{keystrokes:?}")) {
                        // This keystroke was exempted for this insertion text
                        return;
                    }
                }
                None => {
                    // All keystrokes for this insertion text are exempted
                    return;
                }
            }
        }

        let _keybinding_context_handle =
            self.add_assertion_context(format!("Key Binding Under Test: {:?}", keystrokes));
        let _initial_state_context_handle = self.add_assertion_context(format!(
            "Initial State: \"{}\"",
            initial_state.escape_debug().to_string()
        ));
        self.set_shared_state(initial_state).await;
        self.simulate_shared_keystrokes(keystrokes).await;
        self.assert_state_matches().await;
    }

    pub async fn assert_binding_matches_all<const COUNT: usize>(
        &mut self,
        keystrokes: [&str; COUNT],
        marked_positions: &str,
    ) {
        let (unmarked_text, cursor_offsets) = marked_text_offsets(marked_positions);

        for cursor_offset in cursor_offsets.iter() {
            let mut marked_text = unmarked_text.clone();
            marked_text.insert(*cursor_offset, 'ˇ');

            self.assert_binding_matches(keystrokes, &marked_text).await;
        }
    }

    pub fn binding<const COUNT: usize>(
        self,
        keystrokes: [&'static str; COUNT],
    ) -> NeovimBackedBindingTestContext<'a, COUNT> {
        NeovimBackedBindingTestContext::new(keystrokes, self)
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

#[derive(Serialize, Deserialize)]
pub enum NeovimData {
    Text(String),
    Head { row: u32, column: u32 },
    Mode(Option<Mode>),
}

struct NeovimConnection {
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
    async fn new(test_case_id: String) -> Self {
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

    #[cfg(feature = "neovim")]
    pub async fn text(&mut self) -> String {
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

        self.data.push_back(NeovimData::Text(text.clone()));

        text
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn text(&mut self) -> String {
        if let Some(NeovimData::Text(text)) = self.data.pop_front() {
            text
        } else {
            panic!("Invalid test data. Is test deterministic? Try running with '--features neovim' to regenerate");
        }
    }

    #[cfg(feature = "neovim")]
    pub async fn head(&mut self) -> DisplayPoint {
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

        self.data.push_back(NeovimData::Head {
            row: nvim_row,
            column: nvim_column,
        });

        DisplayPoint::new(nvim_row, nvim_column)
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn head(&mut self) -> DisplayPoint {
        if let Some(NeovimData::Head { row, column }) = self.data.pop_front() {
            DisplayPoint::new(row, column)
        } else {
            panic!("Invalid test data. Is test deterministic? Try running with '--features neovim' to regenerate");
        }
    }

    #[cfg(feature = "neovim")]
    pub async fn mode(&mut self) -> Option<Mode> {
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

        self.data.push_back(NeovimData::Mode(mode.clone()));

        mode
    }

    #[cfg(not(feature = "neovim"))]
    pub async fn mode(&mut self) -> Option<Mode> {
        if let Some(NeovimData::Mode(mode)) = self.data.pop_front() {
            mode
        } else {
            panic!("Invalid test data. Is test deterministic? Try running with '--features neovim' to regenerate");
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

        serde_json::from_str(&json)
            .expect("Test data corrupted. Try regenerating it with '--features neovim'")
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
        let path = Self::test_data_path(&self.test_case_id);
        std::fs::create_dir_all(path.parent().unwrap())
            .expect("Could not create test data directory");
        let json = serde_json::to_string(&self.data).expect("Could not serialize test data");
        std::fs::write(path, json).expect("Could not write out test data");
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

#[cfg(test)]
mod test {
    use gpui::TestAppContext;

    use crate::test_contexts::NeovimBackedTestContext;

    #[gpui::test]
    async fn neovim_backed_test_context_works(cx: &mut TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.assert_state_matches().await;
        cx.set_shared_state("This is a tesˇt").await;
        cx.assert_state_matches().await;
    }
}
