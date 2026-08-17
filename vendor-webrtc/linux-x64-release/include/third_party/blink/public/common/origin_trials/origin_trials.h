// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// The implementation for this header file is auto-generated.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_H_

#include <string_view>

#include "base/containers/span.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/origin_trials/origin_trial_feature.mojom-forward.h"

namespace blink {

enum class OriginTrialType { kDefault = 0, kDeprecation, kIntervention };

// A namespace with dynamic tests for experimental features which can be
// enabled by the origin trials framework via origin trial tokens.
namespace origin_trials {

// Return true if there is a feature with the passed |trial_name|.
BLINK_COMMON_EXPORT bool IsTrialValid(std::string_view trial_name);

// Return true if |trial_name| can be enabled in an insecure context.
BLINK_COMMON_EXPORT bool IsTrialEnabledForInsecureContext(
    std::string_view trial_name);

// Return true if |trial_name| can be enabled from third party origins.
BLINK_COMMON_EXPORT bool IsTrialEnabledForThirdPartyOrigins(
    std::string_view trial_name);

// Return true if |trial_name| can be enabled for read access by the browser
// process.
BLINK_COMMON_EXPORT bool IsTrialEnabledForBrowserProcessReadAccess(
    std::string_view trial_name);

// Returns true if |trial_name| should be enabled until the next response
// from the same origin is received.
BLINK_COMMON_EXPORT bool IsTrialPersistentToNextResponse(
    std::string_view trial_name);

// Return true if |trial_name| is a deprecation trial.
BLINK_COMMON_EXPORT bool IsDeprecationTrial(std::string_view trial_name);

// Returns the trial type of the given |feature|.
BLINK_COMMON_EXPORT OriginTrialType
GetTrialType(blink::mojom::OriginTrialFeature feature);

// Return origin trials features that are enabled by the passed |trial_name|.
// The trial name MUST be valid (call IsTrialValid() before calling this
// function).
BLINK_COMMON_EXPORT base::span<const blink::mojom::OriginTrialFeature>
FeaturesForTrial(std::string_view trial_name);

// Return the list of features which will also be enabled if the given
// |feature| is enabled.
BLINK_COMMON_EXPORT base::span<const blink::mojom::OriginTrialFeature>
GetImpliedFeatures(blink::mojom::OriginTrialFeature feature);

// Returns true if |feature| is enabled on the current platform.
BLINK_COMMON_EXPORT bool FeatureEnabledForOS(
    blink::mojom::OriginTrialFeature feature);

// Returns true if |feature| can be enabled across navigations.
BLINK_COMMON_EXPORT bool FeatureEnabledForNavigation(
    blink::mojom::OriginTrialFeature feature);

// Returns true if |feature| has an expiry grace period.
BLINK_COMMON_EXPORT bool FeatureHasExpiryGracePeriod(
    blink::mojom::OriginTrialFeature feature);

}  // namespace origin_trials

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIALS_H_
