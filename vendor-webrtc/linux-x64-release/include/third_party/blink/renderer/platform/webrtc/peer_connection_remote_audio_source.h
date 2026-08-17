// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_PEER_CONNECTION_REMOTE_AUDIO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_PEER_CONNECTION_REMOTE_AUDIO_SOURCE_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_track.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace media {
class AudioBus;
}

namespace blink {

PLATFORM_EXPORT BASE_DECLARE_FEATURE(kPropagateEnabledEventForWebRtcAudioTrack);

// PeerConnectionRemoteAudioTrack is a WebRTC specific implementation of an
// audio track whose data is sourced from a PeerConnection.
class PLATFORM_EXPORT PeerConnectionRemoteAudioTrack final
    : public MediaStreamAudioTrack {
 public:
  explicit PeerConnectionRemoteAudioTrack(
      scoped_refptr<webrtc::AudioTrackInterface> track_interface);
  PeerConnectionRemoteAudioTrack(const PeerConnectionRemoteAudioTrack&) =
      delete;
  PeerConnectionRemoteAudioTrack& operator=(
      const PeerConnectionRemoteAudioTrack&) = delete;
  ~PeerConnectionRemoteAudioTrack() final;

  // If |track| is an instance of PeerConnectionRemoteAudioTrack, return a
  // type-casted pointer to it. Otherwise, return null.
  static PeerConnectionRemoteAudioTrack* From(MediaStreamAudioTrack* track);

  webrtc::AudioTrackInterface* track_interface() const {
    return track_interface_.get();
  }

  // MediaStreamAudioTrack override.
  void SetEnabled(bool enabled) override;

 private:
  // MediaStreamAudioTrack overrides.
  void* GetClassIdentifier() const final;

  const scoped_refptr<webrtc::AudioTrackInterface> track_interface_;

  // In debug builds, check that all methods that could cause object graph
  // or data flow changes are being called on the main thread.
  THREAD_CHECKER(thread_checker_);
};

// Represents the audio provided by the receiving end of a PeerConnection.
class PLATFORM_EXPORT PeerConnectionRemoteAudioSource final
    : public MediaStreamAudioSource,
      protected webrtc::AudioTrackSinkInterface {
 public:
  PeerConnectionRemoteAudioSource(
      scoped_refptr<webrtc::AudioTrackInterface> track_interface,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  PeerConnectionRemoteAudioSource(const PeerConnectionRemoteAudioSource&) =
      delete;
  PeerConnectionRemoteAudioSource& operator=(
      const PeerConnectionRemoteAudioSource&) = delete;
  ~PeerConnectionRemoteAudioSource() final;

 protected:
  // MediaStreamAudioSource implementation.
  std::unique_ptr<MediaStreamAudioTrack> CreateMediaStreamAudioTrack(
      const std::string& id) final;
  bool EnsureSourceIsStarted() final;
  void EnsureSourceIsStopped() final;

  // webrtc::AudioTrackSinkInterface implementation.
  void OnData(const void* audio_data,
              int bits_per_sample,
              int sample_rate,
              size_t number_of_channels,
              size_t number_of_frames) final;

 private:
  // Interface to the implementation that calls OnData().
  const scoped_refptr<webrtc::AudioTrackInterface> track_interface_;

  // In debug builds, check that all methods that could cause object graph
  // or data flow changes are being called on the main thread.
  THREAD_CHECKER(thread_checker_);

  // True if |this| is receiving an audio flow as a sink of the remote
  // PeerConnection via |track_interface_|.
  bool is_sink_of_peer_connection_;

  // Buffer for converting from interleaved signed-integer PCM samples to the
  // planar float format. Only used on the thread that calls OnData().
  std::unique_ptr<media::AudioBus> audio_bus_;

  // In debug builds, use a "try lock" to sanity-check that there are no
  // concurrent calls to OnData(). See notes in OnData() implementation.
#ifndef NDEBUG
  base::Lock single_audio_thread_guard_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_PEER_CONNECTION_REMOTE_AUDIO_SOURCE_H_
