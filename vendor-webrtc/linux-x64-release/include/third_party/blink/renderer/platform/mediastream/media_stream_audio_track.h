// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_TRACK_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_deliverer.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_track_platform.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace media {
struct AudioGlitchInfo;
}

namespace blink {

class WebMediaStreamAudioSink;
class MediaStreamAudioSource;
class MediaStreamComponent;

// Provides the part of the audio pipeline delivering audio from a
// MediaStreamAudioSource to one or more WebMediaStreamAudioSinks. An instance
// of this class is owned by MediaStreamComponent/WebMediaStreamTrack, and
// clients should use From() to gain access to a MediaStreamAudioTrack.
class PLATFORM_EXPORT MediaStreamAudioTrack : public MediaStreamTrackPlatform {
 public:
  explicit MediaStreamAudioTrack(bool is_local_track);
  MediaStreamAudioTrack(const MediaStreamAudioTrack&) = delete;
  MediaStreamAudioTrack& operator=(const MediaStreamAudioTrack&) = delete;

  ~MediaStreamAudioTrack() override;

  std::unique_ptr<MediaStreamTrackPlatform> CreateFromComponent(
      const MediaStreamComponent*,
      const String& id) override;

  // Returns the MediaStreamAudioTrack instance owned by the given blink |track|
  // or null.
  static MediaStreamAudioTrack* From(const MediaStreamComponent* component);

  // Provides a weak reference to this MediaStreamAudioTrack which is
  // invalidated when Stop() is called. The weak pointer may only be
  // dereferenced on the main thread.
  base::WeakPtr<MediaStreamAudioTrack> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

  // Add a sink to the track. This function will trigger a OnSetFormat()
  // call on the |sink| before the first chunk of audio is delivered.
  void AddSink(WebMediaStreamAudioSink* sink) override;

  // Remove a sink from the track. When this method returns, the sink's
  // OnSetFormat() and OnData() methods will not be called again on any thread.
  void RemoveSink(WebMediaStreamAudioSink* sink);

  // Returns the output format of the capture source. May return an invalid
  // AudioParameters if the format is not yet available.
  // Called on the main render thread.
  // TODO(tommi): This method appears to only be used by Pepper and in fact
  // does not appear to be necessary there.  We should remove it since it adds
  // to the complexity of all types of audio tracks+source implementations.
  // https://crbug.com/577874
  media::AudioParameters GetOutputFormat() const;

  // Halts the flow of audio data from the source (and to the sinks), and then
  // notifies all sinks of the "ended" state.
  void StopAndNotify(base::OnceClosure callback) final;

  // MediaStreamTrack override.
  void SetEnabled(bool enabled) override;
  void SetContentHint(
      WebMediaStreamTrack::ContentHintType content_hint) override;
  void TransferAudioFrameStatsTo(
      MediaStreamTrackPlatform::AudioFrameStats& destination) override;

  // Returns the maximum number of channels preferred by any sink connected to
  // this track.
  int NumPreferredChannels() const;

  bool IsEnabled() const;

  // Returns a unique class identifier. Some subclasses override and use this
  // method to provide safe down-casting to their type.
  virtual void* GetClassIdentifier() const;

 private:
  friend class MediaStreamAudioSource;
  friend class MediaStreamAudioDeliverer<MediaStreamAudioTrack>;

  // Called by MediaStreamAudioSource to notify this track that the flow of
  // audio data has started from the source. |stop_callback| is run by Stop()
  // when the source must halt the flow of audio data to this track.
  void Start(base::OnceClosure stop_callback);

  // Called by the MediaStreamAudioDeliverer to notify this track of an audio
  // format change. In turn, all WebMediaStreamAudioSinks will be notified
  // before the next chunk of audio is delivered to them.
  void OnSetFormat(const media::AudioParameters& params);

  // Called by the MediaStreamAudioDeliverer to deliver audio data to this
  // track, which in turn delivers the audio to one or more
  // WebMediaStreamAudioSinks. While this track is disabled, silent audio will
  // be delivered to the sinks instead of the content of |audio_bus|.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks reference_time,
              const media::AudioGlitchInfo& glitch_info);

  MediaStreamTrackPlatform::StreamType Type() const override {
    return MediaStreamTrackPlatform::StreamType::kAudio;
  }

  void UpdateFrameStats(const media::AudioBus& audio_bus,
                        base::TimeTicks reference_time,
                        const media::AudioGlitchInfo& glitch_info);

 private:
  // In debug builds, check that all methods that could cause object graph
  // or data flow changes are being called on the main thread.
  THREAD_CHECKER(thread_checker_);

  // Callback provided to Start() which is run when the audio flow must halt.
  base::OnceClosure stop_callback_;

  // Manages sinks connected to this track and the audio format and data flow.
  MediaStreamAudioDeliverer<WebMediaStreamAudioSink> deliverer_;

  // While false, silent audio is delivered to the sinks.
  std::atomic<bool> is_enabled_;

  // Buffer used to deliver silent audio data while this track is disabled.
  std::unique_ptr<media::AudioBus> silent_bus_;

  // Set to true once at first audio callback after calling Start().
  // Only used for logging purposes.
  bool received_audio_callback_ = false;

  base::Lock mainthread_frame_stats_lock_;

  // The latest frame stats that can be observed on the main thread.
  AudioFrameStats mainthread_frame_stats_
      GUARDED_BY(mainthread_frame_stats_lock_);

  // Frame stats that have not yet been added into mainthread_frame_stats_. This
  // is only used on the audio thread.
  AudioFrameStats pending_frame_stats_;

  // Provides weak pointers that are valid until Stop() is called.
  base::WeakPtrFactory<MediaStreamAudioTrack> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_TRACK_H_
