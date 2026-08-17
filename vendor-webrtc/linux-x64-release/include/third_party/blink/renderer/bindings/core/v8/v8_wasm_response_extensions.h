// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_WASM_RESPONSE_EXTENSIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_WASM_RESPONSE_EXTENSIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

// Injects Web Platform - specific overloads for WebAssembly APIs.
// See https://github.com/WebAssembly/design/blob/master/Web.md
class CORE_EXPORT WasmResponseExtensions {
  STATIC_ONLY(WasmResponseExtensions);

 public:
  static void Initialize(v8::Isolate*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_WASM_RESPONSE_EXTENSIONS_H_
