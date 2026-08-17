/* Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This is an EXPERIMENTAL interface.

#ifndef API_FEC_CONTROLLER_OVERRIDE_H_
#define API_FEC_CONTROLLER_OVERRIDE_H_

namespace webrtc {

// Interface for temporarily overriding FecController's bitrate allocation.
class FecControllerOverride {
 public:
  virtual void SetFecAllowed(bool fec_allowed) = 0;

 protected:
  virtual ~FecControllerOverride() = default;
};

}  // namespace webrtc

#endif  // API_FEC_CONTROLLER_OVERRIDE_H_
