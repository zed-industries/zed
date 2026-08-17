//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_DOUBLE_MOVE_TRACKER_H
#define TEST_SUPPORT_DOUBLE_MOVE_TRACKER_H

#include <cassert>

#include "test_macros.h"

namespace support {

struct double_move_tracker {
  TEST_CONSTEXPR double_move_tracker() : moved_from_(false) {}

  double_move_tracker(double_move_tracker const&) = default;

  TEST_CONSTEXPR_CXX14 double_move_tracker(double_move_tracker&& other) : moved_from_(false) {
    assert(!other.moved_from_);
    other.moved_from_ = true;
  }

  double_move_tracker& operator=(double_move_tracker const&) = default;

  TEST_CONSTEXPR_CXX14 double_move_tracker& operator=(double_move_tracker&& other) {
    assert(!other.moved_from_);
    other.moved_from_ = true;
    moved_from_       = false;
    return *this;
  }

private:
  bool moved_from_;
};

} // namespace support

#endif // TEST_SUPPORT_DOUBLE_MOVE_TRACKER_H
