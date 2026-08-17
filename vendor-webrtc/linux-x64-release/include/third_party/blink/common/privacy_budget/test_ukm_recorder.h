// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_TEST_UKM_RECORDER_H_
#define THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_TEST_UKM_RECORDER_H_

#include <vector>

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/mojom/ukm_interface.mojom.h"

namespace blink {
namespace test {

// This is a barebones UkmRecorder that has just enough functionality to support
// the testing being done in this directory.
//
// Why this and not `ukm::TestUkmRecorder` ? That one has too many dependencies
// none of which is necessary for the testing we are doing here in which we are
// only testing whether the identifiability reporting mechanism properly encodes
// and passes metrics to the underlying `UkmRecorder`.
class TestUkmRecorder : public ukm::UkmRecorder {
 public:
  TestUkmRecorder();
  ~TestUkmRecorder() override;

  // Count of event entries. This is not the count of metrics since there can be
  // more than one in each event.
  size_t entries_count() const { return entries_.size(); }

  // The entries themselves in the order in which they were received.
  const std::vector<ukm::mojom::UkmEntryPtr>& entries() const {
    return entries_;
  }

  // Returns a vector of pointers to entries whose `UkmEntry::event_hash`
  // matches `event_hash`. The returned pointers should remain valid until
  // `Purge()` is called or this recorder is destroyed.
  std::vector<const ukm::mojom::UkmEntry*> GetEntriesByHash(
      uint64_t event_hash) const;

  // Delete all received entries.
  void Purge() { entries_.clear(); }

  // UkmRecorder
  void AddEntry(ukm::mojom::UkmEntryPtr entry) override {
    entries_.emplace_back(std::move(entry));
  }
  void RecordWebDXFeatures(ukm::SourceId source_id,
                           const std::set<int32_t>& features,
                           const size_t max_feature_value) override {}
  void UpdateSourceURL(ukm::SourceId, const GURL&) override {}
  void UpdateAppURL(ukm::SourceId, const GURL&, const ukm::AppType) override {}
  void RecordNavigation(ukm::SourceId,
                        const ukm::UkmSource::NavigationData&) override {}
  void MarkSourceForDeletion(ukm::SourceId) override {}

 private:
  std::vector<ukm::mojom::UkmEntryPtr> entries_;
};

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_TEST_UKM_RECORDER_H_
