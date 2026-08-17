// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_RESOURCE_INTERCEPT_POLICY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_RESOURCE_INTERCEPT_POLICY_H_

namespace blink {

// The ResourceInterceptPolicy enum controls whether a resource request is
// allowed to be handled by plugin (intercept as a stream) or to turn into a
// download (intercept as a download).
//
// Note: For a restriction that only intends to prevent download, if it's also
// guaranteed that the resource won't be handled by plugin (e.g. in sandboxed
// iframe), then it's safe and also recommended to choose the more restricted
// policy |kAllowNone|.
enum class ResourceInterceptPolicy {
  // Allow all type of interceptions.
  kAllowAll = 0,

  // Disallow any type of interceptions.
  //
  // TODO(crbug/930951): the current implementation doesn't completely honor
  // this description. When the resource type is |ResourceType::kObject|, mime
  // sniffing would still check the existence of plugins and may intercept it as
  // a stream.
  kAllowNone = 1,

  // Only allow intercepting as stream.
  kAllowPluginOnly = 2,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_NAVIGATION_RESOURCE_INTERCEPT_POLICY_H_
