// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_IPCZ_PARCEL_QUEUE_H_
#define IPCZ_SRC_IPCZ_PARCEL_QUEUE_H_

#include "ipcz/parcel.h"
#include "ipcz/sequenced_queue.h"
#include "third_party/abseil-cpp/absl/base/macros.h"

namespace ipcz {

struct ParcelQueueTraits {
  static size_t GetElementSize(const std::unique_ptr<Parcel>& parcel) {
    ABSL_ASSERT(parcel);
    return parcel->data_view().size();
  }
};

// A ParcelQueue is a SequencedQueue of Parcel objects which also tracks the
// total data size (in bytes) of available parcels at the head of the queue.
using ParcelQueue = SequencedQueue<std::unique_ptr<Parcel>, ParcelQueueTraits>;

}  // namespace ipcz

#endif  // IPCZ_SRC_IPCZ_PARCEL_QUEUE_H_
