/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_UNITS_DATA_SIZE_H_
#define API_UNITS_DATA_SIZE_H_

#include <cstdint>
#include <string>
#include <type_traits>

#include "rtc_base/system/rtc_export.h"
#include "rtc_base/units/unit_base.h"  // IWYU pragma: export

namespace webrtc {
// DataSize is a class represeting a count of bytes.
class DataSize final : public rtc_units_impl::RelativeUnit<DataSize> {
 public:
  template <typename T>
  static constexpr DataSize Bytes(T value) {
    static_assert(std::is_arithmetic<T>::value, "");
    return FromValue(value);
  }
  static constexpr DataSize Infinity() { return PlusInfinity(); }

  constexpr DataSize() = default;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, DataSize value);

  template <typename T = int64_t>
  constexpr T bytes() const {
    return ToValue<T>();
  }

  constexpr int64_t bytes_or(int64_t fallback_value) const {
    return ToValueOr(fallback_value);
  }

 private:
  friend class rtc_units_impl::UnitBase<DataSize>;
  using RelativeUnit::RelativeUnit;
  static constexpr bool one_sided = true;
};

RTC_EXPORT std::string ToString(DataSize value);

template <typename Sink>
void AbslStringify(Sink& sink, DataSize value) {
  sink.Append(ToString(value));
}

}  // namespace webrtc

#endif  // API_UNITS_DATA_SIZE_H_
