/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_TEST_FAKE_RESOURCE_H_
#define CALL_ADAPTATION_TEST_FAKE_RESOURCE_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/adaptation/resource.h"
#include "api/scoped_refptr.h"

namespace webrtc {

// Fake resource used for testing.
class FakeResource : public Resource {
 public:
  static scoped_refptr<FakeResource> Create(absl::string_view name);

  explicit FakeResource(absl::string_view name);
  ~FakeResource() override;

  void SetUsageState(ResourceUsageState usage_state);

  // Resource implementation.
  std::string Name() const override;
  void SetResourceListener(ResourceListener* listener) override;

 private:
  const std::string name_;
  ResourceListener* listener_;
};

}  // namespace webrtc

#endif  // CALL_ADAPTATION_TEST_FAKE_RESOURCE_H_
