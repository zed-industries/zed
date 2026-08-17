// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_H_

#include <optional>
#include <string>

#include "media/base/video_facing.h"
#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom-blink.h"
#include "third_party/blink/renderer/modules/mediastream/image_capture_device_settings.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_constraints_util_sets.h"
#include "third_party/blink/renderer/modules/mediastream/video_track_adapter_settings.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_processor_options.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

extern const double kMinDeviceCaptureFrameRate;

// This class represents the output the SelectSettings algorithm for video
// constraints (see https://w3c.github.io/mediacapture-main/#dfn-selectsettings)
// The input to SelectSettings is a user-supplied constraints object, and its
// output is a set of implementation-specific settings that are used to
// configure other Chromium objects such as sources, tracks and sinks so that
// they work in the way indicated by the specification. VideoCaptureSettings may
// also be used to implement other constraints-related functionality, such as
// the getSettings() function.
// The following fields are used to control MediaStreamVideoSource objects:
//   * device_id: used for device selection and obtained from the deviceId
//   * capture_params: used to initialize video capture. Its values are obtained
//     from the width, height, aspectRatio, frame_rate, and googNoiseReduction
//     constraints.
// The following fields are used to control MediaStreamVideoTrack objects:
//   * track_adapter_settings: All track objects use a VideoTrackAdapter object
//     that may perform cropping and frame-rate adjustment. This field contains
//     the adapter settings suitable for the track the constraints are being
//     to. These settings are derived from the width, height, aspectRatio and
//     frameRate constraints.
// Some MediaStreamVideoSink objects (e.g. MediaStreamVideoWebRtcSink) require
// configuration derived from constraints that cannot be obtained from the
// source and track settings indicated above. The following fields are used
// to configure sinks:
//   * noise_reduction: used to control noise reduction for a screen-capture
//     track sent to a peer connection. Derive from the googNoiseReduction
//     constraint.
//   * min_frame_rate and max_frame_rate: used to control frame refreshes in
//     screen-capture tracks sent to a peer connection. Derived from the
//     frameRate constraint.
// If SelectSettings fails, the HasValue() method returns false and
// failed_constraint_name() returns the name of one of the (possibly multiple)
// constraints that could not be satisfied.
class MODULES_EXPORT VideoCaptureSettings {
 public:
  // Creates an object without value and with an empty failed constraint name.
  VideoCaptureSettings();

  // Creates an object without value and with the given
  // |failed_constraint_name|. Does not take ownership of
  // |failed_constraint_name|, so it must point to a string that remains
  // accessible. |failed_constraint_name| must be non-null.
  explicit VideoCaptureSettings(const char* failed_constraint_name);

  // Creates an object with the given values.
  VideoCaptureSettings(std::string device_id,
                       media::VideoCaptureParams capture_params,
                       std::optional<bool> noise_reduction,
                       const VideoTrackAdapterSettings& track_adapter_settings,
                       std::optional<double> min_frame_rate,
                       std::optional<double> max_frame_rate,
                       std::optional<ImageCaptureDeviceSettings>
                           image_capture_device_settings = std::nullopt);

  VideoCaptureSettings(const VideoCaptureSettings& other);
  VideoCaptureSettings& operator=(const VideoCaptureSettings& other);
  VideoCaptureSettings(VideoCaptureSettings&& other);
  VideoCaptureSettings& operator=(VideoCaptureSettings&& other);
  ~VideoCaptureSettings();

  bool HasValue() const { return !failed_constraint_name_; }

  // Convenience accessors for fields embedded in |capture_params_|.
  const media::VideoCaptureFormat& Format() const {
    return capture_params_.requested_format;
  }
  int Width() const {
    DCHECK(HasValue());
    return capture_params_.requested_format.frame_size.width();
  }
  int Height() const {
    DCHECK(HasValue());
    return capture_params_.requested_format.frame_size.height();
  }
  float FrameRate() const {
    DCHECK(HasValue());
    return capture_params_.requested_format.frame_rate;
  }
  media::ResolutionChangePolicy ResolutionChangePolicy() const {
    DCHECK(HasValue());
    return capture_params_.resolution_change_policy;
  }

  // Other accessors.
  const char* failed_constraint_name() const { return failed_constraint_name_; }

  const std::string& device_id() const {
    DCHECK(HasValue());
    return device_id_;
  }
  const media::VideoCaptureParams& capture_params() const {
    DCHECK(HasValue());
    return capture_params_;
  }
  const std::optional<bool>& noise_reduction() const {
    DCHECK(HasValue());
    return noise_reduction_;
  }
  const VideoTrackAdapterSettings& track_adapter_settings() const {
    DCHECK(HasValue());
    return track_adapter_settings_;
  }
  const std::optional<double>& min_frame_rate() const {
    DCHECK(HasValue());
    return min_frame_rate_;
  }
  const std::optional<double>& max_frame_rate() const {
    DCHECK(HasValue());
    return max_frame_rate_;
  }
  const std::optional<ImageCaptureDeviceSettings>&
  image_capture_device_settings() const {
    DCHECK(HasValue());
    return image_capture_device_settings_;
  }

 private:
  const char* failed_constraint_name_;
  std::string device_id_;
  media::VideoCaptureParams capture_params_;
  std::optional<bool> noise_reduction_;
  VideoTrackAdapterSettings track_adapter_settings_;
  std::optional<double> min_frame_rate_;
  std::optional<double> max_frame_rate_;
  std::optional<ImageCaptureDeviceSettings> image_capture_device_settings_;
};

// This class represents the output the SelectSettings algorithm for audio
// constraints (see https://w3c.github.io/mediacapture-main/#dfn-selectsettings)
// The input to SelectSettings is a user-supplied constraints object, and its
// output is a set of implementation-specific settings that are used to
// configure other Chromium objects such as sources, tracks and sinks so that
// they work in the way indicated by the specification. AudioCaptureSettings may
// also be used to implement other constraints-related functionality, such as
// the getSettings() function.
// The following fields are used to control MediaStreamVideoSource objects:
//   * device_id: used for device selection and obtained from the deviceId
//   * device_parameters: these are the hardware parameters for the device
//     selected by SelectSettings. They can be used to verify that the
//     parameters with which the audio stream is actually created corresponds
//     to what SelectSettings selected. It can also be used to implement
//     getSettings() for device-related properties such as sampleRate and
//     channelCount.
// The following fields are used to control various audio features:
//   * disable_local_echo
//   * render_to_associated_sink
// The audio_properties field is used to control the audio-processing module,
// which provides features such as software-based echo cancellation.
// If SelectSettings fails, the HasValue() method returns false and
// failed_constraint_name() returns the name of one of the (possibly multiple)
// constraints that could not be satisfied.
class MODULES_EXPORT AudioCaptureSettings {
 public:
  enum class ProcessingType {
    // System echo cancellation can be enabled, but all other processing is
    // disabled.
    kUnprocessed,
    // System echo cancellation and audio mirroring can be enabled, but all
    // other processing is disabled.
    kNoApmProcessed,
    // Processing is performed through WebRTC.
    kApmProcessed
  };

  // Creates an object without value and with an empty failed constraint name.
  AudioCaptureSettings();

  // Creates an object without value and with the given
  // |failed_constraint_name|. Does not take ownership of
  // |failed_constraint_name|, so it must point to a string that remains
  // accessible. |failed_constraint_name| must be non-null.
  explicit AudioCaptureSettings(const char* failed_constraint_name);

  // Creates an object with the given values.
  AudioCaptureSettings(
      std::string device_id,
      const std::optional<int>& requested_buffer_size,
      bool disable_local_echo,
      bool enable_automatic_output_device_selection,
      ProcessingType processing_type,
      const AudioProcessingProperties& audio_processing_properties,
      int num_channels);
  AudioCaptureSettings(const AudioCaptureSettings& other);
  AudioCaptureSettings& operator=(const AudioCaptureSettings& other);
  AudioCaptureSettings(AudioCaptureSettings&& other);
  AudioCaptureSettings& operator=(AudioCaptureSettings&& other);

  bool HasValue() const { return !failed_constraint_name_; }

  // Accessors.
  const char* failed_constraint_name() const { return failed_constraint_name_; }
  const std::string& device_id() const {
    DCHECK(HasValue());
    return device_id_;
  }
  const std::optional<int>& requested_buffer_size() const {
    DCHECK(HasValue());
    return requested_buffer_size_;
  }
  bool disable_local_echo() const {
    DCHECK(HasValue());
    return disable_local_echo_;
  }
  bool render_to_associated_sink() const {
    DCHECK(HasValue());
    return render_to_associated_sink_;
  }
  ProcessingType processing_type() const {
    DCHECK(HasValue());
    return processing_type_;
  }
  AudioProcessingProperties audio_processing_properties() const {
    DCHECK(HasValue());
    return audio_processing_properties_;
  }
  int num_channels() const {
    DCHECK(HasValue());
    return num_channels_;
  }

 private:
  const char* failed_constraint_name_;
  std::string device_id_;
  std::optional<int> requested_buffer_size_;
  bool disable_local_echo_;
  bool render_to_associated_sink_;
  ProcessingType processing_type_;
  AudioProcessingProperties audio_processing_properties_;
  int num_channels_;
};

// Method to get boolean value of constraint with |name| from constraints.
// Returns true if the constraint is specified in either mandatory or optional
// constraints.
MODULES_EXPORT bool GetConstraintValueAsBoolean(
    const MediaConstraints& constraints,
    const blink::BooleanConstraint MediaTrackConstraintSetPlatform::*picker,
    bool* value);

// Method to get int value of constraint with |name| from constraints.
// Returns true if the constraint is specified in either mandatory or Optional
// constraints.
MODULES_EXPORT bool GetConstraintValueAsInteger(
    const MediaConstraints& constraints,
    const blink::LongConstraint MediaTrackConstraintSetPlatform::*picker,
    int* value);

MODULES_EXPORT bool GetConstraintMinAsInteger(
    const MediaConstraints& constraints,
    const blink::LongConstraint MediaTrackConstraintSetPlatform::*picker,
    int* value);

MODULES_EXPORT bool GetConstraintMaxAsInteger(
    const MediaConstraints& constraints,
    const blink::LongConstraint MediaTrackConstraintSetPlatform::*picker,
    int* value);

// Method to get double precision value of constraint with |name| from
// constraints. Returns true if the constraint is specified in either mandatory
// or Optional constraints.
MODULES_EXPORT bool GetConstraintValueAsDouble(
    const MediaConstraints& constraints,
    const blink::DoubleConstraint MediaTrackConstraintSetPlatform::*picker,
    double* value);

bool IsPanTiltZoomConstraintPresentAndNotFalse(
    const MediaConstraints& constraints);

bool IsPanTiltZoomConstraintPresentAndNotFalse(
    const MediaTrackConstraintSetPlatform& constraint_set);

// This function selects track settings from a set of candidate resolutions and
// frame rates, given the source video-capture format and ideal values.
// The output are settings for a VideoTrackAdapter, which can adjust the
// resolution and frame rate of the source, and consist of
// target width, height and frame rate, and minimum and maximum aspect ratio.
// * Minimum and maximum aspect ratios are taken from |resolution_set| and are
//   not affected by ideal values.
// * The selected frame rate is always the value within the |frame_rate_set|
//   range that is closest to the ideal frame rate (or closest to the source
//   frame rate if no ideal is supplied). If the chosen frame rate is greater
//   than or equal to the source's frame rate, a value of 0.0 is returned, which
//   means that there will be no frame-rate adjustment.
// * If |enable_rescale| is false, no target width and height are computed.
// * If |enable_rescale| is true, the target width and height are selected using
//   the ResolutionSet::SelectClosestPointToIdeal function, using ideal values
//   for the width, height and aspectRatio properties from
//   |basic_constraint_set| and using the source's width and height as the
//   default resolution. The width and height returned by
//   SelectClosestPointToIdeal are rounded to the nearest int. For more details,
//   see the documentation for ResolutionSet::SelectClosestPointToIdeal.
// Note that this function ignores the min/max/exact values from
// |basic_constraint_set|. Only the ideal values for the width, height,
// aspectRatio and frameRate are used.
// This function has undefined behavior if any of |resolution_set| or
// |frame_rate_set| are empty.
MODULES_EXPORT VideoTrackAdapterSettings SelectVideoTrackAdapterSettings(
    const MediaTrackConstraintSetPlatform& basic_constraint_set,
    const media_constraints::ResolutionSet& resolution_set,
    const media_constraints::NumericRangeSet<double>& frame_rate_set,
    const media::VideoCaptureFormat& source_format,
    bool enable_rescale);

// Generic distance function between two values for numeric constraints. Based
// on the fitness-distance function described in
// https://w3c.github.io/mediacapture-main/#dfn-fitness-distance
MODULES_EXPORT double NumericConstraintFitnessDistance(double value1,
                                                       double value2);

// Fitness distance between |value| and |constraint|.
// Based on https://w3c.github.io/mediacapture-main/#dfn-fitness-distance.
double StringConstraintFitnessDistance(
    const blink::WebString& value,
    const blink::StringConstraint& constraint);

// This method computes capabilities for a video source based on the given
// |formats|. |facing_mode| is valid only in case of video device capture.
MODULES_EXPORT MediaStreamSource::Capabilities
ComputeCapabilitiesForVideoSource(
    const String& device_id,
    const media::VideoCaptureFormats& formats,
    mojom::blink::FacingMode facing_mode,
    bool is_device_capture,
    const std::optional<std::string>& group_id = std::nullopt);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_CONSTRAINTS_UTIL_H_
