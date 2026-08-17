// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_FEATURES_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_FEATURES_H_

#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8.h"

namespace blink {

class BLINK_EXPORT WebAIFeatures {
 public:
  // Methods exposing built-in AI runtime features for extension renderer.
  static bool IsPromptAPIEnabledForWebPlatform(
      v8::Local<v8::Context> v8_context);
  static bool IsPromptAPIEnabledForExtension(v8::Local<v8::Context> v8_context);
  static bool IsSummarizationAPIEnabled(v8::Local<v8::Context> v8_context);
  static bool IsWriterAPIEnabled(v8::Local<v8::Context> v8_context);
  static bool IsRewriterAPIEnabled(v8::Local<v8::Context> v8_context);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_FEATURES_H_
