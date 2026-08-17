// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRING_UTIL_IMPL_HELPERS_H_
#define BASE_STRINGS_STRING_UTIL_IMPL_HELPERS_H_

#include <algorithm>
#include <array>
#include <numeric>
#include <optional>
#include <string_view>
#include <type_traits>

#include "base/check.h"
#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/logging.h"
#include "base/notreached.h"
#include "base/third_party/icu/icu_utf.h"

namespace base::internal {

// Used by ReplaceStringPlaceholders to track the position in the string of
// replaced parameters.
struct ReplacementOffset {
  ReplacementOffset(uintptr_t parameter, size_t offset)
      : parameter(parameter), offset(offset) {}

  // Index of the parameter.
  size_t parameter;

  // Starting position in the string.
  size_t offset;
};

static bool CompareParameter(const ReplacementOffset& elem1,
                             const ReplacementOffset& elem2) {
  return elem1.parameter < elem2.parameter;
}

// Assuming that a pointer is the size of a "machine word", then
// uintptr_t is an integer type that is also a machine word.
using MachineWord = uintptr_t;

inline bool IsMachineWordAligned(const void* pointer) {
  return !(reinterpret_cast<MachineWord>(pointer) & (sizeof(MachineWord) - 1));
}

template <typename T, typename CharT = typename T::value_type>
std::basic_string<CharT> ToLowerASCIIImpl(T str) {
  std::basic_string<CharT> ret;
  ret.reserve(str.size());
  for (size_t i = 0; i < str.size(); i++) {
    ret.push_back(ToLowerASCII(str[i]));
  }
  return ret;
}

template <typename T, typename CharT = typename T::value_type>
std::basic_string<CharT> ToUpperASCIIImpl(T str) {
  std::basic_string<CharT> ret;
  ret.reserve(str.size());
  for (size_t i = 0; i < str.size(); i++) {
    ret.push_back(ToUpperASCII(str[i]));
  }
  return ret;
}

template <typename T, typename CharT = typename T::value_type>
TrimPositions TrimStringT(T input,
                          T trim_chars,
                          TrimPositions positions,
                          std::basic_string<CharT>* output) {
  // Find the edges of leading/trailing whitespace as desired. Need to use
  // a std::string_view version of input to be able to call find* on it with the
  // std::string_view version of trim_chars (normally the trim_chars will be a
  // constant so avoid making a copy).
  const size_t last_char = input.length() - 1;
  const size_t first_good_char =
      (positions & TRIM_LEADING) ? input.find_first_not_of(trim_chars) : 0;
  const size_t last_good_char = (positions & TRIM_TRAILING)
                                    ? input.find_last_not_of(trim_chars)
                                    : last_char;

  // When the string was all trimmed, report that we stripped off characters
  // from whichever position the caller was interested in. For empty input, we
  // stripped no characters, but we still need to clear |output|.
  if (input.empty() || first_good_char == std::basic_string<CharT>::npos ||
      last_good_char == std::basic_string<CharT>::npos) {
    bool input_was_empty = input.empty();  // in case output == &input
    output->clear();
    return input_was_empty ? TRIM_NONE : positions;
  }

  // Trim.
  output->assign(UNSAFE_TODO(input.data() + first_good_char),
                 last_good_char - first_good_char + 1);

  // Return where we trimmed from.
  return static_cast<TrimPositions>(
      (first_good_char == 0 ? TRIM_NONE : TRIM_LEADING) |
      (last_good_char == last_char ? TRIM_NONE : TRIM_TRAILING));
}

template <typename T, typename CharT = typename T::value_type>
T TrimStringPieceT(T input, T trim_chars, TrimPositions positions) {
  size_t begin =
      (positions & TRIM_LEADING) ? input.find_first_not_of(trim_chars) : 0;
  size_t end = (positions & TRIM_TRAILING)
                   ? input.find_last_not_of(trim_chars) + 1
                   : input.size();
  return input.substr(std::min(begin, input.size()), end - begin);
}

template <typename T, typename CharT = typename T::value_type>
std::basic_string<CharT> CollapseWhitespaceT(
    T text,
    bool trim_sequences_with_line_breaks) {
  std::basic_string<CharT> result;
  result.resize(text.size());

  // Set flags to pretend we're already in a trimmed whitespace sequence, so we
  // will trim any leading whitespace.
  bool in_whitespace = true;
  bool already_trimmed = true;

  size_t chars_written = 0;
  for (auto c : text) {
    if (IsWhitespace(c)) {
      if (!in_whitespace) {
        // Reduce all whitespace sequences to a single space.
        in_whitespace = true;
        result[chars_written++] = L' ';
      }
      if (trim_sequences_with_line_breaks && !already_trimmed &&
          ((c == '\n') || (c == '\r'))) {
        // Whitespace sequences containing CR or LF are eliminated entirely.
        already_trimmed = true;
        --chars_written;
      }
    } else {
      // Non-whitespace characters are copied straight across.
      in_whitespace = false;
      already_trimmed = false;
      result[chars_written++] = c;
    }
  }

  if (in_whitespace && !already_trimmed) {
    // Any trailing whitespace is eliminated.
    --chars_written;
  }

  result.resize(chars_written);
  return result;
}

template <class Char>
bool DoIsStringASCII(const Char* characters, size_t length) {
  // Bitmasks to detect non ASCII characters for character sizes of 8, 16 and 32
  // bits.
  constexpr auto NonASCIIMasks = std::to_array<MachineWord>({
      0,
      MachineWord(0x8080808080808080ULL),
      MachineWord(0xFF80FF80FF80FF80ULL),
      0,
      MachineWord(0xFFFFFF80FFFFFF80ULL),
  });

  if (!length) {
    return true;
  }
  constexpr MachineWord non_ascii_bit_mask = NonASCIIMasks[sizeof(Char)];
  static_assert(non_ascii_bit_mask, "Error: Invalid Mask");
  MachineWord all_char_bits = 0;
  const Char* end = UNSAFE_TODO(characters + length);

  // Prologue: align the input.
  while (!IsMachineWordAligned(characters) && characters < end) {
    all_char_bits |= UNSAFE_TODO(static_cast<MachineWord>(*characters++));
  }
  if (all_char_bits & non_ascii_bit_mask) {
    return false;
  }

  // Compare the values of CPU word size.
  constexpr size_t chars_per_word = sizeof(MachineWord) / sizeof(Char);
  constexpr int batch_count = 16;
  while (characters <= UNSAFE_TODO(end - batch_count * chars_per_word)) {
    all_char_bits = 0;
    for (int i = 0; i < batch_count; ++i) {
      all_char_bits |= *(reinterpret_cast<const MachineWord*>(characters));
      UNSAFE_TODO(characters += chars_per_word);
    }
    if (all_char_bits & non_ascii_bit_mask) {
      return false;
    }
  }

  // Process the remaining words.
  all_char_bits = 0;
  while (characters <= UNSAFE_TODO(end - chars_per_word)) {
    all_char_bits |= *(reinterpret_cast<const MachineWord*>(characters));
    UNSAFE_TODO(characters += chars_per_word);
  }

  // Process the remaining bytes.
  while (characters < end) {
    all_char_bits |= UNSAFE_TODO(static_cast<MachineWord>(*characters++));
  }

  return !(all_char_bits & non_ascii_bit_mask);
}

template <bool (*Validator)(base_icu::UChar32)>
inline bool DoIsStringUTF8(std::string_view str) {
  const uint8_t* src = reinterpret_cast<const uint8_t*>(str.data());
  size_t src_len = str.length();
  size_t char_index = 0;

  while (char_index < src_len) {
    base_icu::UChar32 code_point;
    UNSAFE_TODO(CBU8_NEXT(src, char_index, src_len, code_point));
    if (!Validator(code_point)) {
      return false;
    }
  }
  return true;
}

// Lookup table for fast ASCII case-insensitive comparison.
inline constexpr std::array<unsigned char, 256> kToLower = []() {
  std::array<unsigned char, 256> table;
  std::iota(table.begin(), table.end(), 0);
  std::iota(table.begin() + size_t{'A'}, table.begin() + size_t{'Z'} + 1, 'a');
  return table;
}();

inline constexpr auto lower = [](auto c) constexpr {
  return kToLower[static_cast<unsigned char>(c)];
};

template <typename T, typename CharT = typename T::value_type>
constexpr bool StartsWithT(T str, T search_for, CompareCase case_sensitivity) {
  return case_sensitivity == CompareCase::SENSITIVE
             ? str.starts_with(search_for)
             : std::ranges::equal(str.substr(0, search_for.size()), search_for,
                                  {}, lower, lower);
}

template <typename T, typename CharT = typename T::value_type>
constexpr bool EndsWithT(T str, T search_for, CompareCase case_sensitivity) {
  return case_sensitivity == CompareCase::SENSITIVE
             ? str.ends_with(search_for)
             : (search_for.size() <= str.size() &&
                std::ranges::equal(str.substr(str.size() - search_for.size()),
                                   search_for, {}, lower, lower));
}

// A Matcher for DoReplaceMatchesAfterOffset() that matches substrings.
template <class CharT>
struct SubstringMatcher {
  std::basic_string_view<CharT> find_this;

  size_t Find(const std::basic_string<CharT>& input, size_t pos) {
    return input.find(find_this.data(), pos, find_this.length());
  }
  size_t MatchSize() { return find_this.length(); }
};

// Type deduction helper for SubstringMatcher.
template <typename T, typename CharT = typename T::value_type>
auto MakeSubstringMatcher(T find_this) {
  return SubstringMatcher<CharT>{find_this};
}

// A Matcher for DoReplaceMatchesAfterOffset() that matches single characters.
template <class CharT>
struct CharacterMatcher {
  std::basic_string_view<CharT> find_any_of_these;

  size_t Find(const std::basic_string<CharT>& input, size_t pos) {
    return input.find_first_of(find_any_of_these.data(), pos,
                               find_any_of_these.length());
  }
  constexpr size_t MatchSize() { return 1; }
};

// Type deduction helper for CharacterMatcher.
template <typename T, typename CharT = typename T::value_type>
auto MakeCharacterMatcher(T find_any_of_these) {
  return CharacterMatcher<CharT>{find_any_of_these};
}

enum class ReplaceType { REPLACE_ALL, REPLACE_FIRST };

// Runs in O(n) time in the length of |str|, and transforms the string without
// reallocating when possible. Returns |true| if any matches were found.
//
// This is parameterized on a |Matcher| traits type, so that it can be the
// implementation for both ReplaceChars() and ReplaceSubstringsAfterOffset().
template <typename Matcher, typename T, typename CharT = typename T::value_type>
bool DoReplaceMatchesAfterOffset(std::basic_string<CharT>* str,
                                 size_t initial_offset,
                                 Matcher matcher,
                                 T replace_with,
                                 ReplaceType replace_type) {
  using CharTraits = std::char_traits<CharT>;

  const size_t find_length = matcher.MatchSize();
  if (!find_length) {
    return false;
  }

  // If the find string doesn't appear, there's nothing to do.
  size_t first_match = matcher.Find(*str, initial_offset);
  if (first_match == std::basic_string<CharT>::npos) {
    return false;
  }

  // If we're only replacing one instance, there's no need to do anything
  // complicated.
  const size_t replace_length = replace_with.length();
  if (replace_type == ReplaceType::REPLACE_FIRST) {
    str->replace(first_match, find_length, replace_with.data(), replace_length);
    return true;
  }

  // If the find and replace strings are the same length, we can simply use
  // replace() on each instance, and finish the entire operation in O(n) time.
  if (find_length == replace_length) {
    auto* buffer = &((*str)[0]);
    for (size_t offset = first_match; offset != std::basic_string<CharT>::npos;
         offset = matcher.Find(*str, offset + replace_length)) {
      CharTraits::copy(UNSAFE_TODO(buffer + offset), replace_with.data(),
                       replace_length);
    }
    return true;
  }

  // Since the find and replace strings aren't the same length, a loop like the
  // one above would be O(n^2) in the worst case, as replace() will shift the
  // entire remaining string each time. We need to be more clever to keep things
  // O(n).
  //
  // When the string is being shortened, it's possible to just shift the matches
  // down in one pass while finding, and truncate the length at the end of the
  // search.
  //
  // If the string is being lengthened, more work is required. The strategy used
  // here is to make two find() passes through the string. The first pass counts
  // the number of matches to determine the new size. The second pass will
  // either construct the new string into a new buffer (if the existing buffer
  // lacked capacity), or else -- if there is room -- create a region of scratch
  // space after |first_match| by shifting the tail of the string to a higher
  // index, and doing in-place moves from the tail to lower indices thereafter.
  size_t str_length = str->length();
  size_t expansion = 0;
  if (replace_length > find_length) {
    // This operation lengthens the string; determine the new length by counting
    // matches.
    const size_t expansion_per_match = (replace_length - find_length);
    size_t num_matches = 0;
    for (size_t match = first_match; match != std::basic_string<CharT>::npos;
         match = matcher.Find(*str, match + find_length)) {
      expansion += expansion_per_match;
      ++num_matches;
    }
    const size_t final_length = str_length + expansion;

    if (str->capacity() < final_length) {
      // If we'd have to allocate a new buffer to grow the string, build the
      // result directly into the new allocation via append().
      std::basic_string<CharT> src(str->get_allocator());
      str->swap(src);
      str->reserve(final_length);

      size_t pos = 0;
      for (size_t match = first_match;; match = matcher.Find(src, pos)) {
        str->append(src, pos, match - pos);
        str->append(replace_with.data(), replace_length);
        pos = match + find_length;

        // A mid-loop test/break enables skipping the final Find() call; the
        // number of matches is known, so don't search past the last one.
        if (!--num_matches) {
          break;
        }
      }

      // Handle substring after the final match.
      str->append(src, pos, str_length - pos);
      return true;
    }

    // Prepare for the copy/move loop below -- expand the string to its final
    // size by shifting the data after the first match to the end of the resized
    // string.
    size_t shift_src = first_match + find_length;
    size_t shift_dst = shift_src + expansion;

    // Big |expansion| factors (relative to |str_length|) require padding up to
    // |shift_dst|.
    if (shift_dst > str_length) {
      str->resize(shift_dst);
    }

    str->replace(shift_dst, str_length - shift_src, *str, shift_src,
                 str_length - shift_src);
    str_length = final_length;
  }

  // We can alternate replacement and move operations. This won't overwrite the
  // unsearched region of the string so long as |write_offset| <= |read_offset|;
  // that condition is always satisfied because:
  //
  //   (a) If the string is being shortened, |expansion| is zero and
  //       |write_offset| grows slower than |read_offset|.
  //
  //   (b) If the string is being lengthened, |write_offset| grows faster than
  //       |read_offset|, but |expansion| is big enough so that |write_offset|
  //       will only catch up to |read_offset| at the point of the last match.
  auto* buffer = &((*str)[0]);
  size_t write_offset = first_match;
  size_t read_offset = first_match + expansion;
  do {
    if (replace_length) {
      CharTraits::copy(UNSAFE_TODO(buffer + write_offset), replace_with.data(),
                       replace_length);
      write_offset += replace_length;
    }
    read_offset += find_length;

    // min() clamps std::basic_string<CharT>::npos (the largest unsigned value)
    // to str_length.
    size_t match = std::min(matcher.Find(*str, read_offset), str_length);

    size_t length = match - read_offset;
    if (length) {
      CharTraits::move(UNSAFE_TODO(buffer + write_offset),
                       UNSAFE_TODO(buffer + read_offset), length);
      write_offset += length;
      read_offset += length;
    }
  } while (read_offset < str_length);

  // If we're shortening the string, truncate it now.
  str->resize(write_offset);
  return true;
}

template <typename T, typename CharT = typename T::value_type>
bool ReplaceCharsT(T input,
                   T find_any_of_these,
                   T replace_with,
                   std::basic_string<CharT>* output) {
  // Commonly, this is called with output and input being the same string; in
  // that case, skip the copy.
  if (input.data() != output->data() || input.size() != output->size()) {
    output->assign(input.data(), input.size());
  }

  return DoReplaceMatchesAfterOffset(output, 0,
                                     MakeCharacterMatcher(find_any_of_these),
                                     replace_with, ReplaceType::REPLACE_ALL);
}

template <class string_type>
inline typename string_type::value_type* WriteIntoT(string_type* str,
                                                    size_t length_with_null) {
  DCHECK_GE(length_with_null, 1u);
  str->reserve(length_with_null);
  str->resize(length_with_null - 1);
  return str->data();
}

// Generic version for all JoinString overloads. |list_type| must be a sequence
// (base::span or std::initializer_list) of strings/StringPieces (std::string,
// std::u16string, std::string_view or std::u16string_view). |CharT| is either
// char or char16_t.
template <typename list_type,
          typename T,
          typename CharT = typename T::value_type>
static std::basic_string<CharT> JoinStringT(list_type parts, T sep) {
  if (std::empty(parts)) {
    return std::basic_string<CharT>();
  }

  // Pre-allocate the eventual size of the string. Start with the size of all of
  // the separators (note that this *assumes* parts.size() > 0).
  size_t total_size = (parts.size() - 1) * sep.size();
  for (const auto& part : parts) {
    total_size += part.size();
  }
  std::basic_string<CharT> result;
  result.reserve(total_size);

  auto iter = parts.begin();
  CHECK(iter != parts.end(), base::NotFatalUntil::M125);
  result.append(*iter);
  UNSAFE_TODO(++iter);

  for (; iter != parts.end(); UNSAFE_TODO(++iter)) {
    result.append(sep);
    result.append(*iter);
  }

  // Sanity-check that we pre-allocated correctly.
  DCHECK_EQ(total_size, result.size());

  return result;
}

// StringViewLike will match both std::basic_string_view<Char> and
// std::basic_string<Char>. It ensures that the type satisfies the requirements
// to be passed to std::basic_string::append().
template <typename T, typename CharT, typename TBaseT = std::remove_cvref_t<T>>
concept StringOrStringView =
    std::same_as<TBaseT, std::basic_string<CharT>> ||
    std::same_as<TBaseT, std::basic_string_view<CharT>>;

// Replaces placeholders in `format_string` with values from `subst`.
// * `placeholder_prefix`: Allows using a specific character as the placeholder
// prefix. `base::ReplaceStringPlaceholders` uses '$'.
// * `should_escape_multiple_placeholder_prefixes`:
//   * If this parameter is `true`, which is the case with
//   `base::ReplaceStringPlaceholders`, `placeholder_prefix` characters are
//   replaced by that number less one. Eg $$->$, $$$->$$, etc.
//   * If this parameter is `false`, each literal `placeholder_prefix` character
//   in `format_string` is escaped with another `placeholder_prefix`. For
//   instance, with `%` as the `placeholder_prefix`: %%->%, %%%%->%%, etc.
// * `is_strict_mode`:
//   * If this parameter is `true`, error handling is stricter. The function
//   returns `std::nullopt` if:
//     * a placeholder %N is encountered where N > substitutions.size().
//     * a literal `%` is not escaped with a `%`.
template <typename T, typename Range, typename CharT = typename T::value_type>
  requires(std::ranges::random_access_range<Range> &&
           std::ranges::sized_range<Range> &&
           requires(Range&& range, size_t index) {
             { range.at(index) } -> StringOrStringView<CharT>;
           })
std::optional<std::basic_string<CharT>> DoReplaceStringPlaceholders(
    T format_string,
    Range&& subst,
    const CharT placeholder_prefix,
    const bool should_escape_multiple_placeholder_prefixes,
    const bool is_strict_mode,
    std::vector<size_t>* offsets) {
  const size_t substitutions = subst.size();
  DCHECK_LT(substitutions, 10U);

  size_t sub_length = 0;
  for (const auto& cur : subst) {
    sub_length += cur.size();
  }

  std::basic_string<CharT> formatted;
  formatted.reserve(format_string.length() + sub_length);

  std::vector<ReplacementOffset> r_offsets;
  if (offsets) {
    r_offsets.reserve(substitutions);
  }
  for (auto i = format_string.begin(); i != format_string.end(); ++i) {
    if (placeholder_prefix == *i) {
      if (i + 1 != format_string.end()) {
        ++i;
        if (placeholder_prefix == *i) {
          do {
            formatted.push_back(placeholder_prefix);
            ++i;
          } while (should_escape_multiple_placeholder_prefixes &&
                   i != format_string.end() && placeholder_prefix == *i);
          --i;
        } else {
          if (*i < '1' || *i > '9') {
            if (is_strict_mode) {
              DLOG(ERROR) << "Invalid placeholder after placeholder prefix: "
                          << std::basic_string<CharT>(1, placeholder_prefix)
                          << std::basic_string<CharT>(1, *i);
              return std::nullopt;
            }

            continue;
          }
          const size_t index = static_cast<size_t>(*i - '1');
          if (offsets) {
            ReplacementOffset r_offset(index, formatted.size());
            r_offsets.insert(std::ranges::upper_bound(r_offsets, r_offset,
                                                      &CompareParameter),
                             r_offset);
          }
          if (index < substitutions) {
            formatted.append(subst.at(index));
          } else if (is_strict_mode) {
            DLOG(ERROR) << "index out of range: " << index << ": "
                        << substitutions;
            return std::nullopt;
          }
        }
      } else if (is_strict_mode) {
        DLOG(ERROR) << "unexpected placeholder prefix at end of string";
        return std::nullopt;
      }
    } else {
      formatted.push_back(*i);
    }
  }
  if (offsets) {
    for (const auto& cur : r_offsets) {
      offsets->push_back(cur.offset);
    }
  }
  return formatted;
}

// The following code is compatible with the OpenBSD lcpy interface.  See:
//   http://www.gratisoft.us/todd/papers/strlcpy.html
//   ftp://ftp.openbsd.org/pub/OpenBSD/src/lib/libc/string/{wcs,str}lcpy.c

template <typename CHAR>
size_t lcpyT(span<CHAR> dst, std::basic_string_view<CHAR> src) {
  size_t i = 0;

  const size_t dst_size = dst.size();
  for (; i + 1u < dst_size; ++i) {
    if (i == src.size()) {
      break;
    }
    dst[i] = src[i];
  }

  // Write the terminating NUL.
  if (!dst.empty()) {
    dst[i] = 0;
  }

  return src.size();
}

}  // namespace base::internal

#endif  // BASE_STRINGS_STRING_UTIL_IMPL_HELPERS_H_
