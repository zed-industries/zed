/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CAPTURE_LINUX_CAMERA_PORTAL_H_
#define MODULES_VIDEO_CAPTURE_LINUX_CAMERA_PORTAL_H_

#include <memory>
#include <string>

#include "modules/portal/portal_request_response.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class CameraPortalPrivate;

class RTC_EXPORT CameraPortal {
 public:
  class PortalNotifier {
   public:
    virtual void OnCameraRequestResult(xdg_portal::RequestResponse result,
                                       int fd) = 0;

   protected:
    PortalNotifier() = default;
    virtual ~PortalNotifier() = default;
  };

  explicit CameraPortal(PortalNotifier* notifier);
  ~CameraPortal();

  void Start();

 private:
  std::unique_ptr<CameraPortalPrivate> private_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CAPTURE_LINUX_CAMERA_PORTAL_H_
