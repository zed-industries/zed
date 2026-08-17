// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TYPE_H_

#include "third_party/perfetto/include/perfetto/tracing/string_helpers.h"

namespace blink::scheduler {

// Types of throttling that can be applied to a task queue.
enum class ThrottlingType {
  // No throttling
  kNone,
  // Wake ups aligned to 32ms
  kForegroundUnimportant,
  // Wake ups aligned to 1 second
  kBackground,
  // Wake ups aligned to 1 second or 1 minute, depending on queue properties and
  // page state.
  kBackgroundIntensive,
};

perfetto::StaticString ThrottlingTypeToString(ThrottlingType type);

}  // namespace blink::scheduler

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_THROTTLING_TYPE_H_
