// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_API_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_API_H_

// This file includes the entry points for the Sanitizer API.

namespace blink {

class ContainerNode;
class SetHTMLOptions;
class SetHTMLUnsafeOptions;
class ExceptionState;

class SanitizerAPI final {
 public:
  static void SanitizeSafeInternal(ContainerNode* element,
                                   SetHTMLOptions* options,
                                   ExceptionState& exception_state);
  static void SanitizeUnsafeInternal(ContainerNode* element,
                                     SetHTMLUnsafeOptions* options,
                                     ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SANITIZER_SANITIZER_API_H_
