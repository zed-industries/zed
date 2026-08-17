// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_RECORDING_TASK_TIME_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_RECORDING_TASK_TIME_OBSERVER_H_

#include <utility>

#include "base/task/sequence_manager/task_time_observer.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
namespace scheduler {

class RecordingTaskTimeObserver
    : public base::sequence_manager::TaskTimeObserver {
 public:
  using Result = Vector<std::pair<base::TimeTicks, base::TimeTicks>>;

  RecordingTaskTimeObserver();
  ~RecordingTaskTimeObserver() override;

  void Clear();

  // base::sequence_manager::TaskTimeObserver implementations.
  void WillProcessTask(base::TimeTicks start_time) override;
  void DidProcessTask(base::TimeTicks start_time,
                      base::TimeTicks end_time) override;

  const Result& result() const { return result_; }

 private:
  Result result_;
};

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_TEST_RECORDING_TASK_TIME_OBSERVER_H_
