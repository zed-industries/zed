// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_BUFFER_ID_H_
#define IPCZ_SRC_IPCZ_BUFFER_ID_H_

#include <cstdint>
#include <ostream>

#include "ipcz/link_side.h"
#include "util/strong_alias.h"

namespace ipcz {

// Identifies a shared memory buffer scoped to a NodeLink and owned by its
// NodeLinkMemory via a BufferPool. New BufferIds are allocated atomically by
// either side of the NodeLink.
using BufferId = StrongAlias<class BufferIdTag, uint64_t>;

constexpr BufferId kInvalidBufferId{UINT64_MAX};

inline std::ostream& operator<<(std::ostream& stream, const BufferId& id) {
  // For better log readability, output only the numeric value of the lower bits
  // with an A or B suffix to represent the high bit.
  constexpr uint64_t kIdMask = (1ull << kLinkSideBIdBit) - 1;
  stream << (id.value() & kIdMask)
         << (id.value() >> kLinkSideBIdBit ? ".B" : ".A");
  return stream;
}

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_BUFFER_ID_H_
