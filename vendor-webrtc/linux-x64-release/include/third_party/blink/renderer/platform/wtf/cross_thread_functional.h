// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_FUNCTIONAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_FUNCTIONAL_H_

#include <type_traits>

#include "base/functional/bind.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace WTF {

// `CrossThreadBindOnce()` and `CrossThreadBindRepeating()` are the Blink
// equivalents of `base::BindOnce()` and `base::BindRepeating()` for creating
// a callback that is run or destroyed on a different thread.
//
// Unlike `base::RepeatingCallback`, a repeatable cross-thread function is *not*
// copyable. This is intentional: a number of objects in Blink are
// thread-hostile: allowing a cross-thread function to be copied
// means it would be easy to end up in situations where multiple threads might
// unsafely reference the same object.
//
// TODO(crbug.com/963574): Deprecate `CrossThreadBindRepeating()`.
//
// Example:
// // Given the prototype:
// // void MyFunction(const ThreadUnsafeObject&, int);
// ThreadUnsafeObject obj;
// CrossThreadFunction<void(int)> f =
//     CrossThreadBindOnce(&MyFunction, obj);
// std::move(f).Run(42);  // Calls MyFunction(<deep copy of `obj`>, 42);
//
// Arguments bound to a `CrossThreadFunction` are copied with
// `CrossThreadCopier`.
//
// Important!
// `CrossThreadBindOnce(obj)` is similar to `BindOnce(<deep copy of `obj`>)`,
// but historically, the latter was unsafe since it was possible to end up with
// situations where a thread-hostile object would be referenced on multiple
// threads, leading to crashes. See https://crbug.com/390851 for more details.
//
// In contrast, `CrossThreadBindOnce()` and `CrossThreadBindRepeating()` are
// implemented in a way that only the destination thread can refer to any bound
// arguments.

namespace internal {

template <typename Signature>
auto MakeCrossThreadFunction(base::RepeatingCallback<Signature> callback) {
  return CrossThreadFunction<Signature>(std::move(callback));
}

template <typename Signature>
auto MakeCrossThreadOnceFunction(base::OnceCallback<Signature> callback) {
  return CrossThreadOnceFunction<Signature>(std::move(callback));
}

// Insertion of coercion for specific types; transparent forwarding otherwise.

template <typename T>
decltype(auto) CoerceFunctorForCrossThreadBind(T&& functor) {
  return std::forward<T>(functor);
}

template <typename Signature>
base::RepeatingCallback<Signature> CoerceFunctorForCrossThreadBind(
    CrossThreadFunction<Signature>&& functor) {
  return ConvertToBaseRepeatingCallback(std::move(functor));
}

template <typename Signature>
base::OnceCallback<Signature> CoerceFunctorForCrossThreadBind(
    CrossThreadOnceFunction<Signature>&& functor) {
  return ConvertToBaseOnceCallback(std::move(functor));
}

}  // namespace internal

template <typename FunctionType, typename... Ps>
auto CrossThreadBindRepeating(FunctionType&& function, Ps&&... parameters) {
  static_assert(
      internal::CheckGCedTypeRestrictions<std::index_sequence_for<Ps...>,
                                          std::decay_t<Ps>...>::ok,
      "A bound argument uses a bad pattern.");
  return internal::MakeCrossThreadFunction(
      base::BindRepeating(internal::CoerceFunctorForCrossThreadBind(
                              std::forward<FunctionType>(function)),
                          CrossThreadCopier<std::decay_t<Ps>>::Copy(
                              std::forward<Ps>(parameters))...));
}

template <typename FunctionType, typename... Ps>
auto CrossThreadBindOnce(FunctionType&& function, Ps&&... parameters) {
  static_assert(
      internal::CheckGCedTypeRestrictions<std::index_sequence_for<Ps...>,
                                          std::decay_t<Ps>...>::ok,
      "A bound argument uses a bad pattern.");
  return internal::MakeCrossThreadOnceFunction(
      base::BindOnce(internal::CoerceFunctorForCrossThreadBind(
                         std::forward<FunctionType>(function)),
                     CrossThreadCopier<std::decay_t<Ps>>::Copy(
                         std::forward<Ps>(parameters))...));
}

}  // namespace WTF

using WTF::CrossThreadBindOnce;
using WTF::CrossThreadBindRepeating;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_FUNCTIONAL_H_
