// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_POST_ASYNC_RESULTS_H_
#define BASE_WIN_POST_ASYNC_RESULTS_H_

#include <unknwn.h>

#include <windows.foundation.h>
#include <wrl/async.h>
#include <wrl/client.h>

#include <type_traits>
#include <utility>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/logging.h"
#include "base/task/bind_post_task.h"
#include "base/task/sequenced_task_runner.h"

namespace base {
namespace win {

namespace internal {

// Utility function to pretty print enum values.
constexpr const char* ToCString(AsyncStatus async_status) {
  switch (async_status) {
    case AsyncStatus::Started:
      return "AsyncStatus::Started";
    case AsyncStatus::Completed:
      return "AsyncStatus::Completed";
    case AsyncStatus::Canceled:
      return "AsyncStatus::Canceled";
    case AsyncStatus::Error:
      return "AsyncStatus::Error";
  }

  NOTREACHED();
}

template <typename T>
using IAsyncOperationT = typename ABI::Windows::Foundation::IAsyncOperation<T>;

template <typename T>
using IAsyncOperationCompletedHandlerT =
    typename base::OnceCallback<void(IAsyncOperationT<T>*, AsyncStatus)>;

template <typename T>
using AsyncAbiT = typename ABI::Windows::Foundation::Internal::GetAbiType<
    typename IAsyncOperationT<T>::TResult_complex>::type;

// Compile time switch to decide what container to use for the async results for
// |T|. Depends on whether the underlying Abi type is a pointer to IUnknown or
// not. It queries the internals of Windows::Foundation to obtain this
// information.
template <typename T>
using AsyncResultsT = std::conditional_t<
    std::is_convertible_v<AsyncAbiT<T>, IUnknown*>,
    Microsoft::WRL::ComPtr<std::remove_pointer_t<AsyncAbiT<T>>>,
    AsyncAbiT<T>>;

// Fetches the result of the provided |async_operation| and corresponding
// |async_status| and assigns that value to |result|. Returns an HRESULT
// indicating the success of the operation.
template <typename T>
HRESULT GetAsyncResultsT(IAsyncOperationT<T>* async_operation,
                         AsyncStatus async_status,
                         AsyncResultsT<T>* results) {
  if (async_status == AsyncStatus::Completed) {
    // To expose |results| to GetResults as the expected type, this call first
    // dereferences |results| from ComPtr<T>* or T* to ComPtr<T> or T
    // respectively, then requests the address, converting to T** or T*
    // respectively.
    HRESULT hr = async_operation->GetResults(&(*results));
    if (FAILED(hr)) {
      *results = AsyncResultsT<T>{};
    }
    return hr;
  }

  *results = AsyncResultsT<T>{};
  Microsoft::WRL::ComPtr<ABI::Windows::Foundation::IAsyncInfo> async_info;
  HRESULT hr = async_operation->QueryInterface(IID_PPV_ARGS(&async_info));
  if (FAILED(hr)) {
    return hr;
  }

  HRESULT operation_hr;
  hr = async_info->get_ErrorCode(&operation_hr);
  if (FAILED(hr)) {
    return hr;
  }

  DCHECK(FAILED(operation_hr));
  return operation_hr;
}

// Registers an internal completion handler for |async_operation| and upon
// completion, posts the results to the provided |completed_handler|. Returns an
// HRESULT indicating the success of registering the internal completion
// handler.
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The
// |completed_handler| will be run on the same sequence that invoked this
// method. This call does not ensure the lifetime of the |async_operation|,
// which must be done by the caller.
template <typename T>
HRESULT PostAsyncOperationCompletedHandler(
    IAsyncOperationT<T>* async_operation,
    IAsyncOperationCompletedHandlerT<T> completed_handler) {
  using AsyncResult =
      std::pair<Microsoft::WRL::ComPtr<IAsyncOperationT<T>>, AsyncStatus>;

  auto internal_completed_handler =
      base::BindOnce([](IAsyncOperationT<T>* async_operation,
                        AsyncStatus async_status) -> AsyncResult {
        // Posting the results to the TaskRunner is required, since this
        // CompletedHandler might be invoked on an arbitrary thread however
        // the raw |async_operation| pointer is only guaranteed to be valid
        // for the lifetime of this call, so to ensure it is still valid
        // through the lifetime of the call to the |completed_handler| we
        // capture it in an appropriate ref-counted pointer.
        return std::make_pair(async_operation, async_status);
      })
          .Then(base::BindPostTaskToCurrentDefault(base::BindOnce(
              [](IAsyncOperationCompletedHandlerT<T> completed_handler,
                 AsyncResult async_result) {
                std::move(completed_handler)
                    .Run(async_result.first.Get(), async_result.second);
              },
              std::move(completed_handler))));

  using CompletedHandler = Microsoft::WRL::Implements<
      Microsoft::WRL::RuntimeClassFlags<Microsoft::WRL::ClassicCom>,
      ABI::Windows::Foundation::IAsyncOperationCompletedHandler<T>,
      Microsoft::WRL::FtmBase>;

  return async_operation->put_Completed(
      Microsoft::WRL::Callback<CompletedHandler>(
          [internal_completed_handler(std::move(internal_completed_handler))](
              IAsyncOperationT<T>* async_operation,
              AsyncStatus async_status) mutable {
            std::move(internal_completed_handler)
                .Run(async_operation, async_status);
            return S_OK;
          })
          .Get());
}

}  // namespace internal

// Registers an internal completion handler for |async_operation| and upon
// successful completion invokes the |success_callback| with the result. If the
// |async_operation| encounters an error no callback will be invoked. Returns
// an HRESULT indicating the success of registering the internal completion
// handler.
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The resulting
// callback (i.e. |success_callback|) will be run on the same sequence that
// invoked this method. This call does not ensure the lifetime of the
// |async_operation|, which must be done by the caller.
template <typename T>
HRESULT PostAsyncHandlers(
    internal::IAsyncOperationT<T>* async_operation,
    base::OnceCallback<void(internal::AsyncResultsT<T>)> success_callback) {
  return internal::PostAsyncOperationCompletedHandler(
      async_operation,
      base::BindOnce(
          [](base::OnceCallback<void(internal::AsyncResultsT<T>)>
                 success_callback,
             internal::IAsyncOperationT<T>* async_operation,
             AsyncStatus async_status) {
            internal::AsyncResultsT<T> results;
            HRESULT hr = internal::GetAsyncResultsT(async_operation,
                                                    async_status, &results);
            if (SUCCEEDED(hr)) {
              std::move(success_callback).Run(results);
            }
          },
          std::move(success_callback)));
}

// Registers an internal completion handler for |async_operation| and upon
// successful completion invokes the |success_callback| with the result. If the
// |async_operation| encounters an error the |failure_callback| will instead be
// invoked. Returns an HRESULT indicating the success of registering the
// internal completion handler.
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The resulting
// callback (|success_callback| or |failure_callback|) will be run on the same
// sequence that invoked this method. This call does not ensure the lifetime of
// the |async_operation|, which must be done by the caller.
template <typename T>
HRESULT PostAsyncHandlers(
    internal::IAsyncOperationT<T>* async_operation,
    base::OnceCallback<void(internal::AsyncResultsT<T>)> success_callback,
    base::OnceCallback<void()> failure_callback) {
  return internal::PostAsyncOperationCompletedHandler(
      async_operation,
      base::BindOnce(
          [](base::OnceCallback<void(internal::AsyncResultsT<T>)>
                 success_callback,
             base::OnceCallback<void()> failure_callback,
             internal::IAsyncOperationT<T>* async_operation,
             AsyncStatus async_status) {
            internal::AsyncResultsT<T> results;
            HRESULT hr = internal::GetAsyncResultsT(async_operation,
                                                    async_status, &results);
            if (SUCCEEDED(hr)) {
              std::move(success_callback).Run(results);
            } else {
              std::move(failure_callback).Run();
            }
          },
          std::move(success_callback), std::move(failure_callback)));
}

// Registers an internal completion handler for |async_operation| and upon
// successful completion invokes the |success_callback| with the result. If the
// |async_operation| encounters an error the |failure_callback| will instead be
// invoked with the failing HRESULT. Returns an HRESULT indicating the success
// of registering the internal completion handler.
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The resulting
// callback (|success_callback| or |failure_callback|) will be run on the same
// sequence that invoked this method. This call does not ensure the lifetime of
// the |async_operation|, which must be done by the caller.
template <typename T>
HRESULT PostAsyncHandlers(
    internal::IAsyncOperationT<T>* async_operation,
    base::OnceCallback<void(internal::AsyncResultsT<T>)> success_callback,
    base::OnceCallback<void(HRESULT)> failure_callback) {
  return internal::PostAsyncOperationCompletedHandler(
      async_operation,
      base::BindOnce(
          [](base::OnceCallback<void(internal::AsyncResultsT<T>)>
                 success_callback,
             base::OnceCallback<void(HRESULT)> failure_callback,
             internal::IAsyncOperationT<T>* async_operation,
             AsyncStatus async_status) {
            internal::AsyncResultsT<T> results;
            HRESULT hr = internal::GetAsyncResultsT(async_operation,
                                                    async_status, &results);
            if (SUCCEEDED(hr)) {
              std::move(success_callback).Run(results);
            } else {
              std::move(failure_callback).Run(hr);
            }
          },
          std::move(success_callback), std::move(failure_callback)));
}

// Registers an internal completion handler for |async_operation| and upon
// successful completion invokes the |success_callback| with the result. If the
// |async_operation| encounters an error the |failure_callback| will instead be
// invoked with the result and an HRESULT indicating the success of fetching
// that result (NOT an HRESULT expressing the failure of the operation). Returns
// an HRESULT indicating the success of registering the internal completion
// handler.
//
// This overload is designed for (uncommon) operations whose results encapsulate
// success and failure information (and as a result of that are expected to be
// available under both success and failure conditions).
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The resulting
// callback (|success_callback| or |failure_callback|) will be run on the same
// sequence that invoked this method. This call does not ensure the lifetime of
// the |async_operation|, which must be done by the caller.
template <typename T>
HRESULT PostAsyncHandlers(
    internal::IAsyncOperationT<T>* async_operation,
    base::OnceCallback<void(internal::AsyncResultsT<T>)> success_callback,
    base::OnceCallback<void(HRESULT, internal::AsyncResultsT<T>)>
        failure_callback) {
  return internal::PostAsyncOperationCompletedHandler(
      async_operation,
      base::BindOnce(
          [](base::OnceCallback<void(internal::AsyncResultsT<T>)>
                 success_callback,
             base::OnceCallback<void(HRESULT, internal::AsyncResultsT<T>)>
                 failure_callback,
             internal::IAsyncOperationT<T>* async_operation,
             AsyncStatus async_status) {
            internal::AsyncResultsT<T> results;
            HRESULT hr = internal::GetAsyncResultsT(
                async_operation,
                async_status == AsyncStatus::Error ? AsyncStatus::Completed
                                                   : async_status,
                &results);
            if (SUCCEEDED(hr) && async_status == AsyncStatus::Completed) {
              std::move(success_callback).Run(results);
            } else {
              std::move(failure_callback).Run(hr, results);
            }
          },
          std::move(success_callback), std::move(failure_callback)));
}

// Deprecated.
//
// Registers an internal completion handler for |async_operation| and upon
// invocation, posts the results to the provided |callback|. Returns an HRESULT
// indicating the success of registering the internal completion handler.
//
// Callers need to ensure that this method is invoked in the correct COM
// apartment, i.e. the one that created |async_operation|. The |callback| will
// be run on the same sequence that invoked this method.
//
// WARNING: This call holds a reference to the provided |async_operation| until
// it completes.
template <typename T>
HRESULT PostAsyncResults(
    Microsoft::WRL::ComPtr<internal::IAsyncOperationT<T>> async_operation,
    base::OnceCallback<void(internal::AsyncResultsT<T>)> callback) {
  return internal::PostAsyncOperationCompletedHandler(
      async_operation.Get(),
      base::BindOnce(
          [](Microsoft::WRL::ComPtr<internal::IAsyncOperationT<T>>
                 original_async_operation,
             base::OnceCallback<void(internal::AsyncResultsT<T>)> callback,
             internal::IAsyncOperationT<T>* async_operation,
             AsyncStatus async_status) {
            DCHECK(original_async_operation.Get() == async_operation);
            if (async_status != AsyncStatus::Completed) {
              VLOG(2) << "Got unexpected AsyncStatus: "
                      << internal::ToCString(async_status);
            }

            internal::AsyncResultsT<T> results;
            HRESULT hr = internal::GetAsyncResultsT(async_operation,
                                                    async_status, &results);
            if (FAILED(hr)) {
              VLOG(2) << "GetAsyncResultsT failed: "
                      << logging::SystemErrorCodeToString(hr);
            }
            std::move(callback).Run(results);
          },
          async_operation, std::move(callback)));
}

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_POST_ASYNC_RESULTS_H_
