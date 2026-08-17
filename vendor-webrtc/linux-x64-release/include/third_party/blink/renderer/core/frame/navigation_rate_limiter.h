// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATION_RATE_LIMITER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATION_RATE_LIMITER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {
class Frame;

// TODO(https://crbug.com/394296, https://crbug.com/882238)
// Prevent the renderer process to flood the browser process by sending IPC for
// same-document navigations.
// This is not the long-term fix to IPC flooding. However, it mitigate the
// immediate concern assuming the renderer has not been compromised.
class NavigationRateLimiter final {
  DISALLOW_NEW();

 public:
  explicit NavigationRateLimiter(Frame&);

  // Notify this object a new navigation is requested. Return true if this one
  // is allowed to proceed.
  bool CanProceed();

  void Trace(Visitor*) const;

 private:
  Member<Frame> frame_;
  base::TimeTicks time_first_count_;
  int count_ = 0;
  bool enabled;

  bool error_message_sent_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_NAVIGATION_RATE_LIMITER_H_
