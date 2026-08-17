// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DATA_TRANSFER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DATA_TRANSFER_LIST_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class AudioData;

class MODULES_EXPORT AudioDataTransferList
    : public GarbageCollected<AudioDataTransferList>,
      public Transferables::TransferList {
 public:
  static const void* const kTransferListKey;

  AudioDataTransferList() = default;
  ~AudioDataTransferList() override = default;

  void FinalizeTransfer(ExceptionState&) override;

  void Trace(Visitor*) const override;

  HeapVector<Member<AudioData>> audio_data_collection;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_AUDIO_DATA_TRANSFER_LIST_H_
