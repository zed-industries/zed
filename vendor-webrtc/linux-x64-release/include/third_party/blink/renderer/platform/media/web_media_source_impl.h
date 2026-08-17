// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_SOURCE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_SOURCE_IMPL_H_

#include <vector>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/platform/web_media_source.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace media {
class AudioDecoderConfig;
class ChunkDemuxer;
class VideoDecoderConfig;
}  // namespace media

namespace blink {

class PLATFORM_EXPORT WebMediaSourceImpl : public WebMediaSource {
 public:
  WebMediaSourceImpl(media::ChunkDemuxer* demuxer);
  WebMediaSourceImpl(const WebMediaSourceImpl&) = delete;
  WebMediaSourceImpl& operator=(const WebMediaSourceImpl&) = delete;
  ~WebMediaSourceImpl() override;

  // WebMediaSource implementation.
  std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      const WebString& content_type,
      const WebString& codecs,
      AddStatus& out_status /* out */) override;
  std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      std::unique_ptr<media::AudioDecoderConfig> audio_config,
      AddStatus& out_status /* out */) override;
  std::unique_ptr<WebSourceBuffer> AddSourceBuffer(
      std::unique_ptr<media::VideoDecoderConfig> video_config,
      AddStatus& out_status /* out */) override;
  double Duration() override;
  void SetDuration(double duration) override;
  void MarkEndOfStream(EndOfStreamStatus status) override;
  void UnmarkEndOfStream() override;

 private:
  raw_ptr<media::ChunkDemuxer, DanglingUntriaged>
      demuxer_;  // Owned by WebMediaPlayerImpl.
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_SOURCE_IMPL_H_
