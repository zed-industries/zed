// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_LANGUAGE_MODEL_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_LANGUAGE_MODEL_H_

#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8.h"

namespace blink {

class BLINK_EXPORT WebAILanguageModel {
 public:
  // Returns the `self.ai.languageModel` value even if it's not generated
  // through the binding (i.e. the runtime enabled feature controlling the
  // interface is disabled). This method is used for creating the
  // `chrome.aiOriginTrial.languageModel` for extension, see
  // `NativeExtensionBindingsSystem::UpdateBindingsForPromptAPI` for more
  // information.
  static v8::Local<v8::Value> GetLanguageModelFactory(
      v8::Local<v8::Context> v8_context,
      v8::Isolate* isolate);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_AI_WEB_AI_LANGUAGE_MODEL_H_
