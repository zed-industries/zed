#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "Notification",
        typescript_type = "Notification"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Notification` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub type Notification;
    #[cfg(feature = "NotificationPermission")]
    #[wasm_bindgen(
        static_method_of = "Notification",
        getter,
        js_class = "Notification",
        js_name = "permission"
    )]
    #[doc = "Getter for the `permission` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/permission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationPermission`*"]
    pub fn permission() -> NotificationPermission;
    #[wasm_bindgen(
        static_method_of = "Notification",
        getter,
        js_class = "Notification",
        js_name = "maxActions"
    )]
    #[doc = "Getter for the `maxActions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/maxActions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn max_actions() -> u32;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "onclick")]
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn onclick(this: &Notification) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Notification", js_name = "onclick")]
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn set_onclick(this: &Notification, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "onshow")]
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn onshow(this: &Notification) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Notification", js_name = "onshow")]
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn set_onshow(this: &Notification, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "onerror")]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn onerror(this: &Notification) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Notification", js_name = "onerror")]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn set_onerror(this: &Notification, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "onclose")]
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn onclose(this: &Notification) -> Option<::js_sys::Function>;
    #[wasm_bindgen(method, setter, js_class = "Notification", js_name = "onclose")]
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn set_onclose(this: &Notification, value: Option<&::js_sys::Function>);
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "title")]
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn title(this: &Notification) -> ::alloc::string::String;
    #[cfg(feature = "NotificationDirection")]
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "dir")]
    #[doc = "Getter for the `dir` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/dir)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationDirection`*"]
    pub fn dir(this: &Notification) -> NotificationDirection;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "lang")]
    #[doc = "Getter for the `lang` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/lang)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn lang(this: &Notification) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "body")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn body(this: &Notification) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "tag")]
    #[doc = "Getter for the `tag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/tag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn tag(this: &Notification) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "image")]
    #[doc = "Getter for the `image` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/image)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn image(this: &Notification) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "icon")]
    #[doc = "Getter for the `icon` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/icon)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn icon(this: &Notification) -> Option<::alloc::string::String>;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "badge")]
    #[doc = "Getter for the `badge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/badge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn badge(this: &Notification) -> ::alloc::string::String;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "vibrate")]
    #[doc = "Getter for the `vibrate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/vibrate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn vibrate(this: &Notification) -> ::js_sys::Array;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "timestamp")]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn timestamp(this: &Notification) -> f64;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "renotify")]
    #[doc = "Getter for the `renotify` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/renotify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn renotify(this: &Notification) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "silent")]
    #[doc = "Getter for the `silent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/silent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn silent(this: &Notification) -> Option<bool>;
    #[wasm_bindgen(
        method,
        getter,
        js_class = "Notification",
        js_name = "requireInteraction"
    )]
    #[doc = "Getter for the `requireInteraction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requireInteraction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn require_interaction(this: &Notification) -> bool;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "data")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn data(this: &Notification) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, getter, js_class = "Notification", js_name = "actions")]
    #[doc = "Getter for the `actions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/actions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn actions(this: &Notification) -> ::js_sys::Array;
    #[wasm_bindgen(catch, constructor, js_class = "Notification")]
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn new(title: &str) -> Result<Notification, JsValue>;
    #[cfg(feature = "NotificationOptions")]
    #[wasm_bindgen(catch, constructor, js_class = "Notification")]
    #[doc = "The `new Notification(..)` constructor, creating a new instance of `Notification`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/Notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationOptions`*"]
    pub fn new_with_options(
        title: &str,
        options: &NotificationOptions,
    ) -> Result<Notification, JsValue>;
    #[wasm_bindgen(method, js_class = "Notification")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn close(this: &Notification);
    #[wasm_bindgen(
        catch,
        static_method_of = "Notification",
        js_class = "Notification",
        js_name = "requestPermission"
    )]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn request_permission() -> Result<::js_sys::Promise, JsValue>;
    #[wasm_bindgen(
        catch,
        static_method_of = "Notification",
        js_class = "Notification",
        js_name = "requestPermission"
    )]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Notification/requestPermission_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`*"]
    pub fn request_permission_with_permission_callback(
        permission_callback: &::js_sys::Function,
    ) -> Result<::js_sys::Promise, JsValue>;
}
