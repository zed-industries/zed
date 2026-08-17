// Copyright 2020 The Chromium Authors
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_PARSE_H_
#define THIRD_PARTY_LIBURLPATTERN_PARSE_H_

#include <functional>
#include <string_view>

#include "base/component_export.h"
#include "third_party/abseil-cpp/absl/status/statusor.h"
#include "third_party/liburlpattern/options.h"

namespace liburlpattern {

class Pattern;

// Define a functor-style callback that will be invoked synchronously by the
// Parse() method.  It will be called for each part of the pattern consisting
// of text to match strictly against an input.  For example, for the pattern:
//
//  `/foo/:bar.html`
//
// The callback will be invoked with `/foo`, `/`, and `.html` separately.
//
// The callback should validate the input and potentially perform any encoding
// necessary.  For example, some characters could be percent encoded.  The
// final encoded value for the input should be returned.
typedef std::function<absl::StatusOr<std::string>(std::string_view)>
    EncodeCallback;

// Parse a pattern string and return the result.  The parse will fail if the
// input |pattern| is not valid UTF-8.  Currently only group names may actually
// contain non-ASCII characters, however.  Unicode characters in other parts of
// the pattern will cause an error to be returned.  A |callback| must be
// provided to validate and encode plain text parts of the pattern.  An
// |options| value may be provided to override default behavior.
COMPONENT_EXPORT(LIBURLPATTERN)
absl::StatusOr<Pattern> Parse(std::string_view pattern,
                              EncodeCallback callback,
                              const Options& options = Options());

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_PARSE_H_
