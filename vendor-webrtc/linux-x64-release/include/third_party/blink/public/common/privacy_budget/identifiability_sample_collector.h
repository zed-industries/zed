// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_H_

#include <utility>
#include <vector>

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_sample.h"

namespace blink {

// Do not use this directly. Don't report identifiability metrics directly via
// `UkmRecorder` either.
//
// `UkmRecorder` is not designed for the rate and volume with which
// identifiability metrics are generated. In addition, all of these metrics --
// which aren't really metrics to begin with -- are only required to be recorded
// once per client.
//
// Therefore, rather than report identifiability samples directly to UKM, they
// should instead be funnelled through the global instance of
// `IdentifiabilitySampleCollector` which does the work of de-duplication and
// rate limiting..
//
// `IdentifiabilityMetricBuilder` already uses `IdentifiabilitySampleCollector`
// internally. Look no further.
class BLINK_COMMON_EXPORT IdentifiabilitySampleCollector {
 public:
  virtual ~IdentifiabilitySampleCollector();

  // Gets the singleton per-process collector. Always returns a valid pointer.
  static IdentifiabilitySampleCollector* Get();

  // Record a set of identifiability metrics.
  virtual void Record(ukm::UkmRecorder* recorder,
                      ukm::SourceId source,
                      std::vector<IdentifiableSample> metrics) = 0;

  // Unconditionally write out all pending metrics via `recorder`.
  virtual void Flush(ukm::UkmRecorder* recorder) = 0;

  // Unconditionally write out all pending metrics from `source` via `recorder`.
  virtual void FlushSource(ukm::UkmRecorder* recorder,
                           ukm::SourceId source) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_SAMPLE_COLLECTOR_H_
