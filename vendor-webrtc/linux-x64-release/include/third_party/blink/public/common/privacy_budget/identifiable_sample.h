// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SAMPLE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SAMPLE_H_

#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"

namespace blink {

// Represents a single identifiable sample. It's basically an immutable
// 〈surface, value〉tuple defined as a struct because it's useful in many
// places.
struct IdentifiableSample {
  IdentifiableSample(IdentifiableSurface surface_param,
                     IdentifiableToken value_param)
      : surface(surface_param), value(value_param) {}

  const IdentifiableSurface surface;
  const IdentifiableToken value;

  bool operator==(const IdentifiableSample& other) const {
    return surface == other.surface && value == other.value;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_SAMPLE_H_
