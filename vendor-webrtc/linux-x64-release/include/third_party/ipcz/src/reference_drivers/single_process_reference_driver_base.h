// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_SINGLE_PROCESS_REFERENCE_DRIVER_BASE_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_SINGLE_PROCESS_REFERENCE_DRIVER_BASE_H_

#include <functional>

#include "ipcz/ipcz.h"

namespace ipcz::reference_drivers {

// A partial IpczDriver providing implementation common to both the sync and
// and async single-process drivers.
extern const IpczDriver kSingleProcessReferenceDriverBase;

// Installs a hook to be invoked any time ReportBadTransportActivity() is called
// on any single-process reference driver. If called with null, any previously
// installed hook is removed.
using BadTransportActivityCallback =
    std::function<void(IpczDriverHandle, uintptr_t)>;
void SetBadTransportActivityCallback(BadTransportActivityCallback callback);

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_SINGLE_PROCESS_REFERENCE_DRIVER_BASE_H_
