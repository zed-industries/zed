// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_RECTIFY_CALLBACK_H_
#define BASE_TEST_RECTIFY_CALLBACK_H_

#include <utility>

#include "base/test/rectify_callback_internal.h"

namespace base {

// RectifyCallback:
//
//     CallbackType<DesiredSignature> RectifyCallback<DesiredSignature>(
//         CallbackType<ActualSignature> callback)
//
//     DesiredCallbackType RectifyCallback<DesiredCallbackType>(
//         ActualCallbackType callback)
//
// Rectifies the signature of `callback` with `DesiredSignature` or
// `DesiredCallbackType` by ignoring the first N arguments of the desired
// callback type. Useful when binding callbacks with lots of arguments you don't
// actually care about.
//
// For now, `ActualSignature` and `DesiredSignature` must have the same return
// type, and the common arguments between the two must match.
//
// Example:
//
//    using CbType = OnceCallback<bool(A, B, C)>;
//    void Fn(CbType);
//
//    // These all ignore arguments when passing the callback:
//    Fn(RectifyCallback<CbType>(BindOnce([]{ return true; })));
//    Fn(RectifyCallback<CbType>(BindOnce([](C c){ return true; })));
//    Fn(RectifyCallback<CbType>(BindOnce([](B c, C c){ return true; })));
//
//    // This also works, though it makes no change to the input callback:
//    Fn(RectifyCallback<CbType>(
//        BindOnce([](A a, B c, C c){ return true; })));
//
// You can also make RectifyCallback implicit by embedding it in a template
// version of your function.
//
//    template <typename T>
//    void Fn(T&& t) { FnImpl(RectifyCallback<CbType>(std::forward<T>(t))); }
//
template <typename Desired, typename Actual>
auto RectifyCallback(Actual&& callback) {
  using Impl = internal::RectifyCallbackImpl<Desired, std::decay_t<Actual>>;
  return Impl::Rectify(std::move(callback));
}

}  // namespace base

#endif  // BASE_TEST_RECTIFY_CALLBACK_H_
