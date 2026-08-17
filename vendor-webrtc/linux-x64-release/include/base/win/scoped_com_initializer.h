// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_COM_INITIALIZER_H_
#define BASE_WIN_SCOPED_COM_INITIALIZER_H_

#include <objbase.h>

#include <wrl/client.h>

#include "base/base_export.h"
#include "base/threading/thread_checker.h"
#include "base/win/com_init_balancer.h"
#include "base/win/scoped_windows_thread_environment.h"

namespace base {
namespace win {

// Initializes COM in the constructor (STA or MTA), and uninitializes COM in the
// destructor.
//
// It is strongly encouraged to block premature uninitialization of the COM
// libraries in threads that execute third-party code, as a way to protect
// against unbalanced CoInitialize/CoUninitialize pairs.
//
// WARNING: This should only be used once per thread, ideally scoped to a
// similar lifetime as the thread itself.  You should not be using this in
// random utility functions that make COM calls -- instead ensure these
// functions are running on a COM-supporting thread!
class BASE_EXPORT ScopedCOMInitializer : public ScopedWindowsThreadEnvironment {
 public:
  // Enum value provided to initialize the thread as an MTA instead of STA.
  enum SelectMTA { kMTA };

  // Enum values which enumerates uninitialization modes for the COM library.
  enum class Uninitialization {
    // Default value. Used in threads where no third-party code is executed.
    kAllow,

    // Blocks premature uninitialization of the COM libraries before going out
    // of scope. Used in threads where third-party code is executed.
    kBlockPremature,
  };

  // Constructors for STA initialization.
  explicit ScopedCOMInitializer(
      Uninitialization uninitialization = Uninitialization::kAllow);

  // Constructors for MTA initialization.
  explicit ScopedCOMInitializer(
      SelectMTA mta,
      Uninitialization uninitialization = Uninitialization::kAllow);

  ScopedCOMInitializer(const ScopedCOMInitializer&) = delete;
  ScopedCOMInitializer& operator=(const ScopedCOMInitializer&) = delete;

  ~ScopedCOMInitializer() override;

  // ScopedWindowsThreadEnvironment:
  bool Succeeded() const override;

  // Returns the value returned by `CoInitializeEx` at initialization.
  HRESULT hr() const { return hr_; }

  // Used for testing. Returns the COM balancer's apartment thread ref count.
  DWORD GetCOMBalancerReferenceCountForTesting() const;

 private:
  void Initialize(COINIT init, Uninitialization uninitialization);

  HRESULT hr_ = S_OK;
  Microsoft::WRL::ComPtr<internal::ComInitBalancer> com_balancer_;
  THREAD_CHECKER(thread_checker_);
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_COM_INITIALIZER_H_
