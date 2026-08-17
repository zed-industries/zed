// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_LIB_PROMISE_CONTEXT_H
#define GRPC_SRC_CORE_LIB_PROMISE_CONTEXT_H

#include <grpc/support/port_platform.h>

#include <utility>

#include "absl/log/check.h"
#include "absl/meta/type_traits.h"
#include "src/core/util/down_cast.h"

namespace grpc_core {

// To avoid accidentally creating context types, we require an explicit
// specialization of this template per context type. The specialization need
// not contain any members, only exist.
// The reason for avoiding this is that context types each use a thread local.
template <typename T>
struct ContextType;

// Some contexts can be subclassed. If the subclass is set as that context
// then GetContext<Base>() will return the base, and GetContext<Derived>() will
// DownCast to the derived type.
// Specializations of this type should be created for each derived type, and
// should have a single using statement Base pointing to the derived base class.
// Example:
//  class SomeContext {};
//  class SomeDerivedContext : public SomeContext {};
//  template <> struct ContextType<SomeContext> {};
//  template <> struct ContextSubclass<SomeDerivedContext> {
//    using Base = SomeContext;
//  };
template <typename Derived>
struct ContextSubclass;

namespace promise_detail {

template <typename T, typename = void>
class Context;

template <typename T>
class ThreadLocalContext : public ContextType<T> {
 public:
  explicit ThreadLocalContext(T* p) : old_(current_) { current_ = p; }
  ~ThreadLocalContext() { current_ = old_; }
  ThreadLocalContext(const ThreadLocalContext&) = delete;
  ThreadLocalContext& operator=(const ThreadLocalContext&) = delete;

  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static T* get() { return current_; }

 private:
  T* const old_;
  static thread_local T* current_;
};

template <typename T>
thread_local T* ThreadLocalContext<T>::current_;

template <typename T>
class Context<T, absl::void_t<decltype(ContextType<T>())>>
    : public ThreadLocalContext<T> {
  using ThreadLocalContext<T>::ThreadLocalContext;
};

template <typename T>
class Context<T, absl::void_t<typename ContextSubclass<T>::Base>>
    : public Context<typename ContextSubclass<T>::Base> {
 public:
  using Context<typename ContextSubclass<T>::Base>::Context;
  GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION static T* get() {
    return DownCast<T*>(Context<typename ContextSubclass<T>::Base>::get());
  }
};

template <typename T, typename F>
class WithContext {
 public:
  WithContext(F f, T* context) : context_(context), f_(std::move(f)) {}

  decltype(std::declval<F>()()) operator()() {
    Context<T> ctx(context_);
    return f_();
  }

 private:
  T* context_;
  F f_;
};

}  // namespace promise_detail

// Return true if a context of type T is currently active.
template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline bool HasContext() {
  return promise_detail::Context<T>::get() != nullptr;
}

// Retrieve the current value of a context, or abort if the value is unset.
template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline T* GetContext() {
  auto* p = promise_detail::Context<T>::get();
  DCHECK_NE(p, nullptr);
  return p;
}

// Retrieve the current value of a context, or nullptr if the value is unset.
template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline T* MaybeGetContext() {
  return promise_detail::Context<T>::get();
}

template <typename T>
GPR_ATTRIBUTE_ALWAYS_INLINE_FUNCTION inline void SetContext(T* p) {
  promise_detail::Context<T>::set(p);
}

// Given a promise and a context, return a promise that has that context set.
template <typename T, typename F>
promise_detail::WithContext<T, F> WithContext(F f, T* context) {
  return promise_detail::WithContext<T, F>(std::move(f), context);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_CONTEXT_H
