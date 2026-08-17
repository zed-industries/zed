//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_COUNTING_PREDICATES_H
#define TEST_SUPPORT_COUNTING_PREDICATES_H

#include <cstddef>
#include <utility>
#include "test_macros.h"

template <typename Predicate, typename Arg>
struct unary_counting_predicate {
public:
    typedef Arg argument_type;
    typedef bool result_type;

    unary_counting_predicate(Predicate p) : p_(p), count_(0) {}
    unary_counting_predicate(const unary_counting_predicate&) = default;
    unary_counting_predicate& operator=(const unary_counting_predicate&) = default;
    ~unary_counting_predicate() {}

    bool operator () (const Arg &a) const { ++count_; return p_(a); }
    std::size_t count() const { return count_; }
    void reset() { count_ = 0; }

private:
    Predicate p_;
    mutable std::size_t count_;
};


template <typename Predicate, typename Arg1, typename Arg2=Arg1>
struct binary_counting_predicate {
public:
    typedef Arg1 first_argument_type;
    typedef Arg2 second_argument_type;
    typedef bool result_type;

    TEST_CONSTEXPR binary_counting_predicate(Predicate p) : p_(p), count_(0) {}
    TEST_CONSTEXPR_CXX14 bool operator()(const Arg1& a1, const Arg2& a2) const {
      ++count_;
      return p_(a1, a2);
    }
    TEST_CONSTEXPR std::size_t count() const { return count_; }
    TEST_CONSTEXPR_CXX14 void reset() { count_ = 0; }

  private:
    Predicate p_;
    mutable std::size_t count_;
};

#if TEST_STD_VER > 14

template <class Predicate>
class counting_predicate {
  Predicate pred_;
  int* count_ = nullptr;

public:
  constexpr counting_predicate() = default;
  constexpr counting_predicate(Predicate pred, int& count) : pred_(std::move(pred)), count_(&count) {}

  template <class... Args>
  constexpr decltype(auto) operator()(Args&& ...args) {
    ++(*count_);
    return pred_(std::forward<Args>(args)...);
  }

  template <class... Args>
  constexpr decltype(auto) operator()(Args&& ...args) const {
    ++(*count_);
    return pred_(std::forward<Args>(args)...);
  }
};

template <class Predicate>
counting_predicate(Predicate pred, int& count) -> counting_predicate<Predicate>;

#endif // TEST_STD_VER > 14

#endif // TEST_SUPPORT_COUNTING_PREDICATES_H
