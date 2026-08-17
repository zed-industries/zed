// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_SOURCE_H_

#include <limits>
#include <memory>
#include <string>

#include "base/memory/weak_ptr.h"
#include "media/base/audio_capturer_source.h"
#include "media/base/limits.h"
#include "third_party/blink/public/platform/modules/mediastream/web_platform_media_stream_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_deliverer.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_processor_options.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace media {
struct AudioGlitchInfo;
}

namespace blink {

PLATFORM_EXPORT extern const int kFallbackAudioLatencyMs;

class MediaStreamAudioTrack;
class MediaStreamComponent;

// Represents a source of audio, and manages the delivery of audio data between
// the source implementation and one or more MediaStreamAudioTracks. This is a
// base class providing all the necessary functionality to connect tracks and
// have audio data delivered to them. Subclasses provide the actual audio source
// implementation (e.g., media::AudioCapturerSource), and should implement the
// EnsureSourceIsStarted() and EnsureSourceIsStopped() methods, and call
// SetFormat() and DeliverDataToTracks().
//
// This base class can be instantiated, to be used as a place-holder or a "null"
// source of audio. This can be useful for unit testing, wherever a mock is
// needed, and/or calls to DeliverDataToTracks() must be made at very specific
// times.
//
// An instance of this class is owned by MediaStreamSource.
//
// Usage example:
//
//   class MyAudioSource : public MediaStreamAudioSource { ... };
//
//   MediaStreamSource* media_stream_source =
//       MakeGarbageCollected<MediaStreamSource>(
//           ..., std::make_unique<MyAudioSource>());
//   MediaStreamComponent* media_stream_track = ...;
//   if (MediaStreamAudioSource::From(media_stream_source)
//           ->ConnectToInitializedTrack(media_stream_track)) {
//     LOG(INFO) << "Success!";
//   } else {
//     LOG(ERROR) << "Failed!";
//   }
class PLATFORM_EXPORT MediaStreamAudioSource
    : public WebPlatformMediaStreamSource {
 public:
  MediaStreamAudioSource(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      bool is_local_source);
  MediaStreamAudioSource(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      bool is_local_source,
      bool disable_local_echo);
  MediaStreamAudioSource(const MediaStreamAudioSource&) = delete;
  MediaStreamAudioSource& operator=(const MediaStreamAudioSource&) = delete;
  ~MediaStreamAudioSource() override;

  // Returns the MediaStreamAudioSource instance owned by the given blink
  // |source| or null.
  static MediaStreamAudioSource* From(MediaStreamSource* source);

  // Provides a weak reference to this MediaStreamAudioSource. The weak pointer
  // may only be dereferenced on the main thread.
  base::WeakPtr<MediaStreamAudioSource> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

  // Returns true if the source of audio is local to the application (e.g.,
  // microphone input or loopback audio capture) as opposed to audio being
  // streamed-in from outside the application.
  bool is_local_source() const { return is_local_source_; }

  // Connects this source to the given |component|, which already has an
  // associated MediaStreamAudioTrack. Returns true if the source was
  // successfully started.
  bool ConnectToInitializedTrack(MediaStreamComponent* component);

  // Returns the current format of the audio passing through this source to the
  // sinks. This can return invalid parameters if the source has not yet been
  // started. This method is thread-safe.
  media::AudioParameters GetAudioParameters() const;

  // These accessors return properties that are controlled via constraints.
  bool disable_local_echo() const { return disable_local_echo_; }
  bool RenderToAssociatedSinkEnabled() const;

  // Returns a unique class identifier. Some subclasses override and use this
  // method to provide safe down-casting to their type.
  virtual void* GetClassIdentifier() const;

  // Returns true if the source has audio processing properties and the
  // reconfigurable settings associated to audio processing match
  // |selected_properties|; false otherwise.
  bool HasSameReconfigurableSettings(
      const blink::AudioProcessingProperties& selected_properties) const;

  // Returns true if |this| and |other_source| have audio processing properties
  // and the set of settings that cannot be reconfigured associated to these
  // audio sources match; false otherwise.
  bool HasSameNonReconfigurableSettings(
      MediaStreamAudioSource* other_source) const;

  // Returns the audio processing properties associated to this source if any,
  // or nullopt otherwise.
  virtual std::optional<blink::AudioProcessingProperties>
  GetAudioProcessingProperties() const {
    return std::nullopt;
  }

  std::optional<media::AudioCapturerSource::ErrorCode> ErrorCode() {
    DCHECK(GetTaskRunner()->BelongsToCurrentThread());
    return error_code_;
  }

  // Returns a new MediaStreamAudioTrack. |id| is the blink track's ID in UTF-8.
  // Subclasses may override this to provide an extended implementation.
  virtual std::unique_ptr<MediaStreamAudioTrack> CreateMediaStreamAudioTrack(
      const std::string& id);

  // Number of MediaStreamAudioTracks added as consumers.
  size_t NumTracks() const override;

 protected:
  // Returns true if the source has already been started and has not yet been
  // stopped. Otherwise, attempts to start the source and returns true if
  // successful. While the source is running, it may provide audio on any thread
  // by calling DeliverDataToTracks().
  //
  // A default no-op implementation is provided in this base class. Subclasses
  // should override this method.
  virtual bool EnsureSourceIsStarted();

  // Stops the source and guarantees the the flow of audio data has stopped
  // (i.e., by the time this method returns, there will be no further calls to
  // DeliverDataToTracks() on any thread).
  //
  // A default no-op implementation is provided in this base class. Subclasses
  // should override this method.
  virtual void EnsureSourceIsStopped();

  // Stops the source and start the |new_device|.
  // A default no-op implementation is provided in this base class. Subclasses
  // should override this method.
  virtual void ChangeSourceImpl(const MediaStreamDevice& new_device);

  // Called by subclasses to update the format of the audio passing through this
  // source to the sinks. This may be called at any time, before or after
  // tracks have been connected; but must be called at least once before
  // DeliverDataToTracks(). This method is thread-safe.
  void SetFormat(const media::AudioParameters& params);

  // Called by subclasses to deliver audio data to the currently-connected
  // tracks. This method is thread-safe.
  void DeliverDataToTracks(const media::AudioBus& audio_bus,
                           base::TimeTicks reference_time,
                           const media::AudioGlitchInfo& glitch_info);

  // Called by subclasses when capture error occurs.
  // Note: This can be called on any thread, and will post a task to the main
  // thread to stop the source soon.
  void StopSourceOnError(media::AudioCapturerSource::ErrorCode code,
                         const std::string& why);

  // Sets muted state and notifies it to all registered tracks.
  void SetMutedState(bool state);

  // Maximum number of channels preferred by any connected track or -1 if
  // unknown.
  int NumPreferredChannels() const;

 private:
  // MediaStreamSource override.
  void DoStopSource() final;
  void DoChangeSource(const MediaStreamDevice& new_device) final;

  // Removes |track| from the list of instances that get a copy of the source
  // audio data. The "stop callback" that was provided to the track calls
  // this.
  void StopAudioDeliveryTo(MediaStreamAudioTrack* track);

  void LogMessage(const std::string& message);

  void SetErrorCode(media::AudioCapturerSource::ErrorCode code) {
    DCHECK(GetTaskRunner()->BelongsToCurrentThread());
    error_code_ = code;
  }

  // The portion of StopSourceOnError processing carried out on the main thread.
  void StopSourceOnErrorOnTaskRunner(
      media::AudioCapturerSource::ErrorCode code);

  // True if the source of audio is a local device. False if the source is
  // remote (e.g., streamed-in from a server).
  const bool is_local_source_;

  // Properties controlled by audio constraints.
  const bool disable_local_echo_;

  // Set to true once this source has been permanently stopped.
  bool is_stopped_;

  // Manages tracks connected to this source and the audio format and data flow.
  MediaStreamAudioDeliverer<MediaStreamAudioTrack> deliverer_;

  // Code set if this source was closed due to an error.
  std::optional<media::AudioCapturerSource::ErrorCode> error_code_;

  // Provides weak pointers so that MediaStreamAudioTracks won't call
  // StopAudioDeliveryTo() if this instance dies first.
  base::WeakPtrFactory<MediaStreamAudioSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIASTREAM_MEDIA_STREAM_AUDIO_SOURCE_H_
