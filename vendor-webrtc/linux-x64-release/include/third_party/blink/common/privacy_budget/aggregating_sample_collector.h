// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_AGGREGATING_SAMPLE_COLLECTOR_H_
#define THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_AGGREGATING_SAMPLE_COLLECTOR_H_

#include <cstdint>
#include <unordered_map>
#include <vector>

#include "base/containers/flat_set.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/mojom/ukm_interface.mojom.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_sample_collector.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"

namespace blink {

// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class PrivacyBudgetRecordedSample {
  kAccepted = 0,
  kDroppedMaxTrackedSources = 1,
  kDroppedMaxTrackedSurfaces = 2,
  kDroppedMaxTrackedPerSurfacePerSource = 3,
  kMaxValue = kDroppedMaxTrackedPerSurfacePerSource,
};

// An `IdentifiabilitySampleCollector` that does the following:
//
// * De-duplicates recorded samples so that the same
//   〈IdentifiableSurface,IdentifiableToken〉 tuple doesn't get sent to the
//   UkmRecorder more than once per `ukm::SourceId`.
//
// * Caps the number of samples that can be recorded against the same surface
//   per `ukm::SourceId`. Drops samples in excess of
//   kMaxTrackedSamplesPerSurfaces.
//
// * Caps the total number of surfaces that can be tracked for a single process.
//   Drops samples in excess of kMaxTrackedSurfaces.
//
// * Buffers metrics instead of invoking `UkmRecorder::Record` each time
//   a sample arrives.
//
//   * The number of metrics so buffered is capped at kMaxUnsentSamples. If more
//     than this many are to be buffered, then flushes all unsent metrics.
//
//   * The age of metrics so buffered is capped at kMaxUnsentSampleAge. If
//     samples have been sitting in the unsent buffer for longer than that,
//     flushes all unsent metrics.
//
//   * In so buffering, organizes observed metrics into the fewest number of
//     `UkmEntry` instances that are required to record them via `UkmRecorder.
//
// The goal, obviously is to prevent the identifiability study from DoSing the
// browser process and the UKM subsystem since there can be lots of metrics
// being recorded.
class BLINK_COMMON_EXPORT_PRIVATE AggregatingSampleCollector
    : public IdentifiabilitySampleCollector {
 public:
  // Maximum number of surfaces that this class can track. Prevents unbounded
  // memory growth.
  static constexpr unsigned kMaxTrackedSurfaces = 10000;

  // Maximum number of sources that this class can track. Prevents unbounded
  // memory growth.
  static constexpr unsigned kMaxTrackedSources = 10000;

  // Surfaces may return different values. To account for those, this class
  // tracks the last several distinct samples that were seen for each surface.
  // This is the maximum number of such samples that can be tracked. Again meant
  // as a precaution against unbounded memory growth.
  //
  // If a surface is generating much more than this many distinct samples, it is
  // considered "noisy" and may be considered for removal from the study.
  static constexpr unsigned kMaxTrackedSamplesPerSurfacePerSourceId = 3;

  // Maximum number of unsent samples. This class will automatically flush all
  // samples if this limit overflows.
  static constexpr unsigned kMaxUnsentSamples = 200;

  // Maximum number of sources that this class can track. Flushes automatically
  // if this limit overflows.
  static constexpr unsigned kMaxUnsentSources = 100;

  // Maximum age of the oldest sample in the unsent collection. Again, the class
  // will flush all samples if this limit overflows.
  static constexpr base::TimeDelta kMaxUnsentSampleAge = base::Seconds(5);

  // Should be the same as the type for ukm::UkmEntry::metrics_
  using UkmMetricsContainerType = decltype(ukm::mojom::UkmEntry::metrics);

  AggregatingSampleCollector();
  ~AggregatingSampleCollector() override;

  // IdentifiabilitySampleCollector
  void Record(ukm::UkmRecorder* recorder,
              ukm::SourceId source,
              std::vector<IdentifiableSample> metrics) override
      LOCKS_EXCLUDED(lock_);
  void Flush(ukm::UkmRecorder* recorder) override LOCKS_EXCLUDED(lock_);

  // FlushSource flushes the metrics per source. This will also reset all limits
  // relative to this source.
  void FlushSource(ukm::UkmRecorder* recorder, ukm::SourceId source) override
      LOCKS_EXCLUDED(lock_);

  // Only for testing.
  void ResetForTesting() LOCKS_EXCLUDED(lock_);

 private:
  // Each tracked `IdentifiableSurface` has a corresponding `Samples` instance.
  struct Samples {
    // `samples.size() <= kMaxTrackedSamplesPerSurfaces`. Typically we only want
    // to keep a very small number of these around. Note that we don't do any
    // fancy reservoir sampling or approximate counting here due to the size of
    // the required sketches.
    base::flat_set<IdentifiableToken> samples;

    // true if unique sample count exceeds kMaxTrackedSamplesPerSurfaces.
    bool overflowed = false;

    // Total count of samples that we've observed for this surface. Includes
    // duplicates.
    int total_value_count = 0;
  };

  // Attempts to record the samples in `samples`. Returns true if unsent metrics
  // should be flushed based on the resulting state of `unsent_metrics_`.
  bool TryAcceptSamples(ukm::SourceId source,
                        std::vector<IdentifiableSample> samples)
      LOCKS_EXCLUDED(lock_);

  // Accepts `sample` if it meets certain criteria. Typically duplicates are
  // dropped. So are samples that if accepted causes some limit to be exceeded.
  void TryAcceptSingleSample(ukm::SourceId source,
                             const IdentifiableSample& sample)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // Populates `unsent_metrics_` based on a single `IdentifiableSample`.
  void AddNewUnsentSample(ukm::SourceId source,
                          const IdentifiableSample& sample)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // If there is a `UkmMetricsContainerType` for `source`, the method of adding
  // another sample is stunningly different from the case where `source` is new.
  // In the former case, the existing `UkmMetricsContainerType` objects need to
  // be checked if any of them can be used to store the 〈`key`, `value`〉 pair.
  bool AddNewUnsentSampleToKnownSource(ukm::SourceId source,
                                       uint64_t key,
                                       int64_t value)
      EXCLUSIVE_LOCKS_REQUIRED(lock_);

  // We are heavily dependent on the property that the reference to a value in
  // the map isn't invalidated due to mutations other than erase().
  base::flat_map<
      ukm::SourceId,
      std::unordered_map<IdentifiableSurface, Samples, IdentifiableSurfaceHash>>
      per_source_per_surface_samples_ GUARDED_BY(lock_);

  // Seen surfaces across all sources.
  std::unordered_set<IdentifiableSurface, IdentifiableSurfaceHash>
      seen_surfaces_ GUARDED_BY(lock_);

  // An unordered multi-map of metrics that haven't yet been recorded via
  // a `UkmRecorder`.
  //
  //     `unsent_metrics_.size() <= kMaxUnsentSources`.
  //
  // `UkmEntry`'s `metrics` member is a map, and hence cannot be used to store
  // multiple values for a single surface. If more than one value needs to be
  // recorded, they need to be in different entries.
  //
  // At worst, all values recorded for a single source can be recorded in
  // kMaxTrackedSamplesPerSurfaces maps. So that's what we do.
  //
  // Each source maps to one or more UkmMetricsContainerType instances, which
  // happens to be the minimum required to represent all observed and accepted
  // values for all surfaces.
  std::unordered_multimap<ukm::SourceId, UkmMetricsContainerType>
      unsent_metrics_ GUARDED_BY(lock_);

  // Only valid if `unsent_sample_count_ > 0`.
  base::TimeTicks time_of_first_unsent_arrival_ GUARDED_BY(lock_);

  // Counted separately from `unsent_metrics_.size()` because each metric can
  // hold multiple samples.
  // `unsent_sample_count_ <= kMaxUnsentSamples`.
  size_t unsent_sample_count_ GUARDED_BY(lock_) = 0;

  mutable base::Lock lock_;
};

namespace internal {
// Accesses the global `AggregatingSampleCollector` instance. On non-test
// targets or test targets with no `ScopedSwitchSampleCollector`
// override this is also what's returned by
// `IdentifiabilitySampleCollector::Get()`.
BLINK_COMMON_EXPORT_PRIVATE AggregatingSampleCollector* GetCollectorInstance();
}  // namespace internal

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_COMMON_PRIVACY_BUDGET_AGGREGATING_SAMPLE_COLLECTOR_H_
