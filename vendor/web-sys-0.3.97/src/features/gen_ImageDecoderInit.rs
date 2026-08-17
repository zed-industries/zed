#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "ImageDecoderInit")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDecoderInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ImageDecoderInit;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "colorSpaceConversion")]
    pub fn get_color_space_conversion(this: &ImageDecoderInit) -> Option<ColorSpaceConversion>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "colorSpaceConversion")]
    pub fn set_color_space_conversion(this: &ImageDecoderInit, val: ColorSpaceConversion);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "data")]
    pub fn get_data(this: &ImageDecoderInit) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data(this: &ImageDecoderInit, val: &::js_sys::Object);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[doc = "# Safety"]
    #[doc = ""]
    #[doc = "The `val` slice must outlive any use of the dictionary on the JavaScript side. wasm-bindgen passes a raw view into wasm linear memory — if the underlying data is freed or moved before JS is done reading the field, the behaviour is undefined. Prefer the `_array` variant (takes `&Uint8Array`) for a safe alternative that copies the data."]
    #[wasm_bindgen(method, setter = "data")]
    pub unsafe fn set_data_u8_slice(this: &ImageDecoderInit, val: &mut [u8]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_u8_array(this: &ImageDecoderInit, val: &::js_sys::Uint8Array);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`, `ReadableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "data")]
    pub fn set_data_readable_stream(this: &ImageDecoderInit, val: &ReadableStream);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `desiredHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "desiredHeight")]
    pub fn get_desired_height(this: &ImageDecoderInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `desiredHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "desiredHeight")]
    pub fn set_desired_height(this: &ImageDecoderInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `desiredWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "desiredWidth")]
    pub fn get_desired_width(this: &ImageDecoderInit) -> Option<u32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `desiredWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "desiredWidth")]
    pub fn set_desired_width(this: &ImageDecoderInit, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `preferAnimation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "preferAnimation")]
    pub fn get_prefer_animation(this: &ImageDecoderInit) -> Option<bool>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `preferAnimation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "preferAnimation")]
    pub fn set_prefer_animation(this: &ImageDecoderInit, val: bool);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "transfer")]
    pub fn get_transfer(this: &ImageDecoderInit) -> Option<::js_sys::Array<::js_sys::ArrayBuffer>>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `transfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "transfer")]
    pub fn set_transfer(this: &ImageDecoderInit, val: &[::js_sys::ArrayBuffer]);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "type")]
    pub fn get_type(this: &ImageDecoderInit) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "type")]
    pub fn set_type(this: &ImageDecoderInit, val: &str);
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecoderInit {
    #[doc = "Construct a new `ImageDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(data: &::js_sys::Object, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_data(data);
        ret.set_type(type_);
        ret
    }
    #[doc = "Construct a new `ImageDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub unsafe fn new_with_u8_slice(data: &mut [u8], type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_data_u8_slice(data);
        ret.set_type(type_);
        ret
    }
    #[doc = "Construct a new `ImageDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_u8_array(data: &::js_sys::Uint8Array, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_data_u8_array(data);
        ret.set_type(type_);
        ret
    }
    #[cfg(feature = "ReadableStream")]
    #[doc = "Construct a new `ImageDecoderInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecoderInit`, `ReadableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_readable_stream(data: &ReadableStream, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_data_readable_stream(data);
        ret.set_type(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ColorSpaceConversion")]
    #[deprecated = "Use `set_color_space_conversion()` instead."]
    pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self {
        self.set_color_space_conversion(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_data()` instead."]
    pub fn data(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_data(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_desired_height()` instead."]
    pub fn desired_height(&mut self, val: u32) -> &mut Self {
        self.set_desired_height(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_desired_width()` instead."]
    pub fn desired_width(&mut self, val: u32) -> &mut Self {
        self.set_desired_width(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_prefer_animation()` instead."]
    pub fn prefer_animation(&mut self, val: bool) -> &mut Self {
        self.set_prefer_animation(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_transfer()` instead."]
    pub fn transfer(&mut self, val: &[::js_sys::ArrayBuffer]) -> &mut Self {
        self.set_transfer(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_type()` instead."]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type(val);
        self
    }
}
