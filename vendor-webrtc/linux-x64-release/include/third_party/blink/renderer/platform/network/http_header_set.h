// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_SET_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_SET_H_

#include <set>
#include <string>
#include "base/strings/string_util.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"

namespace blink {

struct CompareIgnoreCase {
  bool operator()(const std::string& left, const std::string& right) const {
    return base::CompareCaseInsensitiveASCII(left, right) < 0;
  }
};

using HTTPHeaderSet ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327") =
    std::set<std::string, CompareIgnoreCase>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_HEADER_SET_H_
