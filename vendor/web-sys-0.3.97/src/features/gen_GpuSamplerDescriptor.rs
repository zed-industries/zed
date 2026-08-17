#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUSamplerDescriptor")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSamplerDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuSamplerDescriptor;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "label")]
    pub fn get_label(this: &GpuSamplerDescriptor) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "label")]
    pub fn set_label(this: &GpuSamplerDescriptor, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeU` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "addressModeU")]
    pub fn get_address_mode_u(this: &GpuSamplerDescriptor) -> Option<GpuAddressMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeU` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "addressModeU")]
    pub fn set_address_mode_u(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeV` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "addressModeV")]
    pub fn get_address_mode_v(this: &GpuSamplerDescriptor) -> Option<GpuAddressMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeV` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "addressModeV")]
    pub fn set_address_mode_v(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Get the `addressModeW` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "addressModeW")]
    pub fn get_address_mode_w(this: &GpuSamplerDescriptor) -> Option<GpuAddressMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[doc = "Change the `addressModeW` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAddressMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "addressModeW")]
    pub fn set_address_mode_w(this: &GpuSamplerDescriptor, val: GpuAddressMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Get the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "compare")]
    pub fn get_compare(this: &GpuSamplerDescriptor) -> Option<GpuCompareFunction>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[doc = "Change the `compare` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCompareFunction`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "compare")]
    pub fn set_compare(this: &GpuSamplerDescriptor, val: GpuCompareFunction);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lodMaxClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "lodMaxClamp")]
    pub fn get_lod_max_clamp(this: &GpuSamplerDescriptor) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lodMaxClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "lodMaxClamp")]
    pub fn set_lod_max_clamp(this: &GpuSamplerDescriptor, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `lodMinClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "lodMinClamp")]
    pub fn get_lod_min_clamp(this: &GpuSamplerDescriptor) -> Option<f32>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `lodMinClamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "lodMinClamp")]
    pub fn set_lod_min_clamp(this: &GpuSamplerDescriptor, val: f32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Get the `magFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "magFilter")]
    pub fn get_mag_filter(this: &GpuSamplerDescriptor) -> Option<GpuFilterMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Change the `magFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "magFilter")]
    pub fn set_mag_filter(this: &GpuSamplerDescriptor, val: GpuFilterMode);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `maxAnisotropy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "maxAnisotropy")]
    pub fn get_max_anisotropy(this: &GpuSamplerDescriptor) -> Option<u16>;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `maxAnisotropy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "maxAnisotropy")]
    pub fn set_max_anisotropy(this: &GpuSamplerDescriptor, val: u16);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Get the `minFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "minFilter")]
    pub fn get_min_filter(this: &GpuSamplerDescriptor) -> Option<GpuFilterMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[doc = "Change the `minFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "minFilter")]
    pub fn set_min_filter(this: &GpuSamplerDescriptor, val: GpuFilterMode);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[doc = "Get the `mipmapFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMipmapFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mipmapFilter")]
    pub fn get_mipmap_filter(this: &GpuSamplerDescriptor) -> Option<GpuMipmapFilterMode>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[doc = "Change the `mipmapFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuMipmapFilterMode`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mipmapFilter")]
    pub fn set_mipmap_filter(this: &GpuSamplerDescriptor, val: GpuMipmapFilterMode);
}
#[cfg(web_sys_unstable_apis)]
impl GpuSamplerDescriptor {
    #[doc = "Construct a new `GpuSamplerDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_label()` instead."]
    pub fn label(&mut self, val: &str) -> &mut Self {
        self.set_label(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[deprecated = "Use `set_address_mode_u()` instead."]
    pub fn address_mode_u(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_u(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[deprecated = "Use `set_address_mode_v()` instead."]
    pub fn address_mode_v(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_v(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAddressMode")]
    #[deprecated = "Use `set_address_mode_w()` instead."]
    pub fn address_mode_w(&mut self, val: GpuAddressMode) -> &mut Self {
        self.set_address_mode_w(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCompareFunction")]
    #[deprecated = "Use `set_compare()` instead."]
    pub fn compare(&mut self, val: GpuCompareFunction) -> &mut Self {
        self.set_compare(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_lod_max_clamp()` instead."]
    pub fn lod_max_clamp(&mut self, val: f32) -> &mut Self {
        self.set_lod_max_clamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_lod_min_clamp()` instead."]
    pub fn lod_min_clamp(&mut self, val: f32) -> &mut Self {
        self.set_lod_min_clamp(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[deprecated = "Use `set_mag_filter()` instead."]
    pub fn mag_filter(&mut self, val: GpuFilterMode) -> &mut Self {
        self.set_mag_filter(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_max_anisotropy()` instead."]
    pub fn max_anisotropy(&mut self, val: u16) -> &mut Self {
        self.set_max_anisotropy(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuFilterMode")]
    #[deprecated = "Use `set_min_filter()` instead."]
    pub fn min_filter(&mut self, val: GpuFilterMode) -> &mut Self {
        self.set_min_filter(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuMipmapFilterMode")]
    #[deprecated = "Use `set_mipmap_filter()` instead."]
    pub fn mipmap_filter(&mut self, val: GpuMipmapFilterMode) -> &mut Self {
        self.set_mipmap_filter(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for GpuSamplerDescriptor {
    fn default() -> Self {
        Self::new()
    }
}
