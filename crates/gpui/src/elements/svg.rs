use std::{
    fs,
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

use crate::{
    App, Asset, Bounds, Element, GlobalElementId, Hitbox, Hsla, InspectorElementId,
    InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels, Point, Radians, SharedString,
    Size, StyleRefinement, Styled, TransformationMatrix, Window, point, px, radians, size,
};
use gpui_util::ResultExt;

enum SvgSource {
    Embedded(SharedString),
    External(SharedString),
    Data {
        path: SharedString,
        bytes: Arc<[u8]>,
    },
}

/// The colors to use when painting an SVG.
#[derive(Clone, Copy, Debug)]
pub enum SvgColor {
    /// Tint the SVG's alpha mask with a single color.
    Monochrome(Hsla),
    /// Preserve the SVG's original colors.
    Polychrome,
}

/// An SVG element.
pub struct Svg {
    interactivity: Interactivity,
    transformation: Option<Transformation>,
    source: SvgSource,
    polychrome: bool,
}

/// Create an SVG element from an embedded asset path.
#[track_caller]
pub fn svg(path: impl Into<SharedString>) -> Svg {
    Svg::new(SvgSource::Embedded(path.into()))
}

impl Svg {
    #[track_caller]
    fn new(source: SvgSource) -> Self {
        Self {
            interactivity: Interactivity::new(),
            transformation: None,
            source,
            polychrome: false,
        }
    }

    /// Create an SVG element from a file on disk.
    #[track_caller]
    pub fn from_external_path(path: impl Into<SharedString>) -> Self {
        Self::new(SvgSource::External(path.into()))
    }

    /// Render the SVG using its original colors instead of the text color.
    ///
    /// Polychrome SVGs do not support [`Self::with_transformation`].
    pub fn polychrome(mut self) -> Self {
        self.polychrome = true;
        self
    }

    /// Create an SVG element from raw SVG data.
    #[track_caller]
    pub fn from_data(data: &[u8]) -> Self {
        // Generate a unique deterministic path based on the data hash for caching
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut hasher);
        let hash = hasher.finish();
        let path = SharedString::from(format!("__binary_svg__{}", hash));
        Self::new(SvgSource::Data {
            path,
            bytes: Arc::from(data),
        })
    }

    /// Transform the SVG element with the given transformation.
    /// Note that this won't effect the hitbox or layout of the element, only the rendering.
    pub fn with_transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = Some(transformation);
        self
    }
}

impl Element for Svg {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<crate::ElementId> {
        self.interactivity.element_id.clone()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.interactivity.source_location()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| window.request_layout(style, None, cx),
        );
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Hitbox> {
        self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_, _, hitbox, _, _| hitbox,
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) where
        Self: Sized,
    {
        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |style, window, cx| {
                let color = if self.polychrome {
                    debug_assert!(
                        self.transformation.is_none(),
                        "polychrome SVGs do not support transformations"
                    );
                    SvgColor::Polychrome
                } else if let Some(color) = style.text.color {
                    SvgColor::Monochrome(color)
                } else {
                    return;
                };

                let (path, bytes) = match &self.source {
                    SvgSource::Embedded(path) => (path, None),
                    SvgSource::External(path) => {
                        let Some(bytes) = window
                            .use_asset::<SvgAsset>(path, cx)
                            .and_then(|asset| asset.log_err())
                        else {
                            return;
                        };
                        (path, Some(bytes))
                    }
                    SvgSource::Data { path, bytes } => (path, Some(bytes.clone())),
                };

                let transformation = self
                    .transformation
                    .as_ref()
                    .map(|transformation| {
                        transformation.into_matrix(bounds.center(), window.scale_factor())
                    })
                    .unwrap_or_default();

                window
                    .paint_svg(
                        bounds,
                        path.clone(),
                        bytes.as_deref(),
                        transformation,
                        color,
                        cx,
                    )
                    .log_err();
            },
        )
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
        //Note: if you read this as a sequence of matrix multiplications, start from the bottom
        TransformationMatrix::unit()
            .translate(center.scale(scale_factor) + self.translate.scale(scale_factor))
            .rotate(self.rotate)
            .scale(self.scale)
            .translate(center.scale(-scale_factor))
    }
}

enum SvgAsset {}

impl Asset for SvgAsset {
    type Source = SharedString;
    type Output = Result<Arc<[u8]>, Arc<std::io::Error>>;

    fn load(
        source: Self::Source,
        _cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        async move {
            let bytes = fs::read(Path::new(source.as_ref())).map_err(|e| Arc::new(e))?;
            let bytes = Arc::from(bytes);
            Ok(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AppContext as _, AtlasTextureKind, Context, DevicePixels, ParentElement, Render,
        ScaledPixels, TestAppContext, div, hsla,
    };

    #[gpui::test]
    fn monochrome_and_polychrome_svg_share_geometry_but_not_atlas_tiles(cx: &mut TestAppContext) {
        struct SvgView;

        impl Render for SvgView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let data = br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2 1"><rect width="2" height="1" fill="#ff0000"/></svg>"##;
                div()
                    .size(px(24.))
                    .opacity(0.5)
                    .child(
                        Svg::from_data(data)
                            .absolute()
                            .size_full()
                            .text_color(hsla(0.5, 1., 0.5, 0.8))
                            .with_transformation(Transformation::translate(point(px(1.), px(2.)))),
                    )
                    .child(Svg::from_data(data).polychrome().absolute().size_full())
            }
        }

        let window = cx.add_window(|_, _| SvgView);
        for (scale, top) in [(1., 6.), (1.25, 7.), (2., 12.)] {
            cx.simulate_window_scale_factor_change(window.into(), scale);
            cx.update_window(window.into(), |_, window, cx| {
                window.draw(cx).clear(cx);
                let scene = &window.rendered_frame.scene;
                assert_eq!(scene.monochrome_sprites.len(), 1);
                assert_eq!(scene.polychrome_sprites.len(), 1);
                let monochrome = scene.monochrome_sprites.first().expect("monochrome sprite");
                let polychrome = scene.polychrome_sprites.first().expect("polychrome sprite");

                assert_eq!(monochrome.bounds, polychrome.bounds);
                assert_eq!(
                    polychrome.bounds.size,
                    size(px(24.).scale(scale), px(12.).scale(scale))
                );
                assert_eq!(
                    polychrome.bounds.origin,
                    point(ScaledPixels(0.), ScaledPixels(top))
                );
                assert_eq!(
                    monochrome.tile.texture_id.kind,
                    AtlasTextureKind::Monochrome
                );
                assert_eq!(
                    polychrome.tile.texture_id.kind,
                    AtlasTextureKind::Polychrome
                );
                assert_ne!(monochrome.tile, polychrome.tile);
                assert_eq!(
                    polychrome.tile.bounds.size,
                    size(
                        DevicePixels((48. * scale) as i32),
                        DevicePixels((24. * scale) as i32)
                    )
                );
                assert_eq!(monochrome.color, hsla(0.5, 1., 0.5, 0.4));
                assert_eq!(
                    monochrome.transformation,
                    TransformationMatrix::unit()
                        .translate(point(px(1.).scale(scale), px(2.).scale(scale)))
                );
                assert_eq!(polychrome.opacity, 0.5);
                assert_eq!(polychrome.premultiplied_alpha, true.into());
            })
            .expect("window should render");
        }
    }
}
