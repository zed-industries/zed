// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file is designed to be included from source files that explicitly should
// not use <windows.h>. The goal is to stop <windows.h> from creeping back into
// the Chromium build, with the namespace pollution which that implies.
//
// See https://crbug.com/796644 for more historical context.
//
// If you don't know why windows.h is #included, there are multiple ways to find
// out. One is to manually compile the file with `/showIncludes:user` and look
// at the include tree that results. Another is to temporarily edit your local
// windows.h (which may be in third_party\depot_tools\win_toolchain or installed
// on the system) and add an #error directive, recompiling to see the
// problematic #include chain.

// Avoid header guards; if this file is somehow multiply-included, we want the
// preprocessor to check in every location. The next two lines disable PRESUBMIT
// and cpplint warnings for missing the guards.
// no-include-guard-because-multiply-included
// NOLINT(build/header_guard)

#if defined(_WINDOWS_)
#error Windows.h was included unexpectedly. See comment above for details.
#endif
