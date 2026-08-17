// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MAP_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MAP_H_

#include <string>
#include "base/containers/flat_map.h"
#include "base/strings/string_util.h"

namespace blink {

struct FetchAPIRequestHeadersCompare {
  bool operator()(const std::string& lhs, const std::string& rhs) const {
    return base::CompareCaseInsensitiveASCII(lhs, rhs) < 0;
  }
};

using FetchAPIRequestHeadersMap =
    base::flat_map<std::string, std::string, FetchAPIRequestHeadersCompare>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FETCH_FETCH_API_REQUEST_HEADERS_MAP_H_
