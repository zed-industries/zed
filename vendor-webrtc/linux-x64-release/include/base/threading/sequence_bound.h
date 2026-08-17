// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SEQUENCE_BOUND_H_
#define BASE_THREADING_SEQUENCE_BOUND_H_

#include <concepts>
#include <new>
#include <tuple>
#include <type_traits>
#include <utility>

#include "base/check.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/run_loop.h"
#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/sequence_bound_internal.h"

namespace base {

// Performing blocking work on a different task runner is a common pattern for
// improving responsiveness of foreground task runners. `SequenceBound<T>`
// provides an abstraction for an owner object living on the owner sequence, to
// construct, call methods on, and destroy an object of type T that lives on a
// different sequence (the bound sequence).
//
// This makes it natural for code running on different sequences to be
// partitioned along class boundaries, e.g.:
//
// class Tab {
//  private:
//   void OnScroll() {
//     // ...
//     io_helper_.AsyncCall(&IOHelper::SaveScrollPosition);
//   }
//   base::SequenceBound<IOHelper> io_helper_{GetBackgroundTaskRunner()};
// };
//
// Note: `SequenceBound<T>` intentionally does not expose a raw pointer to the
// managed `T` to ensure its internal sequence-safety invariants are not
// violated. As a result, `AsyncCall()` cannot simply use `base::OnceCallback`
//
// SequenceBound also supports replies:
//
//   class Database {
//    public:
//     int Query(int value) {
//       return value * value;
//     }
//   };
//
//   // SequenceBound itself is owned on
//   // `SequencedTaskRunner::GetCurrentDefault()`. The managed Database
//   // instance managed by it is constructed and owned on `GetDBTaskRunner()`.
//   base::SequenceBound<Database> db(GetDBTaskRunner());
//
//   // `Database::Query()` runs on `GetDBTaskRunner()`, but
//   // `reply_callback` will run on the owner task runner.
//   auto reply_callback = [] (int result) {
//     LOG(ERROR) << result;  // Prints 25.
//   };
//   db.AsyncCall(&Database::Query).WithArgs(5)
//     .Then(base::BindOnce(reply_callback));
//
//   // When `db` goes out of scope, the Database instance will also be
//   // destroyed via a task posted to `GetDBTaskRunner()`.
//
// Sequence safety:
//
// Const-qualified methods may be used concurrently from multiple sequences,
// e.g. `AsyncCall()` or `is_null()`. Calls that are forwarded to the
// managed `T` will be posted to the bound sequence and executed serially
// there.
//
// Mutable methods (e.g. `Reset()`, destruction, or move assignment) require
// external synchronization if used concurrently with any other methods,
// including const-qualified methods.
//
// Advanced usage:
//
// Using `SequenceBound<std::unique_ptr<T>>` allows transferring ownership of an
// already-constructed `T` to `SequenceBound`. This can be helpful for more
// complex situations, where `T` needs to be constructed on a specific sequence
// that is different from where `T` will ultimately live.
//
// Construction (via the constructor or emplace) takes a `std::unique_ptr<T>`
// instead of forwarding the arguments to `T`'s constructor:
//
//   std::unique_ptr<Database> db_impl = MakeDatabaseOnMainThread();
//   base::SequenceBound<std::unique_ptr<Database>> db(GetDbTaskRunner(),
//                                                     std::move(db_impl));
//
// All other usage (e.g. `AsyncCall()`, `Reset()`) functions identically to a
// regular `SequenceBound<T>`:
//
//   // No need to dereference the `std::unique_ptr` explicitly:
//   db.AsyncCall(&Database::Query).WithArgs(5).Then(base::BindOnce(...));
template <typename T,
          typename CrossThreadTraits =
              sequence_bound_internal::CrossThreadTraits>
class SequenceBound {
 private:
  using Storage = sequence_bound_internal::Storage<T, CrossThreadTraits>;
  using UnwrappedT = typename Storage::element_type;

 public:
  template <typename Signature>
  using CrossThreadTask =
      typename CrossThreadTraits::template CrossThreadTask<Signature>;

  // Note: on construction, SequenceBound binds to the current sequence. Any
  // subsequent SequenceBound calls (including destruction) must run on that
  // same sequence.

  // Constructs a null SequenceBound with no managed `T`.
  SequenceBound() = default;

  // Constructs a SequenceBound that manages a new instance of `T` on
  // `task_runner`. `T` will be constructed on `task_runner`.
  //
  // Once this constructor returns, it is safe to immediately use `AsyncCall()`,
  // et cetera; these calls will be sequenced after the construction of the
  // managed `T`.
  template <typename... Args>
  explicit SequenceBound(scoped_refptr<SequencedTaskRunner> task_runner,
                         Args&&... args)
      : impl_task_runner_(std::move(task_runner)) {
    storage_.Construct(*impl_task_runner_, std::forward<Args>(args)...);
  }

  // If non-null, the managed `T` will be destroyed on `impl_task_runner_`.`
  ~SequenceBound() { Reset(); }

  // Disallow copy or assignment. SequenceBound has single ownership of the
  // managed `T`.
  SequenceBound(const SequenceBound&) = delete;
  SequenceBound& operator=(const SequenceBound&) = delete;

  // Move construction and assignment.
  SequenceBound(SequenceBound&& other) { MoveRecordFrom(other); }

  SequenceBound& operator=(SequenceBound&& other) {
    Reset();
    MoveRecordFrom(other);
    return *this;
  }

  // Move conversion helpers: allows upcasting from SequenceBound<Derived> to
  // SequenceBound<Base>.
  template <typename U>
  // NOLINTNEXTLINE(google-explicit-constructor): Intentionally implicit.
  SequenceBound(SequenceBound<U, CrossThreadTraits>&& other) {
    // TODO(crbug.com/40245687): static_assert that U* is convertible to
    // T*.
    MoveRecordFrom(other);
  }

  template <typename U>
  SequenceBound& operator=(SequenceBound<U, CrossThreadTraits>&& other) {
    // TODO(crbug.com/40245687): static_assert that U* is convertible to
    // T*.
    Reset();
    MoveRecordFrom(other);
    return *this;
  }

  // Constructs a new managed instance of `T` on `task_runner`. If `this` is
  // already managing another instance of `T`, that pre-existing instance will
  // first be destroyed by calling `Reset()`.
  //
  // Once `emplace()` returns, it is safe to immediately use `AsyncCall()`,
  // et cetera; these calls will be sequenced after the construction of the
  // managed `T`.
  template <typename... Args>
  SequenceBound& emplace(scoped_refptr<SequencedTaskRunner> task_runner,
                         Args&&... args) {
    Reset();
    impl_task_runner_ = std::move(task_runner);
    storage_.Construct(*impl_task_runner_, std::forward<Args>(args)...);
    return *this;
  }

  // Invokes `method` of the managed `T` on `impl_task_runner_`. May only be
  // used when `is_null()` is false.
  //
  // Basic usage:
  //
  //   helper.AsyncCall(&IOHelper::DoWork);
  //
  // If `method` accepts arguments, use `WithArgs()` to bind them:
  //
  //   helper.AsyncCall(&IOHelper::DoWorkWithArgs)
  //       .WithArgs(args);
  //
  // Use `Then()` to run a callback on the owner sequence after `method`
  // completes:
  //
  //   helper.AsyncCall(&IOHelper::GetValue)
  //       .Then(std::move(process_result_callback));
  //
  // If a method returns a non-void type, use of `Then()` is required, and the
  // method's return value will be passed to the `Then()` callback. To ignore
  // the method's return value instead, wrap `method` in `base::IgnoreResult()`:
  //
  //   // Calling `GetPrefs` to force-initialize prefs.
  //   helper.AsyncCall(base::IgnoreResult(&IOHelper::GetPrefs));
  //
  // `WithArgs()` and `Then()` may also be combined:
  //
  //   // Ordering is important: `Then()` must come last.
  //   helper.AsyncCall(&IOHelper::GetValueWithArgs)
  //       .WithArgs(args)
  //       .Then(std::move(process_result_callback));
  //
  // Note: internally, `AsyncCall()` is implemented using a series of helper
  // classes that build the callback chain and post it on destruction. Capturing
  // the return value and passing it elsewhere or triggering lifetime extension
  // (e.g. by binding the return value to a reference) are both unsupported.
  template <typename R, typename C, typename... Args>
    requires(std::derived_from<UnwrappedT, C>)
  auto AsyncCall(R (C::*method)(Args...),
                 const Location& location = Location::Current()) const {
    return AsyncCallBuilder<R (C::*)(Args...), R, std::tuple<Args...>>(
        this, &location, method);
  }

  template <typename R, typename C, typename... Args>
    requires(std::derived_from<UnwrappedT, C>)
  auto AsyncCall(R (C::*method)(Args...) const,
                 const Location& location = Location::Current()) const {
    return AsyncCallBuilder<R (C::*)(Args...) const, R, std::tuple<Args...>>(
        this, &location, method);
  }

  template <typename R, typename C, typename... Args>
    requires(std::derived_from<UnwrappedT, C>)
  auto AsyncCall(internal::IgnoreResultHelper<R (C::*)(Args...) const> method,
                 const Location& location = Location::Current()) const {
    return AsyncCallBuilder<
        internal::IgnoreResultHelper<R (C::*)(Args...) const>, void,
        std::tuple<Args...>>(this, &location, method);
  }

  template <typename R, typename C, typename... Args>
    requires(std::derived_from<UnwrappedT, C>)
  auto AsyncCall(internal::IgnoreResultHelper<R (C::*)(Args...)> method,
                 const Location& location = Location::Current()) const {
    return AsyncCallBuilder<internal::IgnoreResultHelper<R (C::*)(Args...)>,
                            void, std::tuple<Args...>>(this, &location, method);
  }

  // Posts `task` to `impl_task_runner_`, passing it a reference to the wrapped
  // object. This allows arbitrary logic to be safely executed on the object's
  // task runner. The object is guaranteed to remain alive for the duration of
  // the task.
  // TODO(crbug.com/40170667): Consider checking whether the task runner can run
  // tasks in current sequence, and using "plain" binds and task posting (here
  // and other places that `CrossThreadTraits::PostTask`).
  using ConstPostTaskCallback = CrossThreadTask<void(const UnwrappedT&)>;
  void PostTaskWithThisObject(
      ConstPostTaskCallback callback,
      const Location& location = Location::Current()) const {
    DCHECK(!is_null());
    // Even though the lifetime of the object managed by `storage_` may not
    // have begun yet, the storage has been allocated. Per [basic.life/6] and
    // [basic.life/7], "Indirection through such a pointer is permitted but the
    // resulting lvalue may only be used in limited ways, as described below."
    CrossThreadTraits::PostTask(
        *impl_task_runner_, location,
        CrossThreadTraits::BindOnce(std::move(callback),
                                    storage_.GetRefForBind()));
  }

  // Same as above, but for non-const operations. The callback takes a pointer
  // to the wrapped object rather than a const ref.
  using PostTaskCallback = CrossThreadTask<void(UnwrappedT*)>;
  void PostTaskWithThisObject(
      PostTaskCallback callback,
      const Location& location = Location::Current()) const {
    DCHECK(!is_null());
    CrossThreadTraits::PostTask(
        *impl_task_runner_, location,
        CrossThreadTraits::BindOnce(std::move(callback),
                                    storage_.GetPtrForBind()));
  }

  void FlushPostedTasksForTesting() const {
    DCHECK(!is_null());
    RunLoop run_loop;
    CrossThreadTraits::PostTask(*impl_task_runner_, FROM_HERE,
                                run_loop.QuitClosure());
    run_loop.Run();
  }

  // TODO(liberato): Add PostOrCall(), to support cases where synchronous calls
  // are okay if it's the same task runner.

  // Resets `this` to null. If `this` is not currently null, posts destruction
  // of the managed `T` to `impl_task_runner_`.
  void Reset() {
    if (is_null()) {
      return;
    }

    storage_.Destruct(*impl_task_runner_);
    impl_task_runner_ = nullptr;
  }

  // Resets `this` to null. If `this` is not currently null, posts destruction
  // of the managed `T` to `impl_task_runner_`. Blocks until the destructor has
  // run.
  void SynchronouslyResetForTest() {
    if (is_null()) {
      return;
    }

    scoped_refptr<SequencedTaskRunner> task_runner = impl_task_runner_;
    Reset();
    // `Reset()` posts a task to destroy the managed `T`; synchronously wait for
    // that posted task to complete.
    RunLoop run_loop;
    CrossThreadTraits::PostTask(*task_runner, FROM_HERE,
                                run_loop.QuitClosure());
    run_loop.Run();
  }

  // Return true if `this` is logically null; otherwise, returns false.
  //
  // A SequenceBound is logically null if there is no managed `T`; it is only
  // valid to call `AsyncCall()` on a non-null SequenceBound.
  //
  // Note that the concept of 'logically null' here does not exactly match the
  // lifetime of `T`, which lives on `impl_task_runner_`. In particular, when
  // SequenceBound is first constructed, `is_null()` may return false, even
  // though the lifetime of `T` may not have begun yet on `impl_task_runner_`.
  // Similarly, after `SequenceBound::Reset()`, `is_null()` may return true,
  // even though the lifetime of `T` may not have ended yet on
  // `impl_task_runner_`.
  bool is_null() const { return storage_.is_null(); }

  // True if `this` is not logically null. See `is_null()`.
  explicit operator bool() const { return !is_null(); }

 private:
  // For move conversion.
  template <typename U, typename V>
  friend class SequenceBound;

  template <template <typename> class CallbackType>
  static constexpr bool IsCrossThreadTask =
      CrossThreadTraits::template IsCrossThreadTask<CallbackType>;

  // Support helpers for `AsyncCall()` implementation.
  //
  // Several implementation notes:
  // 1. Tasks are posted via destroying the builder or an explicit call to
  //    `Then()`.
  //
  // 2. A builder may be consumed by:
  //
  //    - calling `Then()`, which immediately posts the task chain
  //    - calling `WithArgs()`, which returns a new builder with the captured
  //      arguments
  //
  //    Builders that are consumed have the internal `sequence_bound_` field
  //    nulled out; the hope is the compiler can see this and use it to
  //    eliminate dead branches (e.g. correctness checks that aren't needed
  //    since the code can be statically proved correct).
  //
  // 3. Builder methods are rvalue-qualified to try to enforce that the builder
  //    is only used as a temporary. Note that this only helps so much; nothing
  //    prevents a determined caller from using `std::move()` to force calls to
  //    a non-temporary instance.
  //
  // TODO(dcheng): It might also be possible to use Gmock-style matcher
  // composition, e.g. something like:
  //
  //   sb.AsyncCall(&Helper::DoWork, WithArgs(args),
  //                Then(std::move(process_result));
  //
  // In theory, this might allow the elimination of magic destructors and
  // better static checking by the compiler.
  template <typename MethodRef>
  class AsyncCallBuilderBase {
   protected:
    AsyncCallBuilderBase(const SequenceBound* sequence_bound,
                         const Location* location,
                         MethodRef method)
        : sequence_bound_(sequence_bound),
          location_(location),
          method_(method) {
      // Common entry point for `AsyncCall()`, so check preconditions here.
      DCHECK(sequence_bound_);
      DCHECK(!sequence_bound_->storage_.is_null());
    }

    AsyncCallBuilderBase(AsyncCallBuilderBase&&) = default;
    AsyncCallBuilderBase& operator=(AsyncCallBuilderBase&&) = default;

    // `sequence_bound_` is consumed and set to `nullptr` when `Then()` is
    // invoked. This is used as a flag for two potential states
    //
    // - if a method returns void, invoking `Then()` is optional. The destructor
    //   will check if `sequence_bound_` is null; if it is, `Then()` was
    //   already invoked and the task chain has already been posted, so the
    //   destructor does not need to do anything. Otherwise, the destructor
    //   needs to post the task to make the async call. In theory, the compiler
    //   should be able to eliminate this branch based on the presence or
    //   absence of a call to `Then()`.
    //
    // - if a method returns a non-void type, `Then()` *must* be invoked. The
    //   destructor will `CHECK()` if `sequence_bound_` is non-null, since that
    //   indicates `Then()` was not invoked. Similarly, note this branch should
    //   be eliminated by the optimizer if the code is free of bugs. :)
    raw_ptr<const SequenceBound<T, CrossThreadTraits>, DanglingUntriaged>
        sequence_bound_;
    // Subtle: this typically points at a Location *temporary*. This is used to
    // try to detect errors resulting from lifetime extension of the async call
    // factory temporaries, since the factory destructors can perform work. If
    // the lifetime of the factory is incorrectly extended, dereferencing
    // `location_` will trigger a stack-use-after-scope when running with ASan.
    const raw_ptr<const Location> location_;
    MethodRef method_;
  };

  template <typename MethodRef, typename ReturnType, typename ArgsTuple>
  class AsyncCallBuilderImpl;

  // Selected method has no arguments and returns void.
  template <typename MethodRef>
  class AsyncCallBuilderImpl<MethodRef, void, std::tuple<>>
      : public AsyncCallBuilderBase<MethodRef> {
   public:
    // Note: despite being here, this is actually still protected, since it is
    // protected on the base class.
    using AsyncCallBuilderBase<MethodRef>::AsyncCallBuilderBase;

    ~AsyncCallBuilderImpl() {
      if (this->sequence_bound_) {
        CrossThreadTraits::PostTask(
            *this->sequence_bound_->impl_task_runner_, *this->location_,
            CrossThreadTraits::BindOnce(
                this->method_,
                this->sequence_bound_->storage_.GetPtrForBind()));
      }
    }

    void Then(CrossThreadTask<void()> then_callback) && {
      this->sequence_bound_->PostTaskAndThenHelper(
          *this->location_,
          CrossThreadTraits::BindOnce(
              this->method_, this->sequence_bound_->storage_.GetPtrForBind()),
          std::move(then_callback));
      this->sequence_bound_ = nullptr;
    }

   private:
    friend SequenceBound;

    AsyncCallBuilderImpl(AsyncCallBuilderImpl&&) = default;
    AsyncCallBuilderImpl& operator=(AsyncCallBuilderImpl&&) = default;
  };

  // Selected method has no arguments and returns non-void.
  template <typename MethodRef, typename ReturnType>
  class AsyncCallBuilderImpl<MethodRef, ReturnType, std::tuple<>>
      : public AsyncCallBuilderBase<MethodRef> {
   public:
    // Note: despite being here, this is actually still protected, since it is
    // protected on the base class.
    using AsyncCallBuilderBase<MethodRef>::AsyncCallBuilderBase;

    ~AsyncCallBuilderImpl() {
      // Must use Then() since the method's return type is not void.
      // Should be optimized out if the code is bug-free.
      CHECK(!this->sequence_bound_)
          << "Then() not invoked for a method that returns a non-void type; "
          << "make sure to invoke Then() or use base::IgnoreResult()";
    }

    template <template <typename> class CallbackType, typename ThenArg>
      requires(IsCrossThreadTask<CallbackType>)
    void Then(CallbackType<void(ThenArg)> then_callback) && {
      this->sequence_bound_->PostTaskAndThenHelper(
          *this->location_,
          CrossThreadTraits::BindOnce(
              this->method_, this->sequence_bound_->storage_.GetPtrForBind()),
          std::move(then_callback));
      this->sequence_bound_ = nullptr;
    }

   private:
    friend SequenceBound;

    AsyncCallBuilderImpl(AsyncCallBuilderImpl&&) = default;
    AsyncCallBuilderImpl& operator=(AsyncCallBuilderImpl&&) = default;
  };

  // Selected method has arguments. Return type can be void or non-void.
  template <typename MethodRef, typename ReturnType, typename... Args>
  class AsyncCallBuilderImpl<MethodRef, ReturnType, std::tuple<Args...>>
      : public AsyncCallBuilderBase<MethodRef> {
   public:
    // Note: despite being here, this is actually still protected, since it is
    // protected on the base class.
    using AsyncCallBuilderBase<MethodRef>::AsyncCallBuilderBase;

    ~AsyncCallBuilderImpl() {
      // Must use WithArgs() since the method takes arguments.
      // Should be optimized out if the code is bug-free.
      CHECK(!this->sequence_bound_);
    }

    template <typename... BoundArgs>
    auto WithArgs(BoundArgs&&... bound_args) {
      const SequenceBound* const sequence_bound =
          std::exchange(this->sequence_bound_, nullptr);
      return AsyncCallWithBoundArgsBuilder<ReturnType>(
          sequence_bound, this->location_,
          CrossThreadTraits::BindOnce(this->method_,
                                      sequence_bound->storage_.GetPtrForBind(),
                                      std::forward<BoundArgs>(bound_args)...));
    }

   private:
    friend SequenceBound;

    AsyncCallBuilderImpl(AsyncCallBuilderImpl&&) = default;
    AsyncCallBuilderImpl& operator=(AsyncCallBuilderImpl&&) = default;
  };

  // `MethodRef` is either a member function pointer type or a member function
  //     pointer type wrapped with `internal::IgnoreResultHelper`.
  // `R` is the return type of `MethodRef`. This is always `void` if
  //     `MethodRef` is an `internal::IgnoreResultHelper` wrapper.
  // `ArgsTuple` is a `std::tuple` with template type arguments corresponding to
  //     the types of the method's parameters.
  template <typename MethodRef, typename R, typename ArgsTuple>
  using AsyncCallBuilder = AsyncCallBuilderImpl<MethodRef, R, ArgsTuple>;

  // Support factories when arguments are bound using `WithArgs()`. These
  // factories don't need to handle raw method pointers, since everything has
  // already been packaged into a base::OnceCallback.
  template <typename ReturnType>
  class AsyncCallWithBoundArgsBuilderBase {
   protected:
    AsyncCallWithBoundArgsBuilderBase(const SequenceBound* sequence_bound,
                                      const Location* location,
                                      CrossThreadTask<ReturnType()> callback)
        : sequence_bound_(sequence_bound),
          location_(location),
          callback_(std::move(callback)) {
      DCHECK(sequence_bound_);
      DCHECK(!sequence_bound_->storage_.is_null());
    }

    // Subtle: the internal helpers rely on move elision. Preventing move
    // elision (e.g. using `std::move()` when returning the temporary) will
    // trigger a `CHECK()` since `sequence_bound_` is not reset to nullptr on
    // move.
    AsyncCallWithBoundArgsBuilderBase(
        AsyncCallWithBoundArgsBuilderBase&&) noexcept = default;
    AsyncCallWithBoundArgsBuilderBase& operator=(
        AsyncCallWithBoundArgsBuilderBase&&) noexcept = default;

    raw_ptr<const SequenceBound<T, CrossThreadTraits>> sequence_bound_;
    const raw_ptr<const Location> location_;
    CrossThreadTask<ReturnType()> callback_;
  };

  // Note: this doesn't handle a void return type, which has an explicit
  // specialization below.
  template <typename ReturnType>
  class AsyncCallWithBoundArgsBuilderDefault
      : public AsyncCallWithBoundArgsBuilderBase<ReturnType> {
   public:
    ~AsyncCallWithBoundArgsBuilderDefault() {
      // Must use Then() since the method's return type is not void.
      // Should be optimized out if the code is bug-free.
      CHECK(!this->sequence_bound_)
          << "Then() not invoked for a method that returns a non-void type; "
          << "make sure to invoke Then() or use base::IgnoreResult()";
    }

    template <template <typename> class CallbackType, typename ThenArg>
      requires(IsCrossThreadTask<CallbackType>)
    void Then(CallbackType<void(ThenArg)> then_callback) && {
      this->sequence_bound_->PostTaskAndThenHelper(*this->location_,
                                                   std::move(this->callback_),
                                                   std::move(then_callback));
      this->sequence_bound_ = nullptr;
    }

   protected:
    using AsyncCallWithBoundArgsBuilderBase<
        ReturnType>::AsyncCallWithBoundArgsBuilderBase;

   private:
    friend SequenceBound;

    AsyncCallWithBoundArgsBuilderDefault(
        AsyncCallWithBoundArgsBuilderDefault&&) = default;
    AsyncCallWithBoundArgsBuilderDefault& operator=(
        AsyncCallWithBoundArgsBuilderDefault&&) = default;
  };

  class AsyncCallWithBoundArgsBuilderVoid
      : public AsyncCallWithBoundArgsBuilderBase<void> {
   public:
    // Note: despite being here, this is actually still protected, since it is
    // protected on the base class.
    using AsyncCallWithBoundArgsBuilderBase<
        void>::AsyncCallWithBoundArgsBuilderBase;

    ~AsyncCallWithBoundArgsBuilderVoid() {
      if (this->sequence_bound_) {
        CrossThreadTraits::PostTask(*this->sequence_bound_->impl_task_runner_,
                                    *this->location_,
                                    std::move(this->callback_));
      }
    }

    void Then(CrossThreadTask<void()> then_callback) && {
      this->sequence_bound_->PostTaskAndThenHelper(*this->location_,
                                                   std::move(this->callback_),
                                                   std::move(then_callback));
      this->sequence_bound_ = nullptr;
    }

   private:
    friend SequenceBound;

    AsyncCallWithBoundArgsBuilderVoid(AsyncCallWithBoundArgsBuilderVoid&&) =
        default;
    AsyncCallWithBoundArgsBuilderVoid& operator=(
        AsyncCallWithBoundArgsBuilderVoid&&) = default;
  };

  template <typename ReturnType>
  using AsyncCallWithBoundArgsBuilder = typename std::conditional<
      std::is_void_v<ReturnType>,
      AsyncCallWithBoundArgsBuilderVoid,
      AsyncCallWithBoundArgsBuilderDefault<ReturnType>>::type;

  void PostTaskAndThenHelper(const Location& location,
                             CrossThreadTask<void()> callback,
                             CrossThreadTask<void()> then_callback) const {
    CrossThreadTraits::PostTaskAndReply(*impl_task_runner_, location,
                                        std::move(callback),
                                        std::move(then_callback));
  }

  template <typename ReturnType,
            template <typename>
            class CallbackType,
            typename ThenArg>
    requires(IsCrossThreadTask<CallbackType>)
  void PostTaskAndThenHelper(const Location& location,
                             CrossThreadTask<ReturnType()> callback,
                             CallbackType<void(ThenArg)> then_callback) const {
    CrossThreadTask<void(ThenArg)>&& once_then_callback =
        std::move(then_callback);
    CrossThreadTraits::PostTaskAndReplyWithResult(
        *impl_task_runner_, location, std::move(callback),
        std::move(once_then_callback));
  }

  // Helper to support move construction and move assignment.
  //
  // TODO(crbug.com/40245687): Constrain this so converting between
  // std::unique_ptr<T> and T are explicitly forbidden (rather than simply
  // failing to build in spectacular ways).
  template <typename From>
  void MoveRecordFrom(From&& other) {
    impl_task_runner_ = std::move(other.impl_task_runner_);

    storage_.TakeFrom(std::move(other.storage_));
  }

  Storage storage_;

  // Task runner which manages `storage_`. An object owned by `storage_` (if
  // any) will be constructed, destroyed, and otherwise used only on this task
  // runner.
  scoped_refptr<SequencedTaskRunner> impl_task_runner_;
};

}  // namespace base

#endif  // BASE_THREADING_SEQUENCE_BOUND_H_
