// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_FUTURE_H_
#define BASE_TEST_TEST_FUTURE_H_

#include <memory>
#include <optional>
#include <tuple>

#include "base/auto_reset.h"
#include "base/check.h"
#include "base/functional/bind.h"
#include "base/functional/callback_forward.h"
#include "base/functional/callback_helpers.h"
#include "base/memory/weak_ptr.h"
#include "base/run_loop.h"
#include "base/sequence_checker.h"
#include "base/strings/to_string.h"
#include "base/task/bind_post_task.h"
#include "base/task/sequenced_task_runner.h"
#include "base/test/test_future_internal.h"
#include "base/thread_annotations.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base::test {

// Helper class to test code that returns its result(s) asynchronously through a
// callback:
//
//    - Pass the callback provided by `GetCallback()` to the code under test.
//    - Wait for the callback to be invoked by calling `Wait(),` or `Get()` to
//      access the value(s) passed to the callback.
//
//   Example usage:
//
//     TEST_F(MyTestFixture, MyTest) {
//       TestFuture<ResultType> future;
//
//       object_under_test.DoSomethingAsync(future.GetCallback());
//
//       const ResultType& actual_result = future.Get();
//
//       // When you come here, DoSomethingAsync has finished and
//       // `actual_result` contains the result passed to the callback.
//     }
//
//   Example using `Wait()`:
//
//     TEST_F(MyTestFixture, MyWaitTest) {
//       TestFuture<ResultType> future;
//
//       object_under_test.DoSomethingAsync(future.GetCallback());
//
//       // Optional. The Get() call below will also wait until the value
//       // arrives, but this explicit call to Wait() can be useful if you want
//       // to add extra information.
//       ASSERT_TRUE(future.Wait()) << "Detailed error message";
//
//       const ResultType& actual_result = future.Get();
//     }
//
// `TestFuture` supports both single- and multiple-argument callbacks.
// `TestFuture` provides both index and type based accessors for multi-argument
// callbacks. `Get()` and `Take()` return tuples for multi-argument callbacks.
//
//   TestFuture<int, std::string> future;
//   future.Get<0>();   // Reads the first argument
//   future.Get<int>(); // Also reads the first argument
//   future.Get();      // Returns a `const std::tuple<int, std::string>&`
//
//   Example for a multi-argument callback:
//
//     TEST_F(MyTestFixture, MyTest) {
//       TestFuture<int, std::string> future;
//
//       object_under_test.DoSomethingAsync(future.GetCallback());
//
//       // You can use type based accessors:
//       int first_argument = future.Get<int>();
//       const std::string& second_argument = future.Get<std::string>();
//
//       // or index based accessors:
//       int first_argument = future.Get<0>();
//       const std::string& second_argument = future.Get<1>();
//     }
//
// You can also satisfy a `TestFuture` by calling `SetValue()` from the sequence
// on which the `TestFuture` was created. This is mostly useful when
// implementing an observer:
//
//     class MyTestObserver: public MyObserver {
//       public:
//         // `MyObserver` implementation:
//         void ObserveAnInt(int value) override {
//           future_.SetValue(value);
//         }
//
//         int Wait() { return future_.Take(); }
//
//      private:
//        TestFuture<int> future_;
//     };
//
//     TEST_F(MyTestFixture, MyTest) {
//       MyTestObserver observer;
//
//       object_under_test.DoSomethingAsync(observer);
//
//       int value_passed_to_observer = observer.Wait();
//     };
//
// `GetRepeatingCallback()` allows you to use a single `TestFuture` in code
// that invokes the callback multiple times.
// Your test must take care to consume each value before the next value
// arrives. You can consume the value by calling either `Take()` or `Clear()`.
//
//   Example for reusing a `TestFuture`:
//
//     TEST_F(MyTestFixture, MyReuseTest) {
//       TestFuture<std::string> future;
//
//       object_under_test.InstallCallback(future.GetRepeatingCallback());
//
//       object_under_test.DoSomething();
//       EXPECT_EQ(future.Take(), "expected-first-value");
//       // Because we used `Take()` the test future is ready for reuse.
//
//       object_under_test.DoSomethingElse();
//       EXPECT_EQ(future.Take(), "expected-second-value");
//     }
//
//   Example for reusing  a `TestFuture` using `Get()` + `Clear()`:
//
//     TEST_F(MyTestFixture, MyReuseTest) {
//       TestFuture<std::string, int> future;
//
//       object_under_test.InstallCallback(future.GetRepeatingCallback());
//
//       object_under_test.DoSomething();
//
//       EXPECT_EQ(future.Get<std::string>(), "expected-first-value");
//       EXPECT_EQ(future.Get<int>(), 5);
//       // Because we used `Get()`, the test future is not ready for reuse,
//       //so we need an explicit `Clear()` call.
//       future.Clear();
//
//       object_under_test.DoSomethingElse();
//       EXPECT_EQ(future.Get<std::string>(), "expected-second-value");
//       EXPECT_EQ(future.Get<int>(), 2);
//     }
//
// Finally, `TestFuture` also supports no-args callbacks:
//
//   Example for no-args callbacks:
//
//     TEST_F(MyTestFixture, MyTest) {
//       TestFuture<void> signal;
//
//       object_under_test.DoSomethingAsync(signal.GetCallback());
//
//       EXPECT_TRUE(signal.Wait());
//       // When you come here you know the callback was invoked and the async
//       // code is ready.
//     }
//
// All access to this class and its callbacks must be made from the sequence on
// which the `TestFuture` was constructed.
//
template <typename... Types>
class TestFuture {
 public:
  using TupleType = std::tuple<std::decay_t<Types>...>;

  static_assert(std::tuple_size_v<TupleType> > 0,
                "Don't use TestFuture<> but use TestFuture<void> instead");

  TestFuture() = default;
  TestFuture(TestFuture&&) = default;
  TestFuture(const TestFuture&) = delete;
  TestFuture& operator=(TestFuture&&) = default;
  TestFuture& operator=(const TestFuture&) = delete;
  ~TestFuture() = default;

  // Waits for the value to arrive.
  //
  // Returns true if the value arrived, or false if a timeout happens.
  //
  // Directly calling Wait() is not required as Get()/Take() will also wait for
  // the value to arrive, however you can use a direct call to Wait() to
  // improve the error reported:
  //
  //   ASSERT_TRUE(queue.Wait()) << "Detailed error message";
  //
  [[nodiscard]] bool Wait(
      RunLoop::Type run_loop_type = RunLoop::Type::kDefault) {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);

    if (impl_->values) {
      return true;
    }

    // Wait for the value to arrive.
    RunLoop loop(run_loop_type);
    AutoReset<RepeatingClosure> quit_loop(&impl_->ready_signal,
                                          loop.QuitClosure());
    loop.Run();

    return IsReady();
  }

  // Returns true if the value has arrived.
  bool IsReady() const {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    return impl_->values.has_value();
  }

  // Waits for the value to arrive, and returns the I-th value.
  //
  // Will CHECK if a timeout happens.
  //
  // Example usage:
  //
  //   TestFuture<int, std::string> future;
  //   int first = future.Get<0>();
  //   std::string second = future.Get<1>();
  //
  template <std::size_t I, typename T = TupleType>
    requires(internal::IsNonEmptyTuple<T>)
  const auto& Get() {
    return std::get<I>(GetTuple());
  }

  // Waits for the value to arrive, and returns the value with the given type.
  //
  // Will CHECK if a timeout happens.
  //
  // Example usage:
  //
  //   TestFuture<int, std::string> future;
  //   int first = future.Get<int>();
  //   std::string second = future.Get<std::string>();
  //
  template <typename Type>
  const auto& Get() {
    return std::get<Type>(GetTuple());
  }

  // Returns a callback that when invoked will store all the argument values,
  // and unblock any waiters. The callback must be invoked on the sequence the
  // TestFuture was created on.
  //
  // Templated so you can specify how you need the arguments to be passed -
  // const, reference, .... Defaults to simply `Types...`.
  //
  // Example usage:
  //
  //   TestFuture<int, std::string> future;
  //
  //   // Without specifying the callback argument types, this returns
  //   // base::OnceCallback<void(int, std::string)>.
  //   future.GetCallback();
  //
  //   // By explicitly specifying the callback argument types, this returns
  //   // base::OnceCallback<void(int, const std::string&)>.
  //   future.GetCallback<int, const std::string&>();
  //
  template <typename... CallbackArgumentsTypes>
  OnceCallback<void(CallbackArgumentsTypes...)> GetCallback() {
    return GetRepeatingCallback<CallbackArgumentsTypes...>();
  }

  OnceCallback<void(Types...)> GetCallback() { return GetCallback<Types...>(); }

  // Returns a repeating callback that when invoked will store all the argument
  // values, and unblock any waiters. The callback must be invoked on the
  // sequence the TestFuture was created on.
  //
  // You must take care that the stored value is consumed before the callback
  // is invoked a second time. You can consume the value by calling either
  // `Take()` or `Clear()`.
  //
  // Example usage:
  //
  //   TestFuture<std::string> future;
  //
  //   object_under_test.InstallCallback(future.GetRepeatingCallback());
  //
  //   object_under_test.DoSomething();
  //   EXPECT_EQ(future.Take(), "expected-first-value");
  //   // Because we used `Take()` the test future is ready for reuse.
  //
  //   object_under_test.DoSomethingElse();
  //   // We can also use `Get()` + `Clear()` to reuse the callback.
  //   EXPECT_EQ(future.Get(), "expected-second-value");
  //   future.Clear();
  //
  //   object_under_test.DoSomethingElse();
  //   EXPECT_EQ(future.Take(), "expected-third-value");
  //
  template <typename... CallbackArgumentsTypes>
  RepeatingCallback<void(CallbackArgumentsTypes...)> GetRepeatingCallback() {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    return BindRepeating(
        [](WeakPtr<Impl> impl, CallbackArgumentsTypes... values) {
          if (impl) {
            SetValueImpl(*impl,
                         std::forward<CallbackArgumentsTypes>(values)...);
          }
        },
        impl_->weak_ptr_factory.GetWeakPtr());
  }

  RepeatingCallback<void(Types...)> GetRepeatingCallback() {
    return GetRepeatingCallback<Types...>();
  }

  // Returns a callback that can be invoked on any sequence. When invoked it
  // will post a task to the sequence the TestFuture was created on, to store
  // all the argument values, and unblock any waiters.
  //
  // Templated so you can specify how you need the arguments to be passed -
  // const, reference, .... Defaults to simply `Types...`.
  //
  // Example usage:
  //
  //   TestFuture<int, std::string> future;
  //
  //   // Without specifying the callback argument types, this returns
  //   // base::OnceCallback<void(int, std::string)>.
  //   auto callback = future.GetSequenceBoundCallback();
  //
  //   // By explicitly specifying the callback argument types, this returns
  //   // base::OnceCallback<void(int, const std::string&)>.
  //   auto callback =
  //       future.GetSequenceBoundCallback<int, const std::string&>();
  //
  //   // AsyncOperation invokes `callback` with a result.
  //   other_task_runner->PostTask(FROM_HERE, base::BindOnce(&AsyncOperation,
  //                                              std::move(callback));
  //
  //   future.Wait();
  //
  template <typename... CallbackArgumentsTypes>
  OnceCallback<void(CallbackArgumentsTypes...)> GetSequenceBoundCallback() {
    return GetSequenceBoundRepeatingCallback<CallbackArgumentsTypes...>();
  }

  OnceCallback<void(Types...)> GetSequenceBoundCallback() {
    return GetSequenceBoundCallback<Types...>();
  }

  // Returns a repeating callback that can be invoked on any sequence. When
  // invoked it will post a task to the sequence the TestFuture was created on,
  // to store all the argument values, and unblock any waiters.
  //
  // You must take care that the stored value is consumed before the callback
  // is invoked a second time. You can consume the value by calling either
  // `Take()` or `Clear()`.
  //
  // Example usage:
  //
  //   base::SequenceBound<Object> object_under_test(other_task_runner);
  //   TestFuture<std::string> future;
  //
  //   object_under_test.AsyncCall(&Object::InstallCallback,
  //                               future.GetSequenceBoundRepeatingCallback());
  //
  //   object_under_test.AsyncCall(&DoSomething);
  //   EXPECT_EQ(future.Take(), "expected-first-value");
  //   // Because we used `Take()` the test future is ready for reuse.
  //
  //   object_under_test.AsyncCall(&DoSomethingElse);
  //   // We can also use `Get()` + `Clear()` to reuse the callback.
  //   EXPECT_EQ(future.Get(), "expected-second-value");
  //   future.Clear();
  //
  //   object_under_test.AsyncCall(&DoSomethingElse);
  //   EXPECT_EQ(future.Take(), "expected-third-value");
  //
  template <typename... CallbackArgumentsTypes>
  RepeatingCallback<void(CallbackArgumentsTypes...)>
  GetSequenceBoundRepeatingCallback() {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    return BindPostTask(base::SequencedTaskRunner::GetCurrentDefault(),
                        GetRepeatingCallback<CallbackArgumentsTypes...>());
  }

  RepeatingCallback<void(Types...)> GetSequenceBoundRepeatingCallback() {
    return GetSequenceBoundRepeatingCallback<Types...>();
  }

  // Sets the value of the future.
  // This will unblock any pending Wait() or Get() call.
  void SetValue(Types... values) {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    SetValueImpl(*impl_, std::forward<Types>(values)...);
  }

  // Clears the future, allowing it to be reused and accept a new value.
  //
  // All outstanding callbacks issued through `GetCallback()` remain valid.
  void Clear() {
    if (IsReady()) {
      std::ignore = Take();
    }
  }

  //////////////////////////////////////////////////////////////////////////////
  //  Accessor methods only available if the future holds a single value.
  //////////////////////////////////////////////////////////////////////////////

  // Waits for the value to arrive, and returns a reference to it.
  //
  // Will CHECK if a timeout happens.
  template <typename T = TupleType>
    requires(internal::IsSingleValuedTuple<T>)
  [[nodiscard]] const auto& Get() {
    return std::get<0>(GetTuple());
  }

  // Waits for the value to arrive, and returns it.
  //
  // Will CHECK if a timeout happens.
  template <typename T = TupleType>
    requires(internal::IsSingleValuedTuple<T>)
  [[nodiscard]] auto Take() {
    return std::get<0>(TakeTuple());
  }

  //////////////////////////////////////////////////////////////////////////////
  //  Accessor methods only available if the future holds multiple values.
  //////////////////////////////////////////////////////////////////////////////

  // Waits for the values to arrive, and returns a tuple with the values.
  //
  // Will CHECK if a timeout happens.
  template <typename T = TupleType>
    requires(internal::IsMultiValuedTuple<T>)
  [[nodiscard]] const TupleType& Get() {
    return GetTuple();
  }

  // Waits for the values to arrive, and moves a tuple with the values out.
  //
  // Will CHECK if a timeout happens.
  template <typename T = TupleType>
    requires(internal::IsMultiValuedTuple<T>)
  [[nodiscard]] TupleType Take() {
    return TakeTuple();
  }

 private:
  // Nested struct, used together with std::unique_ptr to make TestFuture
  // movable.
  struct Impl {
    Impl() = default;
    ~Impl() = default;

    SEQUENCE_CHECKER(sequence_checker);

    base::RepeatingClosure ready_signal GUARDED_BY_CONTEXT(sequence_checker) =
        base::DoNothing();

    std::optional<TupleType> values GUARDED_BY_CONTEXT(sequence_checker);

    WeakPtrFactory<Impl> weak_ptr_factory{this};
  };

  static void SetValueImpl(Impl& impl, Types... values) {
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl.sequence_checker);

    auto new_values = std::make_tuple(std::forward<Types>(values)...);

    EXPECT_FALSE(impl.values.has_value())
        << "Received new value " << ToString(new_values) << " before old value "
        << ToString(impl.values.value())
        << " was consumed through Take() or Clear().";

    impl.values = std::move(new_values);

    impl.ready_signal.Run();
  }

  void CheckNotUsedAfterMove() const {
    // `impl_` may only be null of `this` is an instance that has been moved
    // away, after which `this` becomes unusable.
    CHECK(impl_);
  }

  [[nodiscard]] const TupleType& GetTuple() {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    bool success = Wait();
    CHECK(success) << "Waiting for value timed out.";
    return impl_->values.value();
  }

  [[nodiscard]] TupleType TakeTuple() {
    CheckNotUsedAfterMove();
    DCHECK_CALLED_ON_VALID_SEQUENCE(impl_->sequence_checker);
    bool success = Wait();
    CHECK(success) << "Waiting for value timed out.";

    return std::exchange(impl_->values, {}).value();
  }

  std::unique_ptr<Impl> impl_ = std::make_unique<Impl>();
};

// Specialization so you can use `TestFuture` to wait for a no-args callback.
//
// This specialization offers a subset of the methods provided on the base
// `TestFuture`, as there is no value to be returned.
template <>
class TestFuture<void> {
 public:
  // Waits until the callback or `SetValue()` is invoked.
  //
  // Fails your test if a timeout happens, but you can check the return value
  // to improve the error reported:
  //
  //   ASSERT_TRUE(future.Wait()) << "Detailed error message";
  [[nodiscard]] bool Wait(
      RunLoop::Type run_loop_type = RunLoop::Type::kDefault) {
    return implementation_.Wait(run_loop_type);
  }

  // Same as above, then clears the future, allowing it to be reused and accept
  // a new value.
  [[nodiscard]] bool WaitAndClear(
      RunLoop::Type run_loop_type = RunLoop::Type::kDefault) {
    auto result = Wait(run_loop_type);
    Clear();
    return result;
  }

  // Waits until the callback or `SetValue()` is invoked.
  void Get() { std::ignore = implementation_.Get(); }

  // Returns true if the callback or `SetValue()` was invoked.
  bool IsReady() const { return implementation_.IsReady(); }

  // Returns a callback that when invoked will unblock any waiters.
  OnceClosure GetCallback() {
    return BindOnce(implementation_.GetCallback(), true);
  }

  // Returns a callback that when invoked will unblock any waiters.
  RepeatingClosure GetRepeatingCallback() {
    return BindRepeating(implementation_.GetRepeatingCallback(), true);
  }

  // Returns a callback that when invoked on any sequence will unblock any
  // waiters.
  OnceClosure GetSequenceBoundCallback() {
    return BindOnce(implementation_.GetSequenceBoundCallback(), true);
  }

  // Returns a callback that when invoked on any sequence will unblock any
  // waiters.
  RepeatingClosure GetSequenceBoundRepeatingCallback() {
    return BindRepeating(implementation_.GetSequenceBoundRepeatingCallback(),
                         true);
  }

  // Indicates this `TestFuture` is ready, and unblocks any waiters.
  void SetValue() { implementation_.SetValue(true); }

  // Clears the future, allowing it to be reused and accept a new value.
  //
  // All outstanding callbacks issued through `GetCallback()` remain valid.
  void Clear() { implementation_.Clear(); }

 private:
  TestFuture<bool> implementation_;
};

// A gmock action that when invoked will store the argument values and
// unblock any waiters. The action must be invoked on the sequence the
// TestFuture was created on.
//
// Usually the action will be used with `WillOnce()` and only invoked once,
// but if you consume the value with `Take()` or `Clear()` it is safe to
// invoke it again.
//
// Example usage:
//   TestFuture<int> future;
//
//   EXPECT_CALL(delegate, OnReadComplete)
//     .WillOnce(InvokeFuture(future));
//
//   object_under_test.Read(buffer, 16);
//
//   EXPECT_EQ(future.Take(), 16);
//
//
//
// Implementation note: this is not implemented using the MATCHER_P macro as the
// C++03-compatible way it implements varargs would make this too verbose.
// Instead, it takes advantage of the ability to pass a functor to .WillOnce()
// and .WillRepeatedly().
template <typename... Types>
class InvokeFuture {
 public:
  // The TestFuture must be an lvalue. Passing an rvalue would make no sense as
  // you wouldn't be able to call Take() on it afterwards.
  explicit InvokeFuture(TestFuture<Types...>& future)
      : callback_(future.GetRepeatingCallback()) {}

  // GMock actions must be copyable.
  InvokeFuture(const InvokeFuture&) = default;
  InvokeFuture& operator=(const InvokeFuture&) = default;

  // WillOnce() can take advantage of move constructors.
  InvokeFuture(InvokeFuture&&) = default;
  InvokeFuture& operator=(InvokeFuture&&) = default;

  void operator()(Types... values) {
    callback_.Run(std::forward<Types>(values)...);
  }

 private:
  RepeatingCallback<void(Types...)> callback_;
};

// Specialization for TestFuture<void>.
template <>
class InvokeFuture<void> {
 public:
  explicit InvokeFuture(TestFuture<void>& future)
      : closure_(future.GetRepeatingCallback()) {}

  InvokeFuture(const InvokeFuture&) = default;
  InvokeFuture& operator=(const InvokeFuture&) = default;
  InvokeFuture(InvokeFuture&&) = default;
  InvokeFuture& operator=(InvokeFuture&&) = default;

  void operator()() { closure_.Run(); }

 private:
  RepeatingClosure closure_;
};

// Deduction guide so the compiler can choose the correct specialisation of
// InvokeFuture.
template <typename... Types>
InvokeFuture(TestFuture<Types...>&) -> InvokeFuture<Types...>;

}  // namespace base::test

#endif  // BASE_TEST_TEST_FUTURE_H_
