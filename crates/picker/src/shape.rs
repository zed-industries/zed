use gpui::Window;
use gpui::{Pixels, Rems, Size};
use ui::{Div, Styled, rems_from_px};

use crate::preview::Layout;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PositionAndShape {
    /// Absolute position of left most side of the picker
    pub(crate) left: Pixels,
    /// Absolute position of right most side of the picker
    pub(crate) right: Pixels,
    /// Absolute position of top most side of the picker
    pub(crate) top: Pixels,
    /// Absolute position of bottom most side of the picker
    pub(crate) bottom: Pixels,
    /// Relative position of divide between results and preview,
    /// either a height or a width depends on previews layoutmode.
    /// Should be zero when preview is disabled or hidden
    pub(crate) preview: Pixels,
}

impl PositionAndShape {
    pub(crate) fn width(&self) -> Pixels {
        self.right - self.left
    }
}

macro_rules! relative_size {
    ($name:ident, $accessor:ident) => {
        /// Size type that is the sum of a relative size to the viewport and a
        /// size relative to the font size (Rems). You can
        /// add/subtract/multiple/divide to your harts content but once you
        /// need a single unit you must provide a window to get it.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            viewport_fraction: f32,
            rems: Rems,
        }

        impl From<Rems> for $name {
            fn from(v: Rems) -> Self {
                Self::rems(v)
            }
        }

        impl $name {
            pub const FULL: Self = Self {
                viewport_fraction: 1.0,
                rems: Rems::ZERO,
            };

            pub const fn viewport(fraction: f32) -> Self {
                debug_assert!(fraction <= 1.0);
                debug_assert!(fraction >= 0.0);
                Self {
                    viewport_fraction: fraction.clamp(0.0, 1.0),
                    rems: Rems::ZERO,
                }
            }

            pub const fn rems(val: Rems) -> Self {
                Self {
                    viewport_fraction: 0.0,
                    rems: val,
                }
            }

            pub fn as_pixels(&self, window: &Window) -> Pixels {
                self.viewport_fraction * window.viewport_size().$accessor
                    + self.rems * window.rem_size()
            }

            pub fn from_pixels(width: Pixels, window: &Window) -> Self {
                Self {
                    viewport_fraction: width / window.viewport_size().$accessor,
                    rems: Rems::ZERO,
                }
            }

            /// Returns this size as [`Rems`] when it has no viewport-relative
            /// component. Used to derive a rems-based minimum from an initial
            /// size without needing a [`Window`].
            pub fn as_rems(&self) -> Option<Rems> {
                (self.viewport_fraction == 0.0).then_some(self.rems)
            }

            pub fn as_viewport_fraction(&self, window: &Window) -> ViewportFraction {
                ViewportFraction(
                    self.viewport_fraction
                        + self.rems * window.rem_size() / window.viewport_size().$accessor,
                )
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    viewport_fraction: self.viewport_fraction + rhs.viewport_fraction,
                    rems: self.rems + rhs.rems,
                }
            }
        }

        impl std::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    viewport_fraction: self.viewport_fraction - rhs.viewport_fraction,
                    rems: self.rems - rhs.rems,
                }
            }
        }

        impl std::ops::Sub<Rems> for $name {
            type Output = Self;

            fn sub(self, rhs: Rems) -> Self::Output {
                Self {
                    viewport_fraction: self.viewport_fraction,
                    rems: self.rems - rhs,
                }
            }
        }

        impl std::ops::Div<f32> for $name {
            type Output = Self;

            fn div(mut self, rhs: f32) -> Self::Output {
                self.viewport_fraction /= rhs;
                self.rems = Rems(self.rems.0 / rhs);
                self
            }
        }

        impl std::ops::Mul<f32> for $name {
            type Output = Self;

            fn mul(mut self, rhs: f32) -> Self::Output {
                self.viewport_fraction *= rhs;
                self.rems = Rems(self.rems.0 * rhs);
                self
            }
        }
    };
}

relative_size!(RelativeHeight, height);
relative_size!(RelativeWidth, width);

#[derive(Debug, Clone, Copy)]
pub struct ViewportFraction(f32);

impl ViewportFraction {
    pub(crate) const ZERO: Self = Self(0.0);
    pub(crate) fn fraction(v: f32) -> Self {
        debug_assert!(
            (0.0..=1.0).contains(&v),
            "ViewportFraction must be between zero and one"
        );
        Self(v)
    }

    pub(crate) fn width_as_pixels(&self, window: &Window) -> Pixels {
        window.viewport_size().width * self.0
    }
    pub(crate) fn height_as_pixels(&self, window: &Window) -> Pixels {
        window.viewport_size().height * self.0
    }

    pub(crate) fn from_height_pixels(preview: Pixels, window: &Window) -> Self {
        Self(preview / window.viewport_size().height)
    }

    pub(crate) fn from_width_pixels(preview: Pixels, window: &Window) -> Self {
        Self(preview / window.viewport_size().width)
    }

    /// Returns the fraction of the viewport that this describes.
    /// Guaranteed to be between zero and one
    pub(crate) fn raw(&self) -> f32 {
        self.0
    }
}

impl std::ops::Mul<f32> for ViewportFraction {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Centered {
    pub(crate) width: RelativeWidth,
    pub(crate) height: RelativeHeight,
    pub(crate) preview_size: ViewportFraction,
}

impl Centered {
    /// The default size for a plain picker (no preview): a fixed standard width
    /// and a standard *max* height that the picker shrinks below when it has
    /// little content.
    pub(crate) fn simple() -> Self {
        Centered {
            width: RelativeWidth::rems(crate::DEFAULT_MODAL_WIDTH),
            height: RelativeHeight::rems(crate::DEFAULT_MODAL_MAX_HEIGHT),
            preview_size: ViewportFraction::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Shape {
    Resizing(PositionAndShape),
    /// This may be persisted between zero and three times:
    /// - for a user resized picker without preview
    /// - for a user resized picker with preview below
    /// - for a user resized picker with preview right
    HorizontallyCentered(Centered),
}

#[derive(Debug)]
pub struct SizeBounds {
    pub(crate) max_width: RelativeWidth,
    pub(crate) max_height: RelativeHeight,
    pub(crate) min_results: Size<Rems>,
    /// Minimum size of the preview pane. Along the split axis this only needs to
    /// be large enough to grab the divider and shrink it back.
    pub(crate) min_preview: Size<Rems>,
}

impl Default for SizeBounds {
    fn default() -> Self {
        Self {
            max_width: RelativeWidth::viewport(0.95),
            // Modals are placed at 5 rems from the top we also do not want them to go
            // over the lower bar so clear another 5 rems there.
            max_height: (RelativeHeight::FULL - Rems(10.0)) * 0.95,
            min_results: Size {
                width: rems_from_px(280.),
                height: rems_from_px(320.),
            },
            min_preview: Size {
                width: rems_from_px(128.),
                height: rems_from_px(96.),
            },
        }
    }
}

impl SizeBounds {
    /// Minimum total picker width for the given layout, composed from the
    /// results and preview minimums (they stack along the split axis and share
    /// the cross axis).
    fn min_width(&self, layout: Option<Layout>, window: &Window) -> Pixels {
        let rem = window.rem_size();
        let results = self.min_results.width * rem;
        let preview = self.min_preview.width * rem;
        match layout {
            Some(Layout::Right) => results + preview,
            Some(Layout::Below) => results.max(preview),
            Some(Layout::Hidden) | None => results,
        }
    }

    /// Minimum total picker height for the given layout. See [`Self::min_width`].
    fn min_height(&self, layout: Option<Layout>, window: &Window) -> Pixels {
        let rem = window.rem_size();
        let results = self.min_results.height * rem;
        let preview = self.min_preview.height * rem;
        match layout {
            Some(Layout::Below) => results + preview,
            Some(Layout::Right) => results.max(preview),
            Some(Layout::Hidden) | None => results,
        }
    }

    /// Clamps a width in pixels to the configured min/max width.
    pub(crate) fn clamp_width(
        &self,
        width: Pixels,
        layout: Option<Layout>,
        window: &Window,
    ) -> Pixels {
        width
            .min(self.max_width.as_pixels(window))
            .max(self.min_width(layout, window))
    }

    /// Clamps a height in pixels to the configured min/max height.
    pub(crate) fn clamp_height(
        &self,
        height: Pixels,
        layout: Option<Layout>,
        window: &Window,
    ) -> Pixels {
        height
            .min(self.max_height.as_pixels(window))
            .max(self.min_height(layout, window))
    }

    /// Clamps the picker's width by moving its left edge back into bounds.
    ///
    /// For a preview-to-the-right drag the preview is corrected by the same
    /// amount any snap moves the edge, so the results pane keeps its size at the
    /// boundary.
    pub(crate) fn clamp_left_edge(
        &self,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let target_width = self.clamp_width(working.right - working.left, layout, window);
        let new_left = working.right - target_width;
        let correction = new_left - working.left;
        working.left = new_left;
        if layout == Some(Layout::Right) {
            working.preview += correction;
        }
    }

    /// Clamps the picker's width by moving its right edge back into bounds.
    /// See [`Self::clamp_left_edge`].
    pub(crate) fn clamp_right_edge(
        &self,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let target_width = self.clamp_width(working.right - working.left, layout, window);
        let new_right = working.left + target_width;
        let correction = new_right - working.right;
        working.right = new_right;
        if layout == Some(Layout::Right) {
            working.preview += correction;
        }
    }

    /// Clamps the picker's height by moving its bottom edge back into bounds.
    /// See [`Self::clamp_left_edge`].
    pub(crate) fn clamp_bottom_edge(
        &self,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let target_height = self.clamp_height(working.bottom - working.top, layout, window);
        let new_bottom = working.top + target_height;
        let correction = new_bottom - working.bottom;
        working.bottom = new_bottom;
        if layout == Some(Layout::Below) {
            working.preview += correction;
        }
    }

    /// Clamps the divider between the results and the preview so both panes stay
    /// above their minimums. Runs last in each side's clamp so the divider gets
    /// the final say after the outer edges have been bounded.
    pub(crate) fn clamp_divider(
        &self,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let rem = window.rem_size();
        let (total, min_preview, min_results) = match layout {
            Some(Layout::Right) => (
                working.right - working.left,
                self.min_preview.width * rem,
                self.min_results.width * rem,
            ),
            Some(Layout::Below) => (
                working.bottom - working.top,
                self.min_preview.height * rem,
                self.min_results.height * rem,
            ),
            Some(Layout::Hidden) | None => return,
        };
        let max_preview = (total - min_results).max(min_preview);
        working.preview = working.preview.clamp(min_preview, max_preview);
    }

    pub(crate) fn would_clamp_width_if_horizontal(&self, shape: &Shape, window: &Window) -> bool {
        let min_width = self.min_width(Some(Layout::Right), window);

        let unbounded_width = shape
            .picker_position_and_size(Some(Layout::Right), window)
            .width();

        unbounded_width <= min_width
    }

    /// Clamps a whole picker rect (results + preview) into bounds: the total size
    /// against the per-layout min/max, then the divider so both panes keep their
    /// minimums. Width is clamped about its center, height anchored at the top.
    ///
    /// This is idempotent on an already-bounded rect, so it can run on every
    /// render and drag-start read without shifting an in-bounds shape.
    pub(crate) fn clamp_position_and_size(
        &self,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let target_width = self.clamp_width(working.right - working.left, layout, window);
        let center = (working.left + working.right) / 2.0;
        working.left = center - target_width / 2.0;
        working.right = center + target_width / 2.0;

        let target_height = self.clamp_height(working.bottom - working.top, layout, window);
        working.bottom = working.top + target_height;

        self.clamp_divider(working, layout, window);
    }
}

impl Shape {
    pub(crate) fn picker_position_and_size(
        &self,
        layout: impl Into<Option<Layout>>,
        window: &Window,
    ) -> PositionAndShape {
        match self {
            Shape::Resizing(pos) => *pos,
            Shape::HorizontallyCentered(Centered {
                width,
                height,
                preview_size,
            }) => PositionAndShape {
                //        W              V: full width     xxxxx: picker modal
                // -----xxxxx------      left = (V - W) / 2
                //     L     R           right = left + W = (V/2 - W/2) + W =  V/2 + W/2
                left: ((RelativeWidth::FULL - *width) / 2.0).as_pixels(window),
                right: (RelativeWidth::FULL / 2.0 + *width / 2.0).as_pixels(window),
                top: Pixels::ZERO,
                bottom: height.as_pixels(window),
                preview: match layout.into() {
                    Some(Layout::Below) => preview_size.height_as_pixels(window),
                    Some(Layout::Right) => preview_size.width_as_pixels(window),
                    Some(Layout::Hidden) | None => Pixels::ZERO,
                },
            },
        }
    }

    /// The picker rect with all bounds applied (total size and divider). This is
    /// the single source of truth for sizing during render and for seeding a
    /// drag, so a shape left dirty (e.g. after a layout toggle changes the
    /// minimums) is sanitized identically everywhere and never jumps on the
    /// first drag.
    pub(crate) fn clamped_position_and_size(
        &self,
        layout: Option<Layout>,
        bounds: &SizeBounds,
        window: &Window,
    ) -> PositionAndShape {
        let mut pos = self.picker_position_and_size(layout, window);
        bounds.clamp_position_and_size(&mut pos, layout, window);
        pos
    }

    pub(crate) fn results_position_and_size(
        &self,
        layout: Layout,
        bounds: &SizeBounds,
        window: &Window,
    ) -> PositionAndShape {
        let mut pos = self.clamped_position_and_size(Some(layout), bounds, window);

        match layout {
            Layout::Below => pos.bottom -= pos.preview,
            Layout::Right => pos.right -= pos.preview,
            Layout::Hidden => (),
        }
        pos
    }

    pub(crate) fn preview_position_and_size(
        &self,
        layout: Layout,
        bounds: &SizeBounds,
        window: &Window,
    ) -> PositionAndShape {
        let mut pos = self.clamped_position_and_size(Some(layout), bounds, window);

        match layout {
            Layout::Below => pos.top = pos.bottom - pos.preview,
            Layout::Right => pos.left = pos.right - pos.preview,
            Layout::Hidden => (),
        }
        pos
    }

    /// How far the center of the picker has been moved during dragging
    /// this allows extending it on one side without the picker centering during
    /// the resize. The drag is clamped to the size bounds (see
    /// [`Side::current_position_and_shape`]), so the center stays in bounds too.
    pub(crate) fn horizontal_offset(&self, window: &Window) -> Pixels {
        let Shape::Resizing(PositionAndShape { left, right, .. }) = self else {
            return Pixels::ZERO; // picker should be centered
        };
        let center = (*left + *right) / 2.0;
        let viewport_center = window.viewport_size().width / 2.0;
        center - viewport_center // shifting the picker by this uncenters it again
    }

    pub(crate) fn apply_results_size(
        &self,
        layout: impl Into<Option<Layout>>,
        bounds: &SizeBounds,
        fill_height: bool,
        div: Div,
        window: &Window,
    ) -> Div {
        let layout = layout.into();
        // Work from the total picker size (full) to keep the divider from
        // growing the picker when dragged.
        let full = self.clamped_position_and_size(layout, bounds, window);
        let width = match layout {
            Some(Layout::Right) => (full.right - full.left) - full.preview,
            _ => full.right - full.left,
        };
        let div = div.w(width);
        if fill_height {
            let height = match layout {
                Some(Layout::Below) => (full.bottom - full.top) - full.preview,
                _ => full.bottom - full.top,
            };
            div.h(height)
        } else {
            div
        }
    }

    pub(crate) fn results_max_height(
        &self,
        bounds: &SizeBounds,
        fill_height: bool,
        window: &Window,
    ) -> Option<Pixels> {
        if fill_height {
            None
        } else {
            Some(self.height(None, bounds, window))
        }
    }

    /// The clamped total height of the picker for the given layout.
    pub(crate) fn height(
        &self,
        layout: Option<Layout>,
        bounds: &SizeBounds,
        window: &Window,
    ) -> Pixels {
        let pos = self.clamped_position_and_size(layout, bounds, window);
        pos.bottom - pos.top
    }

    pub(crate) fn apply_height(
        &self,
        layout: Option<Layout>,
        bounds: &SizeBounds,
        div: Div,
        window: &Window,
    ) -> Div {
        div.h(self.height(layout, bounds, window))
    }

    pub(crate) fn results_height(
        &self,
        layout: Layout,
        bounds: &SizeBounds,
        window: &Window,
    ) -> Pixels {
        let pos = self.results_position_and_size(layout, bounds, window);
        pos.bottom - pos.top
    }

    pub(crate) fn preview_width(
        &self,
        layout: Layout,
        bounds: &SizeBounds,
        window: &Window,
    ) -> Pixels {
        let pos = self.preview_position_and_size(layout, bounds, window);
        pos.right - pos.left
    }

    pub(crate) fn preview_height(
        &self,
        layout: Layout,
        bounds: &SizeBounds,
        window: &Window,
    ) -> Pixels {
        let pos = self.preview_position_and_size(layout, bounds, window);
        pos.bottom - pos.top
    }

    pub(crate) fn centered_and_relative(
        pos: PositionAndShape,
        layout: impl Into<Option<Layout>>,
        window: &Window,
    ) -> Centered {
        use Layout as Pl;
        let preview_size = match layout.into() {
            Some(Pl::Below) => ViewportFraction::from_height_pixels(pos.preview, window),
            Some(Pl::Right) => ViewportFraction::from_width_pixels(pos.preview, window),
            Some(Pl::Hidden) | None => ViewportFraction::ZERO,
        };

        Centered {
            width: RelativeWidth::from_pixels(pos.right - pos.left, window),
            height: RelativeHeight::from_pixels(pos.bottom - pos.top, window),
            preview_size,
        }
    }

    pub(crate) fn set_initial_width(&mut self, w: impl Into<RelativeWidth>) {
        if let Shape::HorizontallyCentered(Centered { width, .. }) = self {
            *width = w.into();
        }
    }

    pub(crate) fn set_initial_height(&mut self, h: impl Into<RelativeHeight>) {
        if let Shape::HorizontallyCentered(Centered { height, .. }) = self {
            *height = h.into();
        }
    }

    pub(crate) fn reset_width(&mut self, default: &Centered) {
        if let Shape::HorizontallyCentered(Centered { width, .. }) = self {
            *width = default.width;
        }
    }

    pub(crate) fn reset_height(&mut self, default: &Centered) {
        if let Shape::HorizontallyCentered(Centered { height, .. }) = self {
            *height = default.height;
        }
    }

    pub(crate) fn center_divider(&mut self, layout: Layout, window: &Window) {
        if let Shape::HorizontallyCentered(Centered {
            width,
            height,
            preview_size,
        }) = self
        {
            let total = match layout {
                Layout::Right => width.as_viewport_fraction(window),
                Layout::Below => height.as_viewport_fraction(window),
                Layout::Hidden => return,
            };
            *preview_size = total * 0.5;
        }
    }
}

impl Default for Centered {
    fn default() -> Self {
        Centered {
            width: RelativeWidth::viewport(0.6),
            height: RelativeHeight::viewport(0.6),
            preview_size: ViewportFraction::fraction(0.3),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::HorizontallyCentered(Centered::default())
    }
}
