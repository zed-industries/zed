/*
 * Copyright (C) 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_H_

#include <algorithm>
#include <ostream>

#include "base/containers/enum_set.h"
#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {

class SMILRepeatCount;
struct SMILInterval;

// SMILTime is used as both a time and time delta in the SMIL animation
// code. It is a small wrapper around TimeDelta that adds two sentinel values
// for SMIL concepts: "indefinite" and "unresolved".
//
// For ordering, the special values have the properties that:
//
//  <finite SMILTime> < SMILTime::Indefinite() < SMILTime::Unresolved()
//
// SMILTime::Earliest() and SMILTime::Latest() are smallest and largest finite
// time values respectively and sort accordingly.
//
// Addition and subtraction follow the semantics defined for computations of
// active duration (https://www.w3.org/TR/SMIL3/smil-timing.html#q78).
class SMILTime {
  DISALLOW_NEW();

 public:
  constexpr SMILTime() = default;

  static constexpr SMILTime Unresolved() { return base::TimeDelta::Max(); }
  static constexpr SMILTime Indefinite() {
    return base::TimeDelta::Max() - base::Microseconds(1);
  }
  static constexpr SMILTime Latest() {
    return base::TimeDelta::Max() - base::Microseconds(2);
  }
  static constexpr SMILTime Earliest() { return base::TimeDelta::Min(); }
  static constexpr SMILTime Epsilon() { return base::Microseconds(1); }
  static constexpr SMILTime FromSecondsD(double seconds) {
    return FromTimeDelta(base::Seconds(seconds));
  }
  static constexpr SMILTime FromTimeDelta(base::TimeDelta delta) {
    return std::min(SMILTime(delta), Latest());
  }
  base::TimeDelta ToTimeDelta() const { return time_; }

  // Used for computing progress. Don't use for anything else.
  double InternalValueAsDouble() const { return time_.InMicrosecondsF(); }
  double InSecondsF() const { return time_.InSecondsF(); }

  bool IsFinite() const { return *this < Indefinite(); }
  bool IsIndefinite() const { return *this == Indefinite(); }
  bool IsUnresolved() const { return *this == Unresolved(); }

  SMILTime Repeat(SMILRepeatCount repeat_count) const;

  SMILTime operator+(SMILTime other) const {
    if (!IsFinite())
      return time_;
    if (!other.IsFinite())
      return other;
    SMILTime result(time_ + other.time_);
    return result.IsFinite() ? result : Latest();
  }
  SMILTime operator-(SMILTime other) const {
    if (!IsFinite())
      return time_;
    if (!other.IsFinite())
      return other;
    SMILTime result(time_ - other.time_);
    return result.IsFinite() ? result : Latest();
  }
  SMILTime operator-() const { return -time_; }
  // Division and /modulo are used primarily for computing interval
  // progress/repeats.
  int64_t IntDiv(SMILTime other) const {
    DCHECK(IsFinite());
    DCHECK(other.IsFinite());
    return time_.IntDiv(other.time_);
  }
  SMILTime operator%(SMILTime other) const {
    DCHECK(IsFinite());
    DCHECK(other.IsFinite());
    return SMILTime(time_ % other.time_);
  }

  constexpr bool operator==(SMILTime other) const {
    return time_ == other.time_;
  }
  explicit operator bool() const { return IsFinite() && !time_.is_zero(); }
  constexpr bool operator!=(SMILTime other) const {
    return time_ != other.time_;
  }

  // Ordering of SMILTimes has to follow: finite < indefinite < unresolved. We
  // set this up by assigning consecutive sentinel values for the two latter
  // (see above).
  constexpr bool operator>(SMILTime other) const { return time_ > other.time_; }
  constexpr bool operator<(SMILTime other) const { return time_ < other.time_; }
  constexpr bool operator>=(SMILTime other) const {
    return time_ >= other.time_;
  }
  constexpr bool operator<=(SMILTime other) const {
    return time_ <= other.time_;
  }

 private:
  constexpr SMILTime(base::TimeDelta time) : time_(time) {}

  base::TimeDelta time_;
};

CORE_EXPORT std::ostream& operator<<(std::ostream& os, SMILTime time);

// What generated a SMILTime.
enum class SMILTimeOrigin {
  kAttribute,       // 'begin' / 'end' attribute
  kScript,          // beginElementAt / endElementAt
  kSyncBase,        // Sync-base
  kRepeat,          // Repeat event
  kEvent,           // DOM event
  kLinkActivation,  // Link activation (Click on link referring to timed
                    // element.)
};

using SMILTimeOriginSet = base::EnumSet<SMILTimeOrigin,
                                        SMILTimeOrigin::kAttribute,
                                        SMILTimeOrigin::kLinkActivation>;

class SMILTimeWithOrigin {
  DISALLOW_NEW();

 public:
  SMILTimeWithOrigin(const SMILTime& time, SMILTimeOrigin origin)
      : time_(time), origin_(origin) {}

  const SMILTime& Time() const { return time_; }
  SMILTimeOrigin Origin() const { return origin_; }

 private:
  SMILTime time_;
  SMILTimeOrigin origin_;
};

inline bool operator<(const SMILTimeWithOrigin& a,
                      const SMILTimeWithOrigin& b) {
  return a.Time() < b.Time();
}

// An interval of SMILTimes.
// "...the begin time of the interval is included in the interval, but the end
// time is excluded from the interval."
// (https://www.w3.org/TR/SMIL3/smil-timing.html#q101)
struct SMILInterval {
  DISALLOW_NEW();
  constexpr SMILInterval(const SMILTime& begin, const SMILTime& end)
      : begin(begin), end(end) {}

  static constexpr SMILInterval Unresolved() {
    return {SMILTime::Unresolved(), SMILTime::Unresolved()};
  }

  bool IsResolved() const { return begin.IsFinite(); }
  bool BeginsAfter(SMILTime time) const { return time < begin; }
  bool BeginsBefore(SMILTime time) const { return !BeginsAfter(time); }
  bool EndsAfter(SMILTime time) const {
    DCHECK(IsResolved());
    return time < end;
  }
  bool EndsBefore(SMILTime time) const { return !EndsAfter(time); }
  bool Contains(SMILTime time) const {
    return BeginsBefore(time) && EndsAfter(time);
  }

  SMILTime begin;
  SMILTime end;
};

inline bool operator==(const SMILInterval& a, const SMILInterval& b) {
  return a.begin == b.begin && a.end == b.end;
}

inline bool operator!=(const SMILInterval& a, const SMILInterval& b) {
  return !(a == b);
}

}  // namespace blink

namespace WTF {
template <>
struct HashTraits<blink::SMILInterval>
    : GenericHashTraits<blink::SMILInterval> {
  static blink::SMILInterval EmptyValue() {
    return blink::SMILInterval::Unresolved();
  }
};
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_TIME_H_
