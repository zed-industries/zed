// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POWER_MONITOR_POWER_MONITOR_FEATURES_H_
#define BASE_POWER_MONITOR_POWER_MONITOR_FEATURES_H_

#include "base/base_export.h"
#include "base/feature_list.h"
#include "build/build_config.h"

namespace base {

#if BUILDFLAG(IS_IOS)
// Under this feature, iOS power monitor will not post power suspend/resume
// event notifications on application entering background/foreground. This
// feature can keep tcp socket connection always alive on iOS.
BASE_EXPORT BASE_DECLARE_FEATURE(kRemoveIOSPowerEventNotifications);
#endif

}  // namespace base

#endif  // BASE_POWER_MONITOR_POWER_MONITOR_FEATURES_H_
