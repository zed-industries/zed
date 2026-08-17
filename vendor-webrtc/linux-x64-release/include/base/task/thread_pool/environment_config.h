// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_ENVIRONMENT_CONFIG_H_
#define BASE_TASK_THREAD_POOL_ENVIRONMENT_CONFIG_H_

#include <stddef.h>

#include <array>

#include "base/base_export.h"
#include "base/feature_list.h"
#include "base/task/task_traits.h"
#include "base/threading/thread.h"

namespace base {
namespace internal {

// TODO(etiennep): This is now specific to
// PooledSingleThreadTaskRunnerManager, move it there.
enum EnvironmentType {
  FOREGROUND = 0,
  FOREGROUND_BLOCKING,
  UTILITY,
  UTILITY_BLOCKING,
  BACKGROUND,
  BACKGROUND_BLOCKING,
  ENVIRONMENT_COUNT  // Always last.
};

// Order must match the EnvironmentType enum.
struct EnvironmentParams {
  // The threads and histograms of this environment will be labeled with
  // the thread pool name concatenated to this.
  const char* name_suffix;

  // Preferred type for threads in this environment; the actual thread type
  // depends on shutdown state and platform capabilities.
  ThreadType thread_type_hint;
};

constexpr auto kEnvironmentParams = std::to_array<EnvironmentParams>({
    {"Foreground", base::ThreadType::kDefault},
    {"ForegroundBlocking", base::ThreadType::kDefault},
    {"Utility", base::ThreadType::kUtility},
    {"UtilityBlocking", base::ThreadType::kUtility},
    {"Background", base::ThreadType::kBackground},
    {"BackgroundBlocking", base::ThreadType::kBackground},
});

// Returns true if this platform supports having WorkerThreads running with a
// background thread type.
bool BASE_EXPORT CanUseBackgroundThreadTypeForWorkerThread();

// Returns true if this platform supports having WorkerThreads running with a
// utility thread type.
bool BASE_EXPORT CanUseUtilityThreadTypeForWorkerThread();

#if BUILDFLAG(IS_ANDROID)
const base::Feature& BASE_EXPORT
FeatureControllingBackgroundPriorityWorkerThreads();
#endif  // BUILDFLAG(IS_ANDROID)

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_ENVIRONMENT_CONFIG_H_
