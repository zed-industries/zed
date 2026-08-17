/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_DELEGATE_H_
#define SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_DELEGATE_H_

#include "api/scoped_refptr.h"
#include "rtc_base/thread.h"

#import "components/audio/RTCAudioDevice.h"

namespace webrtc {
namespace objc_adm {
class ObjCAudioDeviceModule;
}  // namespace objc_adm
}  // namespace webrtc

@interface RTC_OBJC_TYPE(RTCObjCAudioDeviceDelegate) : NSObject <RTC_OBJC_TYPE (RTCAudioDeviceDelegate)>

- (instancetype)
    initWithAudioDeviceModule:
        (webrtc::scoped_refptr<webrtc::objc_adm::ObjCAudioDeviceModule>)
            audioDeviceModule
            audioDeviceThread:(webrtc::Thread*)thread;

- (void)resetAudioDeviceModule;

@end

#endif  // SDK_OBJC_NATIVE_SRC_OBJC_AUDIO_DEVICE_DELEGATE_H_
