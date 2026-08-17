// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_TEST_UTILS_H_

#include "third_party/blink/public/common/privacy_budget/identifiability_sample_collector.h"

namespace blink {
// Sets a IdentifiabilitySampleCollector that
// IdentifiabilitySampleCollector::Get() will return instead of the per-process
// global.
//
// Call with `new_collector` set to `nullptr` to reset to the default collector.
// Callers MUST do this before `new_collector` is destroyed.
//
// This function should not be called to set nested aggregators. I.e. only one
// test collector can be active at one time.
BLINK_COMMON_EXPORT void SetCollectorInstanceForTesting(
    ::blink::IdentifiabilitySampleCollector* new_collector);

// Resets the state of the per-process global collector. Note that this
// modifies the global collector even if SetCollectorInstanceForTesting() has
// overridden it.
BLINK_COMMON_EXPORT void ResetCollectorInstanceStateForTesting();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_TEST_UTILS_H_
