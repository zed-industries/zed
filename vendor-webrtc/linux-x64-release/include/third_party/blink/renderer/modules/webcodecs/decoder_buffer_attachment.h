// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_BUFFER_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_BUFFER_ATTACHMENT_H_

#include <optional>

#include "media/base/decoder_buffer.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

// Used to serialize EncodedAudioChunk and EncodedVideoChunk.
class MODULES_EXPORT DecoderBufferAttachment
    : public SerializedScriptValue::Attachment {
 public:
  using DecoderBufferVector = Vector<scoped_refptr<media::DecoderBuffer>>;

  static const void* const kAttachmentKey;
  DecoderBufferAttachment() = default;
  ~DecoderBufferAttachment() override = default;

  bool IsLockedToAgentCluster() const override { return !buffers_.empty(); }

  size_t size() const { return buffers_.size(); }

  DecoderBufferVector& Buffers() { return buffers_; }

  const DecoderBufferVector& Buffers() const { return buffers_; }

 private:
  DecoderBufferVector buffers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_DECODER_BUFFER_ATTACHMENT_H_
