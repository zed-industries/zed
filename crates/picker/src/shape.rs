use gpui::Window;
use gpui::{Pixels, Rems};
use ui::{Div, Styled};

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

macro_rules! relative_size {
    ($name:ident, $accessor:ident) => {
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
                Self {
                    viewport_fraction: fraction,
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

        impl std::ops::Div<f32> for $name {
            type Output = Self;

            fn div(mut self, rhs: f32) -> Self::Output {
                self.viewport_fraction /= rhs;
                self.rems = Rems(self.rems.0 / rhs);
                self
            }
        }
    };
}

relative_size!(RelativeHeight, height);
relative_size!(RelativeWidth, width);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ViewportFraction(f32);

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
    /// Guarenteed to be between zero and one
    pub(crate) fn raw(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum VerticalPadding {
    /// The picker always fills its height even if there are no resutls
    #[default]
    Pad,
    /// Picker might be shorter then it's height if there is not enough to display
    None,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Centered {
    pub(crate) width: RelativeWidth,
    pub(crate) height: RelativeHeight,
    pub(crate) preview_size: ViewportFraction,
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
    pub(crate) min_width: Rems,
    pub(crate) max_height: RelativeWidth,
    pub(crate) min_height: Rems,
}

impl Default for SizeBounds {
    fn default() -> Self {
        Self {
            max_width: RelativeWidth::viewport(0.95),
            min_width: Rems(15.0),
            max_height: RelativeWidth::viewport(0.95),
            min_height: Rems(20.0),
        }
    }
}

impl SizeBounds {
    /// Clamps a width in pixels to the configured min/max width.
    pub(crate) fn clamp_width(&self, width: Pixels, window: &Window) -> Pixels {
        width
            .min(self.max_width.as_pixels(window))
            .max(self.min_width * window.rem_size())
    }

    /// Clamps a height in pixels to the configured min/max height.
    pub(crate) fn clamp_height(&self, height: Pixels, window: &Window) -> Pixels {
        height
            .min(self.max_height.as_pixels(window))
            .max(self.min_height * window.rem_size())
    }

    /// Clamps an in-progress resize back into bounds.
    pub(crate) fn clamp(
        &self,
        before: &PositionAndShape,
        working: &mut PositionAndShape,
        layout: Option<Layout>,
        window: &Window,
    ) {
        let target_width = self.clamp_width(working.right - working.left, window);
        let width_correction = if working.left != before.left {
            let new_left = working.right - target_width;
            let correction = new_left - working.left;
            working.left = new_left;
            correction
        } else {
            let new_right = working.left + target_width;
            let correction = new_right - working.right;
            working.right = new_right;
            correction
        };

        let target_height = self.clamp_height(working.bottom - working.top, window);
        let new_bottom = working.top + target_height;
        let height_correction = new_bottom - working.bottom;
        working.bottom = new_bottom;

        match layout {
            Some(Layout::Right) => working.preview += width_correction,
            Some(Layout::Below) => working.preview += height_correction,
            Some(Layout::Hidden) | None => {}
        }
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

    pub(crate) fn results_position_and_size(
        &self,
        layout: Layout,
        window: &Window,
    ) -> PositionAndShape {
        let mut pos = self.picker_position_and_size(layout, window);

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
        window: &Window,
    ) -> PositionAndShape {
        let mut pos = self.picker_position_and_size(layout, window);

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

    pub(crate) fn apply_picker_size(
        &self,
        layout: impl Into<Option<Layout>>,
        bounds: &SizeBounds,
        vertical_padding: VerticalPadding,
        div: Div,
        window: &Window,
    ) -> Div {
        let pos = if let Some(layout) = layout.into() {
            self.results_position_and_size(layout, window)
        } else {
            self.picker_position_and_size(None, window)
        };
        let width = bounds.clamp_width(pos.right - pos.left, window);
        let div = div.w(width);
        match vertical_padding {
            VerticalPadding::None => div,
            VerticalPadding::Pad => {
                let height = bounds.clamp_height(pos.bottom - pos.top, window);
                div.h(height)
            }
        }
    }

    pub(crate) fn results_max_height(
        &self,
        bounds: &SizeBounds,
        vertical_padding: VerticalPadding,
        window: &Window,
    ) -> Option<Pixels> {
        match vertical_padding {
            VerticalPadding::None => Some(bounds.clamp_height(self.height(window), window)),
            VerticalPadding::Pad => None,
        }
    }

    pub(crate) fn height(&self, window: &Window) -> Pixels {
        match self {
            Shape::Resizing(pos) => pos.bottom - pos.top,
            Shape::HorizontallyCentered(Centered { height, .. }) => height.as_pixels(window),
        }
    }

    pub(crate) fn apply_height(&self, div: Div, window: &Window) -> Div {
        div.h(self.height(window))
    }

    pub(crate) fn results_height(&self, layout: Layout, window: &mut Window) -> Pixels {
        let pos = self.results_position_and_size(layout, window);
        pos.bottom - pos.top
    }

    pub(crate) fn preview_width(&self, layout: Layout, window: &mut Window) -> Pixels {
        let pos = self.preview_position_and_size(layout, window);
        pos.right - pos.left
    }

    pub(crate) fn preview_height(&self, layout: Layout, window: &mut Window) -> Pixels {
        let pos = self.preview_position_and_size(layout, window);
        pos.bottom - pos.top
    }

    /// Resizing done, re-center the picker and use relative sizes instead of
    /// pixels again.
    pub(crate) fn centered_and_relative(
        pos: PositionAndShape,
        layout: impl Into<Option<Layout>>,
        window: &Window,
    ) -> Self {
        use Layout as Pl;
        let preview_size = match layout.into() {
            Some(Pl::Below) => ViewportFraction::from_height_pixels(pos.preview, window),
            Some(Pl::Right) => ViewportFraction::from_width_pixels(pos.preview, window),
            Some(Pl::Hidden) | None => ViewportFraction::ZERO,
        };

        Shape::HorizontallyCentered(Centered {
            width: RelativeWidth::from_pixels(pos.right - pos.left, window),
            height: RelativeHeight::from_pixels(pos.bottom - pos.top, window),
            preview_size,
        })
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
}

impl Default for Shape {
    fn default() -> Self {
        Self::HorizontallyCentered(Centered {
            width: RelativeWidth::viewport(0.6),
            height: RelativeHeight::viewport(0.6),
            preview_size: ViewportFraction::fraction(0.3),
        })
    }
}
