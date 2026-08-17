// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_CALLBACK_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_CALLBACK_MANAGER_H_

#include "base/functional/callback_forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace base {
class TimeTicks;
}

namespace blink {
struct DOMPaintTimingInfo;
// `PaintTimingCallbackManager` is a unit-test specific interface to capture
// callbacks so that the lifecycle can be be controlled synchronously.
class PaintTimingCallbackManager : public GarbageCollectedMixin {
 public:
  using Callback = base::OnceCallback<void(const base::TimeTicks&,
                                           const DOMPaintTimingInfo&)>;

  virtual void RegisterCallback(Callback) = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_PAINT_TIMING_CALLBACK_MANAGER_H_
