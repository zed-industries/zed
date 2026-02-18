mod head;
pub mod highlighted_match_with_paths;
pub mod popover_menu;

use anyhow::Result;

use gpui::{
    Action, AnyElement, App, Bounds, ClickEvent, Context, DismissEvent, EventEmitter, FocusHandle,
    Focusable, Length, ListSizingBehavior, ListState, MouseButton, MouseUpEvent, Pixels, Render,
    ScrollStrategy, Task, UniformListScrollHandle, Window, actions, canvas, div, list, prelude::*,
    uniform_list,
};
use head::Head;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    cell::Cell, cell::RefCell, collections::HashMap, ops::Range, rc::Rc, sync::Arc, time::Duration,
};
use theme::ThemeSettings;
use ui::{
    Color, Divider, DocumentationAside, DocumentationSide, Label, ListItem, ListItemSpacing,
    ScrollAxes, Scrollbars, WithScrollbar, prelude::*, utils::WithRemSize, v_flex,
};
use ui_input::{ErasedEditor, ErasedEditorEvent};
use workspace::{ModalView, item::Settings};
use zed_actions::editor::{MoveDown, MoveUp};

enum ElementContainer {
    List(ListState),
    UniformList(UniformListScrollHandle),
}

pub enum Direction {
    Up,
    Down,
}

actions!(
    picker,
    [
        /// Confirms the selected completion in the picker.
        ConfirmCompletion
    ]
);

/// ConfirmInput is an alternative editor action which - instead of selecting active picker entry - treats pickers editor input literally,
/// performing some kind of action on it.
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Default, Action)]
#[action(namespace = picker)]
#[serde(deny_unknown_fields)]
pub struct ConfirmInput {
    pub secondary: bool,
}

struct PendingUpdateMatches {
    delegate_update_matches: Option<Task<()>>,
    _task: Task<Result<()>>,
}

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    element_container: ElementContainer,
    head: Head,
    pending_update_matches: Option<PendingUpdateMatches>,
    confirm_on_update: Option<bool>,
    width: Option<Length>,
    widest_item: Option<usize>,
    max_height: Option<Length>,
    /// An external control to display a scrollbar in the `Picker`.
    show_scrollbar: bool,
    /// Whether the `Picker` is rendered as a self-contained modal.
    ///
    /// Set this to `false` when rendering the `Picker` as part of a larger modal.
    is_modal: bool,
    /// Bounds tracking for the picker container (for aside positioning)
    picker_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// Bounds tracking for items (for aside positioning) - maps item index to bounds
    item_bounds: Rc<RefCell<HashMap<usize, Bounds<Pixels>>>>,
    /// Tracks the stable ID of a manually selected item to preserve it across match updates.
    manually_selected_stable_id: Option<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PickerEditorPosition {
    #[default]
    /// Render the editor at the start of the picker. Usually the top
    Start,
    /// Render the editor at the end of the picker. Usually the bottom
    End,
}

pub trait PickerDelegate: Sized + 'static {
    type ListItem: IntoElement;

    fn match_count(&self) -> usize;
    fn selected_index(&self) -> usize;
    fn separators_after_indices(&self) -> Vec<usize> {
        Vec::new()
    }
    fn set_selected_index(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    );

    /// Called before the picker handles `SelectPrevious` or `SelectNext`. Return `Some(query)` to
    /// set a new query and prevent the default selection behavior.
    fn select_history(
        &mut self,
        _direction: Direction,
        _query: &str,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<String> {
        None
    }
    fn can_select(
        &mut self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        true
    }

    // Allows binding some optional effect to when the selection changes.
    fn selected_index_changed(
        &self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Box<dyn Fn(&mut Window, &mut App) + 'static>> {
        None
    }
    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str>;
    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No matches".into())
    }

    /// Returns a stable identifier for the match at the given index.
    /// If implemented, the picker will try to preserve manual selections
    /// across match updates by finding the same item again.
    fn match_stable_id(&self, _ix: usize) -> Option<String> {
        None
    }

    /// Finds the index of a match with the given stable identifier.
    /// Used in conjunction with `match_stable_id` to restore selections.
    fn find_match_by_stable_id(&self, _stable_id: &str) -> Option<usize> {
        None
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()>;

    // Delegates that support this method (e.g. the CommandPalette) can chose to block on any background
    // work for up to `duration` to try and get a result synchronously.
    // This avoids a flash of an empty command-palette on cmd-shift-p, and lets workspace::SendKeystrokes
    // mostly work when dismissing a palette.
    fn finalize_update_matches(
        &mut self,
        _query: String,
        _duration: Duration,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        false
    }

    /// Override if you want to have <enter> update the query instead of confirming.
    fn confirm_update_query(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }
    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>);
    /// Instead of interacting with currently selected entry, treats editor input literally,
    /// performing some kind of action on it.
    fn confirm_input(
        &mut self,
        _secondary: bool,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
    }
    fn dismissed(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>);
    fn should_dismiss(&self) -> bool {
        true
    }
    fn confirm_completion(
        &mut self,
        _query: String,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        None
    }

    fn editor_position(&self) -> PickerEditorPosition {
        PickerEditorPosition::default()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        v_flex()
            .when(
                self.editor_position() == PickerEditorPosition::End,
                |this| this.child(Divider::horizontal()),
            )
            .child(
                h_flex()
                    .overflow_hidden()
                    .flex_none()
                    .h_9()
                    .px_2p5()
                    .child(editor.render(window, cx)),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem>;

    fn render_header(
        &self,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        None
    }

    fn render_footer(
        &self,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        None
    }

    fn documentation_aside(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<DocumentationAside> {
        None
    }

    /// Returns the index of the item whose documentation aside should be shown.
    /// This is used to position the aside relative to that item.
    /// Typically this is the hovered item, not necessarily the selected item.
    fn documentation_aside_index(&self) -> Option<usize> {
        None
    }
}

impl<D: PickerDelegate> Focusable for Picker<D> {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.head {
            Head::Editor(editor) => editor.focus_handle(cx),
            Head::Empty(head) => head.focus_handle(cx),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum ContainerKind {
    List,
    UniformList,
}

impl<D: PickerDelegate> Picker<D> {
    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn uniform_list(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(window, cx),
            Self::on_input_editor_event,
            window,
            cx,
        );

        Self::new(delegate, ContainerKind::UniformList, head, window, cx)
    }

    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn nonsearchable_uniform_list(
        delegate: D,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let head = Head::empty(Self::on_empty_head_blur, window, cx);

        Self::new(delegate, ContainerKind::UniformList, head, window, cx)
    }

    /// A picker, which displays its matches using `gpui::list`, matches can have different heights.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` only returns items with the same height, use `Picker::uniform_list` as its implementation is optimized for that.
    pub fn nonsearchable_list(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let head = Head::empty(Self::on_empty_head_blur, window, cx);

        Self::new(delegate, ContainerKind::List, head, window, cx)
    }

    /// A picker, which displays its matches using `gpui::list`, matches can have different heights.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` only returns items with the same height, use `Picker::uniform_list` as its implementation is optimized for that.
    pub fn list(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(window, cx),
            Self::on_input_editor_event,
            window,
            cx,
        );

        Self::new(delegate, ContainerKind::List, head, window, cx)
    }

    fn new(
        delegate: D,
        container: ContainerKind,
        head: Head,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let element_container = Self::create_element_container(container);
        let mut this = Self {
            delegate,
            head,
            element_container,
            pending_update_matches: None,
            confirm_on_update: None,
            width: None,
            widest_item: None,
            max_height: Some(rems(24.).into()),
            show_scrollbar: false,
            is_modal: true,
            picker_bounds: Rc::new(Cell::new(None)),
            item_bounds: Rc::new(RefCell::new(HashMap::default())),
            manually_selected_stable_id: None,
        };
        this.update_matches("".to_string(), window, cx);
        // give the delegate 4ms to render the first set of suggestions.
        this.delegate
            .finalize_update_matches("".to_string(), Duration::from_millis(4), window, cx);
        this
    }

    fn create_element_container(container: ContainerKind) -> ElementContainer {
        match container {
            ContainerKind::UniformList => {
                ElementContainer::UniformList(UniformListScrollHandle::new())
            }
            ContainerKind::List => {
                ElementContainer::List(ListState::new(0, gpui::ListAlignment::Top, px(1000.)))
            }
        }
    }

    pub fn width(mut self, width: impl Into<gpui::Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn widest_item(mut self, ix: Option<usize>) -> Self {
        self.widest_item = ix;
        self
    }

    pub fn max_height(mut self, max_height: Option<gpui::Length>) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.is_modal = modal;
        self
    }

    pub fn list_measure_all(mut self) -> Self {
        match self.element_container {
            ElementContainer::List(state) => {
                self.element_container = ElementContainer::List(state.measure_all());
            }
            _ => {}
        }
        self
    }

    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.focus_handle(cx).focus(window, cx);
    }

    /// Handles the selecting an index, and passing the change to the delegate.
    /// If `fallback_direction` is set to `None`, the index will not be selected
    /// if the element at that index cannot be selected.
    /// If `fallback_direction` is set to
    /// `Some(..)`, the next selectable element will be selected in the
    /// specified direction (Down or Up), cycling through all elements until
    /// finding one that can be selected or returning if there are no selectable elements.
    /// If `scroll_to_index` is true, the new selected index will be scrolled into
    /// view.
    ///
    /// If some effect is bound to `selected_index_changed`, it will be executed.
    ///
    /// This method is for programmatic selection changes. For user-driven selections
    /// that should be preserved across match updates, use `select_index_sticky` instead.
    pub fn set_selected_index(
        &mut self,
        ix: usize,
        fallback_direction: Option<Direction>,
        scroll_to_index: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_selected_index_impl(ix, fallback_direction, scroll_to_index, false, window, cx);
    }

    /// Selects an index with "sticky" behavior - the selection will be preserved across
    /// match updates if the selected item still matches the search query.
    ///
    /// Use this for user-driven selections (keyboard navigation, mouse clicks) where you want
    /// the user's choice to be maintained as they continue typing. For programmatic selections
    /// that should not persist, use `set_selected_index` instead.
    pub fn select_index_sticky(
        &mut self,
        ix: usize,
        fallback_direction: Option<Direction>,
        scroll_to_index: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_selected_index_impl(ix, fallback_direction, scroll_to_index, true, window, cx);
    }

    fn set_selected_index_impl(
        &mut self,
        mut ix: usize,
        fallback_direction: Option<Direction>,
        scroll_to_index: bool,
        is_manual_selection: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let match_count = self.delegate.match_count();
        if match_count == 0 {
            return;
        }

        if let Some(bias) = fallback_direction {
            let mut curr_ix = ix;
            while !self.delegate.can_select(curr_ix, window, cx) {
                curr_ix = match bias {
                    Direction::Down => {
                        if curr_ix == match_count - 1 {
                            0
                        } else {
                            curr_ix + 1
                        }
                    }
                    Direction::Up => {
                        if curr_ix == 0 {
                            match_count - 1
                        } else {
                            curr_ix - 1
                        }
                    }
                };
                // There is no item that can be selected
                if ix == curr_ix {
                    return;
                }
            }
            ix = curr_ix;
        } else if !self.delegate.can_select(ix, window, cx) {
            return;
        }

        let previous_index = self.delegate.selected_index();
        self.delegate.set_selected_index(ix, window, cx);
        let current_index = self.delegate.selected_index();

        if is_manual_selection {
            self.manually_selected_stable_id = self.delegate.match_stable_id(current_index);
        }

        if previous_index != current_index {
            if let Some(action) = self.delegate.selected_index_changed(ix, window, cx) {
                action(window, cx);
            }
            if scroll_to_index {
                self.scroll_to_item_index(ix);
            }
        }
    }

    pub fn select_next(
        &mut self,
        _: &menu::SelectNext,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let query = self.query(cx);
        if let Some(query) = self
            .delegate
            .select_history(Direction::Down, &query, window, cx)
        {
            self.set_query(&query, window, cx);
            return;
        }
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == count - 1 { 0 } else { index + 1 };
            self.select_index_sticky(ix, Some(Direction::Down), true, window, cx);
            cx.notify();
        }
    }

    pub fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&Default::default(), window, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let query = self.query(cx);
        if let Some(query) = self
            .delegate
            .select_history(Direction::Up, &query, window, cx)
        {
            self.set_query(&query, window, cx);
            return;
        }
        let count = self.delegate.match_count();
        if count > 0 {
            let index = self.delegate.selected_index();
            let ix = if index == 0 { count - 1 } else { index - 1 };
            self.select_index_sticky(ix, Some(Direction::Up), true, window, cx);
            cx.notify();
        }
    }

    pub fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&Default::default(), window, cx);
    }

    pub fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.select_index_sticky(0, Some(Direction::Down), true, window, cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.select_index_sticky(count - 1, Some(Direction::Up), true, window, cx);
            cx.notify();
        }
    }

    pub fn cycle_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.delegate.match_count();
        let index = self.delegate.selected_index();
        let new_index = if index + 1 == count { 0 } else { index + 1 };
        self.select_index_sticky(new_index, Some(Direction::Down), true, window, cx);
        cx.notify();
    }

    pub fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.delegate.should_dismiss() {
            self.delegate.dismissed(window, cx);
            cx.emit(DismissEvent);
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if self.pending_update_matches.is_some()
            && !self.delegate.finalize_update_matches(
                self.query(cx),
                Duration::from_millis(16),
                window,
                cx,
            )
        {
            self.confirm_on_update = Some(false)
        } else {
            self.pending_update_matches.take();
            self.do_confirm(false, window, cx);
        }
    }

    fn secondary_confirm(
        &mut self,
        _: &menu::SecondaryConfirm,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.pending_update_matches.is_some()
            && !self.delegate.finalize_update_matches(
                self.query(cx),
                Duration::from_millis(16),
                window,
                cx,
            )
        {
            self.confirm_on_update = Some(true)
        } else {
            self.do_confirm(true, window, cx);
        }
    }

    fn confirm_input(&mut self, input: &ConfirmInput, window: &mut Window, cx: &mut Context<Self>) {
        self.delegate.confirm_input(input.secondary, window, cx);
    }

    fn confirm_completion(
        &mut self,
        _: &ConfirmCompletion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(new_query) = self.delegate.confirm_completion(self.query(cx), window, cx) {
            self.set_query(&new_query, window, cx);
        } else {
            cx.propagate()
        }
    }

    fn handle_click(
        &mut self,
        ix: usize,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        window.prevent_default();
        self.select_index_sticky(ix, None, false, window, cx);
        self.do_confirm(secondary, window, cx)
    }

    fn do_confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(update_query) = self.delegate.confirm_update_query(window, cx) {
            self.set_query(&update_query, window, cx);
            self.set_selected_index(0, Some(Direction::Down), false, window, cx);
        } else {
            self.delegate.confirm(secondary, window, cx)
        }
    }

    fn on_input_editor_event(
        &mut self,
        event: &ErasedEditorEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Head::Editor(editor) = &self.head else {
            panic!("unexpected call");
        };
        match event {
            ErasedEditorEvent::BufferEdited => {
                let query = editor.text(cx);
                self.update_matches(query, window, cx);
            }
            ErasedEditorEvent::Blurred => {
                if self.is_modal && window.is_window_active() {
                    self.cancel(&menu::Cancel, window, cx);
                }
            }
        }
    }

    fn on_empty_head_blur(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Head::Empty(_) = &self.head else {
            panic!("unexpected call");
        };
        if window.is_window_active() {
            self.cancel(&menu::Cancel, window, cx);
        }
    }

    pub fn refresh_placeholder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &self.head {
            Head::Editor(editor) => {
                let placeholder = self.delegate.placeholder_text(window, cx);

                editor.set_placeholder_text(placeholder.as_ref(), window, cx);
                cx.notify();
            }
            Head::Empty(_) => {}
        }
    }

    pub fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let query = self.query(cx);
        self.update_matches(query, window, cx);
    }

    pub fn update_matches(&mut self, query: String, window: &mut Window, cx: &mut Context<Self>) {
        let delegate_pending_update_matches = self.delegate.update_matches(query, window, cx);

        self.matches_updated(window, cx);
        // This struct ensures that we can synchronously drop the task returned by the
        // delegate's `update_matches` method and the task that the picker is spawning.
        // If we simply capture the delegate's task into the picker's task, when the picker's
        // task gets synchronously dropped, the delegate's task would keep running until
        // the picker's task has a chance of being scheduled, because dropping a task happens
        // asynchronously.
        self.pending_update_matches = Some(PendingUpdateMatches {
            delegate_update_matches: Some(delegate_pending_update_matches),
            _task: cx.spawn_in(window, async move |this, cx| {
                let delegate_pending_update_matches = this.update(cx, |this, _| {
                    this.pending_update_matches
                        .as_mut()
                        .unwrap()
                        .delegate_update_matches
                        .take()
                        .unwrap()
                })?;
                delegate_pending_update_matches.await;
                this.update_in(cx, |this, window, cx| {
                    this.matches_updated(window, cx);
                })
            }),
        });
    }

    fn matches_updated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let ElementContainer::List(state) = &mut self.element_container {
            state.reset(self.delegate.match_count());
        }

        // Try to restore manually selected item
        let match_count = self.delegate.match_count();
        let index = if let Some(stable_id) = &self.manually_selected_stable_id {
            if let Some(ix) = self.delegate.find_match_by_stable_id(stable_id) {
                // Found the manually selected item, restore selection
                self.delegate.set_selected_index(ix, window, cx);
                ix
            } else {
                // Item no longer in results, clear manual selection and reset to first item
                self.manually_selected_stable_id = None;
                let current_index = self.delegate.selected_index();
                let ix = current_index.min(match_count.saturating_sub(1));

                if match_count > 0 {
                    self.delegate.set_selected_index(ix, window, cx);
                }
                ix
            }
        } else {
            // No manual selection - clamp current index to valid range
            let current_index = self.delegate.selected_index();
            let ix = current_index.min(match_count.saturating_sub(1));
            if match_count > 0 && current_index != ix {
                self.delegate.set_selected_index(ix, window, cx);
            }
            ix
        };

        self.scroll_to_item_index(index);
        self.pending_update_matches = None;
        if let Some(secondary) = self.confirm_on_update.take() {
            self.do_confirm(secondary, window, cx);
        }
        cx.notify();
    }

    pub fn query(&self, cx: &App) -> String {
        match &self.head {
            Head::Editor(editor) => editor.text(cx),
            Head::Empty(_) => "".to_string(),
        }
    }

    pub fn set_query(&self, query: &str, window: &mut Window, cx: &mut App) {
        if let Head::Editor(editor) = &self.head {
            editor.set_text(query, window, cx);
            editor.move_selection_to_end(window, cx);
        }
    }

    fn scroll_to_item_index(&mut self, ix: usize) {
        match &mut self.element_container {
            ElementContainer::List(state) => state.scroll_to_reveal_item(ix),
            ElementContainer::UniformList(scroll_handle) => {
                scroll_handle.scroll_to_item(ix, ScrollStrategy::Nearest)
            }
        }
    }

    fn render_element(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        ix: usize,
    ) -> impl IntoElement + use<D> {
        let item_bounds = self.item_bounds.clone();

        div()
            .id(("item", ix))
            .cursor_pointer()
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        item_bounds.borrow_mut().insert(ix, bounds);
                    },
                    |_bounds, _state, _window, _cx| {},
                )
                .size_full()
                .absolute()
                .top_0()
                .left_0(),
            )
            .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                this.handle_click(ix, event.modifiers().secondary(), window, cx)
            }))
            // As of this writing, GPUI intercepts `ctrl-[mouse-event]`s on macOS
            // and produces right mouse button events. This matches platforms norms
            // but means that UIs which depend on holding ctrl down (such as the tab
            // switcher) can't be clicked on. Hence, this handler.
            .on_mouse_up(
                MouseButton::Right,
                cx.listener(move |this, event: &MouseUpEvent, window, cx| {
                    // We specifically want to use the platform key here, as
                    // ctrl will already be held down for the tab switcher.
                    this.handle_click(ix, event.modifiers.platform, window, cx)
                }),
            )
            .children(self.delegate.render_match(
                ix,
                ix == self.delegate.selected_index(),
                window,
                cx,
            ))
            .when(
                self.delegate.separators_after_indices().contains(&ix),
                |picker| {
                    picker
                        .border_color(cx.theme().colors().border_variant)
                        .border_b_1()
                        .py(px(-1.0))
                },
            )
    }

    fn render_element_container(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let sizing_behavior = if self.max_height.is_some() {
            ListSizingBehavior::Infer
        } else {
            ListSizingBehavior::Auto
        };

        match &self.element_container {
            ElementContainer::UniformList(scroll_handle) => uniform_list(
                "candidates",
                self.delegate.match_count(),
                cx.processor(move |picker, visible_range: Range<usize>, window, cx| {
                    visible_range
                        .map(|ix| picker.render_element(window, cx, ix))
                        .collect()
                }),
            )
            .with_sizing_behavior(sizing_behavior)
            .when_some(self.widest_item, |el, widest_item| {
                el.with_width_from_item(Some(widest_item))
            })
            .flex_grow()
            .py_1()
            .track_scroll(&scroll_handle)
            .into_any_element(),
            ElementContainer::List(state) => list(
                state.clone(),
                cx.processor(|this, ix, window, cx| {
                    this.render_element(window, cx, ix).into_any_element()
                }),
            )
            .with_sizing_behavior(sizing_behavior)
            .flex_grow()
            .py_2()
            .into_any_element(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn logical_scroll_top_index(&self) -> usize {
        match &self.element_container {
            ElementContainer::List(state) => state.logical_scroll_top().item_ix,
            ElementContainer::UniformList(scroll_handle) => {
                scroll_handle.logical_scroll_top_index()
            }
        }
    }
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for Picker<D> {}
impl<D: PickerDelegate> ModalView for Picker<D> {}

impl<D: PickerDelegate> Render for Picker<D> {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_wide_window = window_size.width / rem_size > rems_from_px(800.).0;

        let aside = self.delegate.documentation_aside(window, cx);

        let editor_position = self.delegate.editor_position();
        let picker_bounds = self.picker_bounds.clone();
        let menu = v_flex()
            .key_context("Picker")
            .size_full()
            .when_some(self.width, |el, width| el.w(width))
            .overflow_hidden()
            .child(
                canvas(
                    move |bounds, _window, _cx| {
                        picker_bounds.set(Some(bounds));
                    },
                    |_bounds, _state, _window, _cx| {},
                )
                .size_full()
                .absolute()
                .top_0()
                .left_0(),
            )
            // This is a bit of a hack to remove the modal styling when we're rendering the `Picker`
            // as a part of a modal rather than the entire modal.
            //
            // We should revisit how the `Picker` is styled to make it more composable.
            .when(self.is_modal, |this| this.elevation_3(cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::secondary_confirm))
            .on_action(cx.listener(Self::confirm_completion))
            .on_action(cx.listener(Self::confirm_input))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::Start {
                        Some(self.delegate.render_editor(&editor.clone(), window, cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            })
            .when(self.delegate.match_count() > 0, |el| {
                el.child(
                    v_flex()
                        .id("element-container")
                        .relative()
                        .flex_grow()
                        .when_some(self.max_height, |div, max_h| div.max_h(max_h))
                        .overflow_hidden()
                        .children(self.delegate.render_header(window, cx))
                        .child(self.render_element_container(cx))
                        .when(self.show_scrollbar, |this| {
                            let base_scrollbar_config =
                                Scrollbars::new(ScrollAxes::Vertical).width_sm();

                            this.map(|this| match &self.element_container {
                                ElementContainer::List(state) => this.custom_scrollbars(
                                    base_scrollbar_config.tracked_scroll_handle(state),
                                    window,
                                    cx,
                                ),
                                ElementContainer::UniformList(state) => this.custom_scrollbars(
                                    base_scrollbar_config.tracked_scroll_handle(state),
                                    window,
                                    cx,
                                ),
                            })
                        }),
                )
            })
            .when(self.delegate.match_count() == 0, |el| {
                el.when_some(self.delegate.no_matches_text(window, cx), |el, text| {
                    el.child(
                        v_flex().flex_grow().py_2().child(
                            ListItem::new("empty_state")
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .disabled(true)
                                .child(Label::new(text).color(Color::Muted)),
                        ),
                    )
                })
            })
            .children(self.delegate.render_footer(window, cx))
            .children(match &self.head {
                Head::Editor(editor) => {
                    if editor_position == PickerEditorPosition::End {
                        Some(self.delegate.render_editor(&editor.clone(), window, cx))
                    } else {
                        None
                    }
                }
                Head::Empty(empty_head) => Some(div().child(empty_head.clone())),
            });

        let Some(aside) = aside else {
            return menu;
        };

        let render_aside = |aside: DocumentationAside, cx: &mut Context<Self>| {
            WithRemSize::new(ui_font_size)
                .occlude()
                .elevation_2(cx)
                .w_full()
                .p_2()
                .overflow_hidden()
                .when(is_wide_window, |this| this.max_w_96())
                .when(!is_wide_window, |this| this.max_w_48())
                .child((aside.render)(cx))
        };

        if is_wide_window {
            let aside_index = self.delegate.documentation_aside_index();
            let picker_bounds = self.picker_bounds.get();
            let item_bounds =
                aside_index.and_then(|ix| self.item_bounds.borrow().get(&ix).copied());

            let item_position = match (picker_bounds, item_bounds) {
                (Some(picker_bounds), Some(item_bounds)) => {
                    let relative_top = item_bounds.origin.y - picker_bounds.origin.y;
                    let height = item_bounds.size.height;
                    Some((relative_top, height))
                }
                _ => None,
            };

            div()
                .relative()
                .child(menu)
                // Only render the aside once we have bounds to avoid flicker
                .when_some(item_position, |this, (top, height)| {
                    this.child(
                        h_flex()
                            .absolute()
                            .when(aside.side == DocumentationSide::Left, |el| {
                                el.right_full().mr_1()
                            })
                            .when(aside.side == DocumentationSide::Right, |el| {
                                el.left_full().ml_1()
                            })
                            .top(top)
                            .h(height)
                            .child(render_aside(aside, cx)),
                    )
                })
        } else {
            v_flex()
                .w_full()
                .gap_1()
                .justify_end()
                .child(render_aside(aside, cx))
                .child(menu)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use settings::SettingsStore;

    struct TestItem {
        id: String,
        text: String,
    }

    struct TestDelegate {
        items: Vec<TestItem>,
        matches: Vec<usize>,
        selected_index: usize,
    }

    impl TestDelegate {
        fn new(items: Vec<(&str, &str)>) -> Self {
            let items: Vec<TestItem> = items
                .into_iter()
                .map(|(id, text)| TestItem {
                    id: id.to_string(),
                    text: text.to_string(),
                })
                .collect();
            let matches: Vec<usize> = (0..items.len()).collect();
            Self {
                items,
                matches,
                selected_index: 0,
            }
        }
    }

    impl PickerDelegate for TestDelegate {
        type ListItem = ListItem;

        fn match_count(&self) -> usize {
            self.matches.len()
        }

        fn selected_index(&self) -> usize {
            self.selected_index
        }

        fn set_selected_index(
            &mut self,
            ix: usize,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
            self.selected_index = ix;
        }

        fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
            "Search...".into()
        }

        fn match_stable_id(&self, ix: usize) -> Option<String> {
            self.matches
                .get(ix)
                .and_then(|&item_ix| self.items.get(item_ix))
                .map(|item| item.id.clone())
        }

        fn find_match_by_stable_id(&self, stable_id: &str) -> Option<usize> {
            self.matches.iter().position(|&item_ix| {
                self.items
                    .get(item_ix)
                    .is_some_and(|item| item.id == stable_id)
            })
        }

        fn update_matches(
            &mut self,
            query: String,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Task<()> {
            if query.is_empty() {
                self.matches = (0..self.items.len()).collect();
            } else {
                self.matches = self
                    .items
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.text.to_lowercase().contains(&query.to_lowercase()))
                    .map(|(ix, _)| ix)
                    .collect();
            }
            Task::ready(())
        }

        fn confirm(
            &mut self,
            _secondary: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
        }

        fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

        fn render_match(
            &self,
            ix: usize,
            selected: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            let item_ix = self.matches.get(ix)?;
            let item = self.items.get(*item_ix)?;
            Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(item.text.clone())),
            )
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    #[gpui::test]
    fn test_selection_preserved_when_query_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                TestDelegate::new(vec![
                    ("a", "apple"),
                    ("b", "box"),
                    ("c", "cherry"),
                    ("d", "door"),
                ]),
                window,
                cx,
            )
        });

        // Initial state: first item selected
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.selected_index(), 0);
                assert_eq!(picker.delegate.match_count(), 4);
            })
            .unwrap();

        // Navigate to third item (cherry)
        picker
            .update(cx, |picker, window, cx| {
                picker.select_index_sticky(2, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), 2);
            })
            .unwrap();

        // Type a query that still includes cherry (contains "r")
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("r".to_string(), window, cx);
            })
            .unwrap();

        // Cherry should still be selected (it matches "r" and has stable_id "c")
        picker
            .update(cx, |picker, _window, _cx| {
                // "r" matches: cherry (c), door (d)
                assert_eq!(picker.delegate.match_count(), 2);
                // cherry should still be selected - find its new index
                let cherry_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "c")
                    .unwrap();
                assert_eq!(picker.delegate.selected_index(), cherry_index);
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_selection_reset_when_item_no_longer_matches(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                TestDelegate::new(vec![
                    ("a", "apple"),
                    ("b", "box"),
                    ("c", "cherry"),
                    ("d", "door"),
                ]),
                window,
                cx,
            )
        });

        // Navigate to box (index 1)
        picker
            .update(cx, |picker, window, cx| {
                picker.select_index_sticky(1, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), 1);
            })
            .unwrap();

        // Type a query that excludes box
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("apple".to_string(), window, cx);
            })
            .unwrap();

        // Box is no longer in results, selection should reset
        picker
            .update(cx, |picker, _window, _cx| {
                // Only "apple" matches
                assert_eq!(picker.delegate.match_count(), 1);
                // Selection should be clamped to valid range
                assert_eq!(picker.delegate.selected_index(), 0);
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_selection_preserved_when_deleting_query_characters(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                TestDelegate::new(vec![
                    ("a", "apple"),
                    ("b", "box"),
                    ("c", "cherry"),
                    ("d", "door"),
                ]),
                window,
                cx,
            )
        });

        // Type a query
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("o".to_string(), window, cx);
            })
            .unwrap();

        // "o" matches: box, door (2 items)
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 2);
            })
            .unwrap();

        // Navigate to door (last item in filtered list)
        picker
            .update(cx, |picker, window, cx| {
                let door_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "d")
                    .unwrap();
                picker.select_index_sticky(door_index, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), door_index);
            })
            .unwrap();

        // Delete the query (back to empty)
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("".to_string(), window, cx);
            })
            .unwrap();

        // Door should still be selected
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 4);
                let door_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "d")
                    .unwrap();
                assert_eq!(picker.delegate.selected_index(), door_index);
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_programmatic_selection_not_sticky(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                TestDelegate::new(vec![
                    ("a", "apple"),
                    ("b", "box"),
                    ("c", "cherry"),
                    ("d", "door"),
                ]),
                window,
                cx,
            )
        });

        // Use programmatic selection (not sticky)
        picker
            .update(cx, |picker, window, cx| {
                picker.set_selected_index(2, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), 2);
            })
            .unwrap();

        // Type a query - since selection was programmatic, it should NOT be preserved
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("o".to_string(), window, cx);
            })
            .unwrap();

        // Selection should be clamped but not restored to cherry
        picker
            .update(cx, |picker, _window, _cx| {
                // "o" matches: box, door (2 items)
                assert_eq!(picker.delegate.match_count(), 2);
                // Index 2 would be out of bounds, so it's clamped to 1
                assert!(picker.delegate.selected_index() <= 1);
            })
            .unwrap();
    }

    struct BestMatchDelegate {
        items: Vec<TestItem>,
        matches: Vec<usize>,
        selected_index: usize,
    }

    impl BestMatchDelegate {
        fn new(items: Vec<(&str, &str)>) -> Self {
            let items: Vec<TestItem> = items
                .into_iter()
                .map(|(id, text)| TestItem {
                    id: id.to_string(),
                    text: text.to_string(),
                })
                .collect();
            let matches: Vec<usize> = (0..items.len()).collect();
            Self {
                items,
                matches,
                selected_index: 0,
            }
        }
    }

    impl PickerDelegate for BestMatchDelegate {
        type ListItem = ListItem;

        fn match_count(&self) -> usize {
            self.matches.len()
        }

        fn selected_index(&self) -> usize {
            self.selected_index
        }

        fn set_selected_index(
            &mut self,
            ix: usize,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
            self.selected_index = ix;
        }

        fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
            "Search...".into()
        }

        fn match_stable_id(&self, ix: usize) -> Option<String> {
            self.matches
                .get(ix)
                .and_then(|&item_ix| self.items.get(item_ix))
                .map(|item| item.id.clone())
        }

        fn find_match_by_stable_id(&self, stable_id: &str) -> Option<usize> {
            self.matches.iter().position(|&item_ix| {
                self.items
                    .get(item_ix)
                    .is_some_and(|item| item.id == stable_id)
            })
        }

        fn update_matches(
            &mut self,
            query: String,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Task<()> {
            if query.is_empty() {
                self.matches = (0..self.items.len()).collect();
            } else {
                self.matches = self
                    .items
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.text.to_lowercase().contains(&query.to_lowercase()))
                    .map(|(ix, _)| ix)
                    .collect();
            }

            // This mimics OutlineViewDelegate behavior: always select "best" match
            // (in this case, just pick the first match)
            if !self.matches.is_empty() {
                self.selected_index = 0;
            }

            Task::ready(())
        }

        fn confirm(
            &mut self,
            _secondary: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
        }

        fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

        fn render_match(
            &self,
            ix: usize,
            selected: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            let item_ix = self.matches.get(ix)?;
            let item = self.items.get(*item_ix)?;
            Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(item.text.clone())),
            )
        }
    }

    #[gpui::test]
    fn test_selection_preserved_when_query_shortened(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                TestDelegate::new(vec![
                    ("a", "somethingNotifier"),
                    ("b", "anotherNotifier"),
                    ("c", "notifyHandler"),
                ]),
                window,
                cx,
            )
        });

        // Type initial query "otif" - matches all 3
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otif".to_string(), window, cx);
            })
            .unwrap();

        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 3);
            })
            .unwrap();

        // Narrow down to "otifier" - only matches somethingNotifier and anotherNotifier
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otifier".to_string(), window, cx);
            })
            .unwrap();

        picker
            .update(cx, |picker, _window, _cx| {
                // "otifier" matches: somethingNotifier, anotherNotifier (not "notifyHandler")
                assert_eq!(picker.delegate.match_count(), 2);
            })
            .unwrap();

        // Select somethingNotifier (first item)
        picker
            .update(cx, |picker, window, cx| {
                let something_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "a")
                    .unwrap();
                picker.select_index_sticky(something_index, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), something_index);
            })
            .unwrap();

        // Delete "ier" - query becomes "otif", now matches all 3 again
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otif".to_string(), window, cx);
            })
            .unwrap();

        // somethingNotifier should still be selected, NOT notifyHandler
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 3);
                let something_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "a")
                    .unwrap();
                assert_eq!(
                    picker.delegate.selected_index(),
                    something_index,
                    "Expected somethingNotifier to remain selected, but selection changed"
                );
            })
            .unwrap();
    }

    struct ReorderingDelegate {
        items: Vec<TestItem>,
        matches: Vec<usize>,
        selected_index: usize,
    }

    impl ReorderingDelegate {
        fn new(items: Vec<(&str, &str)>) -> Self {
            let items: Vec<TestItem> = items
                .into_iter()
                .map(|(id, text)| TestItem {
                    id: id.to_string(),
                    text: text.to_string(),
                })
                .collect();
            let matches: Vec<usize> = (0..items.len()).collect();
            Self {
                items,
                matches,
                selected_index: 0,
            }
        }
    }

    impl PickerDelegate for ReorderingDelegate {
        type ListItem = ListItem;

        fn match_count(&self) -> usize {
            self.matches.len()
        }

        fn selected_index(&self) -> usize {
            self.selected_index
        }

        fn set_selected_index(
            &mut self,
            ix: usize,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
            self.selected_index = ix;
        }

        fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
            "Search...".into()
        }

        fn match_stable_id(&self, ix: usize) -> Option<String> {
            self.matches
                .get(ix)
                .and_then(|&item_ix| self.items.get(item_ix))
                .map(|item| item.id.clone())
        }

        fn find_match_by_stable_id(&self, stable_id: &str) -> Option<usize> {
            self.matches.iter().position(|&item_ix| {
                self.items
                    .get(item_ix)
                    .is_some_and(|item| item.id == stable_id)
            })
        }

        fn update_matches(
            &mut self,
            query: String,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Task<()> {
            if query.is_empty() {
                self.matches = (0..self.items.len()).collect();
            } else {
                self.matches = self
                    .items
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| item.text.to_lowercase().contains(&query.to_lowercase()))
                    .map(|(ix, _)| ix)
                    .collect();

                // Simulate fuzzy matching that returns results in a different order
                // based on "score" - reverse the order for shorter queries
                if query.len() <= 4 {
                    self.matches.reverse();
                }
            }

            // Always select "best" match (first in list)
            if !self.matches.is_empty() {
                self.selected_index = 0;
            }

            Task::ready(())
        }

        fn confirm(
            &mut self,
            _secondary: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
        }

        fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

        fn render_match(
            &self,
            ix: usize,
            selected: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            let item_ix = self.matches.get(ix)?;
            let item = self.items.get(*item_ix)?;
            Some(
                ListItem::new(ix)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .toggle_state(selected)
                    .child(Label::new(item.text.clone())),
            )
        }
    }

    #[gpui::test]
    fn test_selection_preserved_when_match_order_changes(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                ReorderingDelegate::new(vec![
                    ("a", "somethingNotifier"),
                    ("b", "anotherNotifier"),
                    ("c", "notifyHandler"),
                ]),
                window,
                cx,
            )
        });

        // Type longer query "otifier" - matches somethingNotifier, anotherNotifier
        // With length > 4, order is normal: [0, 1] (somethingNotifier first)
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otifier".to_string(), window, cx);
            })
            .unwrap();

        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 2);
                // Normal order: somethingNotifier (0), anotherNotifier (1)
                assert_eq!(picker.delegate.matches, vec![0, 1]);
            })
            .unwrap();

        // Select somethingNotifier (index 0 in matches)
        picker
            .update(cx, |picker, window, cx| {
                picker.select_index_sticky(0, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), 0);
            })
            .unwrap();

        // Type shorter query "otif" - matches all 3, but order is REVERSED
        // With length <= 4, order becomes: [2, 1, 0] (notifyHandler first!)
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otif".to_string(), window, cx);
            })
            .unwrap();

        // somethingNotifier should still be selected even though:
        // 1. The delegate tried to set selected_index to 0 (which is now notifyHandler)
        // 2. The match order changed
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 3);
                // Reversed order: notifyHandler (2), anotherNotifier (1), somethingNotifier (0)
                assert_eq!(picker.delegate.matches, vec![2, 1, 0]);

                // somethingNotifier (item 0) should still be selected, which is now at match index 2
                let something_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "a")
                    .unwrap();
                assert_eq!(something_index, 2); // It's at position 2 now

                assert_eq!(
                    picker.delegate.selected_index(),
                    something_index,
                    "Expected somethingNotifier to remain selected at new index, but selection is at {}",
                    picker.delegate.selected_index()
                );
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_selection_preserved_when_query_shortened_with_best_match_delegate(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                BestMatchDelegate::new(vec![
                    ("a", "somethingNotifier"),
                    ("b", "anotherNotifier"),
                    ("c", "notifyHandler"),
                ]),
                window,
                cx,
            )
        });

        // Type initial query "otif" - matches all 3
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otif".to_string(), window, cx);
            })
            .unwrap();

        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 3);
            })
            .unwrap();

        // Narrow down to "otifier" - only matches somethingNotifier and anotherNotifier
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otifier".to_string(), window, cx);
            })
            .unwrap();

        picker
            .update(cx, |picker, _window, _cx| {
                // "otifier" matches: somethingNotifier, anotherNotifier (not "notifyHandler")
                assert_eq!(picker.delegate.match_count(), 2);
            })
            .unwrap();

        // Select somethingNotifier (first item)
        picker
            .update(cx, |picker, window, cx| {
                let something_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "a")
                    .unwrap();
                picker.select_index_sticky(something_index, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), something_index);
            })
            .unwrap();

        // Delete "ier" - query becomes "otif", now matches all 3 again
        // The BestMatchDelegate will try to set selection to 0 (first match)
        // but the picker should restore it to somethingNotifier via stable ID
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("otif".to_string(), window, cx);
            })
            .unwrap();

        // somethingNotifier should still be selected, NOT notifyHandler
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.match_count(), 3);
                let something_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "a")
                    .unwrap();
                assert_eq!(
                    picker.delegate.selected_index(),
                    something_index,
                    "Expected somethingNotifier to remain selected, but selection changed"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    fn test_delegate_that_sets_selection_in_update_matches(cx: &mut TestAppContext) {
        init_test(cx);

        let picker = cx.add_window(|window, cx| {
            Picker::uniform_list(
                BestMatchDelegate::new(vec![
                    ("a", "apple"),
                    ("b", "box"),
                    ("c", "cherry"),
                    ("d", "door"),
                ]),
                window,
                cx,
            )
        });

        // Initial state: first item selected
        picker
            .update(cx, |picker, _window, _cx| {
                assert_eq!(picker.delegate.selected_index(), 0);
                assert_eq!(picker.delegate.match_count(), 4);
            })
            .unwrap();

        // Navigate to cherry (index 2) using sticky selection
        picker
            .update(cx, |picker, window, cx| {
                picker.select_index_sticky(2, None, true, window, cx);
                assert_eq!(picker.delegate.selected_index(), 2);
            })
            .unwrap();

        // Type a query that still includes cherry
        // The delegate will set selected_index to 0 (best match), but the picker
        // should restore it to cherry via stable ID
        picker
            .update(cx, |picker, window, cx| {
                picker.update_matches("r".to_string(), window, cx);
            })
            .unwrap();

        // Cherry should still be selected even though delegate tried to select first match
        picker
            .update(cx, |picker, _window, _cx| {
                // "r" matches: cherry (c), door (d)
                assert_eq!(picker.delegate.match_count(), 2);
                // cherry should still be selected - find its new index
                let cherry_index = picker
                    .delegate
                    .matches
                    .iter()
                    .position(|&ix| picker.delegate.items[ix].id == "c")
                    .unwrap();
                assert_eq!(picker.delegate.selected_index(), cherry_index);
            })
            .unwrap();
    }
}
