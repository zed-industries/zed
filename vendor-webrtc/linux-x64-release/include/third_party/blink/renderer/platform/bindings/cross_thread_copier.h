// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CROSS_THREAD_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CROSS_THREAD_COPIER_H_

#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace WTF {

template <>
struct CrossThreadCopier<blink::ExceptionContext> {
  STATIC_ONLY(CrossThreadCopier);
  static blink::ExceptionContext Copy(
      blink::ExceptionContext exception_context) {
    return exception_context;
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_CROSS_THREAD_COPIER_H_
