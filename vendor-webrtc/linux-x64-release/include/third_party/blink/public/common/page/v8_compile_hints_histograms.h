// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_V8_COMPILE_HINTS_HISTOGRAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_V8_COMPILE_HINTS_HISTOGRAMS_H_

namespace blink::v8_compile_hints {

inline constexpr const char* kStatusHistogram =
    "WebCore.Scripts.V8CompileHintsStatus";

inline constexpr const char* kLocalCompileHintsGeneratedHistogram =
    "WebCore.Scripts.V8LocalCompileHintsGenerated";

inline constexpr const char* kLocalCompileHintsObsoletedByCodeCacheHistogram =
    "WebCore.Scripts.V8LocalCompileHintsObsoletedByCodeCache";

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class Status {
  kConsumeLocalCompileHintsClassicNonStreaming = 0,
  kConsumeLocalCompileHintsModuleNonStreaming = 1,
  kConsumeCrowdsourcedCompileHintsClassicNonStreaming = 2,
  kConsumeCrowdsourcedCompileHintsModuleNonStreaming = 3,
  kProduceCompileHintsClassicNonStreaming = 4,
  kProduceCompileHintsModuleNonStreaming = 5,
  kConsumeCodeCacheClassicNonStreaming = 6,
  kConsumeCodeCacheModuleNonStreaming = 7,
  kConsumeLocalCompileHintsStreaming = 8,
  kConsumeCrowdsourcedCompileHintsStreaming = 9,
  kProduceCompileHintsStreaming = 10,
  kNoCompileHintsClassicNonStreaming = 11,
  kNoCompileHintsModuleNonStreaming = 12,
  kNoCompileHintsStreaming = 13,
  kMaxValue = kNoCompileHintsStreaming
};

enum class LocalCompileHintsGenerated {
  kNonFinal = 0,
  kFinal = 1,
  kMaxValue = kFinal
};

}  // namespace blink::v8_compile_hints

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PAGE_V8_COMPILE_HINTS_HISTOGRAMS_H_
