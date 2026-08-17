/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_PCC_PCC_FACTORY_H_
#define MODULES_CONGESTION_CONTROLLER_PCC_PCC_FACTORY_H_

#include <memory>

#include "api/transport/network_control.h"
#include "api/units/time_delta.h"

namespace webrtc {

class PccNetworkControllerFactory : public NetworkControllerFactoryInterface {
 public:
  PccNetworkControllerFactory();
  std::unique_ptr<NetworkControllerInterface> Create(
      NetworkControllerConfig config) override;
  TimeDelta GetProcessInterval() const override;
};
}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_PCC_PCC_FACTORY_H_
