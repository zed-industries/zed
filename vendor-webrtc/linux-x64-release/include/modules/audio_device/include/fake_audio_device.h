/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_INCLUDE_FAKE_AUDIO_DEVICE_H_
#define MODULES_AUDIO_DEVICE_INCLUDE_FAKE_AUDIO_DEVICE_H_

#include "api/audio/audio_device.h"
#include "modules/audio_device/include/audio_device_default.h"

namespace webrtc {

class FakeAudioDeviceModule
    : public webrtc_impl::AudioDeviceModuleDefault<AudioDeviceModule> {
 public:
  // TODO(bugs.webrtc.org/12701): Fix all users of this class to managed
  // references using scoped_refptr. Current code doesn't always use refcounting
  // for this class.
  void AddRef() const override {}
  webrtc::RefCountReleaseStatus Release() const override {
    return webrtc::RefCountReleaseStatus::kDroppedLastRef;
  }
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_INCLUDE_FAKE_AUDIO_DEVICE_H_
