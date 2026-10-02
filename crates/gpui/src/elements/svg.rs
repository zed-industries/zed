use std::{
    fs,
    hash::{Hash, Hasher},
    path::Path,
    sync::Arc,
};

use crate::{
    App, Asset, Bounds, Element, GlobalElementId, Hitbox, InspectorElementId, InteractiveElement,
    Interactivity, IntoElement, LayoutId, Pixels, Point, Radians, SharedString, Size,
    StyleRefinement, Styled, TransformationMatrix, Window, point, px, radians, size,
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
    transformation: Option<Transformation>,
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
                let transformation = self
                    .transformation
                    .as_ref()
                    .map(|transformation| {
                        transformation.into_matrix(bounds.center(), window.scale_factor())
                    })
                    .unwrap_or_default();

                if let Some(data) = self.data.as_ref() {
                    if let Some(color) = style.text.color {
                        window
                            .paint_svg_data(bounds, data, transformation, color, cx)
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
                        .paint_svg(
                            bounds,
                            path.clone(),
                            Some(&bytes),
                            transformation,
                            color,
                            cx,
                        )
                        .log_err();
                } else if let Some((path, color)) = self.path.as_ref().zip(style.text.color) {
                    window
                        .paint_svg(bounds, path.clone(), None, transformation, color, cx)
                        .log_err();
                }
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
    use crate::{AtlasKey, DevicePixels, RenderSvgParams};
    use std::collections::HashMap;

    const BYTES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><path d="M0 0h24v24H0z"/></svg>"#;
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
}
