// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PACKAGE_WEB_PACKAGE_REQUEST_MATCHER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PACKAGE_WEB_PACKAGE_REQUEST_MATCHER_H_

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "base/gtest_prod_util.h"
#include "net/http/http_request_headers.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// WebPackageRequestMatcher implements the Request Matching algorithm [1].
// [1] https://wicg.github.io/webpackage/loading.html#request-matching
class BLINK_COMMON_EXPORT WebPackageRequestMatcher {
 public:
  // Keys must be lower-cased.
  using HeaderMap = std::map<std::string, std::string>;

  // |request_headers| is the request headers of `browserRequest` in [1]. If
  // it does not have an Accept-Language header, languages in |accept_langs|
  // are used for matching.
  // |accept_langs| is a comma separated ordered list of language codes.
  WebPackageRequestMatcher(const net::HttpRequestHeaders& request_headers,
                           const std::string& accept_langs);
  bool MatchRequest(const HeaderMap& response_headers) const;

  // Returns the iterator of |variant_keys_list| which contains the best
  // matching variant key. This method use the preference order of the result of
  // "Cache Behaviour" [2]. If there is no matching variant key, returns
  // |variant_keys_list.end()|.
  // [2]:
  // https://httpwg.org/http-extensions/draft-ietf-httpbis-variants.html#cache
  std::vector<std::string>::const_iterator FindBestMatchingVariantKey(
      const std::string& variants,
      const std::vector<std::string>& variant_keys_list) const;

  // Returns the index of best matching variant key within the all possible
  // key for |variants|, in lexicographic (row-major) order.
  std::optional<size_t> FindBestMatchingIndex(
      const std::string& variants) const;

 private:
  net::HttpRequestHeaders request_headers_;

  static bool MatchRequest(const net::HttpRequestHeaders& request_headers,
                           const HeaderMap& response_headers);
  static std::vector<std::vector<std::string>> CacheBehavior(
      const std::vector<std::pair<std::string, std::vector<std::string>>>&
          variants,
      const net::HttpRequestHeaders& request_headers);

  static std::vector<std::string>::const_iterator FindBestMatchingVariantKey(
      const net::HttpRequestHeaders& request_headers,
      const std::string& variants,
      const std::vector<std::string>& variant_key_list);

  static std::optional<size_t> FindBestMatchingIndex(
      const net::HttpRequestHeaders& request_headers,
      const std::string& variants);

  FRIEND_TEST_ALL_PREFIXES(WebPackageRequestMatcherTest, MatchRequest);
  FRIEND_TEST_ALL_PREFIXES(WebPackageRequestMatcherTest, CacheBehavior);
  FRIEND_TEST_ALL_PREFIXES(WebPackageRequestMatcherTest,
                           FindBestMatchingVariantKey);
  FRIEND_TEST_ALL_PREFIXES(WebPackageRequestMatcherTest, FindBestMatchingIndex);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_WEB_PACKAGE_WEB_PACKAGE_REQUEST_MATCHER_H_
