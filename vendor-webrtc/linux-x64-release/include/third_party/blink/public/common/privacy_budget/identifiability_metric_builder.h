// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRIC_BUILDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRIC_BUILDER_H_

#include <cstdint>
#include <vector>

#include "services/metrics/public/cpp/ukm_recorder.h"
#include "services/metrics/public/cpp/ukm_source_id.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_sample.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-forward.h"

namespace blink {

// IdentifiabilityMetricBuilder builds an identifiability metric encoded into a
// UkmEntry.
//
// This UkmEntry can be recorded via a UkmRecorder.
//
// # Encoding
//
// All identifiability metrics are represented using the tuple
//
//     < identifiable_surface_type, input, output >.
//
// A typical URL-Keyed-Metrics (UKM) entry looks like the following:
//
//     struct UkmEntry {
//       int64 source_id;
//       uint64 event_hash;
//       map<uint64,int64> metrics;
//     };
//
// (From //services/metrics/public/mojom/ukm_interface.mojom)
//
// This class encodes the former into the latter.
//
// The |source_id| is one that is known to UKM. Follow UKM guidelines for how to
// generate or determine the |source_id| corresponding to a document or URL.
//
// The |event_hash| is a digest of the UKM event name via
// |base::HashMetricName()|. For identifiability metrics, this is always
// UINT64_C(287024497009309687) which corresponds to 'Identifiability'.
//
// Metrics for *regular* UKM consist of a mapping from a metric ID to a metric
// value. The metric ID is a digest of the metric name as determined by
// base::IdentifiabilityDigestOfBytes(), similar to how an |event_hash| is
// derived from the event name.
//
// However, for identifiability metrics, the method for generating metric IDs
// is:
//
//     metrics_hash = (input << 8) | identifiable_surface_type;
//
// The |identifiable_surface_type| is an enumeration identifying the input
// identifier defined in |IdentifiableSurface::Type|.
//
// We lose the 8 MSBs of |input|. Retaining the lower bits allow us to use small
// (i.e. under 56-bits) numbers as-is without losing information.
//
// The |IdentifiableSurface| class encapsulates this translation.
//
// The |metrics| field in |UkmEntry| thus contains a mapping from the resulting
// |metric_hash| to the output of the identifiable surface encoded into 64-bits.
//
// To generate a 64-bit hash of a random binary blob, use
// |blink::IdentifiabilityDigestOfBytes()|. For numbers with fewer than 56
// significant bits, you can use the number itself as the input hash.
//
// As mentioned in |identifiability_metrics.h|, this function is **not** a
// cryptographic hash function. While it is expected to have a reasonable
// distribution for a uniform input, it should be assumed that finding
// collisions is trivial.
//
// E.g.:
//
// 1. A simple web exposed API that's represented using a |WebFeature|
//    constant. Values are defined in
//    blink/public/mojom/use_counter/metrics/web_feature.mojom.
//
//        identifiable_surface = IdentifiableSurface::FromTypeAndToken(
//            IdentifiableSurface::Type::kWebFeature,
//            blink::mojom::WebFeature::kDeviceOrientationSecureOrigin);
//        output = IdentifiabilityDigestOfBytes(result_as_binary_blob);
//
// 2. A surface that takes a non-trivial input represented as a binary blob:
//
//        identifiable_surface = IdentifiableSurface::FromTypeAndToken(
//            IdentifiableSurface::Type::kFancySurface,
//            IdentifiabilityDigestOfBytes(input_as_binary_blob));
//        output = IdentifiabilityDigestOfBytes(result_as_binary_blob);
class BLINK_COMMON_EXPORT IdentifiabilityMetricBuilder {
 public:
  // Construct a metrics builder for the given |source_id|. The source must be
  // known to UKM.
  explicit IdentifiabilityMetricBuilder(ukm::SourceIdObj source_id);

  // This has the same effect as the previous constructor but takes the
  // deprecated ukm::SourceId for convenience. Doing so allows callsites to use
  // methods like Document::GetUkmSourceId() without an extra conversion. In
  // addition, when Document::GetUkmSourceId() migrates to returning a
  // ukm::SourceIdObj, callsites won't need to change.
  explicit IdentifiabilityMetricBuilder(ukm::SourceId source_id)
      : IdentifiabilityMetricBuilder(ukm::SourceIdObj::FromInt64(source_id)) {}

  ~IdentifiabilityMetricBuilder();

  // Set the metric using a previously constructed |IdentifiableSurface|.
  IdentifiabilityMetricBuilder& Add(IdentifiableSurface surface,
                                    IdentifiableToken sample);

  // Convenience method for recording the result of invoking a simple API
  // surface with a |UseCounter|.
  IdentifiabilityMetricBuilder& AddWebFeature(mojom::WebFeature feature,
                                              IdentifiableToken sample) {
    return Add(IdentifiableSurface::FromTypeAndToken(
                   IdentifiableSurface::Type::kWebFeature, feature),
               sample);
  }

  // Record collected metrics to `recorder`.
  void Record(ukm::UkmRecorder* recorder);

 private:
  std::vector<IdentifiableSample> metrics_;
  const ukm::SourceIdObj source_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABILITY_METRIC_BUILDER_H_
