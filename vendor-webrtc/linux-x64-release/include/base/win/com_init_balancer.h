// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_COM_INIT_BALANCER_H_
#define BASE_WIN_COM_INIT_BALANCER_H_

#include <objidl.h>
#include <winnt.h>
#include <wrl/implements.h>

#include <optional>

#include "base/base_export.h"
#include "base/threading/thread_checker.h"
#include "base/win/windows_types.h"

namespace base {
namespace win {
namespace internal {

// Implementation class of the IInitializeSpy Interface that prevents premature
// uninitialization of the COM library, often caused by unbalanced
// CoInitialize/CoUninitialize pairs. The use of this class is encouraged in
// COM-supporting threads that execute third-party code.
//
// Disable() must be called before uninitializing the COM library in order to
// revoke the registered spy and allow for the successful uninitialization of
// the COM library.
class BASE_EXPORT ComInitBalancer
    : public Microsoft::WRL::RuntimeClass<
          Microsoft::WRL::RuntimeClassFlags<Microsoft::WRL::ClassicCom>,
          IInitializeSpy> {
 public:
  // Constructs a COM initialize balancer. |co_init| defines the apartment's
  // concurrency model used by the balancer.
  explicit ComInitBalancer(DWORD co_init);

  ComInitBalancer(const ComInitBalancer&) = delete;
  ComInitBalancer& operator=(const ComInitBalancer&) = delete;

  ~ComInitBalancer() override;

  // Disables balancer by revoking the registered spy and consequently
  // unblocking attempts to uninitialize the COM library.
  void Disable();

  DWORD GetReferenceCountForTesting() const;

 private:
  // IInitializeSpy:
  IFACEMETHODIMP PreInitialize(DWORD apartment_type,
                               DWORD reference_count) override;
  IFACEMETHODIMP PostInitialize(HRESULT result,
                                DWORD apartment_type,
                                DWORD new_reference_count) override;
  IFACEMETHODIMP PreUninitialize(DWORD reference_count) override;
  IFACEMETHODIMP PostUninitialize(DWORD new_reference_count) override;

  const DWORD co_init_;

  // The current apartment reference count set after the completion of the last
  // call made to CoInitialize or CoUninitialize.
  DWORD reference_count_ = 0;

  std::optional<ULARGE_INTEGER> spy_cookie_;
  THREAD_CHECKER(thread_checker_);
};

}  // namespace internal
}  // namespace win
}  // namespace base

#endif  // BASE_WIN_COM_INIT_BALANCER_H_
