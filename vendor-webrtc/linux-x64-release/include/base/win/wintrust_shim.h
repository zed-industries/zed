// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_WINTRUST_SHIM_H_
#define BASE_WIN_WINTRUST_SHIM_H_

// Any Chromium headers which want to `#include <wintrust.h>` should instead
// #include this header.

#include <windows.h>

// <wintrust.h> #includes <wincrypt.h>, and so needs to shim it; see comments in
// wincrypt_shim.h.
// clang-format off
#include "base/win/wincrypt_shim.h"
#include <wintrust.h>
// clang-format on

#endif  // BASE_WIN_WINTRUST_SHIM_H_
