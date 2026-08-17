// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_TRACKER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_TRACKER_H_

#include <array>
#include <bitset>
#include <vector>

#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/use_counter/use_counter_feature.h"
#include "third_party/blink/public/mojom/use_counter/metrics/css_property_id.mojom-shared.h"
#include "third_party/blink/public/mojom/use_counter/metrics/web_feature.mojom-shared.h"
#include "third_party/blink/public/mojom/use_counter/metrics/webdx_feature.mojom-shared.h"

namespace blink {

class BLINK_COMMON_EXPORT UseCounterFeatureTracker {
 public:
  bool TestAndSet(const UseCounterFeature&);
  bool Test(const UseCounterFeature&) const;
  std::vector<UseCounterFeature> GetRecordedFeatures() const;

  void ResetForTesting(const UseCounterFeature&);

  // Returns whether all recorded features in `other` are also recorded
  // in `this`.
  bool ContainsForTesting(const UseCounterFeatureTracker& other) const;

 private:
  void Set(const UseCounterFeature&, bool);

  // Track what features have been recorded.
  std::bitset<static_cast<size_t>(mojom::WebFeature::kMaxValue) + 1>
      web_features_;
  std::bitset<static_cast<size_t>(mojom::WebDXFeature::kMaxValue) + 1>
      webdx_features_;
  std::bitset<static_cast<size_t>(mojom::CSSSampleId::kMaxValue) + 1>
      css_properties_;
  std::bitset<static_cast<size_t>(mojom::CSSSampleId::kMaxValue) + 1>
      animated_css_properties_;
  std::bitset<static_cast<size_t>(
                  network::mojom::PermissionsPolicyFeature::kMaxValue) +
              1>
      violated_permissions_policy_features_;
  std::bitset<static_cast<size_t>(
                  network::mojom::PermissionsPolicyFeature::kMaxValue) +
              1>
      iframe_permissions_policy_features_;
  std::bitset<static_cast<size_t>(
                  network::mojom::PermissionsPolicyFeature::kMaxValue) +
              1>
      header_permissions_policy_features_;
  std::bitset<static_cast<size_t>(
                  network::mojom::PermissionsPolicyFeature::kMaxValue) +
              1>
      private_permissions_policy_features_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_TRACKER_H_
