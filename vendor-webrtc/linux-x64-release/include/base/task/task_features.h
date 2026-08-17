// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_TASK_FEATURES_H_
#define BASE_TASK_TASK_FEATURES_H_

#include "base/base_export.h"
#include "base/feature_list.h"
#include "base/metrics/field_trial_params.h"
#include "build/build_config.h"

namespace base {

// Fixed amount of threads that will be used as a cap for thread pools.
BASE_EXPORT BASE_DECLARE_FEATURE(kThreadPoolCap2);

BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(int, kThreadPoolCapRestrictedCount);

// Under this feature, a utility_thread_group will be created for
// running USER_VISIBLE tasks.
BASE_EXPORT BASE_DECLARE_FEATURE(kUseUtilityThreadGroup);

// Under this feature, a non-zero leeway is added to delayed tasks. Along with
// DelayPolicy, this affects the time at which a delayed task runs.
BASE_EXPORT BASE_DECLARE_FEATURE(kAddTaskLeewayFeature);
#if BUILDFLAG(IS_WIN)
constexpr TimeDelta kDefaultLeeway = Milliseconds(16);
#else
constexpr TimeDelta kDefaultLeeway = Milliseconds(8);
#endif  // #if !BUILDFLAG(IS_WIN)
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(TimeDelta, kTaskLeewayParam);

// We consider that delayed tasks above |kMaxPreciseDelay| never need
// DelayPolicy::kPrecise. The default value is slightly above 30Hz timer.
constexpr TimeDelta kDefaultMaxPreciseDelay = Milliseconds(36);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(TimeDelta, kMaxPreciseDelay);

// Under this feature, wake ups are aligned at a 8ms boundary when allowed per
// DelayPolicy.
BASE_EXPORT BASE_DECLARE_FEATURE(kAlignWakeUps);

// Under this feature, slack is added on mac message pumps that support it when
// allowed per DelayPolicy.
BASE_EXPORT BASE_DECLARE_FEATURE(kTimerSlackMac);

// Under this feature, the Windows UI pump uses a WaitableEvent to wake itself
// up when not in a native nested loop. It also uses different control flow,
// calling Win32 MessagePump functions less often.
BASE_EXPORT BASE_DECLARE_FEATURE(kUIPumpImprovementsWin);

// Under this feature, the Android pump will call ALooper_PollOnce() rather than
// unconditionally yielding to native to determine whether there exists native
// work to be done before sleep.
BASE_EXPORT BASE_DECLARE_FEATURE(kPumpFastToSleepAndroid);

// Feature to run tasks by batches before pumping out messages.
BASE_EXPORT BASE_DECLARE_FEATURE(kRunTasksByBatches);

}  // namespace base

#endif  // BASE_TASK_TASK_FEATURES_H_
