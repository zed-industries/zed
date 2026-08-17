// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_BUFFER_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class VideoEncoder;

class MODULES_EXPORT VideoEncoderBuffer final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit VideoEncoderBuffer(VideoEncoder* owner, size_t id);

  // video_encoder_buffer.idl implementation.
  String id() const;

  // GarbageCollected override.
  void Trace(Visitor*) const override;

  size_t internal_id() const { return id_; }
  VideoEncoder* owner() { return owner_; }

 private:
  const size_t id_;
  WeakMember<VideoEncoder> owner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_VIDEO_ENCODER_BUFFER_H_
