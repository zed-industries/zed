// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_ATTACHMENT_H_

#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/video_frame_handle.h"

namespace blink {

// Used to serialize video frames without copying frame data.
class MODULES_EXPORT VideoFrameAttachment
    : public SerializedScriptValue::Attachment {
 public:
  static const void* const kAttachmentKey;
  VideoFrameAttachment() = default;
  ~VideoFrameAttachment() override = default;

  bool IsLockedToAgentCluster() const override {
    return !frame_handles_.empty();
  }

  size_t size() const { return frame_handles_.size(); }

  Vector<scoped_refptr<VideoFrameHandle>>& Handles() { return frame_handles_; }

  const Vector<scoped_refptr<VideoFrameHandle>>& Handles() const {
    return frame_handles_;
  }

 private:
  Vector<scoped_refptr<VideoFrameHandle>> frame_handles_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_FRAME_ATTACHMENT_H_
