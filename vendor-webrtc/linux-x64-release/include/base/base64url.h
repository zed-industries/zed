// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BASE64URL_H_
#define BASE_BASE64URL_H_

#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace base {

enum class Base64UrlEncodePolicy {
  // Include the trailing padding in the output, when necessary.
  INCLUDE_PADDING,

  // Remove the trailing padding from the output.
  OMIT_PADDING
};

// Encodes the |input| binary data in base64url, defined in RFC 4648:
// https://tools.ietf.org/html/rfc4648#section-5
//
// The |policy| defines whether padding should be included or omitted from the
// encoded |*output|. |input| and |*output| may reference the same storage.
BASE_EXPORT void Base64UrlEncode(span<const uint8_t> input,
                                 Base64UrlEncodePolicy policy,
                                 std::string* output);

// Same as the previous function, but accepts an input string.
BASE_EXPORT void Base64UrlEncode(std::string_view input,
                                 Base64UrlEncodePolicy policy,
                                 std::string* output);

enum class Base64UrlDecodePolicy {
  // Require inputs contain trailing padding if non-aligned.
  REQUIRE_PADDING,

  // Accept inputs regardless of whether or not they have the correct padding.
  IGNORE_PADDING,

  // Reject inputs if they contain any trailing padding.
  DISALLOW_PADDING
};

// Decodes the |input| string in base64url, defined in RFC 4648:
// https://tools.ietf.org/html/rfc4648#section-5
//
// The |policy| defines whether padding will be required, ignored or disallowed
// altogether. |input| and |*output| may reference the same storage.
[[nodiscard]] BASE_EXPORT bool Base64UrlDecode(std::string_view input,
                                               Base64UrlDecodePolicy policy,
                                               std::string* output);

// Same as the previous function, but writing to a `std::vector`.
[[nodiscard]] BASE_EXPORT std::optional<std::vector<uint8_t>> Base64UrlDecode(
    std::string_view input,
    Base64UrlDecodePolicy policy);

}  // namespace base

#endif  // BASE_BASE64URL_H_
