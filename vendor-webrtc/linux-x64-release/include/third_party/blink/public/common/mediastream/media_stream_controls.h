// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_CONTROLS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_CONTROLS_H_

#include <string>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"

namespace blink {

// Names for media stream source capture types.
// These are values set via the "chromeMediaSource" constraint.
BLINK_COMMON_EXPORT extern const char kMediaStreamSourceTab[];
BLINK_COMMON_EXPORT extern const char
    kMediaStreamSourceScreen[]; /* video only */
BLINK_COMMON_EXPORT extern const char kMediaStreamSourceDesktop[];
BLINK_COMMON_EXPORT extern const char
    kMediaStreamSourceSystem[]; /* audio only */

struct BLINK_COMMON_EXPORT TrackControls {
  TrackControls();
  explicit TrackControls(mojom::MediaStreamType type);
  TrackControls(const TrackControls& other);
  ~TrackControls();

  bool requested() const {
    return stream_type != mojom::MediaStreamType::NO_SERVICE;
  }

  // Represents the requested  stream type.
  mojom::MediaStreamType stream_type = mojom::MediaStreamType::NO_SERVICE;

  // An empty string represents the default device.
  // A nonempty string represents a specific device.
  std::vector<std::string> device_ids;
};

// StreamControls describes what is sent to the browser process
// from the renderer process in order to control the opening of a device
// pair. This may result in opening one audio and/or one video device.
// This has to be a struct with public members in order to allow it to
// be sent in the mojo IPC.
struct BLINK_COMMON_EXPORT StreamControls {
  StreamControls();
  StreamControls(bool request_audio, bool request_video);
  ~StreamControls();

  TrackControls audio;
  TrackControls video;

  // Hotword functionality (chromeos only)
  // TODO(crbug.com/577627): this is now never set and needs to be removed.
  bool hotword_enabled = false;
  bool disable_local_echo = false;
  bool suppress_local_audio_playback = false;
  bool exclude_system_audio = false;
  bool exclude_self_browser_surface = false;
  bool request_pan_tilt_zoom_permission = false;
  bool request_all_screens = false;
  mojom::PreferredDisplaySurface preferred_display_surface =
      mojom::PreferredDisplaySurface::NO_PREFERENCE;
  // Flag to request that a "Share this tab instead" button is shown to change
  // the target of the tab-capture to the other tab.
  bool dynamic_surface_switching_requested = true;
  bool exclude_monitor_type_surfaces = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_STREAM_CONTROLS_H_
