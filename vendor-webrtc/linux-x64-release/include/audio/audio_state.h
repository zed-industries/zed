/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_AUDIO_STATE_H_
#define AUDIO_AUDIO_STATE_H_

#include <map>
#include <memory>

#include "api/sequence_checker.h"
#include "audio/audio_transport_impl.h"
#include "call/audio_state.h"
#include "rtc_base/containers/flat_set.h"
#include "rtc_base/ref_count.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class AudioSendStream;
class AudioReceiveStreamInterface;

namespace internal {

class AudioState : public webrtc::AudioState {
 public:
  explicit AudioState(const AudioState::Config& config);

  AudioState() = delete;
  AudioState(const AudioState&) = delete;
  AudioState& operator=(const AudioState&) = delete;

  ~AudioState() override;

  AudioProcessing* audio_processing() override;
  AudioTransport* audio_transport() override;

  void SetPlayout(bool enabled) override;
  void SetRecording(bool enabled) override;

  void SetStereoChannelSwapping(bool enable) override;

  AudioDeviceModule* audio_device_module() {
    RTC_DCHECK(config_.audio_device_module);
    return config_.audio_device_module.get();
  }

  void AddReceivingStream(webrtc::AudioReceiveStreamInterface* stream);
  void RemoveReceivingStream(webrtc::AudioReceiveStreamInterface* stream);

  void AddSendingStream(webrtc::AudioSendStream* stream,
                        int sample_rate_hz,
                        size_t num_channels);
  void RemoveSendingStream(webrtc::AudioSendStream* stream);

 private:
  void UpdateAudioTransportWithSendingStreams();
  void UpdateNullAudioPollerState() RTC_RUN_ON(&thread_checker_);

  SequenceChecker thread_checker_;
  SequenceChecker process_thread_checker_{SequenceChecker::kDetached};
  const webrtc::AudioState::Config config_;
  bool recording_enabled_ = true;
  bool playout_enabled_ = true;

  // Transports mixed audio from the mixer to the audio device and
  // recorded audio to the sending streams.
  AudioTransportImpl audio_transport_;

  // Null audio poller is used to continue polling the audio streams if audio
  // playout is disabled so that audio processing still happens and the audio
  // stats are still updated.
  RepeatingTaskHandle null_audio_poller_ RTC_GUARDED_BY(&thread_checker_);

  webrtc::flat_set<webrtc::AudioReceiveStreamInterface*> receiving_streams_;
  struct StreamProperties {
    int sample_rate_hz = 0;
    size_t num_channels = 0;
  };
  std::map<webrtc::AudioSendStream*, StreamProperties> sending_streams_;
};
}  // namespace internal
}  // namespace webrtc

#endif  // AUDIO_AUDIO_STATE_H_
