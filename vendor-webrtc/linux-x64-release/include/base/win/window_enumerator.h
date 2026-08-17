// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINDOW_ENUMERATOR_H_
#define BASE_WIN_WINDOW_ENUMERATOR_H_

#include <string>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/win/windows_types.h"

namespace base::win {

// Enumerates immediate child windows of `parent`, running `filter` for each
// window until `filter` returns true.
using WindowEnumeratorCallback = base::RepeatingCallback<bool(HWND hwnd)>;
BASE_EXPORT void EnumerateChildWindows(HWND parent,
                                       WindowEnumeratorCallback filter);

// Returns true if `hwnd` is an always-on-top window.
BASE_EXPORT bool IsTopmostWindow(HWND hwnd);

// Returns true if `hwnd` is a system dialog.
BASE_EXPORT bool IsSystemDialog(HWND hwnd);

// Returns true if `hwnd` is a window owned by the Windows shell.
BASE_EXPORT bool IsShellWindow(HWND hwnd);

// Returns the class name of `hwnd` or an empty string on error.
BASE_EXPORT std::wstring GetWindowClass(HWND hwnd);

// Returns the window text for `hwnd`, or an empty string on error.
BASE_EXPORT std::wstring GetWindowTextString(HWND hwnd);

}  // namespace base::win

#endif  // BASE_WIN_WINDOW_ENUMERATOR_H_
