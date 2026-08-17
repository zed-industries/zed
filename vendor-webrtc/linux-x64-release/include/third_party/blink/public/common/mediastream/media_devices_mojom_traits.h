// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICES_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICES_MOJOM_TRAITS_H_

#include "media/capture/mojom/video_capture_types.mojom.h"
#include "media/capture/video/video_capture_device_descriptor.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/mediastream/media_devices.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom-shared.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::MediaDeviceInfoDataView,
                                        blink::WebMediaDeviceInfo> {
  static const std::string& device_id(const blink::WebMediaDeviceInfo& info) {
    return info.device_id;
  }

  static const std::string& label(const blink::WebMediaDeviceInfo& info) {
    return info.label;
  }

  static const std::string& group_id(const blink::WebMediaDeviceInfo& info) {
    return info.group_id;
  }

  static const media::VideoCaptureControlSupport& control_support(
      const blink::WebMediaDeviceInfo& info) {
    return info.video_control_support;
  }

  static const blink::mojom::FacingMode& facing_mode(
      const blink::WebMediaDeviceInfo& info) {
    return info.video_facing;
  }

  static const std::optional<media::CameraAvailability>& availability(
      const blink::WebMediaDeviceInfo& info) {
    return info.availability;
  }

  static bool Read(blink::mojom::MediaDeviceInfoDataView input,
                   blink::WebMediaDeviceInfo* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MEDIASTREAM_MEDIA_DEVICES_MOJOM_TRAITS_H_
