/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef CALL_AUDIO_STATE_H_
#define CALL_AUDIO_STATE_H_

#include "api/audio/audio_device.h"
#include "api/audio/audio_mixer.h"
#include "api/audio/audio_processing.h"
#include "api/ref_count.h"
#include "api/scoped_refptr.h"
#include "modules/async_audio_processing/async_audio_processing.h"

namespace webrtc {

class AudioTransport;

// AudioState holds the state which must be shared between multiple instances of
// webrtc::Call for audio processing purposes.
class AudioState : public RefCountInterface {
 public:
  struct Config {
    Config();
    ~Config();

    // The audio mixer connected to active receive streams. One per
    // AudioState.
    scoped_refptr<AudioMixer> audio_mixer;

    // The audio processing module.
    scoped_refptr<webrtc::AudioProcessing> audio_processing;

    // TODO(solenberg): Temporary: audio device module.
    scoped_refptr<webrtc::AudioDeviceModule> audio_device_module;

    scoped_refptr<AsyncAudioProcessing::Factory> async_audio_processing_factory;
  };

  virtual AudioProcessing* audio_processing() = 0;
  virtual AudioTransport* audio_transport() = 0;

  // Enable/disable playout of the audio channels. Enabled by default.
  // This will stop playout of the underlying audio device but start a task
  // which will poll for audio data every 10ms to ensure that audio processing
  // happens and the audio stats are updated.
  virtual void SetPlayout(bool enabled) = 0;

  // Enable/disable recording of the audio channels. Enabled by default.
  // This will stop recording of the underlying audio device and no audio
  // packets will be encoded or transmitted.
  virtual void SetRecording(bool enabled) = 0;

  virtual void SetStereoChannelSwapping(bool enable) = 0;

  static scoped_refptr<AudioState> Create(const AudioState::Config& config);

  ~AudioState() override {}
};
}  // namespace webrtc

#endif  // CALL_AUDIO_STATE_H_
