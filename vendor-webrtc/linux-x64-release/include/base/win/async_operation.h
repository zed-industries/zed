// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_ASYNC_OPERATION_H_
#define BASE_WIN_ASYNC_OPERATION_H_

#include <unknwn.h>

#include <windows.foundation.h>
#include <wrl/async.h>
#include <wrl/client.h>

#include <type_traits>
#include <utility>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/win/winrt_foundation_helpers.h"

namespace base {
namespace win {

// This file provides an implementation of Windows::Foundation::IAsyncOperation.
// Specializations exist for "regular" types and interface types that inherit
// from IUnknown. Both specializations expose a callback() method, which can be
// used to provide the result that will be forwarded to the registered
// completion handler. For regular types it expects an instance of that type,
// and for interface types it expects a corresponding ComPtr. This class is
// thread-affine and all member methods should be called on the same thread that
// constructed the object. In order to offload heavy result computation,
// base's PostTaskAndReplyWithResult() should be used with the ResultCallback
// passed as a reply.
//
// Example usages:
//
// // Regular types
// auto regular_op = WRL::Make<base::win::AsyncOperation<int>>();
// auto cb = regular_op->callback();
// regular_op->put_Completed(...event handler...);
// ...
// // This will invoke the event handler.
// std::move(cb).Run(123);
// ...
// // Results can be queried:
// int results = 0;
// regular_op->GetResults(&results);
// EXPECT_EQ(123, results);
//
// // Interface types
// auto interface_op = WRL::Make<base::win::AsyncOperation<FooBar*>>();
// auto cb = interface_op->callback();
// interface_op->put_Completed(...event handler...);
// ...
// // This will invoke the event handler.
// std::move(cb).Run(WRL::Make<IFooBarImpl>());
// ...
// // Results can be queried:
// WRL::ComPtr<IFooBar> results;
// interface_op->GetResults(&results);
// // |results| points to the provided IFooBarImpl instance.
//
// // Offloading a heavy computation:
// auto my_op = WRL::Make<base::win::AsyncOperation<FooBar*>>();
// base::ThreadPool::PostTaskAndReplyWithResult(
//     base::BindOnce(MakeFooBar), my_op->callback());

namespace internal {

// Template tricks needed to dispatch to the correct implementation below.
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

template <class T>
class AsyncOperation
    : public Microsoft::WRL::RuntimeClass<
          Microsoft::WRL::RuntimeClassFlags<
              Microsoft::WRL::WinRt | Microsoft::WRL::InhibitRoOriginateError>,
          ABI::Windows::Foundation::IAsyncOperation<T>> {
 public:
  using AbiT = internal::AsyncOperationAbi<T>;
  using OptionalStorageT = internal::AsyncOperationOptionalStorage<T>;
  using StorageT = internal::AsyncOperationStorage<T>;
  using Handler = ABI::Windows::Foundation::IAsyncOperationCompletedHandler<T>;
  using ResultCallback = base::OnceCallback<void(StorageT)>;

  AsyncOperation() {
    // Note: This can't be done in the constructor initializer list. This is
    // because it relies on weak_factory_ to be initialized, which needs to be
    // the last class member. Also applies below.
    callback_ =
        base::BindOnce(&AsyncOperation::OnResult, weak_factory_.GetWeakPtr());
  }

  AsyncOperation(const AsyncOperation&) = delete;
  AsyncOperation& operator=(const AsyncOperation&) = delete;

  ~AsyncOperation() override { DCHECK_CALLED_ON_VALID_THREAD(thread_checker_); }

  // ABI::Windows::Foundation::IAsyncOperation:
  IFACEMETHODIMP put_Completed(Handler* handler) override {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    handler_ = handler;
    return S_OK;
  }
  IFACEMETHODIMP get_Completed(Handler** handler) override {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    return handler_.CopyTo(handler);
  }
  IFACEMETHODIMP GetResults(AbiT* results) override {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    return results_ ? internal::CopyTo(results_, results) : E_PENDING;
  }

  ResultCallback callback() {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    DCHECK(!callback_.is_null());
    return std::move(callback_);
  }

 private:
  void InvokeCompletedHandler() {
    handler_->Invoke(this, ABI::Windows::Foundation::AsyncStatus::Completed);
  }

  void OnResult(StorageT result) {
    DCHECK_CALLED_ON_VALID_THREAD(thread_checker_);
    DCHECK(!results_);
    results_ = std::move(result);
    InvokeCompletedHandler();
  }

  ResultCallback callback_;
  Microsoft::WRL::ComPtr<Handler> handler_;
  OptionalStorageT results_;

  THREAD_CHECKER(thread_checker_);
  base::WeakPtrFactory<AsyncOperation> weak_factory_{this};
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_ASYNC_OPERATION_H_
