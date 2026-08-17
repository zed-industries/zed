// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_UNWIND_DATA_H_
#define BASE_PROFILER_STACK_UNWIND_DATA_H_

#include <memory>
#include <tuple>
#include <vector>

#include "base/base_export.h"
#include "base/containers/circular_deque.h"
#include "base/memory/raw_ptr.h"
#include "base/profiler/frame.h"
#include "base/profiler/register_context.h"
#include "base/sequence_checker.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/thread_annotations.h"

namespace base {

class ProfileBuilder;
class Unwinder;
class ModuleCache;
class UnwinderStateCapture;

using UnwinderCapture =
    std::tuple<raw_ptr<Unwinder>, std::unique_ptr<UnwinderStateCapture>>;

// StackUnwindData is an implementation detail of StackSamplingProfiler. It
// contains data so that we can unwind stacks off the sampling thread.
class BASE_EXPORT StackUnwindData {
 public:
  explicit StackUnwindData(std::unique_ptr<ProfileBuilder> profile_builder);
  ~StackUnwindData();

  ProfileBuilder* profile_builder() { return profile_builder_.get(); }

  ModuleCache* module_cache() {
    DCHECK_CALLED_ON_VALID_SEQUENCE(worker_sequence_checker_);
    return module_cache_;
  }

  // These are called by the SamplingThread.
  void Initialize(std::vector<std::unique_ptr<Unwinder>> unwinders);
  std::vector<UnwinderCapture> GetUnwinderSnapshot();
  void OnThreadPoolRunning();

  // This may be called by either:
  // 1) the thread to sample if we haven't started sampling
  // 2) the SamplingThread
  void AddAuxUnwinder(std::unique_ptr<Unwinder> unwinder);

 private:
  SEQUENCE_CHECKER(sampling_thread_sequence_checker_);
  SEQUENCE_CHECKER(worker_sequence_checker_);

  // Receives the sampling data and builds a CallStackProfile.
  std::unique_ptr<ProfileBuilder> profile_builder_;

  // Unwinders are stored in decreasing priority order.
  base::circular_deque<std::unique_ptr<Unwinder>> unwinders_
      GUARDED_BY_CONTEXT(sampling_thread_sequence_checker_);
  const raw_ptr<ModuleCache> module_cache_;
};

}  // namespace base

#endif  // BASE_PROFILER_STACK_UNWIND_DATA_H_
