// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_HPB_TEMPLATE_HELP_H__
#define GOOGLE_PROTOBUF_HPB_TEMPLATE_HELP_H__

#include <type_traits>

#include "google/protobuf/hpb/ptr.h"

namespace hpb {
namespace internal {

template <typename T>
struct RemovePtr;

template <typename T>
struct RemovePtr<Ptr<T>> {
  using type = T;
};

template <typename T>
struct RemovePtr<T*> {
  using type = T;
};

template <typename T>
using RemovePtrT = typename RemovePtr<T>::type;

template <typename T, typename U = RemovePtrT<T>,
          typename = std::enable_if_t<!std::is_const_v<U>>>
using PtrOrRawMutable = T;

template <typename T, typename U = RemovePtrT<T>>
using PtrOrRaw = T;

template <typename T, typename = void>
inline constexpr bool IsHpbClass = false;

template <typename T>
inline constexpr bool
    IsHpbClass<T, std::enable_if_t<std::is_base_of_v<typename T::Access, T>>> =
        true;

template <typename T>
using EnableIfHpbClass = std::enable_if_t<IsHpbClass<T>>;

template <typename T, typename = void>
inline constexpr bool IsHpbClassThatHasExtensions = false;

template <typename T>
inline constexpr bool IsHpbClassThatHasExtensions<
    T, std::enable_if_t<std::is_base_of_v<typename T::Access, T> &&
                        std::is_base_of_v<typename T::ExtendableType, T>>> =
    true;

template <typename T>
using EnableIfHpbClassThatHasExtensions =
    std::enable_if_t<IsHpbClassThatHasExtensions<T>>;

template <typename T>
using EnableIfMutableProto = std::enable_if_t<!std::is_const<T>::value>;

template <typename T, typename T2>
using add_const_if_T_is_const =
    std::conditional_t<std::is_const_v<T>, const T2, T2>;

}  // namespace internal
}  // namespace hpb

#endif  // GOOGLE_PROTOBUF_HPB_TEMPLATE_HELP_H__
