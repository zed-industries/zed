mod indentation_indicator;

use editor::Editor;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Task, WeakEntity, actions};
pub use indentation_indicator::IndentationIndicator;
use language::Buffer;
use language::language_settings::LanguageSettings;
use picker::{Picker, PickerDelegate};
use std::num::NonZeroU32;
use std::sync::Arc;
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::ModalView;

actions!(indentation_selector, [Toggle]);

pub fn init(cx: &mut App) {
    cx.observe_new(IndentationSelector::register).detach();
}

pub struct IndentationSelector {
    picker: Entity<Picker<IndentationSelectorDelegate>>,
}

impl IndentationSelector {
    fn register(editor: &mut Editor, _window: Option<&mut Window>, cx: &mut Context<Editor>) {
        let editor_handle = cx.weak_entity();
        editor
            .register_action(move |_: &Toggle, window, cx| Self::toggle(&editor_handle, window, cx))
            .detach();
    }

    pub fn toggle(editor: &WeakEntity<Editor>, window: &mut Window, cx: &mut App) {
        let Some((workspace, buffer)) = editor
            .update(cx, |editor, cx| {
                Some((editor.workspace()?, editor.active_buffer(cx)?))
            })
            .ok()
            .flatten()
        else {
            return;
        };
        let editor_handle = editor.clone();
        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, move |window, cx| {
                IndentationSelector::new(editor_handle, buffer, window, cx)
            });
        })
    }

    fn new(
        editor: WeakEntity<Editor>,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = IndentationSelectorDelegate::new(cx.entity().downgrade(), editor, buffer);
        let picker = cx.new(|cx| Picker::nonsearchable_uniform_list(delegate, window, cx));
        Self { picker }
    }
}

impl Render for IndentationSelector {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(self.picker.clone())
    }
}

impl Focusable for IndentationSelector {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for IndentationSelector {}
impl ModalView for IndentationSelector {}

#[derive(Clone, Copy, PartialEq)]
enum MainEntry {
    IndentUsingTabs,
    IndentUsingSpaces,
    ConvertIndentationToSpaces,
    ConvertIndentationToTabs,
    DetectIndentationFromContent,
}

impl MainEntry {
    const ALL: [MainEntry; 5] = [
        MainEntry::IndentUsingSpaces,
        MainEntry::IndentUsingTabs,
        MainEntry::DetectIndentationFromContent,
        MainEntry::ConvertIndentationToSpaces,
        MainEntry::ConvertIndentationToTabs,
    ];

    fn label(&self) -> &'static str {
        match self {
            MainEntry::IndentUsingTabs => "Indent Using Tabs",
            MainEntry::IndentUsingSpaces => "Indent Using Spaces",
            MainEntry::ConvertIndentationToSpaces => "Convert Indentation to Spaces",
            MainEntry::ConvertIndentationToTabs => "Convert Indentation to Tabs",
            MainEntry::DetectIndentationFromContent => "Detect Indentation from Content",
        }
    }
}

enum Mode {
    Main,
    TabSize { hard_tabs: bool },
}

struct IndentationSelectorDelegate {
    indentation_selector: WeakEntity<IndentationSelector>,
    editor: WeakEntity<Editor>,
    buffer: Entity<Buffer>,
    mode: Mode,
    selected_index: usize,
}

const TAB_SIZES: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

impl IndentationSelectorDelegate {
    fn new(
        indentation_selector: WeakEntity<IndentationSelector>,
        editor: WeakEntity<Editor>,
        buffer: Entity<Buffer>,
    ) -> Self {
        Self {
            indentation_selector,
            editor,
            buffer,
            mode: Mode::Main,
            selected_index: 0,
        }
    }

    fn resolved_settings<'a>(&self, cx: &'a App) -> std::borrow::Cow<'a, LanguageSettings> {
        LanguageSettings::for_buffer(self.buffer.read(cx), cx)
    }

    fn change_indentation(
        &self,
        hard_tabs: bool,
        tab_size: NonZeroU32,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(editor) = self.editor.upgrade() else {
            return;
        };
        editor.update(cx, |editor, cx| {
            editor.change_indentation(hard_tabs, tab_size, cx);
        });
    }

    fn convert_indentation(&self, hard_tabs: bool, cx: &mut Context<Picker<Self>>) {
        let tab_size = self.resolved_settings(cx).tab_size;
        self.change_indentation(hard_tabs, tab_size, cx);
    }

    fn detect_from_content(&self, cx: &mut Context<Picker<Self>>) {
        self.buffer.update(cx, |buffer, cx| {
            buffer.redetect_indentation(cx);
        });
    }
}

impl PickerDelegate for IndentationSelectorDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "indentation selector"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.mode {
            Mode::Main => "Select an indentation action…".into(),
            Mode::TabSize { .. } => "Select tab size…".into(),
        }
    }

    fn match_count(&self) -> usize {
        match self.mode {
            Mode::Main => MainEntry::ALL.len(),
            Mode::TabSize { .. } => TAB_SIZES.len(),
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        _query: String,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        match self.mode {
            Mode::Main => {
                let Some(entry) = MainEntry::ALL.get(self.selected_index).copied() else {
                    return;
                };
                match entry {
                    MainEntry::IndentUsingTabs => {
                        self.mode = Mode::TabSize { hard_tabs: true };
                        let tab_size = self.resolved_settings(cx).tab_size.get();
                        self.selected_index = tab_size.saturating_sub(1).min(7) as usize;
                        cx.notify();
                        return;
                    }
                    MainEntry::IndentUsingSpaces => {
                        self.mode = Mode::TabSize { hard_tabs: false };
                        let tab_size = self.resolved_settings(cx).tab_size.get();
                        self.selected_index = tab_size.saturating_sub(1).min(7) as usize;
                        cx.notify();
                        return;
                    }
                    MainEntry::ConvertIndentationToSpaces => {
                        self.convert_indentation(false, cx);
                    }
                    MainEntry::ConvertIndentationToTabs => self.convert_indentation(true, cx),
                    MainEntry::DetectIndentationFromContent => self.detect_from_content(cx),
                }
            }
            Mode::TabSize { hard_tabs } => {
                let Some(size) = TAB_SIZES
                    .get(self.selected_index)
                    .copied()
                    .and_then(NonZeroU32::new)
                else {
                    return;
                };
                self.change_indentation(hard_tabs, size, cx);
            }
        }
        self.dismissed(window, cx);
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.indentation_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let settings = self.resolved_settings(cx);
        match self.mode {
            Mode::Main => {
                let entry = MainEntry::ALL.get(ix).copied()?;
                let is_active = matches!(
                    (entry, settings.hard_tabs),
                    (MainEntry::IndentUsingTabs, true) | (MainEntry::IndentUsingSpaces, false)
                );
                let mut item = ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(entry.label()));
                if is_active {
                    item = item.end_slot(Icon::new(IconName::Check).color(Color::Muted));
                }
                Some(item)
            }
            Mode::TabSize { .. } => {
                let size = *TAB_SIZES.get(ix)?;
                let mut item = ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(size.to_string()));
                if settings.tab_size.get() == size {
                    item = item.end_slot(Icon::new(IconName::Check).color(Color::Muted));
                }
                Some(item)
            }
        }
    }
}
