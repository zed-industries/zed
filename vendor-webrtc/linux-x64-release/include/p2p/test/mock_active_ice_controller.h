/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_TEST_MOCK_ACTIVE_ICE_CONTROLLER_H_
#define P2P_TEST_MOCK_ACTIVE_ICE_CONTROLLER_H_

#include <memory>

#include "p2p/base/active_ice_controller_factory_interface.h"
#include "p2p/base/active_ice_controller_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/transport_description.h"
#include "test/gmock.h"

namespace webrtc {

class MockActiveIceController : public ActiveIceControllerInterface {
 public:
  explicit MockActiveIceController(
      const ActiveIceControllerFactoryArgs& /* args */) {}
  ~MockActiveIceController() override = default;

  MOCK_METHOD(void, SetIceConfig, (const webrtc::IceConfig&), (override));
  MOCK_METHOD(void, OnConnectionAdded, (const webrtc::Connection*), (override));
  MOCK_METHOD(void,
              OnConnectionSwitched,
              (const webrtc::Connection*),
              (override));
  MOCK_METHOD(void,
              OnConnectionDestroyed,
              (const webrtc::Connection*),
              (override));
  MOCK_METHOD(void,
              OnConnectionPinged,
              (const webrtc::Connection*),
              (override));
  MOCK_METHOD(void,
              OnConnectionUpdated,
              (const webrtc::Connection*),
              (override));
  MOCK_METHOD(bool,
              GetUseCandidateAttribute,
              (const webrtc::Connection*,
               webrtc::NominationMode,
               webrtc::IceMode),
              (const, override));
  MOCK_METHOD(void,
              OnSortAndSwitchRequest,
              (webrtc::IceSwitchReason),
              (override));
  MOCK_METHOD(void,
              OnImmediateSortAndSwitchRequest,
              (webrtc::IceSwitchReason),
              (override));
  MOCK_METHOD(bool,
              OnImmediateSwitchRequest,
              (webrtc::IceSwitchReason, const webrtc::Connection*),
              (override));
  MOCK_METHOD(const Connection*, FindNextPingableConnection, (), (override));
};

class MockActiveIceControllerFactory
    : public ActiveIceControllerFactoryInterface {
 public:
  ~MockActiveIceControllerFactory() override = default;

  std::unique_ptr<ActiveIceControllerInterface> Create(
      const ActiveIceControllerFactoryArgs& args) {
    RecordActiveIceControllerCreated();
    return std::make_unique<MockActiveIceController>(args);
  }

  MOCK_METHOD(void, RecordActiveIceControllerCreated, ());
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::MockActiveIceController;
using ::webrtc::MockActiveIceControllerFactory;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_TEST_MOCK_ACTIVE_ICE_CONTROLLER_H_
