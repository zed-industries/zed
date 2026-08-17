pub mod console {
    #![allow(unused_imports)]
    #![allow(clippy::all)]
    use super::super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = "console")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert();
        #[wasm_bindgen(variadic, js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data(condition: bool, data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_0(condition: bool);
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_1(condition: bool, data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_2(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_3(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_4(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_5(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_6(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "assert")]
        #[doc = "The `console.assert()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/assert)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn assert_with_condition_and_data_7(
            condition: bool,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console")]
        #[doc = "The `console.clear()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/clear)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn clear();
        #[wasm_bindgen(js_namespace = "console")]
        #[doc = "The `console.count()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn count();
        #[wasm_bindgen(js_namespace = "console", js_name = "count")]
        #[doc = "The `console.count()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/count)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn count_with_label(label: &str);
        #[wasm_bindgen(js_namespace = "console", js_name = "countReset")]
        #[doc = "The `console.countReset()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn count_reset();
        #[wasm_bindgen(js_namespace = "console", js_name = "countReset")]
        #[doc = "The `console.countReset()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/countReset)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn count_reset_with_label(label: &str);
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "debug")]
        #[doc = "The `console.debug()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/debug)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn debug_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dir")]
        #[doc = "The `console.dir()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dir)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dir_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "dirxml")]
        #[doc = "The `console.dirxml()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/dirxml)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn dirxml_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "error")]
        #[doc = "The `console.error()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/error)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn error_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "exception")]
        #[doc = "The `console.exception()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/exception)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn exception_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "group")]
        #[doc = "The `console.group()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/group)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_2(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupCollapsed")]
        #[doc = "The `console.groupCollapsed()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupCollapsed)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_collapsed_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "groupEnd")]
        #[doc = "The `console.groupEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/groupEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn group_end();
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "info")]
        #[doc = "The `console.info()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/info)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn info_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "log")]
        #[doc = "The `console.log()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/log)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn log_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profile")]
        #[doc = "The `console.profile()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profile)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "profileEnd")]
        #[doc = "The `console.profileEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/profileEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn profile_end_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "table")]
        #[doc = "The `console.table()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/table)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn table_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console")]
        #[doc = "The `console.time()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time();
        #[wasm_bindgen(js_namespace = "console", js_name = "time")]
        #[doc = "The `console.time()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/time)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_with_label(label: &str);
        #[wasm_bindgen(js_namespace = "console", js_name = "timeEnd")]
        #[doc = "The `console.timeEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_end();
        #[wasm_bindgen(js_namespace = "console", js_name = "timeEnd")]
        #[doc = "The `console.timeEnd()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeEnd)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_end_with_label(label: &str);
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log();
        #[wasm_bindgen(variadic, js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data(label: &str, data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_0(label: &str);
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_1(label: &str, data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_2(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_3(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_4(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_5(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_6(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeLog")]
        #[doc = "The `console.timeLog()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeLog)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_log_with_label_and_data_7(
            label: &str,
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "timeStamp")]
        #[doc = "The `console.timeStamp()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_stamp();
        #[wasm_bindgen(js_namespace = "console", js_name = "timeStamp")]
        #[doc = "The `console.timeStamp()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/timeStamp)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn time_stamp_with_data(data: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "trace")]
        #[doc = "The `console.trace()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/trace)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn trace_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(variadic, js_namespace = "console")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn(data: &::js_sys::Array);
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_0();
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_1(data_1: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_2(data_1: &::wasm_bindgen::JsValue, data_2: &::wasm_bindgen::JsValue);
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_3(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_4(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_5(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_6(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
        );
        #[wasm_bindgen(js_namespace = "console", js_name = "warn")]
        #[doc = "The `console.warn()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/console/warn)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `console`*"]
        pub fn warn_7(
            data_1: &::wasm_bindgen::JsValue,
            data_2: &::wasm_bindgen::JsValue,
            data_3: &::wasm_bindgen::JsValue,
            data_4: &::wasm_bindgen::JsValue,
            data_5: &::wasm_bindgen::JsValue,
            data_6: &::wasm_bindgen::JsValue,
            data_7: &::wasm_bindgen::JsValue,
        );
    }
}
