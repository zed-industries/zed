// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_PRIORITY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_PRIORITY_H_

namespace blink {

// https://wicg.github.io/scheduling-apis/#sec-task-priorities
enum class WebSchedulingPriority {
  kUserBlockingPriority = 0,
  kUserVisiblePriority = 1,
  kBackgroundPriority = 2,

  kLastPriority = kBackgroundPriority
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_PRIORITY_H_
