#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "BluetoothPermissionStorage")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothPermissionStorage;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedBluetoothDevice")]
    #[doc = "Get the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`, `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "allowedDevices")]
    pub fn get_allowed_devices(
        this: &BluetoothPermissionStorage,
    ) -> ::js_sys::Array<AllowedBluetoothDevice>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedBluetoothDevice")]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`, `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "allowedDevices")]
    pub fn set_allowed_devices(this: &BluetoothPermissionStorage, val: &[AllowedBluetoothDevice]);
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionStorage {
    #[cfg(feature = "AllowedBluetoothDevice")]
    #[doc = "Construct a new `BluetoothPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`, `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(allowed_devices: &[AllowedBluetoothDevice]) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_allowed_devices(allowed_devices);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AllowedBluetoothDevice")]
    #[deprecated = "Use `set_allowed_devices()` instead."]
    pub fn allowed_devices(&mut self, val: &[AllowedBluetoothDevice]) -> &mut Self {
        self.set_allowed_devices(val);
        self
    }
}
