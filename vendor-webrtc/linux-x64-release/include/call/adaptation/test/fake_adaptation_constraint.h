/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_TEST_FAKE_ADAPTATION_CONSTRAINT_H_
#define CALL_ADAPTATION_TEST_FAKE_ADAPTATION_CONSTRAINT_H_

#include <string>

#include "absl/strings/string_view.h"
#include "call/adaptation/adaptation_constraint.h"
#include "call/adaptation/video_source_restrictions.h"
#include "call/adaptation/video_stream_input_state.h"

namespace webrtc {

class FakeAdaptationConstraint : public AdaptationConstraint {
 public:
  explicit FakeAdaptationConstraint(absl::string_view name);
  ~FakeAdaptationConstraint() override;

  void set_is_adaptation_up_allowed(bool is_adaptation_up_allowed);

  // AdaptationConstraint implementation.
  std::string Name() const override;
  bool IsAdaptationUpAllowed(
      const VideoStreamInputState& input_state,
      const VideoSourceRestrictions& restrictions_before,
      const VideoSourceRestrictions& restrictions_after) const override;

 private:
  const std::string name_;
  bool is_adaptation_up_allowed_;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_TEST_FAKE_ADAPTATION_CONSTRAINT_H_
