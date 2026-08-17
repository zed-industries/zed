#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (is_type_of = | _ | false , extends = "::js_sys::Object" , js_name = "Geolocation" , typescript_type = "Geolocation")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Geolocation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub type Geolocation;
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "clearWatch")]
    #[doc = "The `clearWatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/clearWatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub fn clear_watch(this: &Geolocation, watch_id: i32);
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Geolocation",
        js_name = "getCurrentPosition"
    )]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub fn get_current_position(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
    ) -> Result<(), JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Geolocation",
        js_name = "getCurrentPosition"
    )]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub fn get_current_position_with_error_callback(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
    ) -> Result<(), JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "PositionOptions")]
    #[wasm_bindgen(
        catch,
        method,
        js_class = "Geolocation",
        js_name = "getCurrentPosition"
    )]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
    pub fn get_current_position_with_error_callback_and_options(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
        options: &PositionOptions,
    ) -> Result<(), JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GeolocationPosition")]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "getCurrentPosition")]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_position(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GeolocationPosition", feature = "GeolocationPositionError",))]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "getCurrentPosition")]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`, `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_position_with_error_callback(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
        error_callback: Option<
            &::js_sys::Function<fn(GeolocationPositionError) -> ::js_sys::Undefined>,
        >,
    );
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GeolocationPosition",
        feature = "GeolocationPositionError",
        feature = "PositionOptions",
    ))]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "getCurrentPosition")]
    #[doc = "The `getCurrentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`, `GeolocationPositionError`, `PositionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn get_current_position_with_error_callback_and_options(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
        error_callback: Option<
            &::js_sys::Function<fn(GeolocationPositionError) -> ::js_sys::Undefined>,
        >,
        options: &PositionOptions,
    );
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(catch, method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub fn watch_position(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
    ) -> Result<i32, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[wasm_bindgen(catch, method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`*"]
    pub fn watch_position_with_error_callback(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
    ) -> Result<i32, JsValue>;
    #[cfg(not(web_sys_unstable_apis))]
    #[cfg(feature = "PositionOptions")]
    #[wasm_bindgen(catch, method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `PositionOptions`*"]
    pub fn watch_position_with_error_callback_and_options(
        this: &Geolocation,
        success_callback: &::js_sys::Function,
        error_callback: Option<&::js_sys::Function>,
        options: &PositionOptions,
    ) -> Result<i32, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GeolocationPosition")]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn watch_position(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
    ) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GeolocationPosition", feature = "GeolocationPositionError",))]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`, `GeolocationPositionError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn watch_position_with_error_callback(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
        error_callback: Option<
            &::js_sys::Function<fn(GeolocationPositionError) -> ::js_sys::Undefined>,
        >,
    ) -> i32;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GeolocationPosition",
        feature = "GeolocationPositionError",
        feature = "PositionOptions",
    ))]
    #[wasm_bindgen(method, js_class = "Geolocation", js_name = "watchPosition")]
    #[doc = "The `watchPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Geolocation`, `GeolocationPosition`, `GeolocationPositionError`, `PositionOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn watch_position_with_error_callback_and_options(
        this: &Geolocation,
        success_callback: &::js_sys::Function<fn(GeolocationPosition) -> ::js_sys::Undefined>,
        error_callback: Option<
            &::js_sys::Function<fn(GeolocationPositionError) -> ::js_sys::Undefined>,
        >,
        options: &PositionOptions,
    ) -> i32;
}
