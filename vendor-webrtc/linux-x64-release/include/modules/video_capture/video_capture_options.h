/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CAPTURE_VIDEO_CAPTURE_OPTIONS_H_
#define MODULES_VIDEO_CAPTURE_VIDEO_CAPTURE_OPTIONS_H_

#include "api/scoped_refptr.h"
#include "rtc_base/system/rtc_export.h"

#if defined(WEBRTC_USE_PIPEWIRE)
#include "modules/portal/pipewire_utils.h"
#endif

namespace webrtc {

#if defined(WEBRTC_USE_PIPEWIRE)
namespace videocapturemodule {
class PipeWireSession;
}
#endif

// An object that stores initialization parameters for video capturers
class RTC_EXPORT VideoCaptureOptions {
 public:
  VideoCaptureOptions();
  VideoCaptureOptions(const VideoCaptureOptions& options);
  VideoCaptureOptions(VideoCaptureOptions&& options);
  ~VideoCaptureOptions();

  VideoCaptureOptions& operator=(const VideoCaptureOptions& options);
  VideoCaptureOptions& operator=(VideoCaptureOptions&& options);

  enum class Status {
    SUCCESS,
    UNINITIALIZED,
    UNAVAILABLE,
    DENIED,
    ERROR,
    MAX_VALUE = ERROR
  };

  class Callback {
   public:
    virtual void OnInitialized(Status status) = 0;

   protected:
    virtual ~Callback() = default;
  };

  void Init(Callback* callback);

#if defined(WEBRTC_LINUX)
  bool allow_v4l2() const { return allow_v4l2_; }
  void set_allow_v4l2(bool allow) { allow_v4l2_ = allow; }
#endif

#if defined(WEBRTC_USE_PIPEWIRE)
  bool allow_pipewire() const { return allow_pipewire_; }
  void set_allow_pipewire(bool allow) { allow_pipewire_ = allow; }
  void set_pipewire_fd(int fd) { pipewire_fd_ = fd; }
  webrtc::scoped_refptr<videocapturemodule::PipeWireSession> pipewire_session();
#endif

 private:
#if defined(WEBRTC_LINUX)
  bool allow_v4l2_ = false;
#endif
#if defined(WEBRTC_USE_PIPEWIRE)
  bool allow_pipewire_ = false;
  int pipewire_fd_ = kInvalidPipeWireFd;
  webrtc::scoped_refptr<videocapturemodule::PipeWireSession> pipewire_session_;
#endif
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CAPTURE_VIDEO_CAPTURE_OPTIONS_H_
