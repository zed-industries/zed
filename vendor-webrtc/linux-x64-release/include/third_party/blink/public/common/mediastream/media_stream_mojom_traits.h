// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_MOJOM_TRAITS_H_

#include <optional>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/mediastream/media_stream_controls.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-forward.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::MediaStreamDeviceDataView,
                                        blink::MediaStreamDevice> {
  static const blink::mojom::MediaStreamType& type(
      const blink::MediaStreamDevice& device) {
    return device.type;
  }

  static const std::string& id(const blink::MediaStreamDevice& device) {
    return device.id;
  }

  static int64_t display_id(const blink::MediaStreamDevice& device) {
    return device.display_id;
  }

  static const media::VideoFacingMode& video_facing(
      const blink::MediaStreamDevice& device) {
    return device.video_facing;
  }

  static const std::optional<std::string>& group_id(
      const blink::MediaStreamDevice& device) {
    return device.group_id;
  }

  static const std::optional<std::string>& matched_output_device_id(
      const blink::MediaStreamDevice& device) {
    return device.matched_output_device_id;
  }

  static const std::string& name(const blink::MediaStreamDevice& device) {
    return device.name;
  }

  static const media::AudioParameters& input(
      const blink::MediaStreamDevice& device) {
    return device.input;
  }

  static const std::optional<base::UnguessableToken>& session_id(
      const blink::MediaStreamDevice& device) {
    return device.serializable_session_id();
  }

  static const media::mojom::DisplayMediaInformationPtr& display_media_info(
      const blink::MediaStreamDevice& device) {
    return device.display_media_info;
  }

  static bool Read(blink::mojom::MediaStreamDeviceDataView input,
                   blink::MediaStreamDevice* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::TrackControlsDataView, blink::TrackControls> {
  static const blink::mojom::MediaStreamType& stream_type(
      const blink::TrackControls& controls) {
    return controls.stream_type;
  }

  static const std::vector<std::string>& device_ids(
      const blink::TrackControls& controls) {
    return controls.device_ids;
  }

  static bool Read(blink::mojom::TrackControlsDataView input,
                   blink::TrackControls* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::StreamControlsDataView, blink::StreamControls> {
  static const blink::TrackControls& audio(
      const blink::StreamControls& controls) {
    return controls.audio;
  }

  static const blink::TrackControls& video(
      const blink::StreamControls& controls) {
    return controls.video;
  }

  static bool hotword_enabled(const blink::StreamControls& controls) {
    return controls.hotword_enabled;
  }

  static bool disable_local_echo(const blink::StreamControls& controls) {
    return controls.disable_local_echo;
  }

  static bool suppress_local_audio_playback(
      const blink::StreamControls& controls) {
    return controls.suppress_local_audio_playback;
  }

  static bool exclude_system_audio(const blink::StreamControls& controls) {
    return controls.exclude_system_audio;
  }

  static bool exclude_self_browser_surface(
      const blink::StreamControls& controls) {
    return controls.exclude_self_browser_surface;
  }

  static bool request_pan_tilt_zoom_permission(
      const blink::StreamControls& controls) {
    return controls.request_pan_tilt_zoom_permission;
  }

  static bool request_all_screens(const blink::StreamControls& controls) {
    return controls.request_all_screens;
  }

  static blink::mojom::PreferredDisplaySurface preferred_display_surface(
      const blink::StreamControls& controls) {
    return controls.preferred_display_surface;
  }

  static bool dynamic_surface_switching_requested(
      const blink::StreamControls& controls) {
    return controls.dynamic_surface_switching_requested;
  }

  static bool exclude_monitor_type_surfaces(
      const blink::StreamControls& controls) {
    return controls.exclude_monitor_type_surfaces;
  }

  static bool Read(blink::mojom::StreamControlsDataView input,
                   blink::StreamControls* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_MOJOM_TRAITS_H_
