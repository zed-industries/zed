// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_LOADER_FREEZE_MODE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_LOADER_FREEZE_MODE_H_

namespace blink {

// An enum representing the mode of "freezing".
// See also: third_party/blink/renderer/platform/loader/README.md
enum class WebLoaderFreezeMode : uint8_t {
  // Stop any incoming IPCs. Note that freezing loaders too long with this
  // mode has negative performance impact globally, because the network
  // service and the resource scheduler on it will be confused by this behavior.
  kStrict,

  // Do not stop incoming IPCs. Instead, read and buffer them. In some cases
  // this is impossible (e.g., the response body is too large), and in such
  // cases the loading would fail and the system will be notified.
  kBufferIncoming,

  // Stop freezing. This value will be moved out of this enum soon.
  kNone,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_LOADER_FREEZE_MODE_H_
