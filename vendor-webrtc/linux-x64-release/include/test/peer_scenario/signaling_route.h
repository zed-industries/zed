/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PEER_SCENARIO_SIGNALING_ROUTE_H_
#define TEST_PEER_SCENARIO_SIGNALING_ROUTE_H_

#include <string>
#include <utility>

#include "test/network/network_emulation_manager.h"
#include "test/peer_scenario/peer_scenario_client.h"

namespace webrtc {
namespace test {

// Helper class to reduce the amount of boilerplate required for ICE signalling
// ad SDP negotiation.
class SignalingRoute {
 public:
  SignalingRoute(PeerScenarioClient* caller,
                 PeerScenarioClient* callee,
                 CrossTrafficRoute* send_route,
                 CrossTrafficRoute* ret_route);

  void StartIceSignaling();

  // The `modify_offer` callback is used to modify an offer after the local
  // description has been set. This is legal (but odd) behavior.
  // The `munge_offer` callback is used to modify an offer between its creation
  // and set local description. This behavior is forbidden according to the spec
  // but available here in order to allow test coverage on corner cases.
  // `callee_remote_description_set` is invoked when callee has applied the
  // offer but not yet created an answer. The purpose is to allow tests to
  // modify transceivers created from the offer.  The `exchange_finished`
  // callback is called with the answer produced after SDP negotations has
  // completed.
  // TODO(srte): Handle lossy links.
  void NegotiateSdp(
      std::function<void(SessionDescriptionInterface* offer)> munge_offer,
      std::function<void(SessionDescriptionInterface* offer)> modify_offer,
      std::function<void()> callee_remote_description_set,
      std::function<void(const SessionDescriptionInterface& answer)>
          exchange_finished);
  void NegotiateSdp(
      std::function<void(SessionDescriptionInterface* offer)> munge_offer,
      std::function<void(SessionDescriptionInterface* offer)> modify_offer,
      std::function<void(const SessionDescriptionInterface& answer)>
          exchange_finished);
  void NegotiateSdp(
      std::function<void(SessionDescriptionInterface* offer)> modify_offer,
      std::function<void(const SessionDescriptionInterface& answer)>
          exchange_finished);
  void NegotiateSdp(
      std::function<void()> remote_description_set,
      std::function<void(const SessionDescriptionInterface& answer)>
          exchange_finished);
  void NegotiateSdp(
      std::function<void(const SessionDescriptionInterface& answer)>
          exchange_finished);
  SignalingRoute reverse() {
    return SignalingRoute(callee_, caller_, ret_route_, send_route_);
  }

 private:
  PeerScenarioClient* const caller_;
  PeerScenarioClient* const callee_;
  CrossTrafficRoute* const send_route_;
  CrossTrafficRoute* const ret_route_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_PEER_SCENARIO_SIGNALING_ROUTE_H_
