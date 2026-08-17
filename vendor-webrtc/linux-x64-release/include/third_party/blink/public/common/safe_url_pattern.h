// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_H_

#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/liburlpattern/pattern.h"

namespace blink {

struct BLINK_COMMON_EXPORT SafeUrlPatternOptions {
  // The list of members has to stay in sync with the list in the equality
  // operator implementation in third_party/blink/common/safe_url_pattern.cc.
  bool ignore_case = false;
};

// SafeUrlPattern is a reduced version of URLPattern.
// It is used in both the browser process and the renderer process, and it does
// not allow a user-defined regular expression due to a security concern.
//
// To proceed the URLPattern matching, the user of the structure needs to
// set up their own `liburlpattern::Pattern`.
//
// `GetOptions` in
// third_party/blink/renderer/core/url_pattern/url_pattern_component.cc is
// a good example of how to set up `liburlpattern::Options` for
// each field. `SafeUrlPatternOptions::ignore_case` is provided for that
// purpose.
struct BLINK_COMMON_EXPORT SafeUrlPattern {
  SafeUrlPattern();
  ~SafeUrlPattern();

  // The list of members has to stay in sync with the list in the equality
  // operator implementation in third_party/blink/common/safe_url_pattern.cc.
  std::vector<liburlpattern::Part> protocol;
  std::vector<liburlpattern::Part> username;
  std::vector<liburlpattern::Part> password;
  std::vector<liburlpattern::Part> hostname;
  std::vector<liburlpattern::Part> port;
  std::vector<liburlpattern::Part> pathname;
  std::vector<liburlpattern::Part> search;
  std::vector<liburlpattern::Part> hash;

  SafeUrlPatternOptions options;
};

BLINK_COMMON_EXPORT bool operator==(const SafeUrlPattern& left,
                                    const SafeUrlPattern& right);

BLINK_COMMON_EXPORT bool operator==(const SafeUrlPatternOptions& left,
                                    const SafeUrlPatternOptions& right);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_H_
