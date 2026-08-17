/*
 *  Copyright 2019 The Chromium Authors. All rights reserved.
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_TYPES_H_
#define NET_DCSCTP_PUBLIC_TYPES_H_

#include <cstdint>
#include <limits>

#include "api/units/time_delta.h"
#include "rtc_base/strong_alias.h"

namespace dcsctp {

// Stream Identifier
using StreamID = webrtc::StrongAlias<class StreamIDTag, uint16_t>;

// Payload Protocol Identifier (PPID)
using PPID = webrtc::StrongAlias<class PPIDTag, uint32_t>;

// Timeout Identifier
using TimeoutID = webrtc::StrongAlias<class TimeoutTag, uint64_t>;

// Indicates if a message is allowed to be received out-of-order compared to
// other messages on the same stream.
using IsUnordered = webrtc::StrongAlias<class IsUnorderedTag, bool>;

// Stream priority, where higher values indicate higher priority. The meaning of
// this value and how it's used depends on the stream scheduler.
using StreamPriority = webrtc::StrongAlias<class StreamPriorityTag, uint16_t>;

// Duration, as milliseconds. Overflows after 24 days.
class DurationMs : public webrtc::StrongAlias<class DurationMsTag, int32_t> {
 public:
  constexpr explicit DurationMs(const UnderlyingType& v)
      : webrtc::StrongAlias<class DurationMsTag, int32_t>(v) {}

  constexpr explicit DurationMs(webrtc::TimeDelta v)
      : webrtc::StrongAlias<class DurationMsTag, int32_t>(
            v.IsInfinite() ? InfiniteDuration() : DurationMs(v.ms())) {}

  static constexpr DurationMs InfiniteDuration() {
    return DurationMs(std::numeric_limits<int32_t>::max());
  }
  // Convenience methods for working with time.
  constexpr DurationMs& operator+=(DurationMs d) {
    value_ += d.value_;
    return *this;
  }
  constexpr DurationMs& operator-=(DurationMs d) {
    value_ -= d.value_;
    return *this;
  }
  template <typename T>
  constexpr DurationMs& operator*=(T factor) {
    value_ *= factor;
    return *this;
  }
  constexpr webrtc::TimeDelta ToTimeDelta() const {
    return *this == DurationMs::InfiniteDuration()
               ? webrtc::TimeDelta::PlusInfinity()
               : webrtc::TimeDelta::Millis(value_);
  }
};

constexpr inline DurationMs operator+(DurationMs lhs, DurationMs rhs) {
  return lhs += rhs;
}
constexpr inline DurationMs operator-(DurationMs lhs, DurationMs rhs) {
  return lhs -= rhs;
}
template <typename T>
constexpr inline DurationMs operator*(DurationMs lhs, T rhs) {
  return lhs *= rhs;
}
template <typename T>
constexpr inline DurationMs operator*(T lhs, DurationMs rhs) {
  return rhs *= lhs;
}
constexpr inline int32_t operator/(DurationMs lhs, DurationMs rhs) {
  return lhs.value() / rhs.value();
}

// Represents time, in milliseconds since a client-defined epoch.
class TimeMs : public webrtc::StrongAlias<class TimeMsTag, int64_t> {
 public:
  constexpr explicit TimeMs(const UnderlyingType& v)
      : webrtc::StrongAlias<class TimeMsTag, int64_t>(v) {}

  // Convenience methods for working with time.
  constexpr TimeMs& operator+=(DurationMs d) {
    value_ += *d;
    return *this;
  }
  constexpr TimeMs& operator-=(DurationMs d) {
    value_ -= *d;
    return *this;
  }

  static constexpr TimeMs InfiniteFuture() {
    return TimeMs(std::numeric_limits<int64_t>::max());
  }
};

constexpr inline TimeMs operator+(TimeMs lhs, DurationMs rhs) {
  return lhs += rhs;
}
constexpr inline TimeMs operator+(DurationMs lhs, TimeMs rhs) {
  return rhs += lhs;
}
constexpr inline TimeMs operator-(TimeMs lhs, DurationMs rhs) {
  return lhs -= rhs;
}
constexpr inline DurationMs operator-(TimeMs lhs, TimeMs rhs) {
  return DurationMs(*lhs - *rhs);
}

// The maximum number of times the socket should attempt to retransmit a
// message which fails the first time in unreliable mode.
class MaxRetransmits
    : public webrtc::StrongAlias<class MaxRetransmitsTag, uint16_t> {
 public:
  constexpr explicit MaxRetransmits(const UnderlyingType& v)
      : webrtc::StrongAlias<class MaxRetransmitsTag, uint16_t>(v) {}

  // There should be no limit - the message should be sent reliably.
  static constexpr MaxRetransmits NoLimit() {
    return MaxRetransmits(std::numeric_limits<uint16_t>::max());
  }
};

// An identifier that can be set on sent messages, and picked by the sending
// client. If different from `::NotSet()`, lifecycle events will be generated,
// and eventually `DcSctpSocketCallbacks::OnLifecycleEnd` will be called to
// indicate that the lifecycle isn't tracked any longer. The value zero (0) is
// not a valid lifecycle identifier, and will be interpreted as not having it
// set.
class LifecycleId : public webrtc::StrongAlias<class LifecycleIdTag, uint64_t> {
 public:
  constexpr explicit LifecycleId(const UnderlyingType& v)
      : webrtc::StrongAlias<class LifecycleIdTag, uint64_t>(v) {}

  constexpr bool IsSet() const { return value_ != 0; }

  static constexpr LifecycleId NotSet() { return LifecycleId(0); }
};

// To enable zero checksum feature, both peers must agree on which alternate
// error detection method that is used. See
// https://www.ietf.org/archive/id/draft-ietf-tsvwg-sctp-zero-checksum-06.html.
class ZeroChecksumAlternateErrorDetectionMethod
    : public webrtc::StrongAlias<
          class ZeroChecksumAlternateErrorDetectionMethodTag,
          uint32_t> {
 public:
  constexpr explicit ZeroChecksumAlternateErrorDetectionMethod(
      const UnderlyingType& v)
      : webrtc::StrongAlias<class ZeroChecksumAlternateErrorDetectionMethodTag,
                            uint32_t>(v) {}

  static constexpr ZeroChecksumAlternateErrorDetectionMethod None() {
    return ZeroChecksumAlternateErrorDetectionMethod(0);
  }
  static constexpr ZeroChecksumAlternateErrorDetectionMethod LowerLayerDtls() {
    return ZeroChecksumAlternateErrorDetectionMethod(1);
  }
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_TYPES_H_
