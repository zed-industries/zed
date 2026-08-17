#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "IDBTransactionOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbTransactionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type IdbTransactionOptions;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[doc = "Get the `durability` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionDurability`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "durability")]
    pub fn get_durability(this: &IdbTransactionOptions) -> Option<IdbTransactionDurability>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[doc = "Change the `durability` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionDurability`, `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "durability")]
    pub fn set_durability(this: &IdbTransactionOptions, val: IdbTransactionDurability);
}
#[cfg(web_sys_unstable_apis)]
impl IdbTransactionOptions {
    #[doc = "Construct a new `IdbTransactionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbTransactionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "IdbTransactionDurability")]
    #[deprecated = "Use `set_durability()` instead."]
    pub fn durability(&mut self, val: IdbTransactionDurability) -> &mut Self {
        self.set_durability(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for IdbTransactionOptions {
    fn default() -> Self {
        Self::new()
    }
}
