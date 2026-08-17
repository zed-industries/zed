/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_DEGRADATION_PREFERENCE_PROVIDER_H_
#define CALL_ADAPTATION_DEGRADATION_PREFERENCE_PROVIDER_H_

#include "api/rtp_parameters.h"

namespace webrtc {

class DegradationPreferenceProvider {
 public:
  virtual ~DegradationPreferenceProvider();

  virtual DegradationPreference degradation_preference() const = 0;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_DEGRADATION_PREFERENCE_PROVIDER_H_
