//===-- type_traits.h -------------------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SCUDO_TYPE_TRAITS_H_
#define SCUDO_TYPE_TRAITS_H_

namespace scudo {

template <typename T> struct removeConst {
  using type = T;
};
template <typename T> struct removeConst<const T> {
  using type = T;
};

// This is only used for SFINAE when detecting if a type is defined.
template <typename T> struct voidAdaptor {
  using type = void;
};

template <typename L, typename R> struct assertSameType {
  template <typename, typename> struct isSame {
    static constexpr bool value = false;
  };
  template <typename T> struct isSame<T, T> {
    static constexpr bool value = true;
  };
  static_assert(isSame<L, R>::value, "Type mismatches");
  using type = R;
};

template <typename T> struct isPointer {
  static constexpr bool value = false;
};

template <typename T> struct isPointer<T *> {
  static constexpr bool value = true;
};

template <bool Cond, typename L, typename R> struct Conditional {
  using type = L;
};

template <typename L, typename R> struct Conditional<false, L, R> {
  using type = R;
};

} // namespace scudo

#endif // SCUDO_TYPE_TRAITS_H_
