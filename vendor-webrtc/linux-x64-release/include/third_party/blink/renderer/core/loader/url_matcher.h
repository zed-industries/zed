// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_URL_MATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_URL_MATCHER_H_

#include <string_view>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// UrlMatcher is a class to manage the list of URLs stored in the field trial
// param. As the original data from the field trial params is delivered as a
// special format string, this class parses and formats it, and stores the list.
//
// The expected param format is a comma separated string, and each string is
// separated by the vertical bar. The left side of the vertical bar is a
// host name, and the right side is a part of path or search params.
//
// The string is something like
// "https://test.exmaple|/foo,http://another.test.example|?foo=bar,https:://yet.another.test.example"
// Then the UrlMatcher will parse it to the formatted list like
// [
//  ["https://test.example", "/foo"],
//  ["http://another.test.example", "foo=bar"],
//  ["https:://yet.another.test.example", ""]
// ]
// Based on the above list, UrlMatcher::Match() checks 1) if the given url is a
// same origin or not, 2) if it's a same origin, check the second value in the
// list item. If it's an empty string, that means origin-level url matching. If
// it has a string, check the path string and query string in the given url
// contain it or not.
class CORE_EXPORT UrlMatcher final {
 public:
  explicit UrlMatcher(const std::string_view& encoded_url_list_string);
  ~UrlMatcher();

  bool Match(const KURL& url) const;

 private:
  using UrlList = Vector<
      std::pair<scoped_refptr<const SecurityOrigin>, std::optional<String>>>;
  UrlList url_list_;

  void ParseFieldTrialParam(const std::string_view& encoded_url_list_string);
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_URL_MATCHER_H_
