// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CHRONO_GPS_CLOCK_H
#define _LIBCPP___CHRONO_GPS_CLOCK_H

#include <version>
// Enable the contents of the header only when libc++ was built with experimental features enabled.
#if _LIBCPP_HAS_EXPERIMENTAL_TZDB

#  include <__assert>
#  include <__chrono/duration.h>
#  include <__chrono/time_point.h>
#  include <__chrono/utc_clock.h>
#  include <__config>
#  include <__type_traits/common_type.h>

#  if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#    pragma GCC system_header
#  endif

_LIBCPP_PUSH_MACROS
#  include <__undef_macros>

_LIBCPP_BEGIN_NAMESPACE_STD

#  if _LIBCPP_STD_VER >= 20 && _LIBCPP_HAS_TIME_ZONE_DATABASE && _LIBCPP_HAS_FILESYSTEM && _LIBCPP_HAS_LOCALIZATION

namespace chrono {

class gps_clock;

template <class _Duration>
using gps_time    = time_point<gps_clock, _Duration>;
using gps_seconds = gps_time<seconds>;

class gps_clock {
public:
  using rep                       = utc_clock::rep;
  using period                    = utc_clock::period;
  using duration                  = chrono::duration<rep, period>;
  using time_point                = chrono::time_point<gps_clock>;
  static constexpr bool is_steady = false; // The utc_clock is not steady.

  // The static difference between UTC and GPS time as specified in the Standard.
  static constexpr chrono::seconds __offset{315964809};

  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static time_point now() { return from_utc(utc_clock::now()); }

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static utc_time<common_type_t<_Duration, seconds>>
  to_utc(const gps_time<_Duration>& __time) noexcept {
    using _Rp                    = common_type_t<_Duration, seconds>;
    _Duration __time_since_epoch = __time.time_since_epoch();
    _LIBCPP_ASSERT_ARGUMENT_WITHIN_DOMAIN(__time_since_epoch >= utc_time<_Rp>::min().time_since_epoch() + __offset,
                                          "the GPS to UTC conversion would underflow");

    return utc_time<_Rp>{__time_since_epoch + __offset};
  }

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static gps_time<common_type_t<_Duration, seconds>>
  from_utc(const utc_time<_Duration>& __time) noexcept {
    using _Rp                    = common_type_t<_Duration, seconds>;
    _Duration __time_since_epoch = __time.time_since_epoch();
    _LIBCPP_ASSERT_ARGUMENT_WITHIN_DOMAIN(__time_since_epoch <= utc_time<_Rp>::max().time_since_epoch() - __offset,
                                          "the UTC to GPS conversion would overflow");

    return gps_time<_Rp>{__time_since_epoch - __offset};
  }
};

} // namespace chrono

#  endif // _LIBCPP_STD_VER >= 20 && _LIBCPP_HAS_TIME_ZONE_DATABASE && _LIBCPP_HAS_FILESYSTEM &&
         // _LIBCPP_HAS_LOCALIZATION

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP_HAS_EXPERIMENTAL_TZDB

#endif // _LIBCPP___CHRONO_GPS_CLOCK_H
