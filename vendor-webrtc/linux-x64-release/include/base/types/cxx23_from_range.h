// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_CXX23_FROM_RANGE_H_
#define BASE_TYPES_CXX23_FROM_RANGE_H_

namespace base {

// A marker type for constructors from a range. This is a backport of
// `std::from_range_t` in C++23, and will be relaced with the std version one
// day.
//
// https://en.cppreference.com/w/cpp/ranges/from_range
struct from_range_t {
  explicit from_range_t() = default;
};

// The instantiation of `from_range_t`, to be passed to constructors taking the
// marker type `from_range_t`.
constexpr inline from_range_t from_range;

}  // namespace base

#endif  // BASE_TYPES_CXX23_FROM_RANGE_H_
