/*
 *  Copyright 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TRANSPORT_TEST_CREATE_FEEDBACK_GENERATOR_H_
#define API_TRANSPORT_TEST_CREATE_FEEDBACK_GENERATOR_H_

#include <memory>

#include "api/transport/test/feedback_generator_interface.h"

namespace webrtc {
std::unique_ptr<FeedbackGenerator> CreateFeedbackGenerator(
    FeedbackGenerator::Config confg);
}  // namespace webrtc
#endif  // API_TRANSPORT_TEST_CREATE_FEEDBACK_GENERATOR_H_
