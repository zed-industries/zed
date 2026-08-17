/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CAPTURE_LINUX_DEVICE_INFO_V4L2_H_
#define MODULES_VIDEO_CAPTURE_LINUX_DEVICE_INFO_V4L2_H_

#include <stdint.h>

#include "modules/video_capture/device_info_impl.h"

namespace webrtc {
namespace videocapturemodule {
class DeviceInfoV4l2 : public DeviceInfoImpl {
 public:
  DeviceInfoV4l2();
  ~DeviceInfoV4l2() override;
  uint32_t NumberOfDevices() override;
  int32_t GetDeviceName(uint32_t deviceNumber,
                        char* deviceNameUTF8,
                        uint32_t deviceNameLength,
                        char* deviceUniqueIdUTF8,
                        uint32_t deviceUniqueIdUTF8Length,
                        char* productUniqueIdUTF8 = 0,
                        uint32_t productUniqueIdUTF8Length = 0) override;
  /*
   * Fills the membervariable _captureCapabilities with capabilites for the
   * given device name.
   */
  int32_t CreateCapabilityMap(const char* deviceUniqueIdUTF8) override
      RTC_EXCLUSIVE_LOCKS_REQUIRED(_apiLock);
  int32_t DisplayCaptureSettingsDialogBox(const char* /*deviceUniqueIdUTF8*/,
                                          const char* /*dialogTitleUTF8*/,
                                          void* /*parentWindow*/,
                                          uint32_t /*positionX*/,
                                          uint32_t /*positionY*/) override;
  int32_t FillCapabilities(int fd) RTC_EXCLUSIVE_LOCKS_REQUIRED(_apiLock);
  int32_t Init() override;

 private:
  bool IsDeviceNameMatches(const char* name, const char* deviceUniqueIdUTF8);
};
}  // namespace videocapturemodule
}  // namespace webrtc
#endif  // MODULES_VIDEO_CAPTURE_LINUX_DEVICE_INFO_V4L2_H_
