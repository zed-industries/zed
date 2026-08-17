//===----- unique_function.h - moveable type-erasing function ---*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
///
/// unique_function works like std::function, but supports move-only callable
/// objects.
///
/// TODO: Use LLVM's unique_function (llvm/include/llvm/ADT/FunctionExtras.h),
///       which uses some extra inline storage to avoid heap allocations for
///       small objects. Using LLVM's unique_function will require first
///       porting some other utilities like PointerIntPair, PointerUnion, and
///       PointerLikeTypeTraits. (These are likely to be independently useful
///       in the orc runtime, so porting will have additional benefits).
///
//===----------------------------------------------------------------------===//

#ifndef ORC_RT_UNIQUE_FUNCTION_H
#define ORC_RT_UNIQUE_FUNCTION_H

#include <memory>

namespace orc_rt {

namespace unique_function_detail {

template <typename RetT, typename... ArgTs> class Callable {
public:
  virtual ~Callable() = default;
  virtual RetT call(ArgTs &&...Args) = 0;
};

template <typename CallableT, typename RetT, typename... ArgTs>
class CallableImpl : public Callable<RetT, ArgTs...> {
public:
  CallableImpl(CallableT &&Callable) : Callable(std::move(Callable)) {}
  RetT call(ArgTs &&...Args) override {
    return Callable(std::forward<ArgTs>(Args)...);
  }

private:
  CallableT Callable;
};

} // namespace unique_function_detail

template <typename FnT> class unique_function;

template <typename RetT, typename... ArgTs>
class unique_function<RetT(ArgTs...)> {
public:
  unique_function() = default;
  unique_function(std::nullptr_t) {}
  unique_function(unique_function &&) = default;
  unique_function(const unique_function &&) = delete;
  unique_function &operator=(unique_function &&) = default;
  unique_function &operator=(const unique_function &&) = delete;

  template <typename CallableT>
  unique_function(CallableT &&Callable)
      : C(std::make_unique<
            unique_function_detail::CallableImpl<CallableT, RetT, ArgTs...>>(
                std::forward<CallableT>(Callable))) {}

  RetT operator()(ArgTs... Params) {
    return C->call(std::forward<ArgTs>(Params)...);
  }

  explicit operator bool() const { return !!C; }

private:
  std::unique_ptr<unique_function_detail::Callable<RetT, ArgTs...>> C;
};

} // namespace orc_rt

#endif // ORC_RT_UNIQUE_FUNCTION_H
