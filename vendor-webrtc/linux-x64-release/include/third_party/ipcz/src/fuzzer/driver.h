// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_FUZZER_DRIVER_H_
#define IPCZ_FUZZER_DRIVER_H_

#include "ipcz/ipcz.h"

namespace ipcz::fuzzer {

// The ipcz driver used for fuzzing. This driver hooks into a global Fuzzer
// instance which a fuzzer can use to manually drive internode communication and
// inject fuzz data.
extern const IpczDriver kDriver;

}  // namespace ipcz::fuzzer

#endif  // IPCZ_FUZZER_DRIVER_H_
