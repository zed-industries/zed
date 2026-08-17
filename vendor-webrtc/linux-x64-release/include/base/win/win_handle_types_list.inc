// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file intentionally does not have header guards, it's included
// inside a macro to generate enum values. The following line silences a
// presubmit and Tricium warning that would otherwise be triggered by this:
// no-include-guard-because-multiply-included
// NOLINT(build/header_guard)

// This file contains the list of Windows handle types that can be recognized
// via specific pointee types declared in //base/win/windows_types.h
// (e.g. `HDC` points to a fake/forward-declared `HDC__` struct).

CHROME_WINDOWS_HANDLE_TYPE(HBITMAP)
CHROME_WINDOWS_HANDLE_TYPE(HBRUSH)
CHROME_WINDOWS_HANDLE_TYPE(HDC)
CHROME_WINDOWS_HANDLE_TYPE(HDESK)
CHROME_WINDOWS_HANDLE_TYPE(HFONT)
CHROME_WINDOWS_HANDLE_TYPE(HGLRC)
CHROME_WINDOWS_HANDLE_TYPE(HICON)
CHROME_WINDOWS_HANDLE_TYPE(HINSTANCE)
CHROME_WINDOWS_HANDLE_TYPE(HKEY)
CHROME_WINDOWS_HANDLE_TYPE(HKL)
CHROME_WINDOWS_HANDLE_TYPE(HMENU)
CHROME_WINDOWS_HANDLE_TYPE(HPEN)
CHROME_WINDOWS_HANDLE_TYPE(HRGN)
CHROME_WINDOWS_HANDLE_TYPE(HWINSTA)
CHROME_WINDOWS_HANDLE_TYPE(HWND)
CHROME_WINDOWS_HANDLE_TYPE(HMONITOR)
