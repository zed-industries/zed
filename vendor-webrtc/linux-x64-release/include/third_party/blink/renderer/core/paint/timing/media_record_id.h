// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_MEDIA_RECORD_ID_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_MEDIA_RECORD_ID_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
class LayoutObject;
class MediaTiming;

using MediaRecordIdHash = size_t;

class MediaRecordId {
  STACK_ALLOCATED();

 public:
  static MediaRecordIdHash CORE_EXPORT GenerateHash(const LayoutObject* layout,
                                                    const MediaTiming* media);

  MediaRecordId(const LayoutObject* layout, const MediaTiming* media);

  MediaRecordIdHash GetHash() const { return hash_; }
  const LayoutObject* GetLayoutObject() const { return layout_object_; }
  const MediaTiming* GetMediaTiming() const { return media_timing_; }

 private:
  const LayoutObject* const layout_object_;
  const MediaTiming* const media_timing_;
  const MediaRecordIdHash hash_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_TIMING_MEDIA_RECORD_ID_H_
