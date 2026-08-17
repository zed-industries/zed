// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_ATL_THROW_H_
#define BASE_WIN_ATL_THROW_H_

#ifdef __ATLDEF_H__
#error atl_throw.h must be included before atldef.h.
#endif

#include "base/base_export.h"
#include "base/win/windows_types.h"

// Defining _ATL_NO_EXCEPTIONS causes ATL to raise a structured exception
// instead of throwing a CAtlException. While crashpad will eventually handle
// this, the HRESULT that caused the problem is lost. So, in addition, define
// our own custom AtlThrow function (_ATL_CUSTOM_THROW).
#ifndef _ATL_NO_EXCEPTIONS
#define _ATL_NO_EXCEPTIONS
#endif

#define _ATL_CUSTOM_THROW
#define AtlThrow ::base::win::AtlThrowImpl

namespace base {
namespace win {

// Crash the process forthwith in case of ATL errors.
[[noreturn]] BASE_EXPORT void __stdcall AtlThrowImpl(HRESULT hr);

}  // namespace win
}  // namespace base

#include <atldef.h>

// atldef.h mistakenly leaves out the declaration of this function when
// _ATL_CUSTOM_THROW is defined.
namespace ATL {
ATL_NOINLINE __declspec(noreturn) inline void WINAPI AtlThrowLastWin32();
}

#endif  // BASE_WIN_ATL_THROW_H_
