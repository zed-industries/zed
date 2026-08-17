// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_INTER_PROCESS_TIME_TICKS_CONVERTER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_INTER_PROCESS_TIME_TICKS_CONVERTER_H_

#include <stdint.h>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// SiteSpecificTimeDelta<T> is base::TimeDelta with a type tag. It it
// essentially base::TimeDelta, but SiteSpecificTimeDelta<T> is different from
// SiteSpecificTimeDelta<U> if T is different from U.
template <typename T>
class SiteSpecificTimeDelta final {
 public:
  SiteSpecificTimeDelta() = default;
  static SiteSpecificTimeDelta<T> FromTimeDelta(base::TimeDelta delta) {
    return SiteSpecificTimeDelta<T>(delta);
  }
  static SiteSpecificTimeDelta<T> FromMicroseconds(int64_t usec) {
    return SiteSpecificTimeDelta<T>(base::Microseconds(usec));
  }

  base::TimeDelta ToTimeDelta() const { return delta_; }
  bool operator==(const SiteSpecificTimeDelta<T> rhs) const {
    return delta_ == rhs.delta_;
  }
  bool operator<(const SiteSpecificTimeDelta<T> rhs) const {
    return delta_ < rhs.delta_;
  }
  bool operator<=(const SiteSpecificTimeDelta<T> rhs) const {
    return delta_ <= rhs.delta_;
  }

 private:
  explicit SiteSpecificTimeDelta(base::TimeDelta delta) : delta_(delta) {}

  base::TimeDelta delta_;
};

// For logging use only.
template <typename T>
std::ostream& operator<<(std::ostream& os, SiteSpecificTimeDelta<T> delta) {
  return os << delta.ToTimeDelta();
}

// SiteSpecificTimeTicks<T> is base::TimeTicks with a type tag. It is
// essentially base::TimeTicks, but SiteSpecificTimeTicks<T> is different from
// SiteSpecificTimeTicks<U> if T is different from U.
template <typename T>
class SiteSpecificTimeTicks final {
 public:
  SiteSpecificTimeTicks() = default;
  static SiteSpecificTimeTicks<T> FromTimeTicks(base::TimeTicks time_ticks) {
    return SiteSpecificTimeTicks<T>(time_ticks);
  }

  base::TimeTicks ToTimeTicks() const { return time_ticks_; }
  bool is_null() const { return time_ticks_.is_null(); }

  SiteSpecificTimeTicks<T> operator+(SiteSpecificTimeDelta<T> delta) const {
    return SiteSpecificTimeTicks<T>(time_ticks_ + delta.ToTimeDelta());
  }
  SiteSpecificTimeDelta<T> operator-(SiteSpecificTimeTicks<T> rhs) const {
    return SiteSpecificTimeDelta<T>::FromTimeDelta(time_ticks_ -
                                                   rhs.time_ticks_);
  }
  bool operator<(const SiteSpecificTimeTicks<T> rhs) const {
    return time_ticks_ < rhs.time_ticks_;
  }
  bool operator==(const SiteSpecificTimeTicks<T> rhs) const {
    return time_ticks_ == rhs.time_ticks_;
  }
  bool operator<=(const SiteSpecificTimeTicks<T> rhs) const {
    return time_ticks_ <= rhs.time_ticks_;
  }

 private:
  explicit SiteSpecificTimeTicks(base::TimeTicks time_ticks)
      : time_ticks_(time_ticks) {}

  base::TimeTicks time_ticks_;
};

// For logging use only.
template <typename T>
std::ostream& operator<<(std::ostream& os,
                         SiteSpecificTimeTicks<T> time_ticks) {
  return os << time_ticks.ToTimeTicks();
}

class SiteSpecificTimeLocalTag;
using LocalTimeTicks = SiteSpecificTimeTicks<SiteSpecificTimeLocalTag>;
using LocalTimeDelta = SiteSpecificTimeDelta<SiteSpecificTimeLocalTag>;

class SiteSpecificTimeRemoteTag;
using RemoteTimeTicks = SiteSpecificTimeTicks<SiteSpecificTimeRemoteTag>;
using RemoteTimeDelta = SiteSpecificTimeDelta<SiteSpecificTimeRemoteTag>;

// On Windows, TimeTicks are not always consistent between processes as
// indicated by |TimeTicks::IsConsistentAcrossProcesses()|. Often, the values on
// one process have a static offset relative to another. Occasionally, these
// offsets shift while running.
//
// To combat this, any TimeTicks values sent from the remote process to the
// local process must be tweaked in order to appear monotonic.
//
// In order to properly tweak ticks, we need 4 reference points:
//
// - |local_lower_bound|:  A known point, recorded on the local process, that
//                         occurs before any remote values that will be
//                         converted.
// - |remote_lower_bound|: The equivalent point on the remote process. This
//                         should be recorded immediately after
//                         |local_lower_bound|.
// - |local_upper_bound|:  A known point, recorded on the local process, that
//                         occurs after any remote values that will be
//                         converted.
// - |remote_upper_bound|: The equivalent point on the remote process. This
//                         should be recorded immediately before
//                         |local_upper_bound|.
//
// Once these bounds are determined, values within the remote process's range
// can be converted to the local process's range. The values are converted as
// follows:
//
// 1. If the remote's range exceeds the local's range, it is scaled to fit.
//    Any values converted will have the same scale factor applied.
//
// 2. The remote's range is shifted so that it is centered within the
//    local's range. Any values converted will be shifted the same amount.
class BLINK_COMMON_EXPORT InterProcessTimeTicksConverter {
 public:
  InterProcessTimeTicksConverter(LocalTimeTicks local_lower_bound,
                                 LocalTimeTicks local_upper_bound,
                                 RemoteTimeTicks remote_lower_bound,
                                 RemoteTimeTicks remote_upper_bound);

  // Returns the value within the local's bounds that correlates to
  // |remote_ms|.
  LocalTimeTicks ToLocalTimeTicks(RemoteTimeTicks remote_ms) const;

  // Returns the equivalent delta after applying remote-to-local scaling to
  // |remote_delta|.
  LocalTimeDelta ToLocalTimeDelta(RemoteTimeDelta remote_delta) const;

  // Returns the (remote time) - (local time) difference estimated by the
  // converter. This is the constant that is subtracted from remote TimeTicks to
  // get local TimeTicks when no scaling is applied.
  base::TimeDelta GetSkewForMetrics() const;

 private:
  // The local time which |remote_lower_bound_| is mapped to.
  LocalTimeTicks local_base_time_;
  LocalTimeDelta local_range_;

  double range_conversion_rate_;

  RemoteTimeTicks remote_lower_bound_;
  RemoteTimeTicks remote_upper_bound_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_INTER_PROCESS_TIME_TICKS_CONVERTER_H_
