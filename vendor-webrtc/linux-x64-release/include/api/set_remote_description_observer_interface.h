/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_SET_REMOTE_DESCRIPTION_OBSERVER_INTERFACE_H_
#define API_SET_REMOTE_DESCRIPTION_OBSERVER_INTERFACE_H_

#include "api/ref_count.h"
#include "api/rtc_error.h"

namespace webrtc {

// An observer for PeerConnectionInterface::SetRemoteDescription(). The
// callback is invoked such that the state of the peer connection can be
// examined to accurately reflect the effects of the SetRemoteDescription
// operation.
class SetRemoteDescriptionObserverInterface : public webrtc::RefCountInterface {
 public:
  // On success, `error.ok()` is true.
  virtual void OnSetRemoteDescriptionComplete(RTCError error) = 0;
};

}  // namespace webrtc

#endif  // API_SET_REMOTE_DESCRIPTION_OBSERVER_INTERFACE_H_
