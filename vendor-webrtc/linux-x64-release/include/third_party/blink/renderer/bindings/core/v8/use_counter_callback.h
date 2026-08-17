// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_USE_COUNTER_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_USE_COUNTER_CALLBACK_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "v8/include/v8.h"

namespace blink {

// Callback that is used to count the number of times a V8 feature is used.
CORE_EXPORT void UseCounterCallback(v8::Isolate*,
                                    v8::Isolate::UseCounterFeature);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_USE_COUNTER_CALLBACK_H_
