/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_AUDIO_MIXER_H_
#define API_AUDIO_AUDIO_MIXER_H_

#include <cstddef>

#include "api/audio/audio_frame.h"
#include "api/ref_count.h"

namespace webrtc {

// WORK IN PROGRESS
// This class is under development and is not yet intended for for use outside
// of WebRtc/Libjingle.
class AudioMixer : public RefCountInterface {
 public:
  // A callback class that all mixer participants must inherit from/implement.
  class Source {
   public:
    enum class AudioFrameInfo {
      kNormal,  // The samples in audio_frame are valid and should be used.
      kMuted,   // The samples in audio_frame should not be used, but
                // should be implicitly interpreted as zero. Other
                // fields in audio_frame may be read and should
                // contain meaningful values.
      kError,   // The audio_frame will not be used.
    };

    // Overwrites `audio_frame`. The data_ field is overwritten with
    // 10 ms of new audio (either 1 or 2 interleaved channels) at
    // `sample_rate_hz`. All fields in `audio_frame` must be updated.
    virtual AudioFrameInfo GetAudioFrameWithInfo(int sample_rate_hz,
                                                 AudioFrame* audio_frame) = 0;

    // A way for a mixer implementation to distinguish participants.
    virtual int Ssrc() const = 0;

    // A way for this source to say that GetAudioFrameWithInfo called
    // with this sample rate or higher will not cause quality loss.
    virtual int PreferredSampleRate() const = 0;

    virtual ~Source() {}
  };

  // Returns true if adding was successful. A source is never added
  // twice. Addition and removal can happen on different threads.
  virtual bool AddSource(Source* audio_source) = 0;

  // Removal is never attempted if a source has not been successfully
  // added to the mixer.
  virtual void RemoveSource(Source* audio_source) = 0;

  // Performs mixing by asking registered audio sources for audio. The
  // mixed result is placed in the provided AudioFrame. This method
  // will only be called from a single thread. The channels argument
  // specifies the number of channels of the mix result. The mixer
  // should mix at a rate that doesn't cause quality loss of the
  // sources' audio. The mixing rate is one of the rates listed in
  // AudioProcessing::NativeRate. All fields in
  // `audio_frame_for_mixing` must be updated.
  virtual void Mix(size_t number_of_channels,
                   AudioFrame* audio_frame_for_mixing) = 0;

 protected:
  // Since the mixer is reference counted, the destructor may be
  // called from any thread.
  ~AudioMixer() override {}
};
}  // namespace webrtc

#endif  // API_AUDIO_AUDIO_MIXER_H_
