// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_LEVEL_CALCULATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_LEVEL_CALCULATOR_H_

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace media {
class AudioBus;
}

namespace blink {

// This class is used by the WebRtcAudioCapturer to calculate the level of the
// audio signal. And the audio level will be eventually used by the volume
// animation UI.
//
// The algorithm used by this class is the same as how it is done in
// third_party/webrtc/voice_engine/level_indicator.cc.
class PLATFORM_EXPORT MediaStreamAudioLevelCalculator {
 public:
  // Provides thread-safe access to the current signal level.  This object is
  // intended to be passed to modules running on other threads that poll for the
  // current signal level.
  class PLATFORM_EXPORT Level : public ThreadSafeRefCounted<Level> {
   public:
    explicit Level(base::PassKey<MediaStreamAudioLevelCalculator>);
    float GetCurrent() const;

   private:
    friend class MediaStreamAudioLevelCalculator;
    friend class ThreadSafeRefCounted<Level>;
    ~Level();

    void Set(float level);

    mutable base::Lock lock_;
    float level_ = 0.0f;
  };

  MediaStreamAudioLevelCalculator();
  ~MediaStreamAudioLevelCalculator();

  const scoped_refptr<Level>& level() const { return level_; }

  // Scans the audio signal in |audio_bus| and computes a new signal level
  // exposed by Level.  If |assume_nonzero_energy| is true, then a completely
  // zero'ed-out |audio_bus| will be accounted for as having a very faint,
  // non-zero level.
  void Calculate(const media::AudioBus& audio_bus, bool assume_nonzero_energy);

 private:
  int counter_ = 0;
  float max_amplitude_ = 0.0f;
  const scoped_refptr<Level> level_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_LEVEL_CALCULATOR_H_
