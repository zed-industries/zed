use std::{any::Any, fmt::Debug, marker::PhantomData, ops::Not, time::Duration};

use gpui::{
    Along, App, AppContext as _, Axis as ScrollbarAxis, BorderStyle, Bounds, ContentMask, Context,
    Corner, Corners, CursorStyle, Div, Edges, Element, ElementId, Entity, EntityId,
    GlobalElementId, Hitbox, HitboxBehavior, Hsla, InteractiveElement, IntoElement, IsZero,
    LayoutId, ListState, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Negate,
    ParentElement, Pixels, Point, Position, Render, ScrollHandle, ScrollWheelEvent, Size, Stateful,
    StatefulInteractiveElement, Style, Styled, Task, UniformListDecoration,
    UniformListScrollHandle, Window, prelude::FluentBuilder as _, px, quad, relative, size,
};
use settings::SettingsStore;
use smallvec::SmallVec;
use theme::ActiveTheme as _;
use util::ResultExt;

use std::ops::Range;

use crate::scrollbars::{ScrollbarVisibility, ShowScrollbar};

const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_millis(1500);
const SCROLLBAR_PADDING: Pixels = px(4.);

pub mod scrollbars {
    use gpui::{App, Global};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use settings::Settings;

    /// When to show the scrollbar in the editor.
    ///
    /// Default: auto
    #[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum ShowScrollbar {
        /// Show the scrollbar if there's important information or
        /// follow the system's configured behavior.
        Auto,
        /// Match the system's configured behavior.
        System,
        /// Always show the scrollbar.
        Always,
        /// Never show the scrollbar.
        Never,
    }

    impl ShowScrollbar {
        pub(super) fn show(&self) -> bool {
            *self != Self::Never
        }

        pub(super) fn should_auto_hide(&self, cx: &mut App) -> bool {
            match self {
                Self::Auto => true,
                Self::System => cx.default_global::<ScrollbarAutoHide>().should_hide(),
                _ => false,
            }
        }
    }

    pub trait GlobalSetting {
        fn get_value(cx: &App) -> &Self;
    }

    impl<T: Settings> GlobalSetting for T {
        fn get_value(cx: &App) -> &T {
            T::get_global(cx)
        }
    }

    impl GlobalSetting for ShowScrollbar {
        fn get_value(_cx: &App) -> &Self {
            &ShowScrollbar::Always
        }
    }

    pub trait ScrollbarVisibility: GlobalSetting + 'static {
        fn visibility(&self, cx: &App) -> ShowScrollbar;
    }

    impl ScrollbarVisibility for ShowScrollbar {
        fn visibility(&self, cx: &App) -> ShowScrollbar {
            *ShowScrollbar::get_value(cx)
        }
    }

    #[derive(Default)]
    pub struct ScrollbarAutoHide(pub bool);

    impl ScrollbarAutoHide {
        pub fn should_hide(&self) -> bool {
            self.0
        }
    }

    impl Global for ScrollbarAutoHide {}
}

fn get_scrollbar_state<S, T>(
    mut config: Scrollbars<S, T>,
    caller_location: &'static std::panic::Location,
    window: &mut Window,
    cx: &mut App,
) -> Entity<ScrollbarStateWrapper<S, T>>
where
    S: ScrollbarVisibility,
    T: ScrollableHandle,
{
    let element_id = config.id.take().unwrap_or_else(|| caller_location.into());

    window.use_keyed_state(element_id, cx, |window, cx| {
        let parent_id = cx.entity_id();
        ScrollbarStateWrapper(
            cx.new(|cx| ScrollbarState::new_from_config(config, parent_id, window, cx)),
        )
    })
}

pub trait WithScrollbar: Sized {
    type Output;

    fn custom_scrollbars<S, T>(
        self,
        config: Scrollbars<S, T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Output
    where
        S: ScrollbarVisibility,
        T: ScrollableHandle;

    #[track_caller]
    fn horizontal_scrollbar(self, window: &mut Window, cx: &mut App) -> Self::Output {
        self.custom_scrollbars(
            Scrollbars::new(ScrollAxes::Horizontal).ensure_id(core::panic::Location::caller()),
            window,
            cx,
        )
    }

    #[track_caller]
    fn vertical_scrollbar(self, window: &mut Window, cx: &mut App) -> Self::Output {
        self.custom_scrollbars(
            Scrollbars::new(ScrollAxes::Vertical).ensure_id(core::panic::Location::caller()),
            window,
            cx,
        )
    }

    #[track_caller]
    fn vertical_scrollbar_for<ScrollHandle: ScrollableHandle>(
        self,
        scroll_handle: ScrollHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Output {
        self.custom_scrollbars(
            Scrollbars::new(ScrollAxes::Vertical)
                .tracked_scroll_handle(scroll_handle)
                .ensure_id(core::panic::Location::caller()),
            window,
            cx,
        )
    }
}

impl WithScrollbar for Stateful<Div> {
    type Output = Self;

    #[track_caller]
    fn custom_scrollbars<S, T>(
        self,
        config: Scrollbars<S, T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Output
    where
        S: ScrollbarVisibility,
        T: ScrollableHandle,
    {
        render_scrollbar(
            get_scrollbar_state(config, std::panic::Location::caller(), window, cx),
            self,
            cx,
        )
    }
}

impl WithScrollbar for Div {
    type Output = Stateful<Div>;

    #[track_caller]
    fn custom_scrollbars<S, T>(
        self,
        config: Scrollbars<S, T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Output
    where
        S: ScrollbarVisibility,
        T: ScrollableHandle,
    {
        let scrollbar = get_scrollbar_state(config, std::panic::Location::caller(), window, cx);
        // We know this ID stays consistent as long as the element is rendered for
        // consecutive frames, which is sufficient for our use case here
        let scrollbar_entity_id = scrollbar.entity_id();

        render_scrollbar(
            scrollbar,
            self.id(("track-scroll", scrollbar_entity_id)),
            cx,
        )
    }
}

fn render_scrollbar<S, T>(
    scrollbar: Entity<ScrollbarStateWrapper<S, T>>,
    div: Stateful<Div>,
    cx: &App,
) -> Stateful<Div>
where
    S: ScrollbarVisibility,
    T: ScrollableHandle,
{
    let state = &scrollbar.read(cx).0;

    div.when_some(state.read(cx).handle_to_track(), |this, handle| {
        this.track_scroll(handle).when_some(
            state.read(cx).visible_axes(),
            |this, axes| match axes {
                ScrollAxes::Horizontal => this.overflow_x_scroll(),
                ScrollAxes::Vertical => this.overflow_y_scroll(),
                ScrollAxes::Both => this.overflow_scroll(),
            },
        )
    })
    .when_some(
        state
            .read(cx)
            .space_to_reserve_for(ScrollbarAxis::Horizontal),
        |this, space| this.pb(space),
    )
    .when_some(
        state.read(cx).space_to_reserve_for(ScrollbarAxis::Vertical),
        |this, space| this.pr(space),
    )
    .child(state.clone())
}

impl<S: ScrollbarVisibility, T: ScrollableHandle> UniformListDecoration
    for ScrollbarStateWrapper<S, T>
{
    fn compute(
        &self,
        _visible_range: Range<usize>,
        _bounds: Bounds<Pixels>,
        scroll_offset: Point<Pixels>,
        _item_height: Pixels,
        _item_count: usize,
        _window: &mut Window,
        _cx: &mut App,
    ) -> gpui::AnyElement {
        ScrollbarElement {
            origin: scroll_offset.negate(),
            state: self.0.clone(),
        }
        .into_any()
    }
}

// impl WithScrollbar for UniformList {
//     type Output = Self;

//     #[track_caller]
//     fn custom_scrollbars<S, T>(
//         self,
//         config: Scrollbars<S, T>,
//         window: &mut Window,
//         cx: &mut App,
//     ) -> Self::Output
//     where
//         S: ScrollbarVisibilitySetting,
//         T: ScrollableHandle,
//     {
//         let scrollbar = get_scrollbar_state(config, std::panic::Location::caller(), window, cx);
//         self.when_some(
//             scrollbar.read_with(cx, |wrapper, cx| {
//                 wrapper
//                     .0
//                     .read(cx)
//                     .handle_to_track::<UniformListScrollHandle>()
//                     .cloned()
//             }),
//             |this, handle| this.track_scroll(handle),
//         )
//         .with_decoration(scrollbar)
//     }
// }

pub enum ScrollAxes {
    Horizontal,
    Vertical,
    Both,
}

impl ScrollAxes {
    fn apply_to<T>(self, point: Point<T>, value: T) -> Point<T>
    where
        T: Debug + Default + PartialEq + Clone,
    {
        match self {
            Self::Horizontal => point.apply_along(ScrollbarAxis::Horizontal, |_| value),
            Self::Vertical => point.apply_along(ScrollbarAxis::Vertical, |_| value),
            Self::Both => Point::new(value.clone(), value),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
enum ReservedSpace {
    #[default]
    None,
    Thumb,
    Track,
}

impl ReservedSpace {
    fn is_visible(&self) -> bool {
        *self != ReservedSpace::None
    }

    fn needs_scroll_track(&self) -> bool {
        *self == ReservedSpace::Track
    }
}

#[derive(Debug, Default)]
enum ScrollbarWidth {
    #[default]
    Normal,
    Small,
    XSmall,
}

impl ScrollbarWidth {
    fn to_pixels(&self) -> Pixels {
        match self {
            ScrollbarWidth::Normal => px(8.),
            ScrollbarWidth::Small => px(6.),
            ScrollbarWidth::XSmall => px(4.),
        }
    }
}

pub struct Scrollbars<S: ScrollbarVisibility = ShowScrollbar, T: ScrollableHandle = ScrollHandle> {
    id: Option<ElementId>,
    tracked_setting: PhantomData<S>,
    tracked_entity: Option<Option<EntityId>>,
    scrollable_handle: Box<dyn FnOnce() -> T>,
    handle_was_added: bool,
    visibility: Point<ReservedSpace>,
    scrollbar_width: ScrollbarWidth,
}

impl Scrollbars {
    pub fn new(show_along: ScrollAxes) -> Self {
        Self::new_with_setting(show_along)
    }

    pub fn for_settings<S: ScrollbarVisibility>() -> Scrollbars<S> {
        Scrollbars::<S>::new_with_setting(ScrollAxes::Both)
    }
}

impl<S: ScrollbarVisibility> Scrollbars<S> {
    fn new_with_setting(show_along: ScrollAxes) -> Self {
        Self {
            id: None,
            tracked_setting: PhantomData,
            handle_was_added: false,
            scrollable_handle: Box::new(ScrollHandle::new),
            tracked_entity: None,
            visibility: show_along.apply_to(Default::default(), ReservedSpace::Thumb),
            scrollbar_width: ScrollbarWidth::Normal,
        }
    }
}

impl<Setting: ScrollbarVisibility, ScrollHandle: ScrollableHandle>
    Scrollbars<Setting, ScrollHandle>
{
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    fn ensure_id(mut self, id: impl Into<ElementId>) -> Self {
        if self.id.is_none() {
            self.id = Some(id.into());
        }
        self
    }

    /// Notify the current context whenever this scrollbar gets a scroll event
    pub fn notify_content(mut self) -> Self {
        self.tracked_entity = Some(None);
        self
    }

    /// Set a parent model which should be notified whenever this scrollbar gets a scroll event.
    pub fn tracked_entity(mut self, entity_id: EntityId) -> Self {
        self.tracked_entity = Some(Some(entity_id));
        self
    }

    pub fn tracked_scroll_handle<TrackedHandle: ScrollableHandle>(
        self,
        tracked_scroll_handle: TrackedHandle,
    ) -> Scrollbars<Setting, TrackedHandle> {
        let Self {
            id,
            tracked_setting,
            tracked_entity: tracked_entity_id,
            scrollbar_width,
            visibility,
            ..
        } = self;

        Scrollbars {
            handle_was_added: true,
            scrollable_handle: Box::new(|| tracked_scroll_handle),
            id,
            tracked_setting,
            tracked_entity: tracked_entity_id,
            visibility,
            scrollbar_width,
        }
    }

    pub fn show_along(mut self, along: ScrollAxes) -> Self {
        self.visibility = along.apply_to(self.visibility, ReservedSpace::Thumb);
        self
    }

    pub fn with_track_along(mut self, along: ScrollAxes) -> Self {
        self.visibility = along.apply_to(self.visibility, ReservedSpace::Track);
        self
    }

    pub fn width_sm(mut self) -> Self {
        self.scrollbar_width = ScrollbarWidth::Small;
        self
    }

    pub fn width_xs(mut self) -> Self {
        self.scrollbar_width = ScrollbarWidth::XSmall;
        self
    }
}

#[derive(PartialEq, Eq)]
enum VisibilityState {
    Visible,
    Hidden,
    Disabled,
}

impl VisibilityState {
    fn from_show_setting(show_setting: ShowScrollbar) -> Self {
        if show_setting.show() {
            Self::Visible
        } else {
            Self::Disabled
        }
    }

    fn is_visible(&self) -> bool {
        *self == VisibilityState::Visible
    }

    #[inline]
    fn is_disabled(&self) -> bool {
        *self == VisibilityState::Disabled
    }
}

enum ParentHovered {
    Yes(bool),
    No(bool),
}

/// This is used to ensure notifies within the state do not notify the parent
/// unintentionally.
struct ScrollbarStateWrapper<S: ScrollbarVisibility, T: ScrollableHandle>(
    Entity<ScrollbarState<S, T>>,
);

/// A scrollbar state that should be persisted across frames.
struct ScrollbarState<S: ScrollbarVisibility, T: ScrollableHandle = ScrollHandle> {
    thumb_state: ThumbState,
    notify_id: Option<EntityId>,
    manually_added: bool,
    scroll_handle: T,
    width: ScrollbarWidth,
    tracked_setting: PhantomData<S>,
    show_setting: ShowScrollbar,
    visibility: Point<ReservedSpace>,
    show_state: VisibilityState,
    mouse_in_parent: bool,
    last_prepaint_state: Option<ScrollbarPrepaintState>,
    _auto_hide_task: Option<Task<()>>,
}

impl<S: ScrollbarVisibility, T: ScrollableHandle> ScrollbarState<S, T> {
    fn new_from_config(
        config: Scrollbars<S, T>,
        parent_id: EntityId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_global_in::<SettingsStore>(window, Self::settings_changed)
            .detach();

        ScrollbarState {
            thumb_state: Default::default(),
            notify_id: config.tracked_entity.map(|id| id.unwrap_or(parent_id)),
            manually_added: config.handle_was_added,
            scroll_handle: (config.scrollable_handle)(),
            width: config.scrollbar_width,
            visibility: config.visibility,
            tracked_setting: PhantomData,
            show_setting: S::get_value(cx).visibility(cx),
            show_state: VisibilityState::Visible,
            mouse_in_parent: true,
            last_prepaint_state: None,
            _auto_hide_task: None,
        }
    }

    fn settings_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_show_scrollbar(S::get_value(cx).visibility(cx), window, cx);
    }

    /// Schedules a scrollbar auto hide if no auto hide is currently in progress yet.
    fn schedule_auto_hide(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self._auto_hide_task.is_none() {
            self._auto_hide_task =
                (self.visible() && self.show_setting.should_auto_hide(cx)).then(|| {
                    cx.spawn_in(window, async move |scrollbar_state, cx| {
                        cx.background_executor()
                            .timer(SCROLLBAR_SHOW_INTERVAL)
                            .await;
                        scrollbar_state
                            .update(cx, |state, cx| {
                                state.set_visibility(VisibilityState::Hidden, cx);
                                state._auto_hide_task.take()
                            })
                            .log_err();
                    })
                });
        }
    }

    fn show_scrollbars(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_visibility(VisibilityState::Visible, cx);
        self._auto_hide_task.take();
        self.schedule_auto_hide(window, cx);
    }

    fn set_show_scrollbar(
        &mut self,
        show: ShowScrollbar,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.show_setting != show {
            self.show_setting = show;
            self.set_visibility(VisibilityState::from_show_setting(show), cx);
            self.schedule_auto_hide(window, cx);
            cx.notify();
        }
    }

    fn set_visibility(&mut self, visibility: VisibilityState, cx: &mut Context<Self>) {
        if self.show_state != visibility {
            self.show_state = visibility;
            cx.notify();
        }
    }

    #[inline]
    fn visible_axes(&self) -> Option<ScrollAxes> {
        match (&self.visibility.x, &self.visibility.y) {
            (ReservedSpace::None, ReservedSpace::None) => None,
            (ReservedSpace::None, _) => Some(ScrollAxes::Vertical),
            (_, ReservedSpace::None) => Some(ScrollAxes::Horizontal),
            _ => Some(ScrollAxes::Both),
        }
    }

    fn space_to_reserve_for(&self, axis: ScrollbarAxis) -> Option<Pixels> {
        (self.show_state.is_disabled().not() && self.visibility.along(axis).needs_scroll_track())
            .then(|| self.space_to_reserve())
    }

    fn space_to_reserve(&self) -> Pixels {
        self.width.to_pixels() + 2 * SCROLLBAR_PADDING
    }

    fn handle_to_track<Handle: ScrollableHandle>(&self) -> Option<&Handle> {
        (!self.manually_added)
            .then(|| (self.scroll_handle() as &dyn Any).downcast_ref::<Handle>())
            .flatten()
    }

    fn scroll_handle(&self) -> &T {
        &self.scroll_handle
    }

    fn set_offset(&mut self, offset: Point<Pixels>, cx: &mut Context<Self>) {
        self.scroll_handle.set_offset(offset);
        self.notify_parent(cx);
        cx.notify();
    }

    fn is_dragging(&self) -> bool {
        self.thumb_state.is_dragging()
    }

    fn set_dragging(
        &mut self,
        axis: ScrollbarAxis,
        drag_offset: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_thumb_state(ThumbState::Dragging(axis, drag_offset), window, cx);
        self.scroll_handle().drag_started();
    }

    fn update_hovered_thumb(
        &mut self,
        position: &Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_thumb_state(
            if let Some(&ScrollbarLayout { axis, .. }) = self
                .last_prepaint_state
                .as_ref()
                .and_then(|state| state.thumb_for_position(position))
            {
                ThumbState::Hover(axis)
            } else {
                ThumbState::Inactive
            },
            window,
            cx,
        );
    }

    fn set_thumb_state(&mut self, state: ThumbState, window: &mut Window, cx: &mut Context<Self>) {
        if self.thumb_state != state {
            if state == ThumbState::Inactive {
                self.schedule_auto_hide(window, cx);
            } else {
                self.show_scrollbars(window, cx);
            }
            self.thumb_state = state;
            cx.notify();
        }
    }

    fn update_parent_hovered(&mut self, position: &Point<Pixels>) -> ParentHovered {
        let last_parent_hovered = self.mouse_in_parent;
        self.mouse_in_parent = self.parent_hovered(position);
        let state_changed = self.mouse_in_parent != last_parent_hovered;
        match self.mouse_in_parent {
            true => ParentHovered::Yes(state_changed),
            false => ParentHovered::No(state_changed),
        }
    }

    fn parent_hovered(&self, position: &Point<Pixels>) -> bool {
        self.last_prepaint_state
            .as_ref()
            .is_some_and(|state| state.parent_bounds.contains(position))
    }

    fn hit_for_position(&self, position: &Point<Pixels>) -> Option<&ScrollbarLayout> {
        self.last_prepaint_state
            .as_ref()
            .and_then(|state| state.hit_for_position(position))
    }

    fn thumb_for_axis(&self, axis: ScrollbarAxis) -> Option<&ScrollbarLayout> {
        self.last_prepaint_state
            .as_ref()
            .and_then(|state| state.thumbs.iter().find(|thumb| thumb.axis == axis))
    }

    fn thumb_ranges(
        &self,
    ) -> impl Iterator<Item = (ScrollbarAxis, Range<f32>, ReservedSpace)> + '_ {
        const MINIMUM_THUMB_SIZE: Pixels = px(25.);
        let max_offset = self.scroll_handle().max_offset();
        let viewport_size = self.scroll_handle().viewport().size;
        let current_offset = self.scroll_handle().offset();

        [ScrollbarAxis::Horizontal, ScrollbarAxis::Vertical]
            .into_iter()
            .filter(|&axis| self.visibility.along(axis).is_visible())
            .flat_map(move |axis| {
                let max_offset = max_offset.along(axis);
                let viewport_size = viewport_size.along(axis);
                if max_offset.is_zero() || viewport_size.is_zero() {
                    return None;
                }
                let content_size = viewport_size + max_offset;
                let visible_percentage = viewport_size / content_size;
                let thumb_size = MINIMUM_THUMB_SIZE.max(viewport_size * visible_percentage);
                if thumb_size > viewport_size {
                    return None;
                }
                let current_offset = current_offset
                    .along(axis)
                    .clamp(-max_offset, Pixels::ZERO)
                    .abs();
                let start_offset = (current_offset / max_offset) * (viewport_size - thumb_size);
                let thumb_percentage_start = start_offset / viewport_size;
                let thumb_percentage_end = (start_offset + thumb_size) / viewport_size;
                Some((
                    axis,
                    thumb_percentage_start..thumb_percentage_end,
                    self.visibility.along(axis),
                ))
            })
    }

    fn visible(&self) -> bool {
        self.show_state.is_visible()
    }

    #[inline]
    fn disabled(&self) -> bool {
        self.show_state.is_disabled()
    }

    fn notify_parent(&self, cx: &mut App) {
        if let Some(entity_id) = self.notify_id {
            cx.notify(entity_id);
        }
    }
}

impl<S: ScrollbarVisibility, T: ScrollableHandle> Render for ScrollbarState<S, T> {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ScrollbarElement {
            state: cx.entity(),
            origin: Default::default(),
        }
    }
}

struct ScrollbarElement<S: ScrollbarVisibility, T: ScrollableHandle> {
    origin: Point<Pixels>,
    state: Entity<ScrollbarState<S, T>>,
}

#[derive(Default, Debug, PartialEq, Eq)]
enum ThumbState {
    #[default]
    Inactive,
    Hover(ScrollbarAxis),
    Dragging(ScrollbarAxis, Pixels),
}

impl ThumbState {
    fn is_dragging(&self) -> bool {
        matches!(*self, ThumbState::Dragging(..))
    }
}

impl ScrollableHandle for UniformListScrollHandle {
    fn max_offset(&self) -> Size<Pixels> {
        self.0.borrow().base_handle.max_offset()
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.0.borrow().base_handle.set_offset(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.0.borrow().base_handle.offset()
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.0.borrow().base_handle.bounds()
    }
}

impl ScrollableHandle for ListState {
    fn max_offset(&self) -> Size<Pixels> {
        self.max_offset_for_scrollbar()
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.set_offset_from_scrollbar(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.scroll_px_offset_for_scrollbar()
    }

    fn drag_started(&self) {
        self.scrollbar_drag_started();
    }

    fn drag_ended(&self) {
        self.scrollbar_drag_ended();
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.viewport_bounds()
    }
}

impl ScrollableHandle for ScrollHandle {
    fn max_offset(&self) -> Size<Pixels> {
        self.max_offset()
    }

    fn set_offset(&self, point: Point<Pixels>) {
        self.set_offset(point);
    }

    fn offset(&self) -> Point<Pixels> {
        self.offset()
    }

    fn viewport(&self) -> Bounds<Pixels> {
        self.bounds()
    }
}

pub trait ScrollableHandle: 'static + Any + Sized {
    fn max_offset(&self) -> Size<Pixels>;
    fn set_offset(&self, point: Point<Pixels>);
    fn offset(&self) -> Point<Pixels>;
    fn viewport(&self) -> Bounds<Pixels>;
    fn drag_started(&self) {}
    fn drag_ended(&self) {}

    fn scrollable_along(&self, axis: ScrollbarAxis) -> bool {
        self.max_offset().along(axis) > Pixels::ZERO
    }
    fn content_size(&self) -> Size<Pixels> {
        self.viewport().size + self.max_offset()
    }
}

enum ScrollbarMouseEvent {
    TrackClick,
    ThumbDrag(Pixels),
}

struct ScrollbarLayout {
    thumb_bounds: Bounds<Pixels>,
    track_bounds: Bounds<Pixels>,
    cursor_hitbox: Hitbox,
    reserved_space: ReservedSpace,
    axis: ScrollbarAxis,
}

impl ScrollbarLayout {
    fn compute_click_offset(
        &self,
        event_position: Point<Pixels>,
        max_offset: Size<Pixels>,
        event_type: ScrollbarMouseEvent,
    ) -> Pixels {
        let Self {
            track_bounds,
            thumb_bounds,
            axis,
            ..
        } = self;
        let axis = *axis;

        let viewport_size = track_bounds.size.along(axis);
        let thumb_size = thumb_bounds.size.along(axis);
        let thumb_offset = match event_type {
            ScrollbarMouseEvent::TrackClick => thumb_size / 2.,
            ScrollbarMouseEvent::ThumbDrag(thumb_offset) => thumb_offset,
        };

        let thumb_start =
            (event_position.along(axis) - track_bounds.origin.along(axis) - thumb_offset)
                .clamp(px(0.), viewport_size - thumb_size);

        let max_offset = max_offset.along(axis);
        let percentage = if viewport_size > thumb_size {
            thumb_start / (viewport_size - thumb_size)
        } else {
            0.
        };

        -max_offset * percentage
    }
}

impl PartialEq for ScrollbarLayout {
    fn eq(&self, other: &Self) -> bool {
        self.axis == other.axis && self.thumb_bounds == other.thumb_bounds
    }
}

pub struct ScrollbarPrepaintState {
    parent_bounds: Bounds<Pixels>,
    thumbs: SmallVec<[ScrollbarLayout; 2]>,
}

impl ScrollbarPrepaintState {
    fn thumb_for_position(&self, position: &Point<Pixels>) -> Option<&ScrollbarLayout> {
        self.thumbs
            .iter()
            .find(|info| info.thumb_bounds.contains(position))
    }

    fn hit_for_position(&self, position: &Point<Pixels>) -> Option<&ScrollbarLayout> {
        self.thumbs.iter().find(|info| {
            if info.reserved_space.needs_scroll_track() {
                info.track_bounds.contains(position)
            } else {
                info.thumb_bounds.contains(position)
            }
        })
    }
}

impl PartialEq for ScrollbarPrepaintState {
    fn eq(&self, other: &Self) -> bool {
        self.thumbs == other.thumbs
    }
}

impl<S: ScrollbarVisibility, T: ScrollableHandle> Element for ScrollbarElement<S, T> {
    type RequestLayoutState = ();
    type PrepaintState = Option<ScrollbarPrepaintState>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let scrollbar_style = Style {
            position: Position::Absolute,
            inset: Edges::default(),
            size: size(relative(1.), relative(1.)).map(Into::into),
            ..Default::default()
        };

        (window.request_layout(scrollbar_style, None, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let prepaint_state = self
            .state
            .read(cx)
            .disabled()
            .not()
            .then(|| ScrollbarPrepaintState {
                parent_bounds: bounds,
                thumbs: {
                    let thumb_ranges = self.state.read(cx).thumb_ranges().collect::<Vec<_>>();
                    let width = self.state.read(cx).width.to_pixels();

                    let additional_padding = if thumb_ranges.len() == 2 {
                        width
                    } else {
                        Pixels::ZERO
                    };

                    thumb_ranges
                        .into_iter()
                        .map(|(axis, thumb_range, reserved_space)| {
                            let track_anchor = match axis {
                                ScrollbarAxis::Horizontal => Corner::BottomLeft,
                                ScrollbarAxis::Vertical => Corner::TopRight,
                            };
                            let Bounds { origin, size } = Bounds::from_corner_and_size(
                                track_anchor,
                                bounds
                                    .corner(track_anchor)
                                    .apply_along(axis.invert(), |corner| {
                                        corner - SCROLLBAR_PADDING
                                    }),
                                bounds.size.apply_along(axis.invert(), |_| width),
                            );
                            let scroll_track_bounds = Bounds::new(self.origin + origin, size);

                            let padded_bounds = scroll_track_bounds.extend(match axis {
                                ScrollbarAxis::Horizontal => Edges {
                                    right: -SCROLLBAR_PADDING,
                                    left: -SCROLLBAR_PADDING,
                                    ..Default::default()
                                },
                                ScrollbarAxis::Vertical => Edges {
                                    top: -SCROLLBAR_PADDING,
                                    bottom: -SCROLLBAR_PADDING,
                                    ..Default::default()
                                },
                            });

                            let available_space =
                                padded_bounds.size.along(axis) - additional_padding;

                            let thumb_offset = thumb_range.start * available_space;
                            let thumb_end = thumb_range.end * available_space;
                            let thumb_bounds = Bounds::new(
                                padded_bounds
                                    .origin
                                    .apply_along(axis, |origin| origin + thumb_offset),
                                padded_bounds
                                    .size
                                    .apply_along(axis, |_| thumb_end - thumb_offset),
                            );

                            ScrollbarLayout {
                                thumb_bounds,
                                track_bounds: padded_bounds,
                                axis,
                                cursor_hitbox: window.insert_hitbox(
                                    if reserved_space.needs_scroll_track() {
                                        padded_bounds
                                    } else {
                                        thumb_bounds
                                    },
                                    HitboxBehavior::BlockMouseExceptScroll,
                                ),
                                reserved_space,
                            }
                        })
                        .collect()
                },
            });
        if prepaint_state
            .as_ref()
            .is_some_and(|state| Some(state) != self.state.read(cx).last_prepaint_state.as_ref())
        {
            self.state
                .update(cx, |state, cx| state.show_scrollbars(window, cx));
        }

        prepaint_state
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        Bounds { origin, size }: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint_state: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(prepaint_state) = prepaint_state.take() else {
            return;
        };

        let bounds = Bounds::new(self.origin + origin, size);
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let colors = cx.theme().colors();

            if self.state.read(cx).visible() {
                for ScrollbarLayout {
                    thumb_bounds,
                    cursor_hitbox,
                    axis,
                    reserved_space,
                    ..
                } in &prepaint_state.thumbs
                {
                    const MAXIMUM_OPACITY: f32 = 0.7;
                    let thumb_state = &self.state.read(cx).thumb_state;
                    let (thumb_base_color, hovered) = match thumb_state {
                        ThumbState::Dragging(dragged_axis, _) if dragged_axis == axis => {
                            (colors.scrollbar_thumb_active_background, false)
                        }
                        ThumbState::Hover(hovered_axis) if hovered_axis == axis => {
                            (colors.scrollbar_thumb_hover_background, true)
                        }
                        _ => (colors.scrollbar_thumb_background, false),
                    };

                    let blending_color = if hovered || reserved_space.needs_scroll_track() {
                        colors.surface_background
                    } else {
                        let blend_color = colors.surface_background;
                        blend_color.min(blend_color.alpha(MAXIMUM_OPACITY))
                    };

                    let thumb_background = blending_color.blend(thumb_base_color);

                    window.paint_quad(quad(
                        *thumb_bounds,
                        Corners::all(Pixels::MAX).clamp_radii_for_quad_size(thumb_bounds.size),
                        thumb_background,
                        Edges::default(),
                        Hsla::transparent_black(),
                        BorderStyle::default(),
                    ));

                    if thumb_state.is_dragging() {
                        window.set_window_cursor_style(CursorStyle::Arrow);
                    } else {
                        window.set_cursor_style(CursorStyle::Arrow, cursor_hitbox);
                    }
                }
            }

            self.state.update(cx, |state, _| {
                state.last_prepaint_state = Some(prepaint_state)
            });

            window.on_mouse_event({
                let state = self.state.clone();

                move |event: &MouseDownEvent, phase, window, cx| {
                    state.update(cx, |state, cx| {
                        let Some(scrollbar_layout) = (phase.capture()
                            && event.button == MouseButton::Left)
                            .then(|| state.hit_for_position(&event.position))
                            .flatten()
                        else {
                            return;
                        };

                        let ScrollbarLayout {
                            thumb_bounds, axis, ..
                        } = scrollbar_layout;

                        if thumb_bounds.contains(&event.position) {
                            let offset =
                                event.position.along(*axis) - thumb_bounds.origin.along(*axis);
                            state.set_dragging(*axis, offset, window, cx);
                        } else {
                            let scroll_handle = state.scroll_handle();
                            let click_offset = scrollbar_layout.compute_click_offset(
                                event.position,
                                scroll_handle.max_offset(),
                                ScrollbarMouseEvent::TrackClick,
                            );
                            state.set_offset(
                                scroll_handle.offset().apply_along(*axis, |_| click_offset),
                                cx,
                            );
                        };

                        cx.stop_propagation();
                    });
                }
            });

            window.on_mouse_event({
                let state = self.state.clone();

                move |event: &ScrollWheelEvent, phase, window, cx| {
                    if phase.capture() {
                        state.update(cx, |state, cx| {
                            state.update_hovered_thumb(&event.position, window, cx)
                        });
                    }
                }
            });

            window.on_mouse_event({
                let state = self.state.clone();

                move |event: &MouseMoveEvent, phase, window, cx| {
                    if !phase.capture() {
                        return;
                    }

                    match state.read(cx).thumb_state {
                        ThumbState::Dragging(axis, drag_state) if event.dragging() => {
                            if let Some(scrollbar_layout) = state.read(cx).thumb_for_axis(axis) {
                                let scroll_handle = state.read(cx).scroll_handle();
                                let drag_offset = scrollbar_layout.compute_click_offset(
                                    event.position,
                                    scroll_handle.max_offset(),
                                    ScrollbarMouseEvent::ThumbDrag(drag_state),
                                );
                                let new_offset =
                                    scroll_handle.offset().apply_along(axis, |_| drag_offset);

                                state.update(cx, |state, cx| state.set_offset(new_offset, cx));
                                cx.stop_propagation();
                            }
                        }
                        _ => state.update(cx, |state, cx| {
                            match state.update_parent_hovered(&event.position) {
                                ParentHovered::Yes(state_changed)
                                    if event.pressed_button.is_none() =>
                                {
                                    if state_changed {
                                        state.show_scrollbars(window, cx);
                                    }
                                    state.update_hovered_thumb(&event.position, window, cx);
                                    if state.thumb_state != ThumbState::Inactive {
                                        cx.stop_propagation();
                                    }
                                }
                                ParentHovered::No(state_changed) if state_changed => {
                                    state.set_thumb_state(ThumbState::Inactive, window, cx);
                                }
                                _ => {}
                            }
                        }),
                    }
                }
            });

            window.on_mouse_event({
                let state = self.state.clone();
                move |event: &MouseUpEvent, phase, window, cx| {
                    if !phase.capture() {
                        return;
                    }

                    state.update(cx, |state, cx| {
                        if state.is_dragging() {
                            state.scroll_handle().drag_ended();
                        }

                        if !state.parent_hovered(&event.position) {
                            state.schedule_auto_hide(window, cx);
                            return;
                        }

                        state.update_hovered_thumb(&event.position, window, cx);
                    });
                }
            });
        })
    }
}

impl<S: ScrollbarVisibility, T: ScrollableHandle> IntoElement for ScrollbarElement<S, T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
