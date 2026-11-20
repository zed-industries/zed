use std::{
    f32::consts::TAU,
    fs,
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
    time::Duration,
};

use crate::{
    App, Asset, Bounds, Element, GlobalElementId, Hitbox, InspectorElementId, InteractiveElement,
    Interactivity, LayoutId, Pixels, Point, Radians, SharedString, Size, SpriteAnimation,
    StyleRefinement, Styled, TransformationMatrix, Window, ease_in_out, ease_out_quint, heartbeat,
    linear, point, px, quadratic, radians, size,
};
use gpui_util::ResultExt;

/// Immutable SVG bytes that can be reused across elements without copying or rehashing.
///
/// Static data uses the slice's address and length as its cache identity. Shared data
/// uses its contents, so independently constructed values with equal bytes share a cache entry.
/// Static and shared data have separate cache identities.
#[derive(Clone, Debug)]
pub struct SvgData(SvgDataStorage);

#[derive(Clone, Debug)]
enum SvgDataStorage {
    Static(&'static [u8]),
    Shared { bytes: Arc<[u8]>, hash: u64 },
}

impl SvgData {
    /// Borrow embedded SVG bytes without allocating or copying.
    pub const fn from_static(bytes: &'static [u8]) -> Self {
        Self(SvgDataStorage::Static(bytes))
    }

    /// Retain shared SVG bytes without copying them, computing their hash once.
    pub fn from_shared(bytes: Arc<[u8]>) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut hasher);
        Self(SvgDataStorage::Shared {
            bytes,
            hash: hasher.finish(),
        })
    }

    /// Copy borrowed SVG bytes into shared storage.
    pub fn copy_from_slice(bytes: &[u8]) -> Self {
        Self::from_shared(Arc::from(bytes))
    }

    /// Return the original SVG bytes.
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            SvgDataStorage::Static(bytes) => bytes,
            SvgDataStorage::Shared { bytes, .. } => bytes,
        }
    }
}

impl PartialEq for SvgData {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (SvgDataStorage::Static(left), SvgDataStorage::Static(right)) => {
                std::ptr::eq(*left, *right)
            }
            (
                SvgDataStorage::Shared {
                    bytes: left,
                    hash: left_hash,
                },
                SvgDataStorage::Shared {
                    bytes: right,
                    hash: right_hash,
                },
            ) => left_hash == right_hash && (Arc::ptr_eq(left, right) || left == right),
            _ => false,
        }
    }
}

impl Eq for SvgData {}

impl Hash for SvgData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(&self.0).hash(state);
        match &self.0 {
            SvgDataStorage::Static(bytes) => {
                bytes.as_ptr().hash(state);
                bytes.len().hash(state);
            }
            SvgDataStorage::Shared { hash, .. } => hash.hash(state),
        }
    }
}

impl From<Arc<[u8]>> for SvgData {
    fn from(bytes: Arc<[u8]>) -> Self {
        Self::from_shared(bytes)
    }
}

impl From<&[u8]> for SvgData {
    fn from(bytes: &[u8]) -> Self {
        Self::copy_from_slice(bytes)
    }
}

impl<const N: usize> From<&[u8; N]> for SvgData {
    fn from(bytes: &[u8; N]) -> Self {
        Self::copy_from_slice(bytes)
    }
}

impl From<&Vec<u8>> for SvgData {
    fn from(bytes: &Vec<u8>) -> Self {
        Self::copy_from_slice(bytes)
    }
}

impl From<&Arc<[u8]>> for SvgData {
    fn from(bytes: &Arc<[u8]>) -> Self {
        Self::from_shared(bytes.clone())
    }
}

/// An SVG element.
pub struct Svg {
    interactivity: Interactivity,
    transformation: Option<TimedTransformation>,
    path: Option<SharedString>,
    external_path: Option<SharedString>,
    data: Option<SvgData>,
}

/// Create a new SVG element.
#[track_caller]
pub fn svg() -> Svg {
    Svg {
        interactivity: Interactivity::new(),
        transformation: None,
        path: None,
        external_path: None,
        data: None,
    }
}

impl Svg {
    /// Set the path to the SVG file for this element.
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Set the path to the SVG file for this element.
    pub fn external_path(mut self, path: impl Into<SharedString>) -> Self {
        self.external_path = Some(path.into());
        self
    }

    /// Set the raw SVG data for this element.
    ///
    /// Borrowed slices are copied. Pass [`SvgData::from_static`] to borrow embedded
    /// bytes, or an [`Arc<[u8]>`] to retain shared bytes without copying. Reuse an
    /// [`SvgData`] to also avoid hashing its contents on each element construction.
    ///
    /// ```
    /// use gpui::{SvgData, svg};
    ///
    /// const DATA: SvgData = SvgData::from_static(
    ///     br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"/>"#,
    /// );
    /// let element = svg().data(DATA);
    /// ```
    pub fn data(mut self, data: impl Into<SvgData>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Transform the SVG element with the given transformation.
    /// Note that this won't effect the hitbox or layout of the element, only the rendering.
    pub fn with_transformation(mut self, transformation: Transformation) -> Self {
        self.transformation = Some(transformation.into());
        self
    }

    /// Continuously animate the SVG between two transformations, phase-locked to the
    /// app's shared animation clock like [`crate::Animation::repeat_synced`].
    ///
    /// The SVG is painted once. On later frames the window moves it in the scene it
    /// already has and presents that again, instead of re-rendering this view.
    ///
    /// This replaces [`Self::with_transformation`], and like it doesn't affect layout
    /// or hit testing. The SVG is still clipped and ordered by the bounds of its
    /// starting transformation, so keep its motion close to them.
    pub fn with_timed_transformation(mut self, transformation: TimedTransformation) -> Self {
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
                let center = bounds.center();
                let scale_factor = window.scale_factor();
                let animation = self
                    .transformation
                    .filter(|timed| timed.is_animated() && !cx.reduce_motion())
                    .map(|timed| SpriteAnimation::new(timed, center, scale_factor));
                // Without an animation, such as with reduced motion, the SVG is
                // drawn at its starting transformation.
                let pose = self.transformation.map(|timed| timed.from);
                let transformation = pose
                    .map(|pose| pose.into_matrix(center, scale_factor))
                    .unwrap_or_default();
                // The scene applies an animated sprite's alpha before each present,
                // on top of the color it is inserted with.
                let alpha = match animation {
                    Some(_) => 1.,
                    None => pose.map_or(1., |pose| pose.alpha_factor()),
                };

                if let Some(data) = self.data.as_ref() {
                    if let Some(color) = style.text.color {
                        window
                            .paint_svg_data(
                                bounds,
                                data,
                                transformation,
                                animation,
                                color.opacity(alpha),
                                cx,
                            )
                            .log_err();
                    }
                } else if let Some((path, color)) =
                    self.external_path.as_ref().zip(style.text.color)
                {
                    let Some(bytes) = window
                        .use_asset::<SvgAsset>(path, cx)
                        .and_then(|asset| asset.log_err())
                    else {
                        return;
                    };

                    window
                        .paint_animated_svg(
                            bounds,
                            path.clone(),
                            Some(&bytes),
                            transformation,
                            animation,
                            color.opacity(alpha),
                            cx,
                        )
                        .log_err();
                } else if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                    window
                        .paint_animated_svg(
                            bounds,
                            path.clone(),
                            None,
                            transformation,
                            animation,
                            color.opacity(alpha),
                            cx,
                        )
                        .log_err();
                }
            },
        )
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
    alpha: f32,
}

impl Default for Transformation {
    fn default() -> Self {
        Self {
            scale: size(1.0, 1.0),
            translate: point(px(0.0), px(0.0)),
            rotate: radians(0.0),
            alpha: 1.0,
        }
    }
}

impl Transformation {
    /// Create a new Transformation with the specified scale along each axis.
    pub fn scale(scale: Size<f32>) -> Self {
        Self {
            scale,
            ..Self::default()
        }
    }

    /// Create a new Transformation with the specified translation.
    pub fn translate(translate: Point<Pixels>) -> Self {
        Self {
            translate,
            ..Self::default()
        }
    }

    /// Create a new Transformation with the specified rotation in radians.
    pub fn rotate(rotate: impl Into<Radians>) -> Self {
        let rotate = rotate.into();
        Self {
            rotate,
            ..Self::default()
        }
    }

    /// Create a new Transformation that multiplies the SVG's alpha by `alpha`.
    pub fn alpha(alpha: f32) -> Self {
        Self {
            alpha,
            ..Self::default()
        }
    }

    /// Update the alpha factor of this transformation. Unlike an element's
    /// opacity, this only applies to the SVG itself.
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    pub(crate) fn alpha_factor(&self) -> f32 {
        self.alpha
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

    pub(crate) fn into_matrix(
        self,
        center: Point<Pixels>,
        scale_factor: f32,
    ) -> TransformationMatrix {
        //Note: if you read this as a sequence of matrix multiplications, start from the bottom
        TransformationMatrix::unit()
            .translate(center.scale(scale_factor) + self.translate.scale(scale_factor))
            .rotate(self.rotate)
            .scale(self.scale)
            .translate(center.scale(-scale_factor))
    }
}

/// How a value's rate of change varies over one pass of a [`TimedTransformation`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Easing {
    /// A constant rate.
    #[default]
    Linear,
    /// Starts slowly and accelerates.
    Quadratic,
    /// Starts and ends slowly, and speeds up in the middle.
    EaseInOut,
    /// Starts quickly and decelerates to a stop.
    EaseOutQuint,
    /// Two quick beats, a strong one then a weaker one, followed by a rest.
    /// Unlike the other easings it ends where it started, so the value
    /// reaches `to` at the first beat and is back at `from` by the end.
    Heartbeat,
}

impl Easing {
    fn apply(self, delta: f32) -> f32 {
        match self {
            Easing::Linear => linear(delta),
            Easing::Quadratic => quadratic(delta),
            Easing::EaseInOut => ease_in_out(delta),
            Easing::EaseOutQuint => ease_out_quint()(delta),
            Easing::Heartbeat => heartbeat(delta),
        }
    }
}

/// A [`Transformation`] that repeatedly moves from one pose to another over a
/// period, for [`Svg::with_timed_transformation`].
///
/// Each value of the transformation has its own [`Easing`], so for example an
/// SVG can grow quickly at first while fading at a constant rate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimedTransformation {
    period: Duration,
    from: Transformation,
    to: Transformation,
    scale_easing: Easing,
    translation_easing: Easing,
    rotation_easing: Easing,
    alpha_easing: Easing,
    alternate: bool,
    min_frame_interval: Option<Duration>,
}

impl TimedTransformation {
    /// Create a timed transformation that repeats every `period`. It animates
    /// nothing until [`Self::from`] and [`Self::to`] differ.
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            from: Transformation::default(),
            to: Transformation::default(),
            scale_easing: Easing::Linear,
            translation_easing: Easing::Linear,
            rotation_easing: Easing::Linear,
            alpha_easing: Easing::Linear,
            alternate: false,
            min_frame_interval: None,
        }
    }

    /// Create a timed transformation that turns `from` clockwise, one full
    /// turn per `period`.
    pub fn rotation(period: Duration, from: Transformation) -> Self {
        Self::new(period)
            .from(from)
            .to(from.with_rotation(radians(from.rotate.0 + TAU)))
    }

    fn is_animated(&self) -> bool {
        !self.period.is_zero() && self.from != self.to
    }

    /// Set the transformation at the start of each period.
    pub fn from(mut self, transformation: Transformation) -> Self {
        self.from = transformation;
        self
    }

    /// Set the transformation reached at the end of each period.
    pub fn to(mut self, transformation: Transformation) -> Self {
        self.to = transformation;
        self
    }

    /// Set the easing of every value.
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.scale_easing = easing;
        self.translation_easing = easing;
        self.rotation_easing = easing;
        self.alpha_easing = easing;
        self
    }

    /// Set the easing of the scale.
    pub fn with_scale_easing(mut self, easing: Easing) -> Self {
        self.scale_easing = easing;
        self
    }

    /// Set the easing of the translation.
    pub fn with_translation_easing(mut self, easing: Easing) -> Self {
        self.translation_easing = easing;
        self
    }

    /// Set the easing of the rotation.
    pub fn with_rotation_easing(mut self, easing: Easing) -> Self {
        self.rotation_easing = easing;
        self
    }

    /// Set the easing of the alpha.
    pub fn with_alpha_easing(mut self, easing: Easing) -> Self {
        self.alpha_easing = easing;
        self
    }

    /// Move from the start to the end and back again within each period,
    /// instead of jumping back to the start.
    pub fn alternate(mut self) -> Self {
        self.alternate = true;
        self
    }

    /// Present at most `max_fps` frames per second while nothing but timed
    /// transformations needs a frame. The least restrictive limit in a window
    /// wins. Values that are not finite and positive are ignored.
    pub fn with_max_fps(mut self, max_fps: f32) -> Self {
        if max_fps.is_finite() && max_fps > 0.0 {
            self.min_frame_interval = Some(Duration::from_secs_f64(1.0 / max_fps as f64));
        }
        self
    }

    pub(crate) fn min_frame_interval(&self) -> Option<Duration> {
        self.min_frame_interval
    }

    /// The transformation `elapsed` time after the animation clock started.
    pub(crate) fn at(&self, elapsed: Duration) -> Transformation {
        let period_nanos = self.period.as_nanos();
        if period_nanos == 0 {
            return self.from;
        }
        let mut delta = Duration::from_nanos((elapsed.as_nanos() % period_nanos) as u64)
            .as_secs_f32()
            / self.period.as_secs_f32();
        if self.alternate {
            delta = 1.0 - (2.0 * delta - 1.0).abs();
        }

        let interpolate =
            |from: f32, to: f32, easing: Easing| from + (to - from) * easing.apply(delta);
        let (from, to) = (self.from, self.to);
        Transformation {
            scale: size(
                interpolate(from.scale.width, to.scale.width, self.scale_easing),
                interpolate(from.scale.height, to.scale.height, self.scale_easing),
            ),
            translate: point(
                px(interpolate(
                    from.translate.x.0,
                    to.translate.x.0,
                    self.translation_easing,
                )),
                px(interpolate(
                    from.translate.y.0,
                    to.translate.y.0,
                    self.translation_easing,
                )),
            ),
            rotate: radians(interpolate(
                from.rotate.0,
                to.rotate.0,
                self.rotation_easing,
            )),
            alpha: interpolate(from.alpha, to.alpha, self.alpha_easing),
        }
    }
}

impl From<Transformation> for TimedTransformation {
    /// A timed transformation that stays at `transformation`.
    fn from(transformation: Transformation) -> Self {
        Self::new(Duration::ZERO)
            .from(transformation)
            .to(transformation)
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
    use crate::{AtlasKey, DevicePixels, RenderSvgParams};
    use std::collections::HashMap;

    static BYTES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><path d="M0 0h24v24H0z"/></svg>"#;
    const STATIC_DATA: SvgData = SvgData::from_static(BYTES);

    #[test]
    fn static_data_borrows_bytes() {
        let data = STATIC_DATA;
        assert!(std::ptr::eq(data.as_bytes(), BYTES));
        let element = svg().data(data.clone());
        assert_eq!(data, STATIC_DATA);
        assert!(
            element
                .data
                .is_some_and(|cloned| std::ptr::eq(cloned.as_bytes(), BYTES))
        );
    }

    #[test]
    fn shared_data_retains_allocation() {
        let bytes: Arc<[u8]> = Arc::from(BYTES);
        let data = SvgData::from_shared(bytes.clone());
        assert!(std::ptr::eq(data.as_bytes(), bytes.as_ref()));
        let element = svg().data(data.clone());
        assert_eq!(Arc::strong_count(&bytes), 3);
        drop(data);
        assert!(
            element
                .data
                .as_ref()
                .is_some_and(|cloned| std::ptr::eq(cloned.as_bytes(), bytes.as_ref()))
        );
        assert_eq!(Arc::strong_count(&bytes), 2);
    }

    #[test]
    fn borrowed_data_owns_a_copy() {
        let mut bytes = BYTES.to_vec();
        let data = SvgData::copy_from_slice(&bytes);
        assert_ne!(data.as_bytes().as_ptr(), bytes.as_ptr());
        bytes.fill(0);
        drop(bytes);
        assert_eq!(data.as_bytes(), BYTES);
        assert!(std::ptr::eq(data.as_bytes(), data.clone().as_bytes()));
    }

    #[test]
    fn data_accepts_existing_and_shared_inputs() {
        let bytes = BYTES.to_vec();
        let shared: Arc<[u8]> = Arc::from(BYTES);
        let elements = [
            svg().data(BYTES),
            svg().data(b"svg bytes"),
            svg().data(include_bytes!("../../examples/svg/dragon.svg")),
            svg().data(&bytes),
            svg().data(&shared),
            svg().data(shared.clone()),
            svg().data(STATIC_DATA),
            svg().data(SvgData::from_shared(shared)),
        ];
        assert!(elements.iter().all(|element| element.data.is_some()));
    }

    #[test]
    fn cache_keys_preserve_content_size_and_source_identity() {
        let small = size(DevicePixels(24), DevicePixels(24));
        let large = size(DevicePixels(48), DevicePixels(48));
        let data = SvgData::copy_from_slice(BYTES);
        let mut cache = HashMap::new();
        cache.insert(AtlasKey::SvgData(data.clone(), small), 1);
        assert_eq!(cache.get(&AtlasKey::SvgData(data.clone(), small)), Some(&1));
        assert_eq!(
            cache.get(&AtlasKey::SvgData(SvgData::copy_from_slice(BYTES), small)),
            Some(&1)
        );
        assert!(!cache.contains_key(&AtlasKey::SvgData(data, large)));
        assert!(!cache.contains_key(&AtlasKey::SvgData(STATIC_DATA, small)));
        assert!(!cache.contains_key(&AtlasKey::Svg(RenderSvgParams {
            path: "__binary_svg__0".into(),
            size: small,
        })));

        cache.insert(AtlasKey::SvgData(STATIC_DATA, small), 2);
        assert_eq!(cache.get(&AtlasKey::SvgData(STATIC_DATA, small)), Some(&2));
        let shorter = SvgData::from_static(&BYTES[..BYTES.len() - 1]);
        assert!(!cache.contains_key(&AtlasKey::SvgData(shorter, small)));
    }

    #[test]
    fn static_and_shared_data_render_the_same_mask() -> anyhow::Result<()> {
        let renderer = crate::SvgRenderer::new(Arc::new(()));
        let shared = SvgData::copy_from_slice(BYTES);
        for dimension in [24, 48] {
            let params = RenderSvgParams {
                path: SharedString::default(),
                size: size(DevicePixels(dimension), DevicePixels(dimension)),
            };
            let static_mask = renderer.render_alpha_mask(&params, Some(STATIC_DATA.as_bytes()))?;
            let shared_mask = renderer.render_alpha_mask(&params, Some(shared.as_bytes()))?;
            assert_eq!(static_mask, shared_mask);
            let Some((rendered_size, pixels)) = static_mask else {
                anyhow::bail!("SVG produced no mask");
            };
            assert_eq!(rendered_size, params.size);
            assert_eq!(pixels.len(), (dimension * dimension) as usize);
            assert!(pixels.iter().all(|alpha| *alpha == 255));
        }
        assert!(
            renderer
                .render_alpha_mask(
                    &RenderSvgParams {
                        path: SharedString::default(),
                        size: size(DevicePixels(24), DevicePixels(24))
                    },
                    Some(b"invalid SVG"),
                )
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn hash_collisions_do_not_alias_different_contents() {
        let first = SvgData(SvgDataStorage::Shared {
            bytes: Arc::from(b"first".as_slice()),
            hash: 0,
        });
        let second = SvgData(SvgDataStorage::Shared {
            bytes: Arc::from(b"other".as_slice()),
            hash: 0,
        });
        let mut cache = HashMap::new();
        cache.insert(first, 1);
        assert!(!cache.contains_key(&second));
        cache.insert(second, 2);
        assert_eq!(cache.len(), 2);
    }

    use std::{cell::Cell, f32::consts::FRAC_PI_2, rc::Rc, time::Duration};

    use crate::{
        Context, Easing, IntoElement, Render, SpriteAnimationId, Styled, TestAppContext,
        TimedTransformation, Transformation, VisualTestContext, Window, div, prelude::*, px, red,
        size, svg,
    };

    const SQUARE_SVG: &[u8] =
        br#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16"><rect width="8" height="16"/></svg>"#;

    struct AnimatedSvgView {
        data: SvgData,
        transformation: TimedTransformation,
        renders: Rc<Cell<usize>>,
    }

    impl Render for AnimatedSvgView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            self.renders.set(self.renders.get() + 1);
            div().child(
                svg()
                    .data(self.data.clone())
                    .size(px(16.))
                    .text_color(red())
                    .with_timed_transformation(self.transformation),
            )
        }
    }

    struct Pose {
        rotation_scale: [[f32; 2]; 2],
        center_offset: [f32; 2],
        alpha: f32,
    }

    /// Opens a window showing `transformation` and returns a function that
    /// delivers a frame `advance` later and reports the animated sprite's pose,
    /// along with the number of times the view has rendered.
    fn animate(
        transformation: TimedTransformation,
        cx: &mut TestAppContext,
    ) -> (impl FnMut(Duration) -> Pose, Rc<Cell<usize>>) {
        animate_data(SvgData::copy_from_slice(SQUARE_SVG), transformation, cx)
    }

    fn animate_data(
        data: SvgData,
        transformation: TimedTransformation,
        cx: &mut TestAppContext,
    ) -> (impl FnMut(Duration) -> Pose, Rc<Cell<usize>>) {
        let renders = Rc::new(Cell::new(0));
        let (_, cx) = cx.add_window_view({
            let renders = renders.clone();
            move |_, _| AnimatedSvgView {
                data,
                transformation,
                renders,
            }
        });
        cx.run_until_parked();
        let cx: &mut VisualTestContext = cx;
        let pose_after = move |advance: Duration| {
            cx.executor().advance_clock(advance);
            cx.update(|window, cx| {
                window.simulate_next_frame(cx);
                window.advance_scene_animations(cx);
                let sprite = window
                    .rendered_frame
                    .scene
                    .monochrome_sprites
                    .iter()
                    .find(|sprite| sprite.animation != SpriteAnimationId::NONE)
                    .expect("the SVG should be painted as an animated sprite");
                let center = sprite.bounds.center();
                let [[a, b], [c, d]] = sprite.transformation.rotation_scale;
                let [x, y] = sprite.transformation.translation;
                Pose {
                    rotation_scale: sprite.transformation.rotation_scale,
                    center_offset: [
                        a * center.x.0 + b * center.y.0 + x - center.x.0,
                        c * center.x.0 + d * center.y.0 + y - center.y.0,
                    ],
                    alpha: sprite.color.a,
                }
            })
        };
        (pose_after, renders)
    }

    #[crate::test]
    fn test_static_data_animates_without_rerendering(cx: &mut TestAppContext) {
        assert_data_animates_without_rerendering(SvgData::from_static(SQUARE_SVG), cx);
    }

    #[crate::test]
    fn test_shared_data_animates_without_rerendering(cx: &mut TestAppContext) {
        assert_data_animates_without_rerendering(SvgData::from_shared(Arc::from(SQUARE_SVG)), cx);
    }

    fn assert_data_animates_without_rerendering(data: SvgData, cx: &mut TestAppContext) {
        let (mut pose_after, renders) = animate_data(
            data,
            TimedTransformation::rotation(Duration::from_secs(1), Transformation::default()),
            cx,
        );
        let renders_before_frames = renders.get();
        let initial = pose_after(Duration::ZERO);
        assert!((initial.rotation_scale[0][0] - 1.).abs() < 1e-5);
        let quarter = pose_after(Duration::from_millis(250));
        assert!(quarter.rotation_scale[0][0].abs() < 1e-5);
        assert!((quarter.rotation_scale[0][1] + 1.).abs() < 1e-5);
        assert!((quarter.alpha - 1.).abs() < 1e-5);
        assert_eq!(renders.get(), renders_before_frames);
    }

    #[crate::test]
    fn test_rotation_advances_without_rerendering(cx: &mut TestAppContext) {
        let (mut pose_after, renders) = animate(
            TimedTransformation::rotation(Duration::from_secs(1), Transformation::default()),
            cx,
        );
        let renders_before_frames = renders.get();

        for quarter in 0..4 {
            let pose = pose_after(if quarter == 0 {
                Duration::ZERO
            } else {
                Duration::from_millis(250)
            });
            let (sin, cos) = (quarter as f32 * FRAC_PI_2).sin_cos();
            let expected = [[cos, -sin], [sin, cos]];
            for (actual, expected) in pose
                .rotation_scale
                .as_flattened()
                .iter()
                .zip(expected.as_flattened())
            {
                assert!((actual - expected).abs() < 1e-5, "quarter {quarter}");
            }
            assert!(pose.center_offset.iter().all(|offset| offset.abs() < 1e-3));
        }
        assert_eq!(renders.get(), renders_before_frames);
    }

    #[crate::test]
    fn test_timed_transformation_eases_each_value_separately(cx: &mut TestAppContext) {
        let (mut pose_after, renders) = animate(
            TimedTransformation::new(Duration::from_secs(1))
                .from(Transformation::alpha(0.4))
                .to(Transformation::scale(size(1.5, 1.5)).with_alpha(0.))
                .with_scale_easing(Easing::EaseOutQuint),
            cx,
        );
        let renders_before_frames = renders.get();

        let pose = pose_after(Duration::ZERO);
        assert!((pose.rotation_scale[0][0] - 1.).abs() < 1e-5);
        assert!((pose.alpha - 0.4).abs() < 1e-5);

        // Halfway through, the scale has eased most of the way out while the
        // alpha has faded exactly half.
        let pose = pose_after(Duration::from_millis(500));
        assert!((pose.rotation_scale[0][0] - (1. + 0.5 * (1. - 0.5f32.powi(5)))).abs() < 1e-5);
        assert!((pose.alpha - 0.2).abs() < 1e-5);
        assert_eq!(renders.get(), renders_before_frames);
    }

    #[crate::test]
    fn test_heartbeat_returns_to_its_start(cx: &mut TestAppContext) {
        let (mut pose_after, _) = animate(
            TimedTransformation::new(Duration::from_secs(1))
                .to(Transformation::scale(size(1.12, 1.12)))
                .with_scale_easing(Easing::Heartbeat),
            cx,
        );

        assert!((pose_after(Duration::ZERO).rotation_scale[0][0] - 1.).abs() < 1e-3);
        assert!((pose_after(Duration::from_millis(200)).rotation_scale[0][0] - 1.12).abs() < 1e-3);
        // Each call advances the clock further, to 400ms and then 999ms.
        let second_beat = pose_after(Duration::from_millis(200)).rotation_scale[0][0];
        assert!(second_beat > 1.05 && second_beat < 1.09);
        assert!((pose_after(Duration::from_millis(599)).rotation_scale[0][0] - 1.).abs() < 1e-3);
    }

    #[crate::test]
    fn test_scene_presents_as_often_as_its_least_capped_animation(cx: &mut TestAppContext) {
        let spin = TimedTransformation::rotation(Duration::from_secs(1), Transformation::default());
        let (_, cx) = cx.add_window_view(move |_, _| CappedView { spin });
        cx.run_until_parked();
        cx.update(|window, _| {
            assert_eq!(
                window.rendered_frame.scene.animation_frame_interval(),
                Some(Duration::from_millis(50))
            );
        });
    }

    struct CappedView {
        spin: TimedTransformation,
    }

    impl Render for CappedView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let icon = || svg().data(SQUARE_SVG).size(px(16.)).text_color(red());
            div()
                .child(icon().with_timed_transformation(self.spin.with_max_fps(10.)))
                .child(icon().with_timed_transformation(self.spin.with_max_fps(20.)))
                .child(icon().with_transformation(Transformation::alpha(0.5)))
        }
    }
}
