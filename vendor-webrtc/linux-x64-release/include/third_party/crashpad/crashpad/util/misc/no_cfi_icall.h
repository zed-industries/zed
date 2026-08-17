// Copyright 2020 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_MISC_NO_CFI_ICALL_H_
#define CRASHPAD_UTIL_MISC_NO_CFI_ICALL_H_

#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include <windows.h>
#endif  // BUILDFLAG(IS_WIN)

namespace crashpad {

namespace {

template <typename Functor>
struct FunctorTraits;

template <typename R, typename... Args>
struct FunctorTraits<R (*)(Args...) noexcept> {
  template <typename Function, typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(Function&& function, RunArgs&&... args) {
    return std::forward<Function>(function)(std::forward<RunArgs>(args)...);
  }
};

template <typename R, typename... Args>
struct FunctorTraits<R (*)(Args..., ...) noexcept> {
  template <typename Function, typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(Function&& function, RunArgs&&... args) {
    return std::forward<Function>(function)(std::forward<RunArgs>(args)...);
  }
};

#if BUILDFLAG(IS_WIN) && defined(ARCH_CPU_X86)
template <typename R, typename... Args>
struct FunctorTraits<R(__stdcall*)(Args...) noexcept> {
  template <typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(R(__stdcall* function)(Args...),
                                    RunArgs&&... args) {
    return function(std::forward<RunArgs>(args)...);
  }
};
#endif  // BUILDFLAG(IS_WIN) && ARCH_CPU_X86

#if __cplusplus >= 201703L
// These specializations match functions which are not explicitly declared
// noexcept. They must only be present at C++17 when noexcept is part of a
// function's type. If they are present earlier, they redefine the
// specializations above.
template <typename R, typename... Args>
struct FunctorTraits<R (*)(Args...)> {
  template <typename Function, typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(Function&& function, RunArgs&&... args) {
    return std::forward<Function>(function)(std::forward<RunArgs>(args)...);
  }
};

template <typename R, typename... Args>
struct FunctorTraits<R (*)(Args..., ...)> {
  template <typename Function, typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(Function&& function, RunArgs&&... args) {
    return std::forward<Function>(function)(std::forward<RunArgs>(args)...);
  }
};

#if BUILDFLAG(IS_WIN) && defined(ARCH_CPU_X86)
template <typename R, typename... Args>
struct FunctorTraits<R(__stdcall*)(Args...)> {
  template <typename... RunArgs>
  DISABLE_CFI_ICALL static R Invoke(R(__stdcall* function)(Args...),
                                    RunArgs&&... args) {
    return function(std::forward<RunArgs>(args)...);
  }
};
#endif  // BUILDFLAG(IS_WIN) && ARCH_CPU_X86

#endif  // __cplusplus >= 201703L

}  // namespace

//! \brief Disables cfi-icall for calls made through a function pointer.
//!
//! Clang provides several Control-Flow-Integrity (CFI) sanitizers, among them,
//! cfi-icall, which attempts to verify that the dynamic type of a function
//! matches the static type of the function pointer used to call it.
//!
//! https://clang.llvm.org/docs/ControlFlowIntegrity.html#indirect-function-call-checking
//!
//! However, cfi-icall does not have enough information to check indirect calls
//! to functions in other modules, such as through the pointers returned by
//! `dlsym()`. In these cases, CFI aborts the program upon executing the
//! indirect call.
//!
//! This class encapsulates cross-DSO function pointers to disable cfi-icall
//! precisely when calling these pointers.
template <typename Functor>
class NoCfiIcall {
 public:
  //! \brief Constructs this object.
  //!
  //! \param function A pointer to the function to be called.
  explicit NoCfiIcall(Functor function) : function_(function) {}

  //! \see NoCfiIcall
  NoCfiIcall() : function_(static_cast<Functor>(nullptr)) {}

  //! \see NoCfiIcall
  template <typename PointerType,
            typename = std::enable_if_t<
                std::is_same<typename std::remove_cv<PointerType>::type,
                             void*>::value>>
  explicit NoCfiIcall(PointerType function)
      : function_(reinterpret_cast<Functor>(function)) {}

#if BUILDFLAG(IS_WIN)
  //! \see NoCfiIcall
  template <typename = std::enable_if_t<
                !std::is_same<typename std::remove_cv<Functor>::type,
                              FARPROC>::value>>
  explicit NoCfiIcall(FARPROC function)
      : function_(reinterpret_cast<Functor>(function)) {}
#endif  // BUILDFLAG(IS_WIN)

  ~NoCfiIcall() = default;

  //! \brief Updates the pointer to the function to be called.
  //!
  //! \param function A pointer to the function to be called.
  void SetPointer(Functor function) { function_ = function; }

  //! \see SetPointer
  template <typename PointerType,
            typename = std::enable_if_t<
                std::is_same<typename std::remove_cv<PointerType>::type,
                             void*>::value>>
  void SetPointer(PointerType function) {
    function_ = reinterpret_cast<Functor>(function);
  }

  //! \brief Calls the function without sanitization by cfi-icall.
  template <typename... RunArgs>
  decltype(auto) operator()(RunArgs&&... args) const {
    return FunctorTraits<Functor>::Invoke(function_,
                                          std::forward<RunArgs>(args)...);
  }

  //! \brief Returns `true` if not `nullptr`.
  operator bool() const { return function_ != nullptr; }

 private:
  Functor function_;
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_MISC_NO_CFI_ICALL_H_
