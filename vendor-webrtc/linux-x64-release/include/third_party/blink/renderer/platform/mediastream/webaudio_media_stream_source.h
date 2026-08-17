// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBAUDIO_MEDIA_STREAM_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBAUDIO_MEDIA_STREAM_SOURCE_H_

#include <memory>
#include <vector>

#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "media/base/audio_bus.h"
#include "media/base/audio_push_fifo.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/mediastream/webaudio_destination_consumer.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Implements the WebAudioDestinationConsumer interface to provide a source of
// audio data (i.e., the output from a graph of WebAudio nodes) to one or more
// MediaStreamAudioTracks. Audio data is transported directly to the tracks in
// 10 ms chunks.
class PLATFORM_EXPORT WebAudioMediaStreamSource final
    : public MediaStreamAudioSource,
      public WebAudioDestinationConsumer {
 public:
  WebAudioMediaStreamSource(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  WebAudioMediaStreamSource(const WebAudioMediaStreamSource&) = delete;
  WebAudioMediaStreamSource& operator=(const WebAudioMediaStreamSource&) =
      delete;

  ~WebAudioMediaStreamSource() override;

 private:
  // WebAudioDestinationConsumer implementation.
  //
  // Note: Blink ensures setFormat() and consumeAudio() are not called
  // concurrently across threads, but these methods could be called on any
  // thread.
  void SetFormat(int number_of_channels, float sample_rate) override;
  void ConsumeAudio(const Vector<const float*>& audio_data,
                    int number_of_frames) override;

  // Called by AudioPushFifo zero or more times during the call to
  // consumeAudio().  Delivers audio data with the required buffer size to the
  // tracks.
  void DeliverRebufferedAudio(const media::AudioBus& audio_bus,
                              int frame_delay);

  // In debug builds, check that all methods that could cause object graph
  // or data flow changes are being called on the main thread.
  THREAD_CHECKER(thread_checker_);

  // A wrapper used for providing audio to |fifo_|.
  std::unique_ptr<media::AudioBus> wrapper_bus_;

  // Takes in the audio data passed to consumeAudio() and re-buffers it into 10
  // ms chunks for the tracks. This ensures each chunk of audio delivered to
  // the tracks has the required buffer size, regardless of the amount of audio
  // provided via each consumeAudio() call.
  media::AudioPushFifo fifo_;

  // Used to pass the reference timestamp between DeliverDecodedAudio() and
  // DeliverRebufferedAudio().
  base::TimeTicks current_reference_time_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_WEBAUDIO_MEDIA_STREAM_SOURCE_H_
