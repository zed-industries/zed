// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AUDIO_SERVICE_AUDIO_PROCESSOR_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AUDIO_SERVICE_AUDIO_PROCESSOR_PROXY_H_

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace media {
class AudioProcessorControls;
struct AudioProcessingStats;
}  // namespace media

namespace blink {

// Wraps media::AudioProcessorControls passed into SetControls() and provides a
// thread-safe access to it via webrtc::AudioProcessorInterface.
class PLATFORM_EXPORT AudioServiceAudioProcessorProxy
    : public webrtc::AudioProcessorInterface {
 public:
  constexpr static base::TimeDelta kStatsUpdateInterval = base::Seconds(1);

  // All methods (including constructor and destructor) must be called on the
  // main thread except for GetStats() and
  // MaybeUpdateNumPreferredCaptureChannels().
  AudioServiceAudioProcessorProxy();
  AudioServiceAudioProcessorProxy(const AudioServiceAudioProcessorProxy&) =
      delete;
  AudioServiceAudioProcessorProxy& operator=(
      const AudioServiceAudioProcessorProxy&) = delete;

  // Set the AudioProcessorControls which to proxy to. Must only be called once
  // and |controls| cannot be nullptr. |controls| must outlive |this|.
  void SetControls(media::AudioProcessorControls* controls);

  // Must be called once after SetControls(|controls|); upon exit |this| won't
  // make any calls into |controls| any more.
  void Stop();

  // Thread-safe, normally called on the WebRTC signalling thread.
  AudioProcessorStatistics GetStats(bool has_remote_tracks) override;

  // Normally called on the audio capture thread (the thread may changes, the
  // caller must guarantee that it's not called concurrently from multiple
  // threads.
  void MaybeUpdateNumPreferredCaptureChannels(uint32_t num_channels);

 protected:
  ~AudioServiceAudioProcessorProxy() override;

 private:
  void RequestStats();
  void UpdateStats(const media::AudioProcessingStats& new_stats);
  void SetPreferredNumCaptureChannelsOnMainThread(uint32_t num_channels);

  const scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;

  raw_ptr<media::AudioProcessorControls> processor_controls_
      GUARDED_BY_CONTEXT(main_thread_checker_) = nullptr;

  base::Lock stats_lock_;
  AudioProcessorStatistics latest_stats_ GUARDED_BY(stats_lock_);

  base::RepeatingTimer stats_update_timer_
      GUARDED_BY_CONTEXT(main_thread_checker_);

  // Accessed only in MaybeUpdateNumPreferredCaptureChannels().
  uint32_t num_preferred_capture_channels_ = 1;

  THREAD_CHECKER(main_thread_checker_);
  base::WeakPtr<AudioServiceAudioProcessorProxy> weak_this_;
  base::WeakPtrFactory<AudioServiceAudioProcessorProxy> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_AUDIO_SERVICE_AUDIO_PROCESSOR_PROXY_H_
