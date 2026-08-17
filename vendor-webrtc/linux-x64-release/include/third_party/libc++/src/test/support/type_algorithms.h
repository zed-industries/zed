//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_ALGORITHMS_H
#define TEST_SUPPORT_TYPE_ALGORITHMS_H

#include <type_traits>

#include "test_macros.h"

namespace types {
template <class... Types>
struct type_list {};

// concatenates N type_lists to one (for N >= 1)
template <class...>
struct concatenate;

template <class... Types>
using concatenate_t = typename concatenate<Types...>::type;

// for_each takes a type_list calls f with each element as the first template argument
template <class... Types, class Functor>
TEST_CONSTEXPR_CXX14 void for_each(type_list<Types...>, Functor f);

// impl
template <class... Types>
struct concatenate<type_list<Types...> > {
  using type = type_list<Types...>;
};

template <class... Types1, class... Types2>
struct concatenate<type_list<Types1...>, type_list<Types2...> > {
  using type = type_list<Types1..., Types2...>;
};

template <class... Types1, class... Types2, class... Rest>
struct concatenate<type_list<Types1...>, type_list<Types2...>, Rest...> {
  using type = concatenate_t<type_list<Types1..., Types2...>, Rest...>;
};

template <class... Types>
TEST_CONSTEXPR_CXX14 void swallow(Types...) {}

template <class... Types, class Functor>
TEST_CONSTEXPR_CXX14 void for_each(type_list<Types...>, Functor f) {
  swallow((f.template operator()<Types>(), 0)...);
}

template <class T>
struct type_identity {
  using type = T;
};

#if TEST_STD_VER >= 17
template <class Func>
struct apply_type_identity {
  Func func_;

  apply_type_identity(Func func) : func_(func) {}

  template <class... Args>
  decltype(auto) operator()() const {
    return func_(type_identity<Args>{}...);
  }
};

template <class T>
apply_type_identity(T) -> apply_type_identity<T>;

#endif
template <template <class...> class T, class... Args>
struct partial_instantiation {
  template <class Other>
  using apply = T<Args..., Other>;
};

// type categories defined in [basic.fundamental] plus extensions (without CV-qualifiers)

using character_types =
    type_list<char
#ifndef TEST_HAS_NO_WIDE_CHARACTERS
              ,
              wchar_t
#endif
#ifndef TEST_HAS_NO_CHAR8_T
              ,
              char8_t
#endif
#if TEST_STD_VER >= 11
              ,
              char16_t,
              char32_t
#endif
              >;

using signed_integer_types =
    type_list<signed char,
              short,
              int,
              long,
              long long
#ifndef TEST_HAS_NO_INT128
              ,
              __int128_t
#endif
              >;

using unsigned_integer_types =
    type_list<unsigned char,
              unsigned short,
              unsigned int,
              unsigned long,
              unsigned long long
#ifndef TEST_HAS_NO_INT128
              ,
              __uint128_t
#endif
              >;

using integer_types = concatenate_t<character_types, signed_integer_types, unsigned_integer_types>;

using integral_types = concatenate_t<integer_types, type_list<bool> >;

using floating_point_types = type_list<float, double, long double>;

using arithmetic_types = concatenate_t<integral_types, floating_point_types>;

template <class T>
using cv_qualified_versions = type_list<T, const T, volatile T, const volatile T>;

template <class T>
struct type_list_as_pointers;

template <class... Types>
struct type_list_as_pointers<type_list<Types...> > {
  using type = type_list<Types*...>;
};

template <class T>
using as_pointers = typename type_list_as_pointers<T>::type;
} // namespace types

#endif // TEST_SUPPORT_TYPE_ALGORITHMS_H
