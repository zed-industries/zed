/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_UNITS_DATA_RATE_H_
#define API_UNITS_DATA_RATE_H_

#include <cstdint>
#include <limits>
#include <string>
#include <type_traits>

#include "api/units/data_size.h"
#include "api/units/frequency.h"
#include "api/units/time_delta.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/units/unit_base.h"  // IWYU pragma: export

namespace webrtc {
// DataRate is a class that represents a given data rate. This can be used to
// represent bandwidth, encoding bitrate, etc. The internal storage is bits per
// second (bps).
class DataRate final : public rtc_units_impl::RelativeUnit<DataRate> {
 public:
  template <typename T>
  static constexpr DataRate BitsPerSec(T value) {
    static_assert(std::is_arithmetic<T>::value, "");
    return FromValue(value);
  }
  template <typename T>
  static constexpr DataRate BytesPerSec(T value) {
    static_assert(std::is_arithmetic<T>::value, "");
    return FromFraction(8, value);
  }
  template <typename T>
  static constexpr DataRate KilobitsPerSec(T value) {
    static_assert(std::is_arithmetic<T>::value, "");
    return FromFraction(1000, value);
  }
  static constexpr DataRate Infinity() { return PlusInfinity(); }

  constexpr DataRate() = default;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, DataRate value);

  template <typename T = int64_t>
  constexpr T bps() const {
    return ToValue<T>();
  }
  template <typename T = int64_t>
  constexpr T bytes_per_sec() const {
    return ToFraction<8, T>();
  }
  template <typename T = int64_t>
  constexpr T kbps() const {
    return ToFraction<1000, T>();
  }
  constexpr int64_t bps_or(int64_t fallback_value) const {
    return ToValueOr(fallback_value);
  }
  constexpr int64_t kbps_or(int64_t fallback_value) const {
    return ToFractionOr<1000>(fallback_value);
  }

 private:
  // Bits per second used internally to simplify debugging by making the value
  // more recognizable.
  friend class rtc_units_impl::UnitBase<DataRate>;
  using RelativeUnit::RelativeUnit;
  static constexpr bool one_sided = true;
};

namespace data_rate_impl {
inline constexpr int64_t Microbits(const DataSize& size) {
  constexpr int64_t kMaxBeforeConversion =
      std::numeric_limits<int64_t>::max() / 8000000;
  RTC_DCHECK_LE(size.bytes(), kMaxBeforeConversion)
      << "size is too large to be expressed in microbits";
  return size.bytes() * 8000000;
}

inline constexpr int64_t MillibytePerSec(const DataRate& size) {
  constexpr int64_t kMaxBeforeConversion =
      std::numeric_limits<int64_t>::max() / (1000 / 8);
  RTC_DCHECK_LE(size.bps(), kMaxBeforeConversion)
      << "rate is too large to be expressed in microbytes per second";
  return size.bps() * (1000 / 8);
}
}  // namespace data_rate_impl

inline constexpr DataRate operator/(const DataSize size,
                                    const TimeDelta duration) {
  return DataRate::BitsPerSec(data_rate_impl::Microbits(size) / duration.us());
}
inline constexpr TimeDelta operator/(const DataSize size, const DataRate rate) {
  return TimeDelta::Micros(data_rate_impl::Microbits(size) / rate.bps());
}
inline constexpr DataSize operator*(const DataRate rate,
                                    const TimeDelta duration) {
  int64_t microbits = rate.bps() * duration.us();
  return DataSize::Bytes((microbits + 4000000) / 8000000);
}
inline constexpr DataSize operator*(const TimeDelta duration,
                                    const DataRate rate) {
  return rate * duration;
}

inline constexpr DataSize operator/(const DataRate rate,
                                    const Frequency frequency) {
  int64_t millihertz = frequency.millihertz<int64_t>();
  // Note that the value is truncated here reather than rounded, potentially
  // introducing an error of .5 bytes if rounding were expected.
  return DataSize::Bytes(data_rate_impl::MillibytePerSec(rate) / millihertz);
}
inline constexpr Frequency operator/(const DataRate rate, const DataSize size) {
  return Frequency::MilliHertz(data_rate_impl::MillibytePerSec(rate) /
                               size.bytes());
}
inline constexpr DataRate operator*(const DataSize size,
                                    const Frequency frequency) {
  RTC_DCHECK(frequency.IsZero() ||
             size.bytes() <= std::numeric_limits<int64_t>::max() / 8 /
                                 frequency.millihertz<int64_t>());
  int64_t millibits_per_second =
      size.bytes() * 8 * frequency.millihertz<int64_t>();
  return DataRate::BitsPerSec((millibits_per_second + 500) / 1000);
}
inline constexpr DataRate operator*(const Frequency frequency,
                                    const DataSize size) {
  return size * frequency;
}

RTC_EXPORT std::string ToString(DataRate value);

template <typename Sink>
void AbslStringify(Sink& sink, DataRate value) {
  sink.Append(ToString(value));
}

}  // namespace webrtc

#endif  // API_UNITS_DATA_RATE_H_
