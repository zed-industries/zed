#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NetworkCommandOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkCommandOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub type NetworkCommandOptions;
    #[doc = "Get the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "cmd")]
    pub fn get_cmd(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "cmd")]
    pub fn set_cmd(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    pub fn get_cur_external_ifname(this: &NetworkCommandOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    pub fn set_cur_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    pub fn get_cur_internal_ifname(this: &NetworkCommandOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    pub fn set_cur_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns1")]
    pub fn get_dns1(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "dns1")]
    pub fn set_dns1(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns1_long")]
    pub fn get_dns1_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "dns1_long")]
    pub fn set_dns1_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns2")]
    pub fn get_dns2(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "dns2")]
    pub fn set_dns2(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dns2_long")]
    pub fn get_dns2_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "dns2_long")]
    pub fn set_dns2_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "dnses")]
    pub fn get_dnses(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "dnses")]
    pub fn set_dnses(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "domain")]
    pub fn get_domain(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "domain")]
    pub fn set_domain(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "enable")]
    pub fn get_enable(this: &NetworkCommandOptions) -> Option<bool>;
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "enable")]
    pub fn set_enable(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "enabled")]
    pub fn get_enabled(this: &NetworkCommandOptions) -> Option<bool>;
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "enabled")]
    pub fn set_enabled(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "endIp")]
    pub fn get_end_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "endIp")]
    pub fn set_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "externalIfname")]
    pub fn get_external_ifname(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "externalIfname")]
    pub fn set_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateway")]
    pub fn get_gateway(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "gateway")]
    pub fn set_gateway(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateway_long")]
    pub fn get_gateway_long(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "gateway_long")]
    pub fn set_gateway_long(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "gateways")]
    pub fn get_gateways(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "gateways")]
    pub fn set_gateways(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ifname")]
    pub fn get_ifname(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "ifname")]
    pub fn set_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "interfaceList")]
    pub fn get_interface_list(this: &NetworkCommandOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "interfaceList")]
    pub fn set_interface_list(this: &NetworkCommandOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "internalIfname")]
    pub fn get_internal_ifname(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "internalIfname")]
    pub fn set_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ip")]
    pub fn get_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "ip")]
    pub fn set_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr")]
    pub fn get_ipaddr(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "ipaddr")]
    pub fn set_ipaddr(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "key")]
    pub fn get_key(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "key")]
    pub fn set_key(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "link")]
    pub fn get_link(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "link")]
    pub fn set_link(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mask")]
    pub fn get_mask(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "mask")]
    pub fn set_mask(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "maskLength")]
    pub fn get_mask_length(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "maskLength")]
    pub fn set_mask_length(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mode")]
    pub fn get_mode(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "mode")]
    pub fn set_mode(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "mtu")]
    pub fn get_mtu(this: &NetworkCommandOptions) -> Option<i32>;
    #[doc = "Change the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "mtu")]
    pub fn set_mtu(this: &NetworkCommandOptions, val: i32);
    #[doc = "Get the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "preExternalIfname")]
    pub fn get_pre_external_ifname(this: &NetworkCommandOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "preExternalIfname")]
    pub fn set_pre_external_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "preInternalIfname")]
    pub fn get_pre_internal_ifname(this: &NetworkCommandOptions)
        -> Option<::alloc::string::String>;
    #[doc = "Change the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "preInternalIfname")]
    pub fn set_pre_internal_ifname(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "prefix")]
    pub fn get_prefix(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "prefix")]
    pub fn set_prefix(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkCommandOptions) -> Option<u32>;
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "prefixLength")]
    pub fn set_prefix_length(this: &NetworkCommandOptions, val: u32);
    #[doc = "Get the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "report")]
    pub fn get_report(this: &NetworkCommandOptions) -> Option<bool>;
    #[doc = "Change the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "report")]
    pub fn set_report(this: &NetworkCommandOptions, val: bool);
    #[doc = "Get the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "security")]
    pub fn get_security(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "security")]
    pub fn set_security(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "serverIp")]
    pub fn get_server_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "serverIp")]
    pub fn set_server_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "ssid")]
    pub fn get_ssid(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "ssid")]
    pub fn set_ssid(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "startIp")]
    pub fn get_start_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "startIp")]
    pub fn set_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "threshold")]
    pub fn get_threshold(this: &NetworkCommandOptions) -> Option<f64>;
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "threshold")]
    pub fn set_threshold(this: &NetworkCommandOptions, val: f64);
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "threshold")]
    pub fn set_threshold_i32(this: &NetworkCommandOptions, val: i32);
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "threshold")]
    pub fn set_threshold_f64(this: &NetworkCommandOptions, val: f64);
    #[doc = "Get the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "usbEndIp")]
    pub fn get_usb_end_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "usbEndIp")]
    pub fn set_usb_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "usbStartIp")]
    pub fn get_usb_start_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "usbStartIp")]
    pub fn set_usb_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifiEndIp")]
    pub fn get_wifi_end_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "wifiEndIp")]
    pub fn set_wifi_end_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifiStartIp")]
    pub fn get_wifi_start_ip(this: &NetworkCommandOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "wifiStartIp")]
    pub fn set_wifi_start_ip(this: &NetworkCommandOptions, val: &str);
    #[doc = "Get the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, getter = "wifictrlinterfacename")]
    pub fn get_wifictrlinterfacename(
        this: &NetworkCommandOptions,
    ) -> Option<::alloc::string::String>;
    #[doc = "Change the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    #[wasm_bindgen(method, setter = "wifictrlinterfacename")]
    pub fn set_wifictrlinterfacename(this: &NetworkCommandOptions, val: &str);
}
impl NetworkCommandOptions {
    #[doc = "Construct a new `NetworkCommandOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_cmd()` instead."]
    pub fn cmd(&mut self, val: &str) -> &mut Self {
        self.set_cmd(val);
        self
    }
    #[deprecated = "Use `set_cur_external_ifname()` instead."]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_external_ifname(val);
        self
    }
    #[deprecated = "Use `set_cur_internal_ifname()` instead."]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_cur_internal_ifname(val);
        self
    }
    #[deprecated = "Use `set_dns1()` instead."]
    pub fn dns1(&mut self, val: &str) -> &mut Self {
        self.set_dns1(val);
        self
    }
    #[deprecated = "Use `set_dns1_long()` instead."]
    pub fn dns1_long(&mut self, val: i32) -> &mut Self {
        self.set_dns1_long(val);
        self
    }
    #[deprecated = "Use `set_dns2()` instead."]
    pub fn dns2(&mut self, val: &str) -> &mut Self {
        self.set_dns2(val);
        self
    }
    #[deprecated = "Use `set_dns2_long()` instead."]
    pub fn dns2_long(&mut self, val: i32) -> &mut Self {
        self.set_dns2_long(val);
        self
    }
    #[deprecated = "Use `set_dnses()` instead."]
    pub fn dnses(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_dnses(val);
        self
    }
    #[deprecated = "Use `set_domain()` instead."]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        self.set_domain(val);
        self
    }
    #[deprecated = "Use `set_enable()` instead."]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable(val);
        self
    }
    #[deprecated = "Use `set_enabled()` instead."]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        self.set_enabled(val);
        self
    }
    #[deprecated = "Use `set_end_ip()` instead."]
    pub fn end_ip(&mut self, val: &str) -> &mut Self {
        self.set_end_ip(val);
        self
    }
    #[deprecated = "Use `set_external_ifname()` instead."]
    pub fn external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_external_ifname(val);
        self
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: &str) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_gateway_long()` instead."]
    pub fn gateway_long(&mut self, val: i32) -> &mut Self {
        self.set_gateway_long(val);
        self
    }
    #[deprecated = "Use `set_gateways()` instead."]
    pub fn gateways(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_gateways(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_ifname()` instead."]
    pub fn ifname(&mut self, val: &str) -> &mut Self {
        self.set_ifname(val);
        self
    }
    #[deprecated = "Use `set_interface_list()` instead."]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list(val);
        self
    }
    #[deprecated = "Use `set_internal_ifname()` instead."]
    pub fn internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_internal_ifname(val);
        self
    }
    #[deprecated = "Use `set_ip()` instead."]
    pub fn ip(&mut self, val: &str) -> &mut Self {
        self.set_ip(val);
        self
    }
    #[deprecated = "Use `set_ipaddr()` instead."]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr(val);
        self
    }
    #[deprecated = "Use `set_key()` instead."]
    pub fn key(&mut self, val: &str) -> &mut Self {
        self.set_key(val);
        self
    }
    #[deprecated = "Use `set_link()` instead."]
    pub fn link(&mut self, val: &str) -> &mut Self {
        self.set_link(val);
        self
    }
    #[deprecated = "Use `set_mask()` instead."]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask(val);
        self
    }
    #[deprecated = "Use `set_mask_length()` instead."]
    pub fn mask_length(&mut self, val: &str) -> &mut Self {
        self.set_mask_length(val);
        self
    }
    #[deprecated = "Use `set_mode()` instead."]
    pub fn mode(&mut self, val: &str) -> &mut Self {
        self.set_mode(val);
        self
    }
    #[deprecated = "Use `set_mtu()` instead."]
    pub fn mtu(&mut self, val: i32) -> &mut Self {
        self.set_mtu(val);
        self
    }
    #[deprecated = "Use `set_pre_external_ifname()` instead."]
    pub fn pre_external_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_external_ifname(val);
        self
    }
    #[deprecated = "Use `set_pre_internal_ifname()` instead."]
    pub fn pre_internal_ifname(&mut self, val: &str) -> &mut Self {
        self.set_pre_internal_ifname(val);
        self
    }
    #[deprecated = "Use `set_prefix()` instead."]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        self.set_prefix(val);
        self
    }
    #[deprecated = "Use `set_prefix_length()` instead."]
    pub fn prefix_length(&mut self, val: u32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
    #[deprecated = "Use `set_report()` instead."]
    pub fn report(&mut self, val: bool) -> &mut Self {
        self.set_report(val);
        self
    }
    #[deprecated = "Use `set_security()` instead."]
    pub fn security(&mut self, val: &str) -> &mut Self {
        self.set_security(val);
        self
    }
    #[deprecated = "Use `set_server_ip()` instead."]
    pub fn server_ip(&mut self, val: &str) -> &mut Self {
        self.set_server_ip(val);
        self
    }
    #[deprecated = "Use `set_ssid()` instead."]
    pub fn ssid(&mut self, val: &str) -> &mut Self {
        self.set_ssid(val);
        self
    }
    #[deprecated = "Use `set_start_ip()` instead."]
    pub fn start_ip(&mut self, val: &str) -> &mut Self {
        self.set_start_ip(val);
        self
    }
    #[deprecated = "Use `set_threshold()` instead."]
    pub fn threshold(&mut self, val: f64) -> &mut Self {
        self.set_threshold(val);
        self
    }
    #[deprecated = "Use `set_usb_end_ip()` instead."]
    pub fn usb_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_end_ip(val);
        self
    }
    #[deprecated = "Use `set_usb_start_ip()` instead."]
    pub fn usb_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_usb_start_ip(val);
        self
    }
    #[deprecated = "Use `set_wifi_end_ip()` instead."]
    pub fn wifi_end_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_end_ip(val);
        self
    }
    #[deprecated = "Use `set_wifi_start_ip()` instead."]
    pub fn wifi_start_ip(&mut self, val: &str) -> &mut Self {
        self.set_wifi_start_ip(val);
        self
    }
    #[deprecated = "Use `set_wifictrlinterfacename()` instead."]
    pub fn wifictrlinterfacename(&mut self, val: &str) -> &mut Self {
        self.set_wifictrlinterfacename(val);
        self
    }
}
impl Default for NetworkCommandOptions {
    fn default() -> Self {
        Self::new()
    }
}
