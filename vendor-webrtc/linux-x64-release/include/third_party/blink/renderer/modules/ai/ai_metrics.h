// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_METRICS_H_

#include <string>

namespace blink {

class AIMetrics {
 public:
  // This class contains all the supported session types.
  // LINT.IfChange(AISessionType)
  enum class AISessionType {
    kLanguageModel = 0,
    kWriter = 1,
    kRewriter = 2,
    kSummarizer = 3,
    kTranslator = 4,
    kLanguageDetector = 5,
    kMaxValue = kLanguageDetector,
  };
  // LINT.ThenChange(//tools/metrics/histograms/metadata/ai/histograms.xml:SessionType)

  // This class contains all the model execution API supported.
  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  // TODO(crbug.com/355967885): update the enums when adding metrics for
  // language model API.
  // LINT.IfChange(AIAPI)
  enum class AIAPI {
    kCanCreateSession = 0,
    kCreateSession = 1,
    kSessionPrompt = 2,
    kSessionPromptStreaming = 3,
    kDefaultTextSessionOptions = 4,
    kSessionDestroy = 5,
    kSessionClone = 6,
    kTextModelInfo = 7,
    kSessionSummarize = 8,
    kSessionSummarizeStreaming = 9,
    kWriterWrite = 10,
    kWriterWriteStreaming = 11,
    kRewriterRewrite = 12,
    kRewriterRewriteStreaming = 13,
    kSummarizerSummarize = 14,
    kSummarizerSummarizeStreaming = 15,
    kSummarizerCreate = 16,
    kSummarizerDestroy = 17,
    kSessionCountPromptTokens = 18,

    kMaxValue = kSessionCountPromptTokens,
  };
  // LINT.ThenChange(//tools/metrics/histograms/metadata/ai/enums.xml:AIAPI)

  static std::string GetAIAPIUsageMetricName(AISessionType session_type);
  static std::string GetAvailabilityMetricName(AISessionType session_type);
  static std::string GetAISessionRequestSizeMetricName(
      AISessionType session_type);
  static std::string GetAISessionResponseStatusMetricName(
      AISessionType session_type);
  static std::string GetAISessionResponseSizeMetricName(
      AISessionType session_type);
  static std::string GetAISessionResponseCallbackCountMetricName(
      AISessionType session_type);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_METRICS_H_
