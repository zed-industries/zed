// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_ERROR_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;

class ScriptWebBundleError final {
 public:
  // TODO: figure out what to show the user when
  // the type is kSystemError.
  enum class Type {
    kTypeError,
    kSyntaxError,
    kSystemError,
  };

  ScriptWebBundleError(Type type, String message)
      : type_(type), message_(std::move(message)) {
    DCHECK(!message_.empty());
  }

  ~ScriptWebBundleError() = default;

  v8::Local<v8::Value> ToV8(ScriptState* script_state);
  Type GetType() const { return type_; }
  const String& GetMessage() const { return message_; }

 private:
  Type type_;
  String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_BUNDLE_SCRIPT_WEB_BUNDLE_ERROR_H_
