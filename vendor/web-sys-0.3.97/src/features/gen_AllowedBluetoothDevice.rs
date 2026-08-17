#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "AllowedBluetoothDevice")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AllowedBluetoothDevice` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AllowedBluetoothDevice;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowedServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "allowedServices")]
    pub fn get_allowed_services(this: &AllowedBluetoothDevice) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "allowedServices")]
    pub fn set_allowed_services(this: &AllowedBluetoothDevice, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedServices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "allowedServices")]
    pub fn set_allowed_services_str_sequence(
        this: &AllowedBluetoothDevice,
        val: &[::js_sys::JsString],
    );
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "deviceId")]
    pub fn get_device_id(this: &AllowedBluetoothDevice) -> ::alloc::string::String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "deviceId")]
    pub fn set_device_id(this: &AllowedBluetoothDevice, val: &str);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mayUseGATT` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, getter = "mayUseGATT")]
    pub fn get_may_use_gatt(this: &AllowedBluetoothDevice) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mayUseGATT` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    #[wasm_bindgen(method, setter = "mayUseGATT")]
    pub fn set_may_use_gatt(this: &AllowedBluetoothDevice, val: bool);
}
#[cfg(web_sys_unstable_apis)]
impl AllowedBluetoothDevice {
    #[doc = "Construct a new `AllowedBluetoothDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(allowed_services: &str, device_id: &str, may_use_gatt: bool) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_allowed_services(allowed_services);
        ret.set_device_id(device_id);
        ret.set_may_use_gatt(may_use_gatt);
        ret
    }
    #[doc = "Construct a new `AllowedBluetoothDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedBluetoothDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_str_sequence(
        allowed_services: &[::js_sys::JsString],
        device_id: &str,
        may_use_gatt: bool,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.set_allowed_services_str_sequence(allowed_services);
        ret.set_device_id(device_id);
        ret.set_may_use_gatt(may_use_gatt);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_allowed_services()` instead."]
    pub fn allowed_services(&mut self, val: &str) -> &mut Self {
        self.set_allowed_services(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_device_id()` instead."]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_may_use_gatt()` instead."]
    pub fn may_use_gatt(&mut self, val: bool) -> &mut Self {
        self.set_may_use_gatt(val);
        self
    }
}
