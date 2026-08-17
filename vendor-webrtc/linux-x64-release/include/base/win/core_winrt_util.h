// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_CORE_WINRT_UTIL_H_
#define BASE_WIN_CORE_WINRT_UTIL_H_

#include <hstring.h>
#include <inspectable.h>
#include <roapi.h>
#include <windef.h>

#include "base/base_export.h"
#include "base/win/scoped_hstring.h"

namespace base::win {

// The following stubs are provided for when component build is enabled, in
// order to avoid the propagation of delay-loading CoreWinRT to other modules.

BASE_EXPORT HRESULT RoGetActivationFactory(HSTRING class_id,
                                           const IID& iid,
                                           void** out_factory);

BASE_EXPORT HRESULT RoActivateInstance(HSTRING class_id,
                                       IInspectable** instance);

// Retrieves an activation factory for the type specified.
template <typename InterfaceType, wchar_t const* runtime_class_id>
HRESULT GetActivationFactory(InterfaceType** factory) {
  ScopedHString class_id_hstring = ScopedHString::Create(runtime_class_id);
  if (!class_id_hstring.is_valid()) {
    return E_FAIL;
  }

  return base::win::RoGetActivationFactory(class_id_hstring.get(),
                                           IID_PPV_ARGS(factory));
}

}  // namespace base::win

#endif  // BASE_WIN_CORE_WINRT_UTIL_H_
