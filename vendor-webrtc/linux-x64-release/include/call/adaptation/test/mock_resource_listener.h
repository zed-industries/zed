/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_TEST_MOCK_RESOURCE_LISTENER_H_
#define CALL_ADAPTATION_TEST_MOCK_RESOURCE_LISTENER_H_

#include "api/adaptation/resource.h"
#include "api/scoped_refptr.h"
#include "test/gmock.h"

namespace webrtc {

class MockResourceListener : public ResourceListener {
 public:
  MOCK_METHOD(void,
              OnResourceUsageStateMeasured,
              (scoped_refptr<Resource> resource,
               ResourceUsageState usage_state),
              (override));
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_TEST_MOCK_RESOURCE_LISTENER_H_
