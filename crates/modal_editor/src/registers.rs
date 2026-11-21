use collections::HashMap;
use editor::ClipboardSelection;
use gpui::{App, ClipboardEntry, ClipboardItem, SharedString};

/// Text stored in a register. Multiple selections are concatenated in `text`
/// and split back using `clipboard_selections`, the same format `editor::Copy`
/// writes to the system clipboard.
#[derive(Default, Clone, Debug)]
pub struct Register {
    pub text: SharedString,
    pub clipboard_selections: Option<Vec<ClipboardSelection>>,
}

impl From<Register> for ClipboardItem {
    fn from(register: Register) -> Self {
        if let Some(clipboard_selections) = register.clipboard_selections {
            ClipboardItem::new_string_with_json_metadata(register.text.into(), clipboard_selections)
        } else {
            ClipboardItem::new_string(register.text.into())
        }
    }
}

impl From<ClipboardItem> for Register {
    fn from(item: ClipboardItem) -> Self {
        match item.entries().iter().find_map(|entry| match entry {
            ClipboardEntry::String(value) => Some(value),
            _ => None,
        }) {
            Some(value) => Register {
                text: value.text().to_owned().into(),
                clipboard_selections: value.metadata_json::<Vec<ClipboardSelection>>(),
            },
            None => Register::default(),
        }
    }
}

impl From<String> for Register {
    fn from(text: String) -> Self {
        Register {
            text: text.into(),
            clipboard_selections: None,
        }
    }
}

/// Register storage. Which registers a yank or delete writes to is decided by
/// each modal editing mode.
#[derive(Default)]
pub struct Registers {
    registers: HashMap<char, Register>,
    last_yank: Option<SharedString>,
}

impl Registers {
    pub fn get(&self, name: char) -> Option<&Register> {
        self.registers.get(&name)
    }

    pub fn insert(&mut self, name: char, register: Register) -> Option<Register> {
        self.registers.insert(name, register)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&char, &Register)> {
        self.registers.iter()
    }

    pub fn write_to_clipboard(&mut self, content: Register, cx: &mut App) {
        self.last_yank.replace(content.text.clone());
        cx.write_to_clipboard(content.into());
    }

    /// Records the current clipboard text as known, so that
    /// `system_clipboard_is_newer` only reports later changes made outside the editor.
    pub fn remember_clipboard(&mut self, cx: &App) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.last_yank.replace(text.into());
        }
    }

    pub fn system_clipboard_is_newer(&self, cx: &App) -> bool {
        cx.read_from_clipboard().is_some_and(|item| {
            match (item.text().as_deref(), &self.last_yank) {
                (Some(new), Some(last)) => last.as_ref() != new,
                (Some(_), None) => true,
                (None, _) => false,
            }
        })
    }
}

/// Writes to the primary selection where the platform has one, and to the
/// system clipboard otherwise.
pub fn write_to_primary(content: Register, cx: &mut App) {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    cx.write_to_primary(content.into());
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    cx.write_to_clipboard(content.into());
}

/// Reads the primary selection where the platform has one, and the system
/// clipboard otherwise.
pub fn read_from_primary(cx: &App) -> Option<Register> {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        cx.read_from_primary().map(|item| item.into())
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        cx.read_from_clipboard().map(|item| item.into())
    }
}
