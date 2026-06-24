use anyhow::Result;
use gpui::Action;
use gpui::{
    AnyElement, App, Bounds, ClickEvent, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, ListSizingBehavior, ListState, MouseButton, MouseUpEvent, Pixels, ScrollStrategy,
    Task, UniformListScrollHandle, Window, actions, canvas, div, list, prelude::*, uniform_list,
};
use head::Head;
use project::Project;
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    cell::Cell, cell::RefCell, collections::HashMap, ops::Range, rc::Rc, sync::Arc, time::Duration,
};
use ui::{ContextMenu, Divider, DocumentationAside, PopoverMenuHandle, prelude::*, v_flex};
use ui_input::ErasedEditorEvent;
use util::ResultExt;
use workspace::ModalView;
use zed_actions::editor::{MoveDown, MoveUp};

mod footer;
mod head;
pub mod highlighted_match_with_paths;
pub mod parts;
mod persistence;
pub mod popover_menu;
mod preview;
mod render;
mod shape;

use crate::shape::RelativeHeight;
use crate::shape::RelativeWidth;
pub use footer::PickerAction;
pub use language::{HighlightedText, HighlightedTextBuilder};
pub use preview::MatchLocation;
pub use preview::Preview;
pub use preview::PreviewSource;
pub use preview::Update as PreviewUpdate;
pub use ui_input::ErasedEditor;

pub const DEFAULT_MODAL_WIDTH: Rems = Rems(34.0);
pub const DEFAULT_MODAL_MAX_HEIGHT: Rems = Rems(24.0);

enum ElementContainer {
    List(ListState),
    UniformList(UniformListScrollHandle),
}

pub enum Direction {
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollBehavior {
    RevealSelected,
    PreserveOffset,
}

actions!(
    picker,
    [
        /// Confirms the selected completion in the picker.
        ConfirmCompletion,
        /// Toggles the preview between hidden and visible.
        TogglePreview,
        /// Shows the preview to the right of the results.
        SetPreviewRight,
        /// Shows the preview below the results.
        SetPreviewBelow,
        /// Hides the preview.
        SetPreviewHidden,
        /// Opens the footer's actions menu.
        ToggleActionsMenu,
        /// Take the picker's content and open it in a multibuffer
        ToMultiBuffer,
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

#[derive(Clone, Copy)]
enum Presentation {
    /// A self-contained modal: draws its own elevated background and dismisses
    /// when it loses focus. May optionally be resized (persisting its size);
    /// resizing only makes sense for modals.
    Modal { resizable: bool },
    /// A popover attached to a menu/trigger: draws its own elevated background
    /// and dismisses when it loses focus, but is never resizable.
    Popover,
    /// Embedded inside a larger container (e.g. another modal) that provides its
    /// own chrome and handles dismissal.
    Embedded,
}

/// The default size for a given preview layout. With the preview hidden the
/// picker uses its standard size; showing a preview expands it to the larger
/// "telescope" size so the results pane isn't cramped beside the preview.
fn default_shape_for_layout(hidden: shape::Centered, layout: preview::Layout) -> shape::Centered {
    match layout {
        preview::Layout::Hidden => hidden,
        preview::Layout::Right | preview::Layout::Below => shape::Centered::default(),
    }
}

pub struct Picker<D: PickerDelegate> {
    pub delegate: D,
    element_container: ElementContainer,
    head: Head,
    preview: Option<Preview>,
    pending_update_matches: Option<PendingUpdateMatches>,
    confirm_on_update: Option<bool>,
    shape: shape::Shape,
    /// The size the picker opens at (and resets to). Defaults depend on whether
    /// the picker has a preview; see [`Picker::initial_width`] / [`Picker::max_height`].
    default_shape: shape::Centered,
    size_bounds: shape::SizeBounds,
    /// An external control to display a scrollbar in the `Picker`.
    show_scrollbar: bool,
    /// How the picker is presented, which controls its chrome and behavior.
    presentation: Presentation,
    /// Bounds tracking for the picker container (for aside positioning)
    picker_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// Bounds tracking for items (for aside positioning) - maps item index to bounds
    item_bounds: Rc<RefCell<HashMap<usize, Bounds<Pixels>>>>,
    shape_loaded_from_persistence: bool,
    /// Handle for the default footer's Actions popover menu. Used to keep the
    /// picker open while that menu has focus.
    actions_menu_handle: PopoverMenuHandle<ContextMenu>,
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

    /// Name of the picker, this is the key for serialization. We could use the
    /// typename of the delegate but then a rename would break persistence.
    fn name() -> &'static str;
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
        &self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        true
    }
    fn select_on_hover(&self) -> bool {
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

    /// Prevent closing the modal on clicking in a popover menu that portrudes out
    /// This is already set by the Actions menu from the picker, this is here to
    /// support extra menus added by the delegate.
    fn has_another_open_menu(&self, _window: &Window, _cx: &App) -> bool {
        false
    }

    /// An optional control rendered at the trailing edge of the search bar, e.g.
    /// a filter toggle. Returning `Some` is the easy way to add such a control;
    /// for full control over the search bar, override [`Self::render_editor`].
    fn searchbar_trailer(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        None
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
                    .child(div().flex_1().child(editor.render(window, cx)))
                    .children(self.searchbar_trailer(window, cx)),
            )
            .when(
                self.editor_position() == PickerEditorPosition::Start,
                |this| this.child(Divider::horizontal()),
            )
    }

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<PreviewUpdate> {
        None
    }

    /// Called on the delegate when opening a preview to the side. Delegates can
    /// then change how much space they use for rendering the match
    fn preview_layout_changed(&mut self, _layout_is_horizontal: bool) {}

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

    /// Overrides the picker's footer.
    ///
    /// Note there normally isn't a footer unless this is set or the picker has
    /// a preview. If the picker has a preview add actions to it using picker_actions.
    fn render_footer(
        &self,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        None
    }

    /// Make this non-empty to have an `Actions` menu show up in the footer
    fn actions_menu(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Vec<footer::PickerAction> {
        Vec::new()
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

        Self::new(delegate, ContainerKind::UniformList, head, None, window, cx)
    }

    /// A picker similar to [`uniform_list()`](Self::uniform_list) however this picker has a
    /// preview window where it shows extra information.
    pub fn uniform_list_with_preview(
        delegate: D,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(window, cx),
            Self::on_input_editor_event,
            window,
            cx,
        );

        let preview = Preview::new_editor(project, window, cx);
        Self::new(
            delegate,
            ContainerKind::UniformList,
            head,
            Some(preview),
            window,
            cx,
        )
    }

    /// A picker similar to [`list()`](Self::list) (variable-height rows) but with
    /// a preview window. Use this instead of [`uniform_list_with_preview()`](Self::uniform_list_with_preview)
    /// when [`PickerDelegate::render_match`] can return rows of different heights
    /// (e.g. section headers and separators interleaved with matches).
    pub fn list_with_preview(
        delegate: D,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let head = Head::editor(
            delegate.placeholder_text(window, cx),
            Self::on_input_editor_event,
            window,
            cx,
        );

        let preview = Preview::new_editor(project, window, cx);
        Self::new(
            delegate,
            ContainerKind::List,
            head,
            Some(preview),
            window,
            cx,
        )
    }

    /// A picker, which displays its matches using `gpui::uniform_list`, all matches should have the same height.
    /// If `PickerDelegate::render_match` can return items with different heights, use `Picker::list`.
    pub fn nonsearchable_uniform_list(
        delegate: D,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let head = Head::empty(Self::on_empty_head_blur, window, cx);

        Self::new(delegate, ContainerKind::UniformList, head, None, window, cx)
    }

    /// A picker, which displays its matches using `gpui::list`, matches can have different heights.
    /// The picker allows the user to perform search items by text.
    /// If `PickerDelegate::render_match` only returns items with the same height, use `Picker::uniform_list` as its implementation is optimized for that.
    pub fn nonsearchable_list(delegate: D, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let head = Head::empty(Self::on_empty_head_blur, window, cx);

        Self::new(delegate, ContainerKind::List, head, None, window, cx)
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

        Self::new(delegate, ContainerKind::List, head, None, window, cx)
    }

    fn new(
        delegate: D,
        container: ContainerKind,
        head: Head,
        mut preview: Option<Preview>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let element_container = Self::create_element_container(container);
        if let Some(preview) = &mut preview {
            preview.layout = persistence::load_last_preview_layout(D::name(), cx)
                .log_err()
                .flatten()
                .unwrap_or_default();
        };
        let has_preview = preview.is_some();
        let persisted_shape =
            persistence::try_load_shape(D::name(), preview.as_ref().map(|p| p.layout), cx)
                .log_err()
                .flatten();
        // Every picker opens at the standard "simple" size: a fixed width and a
        // standard max height it shrinks below when there's little content.
        // Showing a preview expands it to the larger "telescope" size (see
        // `default_shape_for_layout`).
        let default_shape = shape::Centered::simple();
        let initial_layout = preview
            .as_ref()
            .map(|p| p.layout)
            .unwrap_or(preview::Layout::Hidden);
        let mut size_bounds = shape::SizeBounds::default();
        // For a plain picker the whole-picker minimum is just its opening width,
        // so it can't be resized/clamped narrower than it opens. Preview pickers
        // keep the standard per-pane minimums.
        if !has_preview && let Some(width) = default_shape.width.as_rems() {
            size_bounds.min_results.width = width;
        }
        let mut this = Self {
            delegate,
            head,
            element_container,
            pending_update_matches: None,
            confirm_on_update: None,
            preview,
            shape_loaded_from_persistence: persisted_shape.is_some(),
            shape: persisted_shape.unwrap_or_else(|| {
                shape::Shape::HorizontallyCentered(default_shape_for_layout(
                    default_shape,
                    initial_layout,
                ))
            }),
            default_shape,
            show_scrollbar: false,
            presentation: Presentation::Modal {
                resizable: has_preview,
            },
            picker_bounds: Rc::new(Cell::new(None)),
            item_bounds: Rc::new(RefCell::new(HashMap::default())),
            size_bounds,
            actions_menu_handle: PopoverMenuHandle::default(),
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

    /// Overrides the width the picker opens at (and resets to). Plain pickers
    /// default to [`DEFAULT_MODAL_WIDTH`]; only call this for pickers that need a
    /// different width (e.g. narrow popover selectors).
    ///
    /// For a plain (no-preview) picker the opening width is also its minimum, so
    /// it can't be resized or clamped narrower than it opens.
    pub fn initial_width(mut self, width: impl Into<RelativeWidth>) -> Self {
        let width = width.into();
        self.default_shape.width = width;
        if !self.shape_loaded_from_persistence {
            self.shape.set_initial_width(width);
        }
        // A plain picker's whole-picker minimum tracks its opening width. Preview
        // pickers keep the standard per-pane minimums.
        if self.preview.is_none()
            && let Some(rems) = width.as_rems()
        {
            self.size_bounds.min_results.width = rems;
        }
        self
    }

    /// Overrides the picker's max height. Plain pickers default to
    /// [`DEFAULT_MODAL_MAX_HEIGHT`] and shrink below it to fit their content;
    /// only call this for pickers that want a different cap (e.g. the outline
    /// view, which is taller).
    pub fn max_height(mut self, height: impl Into<RelativeHeight>) -> Self {
        let height = height.into();
        self.default_shape.height = height;
        if !self.shape_loaded_from_persistence {
            self.shape.set_initial_height(height);
        }
        self
    }

    /// Whether the picker fills its full height (preview visible) or shrinks to
    /// fit its content, treating the height as a maximum (no preview visible).
    fn fill_height(&self) -> bool {
        self.preview
            .as_ref()
            .is_some_and(|preview| preview.layout != preview::Layout::Hidden)
    }

    pub fn show_scrollbar(mut self, show_scrollbar: bool) -> Self {
        self.show_scrollbar = show_scrollbar;
        self
    }

    /// Presents the picker as embedded inside a larger container that provides
    /// its own chrome and dismissal, rather than the default self-contained
    /// modal. Use [`popover`](Self::popover) instead for menu-attached pickers.
    pub fn embedded(mut self) -> Self {
        self.presentation = Presentation::Embedded;
        self
    }

    /// Presents the picker as a popover surface: it draws its own elevated
    /// background (like a modal) but is not resizable. Use this for pickers
    /// shown inside a menu/popover.
    pub fn popover(mut self) -> Self {
        self.set_popover();
        self
    }

    pub(crate) fn set_popover(&mut self) {
        self.presentation = Presentation::Popover;
    }

    /// Controls whether the user can drag to resize the picker (and whether its
    /// size is persisted). Only applies to modal pickers; no-op otherwise.
    /// Defaults to `true` for pickers with a preview and `false` otherwise.
    pub fn resizable(mut self, resizable: bool) -> Self {
        if let Presentation::Modal { resizable: r } = &mut self.presentation {
            *r = resizable;
        }
        self
    }

    /// Whether the picker draws its own elevated background and dismisses on
    /// blur (modals and popovers, but not embedded pickers).
    fn draws_own_container(&self) -> bool {
        matches!(
            self.presentation,
            Presentation::Modal { .. } | Presentation::Popover
        )
    }

    /// Whether the picker can be resized (only ever true for modals).
    fn is_resizable(&self) -> bool {
        matches!(self.presentation, Presentation::Modal { resizable: true })
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
    pub fn set_selected_index(
        &mut self,
        mut ix: usize,
        fallback_direction: Option<Direction>,
        scroll_to_index: bool,
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

        if previous_index != current_index {
            if let Some(action) = self.delegate.selected_index_changed(ix, window, cx) {
                action(window, cx);
            }
            if let Some(preview) = &mut self.preview
                && let Some(update) = self.delegate.try_get_preview_data_for_match(cx)
            {
                preview.update(update, window, cx);
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
            self.set_selected_index(ix, Some(Direction::Down), true, window, cx);
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
            self.set_selected_index(ix, Some(Direction::Up), true, window, cx);
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
            self.set_selected_index(0, Some(Direction::Down), true, window, cx);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.delegate.match_count();
        if count > 0 {
            self.set_selected_index(count - 1, Some(Direction::Up), true, window, cx);
            cx.notify();
        }
    }

    pub fn cycle_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.delegate.match_count();
        let index = self.delegate.selected_index();
        let new_index = if index + 1 == count { 0 } else { index + 1 };
        self.set_selected_index(new_index, Some(Direction::Down), true, window, cx);
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

    fn set_preview_right(
        &mut self,
        _: &SetPreviewRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_preview_layout(preview::Layout::Right, window, cx);
    }

    fn set_preview_below(
        &mut self,
        _: &SetPreviewBelow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_preview_layout(preview::Layout::Below, window, cx);
    }

    fn set_preview_hidden(
        &mut self,
        _: &SetPreviewHidden,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_preview_layout(preview::Layout::Hidden, window, cx);
    }

    fn toggle_actions_menu(
        &mut self,
        _: &ToggleActionsMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.actions_menu_handle.toggle(window, cx);
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
        if !self.delegate.can_select(ix, window, cx) {
            return;
        }
        self.set_selected_index(ix, None, false, window, cx);
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
                // Opening a footer/search-bar menu blurs the editor; don't
                // dismiss the picker while such a menu is open/focused.
                let menu_focused = self.actions_menu_handle.is_focused(window, cx)
                    || self.actions_menu_handle.is_deployed()
                    || self.delegate.has_another_open_menu(window, cx);
                if self.draws_own_container() && window.is_window_active() && !menu_focused {
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
        self.update_matches_with_options(query, ScrollBehavior::RevealSelected, window, cx);
    }

    pub fn update_matches_with_options(
        &mut self,
        query: String,
        scroll_behavior: ScrollBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let delegate_pending_update_matches = self.delegate.update_matches(query, window, cx);

        self.matches_updated(scroll_behavior, window, cx);
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
                    this.matches_updated(scroll_behavior, window, cx);
                })
            }),
        });
    }

    fn matches_updated(
        &mut self,
        scroll_behavior: ScrollBehavior,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let match_count = self.delegate.match_count();
        if match_count == 0
            && let Some(preview) = &mut self.preview
        {
            preview.clear(cx)
        }

        match &mut self.element_container {
            ElementContainer::List(state) => match scroll_behavior {
                ScrollBehavior::RevealSelected => {
                    state.reset(match_count);
                    let index = self.delegate.selected_index();
                    self.scroll_to_item_index(index);
                }
                ScrollBehavior::PreserveOffset => {
                    let offset = state.logical_scroll_top();
                    state.reset(match_count);
                    state.scroll_to(offset);
                }
            },
            ElementContainer::UniformList(_) => match scroll_behavior {
                ScrollBehavior::RevealSelected => {
                    let index = self.delegate.selected_index();
                    self.scroll_to_item_index(index);
                }
                ScrollBehavior::PreserveOffset => {}
            },
        }
        self.pending_update_matches = None;
        if let Some(update) = self.delegate.try_get_preview_data_for_match(cx)
            && let Some(preview) = &mut self.preview
        {
            preview.update(update, window, cx);
        }
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

    /// Selects the entire query, so the next keystroke replaces it (and a single
    /// backspace clears it). Matches the buffer search bar's seeded-query behavior.
    pub fn select_query(&self, window: &mut Window, cx: &mut App) {
        if let Head::Editor(editor) = &self.head {
            editor.select_all(window, cx);
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

    pub fn is_scrolled_to_end(&self) -> Option<bool> {
        match &self.element_container {
            ElementContainer::List(state) => state.is_scrolled_to_end(),
            ElementContainer::UniformList(scroll_handle) => scroll_handle.is_scrolled_to_end(),
        }
    }

    fn render_element(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
        ix: usize,
    ) -> impl IntoElement + use<D> {
        let item_bounds = self.item_bounds.clone();
        let selectable =
            ix < self.delegate.match_count() && self.delegate.can_select(ix, window, cx);

        div()
            .id(("item", ix))
            .when(selectable, |this| this.cursor_pointer())
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
            .when(!self.delegate.select_on_hover(), |this| {
                this.on_mouse_down(MouseButton::Left, |_, window, _cx| {
                    window.prevent_default();
                })
            })
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
            .when(self.delegate.select_on_hover(), |this| {
                this.on_hover(cx.listener(move |this, hovered: &bool, window, cx| {
                    if *hovered {
                        this.set_selected_index(ix, None, false, window, cx);
                        cx.notify();
                    }
                }))
            })
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
        // When the picker shrinks to fit its content, the list infers its size
        // from its items. When it fills its full height (preview visible), the
        // list fills the available space.
        let sizing_behavior = if self.fill_height() {
            ListSizingBehavior::Auto
        } else {
            ListSizingBehavior::Infer
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
            .flex_grow_1()
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
            .flex_grow_1()
            .py(DynamicSpacing::Base04.rems(cx))
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

    fn preview_layout(&self) -> Option<preview::Layout> {
        self.preview.as_ref().map(|p| p.layout)
    }

    fn toggle_preview(&mut self, _: &TogglePreview, window: &mut Window, cx: &mut Context<Self>) {
        self.toggle_preview_visible(window, cx);
    }

    fn toggle_preview_visible(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.preview_layout() {
            Some(preview::Layout::Hidden) | None => preview::Layout::Right,
            Some(_) => preview::Layout::Hidden,
        };
        self.set_preview_layout(next, window, cx);
    }

    fn set_preview_layout(
        &mut self,
        layout: preview::Layout,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        persistence::store_last_layout(D::name(), self.preview.as_ref().map(|_| layout), cx);

        let Some(preview) = &mut self.preview else {
            return;
        };
        preview.layout = layout;
        // Restore the size the user last left this layout at, or fall back to the
        // layout's default (simple when hidden, larger when a preview is shown).
        self.shape = persistence::try_load_shape(D::name(), layout, cx)
            .log_err()
            .flatten()
            .unwrap_or_else(|| {
                shape::Shape::HorizontallyCentered(default_shape_for_layout(
                    self.default_shape,
                    layout,
                ))
            });
        self.delegate
            .preview_layout_changed(matches!(layout, preview::Layout::Right));
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use std::cell::Cell;

    struct TestDelegate {
        items: Vec<bool>,
        selected_index: usize,
        confirmed_index: Rc<Cell<Option<usize>>>,
    }

    impl TestDelegate {
        fn new(items: Vec<bool>) -> Self {
            Self {
                items,
                selected_index: 0,
                confirmed_index: Rc::new(Cell::new(None)),
            }
        }
    }

    impl PickerDelegate for TestDelegate {
        type ListItem = ui::ListItem;

        fn name() -> &'static str {
            "test"
        }

        fn match_count(&self) -> usize {
            self.items.len()
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

        fn can_select(
            &self,
            ix: usize,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> bool {
            self.items.get(ix).copied().unwrap_or(false)
        }

        fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
            "Test".into()
        }

        fn update_matches(
            &mut self,
            _query: String,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Task<()> {
            Task::ready(())
        }

        fn confirm(
            &mut self,
            _secondary: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) {
            self.confirmed_index.set(Some(self.selected_index));
        }

        fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

        fn render_match(
            &self,
            ix: usize,
            selected: bool,
            _window: &mut Window,
            _cx: &mut Context<Picker<Self>>,
        ) -> Option<Self::ListItem> {
            Some(
                ui::ListItem::new(ix)
                    .inset(true)
                    .toggle_state(selected)
                    .child(ui::Label::new(format!("Item {ix}"))),
            )
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = settings::SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
    }

    #[gpui::test]
    async fn test_clicking_non_selectable_item_does_not_confirm(cx: &mut TestAppContext) {
        init_test(cx);

        let confirmed_index = Rc::new(Cell::new(None));
        let (picker, cx) = cx.add_window_view(|window, cx| {
            let mut delegate = TestDelegate::new(vec![true, false, true]);
            delegate.confirmed_index = confirmed_index.clone();
            Picker::uniform_list(delegate, window, cx)
        });

        picker.update(cx, |picker, _cx| {
            assert_eq!(picker.delegate.selected_index(), 0);
        });

        picker.update_in(cx, |picker, window, cx| {
            picker.handle_click(1, false, window, cx);
        });
        assert!(
            confirmed_index.get().is_none(),
            "clicking a non-selectable item should not confirm"
        );

        picker.update_in(cx, |picker, window, cx| {
            picker.handle_click(0, false, window, cx);
        });
        assert_eq!(
            confirmed_index.get(),
            Some(0),
            "clicking a selectable item should confirm"
        );
    }

    #[gpui::test]
    async fn test_keyboard_navigation_skips_non_selectable_items(cx: &mut TestAppContext) {
        init_test(cx);

        let (picker, cx) = cx.add_window_view(|window, cx| {
            Picker::uniform_list(TestDelegate::new(vec![true, false, true]), window, cx)
        });

        picker.update(cx, |picker, _cx| {
            assert_eq!(picker.delegate.selected_index(), 0);
        });

        picker.update_in(cx, |picker, window, cx| {
            picker.select_next(&menu::SelectNext, window, cx);
        });
        picker.update(cx, |picker, _cx| {
            assert_eq!(
                picker.delegate.selected_index(),
                2,
                "select_next should skip non-selectable item at index 1"
            );
        });

        picker.update_in(cx, |picker, window, cx| {
            picker.select_previous(&menu::SelectPrevious, window, cx);
        });
        picker.update(cx, |picker, _cx| {
            assert_eq!(
                picker.delegate.selected_index(),
                0,
                "select_previous should skip non-selectable item at index 1"
            );
        });
    }
}

impl<D: PickerDelegate> EventEmitter<DismissEvent> for Picker<D> {}
impl<D: PickerDelegate> ModalView for Picker<D> {}
impl<D: PickerDelegate> ui::FluentBuilder for Picker<D> {}
