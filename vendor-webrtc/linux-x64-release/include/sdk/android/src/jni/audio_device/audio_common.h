/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_COMMON_H_
#define SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_COMMON_H_

namespace webrtc {

namespace jni {

const int kDefaultSampleRate = 44100;
// Delay estimates for the two different supported modes. These values are based
// on real-time round-trip delay estimates on a large set of devices and they
// are lower bounds since the filter length is 128 ms, so the AEC works for
// delays in the range [50, ~170] ms and [150, ~270] ms. Note that, in most
// cases, the lowest delay estimate will not be utilized since devices that
// support low-latency output audio often supports HW AEC as well.
const int kLowLatencyModeDelayEstimateInMilliseconds = 50;
const int kHighLatencyModeDelayEstimateInMilliseconds = 150;

}  // namespace jni

}  // namespace webrtc

#endif  // SDK_ANDROID_SRC_JNI_AUDIO_DEVICE_AUDIO_COMMON_H_
