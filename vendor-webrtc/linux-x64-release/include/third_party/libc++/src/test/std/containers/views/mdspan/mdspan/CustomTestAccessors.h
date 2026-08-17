// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//                        Kokkos v. 4.0
//       Copyright (2022) National Technology & Engineering
//               Solutions of Sandia, LLC (NTESS).
//
// Under the terms of Contract DE-NA0003525 with NTESS,
// the U.S. Government retains certain rights in this software.
//
//===---------------------------------------------------------------------===//

#ifndef TEST_STD_CONTAINERS_VIEWS_MDSPAN_MDSPAN_CUSTOM_TEST_ACCESSORS_H
#define TEST_STD_CONTAINERS_VIEWS_MDSPAN_MDSPAN_CUSTOM_TEST_ACCESSORS_H

#include <cassert>
#include <cstddef>
#include <mdspan>
#include <type_traits>

// This contains a bunch of accessors and handles which have different properties
// regarding constructibility and convertibility in order to test mdspan constraints

// non default constructible data handle
template <class T>
struct no_default_ctor_handle {
  T* ptr;
  no_default_ctor_handle() = delete;
  constexpr no_default_ctor_handle(T* ptr_) : ptr(ptr_) {}
};

// handle that can't convert from T to const T
template <class T>
struct not_const_convertible_handle {
  T* ptr;
  constexpr not_const_convertible_handle() : ptr(nullptr) {}
  constexpr not_const_convertible_handle(T* ptr_) : ptr(ptr_) {}

  constexpr T& operator[](size_t i) const { return ptr[i]; }
};

// handle where move has side effects
template <class T>
struct move_counted_handle {
  T* ptr;
  constexpr move_counted_handle()                           = default;
  constexpr move_counted_handle(const move_counted_handle&) = default;
  template <class OtherT>
    requires(std::is_constructible_v<T*, OtherT*>)
  constexpr move_counted_handle(const move_counted_handle<OtherT>& other) : ptr(other.ptr) {}
  constexpr move_counted_handle(move_counted_handle&& other) {
    ptr = other.ptr;
    if (!std::is_constant_evaluated()) {
      move_counter()++;
    }
  }
  constexpr move_counted_handle(T* ptr_) : ptr(ptr_) {}

  constexpr move_counted_handle& operator=(const move_counted_handle&) = default;

  constexpr T& operator[](size_t i) const { return ptr[i]; }

  static int& move_counter() {
    static int c = 0;
    return c;
  }
};

// non-default constructible accessor with a bunch of different data handles
template <class ElementType>
struct checked_accessor {
  size_t N;
  using offset_policy    = std::default_accessor<ElementType>;
  using element_type     = ElementType;
  using reference        = ElementType&;
  using data_handle_type = move_counted_handle<ElementType>;

  constexpr checked_accessor(size_t N_) : N(N_) {}
  template <class OtherElementType>
    requires(std::is_convertible_v<OtherElementType (*)[], element_type (*)[]>)
  explicit constexpr checked_accessor(const checked_accessor<OtherElementType>& other) noexcept {
    N = other.N;
  }

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return data_handle_type(p.ptr + i);
  }
};

static_assert(std::is_constructible_v<checked_accessor<const int>, const checked_accessor<int>&>);
static_assert(!std::is_convertible_v<const checked_accessor<int>&, checked_accessor<const int>>);

template <>
struct checked_accessor<double> {
  size_t N;
  using offset_policy    = std::default_accessor<double>;
  using element_type     = double;
  using reference        = double&;
  using data_handle_type = no_default_ctor_handle<double>;

  constexpr checked_accessor(size_t N_) : N(N_) {}

  template <class OtherElementType>
    requires(std::is_convertible_v<OtherElementType (*)[], element_type (*)[]>)
  constexpr checked_accessor(checked_accessor<OtherElementType>&& other) noexcept {
    N = other.N;
  }

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p.ptr[i];
  }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p.ptr + i;
  }
};

template <>
struct checked_accessor<unsigned> {
  size_t N;
  using offset_policy    = std::default_accessor<unsigned>;
  using element_type     = unsigned;
  using reference        = unsigned;
  using data_handle_type = not_const_convertible_handle<unsigned>;

  constexpr checked_accessor() : N(0) {}
  constexpr checked_accessor(size_t N_) : N(N_) {}
  constexpr checked_accessor(const checked_accessor& acc) : N(acc.N) {}

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr auto offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p.ptr + i;
  }
};
template <>
struct checked_accessor<const unsigned> {
  size_t N;
  using offset_policy    = std::default_accessor<const unsigned>;
  using element_type     = const unsigned;
  using reference        = unsigned;
  using data_handle_type = not_const_convertible_handle<const unsigned>;

  constexpr checked_accessor() : N(0) {}
  constexpr checked_accessor(size_t N_) : N(N_) {}
  constexpr checked_accessor(const checked_accessor& acc) : N(acc.N) {}

  template <class OtherACC>
  constexpr explicit(std::is_const_v<OtherACC>) checked_accessor(OtherACC&& acc) : N(acc.N) {}

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr auto offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p.ptr + i;
  }
};

template <>
struct checked_accessor<const float> {
  size_t N;
  using offset_policy    = std::default_accessor<const float>;
  using element_type     = const float;
  using reference        = const float&;
  using data_handle_type = move_counted_handle<const float>;

  constexpr checked_accessor() : N(0) {}
  constexpr checked_accessor(size_t N_) : N(N_) {}
  constexpr checked_accessor(const checked_accessor& acc) : N(acc.N) {}

  constexpr checked_accessor(checked_accessor<float>&& acc) : N(acc.N) {}

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return data_handle_type(p.ptr + i);
  }
};

template <>
struct checked_accessor<const double> {
  size_t N;
  using offset_policy    = std::default_accessor<const double>;
  using element_type     = const double;
  using reference        = const double&;
  using data_handle_type = move_counted_handle<const double>;

  constexpr checked_accessor() : N(0) {}
  constexpr checked_accessor(size_t N_) : N(N_) {}
  constexpr checked_accessor(const checked_accessor& acc) : N(acc.N) {}

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return data_handle_type(p.ptr + i);
  }
};

// Data handle pair which has configurable conversion properties
// bool template parameters are used to enable/disable ctors and assignment
// the c is the one for const T the nc for non-const (so we can convert mdspan)
// Note both take non-const T as template parameter though
template <class T, bool, bool, bool, bool>
struct conv_test_accessor_c;

template <class T, bool conv_c, bool conv_nc>
struct conv_test_accessor_nc {
  using offset_policy    = std::default_accessor<T>;
  using element_type     = T;
  using reference        = T&;
  using data_handle_type = T*;

  constexpr conv_test_accessor_nc()                             = default;
  constexpr conv_test_accessor_nc(const conv_test_accessor_nc&) = default;

  template <bool b1, bool b2, bool b3, bool b4>
  constexpr operator conv_test_accessor_c<T, b1, b2, b3, b4>()
    requires(conv_nc)
  {
    return conv_test_accessor_c<T, b1, b2, b3, b4>{};
  }
  template <bool b1, bool b2, bool b3, bool b4>
  constexpr operator conv_test_accessor_c<T, b1, b2, b3, b4>() const
    requires(conv_c)
  {
    return conv_test_accessor_c<T, b1, b2, b3, b4>{};
  }

  constexpr reference access(data_handle_type p, size_t i) const noexcept { return p[i]; }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept { return p + i; }
};

template <class T, bool ctor_c, bool ctor_mv, bool assign_c, bool assign_mv>
struct conv_test_accessor_c {
  using offset_policy    = std::default_accessor<const T>;
  using element_type     = const T;
  using reference        = const T&;
  using data_handle_type = const T*;

  constexpr conv_test_accessor_c()                            = default;
  constexpr conv_test_accessor_c(const conv_test_accessor_c&) = default;

  template <bool b1, bool b2>
  constexpr conv_test_accessor_c(const conv_test_accessor_nc<T, b1, b2>&)
    requires(ctor_c)
  {}
  template <bool b1, bool b2>
  constexpr conv_test_accessor_c(conv_test_accessor_nc<T, b1, b2>&&)
    requires(ctor_mv)
  {}
  template <bool b1, bool b2>
  constexpr conv_test_accessor_c& operator=(const conv_test_accessor_nc<T, b1, b2>&)
    requires(assign_c)
  {
    return {};
  }
  template <bool b1, bool b2>
  constexpr conv_test_accessor_c& operator=(conv_test_accessor_nc<T, b1, b2>&&)
    requires(assign_mv)
  {
    return {};
  }

  constexpr reference access(data_handle_type p, size_t i) const noexcept { return p[i]; }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept { return p + i; }
};

template <class ElementType>
struct convertible_accessor_but_not_handle {
  size_t N;
  using offset_policy    = std::default_accessor<ElementType>;
  using element_type     = ElementType;
  using reference        = ElementType&;
  using data_handle_type = not_const_convertible_handle<element_type>;

  constexpr convertible_accessor_but_not_handle() = default;
  template <class OtherElementType>
    requires(std::is_convertible_v<OtherElementType (*)[], element_type (*)[]>)
  explicit constexpr convertible_accessor_but_not_handle(
      const convertible_accessor_but_not_handle<OtherElementType>& other) noexcept {
    N = other.N;
  }

  constexpr reference access(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return p[i];
  }
  constexpr data_handle_type offset(data_handle_type p, size_t i) const noexcept {
    assert(i < N);
    return data_handle_type(p.ptr + i);
  }
};

#endif // TEST_STD_CONTAINERS_VIEWS_MDSPAN_MDSPAN_CUSTOM_TEST_ACCESSORS_H
