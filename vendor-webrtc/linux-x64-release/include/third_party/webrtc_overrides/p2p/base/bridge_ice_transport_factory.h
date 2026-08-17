// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_TRANSPORT_FACTORY_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_TRANSPORT_FACTORY_H_

#include <string>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/webrtc/api/ice_transport_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"

namespace blink {

// An ICE transport factory to construct transports backed by
// P2PTransportChannel and controlled by a BridgeIceController.
class RTC_EXPORT BridgeIceTransportFactory final
    : public webrtc::IceTransportFactory {
 public:
  // Task runner must be bound to the WebRTC network thread, on which
  // P2PTransportChannel expects to be constructed and used.
  explicit BridgeIceTransportFactory(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~BridgeIceTransportFactory() override = default;

  webrtc::scoped_refptr<webrtc::IceTransportInterface> CreateIceTransport(
      const std::string& transport_name,
      int component,
      webrtc::IceTransportInit init) override;

 private:
  const scoped_refptr<base::SingleThreadTaskRunner> network_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_TRANSPORT_FACTORY_H_
