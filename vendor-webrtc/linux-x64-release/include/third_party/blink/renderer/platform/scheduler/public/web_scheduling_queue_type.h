// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_QUEUE_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_QUEUE_TYPE_H_

namespace blink {

// https://wicg.github.io/scheduling-apis/
enum class WebSchedulingQueueType {
  // Used for `scheduler.postTask()` tasks.
  kTaskQueue,

  // Used for `scheduler.yield()` task continuations.
  kContinuationQueue,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_WEB_SCHEDULING_QUEUE_TYPE_H_
