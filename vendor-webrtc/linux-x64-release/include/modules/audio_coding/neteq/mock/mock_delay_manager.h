/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DELAY_MANAGER_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DELAY_MANAGER_H_

#include "api/neteq/tick_timer.h"
#include "modules/audio_coding/neteq/delay_manager.h"
#include "test/gmock.h"

namespace webrtc {

class MockDelayManager : public DelayManager {
 public:
  MockDelayManager(const MockDelayManager::Config& config,
                   const TickTimer* tick_timer)
      : DelayManager(config, tick_timer) {}
  MOCK_METHOD(int, TargetDelayMs, (), (const));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DELAY_MANAGER_H_
