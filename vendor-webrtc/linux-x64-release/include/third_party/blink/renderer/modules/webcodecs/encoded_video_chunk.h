// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODED_VIDEO_CHUNK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODED_VIDEO_CHUNK_H_

#include <optional>

#include "media/base/decoder_buffer.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/array_buffer_util.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {
class EncodedVideoChunkInit;
class ExceptionState;
class V8EncodedVideoChunkType;

class MODULES_EXPORT EncodedVideoChunk final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit EncodedVideoChunk(scoped_refptr<media::DecoderBuffer> buffer);

  static EncodedVideoChunk* Create(ScriptState* script_state,
                                   const EncodedVideoChunkInit* init,
                                   ExceptionState& exception_state);

  // encoded_video_chunk.idl implementation.
  V8EncodedVideoChunkType type() const;
  int64_t timestamp() const;
  std::optional<uint64_t> duration() const;
  uint64_t byteLength() const;
  void copyTo(const AllowSharedBufferSource* destination,
              ExceptionState& exception_state);

  scoped_refptr<media::DecoderBuffer> buffer() const { return buffer_; }

 private:
  scoped_refptr<media::DecoderBuffer> buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODED_VIDEO_CHUNK_H_
