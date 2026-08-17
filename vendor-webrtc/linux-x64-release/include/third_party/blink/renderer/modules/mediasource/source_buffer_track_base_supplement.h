// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_TRACK_BASE_SUPPLEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_TRACK_BASE_SUPPLEMENT_H_

#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class TrackBase;
class SourceBuffer;

class SourceBufferTrackBaseSupplement
    : public GarbageCollected<SourceBufferTrackBaseSupplement>,
      public Supplement<TrackBase> {
 public:
  static const char kSupplementName[];

  static SourceBuffer* sourceBuffer(TrackBase&);
  static void SetSourceBuffer(TrackBase&, SourceBuffer*);

  SourceBufferTrackBaseSupplement(TrackBase& track);
  void Trace(Visitor*) const override;

 private:
  static SourceBufferTrackBaseSupplement& From(TrackBase&);
  static SourceBufferTrackBaseSupplement* FromIfExists(TrackBase&);

  Member<SourceBuffer> source_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_SOURCE_BUFFER_TRACK_BASE_SUPPLEMENT_H_
