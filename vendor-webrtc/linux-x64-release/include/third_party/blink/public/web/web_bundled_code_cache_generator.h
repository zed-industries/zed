// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BUNDLED_CODE_CACHE_GENERATOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BUNDLED_CODE_CACHE_GENERATOR_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {

class WebString;

// WebCodeCacheGenerator is an API to generate serialized code cache data for a
// given JS module for distribution in Chromium's resource bundle.
// Generated code cache assumes UTF-8 script encoding, which is true of Chromium
// bundled JS scripts.
// This API should be used only by tools/code_cache_generator, which runs during
// Chromium's build step.
class BLINK_EXPORT WebBundledCodeCacheGenerator {
 public:
  using SerializedCodeCacheData = std::vector<uint8_t>;
  static SerializedCodeCacheData CreateSerializedCodeCacheForModule(
      v8::Isolate* isolate,
      const WebString& module_text);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_BUNDLED_CODE_CACHE_GENERATOR_H_
