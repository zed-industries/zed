// Copyright 2020 The Chromium Authors
// Copyright 2014 Blake Embrey (hello@blakeembrey.com)
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_PATTERN_H_
#define THIRD_PARTY_LIBURLPATTERN_PATTERN_H_

#include <optional>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

#include "base/component_export.h"
#include "third_party/liburlpattern/options.h"

namespace liburlpattern {

// Numeric values are set such that more restrictive values come last.  This
// is important for comparison routines in calling code, like URLPattern.
enum class PartType {
  // A part that matches any character to the end of the input string.
  kFullWildcard = 0,

  // A part that matches any character to the next segment separator.
  kSegmentWildcard = 1,

  // A part with a custom regular expression.
  kRegex = 2,

  // A fixed, non-variable part of the pattern.  Consists of kChar and
  // kEscapedChar Tokens.
  kFixed = 3,
};

// Numeric values are set such that more restrictive values come last.  This
// is important for comparison routines in calling code, like URLPattern.
enum class Modifier {
  // The `*` modifier.
  kZeroOrMore = 0,

  // The `?` modifier.
  kOptional = 1,

  // The `+` modifier.
  kOneOrMore = 2,

  // No modifier.
  kNone = 3,
};

// A structure representing one part of a parsed Pattern.  A full Pattern
// consists of an ordered sequence of Part objects.
struct COMPONENT_EXPORT(LIBURLPATTERN) Part {
  // The type of the Part.
  PartType type = PartType::kFixed;

  // The name of the Part.  Only kRegex, kSegmentWildcard, and kFullWildcard
  // parts may have a |name|.  kFixed parts must have an empty |name|.
  std::string name;

  // A fixed string prefix that is expected before any regex or wildcard match.
  // kFixed parts must have an empty |prefix|.
  std::string prefix;

  // The meaning of the |value| depends on the |type| of the Part.  For kFixed
  // parts the |value| contains the fixed string to match.  For kRegex parts
  // the |value| contains a regular expression to match.  The |value| is empty
  // for kSegmentWildcard and kFullWildcard parts since the |type| encodes what
  // to match.
  std::string value;

  // A fixed string prefix that is expected after any regex or wildcard match.
  // kFixed parts must have an empty |suffix|.
  std::string suffix;

  // A |modifier| indicating whether the Part is optional and/or repeated.  Any
  // Part type may have a |modifier|.
  Modifier modifier = Modifier::kNone;

  Part(PartType type, std::string value, Modifier modifier);
  Part(PartType type,
       std::string name,
       std::string prefix,
       std::string value,
       std::string suffix,
       Modifier modifier);
  Part() = default;

  // Returns true if the `name` member is a custom name; e.g. for a `:foo`
  // group.
  bool HasCustomName() const;
};

COMPONENT_EXPORT(LIBURLPATTERN)
inline bool operator==(const Part& lh, const Part& rh) {
  return lh.name == rh.name && lh.prefix == rh.prefix && lh.value == rh.value &&
         lh.suffix == rh.suffix && lh.modifier == rh.modifier;
}

inline bool operator!=(const Part& lh, const Part& rh) {
  return !(lh == rh);
}

COMPONENT_EXPORT(LIBURLPATTERN)
std::ostream& operator<<(std::ostream& o, Part part);

// This class represents a successfully parsed pattern string.  It will contain
// an intermediate representation that can be used to generate either a regular
// expression string or to directly match against input strings.  Not all
// patterns are supported for direct matching.
class COMPONENT_EXPORT(LIBURLPATTERN) Pattern {
 public:
  Pattern(std::vector<Part> part_list,
          Options options,
          std::string segment_wildcard_regex);

  // Generate a canonical string for the parsed pattern.  This may result
  // in a value different from the pattern string originally passed to
  // Parse().  For example, no-op syntax like `{bar}` will be simplified to
  // `bar`.  In addition, the generated string will include any changes mad
  // by EncodingCallback hooks.  Finally, regular expressions equivalent to
  // `*` and named group default matching will be simplified; e.g. `(.*)`
  // will become just `*`.
  std::string GeneratePatternString() const;

  // Generate an ECMA-262 regular expression string that is equivalent to this
  // pattern.  A vector of strings can be optionally passed to |name_list_out|
  // to be populated with the list of group names.  These correspond
  // sequentially to the regular expression capture groups.  Note, the
  // regular expression string does not currently used named capture groups
  // directly in order to match the upstream path-to-regexp behavior.
  std::string GenerateRegexString(
      std::vector<std::string>* name_list_out = nullptr) const;

  const std::vector<Part>& PartList() const { return part_list_; }

  // Returns true if the pattern has at least one kRegex part.
  bool HasRegexGroups() const;

  // Returns true if the pattern can match input strings using `DirectMatch()`.
  bool CanDirectMatch() const;

  // Attempts to match the pattern against the given `input` string directly
  // without using a regular expression.  Only some patterns support this
  // feature.  The caller must only call `DirectMatch()` if `CanDirectMatch()`
  // returns true.
  //
  // `DirectMatch()` returns true if the pattern matches `input` and false
  // otherwise.  If the `group_list_out` pointer is provided then the vector
  // is populated with name:value pairs for matched pattern groups.  If a
  // group had an optional modifier and it did not match any input characters
  // then its `group_list_out` value will be std::nullopt.
  bool DirectMatch(std::string_view input,
                   std::vector<std::pair<std::string_view,
                                         std::optional<std::string_view>>>*
                       group_list_out) const;

 private:
  // Compute the expected size of the string that will be returned by
  // GenerateRegexString().
  size_t RegexStringLength() const;

  // Utility method to help with generating the regex string and length.
  void AppendDelimiterList(std::string& append_target) const;
  size_t DelimiterListLength() const;
  void AppendEndsWith(std::string& append_target) const;
  size_t EndsWithLength() const;

  // Utility methods to help with direct matching.
  bool IsOnlyFullWildcard() const;
  bool IsOnlyFixedText() const;

  std::vector<Part> part_list_;
  Options options_;
  std::string segment_wildcard_regex_;
};

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_PATTERN_H_
