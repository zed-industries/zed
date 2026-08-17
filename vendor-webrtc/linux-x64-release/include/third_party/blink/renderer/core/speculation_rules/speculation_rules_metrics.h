// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_METRICS_H_

namespace blink {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class SpeculationRulesLoadOutcome {
  kSuccess = 0,
  kUnparseableSpeculationRulesHeader = 1,
  kEmptySpeculationRulesHeader = 2,
  kInvalidSpeculationRulesHeaderItem = 3,
  kLoadFailedOrCanceled = 4,
  kInvalidMimeType = 5,
  kEmptyResponseBody = 6,
  kParseErrorFetched = 7,
  kParseErrorInline = 8,
  kParseErrorBrowserInjected = 9,
  kAutoSpeculationRulesOptedOut = 10,
  kMaxValue = kAutoSpeculationRulesOptedOut,
};

void CountSpeculationRulesLoadOutcome(SpeculationRulesLoadOutcome);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_SPECULATION_RULES_METRICS_H_
