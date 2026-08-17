// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_RECTIFY_CALLBACK_INTERNAL_H_
#define BASE_TEST_RECTIFY_CALLBACK_INTERNAL_H_

#include <utility>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/types/is_instantiation.h"

namespace base::internal {

// RectifyCallbackWrapper binds a wrapper around the actual callback that
// discards the ignored arguments and forwards the remaining arguments to the
// wrapped callback.

template <template <typename> typename CallbackType,
          typename PartialSignature,
          typename IgnoreSignature>
struct RectifyCallbackWrapper;

template <template <typename> typename CallbackType,
          typename R,
          typename... PartialArgs,
          typename... IgnoredArgs>
struct RectifyCallbackWrapper<CallbackType,
                              R(PartialArgs...),
                              void(IgnoredArgs...)> {
  template <typename Actual>
  static CallbackType<R(IgnoredArgs..., PartialArgs...)> Rectify(
      Actual&& callback) {
    if constexpr (is_instantiation<OnceCallback, CallbackType<void()>>) {
      return BindOnce(
          [](OnceCallback<R(PartialArgs...)> callback, IgnoredArgs...,
             PartialArgs... args) {
            return std::move(callback).Run(std::forward<PartialArgs>(args)...);
          },
          std::forward<Actual>(callback));
    } else {
      return BindRepeating(
          [](const RepeatingCallback<R(PartialArgs...)> callback,
             IgnoredArgs..., PartialArgs... args) {
            return callback.Run(std::forward<PartialArgs>(args)...);
          },
          std::forward<Actual>(callback));
    }
  }
};

// RectifyCallbackSplitter is a helper that returns the first N args of
// `Signature` as the type `void(Args...)`. These are the arguments that need
// to be ignored when rectifying a callback.

template <size_t count, typename Signature, typename Result = void()>
struct RectifyCallbackSplitter;

template <typename Arg, typename... Args, typename... Results>
struct RectifyCallbackSplitter<0, void(Arg, Args...), void(Results...)> {
  using type = void(Results...);
};

template <typename... Args, typename... Results>
struct RectifyCallbackSplitter<0, void(Args...), void(Results...)> {
  using type = void(Results...);
};

template <size_t count, typename Arg, typename... Args, typename... Results>
struct RectifyCallbackSplitter<count, void(Arg, Args...), void(Results...)>
    : RectifyCallbackSplitter<count - 1, void(Args...), void(Results..., Arg)> {
};

// Given a desired type and an actual type, RectifyCallbackImpl provides a
// Rectify() method that adapts the input callback to be callable using the
// arguments from desired type.

template <typename DesiredType, typename ActualType, typename SFINAE = void>
struct RectifyCallbackImpl;

// Main specialization that handles the case where the desired and actual types
// have already been normalized into callback types. The other specializations
// allow additional flexibility for callers, but eventually all delegate here.
template <template <typename> typename DesiredCallbackType,
          template <typename>
          typename ActualCallbackType,
          typename R,
          typename... DesiredArgs,
          typename... ActualArgs>
struct RectifyCallbackImpl<DesiredCallbackType<R(DesiredArgs...)>,
                           ActualCallbackType<R(ActualArgs...)>> {
  static DesiredCallbackType<R(DesiredArgs...)> Rectify(
      ActualCallbackType<R(ActualArgs...)> callback) {
    if constexpr (std::is_same_v<R(DesiredArgs...), R(ActualArgs...)>) {
      // No adapting needed when the parameter lists already match.
      return callback;
    }

    // For uniformity, if the input callback is null, the output callback should
    // be null as well.
    if (!callback) {
      return NullCallback();
    }

    using IgnoreSignature =
        typename RectifyCallbackSplitter<sizeof...(DesiredArgs) -
                                             sizeof...(ActualArgs),
                                         void(DesiredArgs...)>::type;
    return RectifyCallbackWrapper<
        DesiredCallbackType, R(ActualArgs...),
        IgnoreSignature>::Rectify(std::move(callback));
  }
};

// Specialization that handles a type signature as the desired type. The output
// in this case will be based on the type of callback passed in.
template <typename R,
          typename... Args,
          template <typename>
          typename ActualCallbackType,
          typename ActualSignature>
struct RectifyCallbackImpl<R(Args...), ActualCallbackType<ActualSignature>>
    : RectifyCallbackImpl<ActualCallbackType<R(Args...)>,
                          ActualCallbackType<ActualSignature>> {};

// Fallback for things like DoNothing(), NullCallback(), etc. where a specific
// callback return type is provided.
template <template <typename> typename DesiredCallbackType,
          typename R,
          typename... Args,
          typename T>
struct RectifyCallbackImpl<DesiredCallbackType<R(Args...)>, T>
    : RectifyCallbackImpl<DesiredCallbackType<R(Args...)>,
                          DesiredCallbackType<R(Args...)>> {};

// Fallback for things like DoNothing(), NullCallback(), etc. where only the
// signature of the return type is provided. In this case, `RepeatingCallback`
// is implicitly generated, as it can be used as both a `OnceCallback` or
// `RepeatingCallback`.
template <typename R, typename... Args, typename T>
struct RectifyCallbackImpl<R(Args...), T>
    : RectifyCallbackImpl<RepeatingCallback<R(Args...)>,
                          RepeatingCallback<R(Args...)>> {};

}  // namespace base::internal

#endif  // BASE_TEST_RECTIFY_CALLBACK_INTERNAL_H_
