// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_TRANSFER_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_TRANSFER_LIST_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/transferables.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class VideoFrame;

class MODULES_EXPORT VideoFrameTransferList
    : public GarbageCollected<VideoFrameTransferList>,
      public Transferables::TransferList {
 public:
  static const void* const kTransferListKey;

  VideoFrameTransferList() = default;
  ~VideoFrameTransferList() override = default;

  void FinalizeTransfer(ExceptionState&) override;

  void Trace(Visitor*) const override;

  HeapVector<Member<VideoFrame>> video_frames;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_TRANSFER_LIST_H_
