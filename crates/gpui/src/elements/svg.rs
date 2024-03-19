use crate::{
    geometry::Negate as _, point, px, radians, size, Bounds, Element, ElementContext, Hitbox,
    InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels, Point, Radians, SharedString,
    Size, StyleRefinement, Styled, TransformationMatrix,
};
use util::ResultExt;

/// An SVG element.
pub struct Svg {
    interactivity: Interactivity,
    transformation: Option<Transformation>,
    path: Option<SharedString>,
}

/// Create a new SVG element.
pub fn svg() -> Svg {
    Svg {
        interactivity: Interactivity::default(),
        transformation: None,
        path: None,
    }
}

impl Svg {
    /// Set the path to the SVG file for this element.
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Transform the SVG element with the given transformation.
    /// Note that this won't effect the hitbox or layout of the element, only the rendering.
    pub fn with_transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = Some(transformation);
        self
    }
}

impl Element for Svg {
    type BeforeLayout = ();
    type AfterLayout = Option<Hitbox>;

    fn before_layout(&mut self, cx: &mut ElementContext) -> (LayoutId, Self::BeforeLayout) {
        let layout_id = self
            .interactivity
            .before_layout(cx, |style, cx| cx.request_layout(&style, None));
        (layout_id, ())
    }

    fn after_layout(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        cx: &mut ElementContext,
    ) -> Option<Hitbox> {
        self.interactivity
            .after_layout(bounds, bounds.size, cx, |_, _, hitbox, _| hitbox)
    }

    fn paint(
        &mut self,
        bounds: Bounds<Pixels>,
        _before_layout: &mut Self::BeforeLayout,
        hitbox: &mut Option<Hitbox>,
        cx: &mut ElementContext,
    ) where
        Self: Sized,
    {
        self.interactivity
            .paint(bounds, hitbox.as_ref(), cx, |style, cx| {
                if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                    let transformation = self
                        .transformation
                        .as_ref()
                        .map(|transformation| {
                            transformation.into_matrix(bounds.center(), cx.scale_factor())
                        })
                        .unwrap_or_default();

                    cx.paint_svg(bounds, path.clone(), transformation, color)
                        .log_err();
                }
            })
    }
}

impl IntoElement for Svg {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for Svg {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl InteractiveElement for Svg {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

/// A transformation to apply to an SVG element.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transformation {
    scale: Size<f32>,
    translate: Point<Pixels>,
    rotate: Radians,
}

impl Default for Transformation {
    fn default() -> Self {
        Self {
            scale: size(1.0, 1.0),
            translate: point(px(0.0), px(0.0)),
            rotate: radians(0.0),
        }
    }
}

impl Transformation {
    /// Create a new Transformation with the specified scale along each axis.
    pub fn scale(scale: Size<f32>) -> Self {
        Self {
            scale,
            translate: point(px(0.0), px(0.0)),
            rotate: radians(0.0),
        }
    }

    /// Create a new Transformation with the specified translation.
    pub fn translate(translate: Point<Pixels>) -> Self {
        Self {
            scale: size(1.0, 1.0),
            translate,
            rotate: radians(0.0),
        }
    }

    /// Create a new Transformation with the specified rotation in radians.
    pub fn rotate(rotate: impl Into<Radians>) -> Self {
        let rotate = rotate.into();
        Self {
            scale: size(1.0, 1.0),
            translate: point(px(0.0), px(0.0)),
            rotate,
        }
    }

    /// Update the scaling factor of this transformation.
    pub fn with_scaling(mut self, scale: Size<f32>) -> Self {
        self.scale = scale;
        self
    }

    /// Update the translation value of this transformation.
    pub fn with_translation(mut self, translate: Point<Pixels>) -> Self {
        self.translate = translate;
        self
    }

    /// Update the rotation angle of this transformation.
    pub fn with_rotation(mut self, rotate: impl Into<Radians>) -> Self {
        self.rotate = rotate.into();
        self
    }

    fn into_matrix(self, center: Point<Pixels>, scale_factor: f32) -> TransformationMatrix {
        //Note: if you read this as a sequence of matrix mulitplications, start from the bottom
        TransformationMatrix::unit()
            .translate(center.scale(scale_factor) + self.translate.scale(scale_factor))
            .rotate(self.rotate)
            .scale(self.scale)
            .translate(center.scale(scale_factor).negate())
    }
}
