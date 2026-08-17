/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_UTIL_GLOB_H_
#define SRC_TRACE_PROCESSOR_UTIL_GLOB_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "perfetto/ext/base/small_vector.h"
#include "perfetto/ext/base/string_view.h"

namespace perfetto::trace_processor::util {

// Lightweight implementation of matching on UNIX glob patterns, maintaining
// compatibility of syntax and semantics used by SQLite.
//
// Usage:
//  GlobMatcher matcher = GlobMatcher::FromPattern("*foo*");
//  for (auto string : strings) {
//    if (matcher.Matches(string)) {
//      <do something>
//    }
//  }
//
// This is a class instead of a free function to allow preprocessing the
// pattern (e.g. to compute Kleene star offsets). This can create big savings
// because trace processor needs to match the same pattern on many strings
// when filtering tables.
//
// Implementation:
// The algorithm used in this class is similar to the "alternative"
// algorithm proposed in [1].
//
// We preprocess the pattern (in the constructor) to split the pattern on *,
// accounting for character classes. This breaks the pattern in "segments": our
// name for the parts of the pattern between the stars.
//
// Then at match time, we go through each segment and check if it matches part
// of the string. The number of character matched defines the search start-point
// for the next segment. As described in [1], we don't need to do any
// backtracking which removes the exponential component of the algorithm and
// consequently simplifies the code.
//
// The subtle parts are:
// 1) the first and last segments - they need to be "anchored" to the
//    beginning and end of the string respectively. If not, they fail the match
//    straight away.
// 2) leading/trailing stars: they counteract the above point and "unanchor"
//    the first and last segments respectively by allowing them to happen
//    somewhere after/before the beginning/end.
//
// [1] https://research.swtch.com/glob
class GlobMatcher {
 public:
  GlobMatcher(GlobMatcher&&) = default;
  GlobMatcher& operator=(GlobMatcher&&) = default;

  // Creates a glob matcher from a pattern.
  static GlobMatcher FromPattern(base::StringView pattern_str) {
    return GlobMatcher(pattern_str);
  }

  // Checks the provided string against the pattern and returns whether it
  // matches.
  bool Matches(base::StringView input) const;

  // Returns whether the comparison should really be an equality comparison.
  bool IsEquality() const {
    return !leading_star_ && !trailing_star_ &&
           !contains_char_class_or_question_ && segments_.size() <= 1;
  }

 private:
  // Represents a portion of the pattern in between two * characters.
  struct Segment {
    // The portion of the pattern in the segment. Note that this will not
    // contain a free '*' (i.e. outside a character class).
    base::StringView pattern;

    // The number of consumed characters in an input string if this segment
    // matches.
    uint32_t matched_chars;
  };

  // It would be very rare for a glob pattern to have more than 4 stars so
  // reserve stack space for that many segments.
  static constexpr uint32_t kMaxSegmentsOnStack = 4;

  explicit GlobMatcher(base::StringView pattern);

  GlobMatcher(const GlobMatcher&) = delete;
  GlobMatcher& operator=(const GlobMatcher&) = delete;

  // Returns whether |input| starts with the pattern in |segment| following
  // glob matching rules.
  bool StartsWith(base::StringView input, const Segment& segment) const {
    if (!contains_char_class_or_question_) {
      return input.StartsWith(segment.pattern);
    }
    return StartsWithSlow(input, segment);
  }

  // Returns whether |input| ends with the pattern in |segment| following
  // glob matching rules.
  bool EndsWith(base::StringView input, const Segment& segment) const {
    if (!contains_char_class_or_question_) {
      return input.EndsWith(segment.pattern);
    }
    // Ending with |segment| is the same as taking the substring of |in|
    size_t start = input.size() - segment.matched_chars;
    return StartsWithSlow(input.substr(start), segment);
  }

  // Returns the index where |input| matches the pattern in |segment|
  // following glob matching rules or base::StringView::npos, if no such index
  // exists.
  size_t Find(base::StringView input,
              const Segment& segment,
              size_t start) const {
    if (!contains_char_class_or_question_) {
      return input.find(segment.pattern, start);
    }
    for (uint32_t i = 0; i < input.size(); ++i) {
      if (StartsWithSlow(input.substr(i), segment)) {
        return i;
      }
    }
    return base::StringView::npos;
  }

  // Given a StringView starting at the boundary of a character class, returns
  // a StringView containing only the parts inside the [] or base::StringView()
  // if no character class exists.
  static base::StringView ExtractCharacterClass(base::StringView input);

  // Matches |in| against the given character class.
  static bool MatchesCharacterClass(char input, base::StringView char_class);

  static bool StartsWithSlow(base::StringView input, const Segment& segment);

  // IMPORTANT: this should *not* be modified after the constructor as we store
  // pointers to the data inside here.
  // Note: this vector also allocates space for the null-terminator so is +1
  // the "traditional" size of the string.
  std::vector<char> pattern_;

  // Chunks of the |pattern_| tokenized on '*'. See the class comment for more
  // info.
  base::SmallVector<Segment, kMaxSegmentsOnStack> segments_;

  bool leading_star_ = false;
  bool trailing_star_ = false;
  bool contains_char_class_or_question_ = false;
};

}  // namespace perfetto::trace_processor::util

#endif  // SRC_TRACE_PROCESSOR_UTIL_GLOB_H_
