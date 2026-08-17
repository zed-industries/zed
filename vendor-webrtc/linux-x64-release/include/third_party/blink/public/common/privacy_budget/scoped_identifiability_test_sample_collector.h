// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_IDENTIFIABILITY_TEST_SAMPLE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_IDENTIFIABILITY_TEST_SAMPLE_COLLECTOR_H_

#include <memory>
#include <vector>

#include "base/component_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_sample_collector.h"
#include "third_party/blink/public/common/privacy_budget/scoped_switch_sample_collector.h"

namespace blink {
namespace test {

// An `IdentifiabilitySampleCollector` implementation for testing. Allows
// inspecting recorded metrics.
//
// Instantiating this class automatically sets the per-process
// `IdentifiabilitySampleCollector` to point to the new instance.
//
// Note: Unlike the real collector nothing in this class is thread safe.
class COMPONENT_EXPORT(PRIVACY_BUDGET_TEST_SUPPORT)
    ScopedIdentifiabilityTestSampleCollector
    : public IdentifiabilitySampleCollector {
 public:
  ScopedIdentifiabilityTestSampleCollector();
  ~ScopedIdentifiabilityTestSampleCollector() override;

  // IdentifiabilitySampleCollector
  void Record(ukm::UkmRecorder* recorder,
              ukm::SourceId source,
              std::vector<IdentifiableSample> metrics) override;
  void Flush(ukm::UkmRecorder* recorder) override;
  void FlushSource(ukm::UkmRecorder* recorder, ukm::SourceId source) override;

  // Each call to `Record()` results in one of these being added to `entries()`
  // in order of occurrence that faithfully records the arguments to `Record()`.
  struct Entry {
    Entry(ukm::SourceId source_in, std::vector<IdentifiableSample> metrics_in)
        : source(source_in), metrics(std::move(metrics_in)) {}

    const ukm::SourceId source;
    const std::vector<IdentifiableSample> metrics;
  };

  // Returns a reference to the list of `Entry` objects representing the
  // `Record()` calls received so far.
  const std::vector<Entry>& entries() const { return entries_; }

  // Reset all recorded entries. `entries()` returns an empty list after this.
  void ClearEntries();

 private:
  std::vector<Entry> entries_;
  ScopedSwitchSampleCollector scoped_default_;
};

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_SCOPED_IDENTIFIABILITY_TEST_SAMPLE_COLLECTOR_H_
