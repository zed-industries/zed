// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// NOTE: Header files that do not require the full definition of
// base::{Once,Repeating}Callback or base::{Once,Repeating}Closure should
// #include "base/functional/callback_forward.h" instead of this file.

#ifndef BASE_FUNCTIONAL_CALLBACK_H_
#define BASE_FUNCTIONAL_CALLBACK_H_

#include <stddef.h>

#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/functional/bind.h"
#include "base/functional/callback_forward.h"  // IWYU pragma: export
#include "base/functional/callback_internal.h"
#include "base/functional/callback_tags.h"
#include "base/functional/function_ref.h"
#include "base/notreached.h"
#include "base/types/always_false.h"

// -----------------------------------------------------------------------------
// Usage documentation
// -----------------------------------------------------------------------------
//
// Overview:
// A callback is similar in concept to a function pointer: it wraps a runnable
// object such as a function, method, lambda, or even another callback, allowing
// the runnable object to be invoked later via the callback object.
//
// Unlike function pointers, callbacks are created with base::BindOnce() or
// base::BindRepeating() and support partial function application.
//
// A base::OnceCallback may be Run() at most once; a base::RepeatingCallback may
// be Run() any number of times. |is_null()| is guaranteed to return true for a
// moved-from callback.
//
//   // The lambda takes two arguments, but the first argument |x| is bound at
//   // callback creation.
//   base::OnceCallback<int(int)> cb = base::BindOnce([] (int x, int y) {
//     return x + y;
//   }, 1);
//   // Run() only needs the remaining unbound argument |y|.
//   printf("1 + 2 = %d\n", std::move(cb).Run(2));  // Prints 3
//   printf("cb is null? %s\n",
//          cb.is_null() ? "true" : "false");  // Prints true
//   std::move(cb).Run(2);  // Crashes since |cb| has already run.
//
// Callbacks also support cancellation. A common use is binding the receiver
// object as a WeakPtr<T>. If that weak pointer is invalidated, calling Run()
// will be a no-op. Note that |IsCancelled()| and |is_null()| are distinct:
// simply cancelling a callback will not also make it null.
//
// See //docs/callback.md for the full documentation.

namespace base {

namespace internal {

template <bool is_once,
          typename R,
          typename... UnboundArgs,
          typename... BoundArgs>
auto ToDoNothingCallback(
    DoNothingCallbackTag::WithBoundArguments<BoundArgs...> t);

}  // namespace internal

template <typename R, typename... Args>
class TRIVIAL_ABI OnceCallback<R(Args...)> {
 public:
  using ResultType = R;
  using RunType = R(Args...);
  using PolymorphicInvoke = R (*)(internal::BindStateBase*,
                                  internal::PassingType<Args>...);

  // Constructs a null `OnceCallback`. A null callback has no associated functor
  // and cannot be run.
  constexpr OnceCallback() = default;
  // Disallowed to prevent ambiguity.
  OnceCallback(std::nullptr_t) = delete;

  // `OnceCallback` is not copyable since its bound functor may only run at most
  // once.
  OnceCallback(const OnceCallback&) = delete;
  OnceCallback& operator=(const OnceCallback&) = delete;

  // Subtle: since `this` is marked as TRIVIAL_ABI, the move operations
  // must leave the moved-from callback in a trivially destructible state.
  OnceCallback(OnceCallback&&) noexcept = default;
  OnceCallback& operator=(OnceCallback&&) noexcept = default;

  ~OnceCallback() = default;

  // A `OnceCallback` is a strict subset of `RepeatingCallback`'s functionality,
  // so allow seamless conversion.
  // NOLINTNEXTLINE(google-explicit-constructor)
  OnceCallback(RepeatingCallback<RunType> other)
      : holder_(std::move(other.holder_)) {}
  OnceCallback& operator=(RepeatingCallback<RunType> other) {
    holder_ = std::move(other.holder_);
    return *this;
  }

  // Returns true if `this` is non-null and can be `Run()`.
  explicit operator bool() const { return !!holder_; }
  // Returns true if `this` is null and cannot be `Run()`.
  bool is_null() const { return holder_.is_null(); }

  // Returns true if calling `Run()` is a no-op because of cancellation.
  //
  // - Not thread-safe, i.e. must be called on the same sequence that will
  //   ultimately `Run()` the callback
  // - May not be called on a null callback.
  bool IsCancelled() const { return holder_.IsCancelled(); }

  // Subtle version of `IsCancelled()` that allows cancellation state to be
  // queried from any sequence. May return true even if the callback has
  // actually been cancelled.
  //
  // Do not use. This is intended for internal //base usage.
  // TODO(dcheng): Restrict this since it, in fact, being used outside of its
  // originally intended use.
  bool MaybeValid() const { return holder_.MaybeValid(); }

  // Resets this to a null state.
  REINITIALIZES_AFTER_MOVE void Reset() { holder_.Reset(); }

  // Non-consuming `Run()` is disallowed for `OnceCallback`.
  R Run(Args... args) const& {
    static_assert(!sizeof(*this),
                  "OnceCallback::Run() may only be invoked on a non-const "
                  "rvalue, i.e. std::move(callback).Run().");
    NOTREACHED();
  }

  // Calls the bound functor with any already-bound arguments + `args`. Consumes
  // `this`, i.e. `this` becomes null.
  //
  // May not be called on a null callback.
  R Run(Args... args) && {
    CHECK(!is_null());

    // Move the callback instance into a local variable before the invocation,
    // that ensures the internal state is cleared after the invocation.
    // It's not safe to touch |this| after the invocation, since running the
    // bound function may destroy |this|.
    internal::BindStateHolder holder = std::move(holder_);
    PolymorphicInvoke f =
        reinterpret_cast<PolymorphicInvoke>(holder.polymorphic_invoke());
    return f(holder.bind_state().get(), std::forward<Args>(args)...);
  }

  // Then() returns a new OnceCallback that receives the same arguments as
  // |this|, and with the return type of |then|. The returned callback will:
  // 1) Run the functor currently bound to |this| callback.
  // 2) Run the |then| callback with the result from step 1 as its single
  //    argument.
  // 3) Return the value from running the |then| callback.
  //
  // Since this method generates a callback that is a replacement for `this`,
  // `this` will be consumed and reset to a null callback to ensure the
  // originally-bound functor can be run at most once.
  template <typename ThenR, typename... ThenArgs>
  OnceCallback<ThenR(Args...)> Then(OnceCallback<ThenR(ThenArgs...)> then) && {
    CHECK(then);
    return base::BindOnce(
        internal::ThenHelper<
            OnceCallback, OnceCallback<ThenR(ThenArgs...)>>::CreateTrampoline(),
        std::move(*this), std::move(then));
  }

  // This overload is required; even though RepeatingCallback is implicitly
  // convertible to OnceCallback, that conversion will not used when matching
  // for template argument deduction.
  template <typename ThenR, typename... ThenArgs>
  OnceCallback<ThenR(Args...)> Then(
      RepeatingCallback<ThenR(ThenArgs...)> then) && {
    CHECK(then);
    return base::BindOnce(
        internal::ThenHelper<
            OnceCallback,
            RepeatingCallback<ThenR(ThenArgs...)>>::CreateTrampoline(),
        std::move(*this), std::move(then));
  }

  // Internal constructors for various callback helper tag types, e.g.
  // `base::DoNothing()`.

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OnceCallback(internal::NullCallbackTag) : OnceCallback() {}
  constexpr OnceCallback& operator=(internal::NullCallbackTag) {
    *this = OnceCallback();
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OnceCallback(internal::NullCallbackTag::WithSignature<RunType>)
      : OnceCallback(internal::NullCallbackTag()) {}
  constexpr OnceCallback& operator=(
      internal::NullCallbackTag::WithSignature<RunType>) {
    *this = internal::NullCallbackTag();
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OnceCallback(internal::DoNothingCallbackTag)
    requires(std::is_void_v<R>)
      : OnceCallback(BindOnce([](Args... args) {})) {}
  constexpr OnceCallback& operator=(internal::DoNothingCallbackTag)
    requires(std::is_void_v<R>)
  {
    *this = BindOnce([](Args... args) {});
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OnceCallback(internal::DoNothingCallbackTag::WithSignature<RunType>)
    requires(std::is_void_v<R>)
      : OnceCallback(internal::DoNothingCallbackTag()) {}
  constexpr OnceCallback& operator=(
      internal::DoNothingCallbackTag::WithSignature<RunType>)
    requires(std::is_void_v<R>)
  {
    *this = internal::DoNothingCallbackTag();
    return *this;
  }

  template <typename... BoundArgs>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr OnceCallback(
      internal::DoNothingCallbackTag::WithBoundArguments<BoundArgs...> tag)
    requires(std::is_void_v<R>)
      : OnceCallback(
            internal::ToDoNothingCallback<true, R, Args...>(std::move(tag))) {}
  template <typename... BoundArgs>
  constexpr OnceCallback& operator=(
      internal::DoNothingCallbackTag::WithBoundArguments<BoundArgs...> tag)
    requires(std::is_void_v<R>)
  {
    *this = internal::ToDoNothingCallback<true, R, Args...>(std::move(tag));
    return *this;
  }

  // Internal constructor for `base::BindOnce()`.
  explicit OnceCallback(internal::BindStateBase* bind_state)
      : holder_(bind_state) {}

  template <typename Signature>
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator FunctionRef<Signature>() & {
    static_assert(
        AlwaysFalse<Signature>,
        "need to convert a base::OnceCallback to base::FunctionRef? "
        "Please bring up this use case on #cxx (Slack) or cxx@chromium.org.");
  }

  template <typename Signature>
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator FunctionRef<Signature>() && {
    static_assert(
        AlwaysFalse<Signature>,
        "using base::BindOnce() is not necessary with base::FunctionRef; is it "
        "possible to use a capturing lambda directly? If not, please bring up "
        "this use case on #cxx (Slack) or cxx@chromium.org.");
  }

 private:
  internal::BindStateHolder holder_;
};

template <typename R, typename... Args>
class TRIVIAL_ABI RepeatingCallback<R(Args...)> {
 public:
  using ResultType = R;
  using RunType = R(Args...);
  using PolymorphicInvoke = R (*)(internal::BindStateBase*,
                                  internal::PassingType<Args>...);

  // Constructs a null `RepeatingCallback`. A null callback has no associated
  // functor and cannot be run.
  constexpr RepeatingCallback() = default;
  // Disallowed to prevent ambiguity.
  RepeatingCallback(std::nullptr_t) = delete;

  // Unlike a `OnceCallback`, a `RepeatingCallback` may be copied since its
  // bound functor may be run more than once.
  RepeatingCallback(const RepeatingCallback&) = default;
  RepeatingCallback& operator=(const RepeatingCallback&) = default;

  // Subtle: since `this` is marked as TRIVIAL_ABI, the move operations
  // must leave the moved-from callback in a trivially destructible state.
  RepeatingCallback(RepeatingCallback&&) noexcept = default;
  RepeatingCallback& operator=(RepeatingCallback&&) noexcept = default;

  ~RepeatingCallback() = default;

  // Returns true if `this` is non-null and can be `Run()`.
  explicit operator bool() const { return !!holder_; }
  // Returns true if `this` is null and cannot be `Run()`.
  bool is_null() const { return holder_.is_null(); }

  // Returns true if calling `Run()` is a no-op because of cancellation.
  //
  // - Not thread-safe, i.e. must be called on the same sequence that will
  //   ultimately `Run()` the callback
  // - May not be called on a null callback.
  bool IsCancelled() const { return holder_.IsCancelled(); }

  // Subtle version of `IsCancelled()` that allows cancellation state to be
  // queried from any sequence. May return true even if the callback has
  // actually been cancelled.
  //
  // Do not use. This is intended for internal //base usage.
  // TODO(dcheng): Restrict this since it, in fact, being used outside of its
  // originally intended use.
  bool MaybeValid() const { return holder_.MaybeValid(); }

  // Equality operators: two `RepeatingCallback`'s are equal
  friend bool operator==(const RepeatingCallback&,
                         const RepeatingCallback&) = default;

  // Resets this to null.
  REINITIALIZES_AFTER_MOVE void Reset() { holder_.Reset(); }

  // Calls the bound functor with any already-bound arguments + `args`. Does not
  // consume `this`, i.e. this remains non-null.
  //
  // May not be called on a null callback.
  R Run(Args... args) const& {
    CHECK(!is_null());

    // Keep `bind_state` alive at least until after the invocation to ensure all
    // bound `Unretained` arguments remain protected by MiraclePtr.
    scoped_refptr<internal::BindStateBase> bind_state = holder_.bind_state();

    PolymorphicInvoke f =
        reinterpret_cast<PolymorphicInvoke>(holder_.polymorphic_invoke());
    return f(bind_state.get(), std::forward<Args>(args)...);
  }

  // Calls the bound functor with any already-bound arguments + `args`. Consumes
  // `this`, i.e. `this` becomes null.
  //
  // May not be called on a null callback.
  R Run(Args... args) && {
    CHECK(!holder_.is_null());

    // Move the callback instance into a local variable before the invocation,
    // that ensures the internal state is cleared after the invocation.
    // It's not safe to touch |this| after the invocation, since running the
    // bound function may destroy |this|.
    internal::BindStateHolder holder = std::move(holder_);
    PolymorphicInvoke f =
        reinterpret_cast<PolymorphicInvoke>(holder.polymorphic_invoke());
    return f(holder.bind_state().get(), std::forward<Args>(args)...);
  }

  // Then() returns a new RepeatingCallback that receives the same arguments as
  // |this|, and with the return type of |then|. The
  // returned callback will:
  // 1) Run the functor currently bound to |this| callback.
  // 2) Run the |then| callback with the result from step 1 as its single
  //    argument.
  // 3) Return the value from running the |then| callback.
  //
  // If called on an rvalue (e.g. std::move(cb).Then(...)), this method
  // generates a callback that is a replacement for `this`. Therefore, `this`
  // will be consumed and reset to a null callback to ensure the
  // originally-bound functor will be run at most once.
  template <typename ThenR, typename... ThenArgs>
  RepeatingCallback<ThenR(Args...)> Then(
      RepeatingCallback<ThenR(ThenArgs...)> then) const& {
    CHECK(then);
    return BindRepeating(
        internal::ThenHelper<
            RepeatingCallback,
            RepeatingCallback<ThenR(ThenArgs...)>>::CreateTrampoline(),
        *this, std::move(then));
  }

  template <typename ThenR, typename... ThenArgs>
  RepeatingCallback<ThenR(Args...)> Then(
      RepeatingCallback<ThenR(ThenArgs...)> then) && {
    CHECK(then);
    return BindRepeating(
        internal::ThenHelper<
            RepeatingCallback,
            RepeatingCallback<ThenR(ThenArgs...)>>::CreateTrampoline(),
        std::move(*this), std::move(then));
  }

  // Internal constructors for various callback helper tag types, e.g.
  // `base::DoNothing()`.

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr RepeatingCallback(internal::NullCallbackTag)
      : RepeatingCallback() {}
  constexpr RepeatingCallback& operator=(internal::NullCallbackTag) {
    *this = RepeatingCallback();
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr RepeatingCallback(internal::NullCallbackTag::WithSignature<RunType>)
      : RepeatingCallback(internal::NullCallbackTag()) {}
  constexpr RepeatingCallback& operator=(
      internal::NullCallbackTag::WithSignature<RunType>) {
    *this = internal::NullCallbackTag();
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr RepeatingCallback(internal::DoNothingCallbackTag)
    requires(std::is_void_v<R>)
      : RepeatingCallback(BindRepeating([](Args... args) {})) {}
  constexpr RepeatingCallback& operator=(internal::DoNothingCallbackTag)
    requires(std::is_void_v<R>)
  {
    *this = BindRepeating([](Args... args) {});
    return *this;
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr RepeatingCallback(
      internal::DoNothingCallbackTag::WithSignature<RunType>)
    requires(std::is_void_v<R>)
      : RepeatingCallback(internal::DoNothingCallbackTag()) {}
  constexpr RepeatingCallback& operator=(
      internal::DoNothingCallbackTag::WithSignature<RunType>)
    requires(std::is_void_v<R>)
  {
    *this = internal::DoNothingCallbackTag();
    return *this;
  }

  template <typename... BoundArgs>
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr RepeatingCallback(
      internal::DoNothingCallbackTag::WithBoundArguments<BoundArgs...> tag)
    requires(std::is_void_v<R>)
      : RepeatingCallback(
            internal::ToDoNothingCallback<false, R, Args...>(std::move(tag))) {}
  template <typename... BoundArgs>
  constexpr RepeatingCallback& operator=(
      internal::DoNothingCallbackTag::WithBoundArguments<BoundArgs...> tag)
    requires(std::is_void_v<R>)
  {
    *this = internal::ToDoNothingCallback<false, R, Args...>(std::move(tag));
    return this;
  }

  // Internal constructor for `base::BindRepeating()`.
  explicit RepeatingCallback(internal::BindStateBase* bind_state)
      : holder_(bind_state) {}

  template <typename Signature>
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator FunctionRef<Signature>() & {
    static_assert(
        AlwaysFalse<Signature>,
        "need to convert a base::RepeatingCallback to base::FunctionRef? "
        "Please bring up this use case on #cxx (Slack) or cxx@chromium.org.");
  }

  template <typename Signature>
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator FunctionRef<Signature>() && {
    static_assert(
        AlwaysFalse<Signature>,
        "using base::BindRepeating() is not necessary with base::FunctionRef; "
        "is it possible to use a capturing lambda directly? If not, please "
        "bring up this use case on #cxx (Slack) or cxx@chromium.org.");
  }

 private:
  friend class OnceCallback<R(Args...)>;

  internal::BindStateHolder holder_;
};

namespace internal {

// Helper for the `DoNothingWithBoundArgs()` implementation.
// Unlike the other helpers, this cannot be easily moved to callback_internal.h,
// since it depends on `BindOnce()` and `BindRepeating()`.
template <bool is_once,
          typename R,
          typename... UnboundArgs,
          typename... BoundArgs>
auto ToDoNothingCallback(
    DoNothingCallbackTag::WithBoundArguments<BoundArgs...> t) {
  return std::apply(
      [](auto&&... args) {
        if constexpr (is_once) {
          return BindOnce([](TransformToUnwrappedType<is_once, BoundArgs>...,
                             UnboundArgs...) {},
                          std::move(args)...);
        } else {
          return BindRepeating(
              [](TransformToUnwrappedType<is_once, BoundArgs>...,
                 UnboundArgs...) {},
              std::move(args)...);
        }
      },
      std::move(t.bound_args));
}

}  // namespace internal

}  // namespace base

#endif  // BASE_FUNCTIONAL_CALLBACK_H_
