// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREADING_FEATURES_H_
#define BASE_THREADING_THREADING_FEATURES_H_

#include "base/base_export.h"
#include "base/feature_list.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include "base/metrics/field_trial_params.h"
#endif

namespace base {

#if BUILDFLAG(IS_APPLE)
BASE_EXPORT BASE_DECLARE_FEATURE(kOptimizedRealtimeThreadingMac);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    bool,
    kOptimizedRealtimeThreadingMacPreemptible);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(double,
                                       kOptimizedRealtimeThreadingMacBusy);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(double,
                                       kOptimizedRealtimeThreadingMacBusyLimit);
BASE_EXPORT BASE_DECLARE_FEATURE(kUserInteractiveCompositingMac);
#endif

BASE_EXPORT BASE_DECLARE_FEATURE(kEnableHangWatcher);

}  // namespace base

#endif  // BASE_THREADING_THREADING_FEATURES_H_
