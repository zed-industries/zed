/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_SOURCE_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_SOURCE_H_

#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_time_range.h"
#include "third_party/blink/public/platform/web_url.h"

namespace media {
class AudioDecoderConfig;
class VideoDecoderConfig;
}  // namespace media

namespace blink {

class WebSourceBuffer;

class WebMediaSource {
 public:
  enum AddStatus {
    kAddStatusOk,
    kAddStatusNotSupported,
    kAddStatusReachedIdLimit
  };

  enum EndOfStreamStatus {
    kEndOfStreamStatusNoError,
    kEndOfStreamStatusNetworkError,
    kEndOfStreamStatusDecodeError,
  };

  virtual ~WebMediaSource() = default;

  // Attempts to create a new WebSourceBuffer (with ownership returned to
  // caller) for use with this WebMediaSource.
  // |content_type| is the ContentType string of the new WebSourceBuffer
  // bytestream's MIME type, and |codecs| contains the "codecs" parameter
  // string, if any, of the bytestream's MIME type.
  // |audio_config| is the decoder config that will be used to
  // decode WebCodecs EncodedAudioChunks which will be appended instead of
  // bytes.
  // |video_config| is the decoder config for decoding WebCodecs
  // EncodedVideoChunks which will be appended instead of bytes.
  // Caller's choice of which of these overloads to invoke informs the
  // underlying implementation about how it will be expected to buffer (or
  // reject) upcoming media with the resulting WebSourceBuffer.
  // For all three of the overloads of this method, if this WebMediaSource
  // cannot handle another WebSourceBuffer right now, sets |out_status| to
  // kAddStatusReachedIdLimit and returns an empty unique_ptr.
  // If this WebMediaSource supports the format indicated by |content_type| and
  // |codecs| and has enough resources to support a new WebSourceBuffer, sets
  // |out_status| to kAddStatusOk and creates and returns a new WebSourceBuffer.
  // If |content_type| and |codecs| are not supported, sets |out_status| to
  // kAddStatusNotSupported and returns an empty unique_ptr.
  virtual std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      const WebString& content_type,
      const WebString& codecs,
      AddStatus& out_status /* out */) = 0;
  // Note that the |audio_config| and |video_config| overloads assume the caller
  // has already confirmed that those configs are supported for buffering, so
  // they should not return kAddStatusNotSupported via |out_status|.
  virtual std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      std::unique_ptr<media::AudioDecoderConfig> audio_config,
      AddStatus& out_status /* out */) = 0;
  virtual std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      std::unique_ptr<media::VideoDecoderConfig> video_config,
      AddStatus& out_status /* out */) = 0;

  virtual double Duration() = 0;
  virtual void SetDuration(double) = 0;
  virtual void MarkEndOfStream(EndOfStreamStatus) = 0;
  virtual void UnmarkEndOfStream() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_SOURCE_H_
