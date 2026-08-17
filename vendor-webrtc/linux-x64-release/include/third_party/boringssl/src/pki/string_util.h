// Copyright 2022 The Chromium Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef BSSL_PKI_STRING_UTIL_H_
#define BSSL_PKI_STRING_UTIL_H_

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include <openssl/base.h>
#include <openssl/span.h>

BSSL_NAMESPACE_BEGIN
namespace string_util {

// Returns true if the characters in |str| are all ASCII, false otherwise.
OPENSSL_EXPORT bool IsAscii(std::string_view str);

// Compares |str1| and |str2| ASCII case insensitively (independent of locale).
// Returns true if |str1| and |str2| match.
OPENSSL_EXPORT bool IsEqualNoCase(std::string_view str1, std::string_view str2);

// Compares |str1| and |prefix| ASCII case insensitively (independent of
// locale). Returns true if |str1| starts with |prefix|.
OPENSSL_EXPORT bool StartsWithNoCase(std::string_view str,
                                     std::string_view prefix);

// Compares |str1| and |suffix| ASCII case insensitively (independent of
// locale). Returns true if |str1| starts with |suffix|.
OPENSSL_EXPORT bool EndsWithNoCase(std::string_view str,
                                   std::string_view suffix);

// Finds and replaces all occurrences of |find| of non zero length with
// |replace| in |str|, returning the result.
OPENSSL_EXPORT std::string FindAndReplace(std::string_view str,
                                          std::string_view find,
                                          std::string_view replace);

// TODO(bbe) transition below to c++20
// Compares |str1| and |prefix|. Returns true if |str1| starts with |prefix|.
OPENSSL_EXPORT bool StartsWith(std::string_view str, std::string_view prefix);

// TODO(bbe) transition below to c++20
// Compares |str1| and |suffix|. Returns true if |str1| ends with |suffix|.
OPENSSL_EXPORT bool EndsWith(std::string_view str, std::string_view suffix);

// Returns a hexadecimal string encoding |data|.
OPENSSL_EXPORT std::string HexEncode(Span<const uint8_t> data);

// Returns a decimal string representation of |i|.
OPENSSL_EXPORT std::string NumberToDecimalString(int i);

// Splits |str| on |split_char| returning the list of resulting strings.
OPENSSL_EXPORT std::vector<std::string_view> SplitString(std::string_view str,
                                                         char split_char);

// Collapess whitespace in |text| to a single space and returns the result.
OPENSSL_EXPORT std::string CollapseWhitespaceASCII(
    std::string_view text, bool trim_sequences_with_line_breaks);

// Base64 encodes |input| into |output| returning true on success,
// false otherwise.
OPENSSL_EXPORT bool Base64Encode(const std::string_view &input,
                                 std::string *output);

// Base64 decodes |input| into |output| returning true on success,
// false otherwise.
OPENSSL_EXPORT bool Base64Decode(const std::string_view &input,
                                 std::string *output);

}  // namespace string_util
BSSL_NAMESPACE_END

#endif  // BSSL_PKI_STRING_UTIL_H_
