// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINDOWSX_SHIM_H_
#define BASE_WIN_WINDOWSX_SHIM_H_

// Any Chromium headers which want to `#include <windowsx.h>` should instead
// #include this header.

#include <windowsx.h>

// `<windowsx.h>` #defines some macros which conflict with function names in
// Chromium. Use the expanded form given below if you wanted to use one of these
// macros.
#undef GetNextSibling  // `GetWindow(hwnd, GW_HWNDNEXT)`
#undef GetFirstChild   // `GetTopWindow(hwnd)`
#undef IsMaximized     // `IsZoomed()`
#undef IsMinimized     // `IsIconic()`
#undef IsRestored  // `!(GetWindowStyle(hwnd) & (WS_MINIMIZE | WS_MAXIMIZE))`

#endif  // BASE_WIN_WINDOWSX_SHIM_H_
