// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_INPUT_DEVICE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_INPUT_DEVICE_INFO_H_

#include "third_party/blink/renderer/modules/mediastream/media_device_info.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"

namespace blink {

class MediaTrackCapabilities;

class MODULES_EXPORT InputDeviceInfo final : public MediaDeviceInfo {
  DEFINE_WRAPPERTYPEINFO();

 public:
  InputDeviceInfo(const String& device_id,
                  const String& label,
                  const String& group_id,
                  mojom::blink::MediaDeviceType);

  void SetVideoInputCapabilities(mojom::blink::VideoInputDeviceCapabilitiesPtr);
  void SetAudioInputCapabilities(mojom::blink::AudioInputDeviceCapabilitiesPtr);

  MediaTrackCapabilities* getCapabilities() const;

 private:
  MediaStreamSource::Capabilities platform_capabilities_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_INPUT_DEVICE_INFO_H_
