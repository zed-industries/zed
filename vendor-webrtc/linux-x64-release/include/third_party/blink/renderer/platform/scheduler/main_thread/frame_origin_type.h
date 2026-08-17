// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_ORIGIN_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_ORIGIN_TYPE_H_

#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

namespace blink {
class FrameScheduler;

namespace scheduler {

// This enum is used for a histogram (RendererSchedulerFrameOriginType)
// and should not be renumbered.
enum class FrameOriginType {
  kMainFrame = 0,
  kSameOriginToMainFrame = 1,
  kCrossOriginToMainFrame = 2,
  kMaxValue = kCrossOriginToMainFrame,
};

FrameOriginType GetFrameOriginType(FrameScheduler* frame_scheduler);

perfetto::StaticString FrameOriginTypeToString(FrameOriginType origin);

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_MAIN_THREAD_FRAME_ORIGIN_TYPE_H_
