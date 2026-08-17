// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_REQUEST_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_REQUEST_H_

#include <stddef.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/unguessable_token.h"
#include "media/base/audio_parameters.h"
#include "media/base/video_facing.h"
#include "media/capture/video/video_capture_device_descriptor.h"
#include "media/mojo/mojom/display_media_information.mojom.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-forward.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-shared.h"
#include "ui/display/types/display_constants.h"

namespace blink {

// Types of media stream requests that can be made to the media controller.
enum MediaStreamRequestType {
  MEDIA_DEVICE_ACCESS = 0,
  MEDIA_DEVICE_UPDATE,
  MEDIA_GENERATE_STREAM,
  MEDIA_GET_OPEN_DEVICE,
  MEDIA_OPEN_DEVICE_PEPPER_ONLY  // Only used in requests made by Pepper.
};

// Convenience predicates to determine whether the given type represents some
// audio or some video device.
BLINK_COMMON_EXPORT bool IsAudioInputMediaType(mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsVideoInputMediaType(mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsScreenCaptureMediaType(mojom::MediaStreamType type);
// Whether the |type| captures anything on the screen.
BLINK_COMMON_EXPORT bool IsVideoScreenCaptureMediaType(
    mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsDesktopCaptureMediaType(mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsAudioDesktopCaptureMediaType(
    mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsVideoDesktopCaptureMediaType(
    mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsTabCaptureMediaType(mojom::MediaStreamType type);
BLINK_COMMON_EXPORT bool IsDeviceMediaType(mojom::MediaStreamType type);

// TODO(xians): Change the structs to classes.
// Represents one device in a request for media stream(s).
struct BLINK_COMMON_EXPORT MediaStreamDevice {
  MediaStreamDevice();
  MediaStreamDevice(mojom::MediaStreamType type,
                    const std::string& id,
                    const std::string& name);
  MediaStreamDevice(mojom::MediaStreamType type,
                    const std::string& id,
                    const std::string& name,
                    int64_t display_id);
  MediaStreamDevice(mojom::MediaStreamType type,
                    const std::string& id,
                    const std::string& name,
                    const media::VideoCaptureControlSupport& control_support,
                    media::VideoFacingMode facing,
                    const std::optional<std::string>& group_id = std::nullopt);
  MediaStreamDevice(mojom::MediaStreamType type,
                    const std::string& id,
                    const std::string& name,
                    int sample_rate,
                    const media::ChannelLayoutConfig& channel_layout_config,
                    int frames_per_buffer);
  MediaStreamDevice(const MediaStreamDevice& other);
  ~MediaStreamDevice();

  MediaStreamDevice& operator=(const MediaStreamDevice& other);

  bool IsSameDevice(const MediaStreamDevice& other_device) const;
  bool operator==(const MediaStreamDevice& other_device) const;

  base::UnguessableToken session_id() const {
    return session_id_ ? *session_id_ : base::UnguessableToken();
  }

  const std::optional<base::UnguessableToken>& serializable_session_id() const {
    return session_id_;
  }

  void set_session_id(const base::UnguessableToken& session_id) {
    session_id_ = session_id.is_empty()
                      ? std::optional<base::UnguessableToken>()
                      : session_id;
  }

  // The device's type.
  mojom::MediaStreamType type;

  // The device's unique ID.
  std::string id;

  // The device's unique display id if the device is a display.
  // display::kInvalidDisplayId should be used in case a surface type other
  // than monitor is requested.
  int64_t display_id = display::kInvalidDisplayId;

  // The control support for video capture device.
  media::VideoCaptureControlSupport video_control_support;

  // The facing mode for video capture device.
  media::VideoFacingMode video_facing;

  // The device's group ID.
  std::optional<std::string> group_id;

  // The device id of a matched output device if any (otherwise empty).
  // Only applicable to audio devices.
  std::optional<std::string> matched_output_device_id;

  // The device's "friendly" name. Not guaranteed to be unique.
  std::string name;

  // Contains the device properties of the capture device. It's valid only when
  // the type of device is audio (i.e. IsAudioInputMediaType returns true).
  media::AudioParameters input =
      media::AudioParameters::UnavailableDeviceParams();

  // This field is only non-null for display media devices.
  media::mojom::DisplayMediaInformationPtr display_media_info;

 private:
  // Id for this capture session. Unique for all sessions of the same type.
  std::optional<base::UnguessableToken> session_id_;  // = kNoId;
};

using MediaStreamDevices = std::vector<MediaStreamDevice>;

// TODO(crbug.com/1313021): Remove this function and use
// blink::mojom::StreamDevicesSet directly everywhere.
// Takes a mojom::StreamDevicesSet and returns all contained MediaStreamDevices.
BLINK_COMMON_EXPORT MediaStreamDevices
ToMediaStreamDevicesList(const mojom::StreamDevicesSet& stream_devices_set);

BLINK_COMMON_EXPORT size_t CountDevices(const mojom::StreamDevices& devices);
BLINK_COMMON_EXPORT bool IsMediaStreamDeviceTransferrable(
    const MediaStreamDevice& device);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_REQUEST_H_
