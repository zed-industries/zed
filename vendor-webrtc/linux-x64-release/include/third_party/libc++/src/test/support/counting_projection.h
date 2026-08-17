//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_COUNTING_PROJECTION_H
#define TEST_SUPPORT_COUNTING_PROJECTION_H

#include <functional>
#include <utility>
#include "test_macros.h"

#if TEST_STD_VER > 14

template <class Proj = std::identity>
class counting_projection {
  Proj proj_;
  int* count_ = nullptr;

public:
  constexpr counting_projection() = default;
  constexpr counting_projection(int& count) : count_(&count) {}
  constexpr counting_projection(Proj proj, int& count) : proj_(std::move(proj)), count_(&count) {}

  template <class T>
  constexpr decltype(auto) operator()(T&& value) const {
    ++(*count_);
    return std::invoke(proj_, std::forward<T>(value));
  }
};

counting_projection(int& count) -> counting_projection<std::identity>;
template <class Proj>
counting_projection(Proj proj, int& count) -> counting_projection<Proj>;

#endif // TEST_STD_VER > 14

#endif // TEST_SUPPORT_COUNTING_PROJECTION_H
