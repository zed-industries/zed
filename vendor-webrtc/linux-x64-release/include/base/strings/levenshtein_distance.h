// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_LEVENSHTEIN_DISTANCE_H_
#define BASE_STRINGS_LEVENSHTEIN_DISTANCE_H_

#include <stddef.h>

#include <optional>
#include <string_view>

#include "base/base_export.h"

namespace base {

// Returns the Levenshtein distance of `a` and `b`. Edits, inserts and removes
// each count as one step.
// If `k = max_distance` is provided, the distance is only correctly calculated
// up to k. In case the actual Levenshtein distance is larger than k, k+1 is
// returned instead. This is useful for checking whether the distance is at most
// some small constant, since the algorithm is more efficient in this case.
// Complexity:
// - Without k: O(|a| * |b|) time and O(max(|a|, |b|)) memory.
// - With k: O(min(|a|, |b|) * k + k) time and O(k) memory.
BASE_EXPORT size_t
LevenshteinDistance(std::string_view a,
                    std::string_view b,
                    std::optional<size_t> max_distance = std::nullopt);
BASE_EXPORT size_t
LevenshteinDistance(std::u16string_view a,
                    std::u16string_view b,
                    std::optional<size_t> max_distance = std::nullopt);

}  // namespace base

#endif  // BASE_STRINGS_LEVENSHTEIN_DISTANCE_H_
