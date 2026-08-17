// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/use_counter/use_counter_feature.mojom-shared.h"

namespace blink {

// `UseCounterFeature` is a union of enum feature types that UseCounter can
// count, e.g. mojom::WebFeature.
class BLINK_COMMON_EXPORT UseCounterFeature {
 public:
  using EnumValue = uint32_t;

  // The default constructor should only be called by mojom interface.
  UseCounterFeature() = default;

  UseCounterFeature(mojom::UseCounterFeatureType type, EnumValue value);

  // Getters.
  mojom::UseCounterFeatureType type() const { return type_; }
  EnumValue value() const { return value_; }

  // Used for mojom traits only. Returns whether the input params are valid.
  bool SetTypeAndValue(mojom::UseCounterFeatureType type, EnumValue value);

  bool operator==(const UseCounterFeature& rhs) const;
  bool operator<(const UseCounterFeature& rhs) const;

 private:
  // Bound check `value_` field based on `type_`;
  bool IsValid() const;

  mojom::UseCounterFeatureType type_;
  EnumValue value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_USE_COUNTER_USE_COUNTER_FEATURE_H_
