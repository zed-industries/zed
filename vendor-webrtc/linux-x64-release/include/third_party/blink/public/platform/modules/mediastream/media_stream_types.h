// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_MEDIA_STREAM_TYPES_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_MEDIA_STREAM_TYPES_H_

#include "media/capture/video_capture_types.h"

namespace blink {

using VideoTrackSettingsCallback =
    base::RepeatingCallback<void(gfx::Size frame_size,
                                 double frame_rate,
                                 std::optional<gfx::Size> metadata_source_size,
                                 std::optional<float> device_scale_factor)>;

using VideoTrackFormatCallback =
    base::RepeatingCallback<void(const media::VideoCaptureFormat&)>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_MEDIASTREAM_MEDIA_STREAM_TYPES_H_
