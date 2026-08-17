/*
 *  Copyright (c) 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_API_AUDIO_DEVICE_MODULE_ERROR_HANDLER_H
#define SDK_OBJC_NATIVE_API_AUDIO_DEVICE_MODULE_ERROR_HANDLER_H

namespace webrtc {
enum ADMError : uint8_t {
  // Generic.
  kNotInitialized,
  kInitializationFailed,
  kTerminateFailed,

  // Playout / speaker errors.
  kInitSpeakerFailed,
  kPlayoutInitFailed,
  kPlayoutStartFailed,
  kPlayoutFailed,
  kPlayoutDeviceFailed,
  kPlayoutDelayFailed,
  kPlayoutDeviceNameFailed,
  kStereoPlayoutFailed,
  kSpeakerMuteFailed,
  kSpeakerVolumeFailed,

  // Recording / microphone errors.
  kInitMicrophoneFailed,
  kRecordingInitFailed,
  kRecordingStartFailed,
  kRecordingFailed,
  kRecordingDeviceFailed,
  kRecordingDeviceNameFailed,
  kStereoRecordingFailed,
  kMicrophoneMuteFailed,
  kMicrophoneVolumeFailed,

  // Others.
  kNoActiveAudioLayer,
  kRegisterAudioCallbackFailed,
  kEnableBuiltInAECFailed,
  kEnableBuiltInAGCFailed,
  kEnableBuiltInNSFailed,
};
typedef void (^ADMErrorHandler)(ADMError error);
}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_API_AUDIO_DEVICE_MODULE_ERROR_HANDLER_H
