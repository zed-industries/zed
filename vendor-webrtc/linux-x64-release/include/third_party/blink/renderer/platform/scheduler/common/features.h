// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_FEATURES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_FEATURES_H_

#include "base/feature_list.h"
#include "base/metrics/field_trial_params.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
namespace scheduler {

BASE_FEATURE(kDedicatedWorkerThrottling,
             "BlinkSchedulerWorkerThrottling",
             base::FEATURE_DISABLED_BY_DEFAULT);

BASE_FEATURE(kBestEffortPriorityForFindInPage,
             "BlinkSchedulerBestEffortPriorityForFindInPage",
             base::FEATURE_DISABLED_BY_DEFAULT);

// Enable setting high priority database task type from field trial parameters.
BASE_FEATURE(kHighPriorityDatabaseTaskType,
             "HighPriorityDatabaseTaskType",
             base::FEATURE_DISABLED_BY_DEFAULT);

// When features::kIntensiveWakeUpThrottling is enabled, wake ups from timers
// with a high nesting level are limited to 1 per minute on a page that has been
// backgrounded for GetIntensiveWakeUpThrottlingGracePeriod().
//
// Intensive wake up throttling is enforced in addition to other throttling
// mechanisms:
//  - 1 wake up per second in a background page or hidden cross-origin frame
//  - 1% CPU time in a page that has been backgrounded for 10 seconds
//
// Feature tracking bug: https://crbug.com/1075553
//
// Note that features::kIntensiveWakeUpThrottling should not be read from;
// rather the provided accessors should be used, which also take into account
// the managed policy override of the feature.
//
// Parameter name and default values, exposed for testing.
constexpr int kIntensiveWakeUpThrottling_GracePeriodSeconds_Default = 5 * 60;
constexpr int kIntensiveWakeUpThrottling_GracePeriodSecondsLoaded_Default = 60;

// Exposed so that multiple tests can tinker with the policy override.
PLATFORM_EXPORT void
ClearIntensiveWakeUpThrottlingPolicyOverrideCacheForTesting();
// Determines if the feature is enabled, taking into account base::Feature
// settings and policy overrides.
PLATFORM_EXPORT bool IsIntensiveWakeUpThrottlingEnabled();
// Grace period after hiding a page during which there is no intensive wake up
// throttling for the kIntensiveWakeUpThrottling feature.
// |loading| is the loading state of the page, used to determine if the grace
// period should be overwritten when kQuickIntensiveWakeUpThrottlingAfterLoading
// is enabled.
PLATFORM_EXPORT base::TimeDelta GetIntensiveWakeUpThrottlingGracePeriod(
    bool loading);

// If enabled, base::SingleThreadTaskRunner::GetCurrentDefault() and
// base::SequencedTaskRunner::GetCurrentDefault() returns the current active
// per-ASG task runner instead of the per-thread task runner.
BASE_FEATURE(kMbiOverrideTaskRunnerHandle,
             "MbiOverrideTaskRunnerHandle",
             base::FEATURE_DISABLED_BY_DEFAULT);

// Buffer time that we want to extend the loading state after the FMP is
// received.
PLATFORM_EXPORT base::TimeDelta
GetLoadingPhaseBufferTimeAfterFirstMeaningfulPaint();

// Kill switch for throttling timed-out requestIdleCallback tasks.
PLATFORM_EXPORT BASE_DECLARE_FEATURE(kThrottleTimedOutIdleTasks);

// crbug.com/40785325 and crbug.com/378738907: If enabled, the signals used for
// idle periods are requested via WidgetScheduler::Delegate instead of via
// PageScheduler. This fixes a few issues where we might not get the requisite
// RequestBeginMainFrameNotExpected signals from a BeginMainFrame source, but
// are receiving BeginMainFrames, which can cause idle tasks to stop running.
BASE_FEATURE(kUseWidgetSchedulerForIdlePeriodSignals,
             "UseWidgetSchedulerForIdlePeriodSignals",
             base::FEATURE_ENABLED_BY_DEFAULT);

}  // namespace scheduler
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_COMMON_FEATURES_H_
