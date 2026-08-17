#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "GPUBindGroupLayoutEntry")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBindGroupLayoutEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBindGroupLayoutEntry;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `binding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "binding")]
    pub fn get_binding(this: &GpuBindGroupLayoutEntry) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `binding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "binding")]
    pub fn set_binding(this: &GpuBindGroupLayoutEntry, val: u32);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingLayout")]
    #[doc = "Get the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "buffer")]
    pub fn get_buffer(this: &GpuBindGroupLayoutEntry) -> Option<GpuBufferBindingLayout>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingLayout")]
    #[doc = "Change the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuBufferBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "buffer")]
    pub fn set_buffer(this: &GpuBindGroupLayoutEntry, val: &GpuBufferBindingLayout);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuExternalTextureBindingLayout")]
    #[doc = "Get the `externalTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuExternalTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "externalTexture")]
    pub fn get_external_texture(
        this: &GpuBindGroupLayoutEntry,
    ) -> Option<GpuExternalTextureBindingLayout>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuExternalTextureBindingLayout")]
    #[doc = "Change the `externalTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuExternalTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "externalTexture")]
    pub fn set_external_texture(
        this: &GpuBindGroupLayoutEntry,
        val: &GpuExternalTextureBindingLayout,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingLayout")]
    #[doc = "Get the `sampler` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuSamplerBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "sampler")]
    pub fn get_sampler(this: &GpuBindGroupLayoutEntry) -> Option<GpuSamplerBindingLayout>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingLayout")]
    #[doc = "Change the `sampler` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuSamplerBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "sampler")]
    pub fn set_sampler(this: &GpuBindGroupLayoutEntry, val: &GpuSamplerBindingLayout);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureBindingLayout")]
    #[doc = "Get the `storageTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "storageTexture")]
    pub fn get_storage_texture(
        this: &GpuBindGroupLayoutEntry,
    ) -> Option<GpuStorageTextureBindingLayout>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureBindingLayout")]
    #[doc = "Change the `storageTexture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuStorageTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "storageTexture")]
    pub fn set_storage_texture(
        this: &GpuBindGroupLayoutEntry,
        val: &GpuStorageTextureBindingLayout,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureBindingLayout")]
    #[doc = "Get the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "texture")]
    pub fn get_texture(this: &GpuBindGroupLayoutEntry) -> Option<GpuTextureBindingLayout>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureBindingLayout")]
    #[doc = "Change the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`, `GpuTextureBindingLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "texture")]
    pub fn set_texture(this: &GpuBindGroupLayoutEntry, val: &GpuTextureBindingLayout);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `visibility` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "visibility")]
    pub fn get_visibility(this: &GpuBindGroupLayoutEntry) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `visibility` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "visibility")]
    pub fn set_visibility(this: &GpuBindGroupLayoutEntry, val: u32);
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupLayoutEntry {
    #[doc = "Construct a new `GpuBindGroupLayoutEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayoutEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(binding: u32, visibility: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_binding(binding);
        ret.set_visibility(visibility);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_binding()` instead."]
    pub fn binding(&mut self, val: u32) -> &mut Self {
        self.set_binding(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBufferBindingLayout")]
    #[deprecated = "Use `set_buffer()` instead."]
    pub fn buffer(&mut self, val: &GpuBufferBindingLayout) -> &mut Self {
        self.set_buffer(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuExternalTextureBindingLayout")]
    #[deprecated = "Use `set_external_texture()` instead."]
    pub fn external_texture(&mut self, val: &GpuExternalTextureBindingLayout) -> &mut Self {
        self.set_external_texture(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSamplerBindingLayout")]
    #[deprecated = "Use `set_sampler()` instead."]
    pub fn sampler(&mut self, val: &GpuSamplerBindingLayout) -> &mut Self {
        self.set_sampler(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuStorageTextureBindingLayout")]
    #[deprecated = "Use `set_storage_texture()` instead."]
    pub fn storage_texture(&mut self, val: &GpuStorageTextureBindingLayout) -> &mut Self {
        self.set_storage_texture(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureBindingLayout")]
    #[deprecated = "Use `set_texture()` instead."]
    pub fn texture(&mut self, val: &GpuTextureBindingLayout) -> &mut Self {
        self.set_texture(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_visibility()` instead."]
    pub fn visibility(&mut self, val: u32) -> &mut Self {
        self.set_visibility(val);
        self
    }
}
