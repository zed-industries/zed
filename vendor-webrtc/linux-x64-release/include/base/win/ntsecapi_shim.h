// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_NTSECAPI_SHIM_H_
#define BASE_WIN_NTSECAPI_SHIM_H_

// Any Chromium headers which want to `#include <ntsecapi.h>` should instead
// #include this header.

// By default, `<ntsecapi.h>` attempts to redefine various types like `STRING`
// in ways that are incompatible with `<winternl.h>`, which other files use. To
// prevent this, first `#include <lm.h>` and `<winternl.h>` to define the types,
// then `#define _NTDEF_` to prevent `<ntsecapi.h>` from redefinining them.
#include <winternl.h>

#include <lm.h>
#define _NTDEF_
#include <ntsecapi.h>

#endif  // BASE_WIN_NTSECAPI_SHIM_H_
