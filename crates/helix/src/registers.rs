use anyhow::{Result, anyhow};
use editor::{ClipboardSelection, Editor, ToOffset as _};
use gpui::{App, Global, UpdateGlobal as _};
use modal_editor::{Register, Registers};
use settings::{Settings as _, UseSystemClipboard};

use crate::HelixSettings;

/// The register that yanks and pastes use when no other is selected.
pub const DEFAULT_REGISTER: char = '"';

/// Helix registers.
///
/// A register holds one value per selection. Some registers are special:
/// `_` discards writes and reads as empty, `#` reads the selection indices,
/// `.` the selection contents and `%` the file path, `+` is the system
/// clipboard and `*` the primary selection. [`DEFAULT_REGISTER`] uses the
/// system clipboard as configured by `vim.use_system_clipboard`.
#[derive(Default)]
pub struct HelixRegisters {
    registers: Registers,
}

impl Global for HelixRegisters {}

impl HelixRegisters {
    /// Reads the values of register `name`, one per selection.
    pub fn read(name: char, editor: &Editor, cx: &App) -> Option<Vec<String>> {
        match name {
            '_' => Some(Vec::new()),
            '#' => Some(
                (1..=editor.selections.count())
                    .map(|index| index.to_string())
                    .collect(),
            ),
            '.' => Some(selection_contents(editor, cx)),
            '%' => file_path(editor, cx).map(|path| vec![path]),
            '+' => cx
                .read_from_clipboard()
                .map(|item| values_from_register(&item.into())),
            '*' => {
                modal_editor::read_from_primary(cx).map(|register| values_from_register(&register))
            }
            _ => {
                let registers = &cx.try_global::<Self>()?.registers;
                if name == DEFAULT_REGISTER {
                    let read_clipboard = match HelixSettings::get_global(cx).use_system_clipboard {
                        UseSystemClipboard::Always => true,
                        UseSystemClipboard::OnYank => registers.system_clipboard_is_newer(cx),
                        UseSystemClipboard::Never => false,
                    };
                    if read_clipboard {
                        return cx
                            .read_from_clipboard()
                            .map(|item| values_from_register(&item.into()));
                    }
                }
                registers.get(name).map(values_from_register)
            }
        }
    }

    /// Writes `values`, one per selection, to register `name`. `is_yank`
    /// tells whether the values come from a yank rather than a deletion.
    pub fn write(name: char, values: Vec<String>, is_yank: bool, cx: &mut App) -> Result<()> {
        match name {
            '_' => Ok(()),
            '#' | '.' | '%' => Err(anyhow!("Register {name} does not support writing")),
            '+' => {
                Self::update_registers(cx, |registers, cx| {
                    registers.write_to_clipboard(register_from_values(&values), cx)
                });
                Ok(())
            }
            '*' => {
                modal_editor::write_to_primary(register_from_values(&values), cx);
                Ok(())
            }
            _ => {
                let register = register_from_values(&values);
                let use_system_clipboard = HelixSettings::get_global(cx).use_system_clipboard;
                Self::update_registers(cx, |registers, cx| {
                    if name == DEFAULT_REGISTER {
                        let write_clipboard = match use_system_clipboard {
                            UseSystemClipboard::Always => true,
                            UseSystemClipboard::OnYank => is_yank,
                            UseSystemClipboard::Never => false,
                        };
                        if write_clipboard {
                            registers.write_to_clipboard(register.clone(), cx);
                        } else {
                            registers.remember_clipboard(cx);
                        }
                    }
                    registers.insert(name, register);
                });
                Ok(())
            }
        }
    }

    fn update_registers(cx: &mut App, update: impl FnOnce(&mut Registers, &mut App)) {
        cx.default_global::<Self>();
        Self::update_global(cx, |helix_registers, cx| {
            update(&mut helix_registers.registers, cx)
        });
    }
}

fn selection_contents(editor: &Editor, cx: &App) -> Vec<String> {
    let snapshot = editor.buffer().read(cx).snapshot(cx);
    editor
        .selections
        .disjoint_anchors_arc()
        .iter()
        .map(|selection| {
            let range = selection.start.to_offset(&snapshot)..selection.end.to_offset(&snapshot);
            snapshot.text_for_range(range).collect()
        })
        .collect()
}

fn file_path(editor: &Editor, cx: &App) -> Option<String> {
    let multibuffer = editor.buffer().read(cx);
    let snapshot = multibuffer.snapshot(cx);
    let head = editor.selections.newest_anchor().head();
    let (text_anchor, _) = snapshot.anchor_to_buffer_anchor(head)?;
    let buffer = multibuffer.buffer(text_anchor.buffer_id)?;
    let file = buffer.read(cx).file()?;
    Some(file.path().display(file.path_style(cx)).into_owned())
}

/// Stores multiple values the way `editor::Copy` stores multiple selections,
/// so they paste one per cursor elsewhere in Zed too.
fn register_from_values(values: &[String]) -> Register {
    if values.len() <= 1 {
        return Register::from(values.first().cloned().unwrap_or_default());
    }
    Register {
        text: values.join("\n").into(),
        clipboard_selections: Some(
            values
                .iter()
                .map(|value| ClipboardSelection {
                    len: value.len(),
                    is_entire_line: false,
                    first_line_indent: 0,
                    file_path: None,
                    line_range: None,
                })
                .collect(),
        ),
    }
}

fn values_from_register(register: &Register) -> Vec<String> {
    let text = register.text.as_ref();
    let Some(clipboard_selections) = register
        .clipboard_selections
        .as_ref()
        .filter(|selections| selections.len() > 1)
    else {
        return vec![text.to_string()];
    };
    let mut values = Vec::with_capacity(clipboard_selections.len());
    let mut start = 0;
    for clipboard_selection in clipboard_selections {
        let end = (start + clipboard_selection.len).min(text.len());
        values.push(text.get(start..end).unwrap_or_default().to_string());
        start = end;
        // `editor::Copy` separates selections with a newline unless the
        // selection already ends with one.
        if !clipboard_selection.is_entire_line
            && text.get(start..).is_some_and(|rest| rest.starts_with('\n'))
        {
            start += 1;
        }
    }
    values
}
