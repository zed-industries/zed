/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TURN_CUSTOMIZER_H_
#define API_TURN_CUSTOMIZER_H_

#include <stdlib.h>

#include "api/transport/stun.h"
#include "p2p/base/port_interface.h"

namespace webrtc {

class TurnCustomizer {
 public:
  // This is called before a TURN message is sent.
  // This could be used to add implementation specific attributes to a request.
  virtual void MaybeModifyOutgoingStunMessage(PortInterface* port,
                                              StunMessage* message) = 0;

  // TURN can send data using channel data messages or Send indication.
  // This method should return false if `data` should be sent using
  // a Send indication instead of a ChannelData message, even if a
  // channel is bound.
  virtual bool AllowChannelData(PortInterface* port,
                                const void* data,
                                size_t size,
                                bool payload) = 0;

  virtual ~TurnCustomizer() {}
};

}  // namespace webrtc

#endif  // API_TURN_CUSTOMIZER_H_
