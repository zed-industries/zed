// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_TRACK_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_TRACK_PLATFORM_H_

#include <optional>

#include "base/functional/callback.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "media/base/audio_glitch_info.h"
#include "media/base/audio_parameters.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_sink.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_track.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"
#include "third_party/blink/renderer/platform/audio/audio_frame_stats_accumulator.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/size.h"

namespace blink {
class WebMediaStreamAudioSink;

// MediaStreamTrackPlatform is a low-level object backing a
// WebMediaStreamTrack.
class PLATFORM_EXPORT MediaStreamTrackPlatform {
  USING_FAST_MALLOC(MediaStreamTrackPlatform);

 public:
  enum class FacingMode { kNone, kUser, kEnvironment, kLeft, kRight };

  struct Settings {
    bool HasFrameRate() const { return frame_rate >= 0.0; }
    bool HasWidth() const { return width >= 0; }
    bool HasHeight() const { return height >= 0; }
    bool HasAspectRatio() const { return aspect_ratio >= 0.0; }
    bool HasFacingMode() const { return facing_mode != FacingMode::kNone; }
    bool HasSampleRate() const { return sample_rate >= 0; }
    bool HasSampleSize() const { return sample_size >= 0; }
    bool HasChannelCount() const { return channel_count >= 0; }
    bool HasLatency() const { return latency >= 0; }
    // The variables are read from
    // MediaStreamTrack::GetSettings only.
    double frame_rate = -1.0;
    int32_t width = -1;
    int32_t height = -1;
    double aspect_ratio = -1.0;
    String device_id;
    String group_id;
    FacingMode facing_mode = FacingMode::kNone;
    String resize_mode;
    std::optional<bool> echo_cancellation;
    std::optional<bool> auto_gain_control;
    std::optional<bool> noise_supression;
    std::optional<bool> voice_isolation;
    int32_t sample_rate = -1;
    int32_t sample_size = -1;
    int32_t channel_count = -1;
    double latency = -1.0;

    // Screen Capture extensions
    std::optional<media::mojom::DisplayCaptureSurfaceType> display_surface;
    std::optional<bool> logical_surface;
    std::optional<media::mojom::CursorCaptureType> cursor;
    std::optional<bool> suppress_local_audio_playback;

    // Captured Surface Resolution API
    std::optional<gfx::Size> physical_frame_size;
    std::optional<float> device_scale_factor;
  };

  struct VideoFrameStats {
    size_t deliverable_frames = 0u;
    size_t discarded_frames = 0u;
    size_t dropped_frames = 0u;
  };

  // Corresponds to the MediaStreamTrackAudioStats API.
  // https://w3c.github.io/mediacapture-extensions/#the-mediastreamtrackaudiostats-interface
  class PLATFORM_EXPORT AudioFrameStats {
   public:
    AudioFrameStats() = default;
    AudioFrameStats(const AudioFrameStats&) = delete;
    AudioFrameStats& operator=(const AudioFrameStats&) = delete;
    ~AudioFrameStats() = default;

    // Updates the stats with information from a new buffer.
    void Update(const media::AudioParameters& params,
                base::TimeTicks capture_time,
                const media::AudioGlitchInfo& glitch_info);
    // Absorbs stats from an object that contains stats from a more recent
    // interval. This merges the AverageLatency, MinimumLatency and
    // MaximumLatency information into this object, and resets it on the |from|
    // object. |from|'s latency information interval should start where
    // |this|'s latency information interval ends. The frame counters, frame
    // durations, and current latency are simply copied from |from|.
    void Absorb(AudioFrameStats& from);

    // Implementations of the getters in the API.
    uint64_t DeliveredFrames() const;
    base::TimeDelta DeliveredFramesDuration() const;
    uint64_t TotalFrames() const;
    base::TimeDelta TotalFramesDuration() const;
    base::TimeDelta Latency() const;
    base::TimeDelta AverageLatency() const;
    base::TimeDelta MinimumLatency() const;
    base::TimeDelta MaximumLatency() const;

   private:
    AudioFrameStatsAccumulator accumulator_;
  };

  struct CaptureHandle {
    bool IsEmpty() const { return origin.empty() && handle.empty(); }

    String origin;
    String handle;
  };

  explicit MediaStreamTrackPlatform(bool is_local_track);
  MediaStreamTrackPlatform(const MediaStreamTrackPlatform&) = delete;
  MediaStreamTrackPlatform& operator=(const MediaStreamTrackPlatform&) = delete;
  virtual ~MediaStreamTrackPlatform();

  // Creates a new MediaStreamTrackPlatform of the same type as this based on
  // data retrieved from the supplied MediaStreamComponent. This method must be
  // called on a MediaStreamTrackPlatform object of the same type as the
  // platform track member of the passed MediaStreamComponent.
  //
  // TODO(crbug.com/1302689): This is an instance method of this class solely
  // for creating an object of the right type from either the platform or
  // modules directories.  Remove this method when there is a better way to
  // achieve this.
  virtual std::unique_ptr<MediaStreamTrackPlatform> CreateFromComponent(
      const MediaStreamComponent* component,
      const String& id) = 0;

  static MediaStreamTrackPlatform* GetTrack(const WebMediaStreamTrack& track);

  virtual void SetEnabled(bool enabled) = 0;

  virtual void SetContentHint(
      WebMediaStreamTrack::ContentHintType content_hint) = 0;

  // If |callback| is not null, it is invoked when the track has stopped.
  virtual void StopAndNotify(base::OnceClosure callback) = 0;

  void Stop() { StopAndNotify(base::OnceClosure()); }

  // TODO(hta): Make method pure virtual when all tracks have the method.
  virtual void GetSettings(Settings& settings) const {}

  virtual VideoFrameStats GetVideoFrameStats() const {
    // This method is only callable on video tracks.
    NOTREACHED();
  }

  // Gets the audio frame stats if the platform is an audio track. This also
  // resets the latency information stored in the track, so that subsequent
  // calls to TransferAudioFrameStatsTo() will return latency information for
  // consecutive but non-overlaping intervals.
  virtual void TransferAudioFrameStatsTo(AudioFrameStats& destination) {
    // This method is only callable on audio tracks.
    NOTREACHED();
  }

  virtual CaptureHandle GetCaptureHandle();

  // Adds a one off callback that will be invoked when observing the first frame
  // where |metadata.sub_capture_target_version >= sub_capture_target_version|.
  virtual void AddSubCaptureTargetVersionCallback(
      uint32_t sub_capture_target_version,
      base::OnceClosure callback) {}

  // Removes the callback that was associated with this
  // |sub_capture_target_version|, if any.
  virtual void RemoveSubCaptureTargetVersionCallback(
      uint32_t sub_capture_target_version) {}

  bool is_local_track() const { return is_local_track_; }

  enum class StreamType { kAudio, kVideo };
  virtual StreamType Type() const = 0;

  // Add an audio sink to the track. This function must only be called for audio
  // tracks. It will trigger a OnSetFormat() call on the |sink| before the first
  // chunk of audio is delivered.
  virtual void AddSink(WebMediaStreamAudioSink* sink) { NOTREACHED(); }
  // Add a video sink to track. This function must only be called for video
  // tracks.  The |sink| will receive video track state changes on the main
  // render thread and video frames in the |callback| method on the IO-thread.
  // |callback| will be reset on the render thread.
  virtual void AddSink(WebMediaStreamSink* sink,
                       const VideoCaptureDeliverFrameCB& callback,
                       MediaStreamVideoSink::IsSecure is_secure,
                       MediaStreamVideoSink::UsesAlpha uses_alpha) {
    NOTREACHED();
  }

 private:
  const bool is_local_track_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_TRACK_PLATFORM_H_
