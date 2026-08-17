// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_CONSUMER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CachedMetadata;

namespace v8_compile_hints {

// Reads V8 compile hints from the cached data and makes them available for V8.
class CORE_EXPORT V8LocalCompileHintsConsumer {
 public:
  explicit V8LocalCompileHintsConsumer(CachedMetadata* cached_metadata);

  V8LocalCompileHintsConsumer(const V8LocalCompileHintsConsumer&) = delete;
  V8LocalCompileHintsConsumer& operator=(const V8LocalCompileHintsConsumer&) =
      delete;

  // Suitable for being used as a callback. data must be a ptr to a
  // V8LocalCompileHintsConsumer object.
  static bool GetCompileHint(int position, void* data);

  bool GetCompileHint(int position);

  bool IsRejected() const { return rejected_; }

 private:
  Vector<int32_t> compile_hints_;
  wtf_size_t current_index_ = 0;
  bool rejected_ = false;
};

}  // namespace v8_compile_hints
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_LOCAL_COMPILE_HINTS_CONSUMER_H_
