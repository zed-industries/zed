// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_ASYNC_RESULTS_TEST_VALUES_WIN_H_
#define BASE_TEST_ASYNC_RESULTS_TEST_VALUES_WIN_H_

#include <windows.foundation.collections.h>
#include <wrl/client.h>
#include <wrl/implements.h>

#include "testing/gtest/include/gtest/gtest.h"

// Declare the related template specializations for all the value types
// provided.
namespace ABI {
namespace Windows {
namespace Foundation {

template <>
struct __declspec(uuid("3895C200-8F26-4F5A-B29D-2B5D72E68F99"))
IAsyncOperation<IUnknown*> : IAsyncOperation_impl<IUnknown*> {};

template <>
struct __declspec(uuid("CD99A253-6473-4810-AF0D-763DAB79AC42"))
IAsyncOperationCompletedHandler<IUnknown*>
    : IAsyncOperationCompletedHandler_impl<IUnknown*> {};

template <>
struct __declspec(uuid("CB52D855-8121-4AC8-A164-084A27FB377E"))
IAsyncOperation<int*> : IAsyncOperation_impl<int*> {};

template <>
struct __declspec(uuid("EA868415-A724-40BC-950A-C7DB6B1723C6"))
IAsyncOperationCompletedHandler<int*>
    : IAsyncOperationCompletedHandler_impl<int*> {};

// These specialization templates were included in windows.foundation.h, but
// removed in 10.0.19041.0 SDK, so are included here conditionally
#ifdef NTDDI_WIN10_VB  // Windows 10.0.19041
template <>
struct __declspec(uuid("968b9665-06ed-5774-8f53-8edeabd5f7b5"))
IAsyncOperation<int> : IAsyncOperation_impl<int> {};

template <>
struct __declspec(uuid("d60cae9d-88cb-59f1-8576-3fba44796be8"))
IAsyncOperationCompletedHandler<int>
    : IAsyncOperationCompletedHandler_impl<int> {};
#endif

}  // namespace Foundation
}  // namespace Windows
}  // namespace ABI

namespace base {
namespace test {
// Provides access to values of type |T| and variations of those values relevant
// to IAsyncOperations. Intended for use in TypedTestSuites concerning
// IAsyncOperations or related functionality. Example:
//   template <typename T>
//   class SuiteName : public ::testing::Test {};
//
//   TYPED_TEST_SUITE_P(SuiteName);
//
//   TYPED_TEST_P(SuiteName, TestName) {
//     AsyncResultsTestValues<TypeParam> test_values;
//     ... test_values.GetTestValue_T() ...
//   }
//
//   REGISTER_TYPED_TEST_SUITE_P(SuiteName, TestName);
//   INSTANTIATE_TYPED_TEST_SUITE_P(Prefix,
//                                  SuiteName,
//                                  base::test::AsyncResultsTestValuesTypes);
template <typename T>
class AsyncResultsTestValues {
  // This class body serves only to provide documentation for the functions.
  // Actual use of this class is limited to the types for which it has been
  // specialized.
 private:
  class AsyncResultsT;

 public:
  // Returns a value equal to a variable of type T constructed with an
  // empty initializer.
  //
  // This value will be equal between all instances of the same type.
  //   AsyncResultsTestValues<T> instance1;
  //   AsyncResultsTestValues<T> instance2;
  //   instance1.GetDefaultValue_T() == instance2.GetDefaultValue_T();
  T GetDefaultValue_T();

  // Returns the same value as GetDefaultValue_T(), but in the format expected
  // for the results of an IAsyncOperation<T>.
  //   AsyncResultsTestValues<T> instance;
  //   AsyncResultsT<T> converted_value = instance.GetDefaultValue_T();
  //   converted_value == instance.GetDefaultValue_AsyncResultsT();
  AsyncResultsT GetDefaultValue_AsyncResultsT();

  // Returns an arbitrary value NOT equal to GetDefaultValue_T().
  //
  // Multiple calls to this function on a single instance will return values
  // equal to one another. Calls made on different instances may produce
  // equal or non-equal values.
  //   AsyncResultsTestValues<T> instance1;
  //   AsyncResultsTestValues<T> instance2;
  //   instance1.GetTestValue_T() == instance1.GetTestValue_T();
  //   instance1.GetTestValue_T() == OR != instance2.GetTestValue_T();
  T GetTestValue_T();

  // Returns the same value as GetTestValue_T(), but in the format expected for
  // the results of an IAsyncOperation<T>.
  //   AsyncResultsTestValues<T> instance;
  //   AsyncResultsT<T> converted_value = instance.GetTestValue_T();
  //   converted_value == instance.GetTestValue_AsyncResultsT();
  AsyncResultsT GetTestValue_AsyncResultsT();
};

// The collection of value types supported by AsyncResultsTestValues.
using AsyncResultsTestValuesTypes = ::testing::Types<int, int*, IUnknown*>;

template <>
class AsyncResultsTestValues<int> {
 public:
  int GetDefaultValue_T() { return 0; }
  int GetDefaultValue_AsyncResultsT() { return 0; }

  int GetTestValue_T() { return 4; }
  int GetTestValue_AsyncResultsT() { return 4; }
};

template <>
class AsyncResultsTestValues<int*> {
 public:
  int* GetDefaultValue_T() { return nullptr; }
  int* GetDefaultValue_AsyncResultsT() { return nullptr; }

  int* GetTestValue_T() { return &test_value_; }
  int* GetTestValue_AsyncResultsT() { return &test_value_; }

 private:
  int test_value_ = 4;
};

template <>
class AsyncResultsTestValues<IUnknown*> {
 public:
  AsyncResultsTestValues() {
    auto class_instance = Microsoft::WRL::Make<TestClassImplementingIUnknown>();
    class_instance.As(&test_value_);
  }

  IUnknown* GetDefaultValue_T() { return nullptr; }
  Microsoft::WRL::ComPtr<IUnknown> GetDefaultValue_AsyncResultsT() {
    return Microsoft::WRL::ComPtr<IUnknown>();
  }

  IUnknown* GetTestValue_T() { return test_value_.Get(); }
  Microsoft::WRL::ComPtr<IUnknown> GetTestValue_AsyncResultsT() {
    return test_value_;
  }

 private:
  class TestClassImplementingIUnknown
      : public Microsoft::WRL::RuntimeClass<
            Microsoft::WRL::RuntimeClassFlags<
                Microsoft::WRL::WinRtClassicComMix |
                Microsoft::WRL::InhibitRoOriginateError>,
            IUnknown> {};

  Microsoft::WRL::ComPtr<IUnknown> test_value_;
};

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_ASYNC_RESULTS_TEST_VALUES_WIN_H_
