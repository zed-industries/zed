// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CROSS_THREAD_TASK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CROSS_THREAD_TASK_H_

#include <utility>

#include "base/location.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// For cross-thread posting. Can be called from any thread.
inline bool PostCrossThreadTask(base::SequencedTaskRunner& task_runner,
                                const base::Location& location,
                                WTF::CrossThreadOnceClosure task) {
  return task_runner.PostTask(location,
                              ConvertToBaseOnceCallback(std::move(task)));
}

inline bool PostDelayedCrossThreadTask(base::SequencedTaskRunner& task_runner,
                                       const base::Location& location,
                                       WTF::CrossThreadOnceClosure task,
                                       base::TimeDelta delay) {
  return task_runner.PostDelayedTask(
      location, ConvertToBaseOnceCallback(std::move(task)), delay);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_POST_CROSS_THREAD_TASK_H_
