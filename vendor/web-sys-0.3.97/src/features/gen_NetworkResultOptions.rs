#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = "::js_sys::Object", js_name = "NetworkResultOptions")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkResultOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub type NetworkResultOptions;
    #[doc = "Get the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "broadcast")]
    pub fn get_broadcast(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "broadcast")]
    pub fn set_broadcast(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "curExternalIfname")]
    pub fn get_cur_external_ifname(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "curExternalIfname")]
    pub fn set_cur_external_ifname(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "curInternalIfname")]
    pub fn get_cur_internal_ifname(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "curInternalIfname")]
    pub fn set_cur_internal_ifname(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns1")]
    pub fn get_dns1(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "dns1")]
    pub fn set_dns1(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns1_str")]
    pub fn get_dns1_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "dns1_str")]
    pub fn set_dns1_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns2")]
    pub fn get_dns2(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "dns2")]
    pub fn set_dns2(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "dns2_str")]
    pub fn get_dns2_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "dns2_str")]
    pub fn set_dns2_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "enable")]
    pub fn get_enable(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "enable")]
    pub fn set_enable(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "error")]
    pub fn get_error(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "error")]
    pub fn set_error(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "flag")]
    pub fn get_flag(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "flag")]
    pub fn set_flag(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "gateway")]
    pub fn get_gateway(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "gateway")]
    pub fn set_gateway(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "gateway_str")]
    pub fn get_gateway_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "gateway_str")]
    pub fn set_gateway_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "id")]
    pub fn get_id(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "id")]
    pub fn set_id(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "interfaceList")]
    pub fn get_interface_list(this: &NetworkResultOptions) -> Option<::js_sys::Array>;
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "interfaceList")]
    pub fn set_interface_list(this: &NetworkResultOptions, val: &::wasm_bindgen::JsValue);
    #[doc = "Get the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipAddr")]
    pub fn get_ip_addr(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "ipAddr")]
    pub fn set_ip_addr(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr")]
    pub fn get_ipaddr(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "ipaddr")]
    pub fn set_ipaddr(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ipaddr_str")]
    pub fn get_ipaddr_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "ipaddr_str")]
    pub fn set_ipaddr_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "lease")]
    pub fn get_lease(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "lease")]
    pub fn set_lease(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "macAddr")]
    pub fn get_mac_addr(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "macAddr")]
    pub fn set_mac_addr(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "mask")]
    pub fn get_mask(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "mask")]
    pub fn set_mask(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "mask_str")]
    pub fn get_mask_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "mask_str")]
    pub fn set_mask_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "netId")]
    pub fn get_net_id(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "netId")]
    pub fn set_net_id(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "prefixLength")]
    pub fn get_prefix_length(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "prefixLength")]
    pub fn set_prefix_length(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "reason")]
    pub fn get_reason(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "reason")]
    pub fn set_reason(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "reply")]
    pub fn get_reply(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "reply")]
    pub fn set_reply(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "result")]
    pub fn get_result(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "result")]
    pub fn set_result(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "resultCode")]
    pub fn get_result_code(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "resultCode")]
    pub fn set_result_code(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "resultReason")]
    pub fn get_result_reason(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "resultReason")]
    pub fn set_result_reason(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "ret")]
    pub fn get_ret(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "ret")]
    pub fn set_ret(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "route")]
    pub fn get_route(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "route")]
    pub fn set_route(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "server")]
    pub fn get_server(this: &NetworkResultOptions) -> Option<i32>;
    #[doc = "Change the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "server")]
    pub fn set_server(this: &NetworkResultOptions, val: i32);
    #[doc = "Get the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "server_str")]
    pub fn get_server_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "server_str")]
    pub fn set_server_str(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "success")]
    pub fn get_success(this: &NetworkResultOptions) -> Option<bool>;
    #[doc = "Change the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "success")]
    pub fn set_success(this: &NetworkResultOptions, val: bool);
    #[doc = "Get the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "topic")]
    pub fn get_topic(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "topic")]
    pub fn set_topic(this: &NetworkResultOptions, val: &str);
    #[doc = "Get the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, getter = "vendor_str")]
    pub fn get_vendor_str(this: &NetworkResultOptions) -> Option<::alloc::string::String>;
    #[doc = "Change the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    #[wasm_bindgen(method, setter = "vendor_str")]
    pub fn set_vendor_str(this: &NetworkResultOptions, val: &str);
}
impl NetworkResultOptions {
    #[doc = "Construct a new `NetworkResultOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_broadcast()` instead."]
    pub fn broadcast(&mut self, val: bool) -> &mut Self {
        self.set_broadcast(val);
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
    pub fn dns1(&mut self, val: i32) -> &mut Self {
        self.set_dns1(val);
        self
    }
    #[deprecated = "Use `set_dns1_str()` instead."]
    pub fn dns1_str(&mut self, val: &str) -> &mut Self {
        self.set_dns1_str(val);
        self
    }
    #[deprecated = "Use `set_dns2()` instead."]
    pub fn dns2(&mut self, val: i32) -> &mut Self {
        self.set_dns2(val);
        self
    }
    #[deprecated = "Use `set_dns2_str()` instead."]
    pub fn dns2_str(&mut self, val: &str) -> &mut Self {
        self.set_dns2_str(val);
        self
    }
    #[deprecated = "Use `set_enable()` instead."]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        self.set_enable(val);
        self
    }
    #[deprecated = "Use `set_error()` instead."]
    pub fn error(&mut self, val: bool) -> &mut Self {
        self.set_error(val);
        self
    }
    #[deprecated = "Use `set_flag()` instead."]
    pub fn flag(&mut self, val: &str) -> &mut Self {
        self.set_flag(val);
        self
    }
    #[deprecated = "Use `set_gateway()` instead."]
    pub fn gateway(&mut self, val: i32) -> &mut Self {
        self.set_gateway(val);
        self
    }
    #[deprecated = "Use `set_gateway_str()` instead."]
    pub fn gateway_str(&mut self, val: &str) -> &mut Self {
        self.set_gateway_str(val);
        self
    }
    #[deprecated = "Use `set_id()` instead."]
    pub fn id(&mut self, val: i32) -> &mut Self {
        self.set_id(val);
        self
    }
    #[deprecated = "Use `set_interface_list()` instead."]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_interface_list(val);
        self
    }
    #[deprecated = "Use `set_ip_addr()` instead."]
    pub fn ip_addr(&mut self, val: &str) -> &mut Self {
        self.set_ip_addr(val);
        self
    }
    #[deprecated = "Use `set_ipaddr()` instead."]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        self.set_ipaddr(val);
        self
    }
    #[deprecated = "Use `set_ipaddr_str()` instead."]
    pub fn ipaddr_str(&mut self, val: &str) -> &mut Self {
        self.set_ipaddr_str(val);
        self
    }
    #[deprecated = "Use `set_lease()` instead."]
    pub fn lease(&mut self, val: i32) -> &mut Self {
        self.set_lease(val);
        self
    }
    #[deprecated = "Use `set_mac_addr()` instead."]
    pub fn mac_addr(&mut self, val: &str) -> &mut Self {
        self.set_mac_addr(val);
        self
    }
    #[deprecated = "Use `set_mask()` instead."]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        self.set_mask(val);
        self
    }
    #[deprecated = "Use `set_mask_str()` instead."]
    pub fn mask_str(&mut self, val: &str) -> &mut Self {
        self.set_mask_str(val);
        self
    }
    #[deprecated = "Use `set_net_id()` instead."]
    pub fn net_id(&mut self, val: &str) -> &mut Self {
        self.set_net_id(val);
        self
    }
    #[deprecated = "Use `set_prefix_length()` instead."]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
        self.set_prefix_length(val);
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
    pub fn reason(&mut self, val: &str) -> &mut Self {
        self.set_reason(val);
        self
    }
    #[deprecated = "Use `set_reply()` instead."]
    pub fn reply(&mut self, val: &str) -> &mut Self {
        self.set_reply(val);
        self
    }
    #[deprecated = "Use `set_result()` instead."]
    pub fn result(&mut self, val: bool) -> &mut Self {
        self.set_result(val);
        self
    }
    #[deprecated = "Use `set_result_code()` instead."]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        self.set_result_code(val);
        self
    }
    #[deprecated = "Use `set_result_reason()` instead."]
    pub fn result_reason(&mut self, val: &str) -> &mut Self {
        self.set_result_reason(val);
        self
    }
    #[deprecated = "Use `set_ret()` instead."]
    pub fn ret(&mut self, val: bool) -> &mut Self {
        self.set_ret(val);
        self
    }
    #[deprecated = "Use `set_route()` instead."]
    pub fn route(&mut self, val: &str) -> &mut Self {
        self.set_route(val);
        self
    }
    #[deprecated = "Use `set_server()` instead."]
    pub fn server(&mut self, val: i32) -> &mut Self {
        self.set_server(val);
        self
    }
    #[deprecated = "Use `set_server_str()` instead."]
    pub fn server_str(&mut self, val: &str) -> &mut Self {
        self.set_server_str(val);
        self
    }
    #[deprecated = "Use `set_success()` instead."]
    pub fn success(&mut self, val: bool) -> &mut Self {
        self.set_success(val);
        self
    }
    #[deprecated = "Use `set_topic()` instead."]
    pub fn topic(&mut self, val: &str) -> &mut Self {
        self.set_topic(val);
        self
    }
    #[deprecated = "Use `set_vendor_str()` instead."]
    pub fn vendor_str(&mut self, val: &str) -> &mut Self {
        self.set_vendor_str(val);
        self
    }
}
impl Default for NetworkResultOptions {
    fn default() -> Self {
        Self::new()
    }
}
