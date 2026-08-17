// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_UTILS_H_

#include <string>
#include <string_view>
#include <tuple>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/interest_group/ad_display_size.h"

namespace blink {

BLINK_COMMON_EXPORT std::string ConvertAdSizeUnitToString(
    const blink::AdSize::LengthUnit& unit);

// Converts a valid ad size back to a string.
BLINK_COMMON_EXPORT std::string ConvertAdSizeToString(
    const blink::AdSize& ad_size);

// Converts a valid ad dimension back to a string.
BLINK_COMMON_EXPORT std::string ConvertAdDimensionToString(
    double value,
    AdSize::LengthUnit units);

// Helper function that converts a size string into its corresponding value and
// units. Accepts measurements in pixels (px), screen width (sw) and screen
// height (sh). Examples of allowed inputs:
// - "200.123px"
// - "200px"
// - "50sw"
// - "50sh"
// - " 25sw "
// - "100"
BLINK_COMMON_EXPORT std::tuple<double, blink::AdSize::LengthUnit>
ParseAdSizeString(std::string_view input);

BLINK_COMMON_EXPORT bool IsValidAdSize(const blink::AdSize& size);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_DISPLAY_SIZE_UTILS_H_
