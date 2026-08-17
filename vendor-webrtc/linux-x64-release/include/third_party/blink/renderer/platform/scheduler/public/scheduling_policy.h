// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_POLICY_H_

#include "base/traits_bag.h"
#include "third_party/blink/public/common/scheduler/web_scheduler_tracked_feature.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// A list of things a feature can opt out from on the behalf of the page
// if the page is using this feature.
// See FrameOrWorkerScheduler::RegisterFeature.
struct PLATFORM_EXPORT SchedulingPolicy {
  using Feature = scheduler::WebSchedulerTrackedFeature;

  // List of opt-outs which form a policy.
  struct DisableAggressiveThrottling {};
  struct DisableBackForwardCache {};
  struct DisableAlignWakeUps {};

  struct ValidPolicies {
    ValidPolicies(DisableAggressiveThrottling);
    ValidPolicies(DisableBackForwardCache);
    ValidPolicies(DisableAlignWakeUps);
  };

  template <class... ArgTypes>
    requires base::trait_helpers::AreValidTraits<ValidPolicies, ArgTypes...>
  constexpr SchedulingPolicy(ArgTypes... args)
      : disable_aggressive_throttling(
            base::trait_helpers::HasTrait<DisableAggressiveThrottling,
                                          ArgTypes...>()),
        disable_back_forward_cache(
            base::trait_helpers::HasTrait<DisableBackForwardCache,
                                          ArgTypes...>()),
        disable_align_wake_ups(
            base::trait_helpers::HasTrait<DisableAlignWakeUps, ArgTypes...>()) {
  }

  SchedulingPolicy() {}

  bool disable_aggressive_throttling = false;
  bool disable_back_forward_cache = false;
  bool disable_align_wake_ups = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SCHEDULER_PUBLIC_SCHEDULING_POLICY_H_
