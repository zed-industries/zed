// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_DEVINFO_H_
#define BASE_WIN_SCOPED_DEVINFO_H_

#include <setupapi.h>

#include "base/scoped_generic.h"

namespace base {
namespace win {

struct DevInfoScopedTraits {
  static HDEVINFO InvalidValue() { return INVALID_HANDLE_VALUE; }
  static void Free(HDEVINFO h) { SetupDiDestroyDeviceInfoList(h); }
};
using ScopedDevInfo = base::ScopedGeneric<HDEVINFO, DevInfoScopedTraits>;

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_DEVINFO_H_
