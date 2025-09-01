///! A crate for handling file encodings in the text editor.
use editor::{Editor, EditorSettings};
use encoding_rs::Encoding;
use gpui::{ClickEvent, Entity, Subscription, WeakEntity};
use settings::Settings;
use ui::{Button, ButtonCommon, Context, LabelSize, Render, Tooltip, Window, div};
use ui::{Clickable, ParentElement};
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::selectors::save_or_reopen::EncodingSaveOrReopenSelector;

/// A status bar item that shows the current file encoding and allows changing it.
pub struct EncodingIndicator {
    pub encoding: Option<&'static Encoding>,
    pub workspace: WeakEntity<Workspace>,
    observe: Option<Subscription>, // Subscription to observe changes in the active editor
    show: bool, // Whether to show the indicator or not, based on whether an editor is active
}

pub mod selectors;

impl Render for EncodingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let status_element = div();

        if (EditorSettings::get_global(cx).status_bar.encoding_indicator == false)
            || (self.show == false)
        {
            return status_element;
        }

        status_element.child(
            Button::new("encoding", encoding_name(self.encoding.unwrap_or(encoding_rs::UTF_8)))
                .label_size(LabelSize::Small)
                .tooltip(Tooltip::text("Select Encoding"))
                .on_click(cx.listener(|indicator, _: &ClickEvent, window, cx| {
                    if let Some(workspace) = indicator.workspace.upgrade() {
                        workspace.update(cx, |workspace, cx| {
                            EncodingSaveOrReopenSelector::toggle(workspace, window, cx)
                        })
                    } else {
                    }
                })),
        )
    }
}

impl EncodingIndicator {
    pub fn new(
        encoding: Option<&'static Encoding>,
        workspace: WeakEntity<Workspace>,
        observe: Option<Subscription>,
    ) -> EncodingIndicator {
        EncodingIndicator {
            encoding,
            workspace,
            observe,
            show: true,
        }
    }

    pub fn update(
        &mut self,
        editor: Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<EncodingIndicator>,
    ) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            let encoding = buffer.read(cx).encoding;
            self.encoding = Some(encoding);
        }

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
                self.observe = Some(cx.observe_in(&editor, window, Self::update));
                self.update(editor, window, cx);
                self.show = true;
            }
            None => {
                self.encoding = None;
                self.observe = None;
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
        1 => encoding_rs::WINDOWS_1252,
        2 => encoding_rs::WINDOWS_1251,
        3 => encoding_rs::WINDOWS_1250,
        4 => encoding_rs::ISO_8859_2,
        5 => encoding_rs::ISO_8859_3,
        6 => encoding_rs::ISO_8859_4,
        7 => encoding_rs::ISO_8859_5,
        8 => encoding_rs::ISO_8859_6,
        9 => encoding_rs::ISO_8859_7,
        10 => encoding_rs::ISO_8859_8,
        11 => encoding_rs::ISO_8859_13,
        12 => encoding_rs::ISO_8859_15,
        13 => encoding_rs::KOI8_R,
        14 => encoding_rs::KOI8_U,
        15 => encoding_rs::MACINTOSH,
        16 => encoding_rs::X_MAC_CYRILLIC,
        17 => encoding_rs::WINDOWS_874,
        18 => encoding_rs::WINDOWS_1253,
        19 => encoding_rs::WINDOWS_1254,
        20 => encoding_rs::WINDOWS_1255,
        21 => encoding_rs::WINDOWS_1256,
        22 => encoding_rs::WINDOWS_1257,
        23 => encoding_rs::WINDOWS_1258,
        24 => encoding_rs::EUC_KR,
        25 => encoding_rs::EUC_JP,
        26 => encoding_rs::ISO_2022_JP,
        27 => encoding_rs::GBK,
        28 => encoding_rs::GB18030,
        29 => encoding_rs::BIG5,
        _ => encoding_rs::UTF_8,
    }
}

/// Get an encoding from its name.
pub fn encoding_from_name(name: &str) -> &'static Encoding {
    match name {
        "UTF-8" => encoding_rs::UTF_8,
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
        "HZ-GB-2312" => encoding_rs::UTF_8, // encoding_rs doesn't support HZ, fallback to UTF-8
        _ => encoding_rs::UTF_8, // Default to UTF-8 for unknown names
    }
}
