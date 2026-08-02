use std::ops::Range;
use std::sync::Arc;

use editor::{Anchor, Editor, flatten_text_for_single_line_display};
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, ParentElement, Styled, StyledText, Task,
    WeakEntity, Window, div, rems,
};
use language::OutlineItem;
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use text::BufferId;
use ui::{
    ButtonLike, ButtonStyle, Color, Icon, IconName, IconSize, ListItem, ListItemSpacing,
    PopoverMenu, prelude::*,
};
use workspace::ItemHandle;

use crate::MAX_BREADCRUMB_MENU_ENTRIES;

/// The symbols a breadcrumb segment can move to, filtered by the picker's query.
pub struct BreadcrumbSymbolDelegate {
    editor: WeakEntity<Editor>,
    items: Vec<OutlineItem<Anchor>>,
    matches: Vec<StringMatch>,
    selected_index: usize,
    /// The segment's own symbol, so the row standing for it reads as the current one.
    current_range: Option<Range<Anchor>>,
}

pub type BreadcrumbSymbolPicker = Picker<BreadcrumbSymbolDelegate>;

impl BreadcrumbSymbolDelegate {
    fn picker(
        editor: WeakEntity<Editor>,
        items: Vec<OutlineItem<Anchor>>,
        current_range: Option<Range<Anchor>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<BreadcrumbSymbolPicker> {
        cx.new(|cx| {
            let selected_index = current_range
                .as_ref()
                .and_then(|range| items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            let delegate = Self {
                editor,
                items,
                matches: Vec::new(),
                selected_index,
                current_range,
            };
            Picker::uniform_list(delegate, window, cx)
                .popover()
                .initial_width(rems(18.))
        })
    }

    /// Whether any listed symbol is the segment's own. If none is, the checkmark column is left
    /// out rather than indenting every row for a mark that never appears.
    fn shows_current_marker(&self) -> bool {
        self.current_range
            .as_ref()
            .is_some_and(|range| self.items.iter().any(|item| &item.range == range))
    }

    fn item_at(&self, index: usize) -> Option<&OutlineItem<Anchor>> {
        self.items.get(self.matches.get(index)?.candidate_id)
    }
}

impl PickerDelegate for BreadcrumbSymbolDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "breadcrumb symbol picker"
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        index: usize,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        self.selected_index = index;
        cx.notify();
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search symbols…".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::End
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Task<()> {
        let candidates = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                StringMatchCandidate::new(index, &flatten_text_for_single_line_display(&item.text))
            })
            .collect::<Vec<_>>();

        if query.is_empty() {
            self.matches = candidates
                .into_iter()
                .map(|candidate| StringMatch {
                    candidate_id: candidate.id,
                    string: candidate.string,
                    positions: Vec::new(),
                    score: 0.,
                })
                .collect();
            self.selected_index = self
                .current_range
                .as_ref()
                .and_then(|range| self.items.iter().position(|item| &item.range == range))
                .unwrap_or(0);
            cx.notify();
            return Task::ready(());
        }

        let executor = cx.background_executor().clone();
        cx.spawn(async move |picker, cx| {
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                true,
                MAX_BREADCRUMB_MENU_ENTRIES,
                &Default::default(),
                executor,
            )
            .await;
            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = matches;
                    picker.delegate.selected_index = 0;
                    cx.notify();
                })
                .ok();
        })
    }

    fn confirm(
        &mut self,
        _secondary: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) {
        let Some(item) = self.item_at(self.selected_index).cloned() else {
            return;
        };
        if let Some(editor) = self.editor.upgrade() {
            editor.update(cx, |editor, cx| {
                editor.navigate_to_outline_item(&item, window, cx);
            });
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<BreadcrumbSymbolPicker>) {}

    /// Rendered with the symbol's own syntax highlighting, the way the outline picker and panel
    /// draw it. The fuzzy match positions are left off, since the two highlight sets would
    /// fight.
    fn render_match(
        &self,
        index: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<BreadcrumbSymbolPicker>,
    ) -> Option<Self::ListItem> {
        let item = self.item_at(index)?;
        let is_current = self.current_range.as_ref() == Some(&item.range);

        let mut text_style = window.text_style();
        text_style.color = Color::Default.color(cx);

        Some(
            ListItem::new(SharedString::from(format!(
                "breadcrumb-symbol-entry-{index}"
            )))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .when(self.shows_current_marker(), |this| {
                this.start_slot(div().flex_none().size(IconSize::Small.rems()).when(
                    is_current,
                    |this| {
                        this.child(
                            Icon::new(IconName::Check)
                                .color(Color::Accent)
                                .size(IconSize::Small),
                        )
                    },
                ))
            })
            .child(
                div().text_ui(cx).child(
                    StyledText::new(flatten_text_for_single_line_display(&item.text))
                        .with_default_highlights(&text_style, item.highlight_ranges.clone()),
                ),
            )
            .into_any_element(),
        )
    }
}

/// A segment whose dropdown drills into the outline: `target`'s children, else its siblings, else
/// the buffer's top-level symbols.
pub(crate) fn render_breadcrumb_symbol_segment(
    editor: WeakEntity<Editor>,
    buffer_id: BufferId,
    target: Option<OutlineItem<Anchor>>,
    label: gpui::AnyElement,
    index: usize,
) -> gpui::AnyElement {
    // `ButtonLike` wraps its click handler in `cx.stop_propagation()`, which is what keeps this
    // click from also reaching the outline toggle behind the popover.
    let trigger = ButtonLike::new(("breadcrumb-symbol", index))
        .style(ButtonStyle::Transparent)
        .size(ui::ButtonSize::None)
        .height(rems_from_px(22.).into())
        .child(label);

    PopoverMenu::new(("breadcrumb-symbol-menu", index))
        .trigger(trigger)
        .menu(move |window, cx| {
            let editor_entity = editor.upgrade()?;
            let menu_items =
                editor_entity
                    .read(cx)
                    .breadcrumb_symbol_menu_items(buffer_id, target.as_ref(), cx);
            // Nothing to drill into, so fall through to the outline picker rather than flashing an
            // empty popover.
            if menu_items.is_empty() {
                if let Some(callback) = zed_actions::outline::TOGGLE_OUTLINE.get() {
                    callback(editor_entity.to_any_view(), window, cx);
                }
                return None;
            }
            Some(BreadcrumbSymbolDelegate::picker(
                editor.clone(),
                menu_items,
                target.as_ref().map(|item| item.range.clone()),
                window,
                cx,
            ))
        })
        .into_any_element()
}
