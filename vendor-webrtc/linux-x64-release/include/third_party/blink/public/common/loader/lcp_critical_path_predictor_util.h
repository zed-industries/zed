// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LCP_CRITICAL_PATH_PREDICTOR_UTIL_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LCP_CRITICAL_PATH_PREDICTOR_UTIL_H_

#include "third_party/blink/public/common/common_export.h"

namespace blink {

BLINK_COMMON_EXPORT bool LcppEnabled();
BLINK_COMMON_EXPORT void ResetLcppEnabledForTesting();
BLINK_COMMON_EXPORT bool LcppScriptObserverEnabled();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_LCP_CRITICAL_PATH_PREDICTOR_UTIL_H_
