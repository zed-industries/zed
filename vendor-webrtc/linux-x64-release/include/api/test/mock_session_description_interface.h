/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_SESSION_DESCRIPTION_INTERFACE_H_
#define API_TEST_MOCK_SESSION_DESCRIPTION_INTERFACE_H_

#include <cstddef>
#include <memory>
#include <string>
#include <type_traits>
#include <vector>

#include "api/candidate.h"
#include "api/jsep.h"
#include "test/gmock.h"

namespace webrtc {

class MockSessionDescriptionInterface : public SessionDescriptionInterface {
 public:
  MOCK_METHOD(std::unique_ptr<SessionDescriptionInterface>,
              Clone,
              (),
              (const, override));
  MOCK_METHOD(SessionDescription*, description, (), (override));
  MOCK_METHOD(const SessionDescription*, description, (), (const, override));
  MOCK_METHOD(std::string, session_id, (), (const, override));
  MOCK_METHOD(std::string, session_version, (), (const, override));
  MOCK_METHOD(SdpType, GetType, (), (const, override));
  MOCK_METHOD(std::string, type, (), (const, override));
  MOCK_METHOD(bool, AddCandidate, (const IceCandidateInterface*), (override));
  MOCK_METHOD(size_t,
              RemoveCandidates,
              (const std::vector<webrtc::Candidate>&),
              (override));
  MOCK_METHOD(size_t, number_of_mediasections, (), (const, override));
  MOCK_METHOD(const IceCandidateCollection*,
              candidates,
              (size_t),
              (const, override));
  MOCK_METHOD(bool, ToString, (std::string*), (const, override));
};

static_assert(!std::is_abstract_v<MockSessionDescriptionInterface>);

}  // namespace webrtc

#endif  // API_TEST_MOCK_SESSION_DESCRIPTION_INTERFACE_H_
