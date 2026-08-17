// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_SEQUENCE_NUMBER_H_
#define IPCZ_SRC_IPCZ_SEQUENCE_NUMBER_H_

#include <cstdint>
#include <ostream>

#include "util/strong_alias.h"

namespace ipcz {

// Used to number arbitrary objects in a sequence.
//
// More specifically this is used by ipcz to maintain relative ordering of
// parcels against other parcels from the same source portal, or NodeLink
// messages against other NodeLink messages from the same NodeLink endpoint.
using SequenceNumber = StrongAlias<class SequenceNumberTag, uint64_t>;

constexpr SequenceNumber NextSequenceNumber(SequenceNumber n) {
  return SequenceNumber{n.value() + 1};
}

inline std::ostream& operator<<(std::ostream& stream, const SequenceNumber& n) {
  return stream << n.value();
}

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_SEQUENCE_NUMBER_H_
