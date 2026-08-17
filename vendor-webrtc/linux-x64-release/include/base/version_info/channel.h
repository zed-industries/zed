// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_VERSION_INFO_CHANNEL_H_
#define BASE_VERSION_INFO_CHANNEL_H_

#include <string_view>

#include "base/notreached.h"

namespace version_info {

// The possible channels for an installation, from most fun to most stable.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.version_info
enum class Channel {
  UNKNOWN = 0,
  // DEFAULT is an alias for UNKNOWN because the build files use DEFAULT but the
  // code uses UNKNOWN. TODO(paulmiller): Combine DEFAULT & UNKNOWN.
  DEFAULT = UNKNOWN,
  CANARY = 1,
  DEV = 2,
  BETA = 3,
  STABLE = 4,
};

// Returns a string equivalent of |channel|, independent of whether the build
// is branded or not and without any additional modifiers.
constexpr std::string_view GetChannelString(Channel channel) {
  switch (channel) {
    case Channel::STABLE:
      return "stable";
    case Channel::BETA:
      return "beta";
    case Channel::DEV:
      return "dev";
    case Channel::CANARY:
      return "canary";
    case Channel::UNKNOWN:
      return "unknown";
  }
  NOTREACHED();
}

}  // namespace version_info

#endif  // BASE_VERSION_INFO_CHANNEL_H_
