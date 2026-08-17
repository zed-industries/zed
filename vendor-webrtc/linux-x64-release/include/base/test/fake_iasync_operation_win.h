// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_FAKE_IASYNC_OPERATION_WIN_H_
#define BASE_TEST_FAKE_IASYNC_OPERATION_WIN_H_

#include <wrl/client.h>
#include <wrl/implements.h>

#include "base/notreached.h"
#include "base/win/winrt_foundation_helpers.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {
namespace win {

namespace internal {

// Templates used to allow easy reference to the correct types.
// See base/win/winrt_foundation_helpers.h for explanation.
template <typename T>
using AsyncOperationComplex =
    typename ABI::Windows::Foundation::IAsyncOperation<T>::TResult_complex;

template <typename T>
using AsyncOperationAbi = AbiType<AsyncOperationComplex<T>>;

template <typename T>
using AsyncOperationOptionalStorage =
    OptionalStorageType<AsyncOperationComplex<T>>;

template <typename T>
using AsyncOperationStorage = StorageType<AsyncOperationComplex<T>>;

}  // namespace internal

// Provides an implementation of Windows::Foundation::IAsyncOperation for
// use in GTests.
template <typename T>
class FakeIAsyncOperation final
    : public Microsoft::WRL::RuntimeClass<
          Microsoft::WRL::RuntimeClassFlags<
              Microsoft::WRL::WinRt | Microsoft::WRL::InhibitRoOriginateError>,
          ABI::Windows::Foundation::IAsyncOperation<T>,
          ABI::Windows::Foundation::IAsyncInfo> {
 public:
  FakeIAsyncOperation() = default;
  FakeIAsyncOperation(const FakeIAsyncOperation&) = delete;
  FakeIAsyncOperation& operator=(const FakeIAsyncOperation&) = delete;

  // ABI::Windows::Foundation::IAsyncOperation:
  IFACEMETHODIMP put_Completed(
      ABI::Windows::Foundation::IAsyncOperationCompletedHandler<T>* handler)
      final {
    EXPECT_EQ(nullptr, handler_)
        << "put_Completed called on IAsyncOperation with a CompletedHandler "
           "already defined.";
    handler_ = handler;
    return S_OK;
  }
  IFACEMETHODIMP get_Completed(
      ABI::Windows::Foundation::IAsyncOperationCompletedHandler<T>** handler)
      final {
    NOTREACHED();
  }
  IFACEMETHODIMP GetResults(internal::AsyncOperationAbi<T>* results) final {
    if (!is_complete_) {
      ADD_FAILURE() << "GetResults called on incomplete IAsyncOperation.";
      return E_PENDING;
    }
    if (status_ != AsyncStatus::Completed && !results_includes_failure_) {
      return E_UNEXPECTED;
    }
    return base::win::internal::CopyTo(results_, results);
  }

  // ABI::Windows::Foundation::IAsyncInfo:
  IFACEMETHODIMP get_Id(uint32_t* id) final { NOTREACHED(); }
  IFACEMETHODIMP get_Status(AsyncStatus* status) final {
    *status = status_;
    return S_OK;
  }
  IFACEMETHODIMP get_ErrorCode(HRESULT* error_code) final {
    EXPECT_FALSE(results_includes_failure_)
        << "get_ErrorCode called on IAsyncOperation whose failure is expected "
           "to be expressed through the results instead. If a case arises "
           "where this is actually intended this check can be removed, but is "
           "most likely an indication of incorrectly assuming the error_code "
           "can be used in place of get_Status or GetResults for this kind of "
           "IAsyncOperation.";
    *error_code = error_code_;
    return S_OK;
  }
  IFACEMETHODIMP Cancel() final { NOTREACHED(); }
  IFACEMETHODIMP Close() final { NOTREACHED(); }

  // Completes the operation with |error_code|.
  //
  // The get_ErrorCode API will be set to return |error_code|, the remainder of
  // the APIs will be set to represent an error state, and the CompletedHandler
  // (if defined) will be run.
  void CompleteWithError(HRESULT error_code) {
    error_code_ = error_code;
    status_ = AsyncStatus::Error;
    InvokeCompletedHandler();
  }

  // Completes the operation with |results|, but with an AsyncStatus of Error.
  // This is an uncommon combination only appropriate when |results| includes
  // the failure information.
  //
  // The GetResults API will be set to return |results| and the get_ErrorCode
  // API will be set to return S_OK, but the get_Status API will be set to
  // return AsyncStatus::Error. Then the CompletedHandler (if defined) will be
  // run.
  void CompleteWithErrorResult(internal::AsyncOperationStorage<T> results) {
    error_code_ = S_OK;
    results_ = std::move(results);
    results_includes_failure_ = true;
    status_ = AsyncStatus::Error;
    InvokeCompletedHandler();
  }

  // Completes the operation with |results|.
  //
  // The GetResults API will be set to return |results|, the remainder of the
  // APIs will be set to represent a successfully completed state, and the
  // CompletedHandler (if defined) will be run.
  void CompleteWithResults(internal::AsyncOperationStorage<T> results) {
    error_code_ = S_OK;
    results_ = std::move(results);
    status_ = AsyncStatus::Completed;
    InvokeCompletedHandler();
  }

 private:
  void InvokeCompletedHandler() {
    ASSERT_FALSE(is_complete_)
        << "Attempted to invoke completion on an already "
           "completed IAsyncOperation.";
    is_complete_ = true;
    if (handler_) {
      handler_->Invoke(this, status_);
    }
  }

  HRESULT error_code_ = S_OK;
  Microsoft::WRL::ComPtr<
      ABI::Windows::Foundation::IAsyncOperationCompletedHandler<T>>
      handler_;
  bool is_complete_ = false;
  internal::AsyncOperationOptionalStorage<T> results_;
  bool results_includes_failure_ = false;
  AsyncStatus status_ = AsyncStatus::Started;
};

}  // namespace win
}  // namespace base

#endif  // BASE_TEST_FAKE_IASYNC_OPERATION_WIN_H_
