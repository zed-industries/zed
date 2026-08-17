// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_TRANSFER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_TRANSFER_LIST_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class RTCDataChannel;

class MODULES_EXPORT RTCDataChannelTransferList
    : public GarbageCollected<RTCDataChannelTransferList>,
      public Transferables::TransferList {
 public:
  static const void* const kTransferListKey;

  RTCDataChannelTransferList() = default;
  ~RTCDataChannelTransferList() override = default;

  void Trace(Visitor*) const override;

  HeapVector<Member<RTCDataChannel>> data_channel_collection;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_TRANSFER_LIST_H_
