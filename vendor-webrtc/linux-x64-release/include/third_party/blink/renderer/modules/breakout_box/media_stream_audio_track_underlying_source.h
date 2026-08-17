// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SOURCE_H_

#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "media/base/audio_parameters.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"
#include "third_party/blink/renderer/modules/breakout_box/frame_queue_underlying_source.h"
#include "third_party/blink/renderer/modules/breakout_box/transferred_frame_queue_underlying_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"

namespace blink {

class MediaStreamComponent;
class ReadableStreamTransferringOptimizer;

class MODULES_EXPORT MediaStreamAudioTrackUnderlyingSource
    : public AudioDataQueueUnderlyingSource,
      public WebMediaStreamAudioSink {
  USING_PRE_FINALIZER(MediaStreamAudioTrackUnderlyingSource,
                      DisconnectFromTrack);

 public:
  // Public interface for unit testing purposes.
  class AudioBufferPool {
   public:
    AudioBufferPool() = default;
    virtual ~AudioBufferPool() = default;

    virtual void SetFormat(const media::AudioParameters params) = 0;
    virtual scoped_refptr<media::AudioBuffer> CopyIntoAudioBuffer(
        const media::AudioBus& audio_bus,
        base::TimeTicks capture_time) = 0;

    virtual int GetSizeForTesting() = 0;
  };

  explicit MediaStreamAudioTrackUnderlyingSource(
      ScriptState*,
      MediaStreamComponent*,
      ScriptWrappable* media_stream_track_processor,
      wtf_size_t queue_size);
  MediaStreamAudioTrackUnderlyingSource(
      const MediaStreamAudioTrackUnderlyingSource&) = delete;
  MediaStreamAudioTrackUnderlyingSource& operator=(
      const MediaStreamAudioTrackUnderlyingSource&) = delete;

  // WebMediaStreamAudioSink
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks estimated_capture_time) override;
  void OnSetFormat(const media::AudioParameters& params) override;

  MediaStreamComponent* Track() const { return track_.Get(); }

  std::unique_ptr<ReadableStreamTransferringOptimizer>
  GetTransferringOptimizer();

  void ContextDestroyed() override;
  void Trace(Visitor*) const override;

  AudioBufferPool* GetAudioBufferPoolForTesting();

 private:
  // FrameQueueUnderlyingSource implementation.
  bool StartFrameDelivery() override;
  void StopFrameDelivery() override;

  void DisconnectFromTrack();
  void OnSourceTransferStarted(
      scoped_refptr<base::SequencedTaskRunner> transferred_runner,
      CrossThreadPersistent<TransferredAudioDataQueueUnderlyingSource> source);

  // Only used to prevent the gargabe collector from reclaiming the media
  // stream track processor that created |this|.
  const Member<ScriptWrappable> media_stream_track_processor_;

  Member<MediaStreamComponent> track_;

  std::unique_ptr<AudioBufferPool> buffer_pool_;

  // This prevents collection of this object while it is still connected to a
  // platform MediaStreamTrack.
  SelfKeepAlive<MediaStreamAudioTrackUnderlyingSource> is_connected_to_track_;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_MEDIA_STREAM_AUDIO_TRACK_UNDERLYING_SOURCE_H_
