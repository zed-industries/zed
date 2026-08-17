// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_CRC32_H_
#define BASE_METRICS_CRC32_H_

#include <stddef.h>
#include <stdint.h>

#include <array>

#include "base/base_export.h"
#include "base/containers/span.h"

namespace base {

// This provides a simple, fast CRC-32 calculation that can be used for checking
// the integrity of data.  It is not a "secure" calculation!  |sum| can start
// with any seed or be used to continue an operation began with previous data.
BASE_EXPORT uint32_t Crc32(uint32_t sum, span<const uint8_t> data);

}  // namespace base

#endif  // BASE_METRICS_CRC32_H_
