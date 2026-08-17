// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_ERROR_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8.h"

namespace blink {

class ScriptState;

// ImportMapError carries around the data needed to instantiate a V8 error.
// ImportMapError enables us to decouple ImportMap implementation with V8.
class ImportMapError final {
 public:
  enum class Type {
    kTypeError,
    kSyntaxError,
  };

  ImportMapError(Type type, String message)
      : type_(type), message_(std::move(message)) {
    DCHECK(!message_.empty());
  }

  ImportMapError(ImportMapError&&) = default;
  ImportMapError& operator=(ImportMapError&&) = default;

  ~ImportMapError() = default;

  v8::Local<v8::Value> ToV8(ScriptState* script_state);

 private:
  Type type_;
  String message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_ERROR_H_
