// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_RANDOM_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_RANDOM_H_

#include <cstdint>

#include "third_party/abseil-cpp/absl/types/span.h"

namespace ipcz::reference_drivers {

void RandomBytes(absl::Span<uint8_t> destination);

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_RANDOM_H_
