use editor::Editor;
use encoding::Encoding;
use encoding::all::{
    BIG5_2003, EUC_JP, GB18030, GBK, HZ, IBM866, ISO_2022_JP, ISO_8859_1, ISO_8859_2, ISO_8859_3,
    ISO_8859_4, ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_10, ISO_8859_13,
    ISO_8859_14, ISO_8859_15, ISO_8859_16, KOI8_R, KOI8_U, MAC_CYRILLIC, MAC_ROMAN, UTF_8,
    UTF_16BE, UTF_16LE, WINDOWS_874, WINDOWS_949, WINDOWS_1250, WINDOWS_1251, WINDOWS_1252,
    WINDOWS_1253, WINDOWS_1254, WINDOWS_1255, WINDOWS_1256, WINDOWS_1257, WINDOWS_1258,
};
use gpui::{ClickEvent, Entity, Subscription, WeakEntity};
use ui::{Button, ButtonCommon, Context, LabelSize, Render, Tooltip, Window, div};
use ui::{Clickable, ParentElement};
use workspace::{ItemHandle, StatusItemView, Workspace};

use crate::selectors::save_or_reopen::{EncodingSaveOrReopenSelector, get_current_encoding};

/// A status bar item that shows the current file encoding and allows changing it.
pub struct EncodingIndicator {
    pub encoding: Option<&'static dyn Encoding>,
    pub workspace: WeakEntity<Workspace>,
    observe: Option<Subscription>, // Subscription to observe changes in the active editor
}

pub mod selectors;

impl Render for EncodingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let status_element = div();

        status_element.child(
            Button::new("encoding", encoding_name(self.encoding.unwrap_or(UTF_8)))
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
    pub fn get_current_encoding(&self, cx: &mut Context<Self>, editor: WeakEntity<Editor>) {}

    pub fn new(
        encoding: Option<&'static dyn encoding::Encoding>,
        workspace: WeakEntity<Workspace>,
        observe: Option<Subscription>,
    ) -> EncodingIndicator {
        EncodingIndicator {
            encoding,
            workspace,
            observe,
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
            }
            None => {
                self.encoding = None;
                self.observe = None;
            }
        }
    }
}

/// Get a human-readable name for the given encoding.
pub fn encoding_name(encoding: &'static dyn Encoding) -> String {
    let name = encoding.name();

    match () {
        () if name == UTF_8.name() => "UTF-8",
        () if name == UTF_16LE.name() => "UTF-16 LE",
        () if name == UTF_16BE.name() => "UTF-16 BE",
        () if name == IBM866.name() => "IBM866",
        () if name == ISO_8859_1.name() => "ISO 8859-1",
        () if name == ISO_8859_2.name() => "ISO 8859-2",
        () if name == ISO_8859_3.name() => "ISO 8859-3",
        () if name == ISO_8859_4.name() => "ISO 8859-4",
        () if name == ISO_8859_5.name() => "ISO 8859-5",
        () if name == ISO_8859_6.name() => "ISO 8859-6",
        () if name == ISO_8859_7.name() => "ISO 8859-7",
        () if name == ISO_8859_8.name() => "ISO 8859-8",
        () if name == ISO_8859_10.name() => "ISO 8859-10",
        () if name == ISO_8859_13.name() => "ISO 8859-13",
        () if name == ISO_8859_14.name() => "ISO 8859-14",
        () if name == ISO_8859_15.name() => "ISO 8859-15",
        () if name == ISO_8859_16.name() => "ISO 8859-16",
        () if name == KOI8_R.name() => "KOI8-R",
        () if name == KOI8_U.name() => "KOI8-U",
        () if name == MAC_ROMAN.name() => "MacRoman",
        () if name == MAC_CYRILLIC.name() => "Mac Cyrillic",
        () if name == WINDOWS_874.name() => "Windows-874",
        () if name == WINDOWS_1250.name() => "Windows-1250",
        () if name == WINDOWS_1251.name() => "Windows-1251",
        () if name == WINDOWS_1252.name() => "Windows-1252",
        () if name == WINDOWS_1253.name() => "Windows-1253",
        () if name == WINDOWS_1254.name() => "Windows-1254",
        () if name == WINDOWS_1255.name() => "Windows-1255",
        () if name == WINDOWS_1256.name() => "Windows-1256",
        () if name == WINDOWS_1257.name() => "Windows-1257",
        () if name == WINDOWS_1258.name() => "Windows-1258",
        () if name == WINDOWS_949.name() => "Windows-949",
        () if name == EUC_JP.name() => "EUC-JP",
        () if name == ISO_2022_JP.name() => "ISO 2022-JP",
        () if name == GBK.name() => "GBK",
        () if name == GB18030.name() => "GB18030",
        () if name == BIG5_2003.name() => "Big5",
        () if name == HZ.name() => "HZ-GB-2312",
        _ => "",
    }
    .to_string()
}

/// Get an encoding from its index in the predefined list.
/// If the index is out of range, UTF-8 is returned as a default.
pub fn encoding_from_index(index: usize) -> &'static dyn Encoding {
    match index {
        0 => UTF_8,
        1 => UTF_16LE,
        2 => UTF_16BE,
        3 => IBM866,
        4 => ISO_8859_1,
        5 => ISO_8859_2,
        6 => ISO_8859_3,
        7 => ISO_8859_4,
        8 => ISO_8859_5,
        9 => ISO_8859_6,
        10 => ISO_8859_7,
        11 => ISO_8859_8,
        12 => ISO_8859_10,
        13 => ISO_8859_13,
        14 => ISO_8859_14,
        15 => ISO_8859_15,
        16 => ISO_8859_16,
        17 => KOI8_R,
        18 => KOI8_U,
        19 => MAC_ROMAN,
        20 => MAC_CYRILLIC,
        21 => WINDOWS_874,
        22 => WINDOWS_1250,
        23 => WINDOWS_1251,
        24 => WINDOWS_1252,
        25 => WINDOWS_1253,
        26 => WINDOWS_1254,
        27 => WINDOWS_1255,
        28 => WINDOWS_1256,
        29 => WINDOWS_1257,
        30 => WINDOWS_1258,
        31 => WINDOWS_949,
        32 => EUC_JP,
        33 => ISO_2022_JP,
        34 => GBK,
        35 => GB18030,
        36 => BIG5_2003,
        37 => HZ,
        _ => UTF_8,
    }
}
