// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_WINRT_INITIALIZER_H_
#define BASE_WIN_SCOPED_WINRT_INITIALIZER_H_

#include <objbase.h>

#include "base/base_export.h"
#include "base/threading/thread_checker.h"
#include "base/win/scoped_windows_thread_environment.h"

namespace base {
namespace win {

// Initializes the Windows Runtime in the constructor and uninitalizes the
// Windows Runtime in the destructor. As a side effect, COM is also initialized
// as an MTA in the constructor and correspondingly uninitialized in the
// destructor.
//
// Generally, you should only use this on Windows 8 or above. It is redundant
// to use ScopedComInitializer in conjunction with ScopedWinrtInitializer.
//
// WARNING: This should only be used once per thread, ideally scoped to a
// similar lifetime as the thread itself. You should not be using this in random
// utility functions that make Windows Runtime calls -- instead ensure these
// functions are running on a Windows Runtime supporting thread!
class BASE_EXPORT ScopedWinrtInitializer
    : public ScopedWindowsThreadEnvironment {
 public:
  ScopedWinrtInitializer();

  ScopedWinrtInitializer(const ScopedWinrtInitializer&) = delete;
  ScopedWinrtInitializer& operator=(const ScopedWinrtInitializer&) = delete;

  ~ScopedWinrtInitializer() override;

  // ScopedWindowsThreadEnvironment:
  bool Succeeded() const override;

 private:
  const HRESULT hr_;
  THREAD_CHECKER(thread_checker_);
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_WINRT_INITIALIZER_H_
