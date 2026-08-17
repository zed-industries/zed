// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_ITEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_ITEM_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_blob_string.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class Blob;
class ScriptState;

// A `ClipboardItem` holds data that was read from or will be written to the
// system clipboard. Spec:
// https://w3c.github.io/clipboard-apis/#clipboard-item-interface
class ClipboardItem final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Creates a `ClipboardItem` containing `representations`.
  // If `representations` is empty, writes error info to `exception_state` and
  // returns nullptr.
  static ClipboardItem* Create(
      const HeapVector<
          std::pair<String, MemberScriptPromise<V8UnionBlobOrString>>>&
          representations,
      ExceptionState& exception_state);

  // Constructs a `ClipboardItem` instance from `representations`.
  // Parses the web custom MIME types and stores them in `custom_format_types_`
  // and `representations_`.
  // If an empty `ClipboardItem` is a valid use-case, use the constructor
  // directly, else use `Create` method.
  explicit ClipboardItem(
      const HeapVector<
          std::pair<String, MemberScriptPromise<V8UnionBlobOrString>>>&
          representations);

  // Returns the MIME types contained in the `ClipboardItem`.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboarditem-types
  Vector<String> types() const;

  // Retrieves the data of a specific `type` from the `ClipboardItem` and
  // returns a promise resolved to that data.
  // `script_state`: The script state in which the promise will be resolved.
  // `type`: The MIME type or a custom MIME type with a "web " prefix of data to
  // retrieve. `exception_state`: The exception state to be updated if an error
  // occurs. Spec:
  // https://w3c.github.io/clipboard-apis/#dom-clipboarditem-gettype
  ScriptPromise<Blob> getType(ScriptState* script_state,
                              const String& type,
                              ExceptionState& exception_state) const;

  // Checks if a particular MIME type is supported by the Async Clipboard API.
  // `type` refers to a MIME type or a custom MIME type with a "web " prefix.
  // Spec: https://w3c.github.io/clipboard-apis/#dom-clipboarditem-supports
  static bool supports(const String& type);

  const HeapVector<std::pair<String, MemberScriptPromise<V8UnionBlobOrString>>>&
  GetRepresentations() const {
    return representations_;
  }

  // Returns the custom formats that have a "web " prefix.
  const Vector<String>& CustomFormats() const { return custom_format_types_; }

  // ScriptWrappable
  void Trace(Visitor*) const override;

 private:
  // Stores built-in and web custom MIME types.
  HeapVector<std::pair<String, MemberScriptPromise<V8UnionBlobOrString>>>
      representations_;

  // The vector of custom MIME types that have a "web " prefix.
  Vector<String> custom_format_types_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CLIPBOARD_CLIPBOARD_ITEM_H_
