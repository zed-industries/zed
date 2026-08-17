// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_EXPECTED_MACROS_H_
#define BASE_TYPES_EXPECTED_MACROS_H_

#include <concepts>
#include <functional>
#include <optional>
#include <string_view>
#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "base/macros/concat.h"
#include "base/macros/if.h"
#include "base/macros/is_empty.h"
#include "base/macros/remove_parens.h"
#include "base/macros/uniquify.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/types/expected.h"
#include "base/types/is_instantiation.h"

// Executes an expression `rexpr` that returns an `expected<T, E>` or
// `std::optional<T>`.
//
// For the `expected<T, E>` case:
//   If the result is an error, causes the calling function to return. If no
//   additional arguments are given, the function return value is the `E`
//   returned by `rexpr`. Otherwise, the additional arguments are treated as an
//   invocable that expects an E as its last argument and returns some type
//   (including `void`) convertible to the function's return type; that is, the
//   function returns the result of `std::invoke(..., E)` on the additional
//   arguments.
//
//   This works with move-only types and can be used in functions that return
//   either an `E` directly or a `base::expected<U, E>`, without needing to
//   explicitly wrap the return in `base::unexpected`.
//
// For the `std::optional<T>` case:
//   If the result is `std::nullopt`, causes the calling function to return. If
//   no additional arguments are given, the function return value is the return
//   value of `rexpr` (i.e. an unbound `std::optional<T>`). Otherwise, the
//   additional arguments are treated as an invocable that returns some type
//   (including `void`) convertible to the function's return type; that is, the
//   function returns the result of `std::invoke(...)` on the additional
//   arguments.
//
// # Interface
//
// `RETURN_IF_ERROR(rexpr, ...);`
//
// # Examples for the `expected<T, E>` case
//
// Use with no additional arguments:
// ```
//   SomeErrorCode Foo() {
//     RETURN_IF_ERROR(Function(args...));
//     return SomeErrorCode::kOK;
//   }
// ```
// ```
//   base::expected<int, SomeErrorCode> Bar() {
//     RETURN_IF_ERROR(Function(args...));
//     RETURN_IF_ERROR(obj.Method(args...));
//     return 17;
//   }
// ```
//
// Adjusting the returned error:
// ```
//   RETURN_IF_ERROR(TryProcessing(query),
//                   [](const auto& e) {
//                     return base::StrCat({e, " while processing query"});
//                   });
// ```
//
// Returning a different kind of error:
// ```
//   RETURN_IF_ERROR(TryProcessing(query),
//                   [](auto) { return SomeErrorCode::kFail); });
// ```
//
// Returning void:
// ```
//   RETURN_IF_ERROR(TryProcessing(query), [](auto) {});
// ```
// ```
//   RETURN_IF_ERROR(TryProcessing(query),
//                   [](auto) { LOG(WARNING) << "Uh oh"; }());
// ```
//
// Automatic conversion to `base::expected<U, E>`:
// ```
//   base::expected<int, SomeErrorCode> Foo() {
//     RETURN_IF_ERROR(TryProcessing(query),
//                     [](auto) { return SomeErrorCode::kFail); });
//     return 17;
//   }
// ```
//
// Passing the error to a static/global handler:
// ```
//   RETURN_IF_ERROR(TryProcessing(query), &FailureHandler);
// ```
//
// Passing the error to a handler member function:
// ```
//   RETURN_IF_ERROR(TryProcessing(query), &MyClass::FailureHandler, this);
// ```
//
// # Modified examples for the `std::optional<T>` case
//
// Use with no additional arguments:
// ```
//   std::optional<int> Foo() {
//     RETURN_IF_ERROR(Function(args...));
//     RETURN_IF_ERROR(obj.Method(args...));
//     return 17;
//   }
// ```
//
// Returning some kind of error:
// ```
//   RETURN_IF_ERROR(TryProcessing(query),
//                   [] { return SomeErrorCode::kFail); });
// ```
//
// Returning void:
// ```
//   RETURN_IF_ERROR(TryProcessing(query), [] {});
// ```
// ```
//   RETURN_IF_ERROR(TryProcessing(query), [] { LOG(WARNING) << "Uh oh"; }());
// ```
#define RETURN_IF_ERROR(rexpr, ...)                                        \
  BASE_INTERNAL_EXPECTED_PASS_ARGS(BASE_INTERNAL_EXPECTED_RETURN_IF_ERROR, \
                                   BASE_UNIQUIFY(_expected_value), rexpr,  \
                                   __VA_ARGS__)

// Executes an expression `rexpr` that returns an `expected<T, E>` or
// `std::optional<T>`. If the result is not an error/`std::nullopt`
// (respectively), moves the `T` into whatever `lhs` defines/refers to;
// otherwise, behaves like RETURN_IF_ERROR() above. Avoid side effects in `lhs`,
// as it will not be evaluated in the error case.
//
// # Interface
//
// `ASSIGN_OR_RETURN(lhs, rexpr, ...);`
//
// WARNING: If `lhs` is parenthesized, the parentheses are removed; for this
//          reason, `lhs` may not contain a ternary (`?:`). See examples for
//          motivation.
//
// WARNING: Expands into multiple statements; cannot be used in a single
//          statement (e.g. as the body of an `if` statement without `{}`)!
//
// # Examples for the `expected<T, E>` case
//
// Declaring and initializing a new variable (ValueType can be anything that can
// be initialized with assignment):
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(arg));
// ```
//
// Assigning to an existing variable:
// ```
//   ValueType value;
//   ASSIGN_OR_RETURN(value, MaybeGetValue(arg));
// ```
//
// Initializing a `std::unique_ptr`:
// ```
//   ASSIGN_OR_RETURN(std::unique_ptr<T> ptr, MaybeGetPtr(arg));
// ```
//
// Initializing a map. Because of C++ preprocessor limitations, the type used in
// `ASSIGN_OR_RETURN` cannot contain commas, so wrap `lhs` in parentheses:
// ```
//   ASSIGN_OR_RETURN((flat_map<Foo, Bar> my_map), GetMap());
// ```
// Or use `auto` if the type is obvious enough:
// ```
//   ASSIGN_OR_RETURN(auto code_widget, GetCodeWidget());
// ```
//
// Assigning to structured bindings. The same situation with comma as above, so
// wrap `lhs` in parentheses:
// ```
//   ASSIGN_OR_RETURN((auto [first, second]), GetPair());
// ```
//
// Attempting to assign to a ternary will not compile:
// ```
//   ASSIGN_OR_RETURN((cond ? a : b), MaybeGetValue(arg));  // DOES NOT COMPILE
// ```
//
// Adjusting the returned error:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    [](const auto& e) {
//                      return base::StrCat({e, " while getting value"});
//                    });
// ```
//
// Returning a different kind of error:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    [](auto) { return SomeErrorCode::kFail); });
// ```
//
// Returning void:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query), [](auto) {});
// ```
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    [](auto) { LOG(WARNING) << "Uh oh"; }());
// ```
//
// Automatic conversion to `base::expected<U, E>`:
// ```
//   base::expected<int, SomeErrorCode> Foo() {
//     ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                      [](auto) { return SomeErrorCode::kFail); });
//     return 17;
//   }
// ```
//
// Passing the error to a static/global handler:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query), &FailureHandler);
// ```
//
// Passing the error to a handler member function:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    &MyClass::FailureHandler, this);
// ```
//
// # Modified examples for the `std::optional<T>` case
//
// Returning some kind of error:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    [] { return SomeErrorCode::kFail); });
// ```
//
// Returning void:
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query), [] {});
// ```
// ```
//   ASSIGN_OR_RETURN(ValueType value, MaybeGetValue(query),
//                    [] { LOG(WARNING) << "Uh oh"; }());
// ```
#define ASSIGN_OR_RETURN(lhs, rexpr, ...)                                      \
  BASE_INTERNAL_EXPECTED_PASS_ARGS(BASE_INTERNAL_EXPECTED_ASSIGN_OR_RETURN,    \
                                   lhs, BASE_UNIQUIFY(_expected_value), rexpr, \
                                   __VA_ARGS__)

namespace base::internal {

// =================================================================
// == Implementation details, do not rely on anything below here. ==
// =================================================================

// Helper object to allow returning some `E` from a method either directly or in
// the error of an `expected<T, E>`. Supports move-only `E`, as well as `void`.
//
// In order to support `void` return types, `UnexpectedDeducer` is not
// constructed directly with an `E`, but with a lambda that returns `E`; and
// callers must return `Ret()` rather than returning the deducer itself. Using
// both these indirections allows consistent invocation from macros.
template <typename Lambda,
          typename Arg,
          typename E = std::invoke_result_t<Lambda&&, Arg&&>>
class UnexpectedDeducer {
 public:
  constexpr UnexpectedDeducer(Lambda&& lambda, Arg&& arg) noexcept
      : lambda_(std::move(lambda)), arg_(std::move(arg)) {}

  constexpr decltype(auto) Ret() && noexcept {
    if constexpr (std::is_void_v<E>) {
      std::move(lambda_)(std::move(arg_));
    } else {
      return std::move(*this);
    }
  }

  // Allow implicit conversion from `Ret()` to either `expected<T, E>` (for
  // arbitrary `T`) or `E`.
  template <typename T>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator expected<T, E>() && noexcept
    requires(!std::is_void_v<E>)
  {
    return expected<T, E>(unexpect, std::move(lambda_)(std::move(arg_)));
  }
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator E() && noexcept
    requires(!std::is_void_v<E>)
  {
    return std::move(lambda_)(std::move(arg_));
  }

  // Disallow implicit conversion to `std::optional<T>`. Either `E` is already
  // a type that can convert to this and this is unnecessary due to the
  // conversion operator above, or `E` is some other type and we're discarding
  // whatever was in it. Theoretically this might not be an information loss if
  // e.g. `E` is an unbound `std::optional<U>`, but it seems better to force
  // people to match types in this case. Also note that since `E` isn't
  // convertible, this would be a compile error even without deleting this
  // function; but deleting it makes it clear this isn't an omission in this
  // code, but behavior we explicitly don't want to support.
  template <typename T>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator std::optional<T>() && noexcept
    requires(!std::is_void_v<E> && !std::convertible_to<E, std::optional<T>>)
  = delete;  // Use an adapter that returns this type.

 private:
  // RAW_PTR_EXCLUSION: Not intended to handle &&-qualified members.
  // `UnexpectedDeducer` is a short-lived temporary and tries to minimize
  // copying and other overhead; using raw_ptr/ref goes against this design
  // without adding meaningful safety.
  RAW_PTR_EXCLUSION Lambda&& lambda_;
  RAW_PTR_EXCLUSION Arg&& arg_;
};

// Deduce the type of the lambda automatically so callers don't need to spell
// things twice (or use temps) and use decltype.
template <typename Lambda, typename Arg>
UnexpectedDeducer(Lambda, Arg) -> UnexpectedDeducer<Lambda, Arg>;

// Workaround for https://github.com/llvm/llvm-project/issues/58872: Indirect
// through an extra layer so if the compiler attempts to instantiate both arms
// of the constexpr if in `BASE_INTERNAL_EXPECTED_BODY`, it will succeed.
// TODO(https://github.com/llvm/llvm-project/issues/58872): Remove this struct
// and the constructions of it below, and let them invoke `__VA_ARGS__`
// directly.
struct Trampoline {
  template <typename... Args>
  constexpr auto operator()(Args&&... args) const noexcept {
    // Should always succeed if this is actually reached at runtime.
    if constexpr (std::is_invocable_v<Args&&...>) {
      return std::invoke(std::forward<Args>(args)...);
    }
  }
};

}  // namespace base::internal

#define BASE_INTERNAL_EXPECTED_PASS_ARGS(func, ...) func(__VA_ARGS__)

#define BASE_INTERNAL_EXPECTED_BODY(expected, rexpr, name, ...)               \
  auto expected = (rexpr);                                                    \
  {                                                                           \
    static_assert(                                                            \
        base::internal::IsExpected<decltype(expected)> ||                     \
            base::is_instantiation<std::optional, decltype(expected)>,        \
        #name                                                                 \
        " should only be used with base::expected<> or std::optional<>");     \
  }                                                                           \
  if (!expected.has_value()) [[unlikely]] {                                   \
    /* Pass `expected` as an arg rather than capturing, so the lambda body */ \
    /* is a template context, so `constexpr if` avoids instantiating the */   \
    /* non-matching arm, since it won't compile otherwise. */                 \
    return base::internal::UnexpectedDeducer(                                 \
               [&](auto&& base_internal_expected__) {                         \
                 if constexpr (base::internal::IsExpected<                    \
                                   decltype(base_internal_expected__)>) {     \
                   return BASE_IF(                                            \
                       BASE_IS_EMPTY(__VA_ARGS__),                            \
                       std::move(base_internal_expected__).error(),           \
                       std::invoke(                                           \
                           base::internal::Trampoline(), __VA_ARGS__,         \
                           std::move(base_internal_expected__).error()));     \
                 } else {                                                     \
                   return BASE_IF(BASE_IS_EMPTY(__VA_ARGS__),                 \
                                  std::move(base_internal_expected__),        \
                                  std::invoke(base::internal::Trampoline(),   \
                                              __VA_ARGS__));                  \
                 }                                                            \
               },                                                             \
               std::move(expected))                                           \
        .Ret();                                                               \
  }

#define BASE_INTERNAL_EXPECTED_RETURN_IF_ERROR(expected, rexpr, ...) \
  do {                                                               \
    BASE_INTERNAL_EXPECTED_BODY(expected, rexpr, RETURN_IF_ERROR,    \
                                __VA_ARGS__);                        \
  } while (false)

#define BASE_INTERNAL_EXPECTED_ASSIGN_OR_RETURN(lhs, expected, rexpr, ...)     \
  {                                                                            \
    constexpr auto lhs_v = std::string_view(#lhs);                             \
    static_assert(!(lhs_v.front() == '(' && lhs_v.back() == ')' &&             \
                    lhs_v.rfind('?') != std::string_view::npos),               \
                  "Identified possible ternary in `lhs`; avoid passing "       \
                  "parenthesized expressions containing '?' to the first "     \
                  "argument of ASSIGN_OR_RETURN()");                           \
  }                                                                            \
  BASE_INTERNAL_EXPECTED_BODY(expected, rexpr, ASSIGN_OR_RETURN, __VA_ARGS__); \
  BASE_REMOVE_PARENS(lhs) = std::move(expected).value();

#endif  // BASE_TYPES_EXPECTED_MACROS_H_
