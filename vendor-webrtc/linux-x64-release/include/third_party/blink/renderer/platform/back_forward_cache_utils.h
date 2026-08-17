// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_UTILS_H_

#include <string>

#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Returns true iff back-forward cache and LoadingTasksUnfreezable are enabled.
PLATFORM_EXPORT bool IsInflightNetworkRequestBackForwardCacheSupportEnabled();

// Returns the param |param_name| of LoadingTasksUnfreezable as int, or
// |default_value| if not set.
PLATFORM_EXPORT int GetLoadingTasksUnfreezableParamAsInt(
    const std::string& param_name,
    int default_value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_UTILS_H_
