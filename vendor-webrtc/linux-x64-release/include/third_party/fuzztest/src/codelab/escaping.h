// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_CODELAB_ESCAPING_H_
#define FUZZTEST_CODELAB_ESCAPING_H_

#include <string>
#include <string_view>

namespace codelab {

// Escapes `str`, by replacing special characters, such as '\n' and '\t',
// with their corresponding C escape sequences, i.e., "\\n" and "\\t".
std::string Escape(std::string_view str);

// Unescapes `str`, by replacing any occurrence of C escape sequences with their
// equivalent character. Invalid sequences are ignored.
std::string Unescape(std::string_view str);

}  // namespace codelab

#endif  // FUZZTEST_CODELAB_ESCAPING_H_
