/*
 *  Copyright 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_REMOTE_AUDIO_SOURCE_H_
#define PC_REMOTE_AUDIO_SOURCE_H_

#include <stdint.h>

#include <list>
#include <optional>
#include <string>

#include "api/call/audio_sink.h"
#include "api/media_stream_interface.h"
#include "api/notifier.h"
#include "api/task_queue/task_queue_base.h"
#include "media/base/media_channel.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// This class implements the audio source used by the remote audio track.
// This class works by configuring itself as a sink with the underlying media
// engine, then when receiving data will fan out to all added sinks.
class RemoteAudioSource : public Notifier<AudioSourceInterface> {
 public:
  // In Unified Plan, receivers map to m= sections and their tracks and sources
  // survive SSRCs being reconfigured. The life cycle of the remote audio source
  // is associated with the life cycle of the m= section, and thus even if an
  // audio channel is destroyed the RemoteAudioSource should kSurvive.
  //
  // In Plan B however, remote audio sources map 1:1 with an SSRCs and if an
  // audio channel is destroyed, the RemoteAudioSource should kEnd.
  enum class OnAudioChannelGoneAction {
    kSurvive,
    kEnd,
  };

  explicit RemoteAudioSource(
      TaskQueueBase* worker_thread,
      OnAudioChannelGoneAction on_audio_channel_gone_action);

  // Register and unregister remote audio source with the underlying media
  // engine.
  void Start(VoiceMediaReceiveChannelInterface* media_channel,
             std::optional<uint32_t> ssrc);
  void Stop(VoiceMediaReceiveChannelInterface* media_channel,
            std::optional<uint32_t> ssrc);
  void SetState(SourceState new_state);

  // MediaSourceInterface implementation.
  MediaSourceInterface::SourceState state() const override;
  bool remote() const override;

  // AudioSourceInterface implementation.
  void SetVolume(double volume) override;
  void RegisterAudioObserver(AudioObserver* observer) override;
  void UnregisterAudioObserver(AudioObserver* observer) override;

  void AddSink(AudioTrackSinkInterface* sink) override;
  void RemoveSink(AudioTrackSinkInterface* sink) override;

 protected:
  ~RemoteAudioSource() override;

 private:
  // These are callbacks from the media engine.
  class AudioDataProxy;

  void OnData(const AudioSinkInterface::Data& audio);
  void OnAudioChannelGone();

  TaskQueueBase* const main_thread_;
  TaskQueueBase* const worker_thread_;
  const OnAudioChannelGoneAction on_audio_channel_gone_action_;
  std::list<AudioObserver*> audio_observers_;
  Mutex sink_lock_;
  std::list<AudioTrackSinkInterface*> sinks_;
  SourceState state_;
};

}  // namespace webrtc

#endif  // PC_REMOTE_AUDIO_SOURCE_H_
