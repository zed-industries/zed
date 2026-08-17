// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___CHRONO_UTC_CLOCK_H
#define _LIBCPP___CHRONO_UTC_CLOCK_H

#include <version>
// Enable the contents of the header only when libc++ was built with experimental features enabled.
#if _LIBCPP_HAS_EXPERIMENTAL_TZDB

#  include <__chrono/duration.h>
#  include <__chrono/leap_second.h>
#  include <__chrono/system_clock.h>
#  include <__chrono/time_point.h>
#  include <__chrono/tzdb.h>
#  include <__chrono/tzdb_list.h>
#  include <__config>
#  include <__type_traits/common_type.h>

#  if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#    pragma GCC system_header
#  endif

_LIBCPP_BEGIN_NAMESPACE_STD

#  if _LIBCPP_STD_VER >= 20 && _LIBCPP_HAS_TIME_ZONE_DATABASE && _LIBCPP_HAS_FILESYSTEM && _LIBCPP_HAS_LOCALIZATION

namespace chrono {

class utc_clock;

template <class _Duration>
using utc_time    = time_point<utc_clock, _Duration>;
using utc_seconds = utc_time<seconds>;

class utc_clock {
public:
  using rep                       = system_clock::rep;
  using period                    = system_clock::period;
  using duration                  = chrono::duration<rep, period>;
  using time_point                = chrono::time_point<utc_clock>;
  static constexpr bool is_steady = false; // The system_clock is not steady.

  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static time_point now() { return from_sys(system_clock::now()); }

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static sys_time<common_type_t<_Duration, seconds>>
  to_sys(const utc_time<_Duration>& __time);

  template <class _Duration>
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI static utc_time<common_type_t<_Duration, seconds>>
  from_sys(const sys_time<_Duration>& __time) {
    using _Rp = utc_time<common_type_t<_Duration, seconds>>;
    // TODO TZDB investigate optimizations.
    //
    // The leap second database stores all transitions, this mean to calculate
    // the current number of leap seconds the code needs to iterate over all
    // leap seconds to accumulate the sum. Then the sum can be used to determine
    // the sys_time. Accessing the database involves acquiring a mutex.
    //
    // The historic entries in the database are immutable. Hard-coding these
    // values in a table would allow:
    // - To store the sum, allowing a binary search on the data.
    // - Avoid acquiring a mutex.
    // The disadvantage are:
    // - A slightly larger code size.
    //
    // There are two optimization directions
    // - hard-code the database and do a linear search for future entries. This
    //   search can start at the back, and should probably contain very few
    //   entries. (Adding leap seconds is quite rare and new release of libc++
    //   can add the new entries; they are announced half a year before they are
    //   added.)
    // - During parsing the leap seconds store an additional database in the
    //   dylib with the list of the sum of the leap seconds. In that case there
    //   can be a private function __get_utc_to_sys_table that returns the
    //   table.
    //
    // Note for to_sys there are no optimizations to be done; it uses
    // get_leap_second_info. The function get_leap_second_info could benefit
    // from optimizations as described above; again both options apply.

    // Both UTC and the system clock use the same epoch. The Standard
    // specifies from 1970-01-01 even when UTC starts at
    // 1972-01-01 00:00:10 TAI. So when the sys_time is before epoch we can be
    // sure there both clocks return the same value.

    const tzdb& __tzdb = chrono::get_tzdb();
    _Rp __result{__time.time_since_epoch()};
    for (const auto& __leap_second : __tzdb.leap_seconds) {
      if (__leap_second > __time)
        return __result;

      __result += __leap_second.value();
    }
    return __result;
  }
};

struct leap_second_info {
  bool is_leap_second;
  seconds elapsed;
};

template <class _Duration>
[[nodiscard]] _LIBCPP_HIDE_FROM_ABI leap_second_info get_leap_second_info(const utc_time<_Duration>& __time) {
  const tzdb& __tzdb = chrono::get_tzdb();
  if (__tzdb.leap_seconds.empty()) [[unlikely]]
    return {false, chrono::seconds{0}};

  sys_seconds __sys{chrono::floor<seconds>(__time).time_since_epoch()};
  seconds __elapsed{0};
  for (const auto& __leap_second : __tzdb.leap_seconds) {
    if (__sys == __leap_second.date() + __elapsed)
      // A time point may only be a leap second during a positive leap second
      // insertion, since time points that occur during a (theoretical)
      // negative leap second don't exist.
      return {__leap_second.value() > 0s, __elapsed + __leap_second.value()};

    if (__sys < __leap_second.date() + __elapsed)
      return {false, __elapsed};

    __elapsed += __leap_second.value();
  }

  return {false, __elapsed};
}

template <class _Duration>
[[nodiscard]] _LIBCPP_HIDE_FROM_ABI sys_time<common_type_t<_Duration, seconds>>
utc_clock::to_sys(const utc_time<_Duration>& __time) {
  using _Dp               = common_type_t<_Duration, seconds>;
  leap_second_info __info = chrono::get_leap_second_info(__time);

  // [time.clock.utc.members]/2
  //   Returns: A sys_time t, such that from_sys(t) == u if such a mapping
  //   exists. Otherwise u represents a time_point during a positive leap
  //   second insertion, the conversion counts that leap second as not
  //   inserted, and the last representable value of sys_time prior to the
  //   insertion of the leap second is returned.
  sys_time<common_type_t<_Duration, seconds>> __result{__time.time_since_epoch() - __info.elapsed};
  if (__info.is_leap_second)
    return chrono::floor<seconds>(__result) + chrono::seconds{1} - _Dp{1};

  return __result;
}

} // namespace chrono

#  endif // _LIBCPP_STD_VER >= 20 && _LIBCPP_HAS_TIME_ZONE_DATABASE && _LIBCPP_HAS_FILESYSTEM &&
         // _LIBCPP_HAS_LOCALIZATION

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_HAS_EXPERIMENTAL_TZDB

#endif // _LIBCPP___CHRONO_UTC_CLOCK_H
