//! A crate for handling file encodings in the text editor.

use crate::selectors::encoding::Action;
use editor::Editor;
use encoding_rs::Encoding;
use gpui::{ClickEvent, Entity, Subscription, WeakEntity};
use language::Buffer;
use ui::{App, Button, ButtonCommon, Context, LabelSize, Render, Tooltip, Window, div};
use ui::{Clickable, ParentElement};
use util::ResultExt;
use workspace::{
    CloseActiveItem, ItemHandle, OpenOptions, StatusItemView, Workspace,
    with_active_or_new_workspace,
};
use zed_actions::encodings_ui::{ForceOpen, Toggle};

use crate::selectors::encoding::EncodingSelector;
use crate::selectors::save_or_reopen::EncodingSaveOrReopenSelector;

/// A status bar item that shows the current file encoding and allows changing it.
pub struct EncodingIndicator {
    pub encoding: Option<&'static Encoding>,
    pub workspace: WeakEntity<Workspace>,

    /// Subscription to observe changes in the active editor
    observe_editor: Option<Subscription>,

    /// Subscription to observe changes in the `encoding` field of the `Buffer` struct
    observe_buffer_encoding: Option<Subscription>,

    /// Whether to show the indicator or not, based on whether an editor is active
    show: bool,

    /// Whether to show `EncodingSaveOrReopenSelector`. It will be shown only when
    /// the current buffer is associated with a file.
    show_save_or_reopen_selector: bool,
}

pub mod selectors;

impl Render for EncodingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let status_element = div();
        let show_save_or_reopen_selector = self.show_save_or_reopen_selector;

        if !self.show {
            return status_element;
        }

        status_element.child(
            Button::new(
                "encoding",
                encoding_name(self.encoding.unwrap_or(encoding_rs::UTF_8)),
            )
            .label_size(LabelSize::Small)
            .tooltip(Tooltip::text("Select Encoding"))
            .on_click(cx.listener(move |indicator, _: &ClickEvent, window, cx| {
                if let Some(workspace) = indicator.workspace.upgrade() {
                    workspace.update(cx, move |workspace, cx| {
                        // Open the `EncodingSaveOrReopenSelector` if the buffer is associated with a file,
                        if show_save_or_reopen_selector {
                            EncodingSaveOrReopenSelector::toggle(workspace, window, cx)
                        }
                        // otherwise, open the `EncodingSelector` directly.
                        else {
                            let (_, buffer, _) = workspace
                                .active_item(cx)
                                .unwrap()
                                .act_as::<Editor>(cx)
                                .unwrap()
                                .read(cx)
                                .active_excerpt(cx)
                                .unwrap();

                            let weak_workspace = workspace.weak_handle();

                            if let Some(path) = buffer.read(cx).file() {
                                let path = path.clone().path().to_path_buf();
                                workspace.toggle_modal(window, cx, |window, cx| {
                                    let selector = EncodingSelector::new(
                                        window,
                                        cx,
                                        Action::Save,
                                        Some(buffer.downgrade()),
                                        weak_workspace,
                                        Some(path),
                                    );
                                    selector
                                });
                            }
                        }
                    })
                }
            })),
        )
    }
}

impl EncodingIndicator {
    pub fn new(
        encoding: Option<&'static Encoding>,
        workspace: WeakEntity<Workspace>,
        observe_editor: Option<Subscription>,
        observe_buffer_encoding: Option<Subscription>,
    ) -> EncodingIndicator {
        EncodingIndicator {
            encoding,
            workspace,
            observe_editor,
            show: false,
            observe_buffer_encoding,
            show_save_or_reopen_selector: false,
        }
    }

    /// Update the encoding when the active editor is switched.
    pub fn update_when_editor_is_switched(
        &mut self,
        editor: Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<EncodingIndicator>,
    ) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let encoding = buffer.read(cx).encoding.clone();
            self.encoding = Some(encoding.get());

            if let Some(_) = buffer.read(cx).file() {
                self.show_save_or_reopen_selector = true;
            } else {
                self.show_save_or_reopen_selector = false;
            }
        }

        cx.notify();
    }

    /// Update the encoding when the `encoding` field of the `Buffer` struct changes.
    pub fn update_when_buffer_encoding_changes(
        &mut self,
        buffer: Entity<Buffer>,
        _: &mut Window,
        cx: &mut Context<EncodingIndicator>,
    ) {
        let encoding = buffer.read(cx).encoding.clone();
        self.encoding = Some(encoding.get());
        cx.notify();
    }
}

impl StatusItemView for EncodingIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            Some(editor) => {
                self.observe_editor =
                    Some(cx.observe_in(&editor, window, Self::update_when_editor_is_switched));
                if let Some((_, buffer, _)) = &editor.read(cx).active_excerpt(cx) {
                    self.observe_buffer_encoding = Some(cx.observe_in(
                        buffer,
                        window,
                        Self::update_when_buffer_encoding_changes,
                    ));
                }
                self.update_when_editor_is_switched(editor, window, cx);
                self.show = true;
            }
            None => {
                self.encoding = None;
                self.observe_editor = None;
                self.show = false;
            }
        }
    }
}

/// Get a human-readable name for the given encoding.
pub fn encoding_name(encoding: &'static Encoding) -> String {
    let name = encoding.name();

    match name {
        "UTF-8" => "UTF-8",
        "UTF-16LE" => "UTF-16 LE",
        "UTF-16BE" => "UTF-16 BE",
        "windows-1252" => "Windows-1252",
        "windows-1251" => "Windows-1251",
        "windows-1250" => "Windows-1250",
        "ISO-8859-2" => "ISO 8859-2",
        "ISO-8859-3" => "ISO 8859-3",
        "ISO-8859-4" => "ISO 8859-4",
        "ISO-8859-5" => "ISO 8859-5",
        "ISO-8859-6" => "ISO 8859-6",
        "ISO-8859-7" => "ISO 8859-7",
        "ISO-8859-8" => "ISO 8859-8",
        "ISO-8859-13" => "ISO 8859-13",
        "ISO-8859-15" => "ISO 8859-15",
        "KOI8-R" => "KOI8-R",
        "KOI8-U" => "KOI8-U",
        "macintosh" => "MacRoman",
        "x-mac-cyrillic" => "Mac Cyrillic",
        "windows-874" => "Windows-874",
        "windows-1253" => "Windows-1253",
        "windows-1254" => "Windows-1254",
        "windows-1255" => "Windows-1255",
        "windows-1256" => "Windows-1256",
        "windows-1257" => "Windows-1257",
        "windows-1258" => "Windows-1258",
        "EUC-KR" => "Windows-949",
        "EUC-JP" => "EUC-JP",
        "ISO-2022-JP" => "ISO 2022-JP",
        "GBK" => "GBK",
        "gb18030" => "GB18030",
        "Big5" => "Big5",
        _ => name,
    }
    .to_string()
}

/// Get an encoding from its index in the predefined list.
/// If the index is out of range, UTF-8 is returned as a default.
pub fn encoding_from_index(index: usize) -> &'static Encoding {
    match index {
        0 => encoding_rs::UTF_8,
        1 => encoding_rs::UTF_16LE,
        2 => encoding_rs::UTF_16BE,
        3 => encoding_rs::WINDOWS_1252,
        4 => encoding_rs::WINDOWS_1251,
        5 => encoding_rs::WINDOWS_1250,
        6 => encoding_rs::ISO_8859_2,
        7 => encoding_rs::ISO_8859_3,
        8 => encoding_rs::ISO_8859_4,
        9 => encoding_rs::ISO_8859_5,
        10 => encoding_rs::ISO_8859_6,
        11 => encoding_rs::ISO_8859_7,
        12 => encoding_rs::ISO_8859_8,
        13 => encoding_rs::ISO_8859_13,
        14 => encoding_rs::ISO_8859_15,
        15 => encoding_rs::KOI8_R,
        16 => encoding_rs::KOI8_U,
        17 => encoding_rs::MACINTOSH,
        18 => encoding_rs::X_MAC_CYRILLIC,
        19 => encoding_rs::WINDOWS_874,
        20 => encoding_rs::WINDOWS_1253,
        21 => encoding_rs::WINDOWS_1254,
        22 => encoding_rs::WINDOWS_1255,
        23 => encoding_rs::WINDOWS_1256,
        24 => encoding_rs::WINDOWS_1257,
        25 => encoding_rs::WINDOWS_1258,
        26 => encoding_rs::EUC_KR,
        27 => encoding_rs::EUC_JP,
        28 => encoding_rs::ISO_2022_JP,
        29 => encoding_rs::GBK,
        30 => encoding_rs::GB18030,
        31 => encoding_rs::BIG5,
        _ => encoding_rs::UTF_8,
    }
}

/// Get an encoding from its name.
pub fn encoding_from_name(name: &str) -> &'static Encoding {
    match name {
        "UTF-8" => encoding_rs::UTF_8,
        "UTF-16 LE" => encoding_rs::UTF_16LE,
        "UTF-16 BE" => encoding_rs::UTF_16BE,
        "Windows-1252" => encoding_rs::WINDOWS_1252,
        "Windows-1251" => encoding_rs::WINDOWS_1251,
        "Windows-1250" => encoding_rs::WINDOWS_1250,
        "ISO 8859-2" => encoding_rs::ISO_8859_2,
        "ISO 8859-3" => encoding_rs::ISO_8859_3,
        "ISO 8859-4" => encoding_rs::ISO_8859_4,
        "ISO 8859-5" => encoding_rs::ISO_8859_5,
        "ISO 8859-6" => encoding_rs::ISO_8859_6,
        "ISO 8859-7" => encoding_rs::ISO_8859_7,
        "ISO 8859-8" => encoding_rs::ISO_8859_8,
        "ISO 8859-13" => encoding_rs::ISO_8859_13,
        "ISO 8859-15" => encoding_rs::ISO_8859_15,
        "KOI8-R" => encoding_rs::KOI8_R,
        "KOI8-U" => encoding_rs::KOI8_U,
        "MacRoman" => encoding_rs::MACINTOSH,
        "Mac Cyrillic" => encoding_rs::X_MAC_CYRILLIC,
        "Windows-874" => encoding_rs::WINDOWS_874,
        "Windows-1253" => encoding_rs::WINDOWS_1253,
        "Windows-1254" => encoding_rs::WINDOWS_1254,
        "Windows-1255" => encoding_rs::WINDOWS_1255,
        "Windows-1256" => encoding_rs::WINDOWS_1256,
        "Windows-1257" => encoding_rs::WINDOWS_1257,
        "Windows-1258" => encoding_rs::WINDOWS_1258,
        "Windows-949" => encoding_rs::EUC_KR,
        "EUC-JP" => encoding_rs::EUC_JP,
        "ISO 2022-JP" => encoding_rs::ISO_2022_JP,
        "GBK" => encoding_rs::GBK,
        "GB18030" => encoding_rs::GB18030,
        "Big5" => encoding_rs::BIG5,
        _ => encoding_rs::UTF_8, // Default to UTF-8 for unknown names
    }
}

pub fn init(cx: &mut App) {
    cx.on_action(|action: &Toggle, cx: &mut App| {
        let Toggle(path) = action.clone();
        let path = path.to_path_buf();

        with_active_or_new_workspace(cx, |workspace, window, cx| {
            let weak_workspace = workspace.weak_handle();
            workspace.toggle_modal(window, cx, |window, cx| {
                EncodingSelector::new(window, cx, Action::Reopen, None, weak_workspace, Some(path))
            });
        });
    });

    cx.on_action(|action: &ForceOpen, cx: &mut App| {
        let ForceOpen(path) = action.clone();
        let path = path.to_path_buf();

        with_active_or_new_workspace(cx, |workspace, window, cx| {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.close_active_item(&CloseActiveItem::default(), window, cx)
                    .detach();
            });

            {
                let force = workspace.encoding_options.force.get_mut();

                *force = true;
            }

            let open_task = workspace.open_abs_path(path, OpenOptions::default(), window, cx);
            let weak_workspace = workspace.weak_handle();

            cx.spawn(async move |_, cx| {
                let workspace = weak_workspace.upgrade().unwrap();
                open_task.await.log_err();
                workspace
                    .update(cx, |workspace: &mut Workspace, _| {
                        *workspace.encoding_options.force.get_mut() = false;
                    })
                    .log_err();
            })
            .detach();
        });
    });
}
