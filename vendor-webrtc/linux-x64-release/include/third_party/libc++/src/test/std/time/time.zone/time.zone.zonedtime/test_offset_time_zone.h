// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_TIME_TIME_ZONE_TIME_ZONE_ZONEDTIME_TEST_OFFSET_TIME_ZONE_H
#define TEST_STD_TIME_TIME_ZONE_TIME_ZONE_ZONEDTIME_TEST_OFFSET_TIME_ZONE_H

#include <cassert>
#include <charconv>
#include <chrono>
#include <format>
#include <string_view>
#include <system_error>
#include <type_traits>

enum class offset_time_zone_flags {
  none             = 0,
  has_default_zone = 1,
  has_locate_zone  = 2,
  both             = has_default_zone | has_locate_zone
};

// The enforcement of the flags is done in the zoned_traits
template <offset_time_zone_flags flags = offset_time_zone_flags::both>
class offset_time_zone {
public:
  offset_time_zone() : offset_{std::chrono::seconds{0}} {}
  explicit offset_time_zone(std::string_view name) {
    int count;
    const char* begin             = name.data();
    const char* end               = begin + name.size();
    std::from_chars_result result = std::from_chars(begin, end, count);
    assert(result == std::from_chars_result(end, std::errc{}));

    offset_ = std::chrono::seconds(count);
  }

  std::chrono::seconds offset() const { return offset_; }

  offset_time_zone* operator->() { return this; }

  const offset_time_zone* operator->() const { return this; }

  template <class Duration>
  std::chrono::sys_time<std::common_type_t<Duration, std::chrono::seconds>>
  to_sys(const std::chrono::local_time<Duration>& local) const {
    return std::chrono::sys_time<std::common_type_t<Duration, std::chrono::seconds>>{
        local.time_since_epoch() + offset_};
  }

  template <class Duration>
  std::chrono::local_time<std::common_type_t<Duration, std::chrono::seconds>>
  to_local(const std::chrono::sys_time<Duration>& sys) const {
    return std::chrono::local_time<std::common_type_t<Duration, std::chrono::seconds>>{
        sys.time_since_epoch() - offset_};
  }

  template <class Duration>
  std::chrono::sys_info get_info(const std::chrono::sys_time<Duration>&) const {
    return {std::chrono::sys_seconds::min(),
            std::chrono::sys_seconds::max(),
            offset_,
            std::chrono::minutes{0},
            std::format("{:+03d}s", offset_.count())};
  }

private:
  std::chrono::seconds offset_;
};

template <>
struct std::chrono::zoned_traits<offset_time_zone<offset_time_zone_flags::has_default_zone>> {
  using type = offset_time_zone<offset_time_zone_flags::has_default_zone>;

  static type default_zone() { return {}; }
};

template <>
struct std::chrono::zoned_traits<offset_time_zone<offset_time_zone_flags::has_locate_zone>> {
  using type = offset_time_zone<offset_time_zone_flags::has_locate_zone>;

  static type locate_zone(std::string_view name) { return type{name}; }
};

template <>
struct std::chrono::zoned_traits<offset_time_zone<offset_time_zone_flags::both>> {
  using type = offset_time_zone<offset_time_zone_flags::both>;

  static type default_zone() { return {}; }
  static type locate_zone(std::string_view name) { return type{name}; }
};

#endif // TEST_STD_TIME_TIME_ZONE_TIME_ZONE_ZONEDTIME_TEST_OFFSET_TIME_ZONE_H
