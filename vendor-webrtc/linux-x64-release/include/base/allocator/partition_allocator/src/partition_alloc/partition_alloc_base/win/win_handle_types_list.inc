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

PA_WINDOWS_HANDLE_TYPE(HDC)
PA_WINDOWS_HANDLE_TYPE(HDESK)
PA_WINDOWS_HANDLE_TYPE(HGLRC)
PA_WINDOWS_HANDLE_TYPE(HICON)
PA_WINDOWS_HANDLE_TYPE(HINSTANCE)
PA_WINDOWS_HANDLE_TYPE(HKEY)
PA_WINDOWS_HANDLE_TYPE(HKL)
PA_WINDOWS_HANDLE_TYPE(HMENU)
PA_WINDOWS_HANDLE_TYPE(HWINSTA)
PA_WINDOWS_HANDLE_TYPE(HWND)
PA_WINDOWS_HANDLE_TYPE(HMONITOR)
