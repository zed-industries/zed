// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_ABSEIL_STRING_NUMBER_CONVERSIONS_H_
#define BASE_STRINGS_ABSEIL_STRING_NUMBER_CONVERSIONS_H_

#include <string_view>

#include "base/base_export.h"
#include "third_party/abseil-cpp/absl/numeric/int128.h"

namespace base {

// Best effort conversion, see `base::StringToInt()` for restrictions.
// Will only successfully parse values that will fit into `output`.
BASE_EXPORT bool StringToUint128(std::string_view input, absl::uint128* output);

// Best effort conversion, see `base::StringToInt()` for restrictions.
// Will only successfully parse hex values that will fit into `output`.
// The string is not required to start with 0x.
BASE_EXPORT bool HexStringToUInt128(std::string_view input,
                                    absl::uint128* output);

}  // namespace base

#endif  // BASE_STRINGS_ABSEIL_STRING_NUMBER_CONVERSIONS_H_
