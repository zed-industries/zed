// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CHRONO_TAI_CLOCK_H
#define _LIBCPP___CHRONO_TAI_CLOCK_H

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

class tai_clock;

template <class _Duration>
using tai_time    = time_point<tai_clock, _Duration>;
using tai_seconds = tai_time<seconds>;

// [time.clock.tai.overview]/1
//    The clock tai_clock measures seconds since 1958-01-01 00:00:00 and is
//    offset 10s ahead of UTC at this date. That is, 1958-01-01 00:00:00 TAI is
//    equivalent to 1957-12-31 23:59:50 UTC. Leap seconds are not inserted into
//    TAI. Therefore every time a leap second is inserted into UTC, UTC shifts
//    another second with respect to TAI. For example by 2000-01-01 there had
//    been 22 positive and 0 negative leap seconds inserted so 2000-01-01
//    00:00:00 UTC is equivalent to 2000-01-01 00:00:32 TAI (22s plus the
//    initial 10s offset).
//
// Note this does not specify what the UTC offset before 1958-01-01 00:00:00
// TAI is, nor does it follow the "real" TAI clock between 1958-01-01 and the
// start of the UTC epoch. So while the member functions are fully specified in
// the standard, they do not technically follow the "real-world" TAI clock with
// 100% accuracy.
//
// https://koka-lang.github.io/koka/doc/std_time_utc.html contains more
// information and references.
class tai_clock {
public:
  using rep                       = utc_clock::rep;
  using period                    = utc_clock::period;
  using duration                  = chrono::duration<rep, period>;
  using time_point                = chrono::time_point<tai_clock>;
  static constexpr bool is_steady = false; // The utc_clock is not steady.

  // The static difference between UTC and TAI time.
  static constexpr chrono::seconds __offset{378691210};

  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static time_point now() { return from_utc(utc_clock::now()); }

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static utc_time<common_type_t<_Duration, seconds>>
  to_utc(const tai_time<_Duration>& __time) noexcept {
    using _Rp                    = common_type_t<_Duration, seconds>;
    _Duration __time_since_epoch = __time.time_since_epoch();
    _LIBCPP_ASSERT_ARGUMENT_WITHIN_DOMAIN(__time_since_epoch >= utc_time<_Rp>::min().time_since_epoch() + __offset,
                                          "the TAI to UTC conversion would underflow");

    return utc_time<_Rp>{__time_since_epoch - __offset};
  }

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static tai_time<common_type_t<_Duration, seconds>>
  from_utc(const utc_time<_Duration>& __time) noexcept {
    using _Rp                    = common_type_t<_Duration, seconds>;
    _Duration __time_since_epoch = __time.time_since_epoch();
    _LIBCPP_ASSERT_ARGUMENT_WITHIN_DOMAIN(__time_since_epoch <= utc_time<_Rp>::max().time_since_epoch() - __offset,
                                          "the UTC to TAI conversion would overflow");

    return tai_time<_Rp>{__time_since_epoch + __offset};
  }
};

} // namespace chrono

#  endif // _LIBCPP_STD_VER >= 20 && _LIBCPP_HAS_TIME_ZONE_DATABASE && _LIBCPP_HAS_FILESYSTEM &&
         // _LIBCPP_HAS_LOCALIZATION

_LIBCPP_END_NAMESPACE_STD

_LIBCPP_POP_MACROS

#endif // _LIBCPP_HAS_EXPERIMENTAL_TZDB

#endif // _LIBCPP___CHRONO_TAI_CLOCK_H
