#[cfg(web_sys_unstable_apis)]
#[doc = ""]
#[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
#[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
pub mod gpu_shader_stage {
    #![allow(unused_imports)]
    #![allow(clippy::all)]
    use super::super::*;
    use wasm_bindgen::prelude::*;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUShaderStage.VERTEX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `gpu_shader_stage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const VERTEX: u32 = 1u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUShaderStage.FRAGMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `gpu_shader_stage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const FRAGMENT: u32 = 2u64 as u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `GPUShaderStage.COMPUTE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `gpu_shader_stage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub const COMPUTE: u32 = 4u64 as u32;
}
