#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "::js_sys::Object",
        js_name = "USBDevice",
        typescript_type = "USBDevice"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbDevice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbDevice;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "usbVersionMajor")]
    #[doc = "Getter for the `usbVersionMajor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMajor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb_version_major(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "usbVersionMinor")]
    #[doc = "Getter for the `usbVersionMinor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionMinor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb_version_minor(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "usbVersionSubminor")]
    #[doc = "Getter for the `usbVersionSubminor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/usbVersionSubminor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usb_version_subminor(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "deviceClass")]
    #[doc = "Getter for the `deviceClass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceClass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_class(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "deviceSubclass")]
    #[doc = "Getter for the `deviceSubclass` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceSubclass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_subclass(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "deviceProtocol")]
    #[doc = "Getter for the `deviceProtocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceProtocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_protocol(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "vendorId")]
    #[doc = "Getter for the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/vendorId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(this: &UsbDevice) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "productId")]
    #[doc = "Getter for the `productId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(this: &UsbDevice) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "deviceVersionMajor")]
    #[doc = "Getter for the `deviceVersionMajor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMajor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_version_major(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "deviceVersionMinor")]
    #[doc = "Getter for the `deviceVersionMinor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionMinor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_version_minor(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(
        method,
        getter,
        js_class = "USBDevice",
        js_name = "deviceVersionSubminor"
    )]
    #[doc = "Getter for the `deviceVersionSubminor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/deviceVersionSubminor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_version_subminor(this: &UsbDevice) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "manufacturerName")]
    #[doc = "Getter for the `manufacturerName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/manufacturerName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn manufacturer_name(this: &UsbDevice) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "productName")]
    #[doc = "Getter for the `productName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/productName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_name(this: &UsbDevice) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "serialNumber")]
    #[doc = "Getter for the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/serialNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial_number(this: &UsbDevice) -> Option<::alloc::string::String>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConfiguration")]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "configuration")]
    #[doc = "Getter for the `configuration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configuration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configuration(this: &UsbDevice) -> Option<UsbConfiguration>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbConfiguration")]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "configurations")]
    #[doc = "Getter for the `configurations` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/configurations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbConfiguration`, `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn configurations(this: &UsbDevice) -> ::js_sys::Array<UsbConfiguration>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, getter, js_class = "USBDevice", js_name = "opened")]
    #[doc = "Getter for the `opened` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/opened)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn opened(this: &UsbDevice) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "claimInterface")]
    #[doc = "The `claimInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/claimInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn claim_interface(
        this: &UsbDevice,
        interface_number: u8,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbDirection")]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "clearHalt")]
    #[doc = "The `clearHalt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/clearHalt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbDirection`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clear_halt(
        this: &UsbDevice,
        direction: UsbDirection,
        endpoint_number: u8,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn close(this: &UsbDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "UsbControlTransferParameters",
        feature = "UsbInTransferResult",
    ))]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "controlTransferIn")]
    #[doc = "The `controlTransferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`, `UsbInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn control_transfer_in(
        this: &UsbDevice,
        setup: &UsbControlTransferParameters,
        length: u16,
    ) -> ::js_sys::Promise<UsbInTransferResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "UsbControlTransferParameters",
        feature = "UsbOutTransferResult",
    ))]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "controlTransferOut")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn control_transfer_out(
        this: &UsbDevice,
        setup: &UsbControlTransferParameters,
    ) -> ::js_sys::Promise<UsbOutTransferResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "UsbControlTransferParameters",
        feature = "UsbOutTransferResult",
    ))]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "controlTransferOut")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn control_transfer_out_with_buffer_source(
        this: &UsbDevice,
        setup: &UsbControlTransferParameters,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "UsbControlTransferParameters",
        feature = "UsbOutTransferResult",
    ))]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "controlTransferOut")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn control_transfer_out_with_u8_slice(
        this: &UsbDevice,
        setup: &UsbControlTransferParameters,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "UsbControlTransferParameters",
        feature = "UsbOutTransferResult",
    ))]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "controlTransferOut")]
    #[doc = "The `controlTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/controlTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbControlTransferParameters`, `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn control_transfer_out_with_u8_array(
        this: &UsbDevice,
        setup: &UsbControlTransferParameters,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice")]
    #[doc = "The `forget()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/forget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn forget(this: &UsbDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousInTransferResult")]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "isochronousTransferIn")]
    #[doc = "The `isochronousTransferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbIsochronousInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn isochronous_transfer_in(
        this: &UsbDevice,
        endpoint_number: u8,
        packet_lengths: &[::js_sys::Number],
    ) -> ::js_sys::Promise<UsbIsochronousInTransferResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousOutTransferResult")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "USBDevice",
        js_name = "isochronousTransferOut"
    )]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn isochronous_transfer_out_with_buffer_source(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &::js_sys::Object,
        packet_lengths: &[::js_sys::Number],
    ) -> Result<::js_sys::Promise<UsbIsochronousOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousOutTransferResult")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "USBDevice",
        js_name = "isochronousTransferOut"
    )]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn isochronous_transfer_out_with_u8_slice(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &mut [u8],
        packet_lengths: &[::js_sys::Number],
    ) -> Result<::js_sys::Promise<UsbIsochronousOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbIsochronousOutTransferResult")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "USBDevice",
        js_name = "isochronousTransferOut"
    )]
    #[doc = "The `isochronousTransferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/isochronousTransferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbIsochronousOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn isochronous_transfer_out_with_u8_array(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &::js_sys::Uint8Array,
        packet_lengths: &[::js_sys::Number],
    ) -> Result<::js_sys::Promise<UsbIsochronousOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn open(this: &UsbDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "releaseInterface")]
    #[doc = "The `releaseInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/releaseInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn release_interface(
        this: &UsbDevice,
        interface_number: u8,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice")]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reset(this: &UsbDevice) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "selectAlternateInterface")]
    #[doc = "The `selectAlternateInterface()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectAlternateInterface)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn select_alternate_interface(
        this: &UsbDevice,
        interface_number: u8,
        alternate_setting: u8,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "selectConfiguration")]
    #[doc = "The `selectConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/selectConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn select_configuration(
        this: &UsbDevice,
        configuration_value: u8,
    ) -> ::js_sys::Promise<::js_sys::Undefined>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbInTransferResult")]
    #[wasm_bindgen(method, js_class = "USBDevice", js_name = "transferIn")]
    #[doc = "The `transferIn()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferIn)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbInTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer_in(
        this: &UsbDevice,
        endpoint_number: u8,
        length: u32,
    ) -> ::js_sys::Promise<UsbInTransferResult>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbOutTransferResult")]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "transferOut")]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer_out_with_buffer_source(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &::js_sys::Object,
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbOutTransferResult")]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "transferOut")]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer_out_with_u8_slice(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &mut [u8],
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UsbOutTransferResult")]
    #[wasm_bindgen(catch, method, js_class = "USBDevice", js_name = "transferOut")]
    #[doc = "The `transferOut()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/USBDevice/transferOut)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbDevice`, `UsbOutTransferResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn transfer_out_with_u8_array(
        this: &UsbDevice,
        endpoint_number: u8,
        data: &::js_sys::Uint8Array,
    ) -> Result<::js_sys::Promise<UsbOutTransferResult>, JsValue>;
}
