// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_STRING_SEARCH_H_
#define BASE_I18N_STRING_SEARCH_H_

#include <stddef.h>

#include <string>
#include <string_view>

#include "base/compiler_specific.h"
#include "base/i18n/base_i18n_export.h"
#include "base/memory/raw_ptr.h"

struct UStringSearch;

namespace base {
namespace i18n {

// Returns true if |in_this| contains |find_this|. If |match_index| or
// |match_length| are non-NULL, they are assigned the start position and total
// length of the match.
//
// Only differences between base letters are taken into consideration. Case and
// accent differences are ignored. Please refer to 'primary level' in
// http://userguide.icu-project.org/collation/concepts for additional details.
BASE_I18N_EXPORT
bool StringSearchIgnoringCaseAndAccents(std::u16string find_this,
                                        std::u16string_view in_this,
                                        size_t* match_index,
                                        size_t* match_length);

// Returns true if |in_this| contains |find_this|. If |match_index| or
// |match_length| are non-NULL, they are assigned the start position and total
// length of the match.
//
// When |case_sensitive| is false, only differences between base letters are
// taken into consideration. Case and accent differences are ignored.
// Please refer to 'primary level' in
// http://userguide.icu-project.org/collation/concepts for additional details.
// When |forward_search| is true, finds the first instance of |find_this|,
// otherwise finds the last instance
BASE_I18N_EXPORT
bool StringSearch(std::u16string find_this,
                  std::u16string_view in_this,
                  size_t* match_index,
                  size_t* match_length,
                  bool case_sensitive,
                  bool forward_search);

// This class is for speeding up multiple StringSearch()
// with the same |find_this| argument. |find_this| is passed as the constructor
// argument, and precomputation for searching is done only at that time.
class BASE_I18N_EXPORT GSL_POINTER FixedPatternStringSearch {
 public:
  explicit FixedPatternStringSearch(std::u16string find_this,
                                    bool case_sensitive);
  ~FixedPatternStringSearch();

  // Returns true if |in_this| contains |find_this|. If |match_index| or
  // |match_length| are non-NULL, they are assigned the start position and total
  // length of the match.
  bool Search(std::u16string_view in_this,
              size_t* match_index,
              size_t* match_length,
              bool forward_search);

 private:
  std::u16string find_this_;
  raw_ptr<UStringSearch> search_;
};

// This class is for speeding up multiple StringSearchIgnoringCaseAndAccents()
// with the same |find_this| argument. |find_this| is passed as the constructor
// argument, and precomputation for searching is done only at that time.
class BASE_I18N_EXPORT GSL_POINTER
    FixedPatternStringSearchIgnoringCaseAndAccents {
 public:
  explicit FixedPatternStringSearchIgnoringCaseAndAccents(
      std::u16string find_this);

  // Returns true if |in_this| contains |find_this|. If |match_index| or
  // |match_length| are non-NULL, they are assigned the start position and total
  // length of the match.
  bool Search(std::u16string_view in_this,
              size_t* match_index,
              size_t* match_length);

 private:
  FixedPatternStringSearch base_search_;
};

// This class is for performing all matches of `find_this` in `in_this`.
// `find_this` and `in_this` are passed as arguments in constructor.
class BASE_I18N_EXPORT GSL_POINTER RepeatingStringSearch {
 public:
  RepeatingStringSearch(std::u16string find_this,
                        std::u16string in_this,
                        bool case_sensitive);
  ~RepeatingStringSearch();

  // Returns true if the next match exists. `match_index` and `match_length` are
  // assigned the start position and total length of the match.
  bool NextMatchResult(int& match_index, int& match_length);

 private:
  std::u16string find_this_;
  std::u16string in_this_;
  raw_ptr<UStringSearch> search_;
};

}  // namespace i18n
}  // namespace base

#endif  // BASE_I18N_STRING_SEARCH_H_
