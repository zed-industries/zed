#ifndef TEST_SUPPORT_EMPLACE_CONSTRUCTIBLE_H
#define TEST_SUPPORT_EMPLACE_CONSTRUCTIBLE_H

#include "test_macros.h"

#if TEST_STD_VER >= 11
template <class T>
struct EmplaceConstructible {
  T value;
  TEST_CONSTEXPR_CXX14 explicit EmplaceConstructible(T xvalue) : value(xvalue) {}
  EmplaceConstructible(EmplaceConstructible const&) = delete;
};

template <class T>
struct EmplaceConstructibleAndMoveInsertable {
  int copied = 0;
  T value;
  TEST_CONSTEXPR_CXX14 explicit EmplaceConstructibleAndMoveInsertable(T xvalue) : value(xvalue) {}

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleAndMoveInsertable(
      EmplaceConstructibleAndMoveInsertable&& Other)
      : copied(Other.copied + 1), value(std::move(Other.value)) {}
};

template <class T>
struct EmplaceConstructibleAndMoveable {
  int copied = 0;
  int assigned = 0;
  T value;
  TEST_CONSTEXPR_CXX14 explicit EmplaceConstructibleAndMoveable(T xvalue) noexcept : value(xvalue) {}

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleAndMoveable(EmplaceConstructibleAndMoveable&& Other)
      noexcept : copied(Other.copied + 1),
                 value(std::move(Other.value)) {}

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleAndMoveable&
  operator=(EmplaceConstructibleAndMoveable&& Other) noexcept {
    copied = Other.copied;
    assigned = Other.assigned + 1;
    value = std::move(Other.value);
    return *this;
  }
};

template <class T>
struct EmplaceConstructibleMoveableAndAssignable {
  int copied = 0;
  int assigned = 0;
  T value;
  TEST_CONSTEXPR_CXX14 explicit EmplaceConstructibleMoveableAndAssignable(T xvalue) noexcept
      : value(xvalue) {}

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleMoveableAndAssignable(
      EmplaceConstructibleMoveableAndAssignable&& Other) noexcept
      : copied(Other.copied + 1),
        value(std::move(Other.value)) {}

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleMoveableAndAssignable&
  operator=(EmplaceConstructibleMoveableAndAssignable&& Other) noexcept {
    copied = Other.copied;
    assigned = Other.assigned + 1;
    value = std::move(Other.value);
    return *this;
  }

  TEST_CONSTEXPR_CXX14 EmplaceConstructibleMoveableAndAssignable& operator=(T xvalue) {
    value = std::move(xvalue);
    ++assigned;
    return *this;
  }
};
#endif

#endif // TEST_SUPPORT_EMPLACE_CONSTRUCTIBLE_H
