/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This implementation is borrowed from Chromium.

#ifndef RTC_BASE_CONTAINERS_INVOKE_H_
#define RTC_BASE_CONTAINERS_INVOKE_H_

#include <type_traits>
#include <utility>

namespace webrtc {

namespace invoke_internal {

// Helper struct and alias to deduce the class type from a member function
// pointer or member object pointer.
template <typename DecayedF>
struct member_pointer_class {};

template <typename ReturnT, typename ClassT>
struct member_pointer_class<ReturnT ClassT::*> {
  using type = ClassT;
};

template <typename DecayedF>
using member_pointer_class_t = typename member_pointer_class<DecayedF>::type;

// Utility struct to detect specializations of std::reference_wrapper.
template <typename T>
struct is_reference_wrapper : std::false_type {};

template <typename T>
struct is_reference_wrapper<std::reference_wrapper<T>> : std::true_type {};

// Small helpers used below in invoke_internal::invoke to make the SFINAE more
// concise.
template <typename F>
const bool& IsMemFunPtr =
    std::is_member_function_pointer<std::decay_t<F>>::value;

template <typename F>
const bool& IsMemObjPtr = std::is_member_object_pointer<std::decay_t<F>>::value;

template <typename F,
          typename T,
          typename MemPtrClass = member_pointer_class_t<std::decay_t<F>>>
const bool& IsMemPtrToBaseOf =
    std::is_base_of<MemPtrClass, std::decay_t<T>>::value;

template <typename T>
const bool& IsRefWrapper = is_reference_wrapper<std::decay_t<T>>::value;

template <bool B>
using EnableIf = std::enable_if_t<B, bool>;

// Invokes a member function pointer on a reference to an object of a suitable
// type. Covers bullet 1 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.1
template <typename F,
          typename T1,
          typename... Args,
          EnableIf<IsMemFunPtr<F> && IsMemPtrToBaseOf<F, T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1, Args&&... args) {
  return (std::forward<T1>(t1).*f)(std::forward<Args>(args)...);
}

// Invokes a member function pointer on a std::reference_wrapper to an object of
// a suitable type. Covers bullet 2 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.2
template <typename F,
          typename T1,
          typename... Args,
          EnableIf<IsMemFunPtr<F> && IsRefWrapper<T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1, Args&&... args) {
  return (t1.get().*f)(std::forward<Args>(args)...);
}

// Invokes a member function pointer on a pointer-like type to an object of a
// suitable type. Covers bullet 3 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.3
template <typename F,
          typename T1,
          typename... Args,
          EnableIf<IsMemFunPtr<F> && !IsMemPtrToBaseOf<F, T1> &&
                   !IsRefWrapper<T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1, Args&&... args) {
  return ((*std::forward<T1>(t1)).*f)(std::forward<Args>(args)...);
}

// Invokes a member object pointer on a reference to an object of a suitable
// type. Covers bullet 4 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.4
template <typename F,
          typename T1,
          EnableIf<IsMemObjPtr<F> && IsMemPtrToBaseOf<F, T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1) {
  return std::forward<T1>(t1).*f;
}

// Invokes a member object pointer on a std::reference_wrapper to an object of
// a suitable type. Covers bullet 5 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.5
template <typename F,
          typename T1,
          EnableIf<IsMemObjPtr<F> && IsRefWrapper<T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1) {
  return t1.get().*f;
}

// Invokes a member object pointer on a pointer-like type to an object of a
// suitable type. Covers bullet 6 of the INVOKE definition.
//
// Reference: https://wg21.link/func.require#1.6
template <typename F,
          typename T1,
          EnableIf<IsMemObjPtr<F> && !IsMemPtrToBaseOf<F, T1> &&
                   !IsRefWrapper<T1>> = true>
constexpr decltype(auto) InvokeImpl(F&& f, T1&& t1) {
  return (*std::forward<T1>(t1)).*f;
}

// Invokes a regular function or function object. Covers bullet 7 of the INVOKE
// definition.
//
// Reference: https://wg21.link/func.require#1.7
template <typename F, typename... Args>
constexpr decltype(auto) InvokeImpl(F&& f, Args&&... args) {
  return std::forward<F>(f)(std::forward<Args>(args)...);
}

}  // namespace invoke_internal

// Implementation of C++17's std::invoke. This is not based on implementation
// referenced in original std::invoke proposal, but rather a manual
// implementation, so that it can be constexpr.
//
// References:
// - https://wg21.link/n4169#implementability
// - https://en.cppreference.com/w/cpp/utility/functional/invoke
// - https://wg21.link/func.invoke
template <typename F, typename... Args>
constexpr decltype(auto) invoke(F&& f, Args&&... args) {
  return invoke_internal::InvokeImpl(std::forward<F>(f),
                                     std::forward<Args>(args)...);
}

}  // namespace webrtc

#endif  // RTC_BASE_CONTAINERS_INVOKE_H_
