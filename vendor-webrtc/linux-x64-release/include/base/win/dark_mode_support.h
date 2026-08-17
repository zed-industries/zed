// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_DARK_MODE_SUPPORT_H_
#define BASE_WIN_DARK_MODE_SUPPORT_H_

#include "base/base_export.h"
#include "base/win/windows_types.h"

namespace base::win {

// Returns true if this version of Windows supports dark mode.
BASE_EXPORT bool IsDarkModeAvailable();

// Sets whether the process can support Windows dark mode.
BASE_EXPORT void AllowDarkModeForApp(bool allow);

// Sets whether the given HWND can support Windows dark mode.
BASE_EXPORT bool AllowDarkModeForWindow(HWND hwnd, bool allow);

}  // namespace base::win

#endif  // BASE_WIN_DARK_MODE_SUPPORT_H_
