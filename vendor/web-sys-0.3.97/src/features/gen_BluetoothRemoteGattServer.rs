#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "BluetoothRemoteGATTServer",
        typescript_type = "BluetoothRemoteGATTServer"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothRemoteGattServer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothRemoteGattServer;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothDevice")]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "device"
    )]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothDevice`, `BluetoothRemoteGattServer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device(this: &BluetoothRemoteGattServer) -> BluetoothDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "connected"
    )]
    #[doc = "Getter for the `connected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/connected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn connected(this: &BluetoothRemoteGattServer) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothRemoteGATTServer")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn connect(
        this: &BluetoothRemoteGattServer,
    ) -> ::js_sys::Promise<BluetoothRemoteGattServer>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "BluetoothRemoteGATTServer")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn disconnect(this: &BluetoothRemoteGattServer);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "getPrimaryService"
    )]
    #[doc = "The `getPrimaryService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_primary_service_with_str(
        this: &BluetoothRemoteGattServer,
        service: &str,
    ) -> ::js_sys::Promise<BluetoothRemoteGattService>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "getPrimaryService"
    )]
    #[doc = "The `getPrimaryService()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_primary_service_with_u32(
        this: &BluetoothRemoteGattServer,
        service: u32,
    ) -> ::js_sys::Promise<BluetoothRemoteGattService>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "getPrimaryServices"
    )]
    #[doc = "The `getPrimaryServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_primary_services(
        this: &BluetoothRemoteGattServer,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "getPrimaryServices"
    )]
    #[doc = "The `getPrimaryServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_primary_services_with_str(
        this: &BluetoothRemoteGattServer,
        service: &str,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "BluetoothRemoteGattService")]
    #[wasm_bindgen(
        method,
        js_class = "BluetoothRemoteGATTServer",
        js_name = "getPrimaryServices"
    )]
    #[doc = "The `getPrimaryServices()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryServices)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothRemoteGattServer`, `BluetoothRemoteGattService`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_primary_services_with_u32(
        this: &BluetoothRemoteGattServer,
        service: u32,
    ) -> ::js_sys::Promise<::js_sys::Array<BluetoothRemoteGattService>>;
}
