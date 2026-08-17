#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "BatteryManager",
        typescript_type = "BatteryManager"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BatteryManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub type BatteryManager;
    #[wasm_bindgen(method, getter, js_class = "BatteryManager", js_name = "charging")]
    #[doc = "Getter for the `charging` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/charging)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn charging(this: &BatteryManager) -> bool;
    #[wasm_bindgen(method, getter, js_class = "BatteryManager", js_name = "chargingTime")]
    #[doc = "Getter for the `chargingTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/chargingTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn charging_time(this: &BatteryManager) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BatteryManager",
        js_name = "dischargingTime"
    )]
    #[doc = "Getter for the `dischargingTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/dischargingTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn discharging_time(this: &BatteryManager) -> f64;
    #[wasm_bindgen(method, getter, js_class = "BatteryManager", js_name = "level")]
    #[doc = "Getter for the `level` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/level)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn level(this: &BatteryManager) -> f64;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BatteryManager",
        js_name = "onchargingchange"
    )]
    #[doc = "Getter for the `onchargingchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onchargingchange(this: &BatteryManager) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BatteryManager",
        js_name = "onchargingchange"
    )]
    #[doc = "Setter for the `onchargingchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onchargingchange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BatteryManager",
        js_name = "onchargingtimechange"
    )]
    #[doc = "Getter for the `onchargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onchargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BatteryManager",
        js_name = "onchargingtimechange"
    )]
    #[doc = "Setter for the `onchargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onchargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onchargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "BatteryManager",
        js_name = "ondischargingtimechange"
    )]
    #[doc = "Getter for the `ondischargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn ondischargingtimechange(this: &BatteryManager) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "BatteryManager",
        js_name = "ondischargingtimechange"
    )]
    #[doc = "Setter for the `ondischargingtimechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/ondischargingtimechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_ondischargingtimechange(this: &BatteryManager, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "BatteryManager", js_name = "onlevelchange")]
    #[doc = "Getter for the `onlevelchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn onlevelchange(this: &BatteryManager) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "BatteryManager", js_name = "onlevelchange")]
    #[doc = "Setter for the `onlevelchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BatteryManager/onlevelchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BatteryManager`*"]
    pub fn set_onlevelchange(this: &BatteryManager, value: Option<&::js_sys::Function>);
}
