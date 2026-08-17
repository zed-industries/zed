/*
 *  Copyright 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_DTMF_SENDER_H_
#define API_TEST_MOCK_DTMF_SENDER_H_

#include <string>
#include <type_traits>

#include "api/dtmf_sender_interface.h"
#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"
#include "rtc_base/ref_counted_object.h"
#include "test/gmock.h"

namespace webrtc {

class MockDtmfSenderObserver : public DtmfSenderObserverInterface {
 public:
  MOCK_METHOD(void,
              OnToneChange,
              (const std::string&, const std::string&),
              (override));
  MOCK_METHOD(void, OnToneChange, (const std::string&), (override));
};

static_assert(!std::is_abstract_v<MockDtmfSenderObserver>, "");

class MockDtmfSender : public DtmfSenderInterface {
 public:
  static scoped_refptr<MockDtmfSender> Create() {
    return make_ref_counted<MockDtmfSender>();
  }

  MOCK_METHOD(void,
              RegisterObserver,
              (DtmfSenderObserverInterface * observer),
              (override));
  MOCK_METHOD(void, UnregisterObserver, (), (override));
  MOCK_METHOD(bool, CanInsertDtmf, (), (override));
  MOCK_METHOD(std::string, tones, (), (const, override));
  MOCK_METHOD(int, duration, (), (const, override));
  MOCK_METHOD(int, inter_tone_gap, (), (const, override));

 protected:
  MockDtmfSender() = default;
};

static_assert(!std::is_abstract_v<webrtc::RefCountedObject<MockDtmfSender>>,
              "");

}  // namespace webrtc

#endif  // API_TEST_MOCK_DTMF_SENDER_H_
