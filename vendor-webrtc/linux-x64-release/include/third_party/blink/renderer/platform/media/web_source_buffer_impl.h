// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_SOURCE_BUFFER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_SOURCE_BUFFER_IMPL_H_

#include <stddef.h>

#include <memory>
#include <string>

#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "media/base/stream_parser.h"
#include "third_party/blink/public/platform/web_source_buffer.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace media {
class ChunkDemuxer;
class MediaTracks;
enum class SourceBufferParseWarning;
}  // namespace media

namespace blink {

class PLATFORM_EXPORT WebSourceBufferImpl : public WebSourceBuffer {
 public:
  WebSourceBufferImpl(const std::string& id, media::ChunkDemuxer* demuxer);
  WebSourceBufferImpl(const WebSourceBufferImpl&) = delete;
  WebSourceBufferImpl& operator=(const WebSourceBufferImpl&) = delete;
  ~WebSourceBufferImpl() override;

  // WebSourceBuffer implementation.
  void SetClient(WebSourceBufferClient* client) override;
  bool GetGenerateTimestampsFlag() override;
  bool SetMode(AppendMode mode) override;
  WebTimeRanges Buffered() override;
  double HighestPresentationTimestamp() override;
  bool EvictCodedFrames(double currentPlaybackTime,
                        size_t newDataSize) override;
  [[nodiscard]] bool AppendToParseBuffer(
      base::span<const unsigned char> data) override;
  [[nodiscard]] media::StreamParser::ParseStatus RunSegmentParserLoop(
      double* timestamp_offset) override;
  bool AppendChunks(
      std::unique_ptr<media::StreamParser::BufferQueue> buffer_queue,
      double* timestamp_offset) override;
  void ResetParserState() override;
  void Remove(double start, double end) override;
  bool CanChangeType(const WebString& content_type,
                     const WebString& codecs) override;
  void ChangeType(const WebString& content_type,
                  const WebString& codecs) override;
  bool SetTimestampOffset(double offset) override;
  void SetAppendWindowStart(double start) override;
  void SetAppendWindowEnd(double end) override;
  void RemovedFromMediaSource() override;

 private:
  // Demuxer callback handler to process an initialization segment received
  // during an append() call.
  void InitSegmentReceived(std::unique_ptr<media::MediaTracks> tracks);

  // Demuxer callback handler to notify Blink of a non-fatal parse warning.
  void NotifyParseWarning(const media::SourceBufferParseWarning warning);

  std::string id_;
  // Owned by WebMediaPlayerImpl.
  raw_ptr<media::ChunkDemuxer, DanglingUntriaged> demuxer_;

  raw_ptr<WebSourceBufferClient> client_;

  // Controls the offset applied to timestamps when processing appended media
  // segments. It is initially 0, which indicates that no offset is being
  // applied. Both setTimestampOffset() and append() may update this value.
  base::TimeDelta timestamp_offset_;

  base::TimeDelta append_window_start_;
  base::TimeDelta append_window_end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_SOURCE_BUFFER_IMPL_H_
