#[allow(unused_imports, reason = "conditionally used, including in docs")]
use crate::{DownlevelFlags, Origin2d};

/// View of an external texture that can be used to copy to a texture.
///
/// Corresponds to [WebGPU `GPUCopyExternalImageSourceInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopyexternalimage).
#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[derive(Clone, Debug)]
pub struct CopyExternalImageSourceInfo {
    /// The texture to be copied from. The copy source data is captured at the moment
    /// the copy is issued.
    pub source: ExternalImageSource,
    /// The base texel used for copying from the external image. Together
    /// with the `copy_size` argument to copy functions, defines the
    /// sub-region of the image to copy.
    ///
    /// Relative to the top left of the image.
    ///
    /// Must be [`Origin2d::ZERO`] if [`DownlevelFlags::UNRESTRICTED_EXTERNAL_TEXTURE_COPIES`] is not supported.
    pub origin: Origin2d,
    /// If the Y coordinate of the image should be flipped. Even if this is
    /// true, `origin` is still relative to the top left.
    pub flip_y: bool,
}

/// Source of an external texture copy.
///
/// Corresponds to the [implicit union type on WebGPU `GPUCopyExternalImageSourceInfo.source`](
/// https://gpuweb.github.io/gpuweb/#dom-gpuimagecopyexternalimage-source).
#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[derive(Clone, Debug)]
pub enum ExternalImageSource {
    /// Copy from a previously-decoded image bitmap.
    ImageBitmap(web_sys::ImageBitmap),
    /// Copy from an image element.
    HTMLImageElement(web_sys::HtmlImageElement),
    /// Copy from a current frame of a video element.
    HTMLVideoElement(web_sys::HtmlVideoElement),
    /// Copy from an image.
    ImageData(web_sys::ImageData),
    /// Copy from a on-screen canvas.
    HTMLCanvasElement(web_sys::HtmlCanvasElement),
    /// Copy from a off-screen canvas.
    ///
    /// Requires [`DownlevelFlags::UNRESTRICTED_EXTERNAL_TEXTURE_COPIES`]
    OffscreenCanvas(web_sys::OffscreenCanvas),
    /// Copy from a video frame.
    #[cfg(web_sys_unstable_apis)]
    VideoFrame(web_sys::VideoFrame),
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
impl ExternalImageSource {
    /// Gets the pixel, not css, width of the source.
    pub fn width(&self) -> u32 {
        match self {
            ExternalImageSource::ImageBitmap(b) => b.width(),
            ExternalImageSource::HTMLImageElement(i) => i.width(),
            ExternalImageSource::HTMLVideoElement(v) => v.video_width(),
            ExternalImageSource::ImageData(i) => i.width(),
            ExternalImageSource::HTMLCanvasElement(c) => c.width(),
            ExternalImageSource::OffscreenCanvas(c) => c.width(),
            #[cfg(web_sys_unstable_apis)]
            ExternalImageSource::VideoFrame(v) => v.display_width(),
        }
    }

    /// Gets the pixel, not css, height of the source.
    pub fn height(&self) -> u32 {
        match self {
            ExternalImageSource::ImageBitmap(b) => b.height(),
            ExternalImageSource::HTMLImageElement(i) => i.height(),
            ExternalImageSource::HTMLVideoElement(v) => v.video_height(),
            ExternalImageSource::ImageData(i) => i.height(),
            ExternalImageSource::HTMLCanvasElement(c) => c.height(),
            ExternalImageSource::OffscreenCanvas(c) => c.height(),
            #[cfg(web_sys_unstable_apis)]
            ExternalImageSource::VideoFrame(v) => v.display_height(),
        }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
impl core::ops::Deref for ExternalImageSource {
    type Target = js_sys::Object;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::ImageBitmap(b) => b,
            Self::HTMLImageElement(i) => i,
            Self::HTMLVideoElement(v) => v,
            Self::ImageData(i) => i,
            Self::HTMLCanvasElement(c) => c,
            Self::OffscreenCanvas(c) => c,
            #[cfg(web_sys_unstable_apis)]
            Self::VideoFrame(v) => v,
        }
    }
}

#[cfg(all(
    target_arch = "wasm32",
    feature = "web",
    feature = "fragile-send-sync-non-atomic-wasm",
    not(target_feature = "atomics")
))]
unsafe impl Send for ExternalImageSource {}
#[cfg(all(
    target_arch = "wasm32",
    feature = "web",
    feature = "fragile-send-sync-non-atomic-wasm",
    not(target_feature = "atomics")
))]
unsafe impl Sync for ExternalImageSource {}

/// Color spaces supported on the web.
///
/// Corresponds to [HTML Canvas `PredefinedColorSpace`](
/// https://html.spec.whatwg.org/multipage/canvas.html#predefinedcolorspace).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum PredefinedColorSpace {
    /// sRGB color space
    Srgb,
    /// Display-P3 color space
    DisplayP3,
}

/// View of a texture which can be used to copy to a texture, including
/// color space and alpha premultiplication information.
///
/// Corresponds to [WebGPU `GPUCopyExternalImageDestInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopytexturetagged).
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CopyExternalImageDestInfo<T> {
    /// The texture to be copied to/from.
    pub texture: T,
    /// The target mip level of the texture.
    pub mip_level: u32,
    /// The base texel of the texture in the selected `mip_level`.
    pub origin: crate::Origin3d,
    /// The copy aspect.
    pub aspect: crate::TextureAspect,
    /// The color space of this texture.
    pub color_space: PredefinedColorSpace,
    /// The premultiplication of this texture
    pub premultiplied_alpha: bool,
}

impl<T> CopyExternalImageDestInfo<T> {
    /// Removes the colorspace information from the type.
    pub fn to_untagged(self) -> crate::TexelCopyTextureInfo<T> {
        crate::TexelCopyTextureInfo {
            texture: self.texture,
            mip_level: self.mip_level,
            origin: self.origin,
            aspect: self.aspect,
        }
    }
}
