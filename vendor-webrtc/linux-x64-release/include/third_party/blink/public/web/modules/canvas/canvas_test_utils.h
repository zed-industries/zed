// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CANVAS_CANVAS_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CANVAS_CANVAS_TEST_UTILS_H_

#include "third_party/blink/public/platform/web_common.h"
#include "v8/include/v8-forward.h"

namespace blink {

// Returns true if `value` is a CanvasImageSource whose data is stored on the
// GPU
BLINK_MODULES_EXPORT bool IsAcceleratedCanvasImageSource(
    v8::Isolate* isolate,
    v8::Local<v8::Value> value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_CANVAS_CANVAS_TEST_UTILS_H_
