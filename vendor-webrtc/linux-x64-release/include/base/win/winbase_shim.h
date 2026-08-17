// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINBASE_SHIM_H_
#define BASE_WIN_WINBASE_SHIM_H_

// Any Chromium headers which want to `#include <winbase.h>` should instead
// #include this header.

#include <winbase.h>

// <winbase.h> defines macros mapping various common function names to have a
// `W` suffix. Undefine as necessary. If you need to call one of the relevant
// system APIs, use the full name (with trailing `W`) directly.
#undef GetUserName
#undef ReportEvent

#endif  // BASE_WIN_WINBASE_SHIM_H_
