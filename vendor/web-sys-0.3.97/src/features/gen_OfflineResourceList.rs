#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "OfflineResourceList",
        typescript_type = "OfflineResourceList"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OfflineResourceList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub type OfflineResourceList;
    #[wasm_bindgen(
        catch,
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "status"
    )]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn status(this: &OfflineResourceList) -> Result<u16, JsValue>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "onchecking"
    )]
    #[doc = "Getter for the `onchecking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onchecking(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "onchecking"
    )]
    #[doc = "Setter for the `onchecking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onchecking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onchecking(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "OfflineResourceList", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onerror(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "OfflineResourceList", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onerror(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "onnoupdate"
    )]
    #[doc = "Getter for the `onnoupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onnoupdate(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "onnoupdate"
    )]
    #[doc = "Setter for the `onnoupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onnoupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onnoupdate(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "ondownloading"
    )]
    #[doc = "Getter for the `ondownloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn ondownloading(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "ondownloading"
    )]
    #[doc = "Setter for the `ondownloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/ondownloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_ondownloading(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "onprogress"
    )]
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onprogress(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "onprogress"
    )]
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onprogress(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "onupdateready"
    )]
    #[doc = "Getter for the `onupdateready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onupdateready(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "onupdateready"
    )]
    #[doc = "Setter for the `onupdateready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onupdateready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onupdateready(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "OfflineResourceList", js_name = "oncached")]
    #[doc = "Getter for the `oncached` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn oncached(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "OfflineResourceList", js_name = "oncached")]
    #[doc = "Setter for the `oncached` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/oncached)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_oncached(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(
        method,
        getter,
        js_class = "OfflineResourceList",
        js_name = "onobsolete"
    )]
    #[doc = "Getter for the `onobsolete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn onobsolete(this: &OfflineResourceList) -> Option<::js_sys::Function>;
    #[wasm_bindgen(
        method,
        setter,
        js_class = "OfflineResourceList",
        js_name = "onobsolete"
    )]
    #[doc = "Setter for the `onobsolete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/onobsolete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn set_onobsolete(this: &OfflineResourceList, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(catch, method, js_class = "OfflineResourceList", js_name = "swapCache")]
    #[doc = "The `swapCache()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/swapCache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn swap_cache(this: &OfflineResourceList) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "OfflineResourceList")]
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OfflineResourceList/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub fn update(this: &OfflineResourceList) -> Result<(), JsValue>;
}
impl OfflineResourceList {
    #[doc = "The `OfflineResourceList.UNCACHED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const UNCACHED: u16 = 0i64 as u16;
    #[doc = "The `OfflineResourceList.IDLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const IDLE: u16 = 1u64 as u16;
    #[doc = "The `OfflineResourceList.CHECKING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const CHECKING: u16 = 2u64 as u16;
    #[doc = "The `OfflineResourceList.DOWNLOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const DOWNLOADING: u16 = 3u64 as u16;
    #[doc = "The `OfflineResourceList.UPDATEREADY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const UPDATEREADY: u16 = 4u64 as u16;
    #[doc = "The `OfflineResourceList.OBSOLETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OfflineResourceList`*"]
    pub const OBSOLETE: u16 = 5u64 as u16;
}
