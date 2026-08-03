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
                .show_scrollbar(true)
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

    let menu = PopoverMenu::new(("breadcrumb-symbol-menu", index));
    let menu = if target.is_none() {
        menu.trigger_with_tooltip(trigger, ui::Tooltip::text("Right-Click to Copy Path"))
    } else {
        menu.trigger(trigger)
    };
    menu.menu(move |window, cx| {
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

#[cfg(test)]
mod tests {
    use super::*;

    use editor::MultiBuffer;
    use editor::MultiBufferSnapshot;
    use editor::test::build_editor;
    use gpui::{Focusable, Render, TestAppContext, VisualTestContext};
    use std::cell::Cell;
    use std::rc::Rc;
    use text::Point;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let _app_state = workspace::AppState::test(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            crate::init(cx);
        });
    }

    /// A zero-width item anchored at the start of `row`, standing in for a real outline entry.
    fn test_item(snapshot: &MultiBufferSnapshot, row: u32, text: &str) -> OutlineItem<Anchor> {
        let range =
            snapshot.anchor_before(Point::new(row, 0))..snapshot.anchor_after(Point::new(row, 0));
        OutlineItem {
            depth: 0,
            range: range.clone(),
            selection_range: range.clone(),
            source_range_for_text: range,
            text: text.into(),
            highlight_ranges: Vec::new(),
            name_ranges: Vec::new(),
            body_range: None,
            annotation_range: None,
        }
    }

    fn build_test_editor(cx: &mut TestAppContext) -> (gpui::WindowHandle<Editor>, Entity<Editor>) {
        let buffer = cx.new(|cx| language::Buffer::local("alpha\nbeta\ngamma\n", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
        let editor_window =
            cx.add_window(|window, cx| build_editor(multi_buffer.clone(), window, cx));
        let editor = editor_window.root(cx).unwrap();
        (editor_window, editor)
    }

    #[gpui::test]
    async fn test_breadcrumb_symbol_picker_preselects_current_symbol(cx: &mut TestAppContext) {
        init_test(cx);

        let (editor_window, editor) = build_test_editor(cx);
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let snapshot = editor.read_with(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx));
        let items = vec![
            test_item(&snapshot, 0, "alpha"),
            test_item(&snapshot, 1, "beta"),
            test_item(&snapshot, 2, "gamma"),
        ];
        let current_range = items[1].range.clone();

        let picker = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbSymbolDelegate::picker(
                    editor.downgrade(),
                    items,
                    Some(current_range),
                    window,
                    cx,
                )
            })
            .unwrap();

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker.delegate.selected_index, 1,
                "the segment's own symbol is preselected"
            );
        });

        picker
            .update_in(cx, |picker, window, cx| {
                picker.delegate.update_matches(String::new(), window, cx)
            })
            .await;

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker.delegate.selected_index, 1,
                "clearing the query keeps the current symbol selected"
            );
        });
    }

    #[gpui::test]
    async fn test_breadcrumb_symbol_picker_fuzzy_filtering_resets_selection(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let (editor_window, editor) = build_test_editor(cx);
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let snapshot = editor.read_with(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx));
        let items = vec![
            test_item(&snapshot, 0, "alpha"),
            test_item(&snapshot, 1, "beta"),
            test_item(&snapshot, 2, "gamma"),
        ];

        let picker = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbSymbolDelegate::picker(editor.downgrade(), items, None, window, cx)
            })
            .unwrap();

        // "gam" is a subsequence of "gamma" only: neither "alpha" nor "beta" contains a 'g' or
        // 'm', so this discriminates cleanly under fuzzy matching.
        picker
            .update_in(cx, |picker, window, cx| {
                picker
                    .delegate
                    .update_matches("gam".to_string(), window, cx)
            })
            .await;

        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker.delegate.matches.len(),
                1,
                "the query narrows the listing to the one discriminating match"
            );
            assert_eq!(
                picker.delegate.item_at(0).map(|item| item.text.as_ref()),
                Some("gamma")
            );
            assert_eq!(
                picker.delegate.selected_index, 0,
                "filtering resets the selection to the top match"
            );
        });
    }

    #[gpui::test]
    async fn test_confirming_breadcrumb_symbol_navigates_and_dismisses(cx: &mut TestAppContext) {
        init_test(cx);

        let (editor_window, editor) = build_test_editor(cx);
        let cx = &mut VisualTestContext::from_window(*editor_window, cx);

        let snapshot = editor.read_with(cx, |editor, cx| editor.buffer().read(cx).snapshot(cx));
        let items = vec![
            test_item(&snapshot, 0, "alpha"),
            test_item(&snapshot, 1, "beta"),
            test_item(&snapshot, 2, "gamma"),
        ];

        let picker = editor_window
            .update(cx, |_, window, cx| {
                BreadcrumbSymbolDelegate::picker(editor.downgrade(), items, None, window, cx)
            })
            .unwrap();

        let dismissed = Rc::new(Cell::new(false));
        let subscription = cx.update(|_, cx| {
            cx.subscribe(&picker, {
                let dismissed = dismissed.clone();
                move |_, _: &DismissEvent, _| dismissed.set(true)
            })
        });

        picker.update_in(cx, |picker, window, cx| {
            picker.delegate.selected_index = 2;
            picker.delegate.confirm(false, window, cx);
        });
        cx.run_until_parked();

        editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<Point>(&snapshot).head();
            assert_eq!(
                cursor,
                Point::new(2, 0),
                "confirming navigates the cursor to the chosen symbol"
            );
        });
        assert!(dismissed.get(), "confirming a row dismisses the popover");
        drop(subscription);
    }

    /// The whole flow driven by `menu::` actions rather than by simulated keystrokes: move the
    /// selection, submit it, and end up with the cursor on the chosen symbol.
    #[gpui::test]
    async fn test_breadcrumb_symbol_picker_navigates_from_the_keyboard(cx: &mut TestAppContext) {
        init_test(cx);

        let buffer = cx.new(|cx| language::Buffer::local("alpha\nbeta\ngamma\n", cx));
        let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));

        struct Harness {
            picker: Entity<BreadcrumbSymbolPicker>,
            editor: Entity<Editor>,
        }
        impl Render for Harness {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                self.picker.clone()
            }
        }

        let harness_window = cx.add_window(|window, cx| {
            let editor = cx.new(|cx| build_editor(multi_buffer.clone(), window, cx));
            let snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
            let items = vec![
                test_item(&snapshot, 0, "alpha"),
                test_item(&snapshot, 1, "beta"),
                test_item(&snapshot, 2, "gamma"),
            ];
            let picker =
                BreadcrumbSymbolDelegate::picker(editor.downgrade(), items, None, window, cx);
            Harness { picker, editor }
        });
        let (picker, editor) = harness_window
            .read_with(cx, |harness, _| {
                (harness.picker.clone(), harness.editor.clone())
            })
            .unwrap();
        let cx = &mut VisualTestContext::from_window(*harness_window, cx);
        cx.run_until_parked();

        picker.update_in(cx, |picker, window, cx| {
            window.focus(&picker.focus_handle(cx), cx);
        });
        cx.run_until_parked();

        // The listing opens on `alpha`; one step down lands on `beta`.
        cx.dispatch_action(menu::SelectNext);
        picker.read_with(cx, |picker, _| {
            assert_eq!(
                picker
                    .delegate
                    .item_at(picker.delegate.selected_index)
                    .map(|item| item.text.as_ref()),
                Some("beta"),
            );
        });

        cx.dispatch_action(menu::Confirm);
        cx.run_until_parked();

        editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            let cursor = editor.selections.newest::<Point>(&snapshot).head();
            assert_eq!(
                cursor,
                Point::new(1, 0),
                "confirming the selected row navigates the cursor there"
            );
        });
    }
}
