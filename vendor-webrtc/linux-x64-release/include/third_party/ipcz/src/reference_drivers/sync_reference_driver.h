// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_SYNC_REFERENCE_DRIVER_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_SYNC_REFERENCE_DRIVER_H_

#include "ipcz/ipcz.h"

namespace ipcz::reference_drivers {

// This is a fully synchronous driver for single-process use cases. Transmitting
// on one transport directly calls into the activity handler of its peer, so all
// node operations and therefore all ipcz operations complete synchronously from
// end to end.
extern const IpczDriver kSyncReferenceDriver;

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_SYNC_REFERENCE_DRIVER_H_
