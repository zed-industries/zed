/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_SET_LOCAL_DESCRIPTION_OBSERVER_INTERFACE_H_
#define API_SET_LOCAL_DESCRIPTION_OBSERVER_INTERFACE_H_

#include "api/ref_count.h"
#include "api/rtc_error.h"

namespace webrtc {

// OnSetLocalDescriptionComplete() invokes as soon as
// PeerConnectionInterface::SetLocalDescription() operation completes, allowing
// the observer to examine the effects of the operation without delay.
class SetLocalDescriptionObserverInterface : public webrtc::RefCountInterface {
 public:
  // On success, `error.ok()` is true.
  virtual void OnSetLocalDescriptionComplete(RTCError error) = 0;
};

}  // namespace webrtc

#endif  // API_SET_LOCAL_DESCRIPTION_OBSERVER_INTERFACE_H_
