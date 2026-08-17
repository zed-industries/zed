// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_ATL_H_
#define BASE_WIN_ATL_H_

// Check no prior poisonous defines were made.
#include "base/win/windows_defines.inc"
// Undefine before windows header will make the poisonous defines
#include "base/win/windows_undefines.inc"

// clang-format off
// Declare our own exception thrower (atl_throw.h includes atldef.h).
#include "base/win/atl_throw.h"
// clang-format on

// Now include the real ATL headers.
#include <atlbase.h>

#include <atlcom.h>
#include <atlcomcli.h>
#include <atlctl.h>
#include <atlhost.h>
#include <atlsecurity.h>
#include <atltypes.h>
#include <atlwin.h>

// Undefine the poisonous defines
#include "base/win/windows_undefines.inc"  // NOLINT(build/include)
// Check no poisonous defines follow this include
#include "base/win/windows_defines.inc"  // NOLINT(build/include)

#endif  // BASE_WIN_ATL_H_
