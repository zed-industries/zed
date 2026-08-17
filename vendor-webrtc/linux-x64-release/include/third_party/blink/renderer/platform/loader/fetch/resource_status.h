// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_STATUS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_STATUS_H_

namespace blink {

enum class ResourceStatus : uint8_t {
  kNotStarted,
  kPending,  // load in progress
  kCached,   // load completed successfully
  kLoadError,
  kDecodeError
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_STATUS_H_
