/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_MOCK_AUDIO_ENCODER_FACTORY_H_
#define TEST_MOCK_AUDIO_ENCODER_FACTORY_H_

#include <memory>
#include <vector>

#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/environment/environment.h"
#include "api/make_ref_counted.h"
#include "api/scoped_refptr.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioEncoderFactory
    : public ::testing::NiceMock<AudioEncoderFactory> {
 public:
  MOCK_METHOD(std::vector<AudioCodecSpec>,
              GetSupportedEncoders,
              (),
              (override));
  MOCK_METHOD(std::optional<AudioCodecInfo>,
              QueryAudioEncoder,
              (const SdpAudioFormat& format),
              (override));
  MOCK_METHOD(std::unique_ptr<AudioEncoder>,
              Create,
              (const Environment&, const SdpAudioFormat&, Options),
              (override));

  // Creates a MockAudioEncoderFactory with no formats and that may not be
  // invoked to create a codec - useful for initializing a voice engine, for
  // example.
  static scoped_refptr<MockAudioEncoderFactory> CreateUnusedFactory() {
    auto factory = make_ref_counted<MockAudioEncoderFactory>();
    EXPECT_CALL(*factory, Create).Times(0);
    return factory;
  }

  // Creates a MockAudioEncoderFactory with no formats that may be invoked to
  // create a codec any number of times. It will, though, return nullptr on each
  // call, since it supports no codecs.
  static scoped_refptr<MockAudioEncoderFactory> CreateEmptyFactory() {
    return make_ref_counted<MockAudioEncoderFactory>();
  }
};

}  // namespace webrtc

#endif  // TEST_MOCK_AUDIO_ENCODER_FACTORY_H_
