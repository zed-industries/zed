// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_ERROR_H_

#include "third_party/blink/public/mojom/push_messaging/push_messaging.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ScriptPromiseResolverBase;

class PushError {
  STATIC_ONLY(PushError);

 public:
  // For CallbackPromiseAdapter.
  using WebType = const mojom::PushErrorType;
  static DOMException* Take(ScriptPromiseResolverBase* resolver,
                            mojom::PushErrorType error) {
    return CreateException(error);
  }

  static DOMException* CreateException(mojom::PushErrorType error,
                                       const String& message = String());
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PUSH_MESSAGING_PUSH_ERROR_H_
