#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        extends = "Document",
        extends = "Node",
        extends = "EventTarget",
        extends = "::js_sys::Object",
        js_name = "HTMLDocument",
        typescript_type = "HTMLDocument"
    )]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDocument` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub type HtmlDocument;
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "domain")]
    #[doc = "Getter for the `domain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn domain(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "domain")]
    #[doc = "Setter for the `domain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_domain(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(catch, method, getter, js_class = "HTMLDocument", js_name = "cookie")]
    #[doc = "Getter for the `cookie` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn cookie(this: &HtmlDocument) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(catch, method, setter, js_class = "HTMLDocument", js_name = "cookie")]
    #[doc = "Setter for the `cookie` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_cookie(this: &HtmlDocument, value: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "designMode")]
    #[doc = "Getter for the `designMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn design_mode(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "designMode")]
    #[doc = "Setter for the `designMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_design_mode(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "fgColor")]
    #[doc = "Getter for the `fgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn fg_color(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "fgColor")]
    #[doc = "Setter for the `fgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_fg_color(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "linkColor")]
    #[doc = "Getter for the `linkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn link_color(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "linkColor")]
    #[doc = "Setter for the `linkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_link_color(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "vlinkColor")]
    #[doc = "Getter for the `vlinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn vlink_color(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "vlinkColor")]
    #[doc = "Setter for the `vlinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_vlink_color(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "alinkColor")]
    #[doc = "Getter for the `alinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn alink_color(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "alinkColor")]
    #[doc = "Setter for the `alinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_alink_color(this: &HtmlDocument, value: &str);
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "bgColor")]
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn bg_color(this: &HtmlDocument) -> ::alloc::string::String;
    #[wasm_bindgen(method, setter, js_class = "HTMLDocument", js_name = "bgColor")]
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn set_bg_color(this: &HtmlDocument, value: &str);
    #[cfg(feature = "HtmlAllCollection")]
    #[wasm_bindgen(method, getter, js_class = "HTMLDocument", js_name = "all")]
    #[doc = "Getter for the `all` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/all)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `HtmlDocument`*"]
    pub fn all(this: &HtmlDocument) -> HtmlAllCollection;
    #[wasm_bindgen(method, js_class = "HTMLDocument", js_name = "captureEvents")]
    #[doc = "The `captureEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/captureEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn capture_events(this: &HtmlDocument);
    #[wasm_bindgen(method, js_class = "HTMLDocument")]
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn clear(this: &HtmlDocument);
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument")]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn close(this: &HtmlDocument) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "execCommand")]
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "execCommand")]
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command_with_show_ui(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "execCommand")]
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn exec_command_with_show_ui_and_value(
        this: &HtmlDocument,
        command_id: &str,
        show_ui: bool,
        value: &str,
    ) -> Result<bool, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn open(this: &HtmlDocument) -> Result<Document, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn open_with_type(this: &HtmlDocument, type_: &str) -> Result<Document, JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn open_with_type_and_replace(
        this: &HtmlDocument,
        type_: &str,
        replace: &str,
    ) -> Result<Document, JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    pub fn open_with_url_and_name_and_features(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
    ) -> Result<Option<Window>, JsValue>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "open")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
    pub fn open_with_url_and_name_and_features_and_replace(
        this: &HtmlDocument,
        url: &str,
        name: &str,
        features: &str,
        replace: bool,
    ) -> Result<Option<Window>, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLDocument",
        js_name = "queryCommandEnabled"
    )]
    #[doc = "The `queryCommandEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_enabled(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLDocument",
        js_name = "queryCommandIndeterm"
    )]
    #[doc = "The `queryCommandIndeterm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandIndeterm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_indeterm(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLDocument",
        js_name = "queryCommandState"
    )]
    #[doc = "The `queryCommandState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_state(this: &HtmlDocument, command_id: &str) -> Result<bool, JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLDocument", js_name = "queryCommandSupported")]
    #[doc = "The `queryCommandSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_supported(this: &HtmlDocument, command_id: &str) -> bool;
    #[wasm_bindgen(
        catch,
        method,
        js_class = "HTMLDocument",
        js_name = "queryCommandValue"
    )]
    #[doc = "The `queryCommandValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn query_command_value(
        this: &HtmlDocument,
        command_id: &str,
    ) -> Result<::alloc::string::String, JsValue>;
    #[wasm_bindgen(method, js_class = "HTMLDocument", js_name = "releaseEvents")]
    #[doc = "The `releaseEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/releaseEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn release_events(this: &HtmlDocument);
    #[wasm_bindgen(catch, method, variadic, js_class = "HTMLDocument")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write(this: &HtmlDocument, text: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_0(this: &HtmlDocument) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "write")]
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn write_7(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, variadic, js_class = "HTMLDocument")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln(this: &HtmlDocument, text: &::js_sys::Array) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_0(this: &HtmlDocument) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_1(this: &HtmlDocument, text_1: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_2(this: &HtmlDocument, text_1: &str, text_2: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_3(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_4(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_5(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_6(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", js_name = "writeln")]
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn writeln_7(
        this: &HtmlDocument,
        text_1: &str,
        text_2: &str,
        text_3: &str,
        text_4: &str,
        text_5: &str,
        text_6: &str,
        text_7: &str,
    ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch, method, js_class = "HTMLDocument", indexing_getter)]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
    pub fn get(this: &HtmlDocument, name: &str) -> Result<::js_sys::Object, JsValue>;
}
