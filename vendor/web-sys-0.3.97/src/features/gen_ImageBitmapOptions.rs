#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ImageBitmapOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub type ImageBitmapOptions;
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, getter = "colorSpaceConversion")]
    pub fn get_color_space_conversion(this: &ImageBitmapOptions) -> Option<ColorSpaceConversion>;
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, setter = "colorSpaceConversion")]
    pub fn set_color_space_conversion(this: &ImageBitmapOptions, val: ColorSpaceConversion);
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Get the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    #[wasm_bindgen(method, getter = "imageOrientation")]
    pub fn get_image_orientation(this: &ImageBitmapOptions) -> Option<ImageOrientation>;
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Change the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    #[wasm_bindgen(method, setter = "imageOrientation")]
    pub fn set_image_orientation(this: &ImageBitmapOptions, val: ImageOrientation);
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Get the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    #[wasm_bindgen(method, getter = "premultiplyAlpha")]
    pub fn get_premultiply_alpha(this: &ImageBitmapOptions) -> Option<PremultiplyAlpha>;
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    #[wasm_bindgen(method, setter = "premultiplyAlpha")]
    pub fn set_premultiply_alpha(this: &ImageBitmapOptions, val: PremultiplyAlpha);
    #[doc = "Get the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, getter = "resizeHeight")]
    pub fn get_resize_height(this: &ImageBitmapOptions) -> Option<u32>;
    #[doc = "Change the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, setter = "resizeHeight")]
    pub fn set_resize_height(this: &ImageBitmapOptions, val: u32);
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Get the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    #[wasm_bindgen(method, getter = "resizeQuality")]
    pub fn get_resize_quality(this: &ImageBitmapOptions) -> Option<ResizeQuality>;
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Change the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    #[wasm_bindgen(method, setter = "resizeQuality")]
    pub fn set_resize_quality(this: &ImageBitmapOptions, val: ResizeQuality);
    #[doc = "Get the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, getter = "resizeWidth")]
    pub fn get_resize_width(this: &ImageBitmapOptions) -> Option<u32>;
    #[doc = "Change the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    #[wasm_bindgen(method, setter = "resizeWidth")]
    pub fn set_resize_width(this: &ImageBitmapOptions, val: u32);
}
impl ImageBitmapOptions {
    #[doc = "Construct a new `ImageBitmapOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ColorSpaceConversion")]
    #[deprecated = "Use `set_color_space_conversion()` instead."]
    pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self {
        self.set_color_space_conversion(val);
        self
    }
    #[cfg(feature = "ImageOrientation")]
    #[deprecated = "Use `set_image_orientation()` instead."]
    pub fn image_orientation(&mut self, val: ImageOrientation) -> &mut Self {
        self.set_image_orientation(val);
        self
    }
    #[cfg(feature = "PremultiplyAlpha")]
    #[deprecated = "Use `set_premultiply_alpha()` instead."]
    pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self {
        self.set_premultiply_alpha(val);
        self
    }
    #[deprecated = "Use `set_resize_height()` instead."]
    pub fn resize_height(&mut self, val: u32) -> &mut Self {
        self.set_resize_height(val);
        self
    }
    #[cfg(feature = "ResizeQuality")]
    #[deprecated = "Use `set_resize_quality()` instead."]
    pub fn resize_quality(&mut self, val: ResizeQuality) -> &mut Self {
        self.set_resize_quality(val);
        self
    }
    #[deprecated = "Use `set_resize_width()` instead."]
    pub fn resize_width(&mut self, val: u32) -> &mut Self {
        self.set_resize_width(val);
        self
    }
}
impl Default for ImageBitmapOptions {
    fn default() -> Self {
        Self::new()
    }
}
