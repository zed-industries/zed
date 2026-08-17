/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_OBJC_AUDIO_DEVICE_MODULE_H_
#define SDK_OBJC_NATIVE_API_OBJC_AUDIO_DEVICE_MODULE_H_

#include "api/audio/audio_device.h"
#import "components/audio/RTCAudioDevice.h"

namespace webrtc {

webrtc::scoped_refptr<AudioDeviceModule> CreateAudioDeviceModule(
    id<RTC_OBJC_TYPE(RTCAudioDevice)> audio_device);

}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_OBJC_AUDIO_DEVICE_MODULE_H_
