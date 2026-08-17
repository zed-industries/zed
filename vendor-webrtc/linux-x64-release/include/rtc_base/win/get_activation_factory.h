/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_WIN_GET_ACTIVATION_FACTORY_H_
#define RTC_BASE_WIN_GET_ACTIVATION_FACTORY_H_

#include <winerror.h>

#include "rtc_base/win/hstring.h"

namespace webrtc {

// Provides access to Core WinRT functions which may not be available on
// Windows 7. Loads functions dynamically at runtime to prevent library
// dependencies.

// Callers must check the return value of ResolveCoreWinRTDelayLoad() before
// using these functions.

bool ResolveCoreWinRTDelayload();

HRESULT RoGetActivationFactoryProxy(HSTRING class_id,
                                    const IID& iid,
                                    void** out_factory);

// Retrieves an activation factory for the type specified.
template <typename InterfaceType, wchar_t const* runtime_class_id>
HRESULT GetActivationFactory(InterfaceType** factory) {
  HSTRING class_id_hstring;
  HRESULT hr = CreateHstring(runtime_class_id, wcslen(runtime_class_id),
                             &class_id_hstring);
  if (FAILED(hr))
    return hr;

  hr = RoGetActivationFactoryProxy(class_id_hstring, IID_PPV_ARGS(factory));
  if (FAILED(hr)) {
    DeleteHstring(class_id_hstring);
    return hr;
  }

  return DeleteHstring(class_id_hstring);
}

}  // namespace webrtc

#endif  // RTC_BASE_WIN_GET_ACTIVATION_FACTORY_H_
