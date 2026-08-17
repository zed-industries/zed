// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_IDENTIFIABILITY_METRICS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_IDENTIFIABILITY_METRICS_H_

#include <optional>

#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"

namespace blink {

class ExecutionContext;
class MediaCapabilitiesDecodingInfo;
class MediaDecodingConfiguration;

// Defines methods used to emit UKM events for the identifiability study to
// determine whether MediaCapabilities::decodingInfo() calls can be used for
// fingerprinting users and, if so, how much entropy is exposed by the API. Only
// emits these events when the study is active.
namespace media_capabilities_identifiability_metrics {

// Reports that a call to decodingInfo() occurred that had the given |input|,
// resulted in |output| and was performed on the |context|.
void ReportDecodingInfoResult(ExecutionContext*,
                              const MediaDecodingConfiguration*,
                              const MediaCapabilitiesDecodingInfo*);

// Reports that a call to decodingInfo() occurred that had an input with a
// digest of |input_token.value()|, resulted in |output| and was performed on
// the |context|. However, |input_token| should be std::nullopt if the study
// is not active. These calls should be used when the input object may have
// been destroyed by the time the output is determined. See
// ComputeDecodingInfoInputToken below.
void ReportDecodingInfoResult(ExecutionContext*,
                              std::optional<IdentifiableToken>,
                              const MediaCapabilitiesDecodingInfo*);

// Returns a digest of the |input| for use in ReportDecodingInfoResult()
// above. Returns std::nullopt if the identifiability study is not active.
std::optional<IdentifiableToken> ComputeDecodingInfoInputToken(
    const MediaDecodingConfiguration*);

}  // namespace media_capabilities_identifiability_metrics
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_CAPABILITIES_MEDIA_CAPABILITIES_IDENTIFIABILITY_METRICS_H_
