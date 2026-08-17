// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FEATURES_H_
#define BASE_FEATURES_H_

#include "base/base_export.h"
#include "base/feature_list.h"
#include "base/metrics/field_trial_params.h"

namespace base::features {

// All features in alphabetical order. The features should be documented
// alongside the definition of their values in the .cc file.

// Alphabetical:
BASE_EXPORT BASE_DECLARE_FEATURE(kFeatureParamWithCache);

BASE_EXPORT BASE_DECLARE_FEATURE(kFastFilePathIsParent);

BASE_EXPORT BASE_DECLARE_FEATURE(kUseRustJsonParser);

BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(bool,
                                       kUseRustJsonParserInCurrentSequence);

BASE_EXPORT BASE_DECLARE_FEATURE(kLowEndMemoryExperiment);

BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(size_t, kLowMemoryDeviceThresholdMB);

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS)
BASE_EXPORT BASE_DECLARE_FEATURE(kPartialLowEndModeOn3GbDevices);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartialLowEndModeOnMidRangeDevices);
#endif

#if BUILDFLAG(IS_ANDROID)
BASE_EXPORT BASE_DECLARE_FEATURE(kBackgroundNotPerceptibleBinding);
BASE_EXPORT BASE_DECLARE_FEATURE(kCollectAndroidFrameTimelineMetrics);
BASE_EXPORT BASE_DECLARE_FEATURE(
    kPostPowerMonitorBroadcastReceiverInitToBackground);
BASE_EXPORT BASE_DECLARE_FEATURE(kPostGetMyMemoryStateToBackground);
#endif

#if BUILDFLAG(ENABLE_MUTEX_PRIORITY_INHERITANCE)
BASE_EXPORT BASE_DECLARE_FEATURE(kUsePriorityInheritanceMutex);
#endif  // BUILDFLAG(ENABLE_MUTEX_PRIORITY_INHERITANCE)

// Policy for emitting profiler metadata from `ThreadController`.
enum class EmitThreadControllerProfilerMetadata {
  // Always emit metadata.
  kForce,
  // Emit metadata only if enabled via the `FeatureList`.
  kFeatureDependent,
};

// Initializes global variables that depend on `FeatureList`. Must be invoked
// early on process startup, but after `FeatureList` initialization. Different
// parts of //base read experiment state from global variables instead of
// directly from `FeatureList` to avoid data races (default values are used
// before this function is called to initialize the global variables).
BASE_EXPORT void Init(EmitThreadControllerProfilerMetadata
                          emit_thread_controller_profiler_metadata);

}  // namespace base::features

#endif  // BASE_FEATURES_H_
