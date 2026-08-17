/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_EXT_BASE_STRING_VIEW_SPLITTER_H_
#define INCLUDE_PERFETTO_EXT_BASE_STRING_VIEW_SPLITTER_H_

#include "perfetto/ext/base/string_view.h"

namespace perfetto {
namespace base {

// C++ version of strtok(). Splits a StringView without making copies or any
// heap allocations. Supports the special case of using \0 as a delimiter.
// The token returned in output are valid as long as the input string is valid.
class StringViewSplitter {
 public:
  // Whether an empty string (two delimiters side-to-side) is a valid token.
  enum class EmptyTokenMode {
    DISALLOW_EMPTY_TOKENS,
    ALLOW_EMPTY_TOKENS,

    DEFAULT = DISALLOW_EMPTY_TOKENS,
  };

  // Can take ownership of the string if passed via std::move(), e.g.:
  // StringViewSplitter(std::move(str), '\n');
  StringViewSplitter(base::StringView,
                     char delimiter,
                     EmptyTokenMode empty_token_mode = EmptyTokenMode::DEFAULT);

  // Splits the current token from an outer StringViewSplitter instance. This is
  // to chain splitters as follows: for (base::StringViewSplitter lines(x,
  // '\n'); ss.Next();)
  //   for (base::StringViewSplitter words(&lines, ' '); words.Next();)
  StringViewSplitter(StringViewSplitter*,
                     char delimiter,
                     EmptyTokenMode empty_token_mode = EmptyTokenMode::DEFAULT);

  // Returns true if a token is found (in which case it will be stored in
  // cur_token()), false if no more tokens are found.
  bool Next();

  // Returns the next token if, found (in which case it will be stored in
  // cur_token()), and the empty string if no more tokens are found.
  base::StringView NextToken() { return Next() ? cur_token() : ""; }

  // Returns the current token iff last call to Next() returned true.
  // In all other cases (before the 1st call to Next() and after Next() returns
  // false) returns the empty string.
  base::StringView cur_token() { return cur_; }

  // Returns the remainder of the current input string that has not yet been
  // tokenized.
  base::StringView remainder() { return next_; }

 private:
  StringViewSplitter(const StringViewSplitter&) = delete;
  StringViewSplitter& operator=(const StringViewSplitter&) = delete;
  void Initialize(base::StringView);

  base::StringView str_;
  base::StringView cur_;
  base::StringView next_;
  bool end_of_input_;
  const char delimiter_;
  const EmptyTokenMode empty_token_mode_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_STRING_VIEW_SPLITTER_H_
