/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_FUNCTION_VIEW_H_
#define API_FUNCTION_VIEW_H_

#include <cstddef>
#include <type_traits>
#include <utility>

#include "rtc_base/checks.h"

// Just like std::function, FunctionView will wrap any callable and hide its
// actual type, exposing only its signature. But unlike std::function,
// FunctionView doesn't own its callable---it just points to it. Thus, it's a
// good choice mainly as a function argument when the callable argument will
// not be called again once the function has returned.
//
// Its constructors are implicit, so that callers won't have to convert lambdas
// and other callables to FunctionView<Blah(Blah, Blah)> explicitly. This is
// safe because FunctionView is only a reference to the real callable.
//
// Example use:
//
//   void SomeFunction(webrtc::FunctionView<int(int)> index_transform);
//   ...
//   SomeFunction([](int i) { return 2 * i + 1; });
//
// Note: FunctionView is tiny (essentially just two pointers) and trivially
// copyable, so it's probably cheaper to pass it by value than by const
// reference.

namespace webrtc {

template <typename T>
class FunctionView;  // Undefined.

template <typename RetT, typename... ArgT>
class FunctionView<RetT(ArgT...)> final {
 public:
  // Constructor for lambdas and other callables; it accepts every type of
  // argument except those noted in its enable_if call.
  template <
      typename F,
      typename std::enable_if<
          // Not for function pointers; we have another constructor for that
          // below.
          !std::is_function<typename std::remove_pointer<
              typename std::remove_reference<F>::type>::type>::value &&

          // Not for nullptr; we have another constructor for that below.
          !std::is_same<std::nullptr_t,
                        typename std::remove_cv<F>::type>::value &&

          // Not for FunctionView objects; we have another constructor for that
          // (the implicitly declared copy constructor).
          !std::is_same<FunctionView,
                        typename std::remove_cv<typename std::remove_reference<
                            F>::type>::type>::value>::type* = nullptr>
  FunctionView(F&& f)
      : call_(CallVoidPtr<typename std::remove_reference<F>::type>) {
    f_.void_ptr = &f;
  }

  // Constructor that accepts function pointers. If the argument is null, the
  // result is an empty FunctionView.
  template <
      typename F,
      typename std::enable_if<std::is_function<typename std::remove_pointer<
          typename std::remove_reference<F>::type>::type>::value>::type* =
          nullptr>
  FunctionView(F&& f)
      : call_(f ? CallFunPtr<typename std::remove_pointer<F>::type> : nullptr) {
    f_.fun_ptr = reinterpret_cast<void (*)()>(f);
  }

  // Constructor that accepts nullptr. It creates an empty FunctionView.
  template <typename F,
            typename std::enable_if<std::is_same<
                std::nullptr_t,
                typename std::remove_cv<F>::type>::value>::type* = nullptr>
  FunctionView(F&& /* f */) : call_(nullptr) {}

  // Default constructor. Creates an empty FunctionView.
  FunctionView() : call_(nullptr) {}

  RetT operator()(ArgT... args) const {
    RTC_DCHECK(call_);
    return call_(f_, std::forward<ArgT>(args)...);
  }

  // Returns true if we have a function, false if we don't (i.e., we're null).
  explicit operator bool() const { return !!call_; }

 private:
  union VoidUnion {
    void* void_ptr;
    void (*fun_ptr)();
  };

  template <typename F>
  static RetT CallVoidPtr(VoidUnion vu, ArgT... args) {
    return (*static_cast<F*>(vu.void_ptr))(std::forward<ArgT>(args)...);
  }
  template <typename F>
  static RetT CallFunPtr(VoidUnion vu, ArgT... args) {
    return (reinterpret_cast<typename std::add_pointer<F>::type>(vu.fun_ptr))(
        std::forward<ArgT>(args)...);
  }

  // A pointer to the callable thing, with type information erased. It's a
  // union because we have to use separate types depending on if the callable
  // thing is a function pointer or something else.
  VoidUnion f_;

  // Pointer to a dispatch function that knows the type of the callable thing
  // that's stored in f_, and how to call it. A FunctionView object is empty
  // (null) iff call_ is null.
  RetT (*call_)(VoidUnion, ArgT...);
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::FunctionView;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // API_FUNCTION_VIEW_H_
